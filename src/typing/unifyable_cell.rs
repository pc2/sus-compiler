#![allow(dead_code)]

//! This file contains the final Unifier I make. It should have all the features I'll ever need.
//!
//! Important types:
//! - [UniCell]: The wrapper type for your types. This Cell can be unified with others. It can also be [Unifier::fully_substitute]d from a shared ref.
//! - [Substitutor]: A substitutor responsible for one type of substitutions. A single [UnifierTop] implementor can have multiple [Substitutor]s.
//! - [Unifier]: Trait the user should implement for each [Substitutor] they wish to use.
//! - [UnifierTop]: This trait should be implemented once for any unifier you wish to create.
//!
//! An example type hierarchy for your custom unifier would then be:
//! ```
//! /// This Unifier unifies structures containing both `UniCell<PeanoType>` and `UniCell<DomainType>`
//! struct MyUnifier<'s> {
//!     peano_subs : Substitutor<'s, PeanoType>,
//!     domain_subs : Substitutor<'s, DomainType>,
//! }
//!
//! impl<'s> UnifierTop for MyUnifier<'s> {}
//! impl<'slf, 's> Unifier<'slf, 's, PeanoType> for MyUnifier<'s> {}
//! impl<'slf, 's> Unifier<'slf, 's, DomainType> for MyUnifier<'s> {}
//! ```

use crate::prelude::*;

use std::{
    cell::{Cell, RefCell, UnsafeCell},
    fmt::Debug,
    marker::PhantomData,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UnifyError {
    Failure,
    FailureInfiniteTypes,
}

/// Basically a [std::cell::OnceCell] for type checking. We implement it safely by maintaining the following invariant:
///
/// - [UniCell] starts out [UniCell::UNKNOWN]. No interior references can be taken in this state. (But the type variable we refer to *can* be updated)
/// - At some point, it is set to [Interior::Known]. After this point references to this interior value can be taken.
///   Afterwards, we can *never* reset a [Interior::Known] back to an [Interior::Unallocated] or [Interior::SubstitutesTo], or mess with it in any mutable way. (Panics when trying otherwise)
///
/// A "Prototype" refers to a [UniCell] that has thus far not been touched by a [Substitutor]. [UniCell::UNKNOWN] is a valid prototype.
/// A [UniCell] ceases to be a "Prototype" when
pub struct UniCell<T>(UnsafeCell<Interior<T>>);
enum Interior<T> {
    Known(T),
    /// If no substitution is known yet, then this points to itself ([SubstitutorInterior::resolve_chain] walks these until it finds a [UniCell] substituting to itself).
    SubstitutesTo(usize),
    /// Default state of a new Type Variable. This means the variable is *unique*, and so we don't yet need an ID to track its Unification.
    /// CANNOT BE CLONED (panics)
    /// When [UniCell] is [Interior::Unallocated], no [Substitutor] references can exist to it yet.
    Unallocated,
}

impl<T: Debug> UniCell<T> {
    #[allow(clippy::declare_interior_mutable_const)]
    pub const UNKNOWN: Self = Self(UnsafeCell::new(Interior::Unallocated));

    /// Either get a shared reference to the known value if it's set, or a mutable reference to the whole thing if it's not yet known
    /// This is safe, because [UniCell] only allows references to [Interior::Known] once it is set, and it can never be unset through a shared ref
    fn get_interior(&self) -> Result<&T, Option<usize>> {
        // SAFETY: See [UniCell]'s definition
        unsafe {
            let interior_ptr: *mut Interior<T> = self.0.get();
            match &*(interior_ptr as *const Interior<T>) {
                Interior::Known(known) => Ok(known),
                Interior::SubstitutesTo(id) => Err(Some(*id)),
                Interior::Unallocated => Err(None),
            }
        }
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

        // SAFETY: We already know we're not Interior::Known, See [UniCell]'s definition
        unsafe { *self.0.get() = v };
    }

    #[track_caller]
    pub fn unwrap(&self) -> &T {
        self.get_interior().unwrap()
    }
    #[track_caller]
    pub fn unwrap_mut(&mut self) -> &mut T {
        let_unwrap!(Interior::Known(v), self.0.get_mut());
        v
    }
    pub fn into_inner(self) -> T {
        let Interior::Known(v) = self.0.into_inner() else {
            unreachable!("UniCell::into_inner on not a Interior::Known");
        };
        v
    }

    /// `self` must be [UniCell::UNKNOWN]
    pub fn set_initial(&self, v: T) {
        self.set_interior(None, Interior::Known(v));
    }

    /// `self` must be [UniCell::UNKNOWN]
    pub fn set_initial_cell(&self, v: UniCell<T>) {
        self.set_interior(None, v.0.into_inner());
    }

    /// Used to clone types that have been created with several [UniCell::UNKNOWN]s.
    /// The cloned [UniCell::UNKNOWN]s become distinct type variables.
    ///
    /// For clones after successful typechecking, use the regular [std::clone::Clone]
    ///
    /// For clones that *do* unify type variables, use [Unifier::clone_unify]
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

struct SubstitutorElem<'s, T: Debug, Unif: UnifierTop> {
    substitute_to: &'s UniCell<T>,
    constraint_waiting_for: Option<Box<DelayedConstraint<'s, Unif>>>,
}
impl<'s, T: Debug, Unif: UnifierTop> Debug for SubstitutorElem<'s, T, Unif> {
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
impl<'s, T: Debug, Unif: UnifierTop> Debug for Substitutor<'s, T, Unif> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut list = f.debug_list();
        for (idx, e) in self.substitutor.borrow().substitutions.iter().enumerate() {
            list.entry(&(idx, e));
        }
        Ok(())
    }
}

