use std::{
    borrow::BorrowMut,
    cell::{RefCell, RefMut, UnsafeCell},
    fmt::{Debug, Pointer},
    marker::PhantomData,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UnifyError {
    Failure,
    FailureInfiniteTypes,
}

use crate::{let_unwrap, typing::type_inference::UnifyResult};

/// Basically a [std::cell::OnceCell] for type checking. We implement it safely by maintaining the following invariant:
///
/// - [UniCell] starts out [UniCell::UNKNOWN]. No interior references can be taken in this state. (But the type variable we refer to *can* be updated)
/// - At some point, it is set to [Interior::Known]. After this point references to this interior value can be taken.
///   Afterwards, we can *never* reset a [Interior::Known] back to an [Interior::Unallocated] or [Interior::SubstitutesTo], or mess with it in any mutable way. (Panics when trying otherwise)
pub struct UniCell<T>(UnsafeCell<Interior<T>>);
enum Interior<T> {
    Known(T),
    /// If no substitution is known yet, then this points to itself (may be in any cycle length, [Substitutor::resolve_substitution_chain] is there to contract it).
    SubstitutesTo(usize),
    /// Default state of a new Type Variable. This means the variable is *unique*, and so we don't yet need an ID to track its Unification.
    /// CANNOT BE CLONED (panics)
    Unallocated,
}

impl<T: Debug> UniCell<T> {
    #[allow(clippy::declare_interior_mutable_const)]
    pub const UNKNOWN: Self = Self(UnsafeCell::new(Interior::Unallocated));

    /// Either get a shared reference to the known value if it's set, or a mutable reference to the whole thing if it's not yet known
    /// This is safe, because [UniCell] only allows references to [Interior::Known] once it is set, and it can never be unset through a shared ref
    ///
    /// SAFETY: of course, only such [Self::get_interior] may be active at any one time. You must ensure this.
    fn get_interior(&self) -> Result<&T, Option<usize>> {
        // SAFETY:
        // See [UniCell]'s definition
        unsafe {
            let interior_ptr: *mut Interior<T> = self.0.get();
            match &*(interior_ptr as *const Interior<T>) {
                Interior::Known(known) => Ok(known),
                Interior::SubstitutesTo(id) => Err(Some(*id)),
                Interior::Unallocated => Err(None),
            }
        }
    }

    #[track_caller]
    pub fn unwrap(&self) -> &T {
        self.get_interior().unwrap()
    }
    #[track_caller]
    pub fn unwrap_mut(&mut self) -> &mut T {
        // No need for unsafe here, as we're guaranteed to have a unique reference anyway
        let_unwrap!(Interior::Known(v), self.0.get_mut());
        v
    }
    pub fn into_inner(self) -> T {
        let Interior::Known(v) = self.0.into_inner() else {
            unreachable!("UniCell::into_inner on not a Interior::Known");
        };
        v
    }

    /// Panics if [Substitutor::unify] has ever been called on this
    ///
    /// So only allowed if [Self::is_unallocated]
    fn set_interior(&self, existing_id: Option<usize>, v: Interior<T>) {
        let interior = self.get_interior();
        assert_eq!(
            interior.unwrap_err(),
            existing_id,
            "`set_interior({existing_id:?}, {v:?})` had expected id {existing_id:?}, but found {interior:?}!",
        );

        // SAFETY: We already know we're not Interior::Known
        unsafe { *self.0.get() = v };
    }
    pub fn set_initial(&self, v: T) {
        self.set_interior(None, Interior::Known(v));
    }

    /// Used to clone types that have been created with several [UniCell::UNKNOWN]s.
    /// The cloned [UniCell::UNKNOWN]s become distinct type variables.
    ///
    /// For clones after successful typechecking, use the regular [std::clone::Clone]
    ///
    /// For clones that *do* unify type variables, use [Substitutor::clone_unify]
    pub fn clone_prototype_step(&self, clone_recurse: impl FnOnce(&T) -> T) -> Self {
        match self.get_interior() {
            Ok(known) => Self(UnsafeCell::new(Interior::Known(clone_recurse(known)))),
            Err(Some(_)) => unreachable!(
                "An already unified UniCell cannot be used in [UniCell::clone_prototype_step]"
            ),
            Err(None) => Self(UnsafeCell::new(Interior::Unallocated)),
        }
    }
}

impl<T: Debug> From<T> for UniCell<T> {
    fn from(known: T) -> Self {
        Self(UnsafeCell::new(Interior::Known(known)))
    }
}

impl<T: Debug + Clone> Clone for UniCell<T> {
    fn clone(&self) -> Self {
        // We cast to a const pointer here instead, such that we never actually create a &mut that might conflict with another existing shared ref
        let known = self.get_interior().expect("Not fully known substitutables can't be Cloned at all! Use [Unifier::clone_unify] or [Unifier::clone_prototype_step] to make clones.");
        let known_clone = known.clone();
        Self(UnsafeCell::new(Interior::Known(known_clone)))
    }
}

impl<T: Debug> Debug for UniCell<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.get_interior() {
            Ok(known) => known.fmt(f),
            Err(Some(id)) => write!(f, "SubstituteTo({id})"),
            Err(None) => write!(f, "Unallocated"),
        }
    }
}
impl<T: Debug> Debug for Interior<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Known(known) => f.debug_tuple("Known").field(known).finish(),
            Self::SubstitutesTo(to) => f.debug_tuple("SubstitutesTo").field(to).finish(),
            Self::Unallocated => write!(f, "Unallocated"),
        }
    }
}

