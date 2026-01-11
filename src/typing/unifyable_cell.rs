#![allow(dead_code)]
#![allow(clippy::type_complexity)]

//! This file contains the final Unifier I make. It should have all the features I'll ever need.
//!
//! Important types:
//! - [UniCell]: The wrapper type for your types. This Cell can be unified with others. It can also be [Unifier::fully_substitute]d from a shared ref.
//! - [SubstituteRecurse]: Trait that provides [SubstituteRecurse::fully_substitute_recurse]
//! - [UnifyRecurse]: Trait needed for recursive unification of subcomponents.
//! - [Unifier]: Trait the user should implement for each [UniCell]`<T>` you wish to use.
//! - [UnifierTop]: This trait should be implemented once for any unifier you wish to create.
//!
//! An example type hierarchy for your custom unifier would then be:
//! ```
//! /// This Unifier unifies structures containing both `UniCell<PeanoType>` and `UniCell<DomainType>`
//! struct MyUnifier<'s> {
//!     unifier_info: UnifierTopInfo<'s, Self>,
//! }
//!
//! impl<'s> UnifierTop for MyUnifier<'s> {}
//! impl<'s> SubstituteRecurse<'s, PeanoType> for MyUnifier<'s> {}
//! impl<'s> SubstituteRecurse<'s, DomainType> for MyUnifier<'s> {}
//! impl<'s> UnifyRecurse<'s, PeanoType> for MyUnifier<'s> {}
//! impl<'s> UnifyRecurse<'s, DomainType> for MyUnifier<'s> {}
//! impl<'s> Unifier<'s, PeanoType> for MyUnifier<'s> {}
//! impl<'s> Unifier<'s, DomainType> for MyUnifier<'s> {}
//! ```
//!
//! The Lifecycle of [UniCell]s goes through the following 3 stages:
//! - [UniCell]s are created in the program structure either through [UniCell::new] or [UniCell::UNKNOWN].
//!   At this time the only two possible states are [Interior::Known] and [Interior::Unallocated]
//! - A [Unifier]`<'s>` is created, and begins performing unifications on the program structure.
//!   For this, the program structure will have a lifetime we will henceforth call `&'s`.
//!   [UniCell]s in `'s` can be freely [Unifier::unify]'d, and unification between `&'s UniCell`
//!   and an outside `&mut UniCell` happens through [Unifier::set].
//!   It is still possible to create new [UniCell]s, for instance through [Unifier::clone_unify].
//!   Critically, ONLY [UniCell]s in `'s` can take on [Interior::Terminal], and
//!   [Interior::SubstitutesTo] can only ever point to [UniCell]s in `'s`.
//! - Finally, all [UniCell]s in the program structure are [Unifier::fully_substitute]'d, and
//!   the unifier is decomissioned. At this point the [UniCell]s can still be left in any of [Interior]'s states, since
//!   without a [Unifier] the user can't observe anything ([UniCell::get]) beyond being [Interior::Known] or not known.
//!
//! The correctness of this whole shebang does depend on only one [Unifier] ever touching any given [UniCell].
//! It's not practical to enforce this, but this also doesn't seem like something we would be likely to run into.

use crate::{alloc::ArenaAllocator, append_only_vec::AppendOnlyVec};

use std::{
    cell::{RefCell, UnsafeCell},
    fmt::{Debug, Display, Write},
    hash::Hash,
    ops::{BitAnd, BitAndAssign, Deref},
};

/// Basically a [std::cell::OnceCell] for type checking. We implement it safely by maintaining the following invariant:
///
/// - [UniCell] starts out [UniCell::UNKNOWN]. No interior references can be taken in this state. (But the type variable we refer to *can* be updated)
/// - At some point, it is set to [Interior::Known]. After this point references to this interior value can be taken.
///   Afterwards, we can *never* reset a [Interior::Known] back to an [Interior::Unallocated] or [Interior::SubstitutesTo], or mess with it in any mutable way. (Panics when trying otherwise)
///
/// A "Prototype" refers to a [UniCell] that has thus far not been touched by a [Substitutor]. [UniCell::UNKNOWN] is a valid prototype.
/// A [UniCell] ceases to be a "Prototype" when it has touched a [Unifier] in any way.
pub struct UniCell<T>(UnsafeCell<Interior<T>>);

/// Interior data for [UniCell].
enum Interior<T> {
    Known(T),
    /// This [UniCell] is the same as the [UniCell] that this points to. [resolve_chain] walks these until it finds a [Interior::Terminal] or [Interior::Known].
    /// CANNOT BE CLONED
    /// The lifetime `'s` of the contained pointer is guarded by [UnifierTop]`<'s>`.
    /// The pointer and other variants can only be accessed *through* a [UnifierTop],
    /// and as long as that exists it forces every [UniCell] it's touched to continue to exist.
    ///
    /// Of course, this is broken if ever a second unrelated [UnifierTop] is used to access a [UniCell] created by a previous one,
    /// but it seems an acceptable assumption.
    SubstitutesTo(*const UniCell<T>),
    /// The end of a chain of [Interior::SubstitutesTo]. This means this [UniCell] is the representative for all [UniCell]s that point to it.
    /// Any cluster of such unified [UniCell]s has exactly one [Interior::Terminal] representative.
    /// CANNOT BE CLONED
    /// The given [ConstraintsID] can be [ConstraintsID::PLACEHOLDER], at least until `NonMaxUsize` gets stabilized.
    /// If we were to use an [Option], it would increase the minimum size of a [UniCell] to 24 instead of 16 bytes.
    Terminal(ConstraintsID),
    /// Default state of a new Type Variable. This means the variable is *unique*, and nothing points to it yet.
    /// CANNOT BE CLONED
    /// When [UniCell] is [Interior::Unallocated], no [Substitutor] references can exist to it yet.
    Unallocated,
}

pub struct ConstraintsIDMarker;
impl crate::alloc::UUIDMarker for ConstraintsIDMarker {
    const DISPLAY_NAME: &'static str = "delayed_constraint_";
}
pub type ConstraintsID = crate::alloc::UUID<ConstraintsIDMarker>;

#[derive(Clone, Copy, PartialEq, Eq)]
enum UnknownInterior<T> {
    SubstitutesTo(*const UniCell<T>),
    Terminal(ConstraintsID),
}
impl<T> Debug for UnknownInterior<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::SubstitutesTo(arg0) => f.debug_tuple("SubstitutesTo").field(arg0).finish(),
            Self::Terminal(arg0) => f.debug_tuple("Terminal").field(arg0).finish(),
        }
    }
}

impl<T> UniCell<T> {
    #[allow(clippy::declare_interior_mutable_const)]
    pub const UNKNOWN: Self = Self(UnsafeCell::new(Interior::Unallocated));

    pub const fn from_known(known: T) -> Self {
        Self(UnsafeCell::new(Interior::Known(known)))
    }
    pub fn new(known: impl Into<T>) -> Self {
        Self(UnsafeCell::new(Interior::Known(known.into())))
    }

