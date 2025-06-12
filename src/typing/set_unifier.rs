#![allow(clippy::type_complexity)]

use std::fmt::{Debug, Display, Formatter};
use std::marker::PhantomData;
use std::ops::Deref;

use crate::alloc::{ArenaAllocator, UUIDAllocator, UUIDMarker, UUID};
use crate::append_only_vec::AppendOnlyVec;
use crate::prelude::*;

use crate::util::merge_vec_into;

#[derive(Debug)]
enum KnownValue<T, ID> {
    Unknown {
        backrefs: Vec<ID>,
        used_in: Vec<UUID<ConstraintIDMarker>>,
    },
    Known(T),
}

struct Constraint<'inst, T: Eq + Clone, IDMarker> {
    num_unknown_dependencies: usize,
    constraint: Box<dyn FnOnce(&mut SetUnifier<T, IDMarker>) + 'inst>,
}
pub struct ConstraintReservation(UUID<ConstraintIDMarker>, usize);

struct KnownIDMarker;
impl UUIDMarker for KnownIDMarker {
    const DISPLAY_NAME: &'static str = "known_";
}
struct ConstraintIDMarker;
impl UUIDMarker for ConstraintIDMarker {
    const DISPLAY_NAME: &'static str = "constraint_";
}

/// Referencing [Unifyable::Unknown] is a strong code smell.
/// It is likely you should use [crate::typing::type_inference::TypeSubstitutor::unify_must_succeed]
/// or [crate::typing::type_inference::TypeSubstitutor::unify_report_error] instead
///
/// It should only occur in creation `Unifyable::Unknown(self.type_substitutor.alloc())`
pub enum Unifyable<T, IDMarker> {
    Set(T),
    Unknown(UUID<IDMarker>),
}

impl<T: Eq + Clone, IDMarker: UUIDMarker> Unifyable<T, IDMarker> {
    pub fn is_unknown(&self) -> bool {
        match self {
            Unifyable::Set(_) => false,
            Unifyable::Unknown(_) => true,
        }
    }
    pub fn unwrap_set(&self) -> &T {
        match self {
            Unifyable::Set(s) => s,
            Unifyable::Unknown(_) => panic!("unwrap_set not allowed to be Unknown"),
        }
    }
}

impl<T, IDMarker: UUIDMarker> Deref for Unifyable<T, IDMarker> {
    type Target = T;

    #[track_caller]
    fn deref(&self) -> &T {
        let Self::Set(v) = self else {
            unreachable!("Attempting to Deref a not-Set Unifyable!")
        };
        v
    }
}

impl<T: Display, IDMarker: UUIDMarker> Display for Unifyable<T, IDMarker> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Unifyable::Set(v) => v.fmt(f),
            Unifyable::Unknown(id) => f.write_fmt(format_args!("{id:?}")),
        }
    }
}

impl<T: Debug, IDMarker: UUIDMarker> Debug for Unifyable<T, IDMarker> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Unifyable::Set(v) => v.fmt(f),
            Unifyable::Unknown(id) => f.write_fmt(format_args!("{id:?}")),
        }
    }
}

impl<T: Clone, IDMarker: UUIDMarker> Clone for Unifyable<T, IDMarker> {
    fn clone(&self) -> Self {
        match self {
            Self::Set(arg0) => Self::Set(arg0.clone()),
            Self::Unknown(arg0) => Self::Unknown(*arg0),
        }
    }
}
impl<T: PartialEq + Debug, IDMarker: UUIDMarker> PartialEq for Unifyable<T, IDMarker> {
    fn eq(&self, other: &Self) -> bool {
        let_unwrap!(Self::Set(a), self);
        let_unwrap!(Self::Set(b), other);
        a.eq(b)
    }
}
impl<T: Eq + Debug, IDMarker: UUIDMarker> Eq for Unifyable<T, IDMarker> {}
impl<T: std::hash::Hash + Debug, IDMarker: UUIDMarker> std::hash::Hash for Unifyable<T, IDMarker> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        let_unwrap!(Self::Set(a), self);
        a.hash(state);
    }
}