struct SubstitutorElem<'s, T: Debug> {
    substitute_to: &'s UniCell<T>,
    constraint_waiting_for: Option<Box<DelayedConstraint<'s>>>,
}
impl<'s, T: Debug> Debug for SubstitutorElem<'s, T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SubstitutorElem")
            .field("substitute_to", &self.substitute_to)
            .field("substitute_ptr", &((&self.substitute_to) as *const _))
            .field(
                "constraints",
                &DelayedConstraint::count(&self.constraint_waiting_for),
            )
            .finish()
    }
}
impl<'s, T: Debug> Debug for Substitutor<'s, T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut list = f.debug_list();
        for (idx, e) in self.substitutor.borrow().0.iter().enumerate() {
            list.entry(&(idx, e));
        }
        Ok(())
    }
}

/// This struct bookkeeps the extra state for a Hindley Mindley Union-Find algorithm. It contains the counterparts to [UniCell]'s [Interior::SubstitutesTo]'s ID field.
/// All references are to [UniCell]s in the field. If a new value needs to be injected into the graph of [UniCell]s, then it should be [UniCell::set_initial].
///
/// To use, you should make custom wrappers around:
/// - [UniCell::clone_prototype_step]
/// - [Self::unify]
/// - [Self::set]
/// - [Self::clone_unify]
/// - [Self::resolve]
/// - [delayed_constraint]
///
/// For examples see [PeanoUnifier]
///
/// The reason we don't do a custom trait, but rather require these wrappers, is that a trait makes using nested substitutors more cumbersome.
/// We'd have to pass "extra data" (the nested substitutors) down the call hierarchy manually.
///
/// [Substitutor] references are *shared* on purpose (I've tried to replace them with &mut many times before).
/// The reason is that shared refs allow for more ergonomic recursive implementations of [Self::unify] and friends.
/// If we're building a [Substitutor] wrapper that includes more data (like delayed constraints for instance), then
/// going through the trouble with &mut refs is not worth it. Passing it along the call stack is also no bueno,
/// we'd have to pass the unifier itself, plus whatever extra data the user wants to attach to it. Lots of complexity for nothing.
///
/// Times we've been through the `&mut Substitutor` dead-end thus far: 3
pub struct Substitutor<'s, T: Debug> {
    /// Care must be taken to never hold a substitutor RefMut across a recursive call.
    substitutor: RefCell<SubstitutorIntern<'s, T>>,
}

struct SubstitutorIntern<'s, T: Debug>(Vec<SubstitutorElem<'s, T>>);

impl<'s, T: Debug> Substitutor<'s, T> {
    pub fn new() -> Self {
        Self {
            substitutor: RefCell::new(SubstitutorIntern(Vec::new())),
        }
    }
}

impl<'s, T: Debug> SubstitutorIntern<'s, T> {
    /// Creates a new substitution map that points to the passed-in object. The passed-in object must be [Interior::Unallocated].
    /// Edits the passed-in object to also point to the newly created ID.
    /// Returns the ID of this map.
    fn alloc(&mut self, point_to: &'s UniCell<T>, obj: &UniCell<T>) -> usize {
        let idx = self.0.len();
        self.0.push(SubstitutorElem {
            substitute_to: point_to,
            constraint_waiting_for: None,
        });
        obj.set_interior(None, Interior::SubstitutesTo(idx));
        idx
    }