    /// Either get a shared reference to the known value if it's set, or a mutable reference to the whole thing if it's not yet known
    /// This is safe, because [UniCell] only allows references to [Interior::Known] once it is set, and it can never be unset through a shared ref
    /// Immediately converts [Interior::Unallocated] to [Interior::Terminal]
    fn get_interior(&self) -> Result<&T, UnknownInterior<T>> {
        // SAFETY: See [UniCell]'s definition
        unsafe {
            let interior_ptr: *mut Interior<T> = self.0.get();
            match &*(interior_ptr as *const Interior<T>) {
                Interior::Known(known) => Ok(known),
                Interior::SubstitutesTo(id) => Err(UnknownInterior::SubstitutesTo(*id)),
                Interior::Terminal(constraints) => Err(UnknownInterior::Terminal(*constraints)),
                Interior::Unallocated => {
                    *interior_ptr = Interior::Terminal(ConstraintsID::PLACEHOLDER);
                    Err(UnknownInterior::Terminal(ConstraintsID::PLACEHOLDER))
                }
            }
        }
    }
    #[track_caller]
    fn replace_substitution(&self, existing_substitution: *const UniCell<T>, v: Interior<T>) {
        if let Err(UnknownInterior::SubstitutesTo(subs)) = self.get_interior()
            && subs == existing_substitution
        {
            // SAFETY: We already know we're not Interior::Known, See [UniCell]'s definition
            unsafe { *self.0.get() = v };
        } else {
            panic!("UniCell::replace_substitution's existing_id did not match the interior!");
        }
    }
    #[track_caller]
    fn replace_terminal(&self, v: Interior<T>) -> ConstraintsID {
        if let Err(UnknownInterior::Terminal(constraints)) = self.get_interior() {
            // SAFETY: We already know we're not Interior::Known, See [UniCell]'s definition
            unsafe { *self.0.get() = v };
            constraints
        } else {
            panic!("UniCell::replace_substitution's existing_id did not match the interior!");
        }
    }

    #[track_caller]
    pub fn get(&self) -> Option<&T> {
        unsafe {
            if let Interior::Known(known) = &*self.0.get() {
                Some(known)
            } else {
                None
            }
        }
    }
    #[track_caller]
    pub fn unwrap(&self) -> &T {
        self.get().unwrap()
    }
    #[track_caller]
    pub fn into_inner(self) -> T {
        let Interior::Known(v) = self.0.into_inner() else {
            unreachable!("UniCell::into_inner on not a Interior::Known");
        };
        v
    }

    /// `self` must be [UniCell::UNKNOWN]
    #[track_caller]
    pub fn set_initial(&self, v: T) {
        self.set_initial_cell(UniCell(UnsafeCell::new(Interior::Known(v))));
    }

    /// `self` must be [UniCell::UNKNOWN]
    #[track_caller]
    pub fn set_initial_cell(&self, v: UniCell<T>) {
        unsafe {
            let interior_ptr: *mut Interior<T> = self.0.get();
            let interior_should_be_unallocated = &*(interior_ptr as *const Interior<T>);
            if let Interior::Unallocated = interior_should_be_unallocated {
                *interior_ptr = v.0.into_inner();
            }
        }
    }

    /// Used to clone types that have been created with several [UniCell::UNKNOWN]s.
    /// The cloned [UniCell::UNKNOWN]s become distinct type variables.
    ///
    /// For clones after successful typechecking, use the regular [std::clone::Clone]
    ///
    /// For clones that *do* unify type variables, use [Unifier::clone_unify]
    pub fn clone_prototype_step(&self, clone_recurse: impl FnOnce(&T) -> T) -> Self {
        unsafe {
            let interior_ptr: *const Interior<T> = self.0.get();
            match &*interior_ptr {
                Interior::Known(known) => {
                    Self(UnsafeCell::new(Interior::Known(clone_recurse(known))))
                }
                Interior::SubstitutesTo(_) | Interior::Terminal(_) => {
                    unreachable!(
                        "An already unified UniCell cannot be used in [UniCell::clone_prototype_step]"
                    )
                }
                Interior::Unallocated => Self(UnsafeCell::new(Interior::Unallocated)),
            }
        }
    }
}

impl<T> From<T> for UniCell<T> {
    fn from(known: T) -> Self {
        Self(UnsafeCell::new(Interior::Known(known)))
    }
}

impl<T: Clone> Clone for UniCell<T> {
    #[track_caller]
    fn clone(&self) -> Self {
        // We cast to a const pointer here instead, such that we never actually create a &mut that might conflict with another existing shared ref
        let known = self.get_interior().expect("Not fully known substitutables can't be Cloned at all! Use [Unifier::clone_unify] or [Unifier::clone_prototype_step] to make clones.");
        let known_clone = known.clone();
        Self(UnsafeCell::new(Interior::Known(known_clone)))
    }
}

impl<T: Debug> Debug for UniCell<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        unsafe {
            let interior_ptr: *const Interior<T> = self.0.get();
            match &*interior_ptr {
                Interior::Known(known) => known.fmt(f),
                Interior::SubstitutesTo(subs) => write!(f, "SubstitutesTo({subs:?})"),
                Interior::Terminal(_) => write!(f, "Terminal"),
                Interior::Unallocated => write!(f, "Unallocated"),
            }
        }
    }
}

// Extra [UniCell] traits for convenience
impl<T: Debug> Deref for UniCell<T> {
    type Target = T;

    #[track_caller]
    fn deref(&self) -> &Self::Target {
        self.unwrap()
    }
}
impl<T: Debug + PartialEq> PartialEq for UniCell<T> {
    fn eq(&self, other: &Self) -> bool {
        self.unwrap() == other.unwrap()
    }
}
impl<T: Debug + Eq> Eq for UniCell<T> {}
impl<T: Display> Display for UniCell<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.get() {
            Some(known) => known.fmt(f),
            None => f.write_char('?'),
        }
    }
}
impl<T: Debug + PartialOrd> PartialOrd for UniCell<T> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.unwrap().partial_cmp(other.unwrap())
    }
}
impl<T: Debug + Ord> Ord for UniCell<T> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.unwrap().cmp(other.unwrap())
    }
}
impl<T: Debug + Hash> Hash for UniCell<T> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.unwrap().hash(state);
    }
}

/// Used for global Ts containing [UniCell] (Such as when using [std::sync::LazyLock]).
/// ONLY SAFE IF T IS FULLY RESOLVED.
pub struct SyncWrapper<T>(T);
impl<T> SyncWrapper<T> {
    pub const fn new(v: T) -> Self {
        Self(v)
    }
}
unsafe impl<T> Sync for SyncWrapper<T> {}
unsafe impl<T> Send for SyncWrapper<T> {}
impl<T> Deref for SyncWrapper<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

pub struct SubTree<T>(*const UniCell<T>);
impl<T> Debug for SubTree<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("SubTree").field(&self.0).finish()
    }
}
impl<T> Clone for SubTree<T> {
    fn clone(&self) -> Self {
        *self
    }
}
impl<T> Copy for SubTree<T> {}