pub struct UnifyableAlloc<T: Eq + Clone, IDMarker> {
    ptrs: UUIDAllocator<IDMarker>,
    _ph: PhantomData<T>,
}

impl<T: Eq + Clone, IDMarker> Default for UnifyableAlloc<T, IDMarker> {
    fn default() -> Self {
        Self {
            ptrs: Default::default(),
            _ph: Default::default(),
        }
    }
}

impl<T: Eq + Clone, IDMarker> UnifyableAlloc<T, IDMarker> {
    pub fn alloc_unknown(&mut self) -> Unifyable<T, IDMarker> {
        Unifyable::Unknown(self.ptrs.alloc())
    }
}

pub struct SetUnifierStore<T: Clone, IDMarker> {
    ptrs: FlatAlloc<UUID<KnownIDMarker>, IDMarker>,
    known_values: ArenaAllocator<KnownValue<T, UUID<IDMarker>>, KnownIDMarker>,
}

impl<T: Clone, IDMarker> SetUnifierStore<T, IDMarker> {
    pub fn get_substitution<'v>(&'v self, val: &'v Unifyable<T, IDMarker>) -> Option<&'v T> {
        match val {
            Unifyable::Set(v) => Some(v),
            Unifyable::Unknown(id) => match &self.known_values[self.ptrs[*id]] {
                KnownValue::Unknown { .. } => None,
                KnownValue::Known(new_v) => Some(new_v),
            },
        }
    }
}

pub struct SetUnifier<'inst, T: Eq + Clone, IDMarker> {
    pub store: SetUnifierStore<T, IDMarker>,
    constraints: ArenaAllocator<Constraint<'inst, T, IDMarker>, ConstraintIDMarker>,
    constraints_ready_for_unification: Vec<Box<dyn FnOnce(&mut SetUnifier<T, IDMarker>) + 'inst>>,
}
impl<'inst, T: Eq + Clone + Debug, IDMarker: UUIDMarker> SetUnifier<'inst, T, IDMarker> {
    pub fn from_alloc(alloc: UnifyableAlloc<T, IDMarker>) -> Self {
        let mut known_values = ArenaAllocator::new();
        let ptrs = alloc.ptrs.as_range().map(|id| {
            known_values.alloc(KnownValue::Unknown {
                backrefs: vec![id],
                used_in: Vec::new(),
            })
        });
        SetUnifier {
            store: SetUnifierStore { ptrs, known_values },
            constraints: ArenaAllocator::new(),
            constraints_ready_for_unification: Vec::new(),
        }
    }
    /// Executes all constraints (that become ready). Returns `false` if no constraints were ready
    pub fn execute_ready_constraints(&mut self) -> bool {
        let at_least_one = !self.constraints_ready_for_unification.is_empty();
        while let Some(cstr) = self.constraints_ready_for_unification.pop() {
            cstr(self);
        }
        at_least_one
    }

    pub fn decomission(self) -> SetUnifierStore<T, IDMarker> {
        self.store
    }

    fn notify_constraints(
        constraints: &mut ArenaAllocator<Constraint<'inst, T, IDMarker>, ConstraintIDMarker>,
        constraints_ready_for_unification: &mut Vec<
            Box<dyn FnOnce(&mut SetUnifier<T, IDMarker>) + 'inst>,
        >,
        used_in: Vec<UUID<ConstraintIDMarker>>,
    ) {
        for u in &*used_in {
            constraints[*u].num_unknown_dependencies -= 1;
            if constraints[*u].num_unknown_dependencies == 0 {
                constraints_ready_for_unification.push(constraints.free(*u).constraint);
            }
        }
    }