    /// Implements the Union-Find algorithm.
    ///
    /// Resolves a possibly extensive chain of substitutions to a single node.
    ///
    /// Returns:
    /// - &'s T [Interior::Known] if known
    /// - the last [UniCell] in the chain
    /// - the last ID in the chain (corresponds to this [UniCell]). If no Known is given, then this is the final substitution ID that can be pointed to.
    fn resolve_chain(&mut self, mut ptr_target: usize) -> (Option<&'s T>, &'s UniCell<T>, usize) {
        loop {
            let target = &self.0[ptr_target];

            match target.substitute_to.get_interior() {
                Ok(known) => break (Some(known), target.substitute_to, ptr_target),
                Err(targets_target) => {
                    // A UniCell that's been accepted into the substitution list will never be Unallocated.
                    let targets_target = targets_target.unwrap();
                    if targets_target == ptr_target {
                        break (None, target.substitute_to, ptr_target);
                    } else {
                        ptr_target = targets_target;
                    }
                }
            }
        }
    }

    /// Walks the substitution chain until either finding a Known value (returned on [Ok()]), or the substitution [Err()] ID (if it exists))
    fn try_get<'out, 'in_cell>(
        &mut self,
        obj: &'in_cell UniCell<T>,
    ) -> (Result<&'out T, Option<usize>>, &'out UniCell<T>)
    where
        's: 'out,
        'in_cell: 'out,
    {
        match obj.get_interior() {
            Ok(known) => (Ok(known), obj),
            Err(Some(id)) => {
                let (chain_resolution, last, last_id) = self.resolve_chain(id);
                if let Some(chain_resolution) = chain_resolution {
                    (Ok(chain_resolution), last)
                } else {
                    (Err(Some(last_id)), last)
                }
            }
            Err(None) => (Err(None), obj),
        }
    }
    fn add_constraints(&mut self, id: usize, mut constraint: Box<DelayedConstraint<'s>>) {
        let working_on = &mut self.0[id];

        let old = working_on.constraint_waiting_for.take();
        let mut walking_constraint_ptr = &mut constraint.next;
        while let Some(walk) = walking_constraint_ptr {
            walking_constraint_ptr = &mut walk.next;
        }
        std::mem::replace(walking_constraint_ptr, old);
        working_on.constraint_waiting_for = Some(constraint);
    }
}

#[derive(Debug, Clone, Copy)]
pub struct SubTree(usize);