/// All types in the unifyable type hierarchy should implement this.
pub trait SubstituteRecurse<'s, T>: UnifierTop<'s> + 's {
    /// Recursively call [Unifier::fully_substitute] on every contained [UniCell]`<*>`
    ///
    /// IMPORTANT: Do not stop at the first recursive failure!
    ///
    /// Return `true` if all substitutions were successful. If this is the case, the
    fn fully_substitute_recurse(&self, v: &T) -> bool;

    /// Recursively call [Unifier::resolve_all] on every contained [UniCell]`<*>`
    ///
    /// As opposed to [SubstituteRecurse::fully_substitute_recurse], this one should stop as soon as possible.
    /// Mostly meant for [UnifierTop::delayed_constraint] that wish to wait until a type is fully resolved.
    fn resolve_recurse(&self, v: &T) -> Result<(), ResolveError<'s>>;
}

/// Should *not* be implemented for types that have some kind of subtyiping relation. For this you should create your own subtyping methods.
pub trait UnifyRecurse<'s, T>: SubstituteRecurse<'s, T> + 's {
    /// `unify_subtrees` should recursively call [Unifier::unify] for every pair of subtrees. (Even for foreign [Unifier]s).
    /// If some irreconcilable difference is found it should return [UnifyResult::Failure].
    /// Otherwise return the binary AND of subtree unifications.
    /// Regardless of failure, all subtrees should be unified for best possible type error information.
    /// You as the user should never return [UnifyResult::FailureInfiniteTypes]
    fn unify_subtrees(&self, a: &'s T, b: &'s T) -> UnifyResult;
    /// Owning variant of [UnifyRecurse::unify_subtrees]
    ///
    /// Recursively call [Unifier::set] on every contained [UniCell]`<*>`
    ///
    /// Should check if two Knowns are compatible (if not, return [UnifyResult::Failure]),
    /// If they are compatible, then call [Unifier::set] recursively.
    ///
    /// This may steal interior [UniCell]s of `b`, and replace them with [Unifier::clone_unify]d copies of `a`
    fn set_subtrees(&self, a: &'s T, b: &mut T) -> UnifyResult;
    /// Create a clone of T, tolerant of as-yet-unknown variables.
    ///
    /// To implement, do not use regular [Clone], rather, clone nested [UniCell]`<*>` with [Unifier::clone_unify].
    fn clone_known(&self, known: &'s T) -> T;
}

/// To use, you should implement:
/// - [SubstituteRecurse::fully_substitute_recurse]
/// - [UnifyRecurse::unify_subtrees]
/// - [UnifyRecurse::set_subtrees]
/// - [UnifyRecurse::clone_known]
/// - [Unifier::contains_subtree]
/// - [UnifierTop::get_unifier_info]
///
/// The trait will then give you access to the following methods you can use to use:
/// - [UniCell::clone_prototype_step]
/// - [Unifier::fully_substitute]
/// - [Unifier::unify]
/// - [Unifier::set]
/// - [Unifier::clone_unify]
/// - [Unifier::resolve]
/// - [Unifier::resolve_all]
/// - [UnifierTop::delayed_constraint]
///
/// For examples see [PeanoUnifier]
///
/// [Unifier] references are *shared* on purpose (I've tried to replace them with &mut many times before).
/// The reason is that shared refs allow slightly more freedom for putting unifications together.
/// If we're building a [Unifier] wrapper that includes more data (like delayed constraints for instance), then
/// going through the trouble with &mut refs is not worth it. Passing it along the call stack is also no bueno,
/// we'd have to pass the unifier itself, plus whatever extra data the user wants to attach to it. Lots of complexity for nothing.
///
/// Times we've been through the `&mut Unifier` dead-end thus far: 4
pub trait Unifier<'s, T: 's>: UnifyRecurse<'s, T> + 's {
    /// Recursively call [Unifier::contains_subtree_recurse] on every contained [UniCell]`<T>`
    /// If multiple substitutors are in play, then try to [Unifier::resolve] foreign [UniCell]`<F>` first,
    /// and call [Unifier::contains_subtree_recurse] on any [UniCell]`<T>` found inside.
    ///
    /// `contains_subtree` is used to prevent infinite types.
    fn contains_subtree(&self, in_obj: &T, subtree: SubTree<T>) -> bool;

    /// Most fundamental operation of [Unifier]. This makes it so the left type, and the right type must be identical.
    /// This information is kept in a graph of unifications, and conflicting unifications will lead to [UnifyResult::Failure]
    ///
    /// For unifying a [UniCell] already in the graph (and thus not mutably accessible), with a new owned [UniCell], see [Unifier::set]
    fn unify(&self, a: &'s UniCell<T>, b: &'s UniCell<T>) -> UnifyResult {
        match (resolve_chain(self, a), resolve_chain(self, b)) {
            (Ok((a, _)), Ok((b, _))) => {
                // Simple optimization. Unification will often create referential identity.
                if std::ptr::eq(a, b) {
                    UnifyResult::Success
                } else {
                    self.unify_subtrees(a, b)
                }
            }
            (Err(unknown_cell), Ok((known, known_cell)))
            | (Ok((known, known_cell)), Err(unknown_cell)) => {
                if self.contains_subtree(known, SubTree(unknown_cell)) {
                    // Always have to check contains_subtree.
                    // Could be that a contains b which means if we merge them we'll get an infinite type.
                    return UnifyResult::FailureInfiniteTypes;
                }

                let ready_constraints =
                    unknown_cell.replace_terminal(mk_substitute_to(self, known_cell));

                self.get_unifier_info()
                    .mark_constraints_ready(ready_constraints);

                UnifyResult::Success
            }
            (Err(cell_a), Err(cell_b)) => {
                // We don't want to create a self-referential SubstituteTo
                if !std::ptr::eq(cell_a, cell_b) {
                    let b_constraints = cell_b.replace_terminal(mk_substitute_to(self, cell_a));

                    self.get_unifier_info()
                        .merge_constraints(cell_a, b_constraints);
                }
                UnifyResult::Success
            }
        }
    }
    /// Basically the same as [Unifier::unify], but with the second argument an owned object.
    /// This allows you to inject new Ts into the substitution graph.
    ///
    /// This may steal interior [UniCell]s of `to`, and replace them with [Unifier::clone_unify]d copies of `cell`
    ///
    /// Afterwards, `to` is still equivalent to the original `to`, and can be used for error reporting.
    fn set(&self, cell: &'s UniCell<T>, to: &mut UniCell<T>) -> UnifyResult {
        match to.0.get_mut() {
            Interior::Known(to_known) => {
                match resolve_chain(self, cell) {
                    Ok((cell_known, _)) => self.set_subtrees(cell_known, to_known),
                    Err(cell_terminal) => {
                        // Not known
                        // Same reasoning as in [Unifier::unify]
                        if self.contains_subtree(to_known, SubTree(cell_terminal)) {
                            UnifyResult::FailureInfiniteTypes
                        } else {
                            let stolen_to = std::mem::replace(
                                to,
                                UniCell(UnsafeCell::new(mk_substitute_to(self, cell_terminal))),
                            );
                            let ready_cs = cell_terminal.replace_terminal(stolen_to.0.into_inner());
                            self.get_unifier_info().mark_constraints_ready(ready_cs);
                            UnifyResult::Success
                        }
                    }
                }
            }
            Interior::SubstitutesTo(to_subs_to) => {
                // SAFETY: All [Interior::SubstitutesTo] MUST be created through [Unifier::mk_substitute_to], which enforces 's lifetime.
                unsafe {
                    let to_subs_to: &'s UniCell<T> = &**to_subs_to;
                    self.unify(cell, to_subs_to)
                }
            }
            // Unifying with an anonymous variable always succeeds, of course
            Interior::Unallocated => UnifyResult::Success,
            Interior::Terminal(_) => unreachable!(
                "Since `to` is by definition not part of the 's UniCell graph, it cannot be a Interior::Terminal. Rather it MUST be SubstitutesTo, Known, or Unallocated"
            ),
        }
    }
    /// Wrapper around [Unifier::set], for Known [UniCell]
    fn set_unwrap(&self, cell: &'s UniCell<T>, to: impl Into<T>) {
        self.set(
            cell,
            &mut UniCell(UnsafeCell::new(Interior::Known(to.into()))),
        )
        .unwrap();
    }

    /// Shorthand for creating a [UniCell::UNKNOWN], and then [Unifier::unify]-ing with `obj`.
    ///
    /// For clones after successful typechecking, use the regular [std::clone::Clone]
    ///
    /// For clones that *don't* unify type variables, use [UniCell::clone_prototype_step]
    fn clone_unify(&self, to_clone: &'s UniCell<T>) -> UniCell<T> {
        match resolve_chain(self, to_clone) {
            Ok((_, to_clone_terminal)) | Err(to_clone_terminal) => {
                UniCell(UnsafeCell::new(mk_substitute_to(self, to_clone_terminal)))
            }
        }
    }

    /// Walks the substitution chains to determine if it ends in a [Interior::Known]. If it does, it returns a reference to the known value.
    ///
    /// Use this for resolving dependencies in [UnifierTop::delayed_constraint]
    fn resolve<'obj>(&self, obj: &'obj UniCell<T>) -> Result<&'obj T, ResolveError<'s>>
    where
        's: 'obj,
    {
        match resolve_chain(self, obj) {
            Ok((obj_known, _)) => Ok(obj_known),
            Err(obj_last_cell) => Err(ResolveError(obj_last_cell)),
        }
    }

    /// Walks the substitution chains to determine if it ends in a [Interior::Known]. If it does, it returns a reference to the known value.
    ///
    /// Use this for resolving dependencies in [UnifierTop::delayed_constraint]
    fn resolve_all(&self, obj: &UniCell<T>) -> Result<(), ResolveError<'s>> {
        let known = self.resolve(obj)?;
        self.resolve_recurse(known)
    }

    /// Substitutes and clones the substitutions such that `obj` actually owns them.
    ///
    /// If this succeeds, then `obj` can be safely [Clone]-d.
    ///
    /// Use this for resolving dependencies in [UnifierTop::delayed_constraint]
    ///
    /// Complete this implementation by implementing [SubstituteRecurse::fully_substitute_recurse]
    fn fully_substitute(&self, cell: &UniCell<T>) -> bool {
        match cell.get_interior() {
            Ok(known) => {
                // Already known
                self.fully_substitute_recurse(known)
            }
            Err(UnknownInterior::SubstitutesTo(subs_to)) => unsafe {
                let subs_to: &'s UniCell<T> = &*subs_to;
                if let Ok((substitute_known, _)) = resolve_chain(self, subs_to) {
                    let known = self.clone_known(substitute_known);
                    let substitute_result = self.fully_substitute_recurse(&known);
                    cell.replace_substitution(subs_to, Interior::Known(known));
                    substitute_result
                } else {
                    false
                }
            },
            Err(UnknownInterior::Terminal(_)) => false,
        }
    }

    /// Used to detect infinite types and report errors on them.
    ///
    /// Complete this by implementing [UnifyRecurse::contains_subtree]
    fn contains_subtree_recurse(&self, obj: &UniCell<T>, subtree: SubTree<T>) -> bool {
        match resolve_chain(self, obj) {
            Ok((known, _)) => self.contains_subtree(known, subtree),
            Err(terminal_cell) => std::ptr::eq(terminal_cell, subtree.0),
        }
    }
}

