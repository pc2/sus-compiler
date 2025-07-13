use std::{cell::UnsafeCell, fmt::Debug, ops::Deref};

use crate::{append_only_vec::AppendOnlyVec, let_unwrap, typing::type_inference::UnifyResult};

enum Interior<T: Debug + Clone> {
    Known(T),
    /// If no substitution is known yet, then this points to itself (may be in any cycle length, [Substitutor::resolve_substitution_chain] is there to contract it).
    SubstitutesTo(usize),
    /// Default state of a new Type Variable. This means the variable is *unique*, and so we don't yet need an ID to track its Unification.
    /// CANNOT BE CLONED (panics)
    Unallocated,
}

impl<T: Debug + Clone> Clone for Interior<T> {
    fn clone(&self) -> Self {
        match self {
            Self::Known(arg0) => Self::Known(arg0.clone()),
            Self::SubstitutesTo(var) => Self::SubstitutesTo(*var),
            Self::Unallocated => {
                unreachable!("Cannot clone Unallocated! That would add an incorrect dependency! Use [Substitutor::clone_type] instead!")
            }
        }
    }
}

impl<T: Debug + Clone> Debug for Interior<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Known(known) => f.debug_tuple("Known").field(known).finish(),
            Self::SubstitutesTo(to) => f.debug_tuple("SubstitutesTo").field(to).finish(),
            Self::Unallocated => write!(f, "Unallocated"),
        }
    }
}

/// Basically a [std::cell::OnceCell] for type checking. We implement it safely by maintaining the following invariant:
///
/// - [UniCell] starts out [UniCell::UNKNOWN]. No interior references can be taken in this state. (But the type variable we refer to *can* be updated)
/// - At some point, it is set to some Known value. After this point references to this interior value can be taken.
///     Afterwards, we can *never* reset a Known back to an Unknown, or mess with it in any mutable way. (Panics when trying otherwise)
pub struct UniCell<T: Debug + Clone>(UnsafeCell<Interior<T>>);

impl<T: Debug + Clone> Debug for UniCell<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("UniCell").field(&self.0).finish()
    }
}

impl<T: Debug + Clone> UniCell<T> {
    #[allow(clippy::declare_interior_mutable_const)]
    pub const UNKNOWN: Self = Self(UnsafeCell::new(Interior::Unallocated));

    pub fn get(&self) -> Option<&T> {
        // We cast to a const pointer here instead, such that we never actually create a &mut that might conflict with another existing shared ref
        let content_ptr: *const Interior<T> = self.0.get();
        // SAFETY: In shared context, once we're [Interior::Known] that reference will never be invalidated.
        match unsafe { &*content_ptr } {
            Interior::Known(known) => Some(known),
            Interior::SubstitutesTo(_) | Interior::Unallocated => None,
        }
    }
    pub fn get_mut(&mut self) -> Option<&mut T> {
        // No need for unsafe here, as we're guaranteed to have a unique reference anyway
        match self.0.get_mut() {
            Interior::Known(known) => Some(known),
            Interior::SubstitutesTo(_) | Interior::Unallocated => None,
        }
    }

    #[track_caller]
    fn unwrap(&self) -> &T {
        let content_ptr: *const Interior<T> = self.0.get();
        // SAFETY: In shared context, once we're [Interior::Known] that reference will never be invalidated.
        let content = unsafe { &*content_ptr };
        let_unwrap!(Interior::Known(v), content);
        v
    }
    #[track_caller]
    fn unwrap_mut(&mut self) -> &mut T {
        let_unwrap!(Interior::Known(v), self.0.get_mut());
        v
    }

    /// Panics if this has ever been unified with anything else
    pub fn set_initial(&self, v: T) {
        let content_ptr_mut: *mut Interior<T> = self.0.get();
        let context_ptr_while_maybe_shared: *const Interior<T> = content_ptr_mut;
        {
            // SAFETY: Either this is already Interior::Known, in which case we panic before messing with a shared ref, OR this is not Interior::Known, and therefore no interior shared references have been given out
            //
            // Careful here! If we use the &mut for the check, then we've technically created UB between here and the panic. That's why we have to use a *const _, and only create the mutable ref when we're sure no shares exist
            let content = unsafe { &*context_ptr_while_maybe_shared };
            if !matches!(content, Interior::Unallocated) {
                unreachable!("`set_initial({v:?})` on a UniCell that's not in the Unallocated state ({content:?})!");
            }
        }

        // SAFETY: Only now do we actually use the ptr mutably
        unsafe { *content_ptr_mut = Interior::Known(v) }
    }
}

impl<T: Debug + Clone> From<T> for UniCell<T> {
    fn from(known: T) -> Self {
        Self(UnsafeCell::new(Interior::Known(known)))
    }
}