fn retry_constraints<'s>(mut constraints: Option<Box<DelayedConstraint<'s>>>) {
    while let Some(mut c) = constraints {
        constraints = c.next.take(); // Already separate the rest of the constraints from this one, so we re-add it individually
        if let Err(not_found_var) = (c.f)() {
            // May be a not_found_var from a different Substitutor
            not_found_var.add_delayed_constraint(c);
        }
    }
}
pub trait Unifier<'s, T: Debug + Clone + 's> {
    fn get_substitutor(&self) -> &Substitutor<'s, T>;
    fn unify_subtrees(&self, a: &'s T, b: &'s T) -> Result<(), UnifyError>;
    fn set_subtrees(&self, a: &'s T, b: T) -> Result<(), UnifyError>;
    fn contains_subtree(&self, in_obj: &T, subtree: SubTree) -> bool;
    fn fully_substitute_recurse(&self, known: &'s T) -> Result<(), ResolutionError<'s>>;

    /// `unify_subtrees` should recursively call [Substitutor::unify] for every pair of subtrees.
    /// If some irreconcilable difference is found it should return [UnifyError::Failure].
    /// Otherwise return the binary AND of subtree unifications.
    /// Regardless of failure, all subtrees should be unified for best possible type error information.
    /// You as the user should never return [UnifyError::FailureInfiniteTypes]
    /// `contains_subtree` is used to prevent infinite types.
    /// It must be implemented using [Substitutor::resolve_substitution_chain] to iterate through its subtrees.
    /// If a subtree is found that contains the given pointer it must return true.
    fn unify(&self, a: &'s UniCell<T>, b: &'s UniCell<T>) -> Result<(), UnifyError> {
        let subs = &self.get_substitutor().substitutor;
        let mut subs_borrow = subs.borrow_mut();

        match (subs_borrow.try_get(a), subs_borrow.try_get(b)) {
            ((Ok(a), _), (Ok(b), _)) => {
                std::mem::drop(subs_borrow);
                // Simple optimization. Unification will often create referential identity.
                if std::ptr::eq(a, b) {
                    Ok(())
                } else {
                    self.unify_subtrees(a, b)
                }
            }
            ((Ok(known), known_cell), (Err(Some(var_id)), _))
            | ((Err(Some(var_id)), _), (Ok(known), known_cell)) => {
                std::mem::drop(subs_borrow); // contains_subtree will need its own mutable borrows
                if self.contains_subtree(known, SubTree(var_id)) {
                    // Always have to check contains_subtree. Could be that a contains b which was uninit
                    return Err(UnifyError::FailureInfiniteTypes);
                }
                let mut subs_borrow = subs.borrow_mut();
                let removing_var = &mut subs_borrow.0[var_id];
                removing_var.substitute_to = known_cell;
                let constraints = removing_var.constraint_waiting_for.take();
                std::mem::drop(subs_borrow);
                retry_constraints(constraints);
                Ok(())
            }
            ((Ok(_known), known_cell), (Err(None), unknown_cell))
            | ((Err(None), unknown_cell), (Ok(_known), known_cell)) => {
                let unknown_id = subs_borrow.alloc(unknown_cell, unknown_cell);
                // New var cannot already have constraints attached to it.
                subs_borrow.0[unknown_id].substitute_to = known_cell;
                Ok(())
            }
            ((Err(Some(a_id)), a_cell), (Err(Some(b_id)), _)) => {
                let b = &mut subs_borrow.0[b_id];
                let constraints_to_move = b.constraint_waiting_for.take();
                b.substitute_to = a_cell;
                if let Some(constraints_to_move) = constraints_to_move {
                    subs_borrow.add_constraints(a_id, constraints_to_move);
                }
                Ok(())
            }
            ((Err(Some(id)), _), (Err(None), unalloc_cell))
            | ((Err(None), unalloc_cell), (Err(Some(id)), _)) => {
                unalloc_cell.set_interior(None, Interior::SubstitutesTo(id));
                Ok(())
            }
            ((Err(None), a_cell), (Err(None), b_cell)) => {
                let a_id = subs_borrow.alloc(a_cell, a_cell);
                b_cell.set_interior(None, Interior::SubstitutesTo(a_id));
                Ok(())
            }
        }
    }
    fn set(&self, cell: &'s UniCell<T>, to: UniCell<T>) -> Result<(), UnifyError> {
        let subs = &self.get_substitutor().substitutor;
        match to.0.into_inner() {
            Interior::Known(to) => match cell.get_interior() {
                Ok(known) => self.set_subtrees(known, to),
                Err(Some(id)) => {
                    let mut subs_borrow = subs.borrow_mut();
                    let (known, last_cell, last_id) = subs_borrow.resolve_chain(id);
                    std::mem::drop(subs_borrow);
                    if let Some(known) = known {
                        self.set_subtrees(known, to)
                    } else if self.contains_subtree(&to, SubTree(id)) {
                        Err(UnifyError::FailureInfiniteTypes)
                    } else {
                        let mut subs_borrow = subs.borrow_mut();
                        let constraints = subs_borrow.0[id].constraint_waiting_for.take();
                        std::mem::drop(subs_borrow);
                        retry_constraints(constraints);
                        last_cell.set_interior(Some(last_id), Interior::Known(to));
                        Ok(())
                    }
                }
                Err(None) => {
                    cell.set_interior(None, Interior::Known(to));
                    Ok(())
                }
            },
            Interior::SubstitutesTo(to_id) => {
                let mut subs_borrow = subs.borrow_mut();
                let (_known, last_cell, _last_id) = subs_borrow.resolve_chain(to_id);
                std::mem::drop(subs_borrow);
                self.unify(cell, last_cell)
            }
            // Unifying with an anonymous variable always succeeds, of course
            Interior::Unallocated => Ok(()),
        }
    }
    /// Shorthand for creating a [UniCell::UNKNOWN], and then [Self::unify]-ing with `obj`.
    ///
    /// For clones after successful typechecking, use the regular [std::clone::Clone]
    ///
    /// For clones that *don't* unify type variables, use [UniCell::clone_prototype_step]
    fn clone_unify(&self, to_clone: &'s UniCell<T>) -> UniCell<T> {
        let mut subs_borrow = self.get_substitutor().substitutor.borrow_mut();
        match to_clone.get_interior() {
            Ok(_known) => {
                let new_cell = UniCell::UNKNOWN;
                let _id = subs_borrow.alloc(to_clone, &new_cell);
                new_cell
            }
            Err(Some(id)) => UniCell(UnsafeCell::new(Interior::SubstitutesTo(id))),
            Err(None) => {
                let id = subs_borrow.alloc(to_clone, to_clone);
                UniCell(UnsafeCell::new(Interior::SubstitutesTo(id)))
            }
        }
    }

    /// Walks the substitution chains to determine if it ends in a Known. If it does, then it clones the Known value into `obj` using the provided clone function.
    ///
    /// Use this for resolving delayed constraints ([delayed_constraint]), and to implement `resolve`
    fn resolve(&self, obj: &'s UniCell<T>) -> Result<&'s T, ResolutionError<'s>> {
        let subs = self.get_substitutor();
        let mut subs_borrow = subs.substitutor.borrow_mut();
        match obj.get_interior() {
            Ok(known) => Ok(known),
            Err(Some(id)) => {
                let (known, _last, id) = subs_borrow.resolve_chain(id);
                if let Some(known) = known {
                    Ok(known)
                } else {
                    Err(ResolutionError { subs, id })
                }
            }
            Err(None) => {
                // We must have a valid substitution table entry, to be able to add constraints to it.
                let id = subs_borrow.alloc(obj, obj);
                Err(ResolutionError { subs, id })
            }
        }
    }

    fn contains_subtree_recurse(&self, obj: &UniCell<T>, subtree: SubTree) -> bool {
        let subs = self.get_substitutor();
        let mut subs_borrow = subs.substitutor.borrow_mut();
        match subs_borrow.try_get(obj).0 {
            Ok(known) => {
                std::mem::drop(subs_borrow);
                self.contains_subtree(known, subtree)
            }
            Err(Some(id)) => id == subtree.0,
            Err(None) => false,
        }
    }

    fn fully_substitute(&self, obj: &'s UniCell<T>) -> Result<&'s T, ResolutionError<'s>> {
        let subs = self.get_substitutor();
        match obj.get_interior() {
            Ok(known) => {
                self.fully_substitute_recurse(known)?;
                Ok(known)
            }
            Err(Some(id)) => {
                let mut subs_borrow = subs.substitutor.borrow_mut();
                let (known, _last_cell, last_id) = subs_borrow.resolve_chain(id);
                std::mem::drop(subs_borrow);
                if let Some(known) = known {
                    self.fully_substitute_recurse(known)?;
                    // At this point it's safe to clone, because known (should) have no more type variables.
                    obj.set_interior(Some(id), Interior::Known(known.clone()));
                    Ok(obj.unwrap())
                } else {
                    Err(ResolutionError { subs, id: last_id })
                }
            }
            Err(None) => {
                let mut subs_borrow = subs.substitutor.borrow_mut();
                let id = subs_borrow.alloc(obj, obj);
                Err(ResolutionError { subs, id })
            }
        }
    }
}

pub fn delayed_constraint<'s>(mut f: impl FnMut() -> Result<(), ResolutionError<'s>> + 's) {
    if let Err(not_found_var) = f() {
        // May be a not_found_var from a different Substitutor
        not_found_var.add_delayed_constraint(Box::new(DelayedConstraint { next: None, f }));
    }
}

/// Fancy trick! [Rust Forum](https://users.rust-lang.org/t/creating-a-dst-struct-with-a-dyn-fnmut/137256/3).
/// The trick is, that there is a subtyping coersion `MyStruct<impl Trait>` -> `MyStruct<dyn Trait>`.
/// The type we like to hold is `DelayedConstraint<dyn FnMut...>`,
/// but to create it, we have to pass through types with concrete implementations. (`DelayedConstraint<impl FnMut...>`)
/// Very tricky
struct DelayedConstraint<'s, F = dyn FnMut() -> Result<(), ResolutionError<'s>> + 's>
where
    F: ?Sized,
{
    next: Option<Box<DelayedConstraint<'s>>>,
    f: F,
}

impl<'s> DelayedConstraint<'s> {
    fn count(mut cur: &Option<Box<Self>>) -> usize {
        let mut total = 0;
        while let Some(nested) = cur {
            total += 1;
            cur = &nested.next;
        }
        total
    }
}

trait DelayedConstraintAcceptor<'s> {
    fn add_delayed_constraint(&self, id: usize, constraint: Box<DelayedConstraint<'s>>);
}
#[derive(Debug)]
pub struct ResolutionError<'s> {
    subs: *const (dyn DelayedConstraintAcceptor<'s> + 's),
    id: usize,
}