/// Implements the Union-Find algorithm.
///
/// Resolves a possibly extensive chain of substitutions to a single node.
///
/// To help catch bugs, this panics on [Interior::Unallocated], so you have to [UniCell::get_interior] at least once before to make sure it's set.
///
/// Walks the substitution chain until either finding a [Interior::Known] value, or a [Interior::Terminal].
/// In either case, the last cell in the chain is also returned.
///
/// Due to it being impossible for non `'s` UniCells to be [Interior::Terminal],
/// this returns a `Err(&'s UniCell)` if the substitution isn't known. Sadly we can't really assert this, so we just have to trust the reasoning.
fn resolve_chain<'out, 's: 'out, T, Unif: Unifier<'s, T>>(
    _slf: &Unif,
    from: &'out UniCell<T>,
) -> Result<(&'out T, &'out UniCell<T>), &'s UniCell<T>> {
    // SAFETY: Each [Interior::SubstitutesTo] points to the next cell,
    // and due to the fact that Unifier<'s> does not allow such SubstitutesTo to be created from non-'s references
    // we can safely dereference each intermediary pointer.
    unsafe {
        // Initialize the first element. It could have been [Interior::Unknown]. No other elements could be Unknown though!
        let _ = from.get_interior();
        let mut cur = from;
        let result = loop {
            let interior_ptr: *const Interior<T> = cur.0.get();
            match &*interior_ptr {
                Interior::Known(known) => break Ok((known, cur)),
                Interior::SubstitutesTo(next_link) => {
                    cur = &**next_link;
                }
                Interior::Terminal(_) => {
                    // This is only allowed because it's impossible for a UniCell to be [Interior::Terminal] UNLESS it is in 's.
                    let cur_in_s: &'s UniCell<T> = &*(cur as *const UniCell<T>);
                    break Err(cur_in_s);
                }
                Interior::Unallocated => {
                    unreachable!("Interior::Unknown in the substitution chain???")
                }
            }
        };

        // Now we do chain compression, basically we link every pointer we encountered on our path to final_cell
        let mut cur = from;
        let final_cell: *const UniCell<T> = match result {
            Ok((_, final_cell)) | Err(final_cell) => final_cell as *const UniCell<T>,
        };
        while !std::ptr::eq(cur, final_cell) {
            let Interior::SubstitutesTo(next_link) = &mut *cur.0.get() else {
                unreachable!()
            };
            cur = &**next_link;
            *next_link = final_cell;
        }

        result
    }
}