impl<T: Debug + Clone> Clone for UniCell<T> {
    fn clone(&self) -> Self {
        // We cast to a const pointer here instead, such that we never actually create a &mut that might conflict with another existing shared ref
        let content_ptr: *const Interior<T> = self.0.get();
        // SAFETY: In shared context, once we're [Interior::Known] that reference will never be invalidated.
        let result = match unsafe { &*content_ptr } {
            Interior::Known(known) => Interior::Known(known.clone()),
            Interior::SubstitutesTo(id) => Interior::SubstitutesTo(*id),
            Interior::Unallocated => {
                panic!("Type Variables that aren't yet allocated cannot be cloned!")
            }
        };
        Self(UnsafeCell::new(result))
    }
}

pub enum ChainResolution<'s, T> {
    Known(&'s T),
    Unknown(usize),
}

pub struct Substitutor<'s, Typ: Debug + Clone>(AppendOnlyVec<&'s UniCell<Typ>>);

impl<'s, Typ: Debug + Clone> Default for Substitutor<'s, Typ> {
    fn default() -> Self {
        Self(Default::default())
    }
}

impl<'s, T: Debug + Clone> Substitutor<'s, T> {
    pub fn new() -> Self {
        Self(AppendOnlyVec::new())
    }

    /// `ptr` must be [Interior::SubstitutesTo]
    fn resolve_chain_recurse(&self, ptr: *mut Interior<T>, ptr_target: usize) -> usize {
        unsafe {
            let target = self.0.copy_elem(ptr_target);

            let target_ptr: *mut _ = target.0.get();
            match &*target_ptr {
                Interior::Known(_) => ptr_target,
                Interior::SubstitutesTo(targets_target) => {
                    // Deref early so we don't create UB
                    let targets_target = *targets_target;
                    // Yes, this gets overwritten at the end, but we need to already be updating pointers to contract the final cycle so we can detect it
                    *ptr = Interior::SubstitutesTo(targets_target);

                    if targets_target == ptr_target {
                        ptr_target
                    } else {
                        let new_target = self.resolve_chain_recurse(target_ptr, targets_target);
                        // Not strictly necessary, but this makes *all* pointers in the chain point to the final node
                        *ptr = Interior::SubstitutesTo(new_target);
                        new_target
                    }
                }
                Interior::Unallocated => unreachable!(),
            }
        }
    }
    /// Resolves a possibly extensive chain of substitutions to a single node.
    ///
    /// Result is either:
    ///     [Interior::Known] is then of course a known value.
    ///     [Interior::SubstitutesTo] that points to itself, signifying no substitution known yet
    pub fn get_with_substitution(&self, typ: &'s UniCell<T>) -> ChainResolution<'s, T> {
        unsafe {
            let ptr: *mut Interior<T> = typ.0.get();
            match &*ptr {
                Interior::Known(known) => ChainResolution::Known(known),
                Interior::SubstitutesTo(to_id) => {
                    let final_target_id = self.resolve_chain_recurse(ptr, *to_id);
                    let final_target = self.0.copy_elem(final_target_id);
                    match &*final_target.0.get() {
                        Interior::Known(known) => ChainResolution::Known(known),
                        Interior::SubstitutesTo(final_target_id_copy) => {
                            assert_eq!(*final_target_id_copy, final_target_id);
                            ChainResolution::Unknown(final_target_id)
                        }
                        Interior::Unallocated => unreachable!(),
                    }
                }
                Interior::Unallocated => {
                    let result_id = self.alloc(typ);
                    *ptr = Interior::SubstitutesTo(result_id);
                    ChainResolution::Unknown(result_id)
                }
            }
        }
    }

    /// `contains_subtree` is used to prevent infinite types.
    /// It must be implemented using [Self::get_with_substitution] to iterate through its subtrees.
    /// If a subtree is found that contains the given pointer it must return true.
    ///
    /// `unify_subtrees` should recursively call [Self::unify] for every pair of subtrees.
    /// If some irreconcilable difference is found it should return [UnifyResult::Failure].
    /// Otherwise return the binary AND of subtree unifications.
    /// Regardless of failure, all subtrees should be unified for best possible type error information.
    /// You as the user should never return [UnifyResult::FailureInfiniteTypes]
    pub fn unify(
        &self,
        a: &'s UniCell<T>,
        b: &'s UniCell<T>,
        contains_subtree: impl FnOnce(&'s T, usize) -> bool,
        unify_subtrees: impl FnOnce(&'s T, &'s T) -> UnifyResult,
    ) -> UnifyResult {
        let resol_a = self.get_with_substitution(a);
        let resol_b = self.get_with_substitution(b);

        match (resol_a, resol_b) {
            (ChainResolution::Known(a), ChainResolution::Known(b)) => {
                if std::ptr::eq(a, b) {
                    UnifyResult::Success // Minor optimization, when we see referential equality
                } else {
                    unify_subtrees(a, b)
                }
            }
            (ChainResolution::Known(known), ChainResolution::Unknown(id)) => {
                if contains_subtree(known, id) {
                    UnifyResult::FailureInfiniteTypes
                } else {
                    let _ = self.0.set_elem(id, a);
                    UnifyResult::Success
                }
            }
            (ChainResolution::Unknown(id), ChainResolution::Known(known)) => {
                if contains_subtree(known, id) {
                    UnifyResult::FailureInfiniteTypes
                } else {
                    let _ = self.0.set_elem(id, b);
                    UnifyResult::Success
                }
            }
            (ChainResolution::Unknown(id), ChainResolution::Unknown(_)) => {
                let _ = self.0.set_elem(id, b);
                UnifyResult::Success
            }
        }
    }

    /// `recursively_subsitute` must call [Self::substitute] on every child [UniCell]
    pub fn substitute(&self, typ: &'s UniCell<T>, recursively_subsitute: impl FnOnce(&'s T)) {
        if let Some(known) = typ.get() {
            recursively_subsitute(known);
        } else {
            match self.get_with_substitution(typ) {
                ChainResolution::Known(known) => unsafe {
                    recursively_subsitute(known);

                    // Because we've done the recusive_substitute, that should've called `substitute` on every recursive [UniCell], `known` can safely be cloned, it can't contain any Unallocateds anymore
                    let known_clone = known.clone();

                    *typ.0.get() = Interior::Known(known_clone);
                },
                ChainResolution::Unknown(_) => {}
            }
        }
    }

    /// When constructing types regular Clone is not a good option. It'll crash due to [Interior::Unallocated]. Instead this version either allocates a new variable for it, or copies the existing substitution
    pub fn clone_type(&self, typ: &'s UniCell<T>) -> UniCell<T> {
        let ptr: *mut _ = typ.0.get();
        let new_id = match unsafe { &*ptr } {
            Interior::Known(_) => self.alloc(typ),
            Interior::SubstitutesTo(id) => *id,
            Interior::Unallocated => {
                let slf_id = self.alloc(typ);
                unsafe {
                    *ptr = Interior::SubstitutesTo(slf_id);
                }
                slf_id
            }
        };
        UniCell(UnsafeCell::new(Interior::SubstitutesTo(new_id)))
    }

    fn alloc(&self, elem: &'s UniCell<T>) -> usize {
        let idx = self.0.len();
        self.0.push(elem);
        idx
    }
}

#[derive(Debug, Clone)]
enum PeanoType {
    Zero,
    Succ(Box<UniCell<PeanoType>>),
}

impl PeanoType {
    #[allow(clippy::declare_interior_mutable_const)]
    pub const UNKNOWN: UniCell<PeanoType> = UniCell::<PeanoType>::UNKNOWN;