impl<'s> ResolutionError<'s> {
    fn add_delayed_constraint(self, constraint: Box<DelayedConstraint<'s>>) {
        // SAFETY: We use a *const dyn to not have to specify the self-referential lifetime to the substitutor
        // Since DelayedConstraints are immediately consumed upon creation, or are stored inside the
        // Substitutor that last failed to resolve a variable. And well, since its stored inside the substitutor,
        // the substitutor will still exist.
        unsafe {
            (*self.subs).add_delayed_constraint(self.id, constraint);
        }
    }
}

impl<'s, T: Debug> DelayedConstraintAcceptor<'s> for Substitutor<'s, T> {
    fn add_delayed_constraint(&self, id: usize, constraint: Box<DelayedConstraint<'s>>) {
        let mut subs = self.substitutor.borrow_mut();
        subs.add_constraints(id, constraint);
    }
}

impl<'s, T: Clone + Debug> Default for Substitutor<'s, T> {
    fn default() -> Self {
        Self::new()
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

#[derive(Debug)]
struct PeanoUnifier<'s> {
    substitutor: Substitutor<'s, PeanoType>,
}

impl<'s> PeanoUnifier<'s> {
    pub fn new() -> Self {
        Self {
            substitutor: Substitutor::new(),
        }
    }
}

impl<'s> Unifier<'s, PeanoType> for PeanoUnifier<'s> {
    fn get_substitutor(&self) -> &Substitutor<'s, PeanoType> {
        &self.substitutor
    }
    fn unify_subtrees(&self, a: &'s PeanoType, b: &'s PeanoType) -> Result<(), UnifyError> {
        match (a, b) {
            (PeanoType::Zero, PeanoType::Zero) => Ok(()),
            (PeanoType::Succ(a), PeanoType::Succ(b)) => self.unify(a, b),
            _ => Err(UnifyError::Failure),
        }
    }
    fn set_subtrees(&self, a: &'s PeanoType, b: PeanoType) -> Result<(), UnifyError> {
        match (a, b) {
            (PeanoType::Zero, PeanoType::Zero) => Ok(()),
            (PeanoType::Succ(a), PeanoType::Succ(b)) => self.set(a, *b),
            _ => Err(UnifyError::Failure),
        }
    }
    fn contains_subtree(&self, in_obj: &PeanoType, subtree: SubTree) -> bool {
        match in_obj {
            PeanoType::Zero => false,
            PeanoType::Succ(succ) => self.contains_subtree_recurse(succ, subtree),
        }
    }
    fn fully_substitute_recurse(&self, known: &'s PeanoType) -> Result<(), ResolutionError<'s>> {
        match known {
            PeanoType::Zero => Ok(()),
            PeanoType::Succ(succ) => {
                self.fully_substitute(succ)?;
                Ok(())
            }
        }
    }
}