/// This struct bookkeeps the extra state for a Hindley Mindley Union-Find algorithm. It contains the counterparts to [UniCell]'s [Interior::SubstitutesTo]'s ID field.
/// All references are to [UniCell]s in the field. If a new value needs to be injected into the graph of [UniCell]s, then it should be [Unifier::set].
///
/// For usage, see [Unifier]
pub struct Substitutor<'s, T: Debug, Unif: UnifierTop> {
    /// Care must be taken to never hold a substitutor RefMut across a recursive call.
    substitutor: RefCell<SubstitutorInterior<'s, T, Unif>>,
    ready_constraints: Cell<Option<Box<DelayedConstraint<'s, Unif>>>>,
}

struct SubstitutorInterior<'s, T: Debug, Unif: UnifierTop> {
    substitutions: Vec<SubstitutorElem<'s, T, Unif>>,
}

impl<'s, T: Debug, Unif: UnifierTop> Drop for Substitutor<'s, T, Unif> {
    fn drop(&mut self) {
        if !std::thread::panicking() && self.ready_constraints.take().is_some() {
            panic!(
                "Substitutor dropped while still holding Ready Constraints! These should have been resolved using [Unifier::execute_ready_delayed_constraints]"
            );
        }
    }
}

impl<'s, T: Debug, Unif: UnifierTop> Substitutor<'s, T, Unif> {
    pub fn new() -> Self {
        Self {
            substitutor: RefCell::new(SubstitutorInterior {
                substitutions: Vec::new(),
            }),
            ready_constraints: Cell::new(None),
        }
    }

    fn add_ready_constraints(&self, constraints: Option<Box<DelayedConstraint<'s, Unif>>>) {
        let long_list = self.ready_constraints.take();
        self.ready_constraints
            .set(DelayedConstraint::add_to_list(long_list, constraints));
    }
}