/// All [Interior::SubstitutesTo] MUST be created through this method!
/// It ensures that they are all bound to the `'s` lifetime!
fn mk_substitute_to<'s, T, Unif: Unifier<'s, T>>(
    _slf: &Unif,
    cell_to: &'s UniCell<T>,
) -> Interior<T> {
    Interior::SubstitutesTo(cell_to)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UnifyResult {
    Success,
    Failure,
    FailureInfiniteTypes,
}
impl UnifyResult {
    pub fn unwrap(&self) {
        assert_eq!(*self, UnifyResult::Success);
    }
    pub fn expect(&self, msg: &str) {
        assert_eq!(*self, UnifyResult::Success, "{msg}");
    }
}
impl BitAnd for UnifyResult {
    type Output = UnifyResult;

    fn bitand(self, rhs: Self) -> UnifyResult {
        use UnifyResult::*;
        match (self, rhs) {
            (FailureInfiniteTypes, _) | (_, FailureInfiniteTypes) => FailureInfiniteTypes,
            (Failure, _) | (_, Failure) => Failure,
            (Success, Success) => Success,
        }
    }
}
impl BitAndAssign for UnifyResult {
    fn bitand_assign(&mut self, rhs: Self) {
        *self = self.bitand(rhs);
    }
}
impl From<bool> for UnifyResult {
    fn from(value: bool) -> Self {
        if value {
            UnifyResult::Success
        } else {
            UnifyResult::Failure
        }
    }
}

trait SetDelayedConstraintID {
    /// `self` must be [Interior::Terminal]. If it already contains a (non-null) [ConstraintsID] return that,
    /// otherwise set it, and return the given new [ConstraintsID]
    fn try_set_delayed_constraint_id(&self, delayed_constraint_id: ConstraintsID) -> ConstraintsID;
}

impl<T> SetDelayedConstraintID for UniCell<T> {
    fn try_set_delayed_constraint_id(&self, new_constraints_id: ConstraintsID) -> ConstraintsID {
        let Err(UnknownInterior::Terminal(existing_constraints)) = self.get_interior() else {
            unreachable!("SetDelayedConstraintID expects it to be UniCell::Terminal!")
        };
        if existing_constraints == ConstraintsID::PLACEHOLDER {
            let _placeholder = self.replace_terminal(Interior::Terminal(new_constraints_id));
            new_constraints_id
        } else {
            existing_constraints
        }
    }
}

pub struct ResolveError<'s>(&'s dyn SetDelayedConstraintID);

impl<'s> Debug for ResolveError<'s> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ResolutionError").finish_non_exhaustive()
    }
}

type DelayedConstraintBox<'s, Unif> = Box<dyn FnMut(&Unif) -> Result<(), ResolveError<'s>> + 's>;
pub struct UnifierTopInfo<'s, Unif: UnifierTop<'s>> {
    delayed_errors: AppendOnlyVec<Box<dyn FnOnce(&Unif) + 's>>,
    ready_constraints: AppendOnlyVec<DelayedConstraintBox<'s, Unif>>,
    constraints_per_terminal:
        RefCell<ArenaAllocator<Vec<DelayedConstraintBox<'s, Unif>>, ConstraintsIDMarker>>,
}
impl<'s, Unif: UnifierTop<'s>> UnifierTopInfo<'s, Unif> {
    pub fn new() -> Self {
        UnifierTopInfo {
            delayed_errors: AppendOnlyVec::new(),
            ready_constraints: AppendOnlyVec::new(),
            constraints_per_terminal: RefCell::new(ArenaAllocator::new()),
        }
    }
    fn add_blocked_constraint(
        &self,
        err: ResolveError<'s>,
        constraint: DelayedConstraintBox<'s, Unif>,
    ) {
        let mut constraints_borrow = self.constraints_per_terminal.borrow_mut();
        let next_id = constraints_borrow.reserve();
        let stored_id = err.0.try_set_delayed_constraint_id(next_id);
        if stored_id == next_id {
            constraints_borrow.alloc_reservation(next_id, Vec::new());
        } else {
            constraints_borrow.free_reservation(next_id);
        }
        constraints_borrow[stored_id].push(constraint);
    }
    fn mark_constraints_ready(&self, constraints_id: ConstraintsID) {
        if constraints_id == ConstraintsID::PLACEHOLDER {
            return; // No constraints
        }
        let mut constraints = self.constraints_per_terminal.borrow_mut();
        self.ready_constraints
            .append(&mut constraints.free(constraints_id));
    }
    /// `to` must be [Interior::Terminal]
    ///
    /// *if* constraint lists need to be merged, then this returns the constraint list stored in `self`
    fn merge_constraints<T>(&self, to: &'s UniCell<T>, constraints_to_merge: ConstraintsID) {
        if constraints_to_merge == ConstraintsID::PLACEHOLDER {
            // Nothing needs to be done
        } else {
            // SAFETY: We first assert that we're Terminal, so we're certain we're not producing a &mut to a Known.
            unsafe {
                assert!(matches!(
                    to.get_interior(),
                    Err(UnknownInterior::Terminal(_))
                ));
                let Interior::Terminal(terminal_constraints) = &mut *to.0.get() else {
                    unreachable!()
                };

                if *terminal_constraints == ConstraintsID::PLACEHOLDER {
                    *terminal_constraints = constraints_to_merge;
                    // `to` just takes the list
                } else {
                    let mut constraints_borrow = self.constraints_per_terminal.borrow_mut();
                    let mut constraints_list = constraints_borrow.free(constraints_to_merge);
                    constraints_borrow[*terminal_constraints].append(&mut constraints_list);
                }
            }
        }
    }
}
impl<'s, Unif: UnifierTop<'s>> Debug for UnifierTopInfo<'s, Unif> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("UnifierTopInfo")
            .field("num_delayed_errors", &self.delayed_errors.len())
            .finish()
    }
}
impl<'s, Unif: UnifierTop<'s>> Default for UnifierTopInfo<'s, Unif> {
    fn default() -> Self {
        Self::new()
    }
}
impl<'s, Unif: UnifierTop<'s>> Drop for UnifierTopInfo<'s, Unif> {
    fn drop(&mut self) {
        if !std::thread::panicking() {
            if !self.delayed_errors.is_empty() {
                panic!("UnifierTop Delayed Errors weren't reported!");
            }
            if !self.ready_constraints.is_empty() {
                panic!(
                    "UnifierTop was dropped without handling some leftover constraints! Call UnifierTop::execute_ready_constraints!"
                );
            }
        }
    }
}

pub trait UnifierTop<'s>: Sized {
    /// Your [UnifierTop] should store a field [UnifierTopInfo]. Return it here.
    fn get_unifier_info(&self) -> &UnifierTopInfo<'s, Self>;

    /// You may call [UnifierTop::execute_ready_constraints] multiple times, but certainly you must make sure to call it before the [UnifierTop] is dropped.
    fn execute_ready_constraints(&self) {
        let unif_top = self.get_unifier_info();
        while let Some(mut c) = unif_top.ready_constraints.pop() {
            if let Err(resolve_err) = c(self) {
                unif_top.add_blocked_constraint(resolve_err, c);
            }
        }
    }

    /// You must call [UnifierTop::execute_ready_constraints] after creating delayed_constraints,
    /// as not immediately resolved delayed constraints don't immediately get resolved the moment they become eligible.
    fn delayed_constraint<'unif>(
        &self,
        mut f: impl FnMut(&Self) -> Result<(), ResolveError<'s>> + 's,
    ) where
        's: 'unif,
    {
        if let Err(resolve_err) = f(self) {
            let unif_top = self.get_unifier_info();
            unif_top.add_blocked_constraint(resolve_err, Box::new(f));
        }
    }

    /// Adds an error, that will be reported after all typechecking has finished. (When [UnifierTop::decomission] is called)
    fn delayed_error(&self, f: impl FnOnce(&Self) + 's) {
        let unifier_info = self.get_unifier_info();

        unifier_info.delayed_errors.push(Box::new(f));
    }

    fn report_delayed_errors(&self) {
        let info = self.get_unifier_info();
        for e in info.delayed_errors.take() {
            e(self)
        }
    }
}