    pub fn unify(&mut self, a: &Unifyable<T, IDMarker>, b: &Unifyable<T, IDMarker>) -> bool {
        match (a, b) {
            (Unifyable::Set(a), Unifyable::Set(b)) => a == b,
            (Unifyable::Set(v), Unifyable::Unknown(var))
            | (Unifyable::Unknown(var), Unifyable::Set(v)) => {
                let k = &mut self.store.known_values[self.store.ptrs[*var]];
                match k {
                    KnownValue::Unknown {
                        backrefs: _,
                        used_in,
                    } => {
                        let used_in = std::mem::take(used_in);
                        *k = KnownValue::Known(v.clone());
                        Self::notify_constraints(
                            &mut self.constraints,
                            &mut self.constraints_ready_for_unification,
                            used_in,
                        );
                        true
                    }
                    KnownValue::Known(k) => k == v,
                }
            }
            (Unifyable::Unknown(idx_a), Unifyable::Unknown(idx_b)) => {
                let idx_a = self.store.ptrs[*idx_a];
                let idx_b = self.store.ptrs[*idx_b];
                match self.store.known_values.get2_mut(idx_a, idx_b) {
                    Some((
                        KnownValue::Unknown {
                            backrefs: backrefs_a,
                            used_in: used_in_a,
                        },
                        KnownValue::Unknown {
                            backrefs: backrefs_b,
                            used_in: used_in_b,
                        },
                    )) => {
                        if backrefs_a.len() > backrefs_b.len() {
                            // Merge into a
                            merge_vec_into(used_in_a, std::mem::take(used_in_b));
                            for v in &*backrefs_b {
                                self.store.ptrs[*v] = idx_a;
                            }
                            backrefs_a.extend_from_slice(backrefs_b);
                            self.store.known_values.free(idx_b);
                        } else {
                            // Merge into b
                            merge_vec_into(used_in_b, std::mem::take(used_in_a));
                            for v in &*backrefs_a {
                                self.store.ptrs[*v] = idx_b;
                            }
                            backrefs_b.extend_from_slice(backrefs_a);
                            self.store.known_values.free(idx_a);
                        }
                        true
                    }
                    Some((KnownValue::Unknown { backrefs, used_in }, KnownValue::Known(_))) => {
                        // Resolve references to a to point to b
                        for v in &*backrefs {
                            self.store.ptrs[*v] = idx_b;
                        }
                        let used_in = std::mem::take(used_in);
                        Self::notify_constraints(
                            &mut self.constraints,
                            &mut self.constraints_ready_for_unification,
                            used_in,
                        );
                        self.store.known_values.free(idx_a);
                        true
                    }
                    Some((KnownValue::Known(_), KnownValue::Unknown { backrefs, used_in })) => {
                        // Resolve references to b to point to a
                        for v in &*backrefs {
                            self.store.ptrs[*v] = idx_a;
                        }
                        let used_in = std::mem::take(used_in);
                        Self::notify_constraints(
                            &mut self.constraints,
                            &mut self.constraints_ready_for_unification,
                            used_in,
                        );
                        self.store.known_values.free(idx_b);
                        true
                    }
                    Some((KnownValue::Known(x), KnownValue::Known(y))) => x == y,
                    None => true,
                }
            }
        }
    }

    /// If unification is with an incompatible target, then
    pub fn set(&mut self, a: &Unifyable<T, IDMarker>, v: T) -> Result<(), T> {
        match a {
            Unifyable::Set(k) => {
                if k == &v {
                    Ok(())
                } else {
                    Err(k.clone())
                }
            }
            Unifyable::Unknown(var) => {
                let k = &mut self.store.known_values[self.store.ptrs[*var]];
                match k {
                    KnownValue::Unknown {
                        backrefs: _,
                        used_in,
                    } => {
                        let used_in = std::mem::take(used_in);
                        *k = KnownValue::Known(v.clone());
                        Self::notify_constraints(
                            &mut self.constraints,
                            &mut self.constraints_ready_for_unification,
                            used_in,
                        );
                        Ok(())
                    }
                    KnownValue::Known(k) => {
                        if k == &v {
                            Ok(())
                        } else {
                            Err(k.clone())
                        }
                    }
                }
            }
        }
    }

    /// The parameters given to this can be safely unwrapped in [Self::unwrap_known]
    pub fn add_constraint<'a>(
        &mut self,
        dependencies: impl IntoIterator<Item = &'a Unifyable<T, IDMarker>>,
        f: impl FnOnce(&mut SetUnifier<'_, T, IDMarker>) + 'inst,
    ) where
        T: 'a,
        IDMarker: 'a,
    {
        let reservation = self.reserve_constraint(dependencies);
        self.place_reserved_constraint(reservation, f);
    }