impl UniCell<PeanoType> {
    pub fn clone_prototype(&self) -> Self {
        self.clone_prototype_step(|known| match known {
            PeanoType::Zero => PeanoType::Zero,
            PeanoType::Succ(uni_cell) => PeanoType::Succ(Box::new(uni_cell.clone_prototype())),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// up_to must be > 0
    fn add_to(to: UniCell<PeanoType>, amount: usize) -> PeanoType {
        assert!(amount > 0);
        let mut cur = PeanoType::Succ(Box::new(to));
        for _ in 1..amount {
            cur = PeanoType::Succ(Box::new(cur.into()));
        }

        cur
    }
    /// up_to must be >= 1
    fn add_to_cell(to: UniCell<PeanoType>, amount: usize) -> UniCell<PeanoType> {
        if amount == 0 {
            to
        } else {
            add_to(to, amount).into()
        }
    }
    fn mk_peano(up_to: usize) -> PeanoType {
        if up_to == 0 {
            PeanoType::Zero
        } else {
            add_to(PeanoType::Zero.into(), up_to)
        }
    }
    fn mk_peano_cell(up_to: usize) -> UniCell<PeanoType> {
        mk_peano(up_to).into()
    }

    #[test]
    fn test_peano_initial() {
        let a = mk_peano_cell(4);
        let b = mk_peano_cell(2);
        let b_clone = b.clone();

        let unknown = PeanoType::UNKNOWN;

        unknown.set_initial(PeanoType::Zero);

        assert_eq!(a.unwrap().count(), 4);
        assert_eq!(b.unwrap().count(), 2);
        assert_eq!(b_clone.unwrap().count(), 2);
        assert_eq!(unknown.unwrap().count(), 0);
    }
    #[test]
    fn test_peano_unify() {
        let four = mk_peano_cell(4);

        let a = PeanoType::UNKNOWN;
        let three_plus_a = PeanoType::UNKNOWN;

        let substitutor = PeanoUnifier::new();
        substitutor
            .set(&three_plus_a, add_to_cell(substitutor.clone_unify(&a), 3))
            .unwrap();

        substitutor.unify(&four, &three_plus_a).unwrap();

        substitutor.fully_substitute(&a).unwrap();

        assert_eq!(a.unwrap().count(), 1)
    }

    #[test]
    fn test_non_infinite_peano() {
        let a = PeanoType::UNKNOWN;
        let a_plus_zero = PeanoType::UNKNOWN;

        let substitutor = PeanoUnifier::new();
        substitutor
            .set(&a_plus_zero, add_to_cell(substitutor.clone_unify(&a), 0))
            .unwrap();

        substitutor.unify(&a, &a_plus_zero).unwrap();
        substitutor.unify(&a_plus_zero, &a).unwrap();

        // a and a_plus_zero should both still have a type variable.
        assert!(substitutor.fully_substitute(&a).is_err());
        assert!(substitutor.fully_substitute(&a_plus_zero).is_err());
    }

    #[test]
    fn test_invalid_unification() {
        let three = mk_peano_cell(3);
        let four = mk_peano_cell(4);
        let substitutor = PeanoUnifier::new();

        assert_eq!(substitutor.unify(&three, &four), Err(UnifyError::Failure));
        assert_eq!(substitutor.unify(&four, &three), Err(UnifyError::Failure));

        dbg!(&substitutor, &three, &four);

        substitutor.fully_substitute(&three).unwrap();
        substitutor.fully_substitute(&four).unwrap();
    }

    #[test]
    fn test_infinite_peano() {
        let a = PeanoType::UNKNOWN;
        let a_plus_one = PeanoType::UNKNOWN;

        let substitutor = PeanoUnifier::new();
        substitutor
            .set(&a_plus_one, add_to_cell(substitutor.clone_unify(&a), 1))
            .unwrap();

        // Both of these try to unify a = a + 1, which would lead to an infinite tower of +1s
        assert_eq!(
            substitutor.unify(&a, &a_plus_one),
            Err(UnifyError::FailureInfiniteTypes)
        );
        assert_eq!(
            substitutor.unify(&a_plus_one, &a),
            Err(UnifyError::FailureInfiniteTypes)
        );

        assert!(substitutor.fully_substitute(&a).is_err());
        assert!(substitutor.fully_substitute(&a_plus_one).is_err());
    }

    #[test]
    fn test_peano_equivalence_simple() {
        let one = mk_peano_cell(1);
        let two = mk_peano_cell(2);
        let one_plus_three = PeanoType::UNKNOWN;
        let two_plus_two = PeanoType::UNKNOWN;

        let substitutor = PeanoUnifier::new();
        substitutor
            .set(
                &one_plus_three,
                add_to_cell(substitutor.clone_unify(&one), 3),
            )
            .unwrap();
        substitutor
            .set(&two_plus_two, add_to_cell(substitutor.clone_unify(&two), 2))
            .unwrap();
        // 2+2 == 1+3
        substitutor.unify(&two_plus_two, &one_plus_three).unwrap();
    }

    #[test]
    fn test_peano_multiple_variables_chain() {
        let x = PeanoType::UNKNOWN;
        let y = PeanoType::UNKNOWN;
        let z = PeanoType::UNKNOWN;

        // x = 2, y = x + 1, z = y + 1
        x.set_initial(PeanoType::Zero);

        let x_plus_2 = PeanoType::UNKNOWN;
        let y_val = PeanoType::UNKNOWN;
        let z_val = PeanoType::UNKNOWN;

        let substitutor = PeanoUnifier::new();
        substitutor
            .set(&x_plus_2, add_to_cell(substitutor.clone_unify(&x), 2))
            .unwrap();
        substitutor
            .set(&y_val, add_to_cell(substitutor.clone_unify(&x), 1))
            .unwrap();
        substitutor
            .set(&z_val, add_to_cell(substitutor.clone_unify(&y), 1))
            .unwrap();

        // Unify y with x+1, z with y+1, and z with x+2
        substitutor.unify(&y, &y_val).unwrap();
        substitutor.unify(&z, &z_val).unwrap();
        substitutor.unify(&z, &x_plus_2).unwrap();

        substitutor.fully_substitute(&x).unwrap();
        substitutor.fully_substitute(&y).unwrap();
        substitutor.fully_substitute(&z).unwrap();

        assert_eq!(x.unwrap().count(), 0);
        assert_eq!(y.unwrap().count(), 1);
        assert_eq!(z.unwrap().count(), 2);
    }

    #[test]
    fn test_peano_complex_substitution_graph() {
        let a = PeanoType::UNKNOWN;
        let b = PeanoType::UNKNOWN;
        let c = PeanoType::UNKNOWN;

        // a = 2, b = a + 2, c = b + 1
        a.set_initial(PeanoType::Zero);
        let b_val = PeanoType::UNKNOWN;
        let c_val = PeanoType::UNKNOWN;

        let substitutor = PeanoUnifier::new();
        substitutor
            .set(&b_val, add_to_cell(substitutor.clone_unify(&a), 2))
            .unwrap();
        substitutor
            .set(&c_val, add_to_cell(substitutor.clone_unify(&b), 1))
            .unwrap();

        substitutor.unify(&b, &b_val).unwrap();
        substitutor.unify(&c, &c_val).unwrap();

        substitutor.fully_substitute(&a).unwrap();
        substitutor.fully_substitute(&b).unwrap();
        substitutor.fully_substitute(&c).unwrap();

        assert_eq!(a.unwrap().count(), 0);
        assert_eq!(b.unwrap().count(), 2);
        assert_eq!(c.unwrap().count(), 3);
    }

    /// Just a stress test to cover all possible code paths. To check under miri that everything is alright.
    #[test]
    fn stress_test_for_miri() {
        use rand::prelude::IndexedRandom;
        use rand::{Rng, SeedableRng};

        let mut rng = rand::rngs::StdRng::seed_from_u64(42);

        // Create a bunch of unknowns
        let cells: Vec<UniCell<PeanoType>> = (0..1000).map(|_| PeanoType::UNKNOWN).collect();

        let substitutor = PeanoUnifier::new();

        // Randomly set some initial values
        for cell in cells.iter().take(100) {
            cell.set_initial(mk_peano(rng.random_range(0..5)));
        }

        for _ in 0..1000 {
            match rng.random_range(0..3) {
                0 => {
                    // Add a computed successor
                    let ontu = cells.choose(&mut rng).unwrap();
                    let add_count = rng.random_range(0..5);
                    let new_cell = add_to_cell(substitutor.clone_unify(ontu), add_count);
                    // May fail, may not fail
                    let _ = substitutor.set(cells.choose(&mut rng).unwrap(), new_cell);
                }
                1 => {
                    // Unify two peanos
                    let a = cells.choose(&mut rng).unwrap();
                    let b = cells.choose(&mut rng).unwrap();

                    // May fail, may not fail
                    let _ = substitutor.unify(a, b);
                }
                2 => {
                    // Fully substitute something
                    let a = cells.choose(&mut rng).unwrap();

                    if substitutor.fully_substitute(a).is_ok() {
                        // Can clone values after a successful substitute
                        let _a_clone = a.clone();
                    }
                }
                _ => unreachable!(),
            }
        }
    }
}