// ==================================================
// ======= FROM HERE ON OUT IT'S EXAMPLE CODE =======
// ==================================================

#[derive(Debug, Clone)]
enum PeanoType {
    Zero,
    Succ(Box<UniCell<PeanoType>>),
}

impl PeanoType {
    #[allow(clippy::declare_interior_mutable_const)]
    pub const UNKNOWN: UniCell<PeanoType> = UniCell::<PeanoType>::UNKNOWN;

    fn count(&self) -> usize {
        let mut cur = self;
        let mut total = 0;
        while let PeanoType::Succ(inner) = cur {
            cur = inner.unwrap();
            total += 1;
        }
        total
    }
}

#[derive(Debug, Clone)]
enum SecondType {
    None,
    OnePeano(UniCell<PeanoType>),
    TwoPeano(UniCell<PeanoType>, UniCell<PeanoType>),
}

impl SecondType {
    #[allow(clippy::declare_interior_mutable_const)]
    pub const UNKNOWN: UniCell<SecondType> = UniCell::<SecondType>::UNKNOWN;
}

#[derive(Debug)]
struct PeanoUnifier<'s> {
    unifier_info: UnifierTopInfo<'s, Self>,
}

impl<'s> PeanoUnifier<'s> {
    pub fn new() -> Self {
        Self {
            unifier_info: UnifierTopInfo::new(),
        }
    }
}