impl<'s, T: Debug, Unif: UnifierTop> SubstitutorInterior<'s, T, Unif> {
    /// Creates a new substitution map that points to the passed-in object. The passed-in object must be [Interior::Unallocated].
    /// Edits the passed-in object to also point to the newly created ID.
    /// Returns the ID of this map.
    fn alloc(&mut self, point_to: &'s UniCell<T>, obj: &UniCell<T>) -> usize {
        let idx = self.substitutions.len();
        self.substitutions.push(SubstitutorElem {
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
            let target = &self.substitutions[ptr_target];

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

    fn add_constraints(
        &mut self,
        id: usize,
        constraints: Option<Box<DelayedConstraint<'s, Unif>>>,
    ) {
        let working_on = &mut self.substitutions[id];

        let old = working_on.constraint_waiting_for.take();
        working_on.constraint_waiting_for = DelayedConstraint::add_to_list(old, constraints);
    }
}

#[derive(Debug, Clone, Copy)]
pub struct SubTree<T>(usize, PhantomData<T>);

/// To use, you should implement:
/// - [Unifier::get_substitutor]
/// - [Unifier::unify_subtrees]
/// - [Unifier::set_subtrees]
/// - [Unifier::contains_subtree]
/// - [Unifier::fully_substitute_recurse]
///
/// The trait will then give you access to the following methods you can use to use:
/// - [UniCell::clone_prototype_step]
/// - [Unifier::unify]
/// - [Unifier::set]
/// - [Unifier::clone_unify]
/// - [Unifier::resolve]
/// - [UnifierTop::delayed_constraint]
/// - [Unifier::execute_ready_delayed_constraints]
///
/// For examples see [PeanoUnifier]
///
/// [Substitutor] references are *shared* on purpose (I've tried to replace them with &mut many times before).
/// The reason is that shared refs allow for more ergonomic recursive implementations of [Unifier::unify] and friends.
/// If we're building a [Substitutor] wrapper that includes more data (like delayed constraints for instance), then
/// going through the trouble with &mut refs is not worth it. Passing it along the call stack is also no bueno,
/// we'd have to pass the unifier itself, plus whatever extra data the user wants to attach to it. Lots of complexity for nothing.
///
/// Times we've been through the `&mut Substitutor` dead-end thus far: 3
pub trait Unifier<'slf, 's: 'slf, T: Debug + Clone + 's>: UnifierTop + Sized + 's {
    /// You should declare a [Substitutor] field for each [UniCell]`<T>` you wish to support. Return it here.
    fn get_substitutor(&'slf self) -> &'slf Substitutor<'s, T, Self>;
    /// Recursively call [Unifier::unify] on every contained [UniCell]`<T>`
    ///
    /// Should check if two Knowns are the same (if not, return [UnifyError::Failure]),
    /// If they are the same, then call [Unifier::unify] recursively.
    fn unify_subtrees(&'slf self, a: &'s T, b: &'s T) -> Result<(), UnifyError>;
    /// Owning variant of [Unifier::unify_subtrees]
    ///
    /// Recursively call [Unifier::set_cell] on every contained [UniCell]`<T>`
    ///
    /// Should check if two Knowns are the same (if not, return [UnifyError::Failure]),
    /// If they are the same, then call [Unifier::set_cell] recursively.
    fn set_subtrees(&'slf self, a: &'s T, b: T) -> Result<(), UnifyError>;
    /// Recursively call [Unifier::contains_subtree_recurse] on every contained [UniCell]`<T>`
    fn contains_subtree(&'slf self, in_obj: &T, subtree: SubTree<T>) -> bool;
    /// Recursively call [Unifier::fully_substitute] on every contained [UniCell]`<T>`
    fn fully_substitute_recurse(
        &'slf self,
        known: &'s T,
    ) -> Result<(), ResolutionError<'slf, 's, Self>>;

    /// `unify_subtrees` should recursively call [Unifier::unify] for every pair of subtrees.
    /// If some irreconcilable difference is found it should return [UnifyError::Failure].
    /// Otherwise return the binary AND of subtree unifications.
    /// Regardless of failure, all subtrees should be unified for best possible type error information.
    /// You as the user should never return [UnifyError::FailureInfiniteTypes]
    /// `contains_subtree` is used to prevent infinite types.
    /// It must be implemented using [SubstitutorInterior::resolve_chain] to iterate through its subtrees.
    /// If a subtree is found that contains the given pointer it must return true.
    fn unify(&'slf self, a: &'s UniCell<T>, b: &'s UniCell<T>) -> Result<(), UnifyError> {
        let subs = self.get_substitutor();
        let mut subs_borrow = subs.substitutor.borrow_mut();

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
                if self.contains_subtree(known, SubTree(var_id, PhantomData)) {
                    // Always have to check contains_subtree. Could be that a contains b which was uninit
                    return Err(UnifyError::FailureInfiniteTypes);
                }
                let mut subs_borrow = subs.substitutor.borrow_mut();
                let removing_var = &mut subs_borrow.substitutions[var_id];
                removing_var.substitute_to = known_cell;
                let constraints = removing_var.constraint_waiting_for.take();
                subs.add_ready_constraints(constraints);
                Ok(())
            }
            ((Ok(_known), known_cell), (Err(None), unknown_cell))
            | ((Err(None), unknown_cell), (Ok(_known), known_cell)) => {
                let unknown_id = subs_borrow.alloc(unknown_cell, unknown_cell);
                // New var cannot already have constraints attached to it.
                subs_borrow.substitutions[unknown_id].substitute_to = known_cell;
                Ok(())
            }
            ((Err(Some(a_id)), a_cell), (Err(Some(b_id)), _)) => {
                let b = &mut subs_borrow.substitutions[b_id];
                let constraints_to_move = b.constraint_waiting_for.take();
                b.substitute_to = a_cell;
                subs_borrow.add_constraints(a_id, constraints_to_move);
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
    /// Basically the same as [Unifier::unify], but with the second argument an owned object.
    /// This allows you to inject new Ts into the substitution graph.
    fn set_cell(&'slf self, cell: &'s UniCell<T>, to: UniCell<T>) -> Result<(), UnifyError> {
        let subs = self.get_substitutor();
        match to.0.into_inner() {
            Interior::Known(to) => match cell.get_interior() {
                Ok(known) => self.set_subtrees(known, to),
                Err(Some(id)) => {
                    let mut subs_borrow = subs.substitutor.borrow_mut();
                    let (known, last_cell, last_id) = subs_borrow.resolve_chain(id);
                    std::mem::drop(subs_borrow);
                    if let Some(known) = known {
                        self.set_subtrees(known, to)
                    } else if self.contains_subtree(&to, SubTree(last_id, PhantomData)) {
                        Err(UnifyError::FailureInfiniteTypes)
                    } else {
                        last_cell.set_interior(Some(last_id), Interior::Known(to));
                        let mut subs_borrow = subs.substitutor.borrow_mut();
                        let constraints = subs_borrow.substitutions[last_id]
                            .constraint_waiting_for
                            .take();
                        dbg!(last_id, DelayedConstraint::count(&constraints));
                        subs.add_ready_constraints(constraints);
                        Ok(())
                    }
                }
                Err(None) => {
                    cell.set_interior(None, Interior::Known(to));
                    Ok(())
                }
            },
            Interior::SubstitutesTo(to_id) => {
                let mut subs_borrow = subs.substitutor.borrow_mut();
                let (_known, last_cell, _last_id) = subs_borrow.resolve_chain(to_id);
                std::mem::drop(subs_borrow);
                self.unify(cell, last_cell)
            }
            // Unifying with an anonymous variable always succeeds, of course
            Interior::Unallocated => Ok(()),
        }
    }
    /// Wrapper around [Unifier::set_cell], for Known [UniCell]
    fn set(&'slf self, cell: &'s UniCell<T>, to: T) -> Result<(), UnifyError> {
        self.set_cell(cell, UniCell(UnsafeCell::new(Interior::Known(to))))
    }
    /// Shorthand for creating a [UniCell::UNKNOWN], and then [Unifier::unify]-ing with `obj`.
    ///
    /// For clones after successful typechecking, use the regular [std::clone::Clone]
    ///
    /// For clones that *don't* unify type variables, use [UniCell::clone_prototype_step]
    fn clone_unify(&'slf self, to_clone: &'s UniCell<T>) -> UniCell<T> {
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

    /// Walks the substitution chains to determine if it ends in a [Interior::Known]. If it does, it returns a reference to the known value.
    ///
    /// Use this for resolving dependencies in [UnifierTop::delayed_constraint]
    fn resolve(&'slf self, obj: &'s UniCell<T>) -> Result<&'s T, ResolutionError<'slf, 's, Self>> {
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

    /// Substitutes and clones the substitutions such that `obj` actually owns them.
    ///
    /// If this succeeds, then `obj` can be safely [Clone]-d.
    ///
    /// Use this for resolving dependencies in [UnifierTop::delayed_constraint]
    ///
    /// Complete this implementation by implementing [Unifier::fully_substitute_recurse]
    fn fully_substitute(
        &'slf self,
        obj: &'s UniCell<T>,
    ) -> Result<&'s T, ResolutionError<'slf, 's, Self>> {
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

    /// Used to detect infinite types and report errors on them.
    ///
    /// Complete this by implementing [Unifier::contains_subtree]
    fn contains_subtree_recurse(&'slf self, obj: &UniCell<T>, subtree: SubTree<T>) -> bool {
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

    /// See [UnifierTop::delayed_constraint]
    ///
    /// We bubble delayed constraints up to the top of the call stack,
    /// because if we immediately execute any delayed constraint that becomes ready,
    /// we might blow out the stack.
    fn execute_ready_delayed_constraints(&'slf self) {
        let subs = self.get_substitutor();
        // During of a delayed constraint, other constraints may of course become ready.
        // We retry delayed constraints in a stack-like manner, as my intuition tells me this is more efficient.
        while let Some(mut first_constraint) = subs.ready_constraints.take() {
            subs.ready_constraints.set(first_constraint.next.take());

            if let Err(resolution_err) = (first_constraint.f)(self) {
                resolution_err.add_delayed_constraint(first_constraint);
            }
        }
    }
}

pub trait UnifierTop: Sized {
    /// When using [UnifierTop::delayed_constraint],
    /// you must also call [Unifier::execute_ready_delayed_constraints] once, or multiple times.
    fn delayed_constraint<'slf2, 's: 'slf2>(
        &'slf2 self,
        mut f: impl for<'slf> FnMut(&'slf Self) -> Result<(), ResolutionError<'slf, 's, Self>> + 's,
    ) {
        if let Err(not_found_var) = f(self) {
            // May be a not_found_var from a different Substitutor
            not_found_var.add_delayed_constraint(Box::new(DelayedConstraint { next: None, f }));
        }
    }
}

/// Fancy trick! [Rust Forum - Creating a DST struct with a dyn FnMut](https://users.rust-lang.org/t/creating-a-dst-struct-with-a-dyn-fnmut/137256/3).
/// The trick is, that there is a subtyping coersion `MyStruct<impl Trait>` -> `MyStruct<dyn Trait>`.
/// The type we like to hold is `DelayedConstraint<dyn FnMut...>`,
/// but to create it, we have to pass through types with concrete implementations. (`DelayedConstraint<impl FnMut...>`)
/// Very tricky
struct DelayedConstraint<
    's,
    Unif: UnifierTop,
    F = dyn for<'slf> FnMut(&'slf Unif) -> Result<(), ResolutionError<'slf, 's, Unif>> + 's,
> where
    F: ?Sized,
{
    next: Option<Box<DelayedConstraint<'s, Unif>>>,
    f: F,
}

impl<'s, Unif: UnifierTop, F> Debug for DelayedConstraint<'s, Unif, F>
where
    F: ?Sized,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("DelayedConstraint")
            .field("next", &self.next)
            .finish()
    }
}

impl<'s, Unif: UnifierTop> DelayedConstraint<'s, Unif> {
    fn count(mut cur: &Option<Box<Self>>) -> usize {
        let mut total = 0;
        while let Some(nested) = cur {
            total += 1;
            cur = &nested.next;
        }
        total
    }
    /// Prefer to pass lists that tend to be longer first, so they don't get traversed again and again.
    fn add_to_list(long: Option<Box<Self>>, mut short: Option<Box<Self>>) -> Option<Box<Self>> {
        let mut last_ref = &mut short;
        while let Some(list_continues) = last_ref {
            last_ref = &mut list_continues.next;
        }
        *last_ref = long;
        short
    }
}

trait DelayedConstraintAcceptor<'s, Unif: UnifierTop> {
    fn add_delayed_constraint(&self, id: usize, constraint: Box<DelayedConstraint<'s, Unif>>);
}
pub struct ResolutionError<'slf, 's, Unif: UnifierTop> {
    subs: &'slf (dyn DelayedConstraintAcceptor<'s, Unif> + 's),
    id: usize,
}

impl<'slf, 's: 'slf, Unif: UnifierTop> Debug for ResolutionError<'slf, 's, Unif> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ResolutionError")
            .field("subs", &(self.subs as *const _))
            .field("id", &self.id)
            .finish()
    }
}

impl<'slf, 's, Unif: UnifierTop> ResolutionError<'slf, 's, Unif> {
    fn add_delayed_constraint(self, constraint: Box<DelayedConstraint<'s, Unif>>) {
        self.subs.add_delayed_constraint(self.id, constraint);
    }
}

impl<'s, T: Debug, Unif: UnifierTop> DelayedConstraintAcceptor<'s, Unif>
    for Substitutor<'s, T, Unif>
{
    fn add_delayed_constraint(&self, id: usize, constraint: Box<DelayedConstraint<'s, Unif>>) {
        assert!(
            constraint.next.is_none(),
            "DelayedConstraintAcceptor::add_delayed_constraint can only ever accept a single constraint at a time, because it should be called when a single constraint failed to resolve"
        );
        let mut subs = self.substitutor.borrow_mut();
        subs.add_constraints(id, Some(constraint));
    }
}

impl<'s, T: Clone + Debug, Unif: UnifierTop> Default for Substitutor<'s, T, Unif> {
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
    substitutor: Substitutor<'s, PeanoType, Self>,
}

impl<'s> PeanoUnifier<'s> {
    pub fn new() -> Self {
        Self {
            substitutor: Substitutor::new(),
        }
    }
}

impl<'s> UnifierTop for PeanoUnifier<'s> {}
impl<'slf, 's: 'slf> Unifier<'slf, 's, PeanoType> for PeanoUnifier<'s> {
    fn get_substitutor(&'slf self) -> &'slf Substitutor<'s, PeanoType, Self> {
        &self.substitutor
    }
    fn unify_subtrees(&'slf self, a: &'s PeanoType, b: &'s PeanoType) -> Result<(), UnifyError> {
        match (a, b) {
            (PeanoType::Zero, PeanoType::Zero) => Ok(()),
            (PeanoType::Succ(a), PeanoType::Succ(b)) => self.unify(a, b),
            _ => Err(UnifyError::Failure),
        }
    }
    fn set_subtrees(&'slf self, a: &'s PeanoType, b: PeanoType) -> Result<(), UnifyError> {
        match (a, b) {
            (PeanoType::Zero, PeanoType::Zero) => Ok(()),
            (PeanoType::Succ(a), PeanoType::Succ(b)) => self.set_cell(a, *b),
            _ => Err(UnifyError::Failure),
        }
    }
    fn contains_subtree(&'slf self, in_obj: &PeanoType, subtree: SubTree<PeanoType>) -> bool {
        match in_obj {
            PeanoType::Zero => false,
            PeanoType::Succ(succ) => self.contains_subtree_recurse(succ, subtree),
        }
    }
    fn fully_substitute_recurse(
        &'slf self,
        known: &'s PeanoType,
    ) -> Result<(), ResolutionError<'slf, 's, Self>> {
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

    use rand::prelude::IndexedRandom;
    use rand::seq::SliceRandom;
    use rand::{Rng, SeedableRng};

    /// `amount` must be > 0
    fn add_to(to: UniCell<PeanoType>, amount: usize) -> PeanoType {
        assert!(amount > 0);
        let mut cur = PeanoType::Succ(Box::new(to));
        for _ in 1..amount {
            cur = PeanoType::Succ(Box::new(cur.into()));
        }

        cur
    }
    /// `amount` must be >= 1
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
    /// Returns a [PeanoType] with an [Interior::Unallocated] at the bottom
    fn mk_peano_at_least(up_to: usize) -> UniCell<PeanoType> {
        add_to_cell(PeanoType::UNKNOWN, up_to)
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
            .set(&three_plus_a, add_to(substitutor.clone_unify(&a), 3))
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
            .set_cell(&a_plus_zero, add_to_cell(substitutor.clone_unify(&a), 0))
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
            .set(&a_plus_one, add_to(substitutor.clone_unify(&a), 1))
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
            .set(&one_plus_three, add_to(substitutor.clone_unify(&one), 3))
            .unwrap();
        substitutor
            .set(&two_plus_two, add_to(substitutor.clone_unify(&two), 2))
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
            .set(&x_plus_2, add_to(substitutor.clone_unify(&x), 2))
            .unwrap();
        substitutor
            .set(&y_val, add_to(substitutor.clone_unify(&x), 1))
            .unwrap();
        substitutor
            .set(&z_val, add_to(substitutor.clone_unify(&y), 1))
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
            .set(&b_val, add_to(substitutor.clone_unify(&a), 2))
            .unwrap();
        substitutor
            .set(&c_val, add_to(substitutor.clone_unify(&b), 1))
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

    #[test]
    fn test_delayed_constraint() {
        let a = PeanoType::UNKNOWN;
        let b = PeanoType::UNKNOWN;
        let c = PeanoType::UNKNOWN;

        let substitutor = PeanoUnifier::new();

        substitutor.delayed_constraint(|substitutor| {
            let a = substitutor.fully_substitute(&a)?;
            let b = substitutor.fully_substitute(&b)?;

            substitutor
                .set(&c, mk_peano(a.count() + b.count()))
                .unwrap();

            Ok(())
        });

        dbg!(&substitutor, &a, &b, &c);

        substitutor.set(&a, mk_peano(3)).unwrap();
        dbg!(&substitutor, &a, &b, &c);
        substitutor.set(&b, mk_peano(4)).unwrap();
        dbg!(&substitutor, &a, &b, &c);

        substitutor.execute_ready_delayed_constraints();

        let cc = substitutor.fully_substitute(&c).unwrap();
        dbg!(&substitutor, &a, &b, &c);

        assert_eq!(cc.count(), 7);
    }

    /// Performs a bunch of unifications, delayed_constraints, sets, etc in random order. This should be a thorough test for the correctness of [Unifier]
    #[test]
    fn test_unifications_heavy() {
        const NUM_PEANOS: usize = 1000;
        const INITIAL_PEANO: usize = 200;
        const PEANO_SPREAD: usize = 100;

        let mut rng = rand::rngs::StdRng::seed_from_u64(42);

        // Initialize all cells to "at least" small values. Since INITIAL_PEANO is quite large, we're unlikely to dip below these minima
        let cells: Vec<UniCell<PeanoType>> = (0..NUM_PEANOS)
            .map(|_| mk_peano_at_least(rng.random_range(0..50)))
            .collect();

        let mut idxes: Vec<usize> = (0..NUM_PEANOS).collect();
        let deltas: RefCell<Vec<i64>> = RefCell::new(vec![-9999999; NUM_PEANOS]);

        idxes.shuffle(&mut rng);

        let substitutor = PeanoUnifier::new();

        for (nth, idx) in idxes.into_iter().enumerate() {
            println!("{nth}th unify is idx {idx}");
            let cur = &cells[idx];
            if idx == 0 {
                // Large initial value, such that we can be reasonably certain that it's always possible to subtract by unifying.
                substitutor.set(cur, mk_peano(INITIAL_PEANO)).unwrap();
                deltas.borrow_mut()[idx] = INITIAL_PEANO as i64;
            } else {
                let prev = &cells[idx - 1];

                // Roughly balance the positive & negative deltas
                match rng.random_range(0..6) {
                    0 => {
                        substitutor.unify(cur, prev).unwrap();
                        deltas.borrow_mut()[idx] = 0;
                    }
                    1 => {
                        substitutor
                            .set_cell(cur, substitutor.clone_unify(prev))
                            .unwrap();
                        deltas.borrow_mut()[idx] = 0;
                    }
                    2 => {
                        let selected_amount: i64 = rng.random_range(0..=4);
                        substitutor
                            .set_cell(
                                cur,
                                add_to_cell(
                                    substitutor.clone_unify(prev),
                                    selected_amount as usize,
                                ),
                            )
                            .unwrap();
                        deltas.borrow_mut()[idx] = selected_amount;
                    }
                    3 => {
                        let selected_amount: i64 = rng.random_range(0..=4);
                        substitutor
                            .set_cell(
                                prev,
                                add_to_cell(substitutor.clone_unify(cur), selected_amount as usize),
                            )
                            .unwrap(); // Very unlikely to fail, since we start at a large value. (INITIAL_PEANO)
                        deltas.borrow_mut()[idx] = -selected_amount;
                    }
                    4 => {
                        let delta = rng.random_range(0..=4);
                        substitutor.delayed_constraint(move |substitutor| {
                            let mut prev = prev;
                            for _ in 0..delta {
                                // Very unlikely to fail, since we start at a large value. (INITIAL_PEANO)
                                let_unwrap!(PeanoType::Succ(prev_prev), substitutor.resolve(prev)?);
                                prev = prev_prev;
                            }
                            // Very unlikely to fail, since we start at a large value. (INITIAL_PEANO)
                            substitutor.unify(cur, prev).unwrap();
                            Ok(())
                        });
                        deltas.borrow_mut()[idx] = -delta;
                    }
                    5 => {
                        let delta: i64 = rng.random_range(-4..=4);
                        let deltas = &deltas;
                        substitutor.delayed_constraint(move |substitutor| {
                            let prev = substitutor.fully_substitute(prev)?;

                            let prev_count = prev.count();

                            let new_count = prev_count as i64 + delta;

                            // Clamp the value back to 100-300 every once in a whle
                            let new_count = new_count.clamp(
                                (INITIAL_PEANO - PEANO_SPREAD) as i64,
                                (INITIAL_PEANO + PEANO_SPREAD) as i64,
                            ) as usize;
                            deltas.borrow_mut()[idx] = new_count as i64 - prev_count as i64;

                            substitutor.set(cur, mk_peano(new_count)).unwrap();
                            Ok(())
                        });
                    }
                    _ => unreachable!(),
                }
                substitutor.execute_ready_delayed_constraints();
            }
        }

        println!("All unifies completed");

        // Once all indexes have been touched, there should be a unification chain all the way from the first (known) PeanoType, to the last PeanoType
        let mut total = 0;
        let mut expecteds: Vec<i64> = deltas.borrow().clone();
        for v in &mut expecteds {
            total += *v;
            *v = total;
        }

        // At this point further unifications shouldn't be able to change anything about the Peanos, since they're fully known.
        // Therefore we can start randomly unifying Peanos together and checking if their unification results match what we expect.
        for _ in 0..100 {
            let idx_a = rng.random_range(0..NUM_PEANOS);
            let idx_b = rng.random_range(0..NUM_PEANOS);

            let unify_result = if rng.random_bool(0.5) {
                substitutor.unify(&cells[idx_a], &cells[idx_b])
            } else {
                substitutor.set_cell(&cells[idx_a], substitutor.clone_unify(&cells[idx_b]))
            };
            if expecteds[idx_a] == expecteds[idx_b] {
                unify_result.unwrap();
            } else {
                unify_result.unwrap_err();
            }
        }

        // Finally, let's fully_substitute them, and actually count that they are correct
        for idx in 0..NUM_PEANOS {
            let peano = substitutor.fully_substitute(&cells[idx]).unwrap();
            assert_eq!(peano.count(), expecteds[idx] as usize);
            println!("peanos[{idx}]: {}", expecteds[idx]);
        }
    }

    #[test]
    fn test_longer_chain() {
        for i in 0..4 {
            let peanos = [PeanoType::UNKNOWN; 4];
            let substitutor = PeanoUnifier::new();

            substitutor.unify(&peanos[0], &peanos[1]).unwrap();
            substitutor.unify(&peanos[2], &peanos[3]).unwrap();
            substitutor.unify(&peanos[0], &peanos[3]).unwrap();

            substitutor.set(&peanos[i], PeanoType::Zero).unwrap();

            for p in &peanos {
                assert_eq!(substitutor.resolve(p).unwrap().count(), 0);
            }
        }
    }

    /// Just a stress test to cover all possible code paths. To check under miri that everything is alright.
    #[test]
    fn stress_test_for_miri() {
        let mut rng = rand::rngs::StdRng::seed_from_u64(42);

        // Create a bunch of unknowns
        let cells: Vec<UniCell<PeanoType>> = (0..1000).map(|_| PeanoType::UNKNOWN).collect();

        let substitutor = PeanoUnifier::new();

        // Randomly set some initial values
        for cell in cells.iter().take(100) {
            match rng.random_range(0..3) {
                0 => {
                    cell.set_initial(mk_peano(rng.random_range(0..5)));
                }
                1 => {
                    cell.set_initial_cell(mk_peano_at_least(rng.random_range(0..5)));
                }
                2 => {
                    cell.set_initial_cell(cells[rng.random_range(0..100)].clone_prototype());
                }
                _ => unreachable!(),
            }
        }

        for _ in 0..1000 {
            match rng.random_range(0..5) {
                0 => {
                    // Add a computed successor
                    let ontu = cells.choose(&mut rng).unwrap();
                    let add_count = rng.random_range(0..5);
                    let new_cell = add_to_cell(substitutor.clone_unify(ontu), add_count);
                    // May fail, may not fail
                    let _may_fail = substitutor.set_cell(cells.choose(&mut rng).unwrap(), new_cell);
                }
                1 => {
                    // Unify two peanos
                    let a = cells.choose(&mut rng).unwrap();
                    let b = cells.choose(&mut rng).unwrap();

                    // May fail, may not fail
                    let _may_fail = substitutor.unify(a, b);
                }
                2 => {
                    // Fully substitute something
                    let a = cells.choose(&mut rng).unwrap();

                    if substitutor.fully_substitute(a).is_ok() {
                        // Can clone values after a successful substitute
                        let _a_clone = a.clone();
                    }
                }
                3 => {
                    let a = cells.choose(&mut rng).unwrap();
                    let b = cells.choose(&mut rng).unwrap();
                    let c = cells.choose(&mut rng).unwrap();
                    substitutor.delayed_constraint(move |substitutor| {
                        let a = substitutor.fully_substitute(a)?;
                        let b = substitutor.fully_substitute(b)?;
                        let _may_fail = substitutor.set(c, mk_peano(a.count() + b.count()));
                        Ok(())
                    });
                }
                4 => {
                    let a = cells.choose(&mut rng).unwrap();
                    let b = cells.choose(&mut rng).unwrap();
                    let c = cells.choose(&mut rng).unwrap();
                    substitutor.delayed_constraint(move |substitutor| {
                        let a = substitutor.resolve(a)?;
                        let b = substitutor.resolve(b)?;
                        if let PeanoType::Zero = a
                            && let PeanoType::Zero = b
                        {
                            let _may_fail = substitutor.set(c, mk_peano(1));
                        } else {
                            let _may_fail = substitutor.set_cell(c, mk_peano_at_least(1));
                        }
                        Ok(())
                    });
                }
                _ => unreachable!(),
            }

            substitutor.execute_ready_delayed_constraints();
        }
    }
}