    fn count(&self) -> usize {
        match self {
            PeanoType::Zero => 0,
            PeanoType::Succ(inner) => inner.unwrap().count() + 1,
        }
    }
}

impl<'s> Substitutor<'s, PeanoType> {
    fn contains_subtree(&self, find_in: &'s PeanoType, target: usize) -> bool {
        match find_in {
            PeanoType::Zero => false,
            PeanoType::Succ(subtree_cell) => match self.get_with_substitution(subtree_cell) {
                ChainResolution::Known(known) => self.contains_subtree(known, target),
                ChainResolution::Unknown(var) => var == target,
            },
        }
    }
    fn unify_peanos(&self, l: &'s UniCell<PeanoType>, r: &'s UniCell<PeanoType>) -> UnifyResult {
        self.unify(
            l,
            r,
            |known, to_find| self.contains_subtree(known, to_find),
            |lc, rc| match (lc, rc) {
                (PeanoType::Zero, PeanoType::Zero) => UnifyResult::Success,
                (PeanoType::Succ(lc), PeanoType::Succ(rc)) => self.unify_peanos(lc, rc),
                _ => UnifyResult::Failure,
            },
        )
    }
    fn fully_substitute(&self, typ: &'s UniCell<PeanoType>) {
        self.substitute(typ, |inner| match inner {
            PeanoType::Zero => {}
            PeanoType::Succ(inner) => self.fully_substitute(inner),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn add_to(mut cur: UniCell<PeanoType>, up_to: usize) -> UniCell<PeanoType> {
        for _ in 0..up_to {
            cur = PeanoType::Succ(Box::new(cur)).into();
        }

        cur
    }
    fn mk_peano(up_to: usize) -> UniCell<PeanoType> {
        add_to(PeanoType::Zero.into(), up_to)
    }

    #[test]
    fn test_peano_initial() {
        let a = mk_peano(4);
        let b = mk_peano(2);

        let unknown = PeanoType::UNKNOWN;

        unknown.set_initial(PeanoType::Zero);

        assert_eq!(a.unwrap().count(), 4);
        assert_eq!(b.unwrap().count(), 2);
        assert_eq!(unknown.unwrap().count(), 0);
    }
    #[test]
    fn test_peano_unify() {
        let substitutor = Substitutor::new();

        let four = mk_peano(4);

        let a = PeanoType::UNKNOWN;
        let three_plus_a = add_to(substitutor.clone_type(&a), 3);

        substitutor.unify_peanos(&four, &three_plus_a);

        substitutor.fully_substitute(&a);

        assert_eq!(a.unwrap().count(), 1)
    }

    #[test]
    fn test_non_infinite_peano() {
        let substitutor = Substitutor::new();

        let a = PeanoType::UNKNOWN;
        let a_plus_zero = add_to(substitutor.clone_type(&a), 0);

        assert_eq!(
            substitutor.unify_peanos(&a, &a_plus_zero),
            UnifyResult::Success
        );

        substitutor.fully_substitute(&a);
    }

    #[test]
    fn test_invalid_unification() {
        let substitutor = Substitutor::new();

        let three = mk_peano(3);
        let four = mk_peano(4);

        assert_eq!(
            substitutor.unify_peanos(&three, &four),
            UnifyResult::Failure
        );

        substitutor.fully_substitute(&three);
        substitutor.fully_substitute(&four);
    }

    #[test]
    fn test_infinite_peano() {
        let substitutor = Substitutor::new();

        let a = PeanoType::UNKNOWN;
        let a_plus_one = add_to(substitutor.clone_type(&a), 1);

        assert_eq!(
            substitutor.unify_peanos(&a, &a_plus_one),
            UnifyResult::FailureInfiniteTypes
        );

        substitutor.fully_substitute(&a);
    }

    #[test]
    fn test_peano_equivalence_simple() {
        let substitutor = Substitutor::new();

        let one = mk_peano(1);
        let two = mk_peano(2);
        let one_plus_three = add_to(substitutor.clone_type(&one), 3);
        let two_plus_two = add_to(substitutor.clone_type(&two), 2);

        // 2+2 == 1+3
        assert_eq!(
            substitutor.unify_peanos(&two_plus_two, &one_plus_three),
            UnifyResult::Success
        );
    }

    #[test]
    fn test_peano_multiple_variables_chain() {
        let substitutor = Substitutor::new();

        let x = PeanoType::UNKNOWN;
        let y = PeanoType::UNKNOWN;
        let z = PeanoType::UNKNOWN;

        // x = 2, y = x + 1, z = y + 1
        x.set_initial(PeanoType::Zero);
        let x_plus_2 = add_to(substitutor.clone_type(&x), 2);
        let y_val = add_to(substitutor.clone_type(&x), 1);
        let z_val = add_to(substitutor.clone_type(&y), 1);

        // Unify y with x+1, z with y+1, and z with x+2
        assert_eq!(substitutor.unify_peanos(&y, &y_val), UnifyResult::Success);
        assert_eq!(substitutor.unify_peanos(&z, &z_val), UnifyResult::Success);
        assert_eq!(
            substitutor.unify_peanos(&z, &x_plus_2),
            UnifyResult::Success
        );

        substitutor.fully_substitute(&x);
        substitutor.fully_substitute(&y);
        substitutor.fully_substitute(&z);

        assert_eq!(x.unwrap().count(), 0);
        assert_eq!(y.unwrap().count(), 1);
        assert_eq!(z.unwrap().count(), 2);
    }

    #[test]
    fn test_peano_complex_substitution_graph() {
        let substitutor = Substitutor::new();

        let a = PeanoType::UNKNOWN;
        let b = PeanoType::UNKNOWN;
        let c = PeanoType::UNKNOWN;

        // a = 2, b = a + 2, c = b + 1
        a.set_initial(PeanoType::Zero);
        let b_val = add_to(substitutor.clone_type(&a), 2);
        let c_val = add_to(substitutor.clone_type(&b), 1);

        assert_eq!(substitutor.unify_peanos(&b, &b_val), UnifyResult::Success);
        assert_eq!(substitutor.unify_peanos(&c, &c_val), UnifyResult::Success);

        substitutor.fully_substitute(&a);
        substitutor.fully_substitute(&b);
        substitutor.fully_substitute(&c);

        assert_eq!(a.unwrap().count(), 0);
        assert_eq!(b.unwrap().count(), 2);
        assert_eq!(c.unwrap().count(), 3);
    }
}