impl<'s> UnifierTop<'s> for PeanoUnifier<'s> {
    fn get_unifier_info(&self) -> &UnifierTopInfo<'s, Self> {
        &self.unifier_info
    }
}
impl<'s> SubstituteRecurse<'s, PeanoType> for PeanoUnifier<'s> {
    fn fully_substitute_recurse(&self, v: &PeanoType) -> bool {
        match v {
            PeanoType::Zero => true,
            PeanoType::Succ(succ) => self.fully_substitute(succ),
        }
    }

    fn resolve_recurse(&self, v: &PeanoType) -> Result<(), ResolveError<'s>> {
        match v {
            PeanoType::Zero => Ok(()),
            PeanoType::Succ(succ) => self.resolve_all(succ),
        }
    }
}
impl<'s> UnifyRecurse<'s, PeanoType> for PeanoUnifier<'s> {
    fn unify_subtrees(&self, a: &'s PeanoType, b: &'s PeanoType) -> UnifyResult {
        match (a, b) {
            (PeanoType::Zero, PeanoType::Zero) => UnifyResult::Success,
            (PeanoType::Succ(a), PeanoType::Succ(b)) => self.unify(a, b),
            _ => UnifyResult::Failure,
        }
    }
    fn set_subtrees(&self, a: &'s PeanoType, b: &mut PeanoType) -> UnifyResult {
        match (a, b) {
            (PeanoType::Zero, PeanoType::Zero) => UnifyResult::Success,
            (PeanoType::Succ(a), PeanoType::Succ(b)) => self.set(a, b),
            _ => UnifyResult::Failure,
        }
    }
    fn clone_known(&self, known: &'s PeanoType) -> PeanoType {
        match known {
            PeanoType::Zero => PeanoType::Zero,
            PeanoType::Succ(succ) => PeanoType::Succ(Box::new(self.clone_unify(succ))),
        }
    }
}
impl<'s> SubstituteRecurse<'s, SecondType> for PeanoUnifier<'s> {
    fn fully_substitute_recurse(&self, v: &SecondType) -> bool {
        match v {
            SecondType::None => false,
            SecondType::OnePeano(a) => self.fully_substitute(a),
            SecondType::TwoPeano(a, b) => self.fully_substitute(a) & self.fully_substitute(b),
        }
    }

    fn resolve_recurse(&self, v: &SecondType) -> Result<(), ResolveError<'s>> {
        match v {
            SecondType::None => Ok(()),
            SecondType::OnePeano(a) => self.resolve_all(a),
            SecondType::TwoPeano(a, b) => {
                self.resolve_all(a)?;
                self.resolve_all(b)
            }
        }
    }
}
impl<'s> UnifyRecurse<'s, SecondType> for PeanoUnifier<'s> {
    fn unify_subtrees(&self, a: &'s SecondType, b: &'s SecondType) -> UnifyResult {
        match (a, b) {
            (SecondType::None, SecondType::None) => UnifyResult::Success,
            (SecondType::OnePeano(a), SecondType::OnePeano(b)) => self.unify(a, b),
            (SecondType::TwoPeano(a1, a2), SecondType::TwoPeano(b1, b2)) => {
                self.unify(a1, b1) & self.unify(a2, b2)
            }
            _ => UnifyResult::Failure,
        }
    }
    fn set_subtrees(&self, a: &'s SecondType, b: &mut SecondType) -> UnifyResult {
        match (a, b) {
            (SecondType::None, SecondType::None) => UnifyResult::Success,
            (SecondType::OnePeano(a), SecondType::OnePeano(b)) => self.set(a, b),
            (SecondType::TwoPeano(a1, a2), SecondType::TwoPeano(b1, b2)) => {
                self.set(a1, b1) & self.set(a2, b2)
            }
            _ => UnifyResult::Failure,
        }
    }
    fn clone_known(&self, known: &'s SecondType) -> SecondType {
        match known {
            SecondType::None => SecondType::None,
            SecondType::OnePeano(a) => SecondType::OnePeano(self.clone_unify(a)),
            SecondType::TwoPeano(a, b) => {
                SecondType::TwoPeano(self.clone_unify(a), self.clone_unify(b))
            }
        }
    }
}
impl<'s> Unifier<'s, PeanoType> for PeanoUnifier<'s> {
    fn contains_subtree(&self, in_obj: &PeanoType, subtree: SubTree<PeanoType>) -> bool {
        match in_obj {
            PeanoType::Zero => false,
            PeanoType::Succ(succ) => self.contains_subtree_recurse(succ, subtree),
        }
    }
}
impl<'s> Unifier<'s, SecondType> for PeanoUnifier<'s> {
    fn contains_subtree(&self, _in_obj: &SecondType, _subtree: SubTree<SecondType>) -> bool {
        false // SecondType doesn't recurse
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

    use crate::prelude::*;

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

        let unifier = PeanoUnifier::new();
        three_plus_a.set_initial(add_to(unifier.clone_unify(&a), 3));

        unifier.unify(&four, &three_plus_a).unwrap();

        assert!(unifier.fully_substitute(&a));

        assert_eq!(a.unwrap().count(), 1)
    }

    #[test]
    fn test_non_infinite_peano() {
        let a = PeanoType::UNKNOWN;
        let a_plus_zero = PeanoType::UNKNOWN;

        let unifier = PeanoUnifier::new();
        a_plus_zero.set_initial_cell(add_to_cell(unifier.clone_unify(&a), 0));

        unifier.unify(&a, &a_plus_zero).unwrap();
        unifier.unify(&a_plus_zero, &a).unwrap();

        // a and a_plus_zero should both still have a type variable.
        assert!(!unifier.fully_substitute(&a));
        assert!(!unifier.fully_substitute(&a_plus_zero));
    }

    #[test]
    fn test_invalid_unification() {
        let three = mk_peano_cell(3);
        let four = mk_peano_cell(4);
        let unifier = PeanoUnifier::new();

        assert_eq!(unifier.unify(&three, &four), UnifyResult::Failure);
        assert_eq!(unifier.unify(&four, &three), UnifyResult::Failure);

        assert!(unifier.fully_substitute(&three));
        assert!(unifier.fully_substitute(&four));
    }

    #[test]
    fn test_infinite_peano() {
        let a = PeanoType::UNKNOWN;
        let a_plus_one = PeanoType::UNKNOWN;

        let unifier = PeanoUnifier::new();
        a_plus_one.set_initial(add_to(unifier.clone_unify(&a), 1));

        // Both of these try to unify a = a + 1, which would lead to an infinite tower of +1s
        assert_eq!(
            unifier.unify(&a, &a_plus_one),
            UnifyResult::FailureInfiniteTypes
        );
        assert_eq!(
            unifier.unify(&a_plus_one, &a),
            UnifyResult::FailureInfiniteTypes
        );

        assert!(!unifier.fully_substitute(&a));
        assert!(!unifier.fully_substitute(&a_plus_one));
    }

    #[test]
    fn test_peano_equivalence_simple() {
        let one = mk_peano_cell(1);
        let two = mk_peano_cell(2);
        let one_plus_three = PeanoType::UNKNOWN;
        let two_plus_two = PeanoType::UNKNOWN;

        let unifier = PeanoUnifier::new();
        one_plus_three.set_initial(add_to(unifier.clone_unify(&one), 3));
        two_plus_two.set_initial(add_to(unifier.clone_unify(&two), 2));
        // 2+2 == 1+3
        unifier.unify(&two_plus_two, &one_plus_three).unwrap();
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

        let unifier = PeanoUnifier::new();
        x_plus_2.set_initial(add_to(unifier.clone_unify(&x), 2));
        y_val.set_initial(add_to(unifier.clone_unify(&x), 1));
        z_val.set_initial(add_to(unifier.clone_unify(&y), 1));

        // Unify y with x+1, z with y+1, and z with x+2
        unifier.unify(&y, &y_val).unwrap();
        unifier.unify(&z, &z_val).unwrap();
        unifier.unify(&z, &x_plus_2).unwrap();

        assert!(unifier.fully_substitute(&x));
        assert!(unifier.fully_substitute(&y));
        assert!(unifier.fully_substitute(&z));

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

        let unifier = PeanoUnifier::new();
        b_val.set_initial(add_to(unifier.clone_unify(&a), 2));
        c_val.set_initial(add_to(unifier.clone_unify(&b), 1));

        unifier.unify(&b, &b_val).unwrap();
        unifier.unify(&c, &c_val).unwrap();

        assert!(unifier.fully_substitute(&a));
        assert!(unifier.fully_substitute(&b));
        assert!(unifier.fully_substitute(&c));

        assert_eq!(a.unwrap().count(), 0);
        assert_eq!(b.unwrap().count(), 2);
        assert_eq!(c.unwrap().count(), 3);
    }

    #[test]
    fn test_delayed_constraint() {
        let a = PeanoType::UNKNOWN;
        let b = PeanoType::UNKNOWN;
        let c = PeanoType::UNKNOWN;

        let unifier = PeanoUnifier::new();

        unifier.delayed_constraint(|unifier| {
            unifier.resolve_all(&a)?;
            unifier.resolve_all(&b)?;

            unifier.set_unwrap(&c, mk_peano(a.unwrap().count() + b.unwrap().count()));

            Ok(())
        });

        unifier.set_unwrap(&a, mk_peano(3));
        unifier.execute_ready_constraints();

        unifier.set_unwrap(&b, mk_peano(4));
        unifier.execute_ready_constraints();

        assert!(unifier.fully_substitute(&c));

        assert_eq!(c.unwrap().count(), 7);
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

        let unifier = PeanoUnifier::new();

        for (nth, idx) in idxes.into_iter().enumerate() {
            println!("{nth}th unify is idx {idx}");
            let cur = &cells[idx];
            if idx == 0 {
                // Large initial value, such that we can be reasonably certain that it's always possible to subtract by unifying.
                unifier.set_unwrap(cur, mk_peano(INITIAL_PEANO));
                deltas.borrow_mut()[idx] = INITIAL_PEANO as i64;
            } else {
                let prev = &cells[idx - 1];

                // Roughly balance the positive & negative deltas
                match rng.random_range(0..6) {
                    0 => {
                        unifier.unify(cur, prev).unwrap();
                        deltas.borrow_mut()[idx] = 0;
                    }
                    1 => {
                        unifier.set(cur, &mut unifier.clone_unify(prev)).unwrap();
                        deltas.borrow_mut()[idx] = 0;
                    }
                    2 => {
                        let selected_amount: i64 = rng.random_range(0..=4);
                        unifier
                            .set(
                                cur,
                                &mut add_to_cell(
                                    unifier.clone_unify(prev),
                                    selected_amount as usize,
                                ),
                            )
                            .unwrap();
                        deltas.borrow_mut()[idx] = selected_amount;
                    }
                    3 => {
                        let selected_amount: i64 = rng.random_range(0..=4);
                        unifier
                            .set(
                                prev,
                                &mut add_to_cell(
                                    unifier.clone_unify(cur),
                                    selected_amount as usize,
                                ),
                            )
                            .unwrap(); // Very unlikely to fail, since we start at a large value. (INITIAL_PEANO)
                        deltas.borrow_mut()[idx] = -selected_amount;
                    }
                    4 => {
                        let delta = rng.random_range(0..=4);
                        unifier.delayed_constraint(move |unifier| {
                            let mut prev = prev;
                            for _ in 0..delta {
                                // Very unlikely to fail, since we start at a large value. (INITIAL_PEANO)
                                let_unwrap!(PeanoType::Succ(prev_prev), unifier.resolve(prev)?);
                                prev = prev_prev;
                            }
                            // Very unlikely to fail, since we start at a large value. (INITIAL_PEANO)
                            unifier.unify(cur, prev).unwrap();
                            Ok(())
                        });
                        deltas.borrow_mut()[idx] = -delta;
                    }
                    5 => {
                        let delta: i64 = rng.random_range(-4..=4);
                        let deltas = &deltas;
                        unifier.delayed_constraint(move |unifier| {
                            unifier.resolve_all(prev)?;
                            assert!(unifier.fully_substitute(prev));

                            let prev_count = prev.unwrap().count();

                            let new_count = prev_count as i64 + delta;

                            // Clamp the value back to 100-300 every once in a whle
                            let new_count = new_count.clamp(
                                (INITIAL_PEANO - PEANO_SPREAD) as i64,
                                (INITIAL_PEANO + PEANO_SPREAD) as i64,
                            ) as usize;
                            deltas.borrow_mut()[idx] = new_count as i64 - prev_count as i64;

                            unifier.set_unwrap(cur, mk_peano(new_count));
                            Ok(())
                        });
                    }
                    _ => unreachable!(),
                }
            }
            unifier.execute_ready_constraints();
        }

        assert!(unifier.unifier_info.ready_constraints.is_empty());
        assert!(
            unifier
                .unifier_info
                .constraints_per_terminal
                .borrow()
                .is_empty()
        );

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
                unifier.unify(&cells[idx_a], &cells[idx_b])
            } else {
                unifier.set(&cells[idx_a], &mut unifier.clone_unify(&cells[idx_b]))
            };
            let exp_a = expecteds[idx_a];
            let exp_b = expecteds[idx_b];
            if exp_a == exp_b {
                unify_result.unwrap();
            } else {
                assert_eq!(
                    unify_result,
                    UnifyResult::Failure,
                    "Should be failure: {exp_a} != {exp_b}"
                );
            }
        }

        // Finally, let's fully_substitute them, and actually count that they are correct
        for idx in 0..NUM_PEANOS {
            assert!(unifier.fully_substitute(&cells[idx]));
            assert_eq!(cells[idx].unwrap().count(), expecteds[idx] as usize);
            println!("peanos[{idx}]: {}", expecteds[idx]);
        }
    }

    #[test]
    fn test_longer_chain() {
        for i in 0..4 {
            let peanos = [PeanoType::UNKNOWN; 4];
            let unifier = PeanoUnifier::new();

            unifier.unify(&peanos[0], &peanos[1]).unwrap();
            unifier.unify(&peanos[2], &peanos[3]).unwrap();
            unifier.unify(&peanos[0], &peanos[3]).unwrap();

            unifier.set_unwrap(&peanos[i], PeanoType::Zero);

            for p in &peanos {
                assert_eq!(unifier.resolve(p).unwrap().count(), 0);
            }
        }
    }

    #[test]
    fn test_multi_substitutor() {
        let a = SecondType::UNKNOWN;
        let b = SecondType::UNKNOWN;

        let unifier = PeanoUnifier::new();
        a.set_initial(SecondType::TwoPeano(mk_peano_cell(1), PeanoType::UNKNOWN));
        b.set_initial(SecondType::TwoPeano(PeanoType::UNKNOWN, mk_peano_cell(2)));

        unifier.unify(&a, &b).unwrap();

        assert!(unifier.fully_substitute(&a));
        assert!(unifier.fully_substitute(&b));

        let_unwrap!(SecondType::TwoPeano(a1, a2), a.unwrap());
        let_unwrap!(SecondType::TwoPeano(b1, b2), b.unwrap());

        assert_eq!(a1.unwrap().count(), 1);
        assert_eq!(a2.unwrap().count(), 2);
        assert_eq!(b1.unwrap().count(), 1);
        assert_eq!(b2.unwrap().count(), 2);
    }

    #[test]
    fn test_infinite_type_detection() {
        const TOTAL_SIZE: usize = 100;
        const TOTAL_ITER: usize = 1000;
        let mut rng = rand::rngs::StdRng::seed_from_u64(42);

        // Create a bunch of unknowns
        let cells: Vec<UniCell<PeanoType>> = (0..TOTAL_SIZE).map(|_| PeanoType::UNKNOWN).collect();

        let unifier = PeanoUnifier::new();

        for _ in 0..10 {
            let a = cells.choose(&mut rng).unwrap();
            if rng.random_bool(0.5) {
                a.set_initial(PeanoType::Zero);
            } else {
                unifier.set_unwrap(a, PeanoType::Zero);
            }
        }

        for _ in 0..TOTAL_ITER {
            //let [a, b] = rand::seq::index::sample_array(&mut rng, TOTAL_SIZE).unwrap();
            let a = cells.choose(&mut rng).unwrap();
            let b = cells.choose(&mut rng).unwrap();

            match rng.random_range(0..3) {
                0 => {
                    unifier.unify(a, b).unwrap();
                }
                1 => {
                    unifier.set(a, &mut unifier.clone_unify(b)).unwrap();
                }
                2 => {
                    if let Err(UnknownInterior::Terminal(_)) = a.get_interior()
                        && !std::ptr::eq(a, b)
                    {
                        a.set_initial_cell(unifier.clone_unify(b));
                    }
                }
                _ => unreachable!(),
            }
        }
    }

    /// Just a stress test to cover all possible code paths. To check under miri that everything is alright.
    #[test]
    fn stress_test_for_miri() {
        const TOTAL_SIZE: usize = 1000;
        let mut rng = rand::rngs::StdRng::seed_from_u64(42);

        // Create a bunch of unknowns
        let cells: Vec<UniCell<PeanoType>> = (0..TOTAL_SIZE).map(|_| PeanoType::UNKNOWN).collect();

        let unifier = PeanoUnifier::new();

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

        for _ in 0..cells.len() {
            match rng.random_range(0..5) {
                0 => {
                    // Add a computed successor
                    let ontu = cells.choose(&mut rng).unwrap();
                    let add_count = rng.random_range(0..5);
                    let mut new_cell = add_to_cell(unifier.clone_unify(ontu), add_count);
                    // May fail, may not fail
                    let _may_fail = unifier.set(cells.choose(&mut rng).unwrap(), &mut new_cell);
                }
                1 => {
                    // Unify two peanos
                    let a = cells.choose(&mut rng).unwrap();
                    let b = cells.choose(&mut rng).unwrap();

                    // May fail, may not fail
                    let _may_fail = unifier.unify(a, b);
                }
                2 => {
                    // Fully substitute something
                    let a = cells.choose(&mut rng).unwrap();

                    if unifier.fully_substitute(a) {
                        // Can clone values after a successful substitute
                        let _a_clone = a.clone();
                    }
                }
                3 => {
                    let a = cells.choose(&mut rng).unwrap();
                    let b = cells.choose(&mut rng).unwrap();
                    let c = cells.choose(&mut rng).unwrap();
                    unifier.delayed_constraint(move |unifier| {
                        unifier.resolve_all(a)?;
                        unifier.resolve_all(b)?;
                        assert!(unifier.fully_substitute(a));
                        assert!(unifier.fully_substitute(b));
                        let _may_fail = unifier.set(
                            c,
                            &mut UniCell::new(mk_peano(a.unwrap().count() + b.unwrap().count())),
                        );
                        Ok(())
                    });
                }
                4 => {
                    let a = cells.choose(&mut rng).unwrap();
                    let b = cells.choose(&mut rng).unwrap();
                    let c = cells.choose(&mut rng).unwrap();
                    unifier.delayed_constraint(move |unifier| {
                        let a = unifier.resolve(a)?;
                        let b = unifier.resolve(b)?;
                        if let PeanoType::Zero = a
                            && let PeanoType::Zero = b
                        {
                            let _may_fail = unifier.set(c, &mut UniCell::new(mk_peano(1)));
                        } else {
                            let _may_fail = unifier.set(c, &mut mk_peano_at_least(1));
                        }
                        Ok(())
                    });
                }
                _ => unreachable!(),
            }

            unifier.execute_ready_constraints();
        }
    }
}