    /// This function and [Self::place_reserved_constraint] are used in tandem to split up the dependency on `dependencies` in [Self::add_constraint]. (The passed function will probably want ownership) For small lists this is no issue, but for bigger lists it would require a large clone.
    pub fn reserve_constraint<'a>(
        &mut self,
        dependencies: impl IntoIterator<Item = &'a Unifyable<T, IDMarker>>,
    ) -> ConstraintReservation
    where
        T: 'a,
        IDMarker: 'a,
    {
        let constr_id = self.constraints.reserve();
        let mut num_unknown_dependencies = 0;
        for d in dependencies {
            if let Unifyable::Unknown(var_id) = d {
                if let KnownValue::Unknown { used_in, .. } =
                    &mut self.store.known_values[self.store.ptrs[*var_id]]
                {
                    num_unknown_dependencies += 1;
                    used_in.push(constr_id);
                }
            }
        }
        ConstraintReservation(constr_id, num_unknown_dependencies)
    }
    /// This function and [Self::reserve_constraint] are used in tandem to split up the dependency on `dependencies` in [Self::add_constraint]. (The passed function will probably want ownership) For small lists this is no issue, but for bigger lists it would require a large clone.
    pub fn place_reserved_constraint<'a>(
        &mut self,
        ConstraintReservation(constr_id, num_unknown_dependencies): ConstraintReservation,
        f: impl FnOnce(&mut SetUnifier<T, IDMarker>) + 'inst,
    ) where
        T: 'a,
        IDMarker: 'a,
    {
        if num_unknown_dependencies > 0 {
            self.constraints.alloc_reservation(
                constr_id,
                Constraint {
                    num_unknown_dependencies,
                    constraint: Box::new(f),
                },
            );
        } else {
            self.constraints.free_reservation(constr_id);
            f(self);
        }
    }
    /// To be used by [Self::add_constraint]
    pub fn unwrap_known<'v>(&'v self, val: &'v Unifyable<T, IDMarker>) -> &'v T {
        self.store.get_substitution(val).unwrap()
    }
}

pub trait FullySubstitutable<T: Clone, IDMarker> {
    fn gather_all_substitutables<'slf>(
        &'slf self,
        gather_here: &mut Vec<&'slf Unifyable<T, IDMarker>>,
    );
    fn fully_substitute(&mut self, substitutor: &SetUnifierStore<T, IDMarker>) -> bool;
}

impl<T: Clone, IDMarker> FullySubstitutable<T, IDMarker> for Unifyable<T, IDMarker> {
    fn gather_all_substitutables<'slf>(
        &'slf self,
        gather_here: &mut Vec<&'slf Unifyable<T, IDMarker>>,
    ) {
        gather_here.push(self);
    }
    fn fully_substitute(&mut self, substitutor: &SetUnifierStore<T, IDMarker>) -> bool {
        match self {
            Unifyable::Set(_) => true,
            Unifyable::Unknown(id) => match &substitutor.known_values[substitutor.ptrs[*id]] {
                KnownValue::Unknown { .. } => false,
                KnownValue::Known(new_v) => {
                    *self = Unifyable::Set(new_v.clone());
                    true
                }
            },
        }
    }
}

pub struct DelayedErrorCollector<'inst, T: Clone, IDMarker> {
    failures: AppendOnlyVec<Box<dyn FnOnce(&SetUnifierStore<T, IDMarker>) + 'inst>>,
}

impl<'inst, T: Clone, IDMarker> Default for DelayedErrorCollector<'inst, T, IDMarker> {
    fn default() -> Self {
        Self {
            failures: Default::default(),
        }
    }
}

impl<'inst, T: Clone, IDMarker> DelayedErrorCollector<'inst, T, IDMarker> {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn report(self, store: &SetUnifierStore<T, IDMarker>) {
        for f in Vec::from(self.failures) {
            f(store)
        }
    }
    pub fn error(&self, f: impl FnOnce(&SetUnifierStore<T, IDMarker>) + 'inst) {
        self.failures.push(Box::new(f));
    }
}
