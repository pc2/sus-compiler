use std::{
    cell::UnsafeCell,
    fmt::Debug,
    ops::{Deref, DerefMut},
};

use crate::{
    alloc::{AppendOnlyAlloc, UUIDMarker, UUID},
    let_unwrap,
    typing::type_inference::PeanoVariableIDMarker,
};

enum Interior<T: Debug + Clone, IDMarker: UUIDMarker> {
    Known(T),
    /// If no substitution is known yet, then this points to itself (may be in any cycle length, [UnifyableCell::resolve_substitution_chain] is there to contract it).
    SubstitutesTo(UUID<IDMarker>),
    /// Default state of a new Type Variable. This means the variable is *unique*, and so we don't yet need an ID to track its Unification.
    /// CANNOT BE CLONED (panics)
    Unallocated,
}

impl<T: Debug + Clone, IDMarker: UUIDMarker> Clone for Interior<T, IDMarker> {
    fn clone(&self) -> Self {
        match self {
            Self::Known(arg0) => Self::Known(arg0.clone()),
            Self::SubstitutesTo(var) => Self::SubstitutesTo(*var),
            Self::Unallocated => {
                unreachable!("Cannot clone Unallocated! That would add an incorrect dependency!")
            }
        }
    }
}

impl<T: Debug + Clone, IDMarker: UUIDMarker> Debug for Interior<T, IDMarker> {
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
/// - [UnifyableCell] starts out Unknown. No interior references can be taken in this state. (But the type variable we refer to *can* be updated)
/// - At some point, it is [UnifyableCell::set()] to some Known value. After this point references to this interior value can be taken
/// - With a shared reference, we can *never* reset a Known back to an Unknown, or mess with it in any mutable way. (Panics when trying otherwise)
pub struct UnifyableCell<T: Debug + Clone, IDMarker: UUIDMarker>(UnsafeCell<Interior<T, IDMarker>>);

impl<T: Debug + Clone, IDMarker: UUIDMarker> Debug for UnifyableCell<T, IDMarker> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("UnifyableCell").field(&self.0).finish()
    }
}

impl<T: Debug + Clone, IDMarker: UUIDMarker> UnifyableCell<T, IDMarker> {
    pub const UNKNOWN: Self = Self(UnsafeCell::new(Interior::Unallocated));
    pub fn get(&self) -> Option<&T> {
        // We cast to a const pointer here instead, such that we never actually create a &mut that might conflict with another existing shared ref
        let content_ptr: *const Interior<T, IDMarker> = self.0.get();
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
    /// Panics if this has ever been unified with anything else
    pub fn set_initial(&self, v: T) {
        let content_ptr_mut: *mut Interior<T, IDMarker> = self.0.get();
        let context_ptr_while_maybe_shared: *const Interior<T, IDMarker> = content_ptr_mut;
        {
            // SAFETY: Either this is already Interior::Known, in which case we panic before messing with a shared ref, OR this is not Interior::Known, and therefore no interior shared references have been given out
            //
            // Careful here! If we use the &mut for the check, then we've technically created UB between here and the panic. That's why we have to use a *const _, and only create the mutable ref when we're sure no shares exist
            let content = unsafe { &*context_ptr_while_maybe_shared };
            if !matches!(content, Interior::Unallocated) {
                unreachable!("`set_initial({v:?})` on a UnifyableCell that's not in the Unallocated state ({content:?})!");
            }
        }

        // SAFETY: Only now do we actually use the ptr mutably
        unsafe { *content_ptr_mut = Interior::Known(v) }
    }

    /// Resolves a possibly extensive chain of substitutions to a single node.
    ///
    /// Result is either:
    ///     [Interior::Known] is then of course a known value.
    ///     [Interior::SubstitutesTo] that points to itself, signifying no substitution known yet
    fn resolve_substitution_chain<'s>(
        &'s self,
        substitutor: &AppendOnlyAlloc<&'s UnifyableCell<T, IDMarker>, IDMarker>,
    ) -> ChainResolution<'s, T, IDMarker> {
        /// `ptr` must be [Interior::SubstitutesTo]
        /*fn resolve_substitution_chain<'s, T: Debug, IDMarker: UUIDMarker>(
            mut ptr: *mut Interior<T, IDMarker>,
            substitutor: &AppendOnlyAlloc<&'s UnifyableCell<T, IDMarker>, IDMarker>,
        ) -> UUID<IDMarker> {
            unsafe {
                let_unwrap!(Interior::SubstitutesTo(to_id), &*ptr);
                let mut to_id = *to_id;

                loop {
                    let target = substitutor.copy_elem(to_id);

                    let target_ptr: *mut _ = target.0.get();
                    match &*target_ptr {
                        Interior::Known(_) => return to_id,
                        Interior::SubstitutesTo(targets_target) => {
                            *ptr = Interior::SubstitutesTo(*targets_target);

                            if *targets_target == to_id {
                                return to_id;
                            } else {
                                to_id = *targets_target;
                                ptr = target_ptr;
                            }
                        }
                        Interior::Unallocated => unreachable!(),
                    }
                }
            }
        }*/

        /// `ptr` must be [Interior::SubstitutesTo]
        fn resolve_substitution_chain_recurse<T: Debug + Clone, IDMarker: UUIDMarker>(
            ptr: *mut Interior<T, IDMarker>,
            ptr_target: UUID<IDMarker>,
            substitutor: &AppendOnlyAlloc<&UnifyableCell<T, IDMarker>, IDMarker>,
        ) -> UUID<IDMarker> {
            unsafe {
                let target = substitutor.copy_elem(ptr_target);

                let target_ptr: *mut _ = target.0.get();
                match &*target_ptr {
                    Interior::Known(_) => ptr_target,
                    Interior::SubstitutesTo(targets_target) => {
                        // Yes, this gets overwritten at the end, but we need to already be updating pointers to contract the final cycle so we can detect it
                        *ptr = Interior::SubstitutesTo(*targets_target);

                        if *targets_target == ptr_target {
                            ptr_target
                        } else {
                            let new_target = resolve_substitution_chain_recurse(
                                target_ptr,
                                *targets_target,
                                substitutor,
                            );
                            // Not strictly necessary, but this makes *all* pointers in the chain point to the final node
                            *ptr = Interior::SubstitutesTo(new_target);
                            new_target
                        }
                    }
                    Interior::Unallocated => unreachable!(),
                }
            }
        }

        unsafe {
            let ptr: *mut Interior<T, IDMarker> = self.0.get();
            match &*ptr {
                Interior::Known(known) => ChainResolution::Known(known),
                Interior::SubstitutesTo(to_id) => {
                    let final_target_id =
                        resolve_substitution_chain_recurse(ptr, *to_id, substitutor);
                    let final_target = substitutor.copy_elem(final_target_id);
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
                    let result_id = substitutor.alloc(self);
                    *ptr = Interior::SubstitutesTo(result_id);
                    ChainResolution::Unknown(result_id)
                }
            }
        }
    }

    /// If unification of the content is required, returns Some(a_inner, b_inner)
    pub fn unify<'s>(
        a: &'s Self,
        b: &'s Self,
        substitutor: &AppendOnlyAlloc<&'s Self, IDMarker>,
    ) -> SubstitutionResult<'s, T> {
        let resol_a = a.resolve_substitution_chain(substitutor);
        let resol_b = b.resolve_substitution_chain(substitutor);

        match (resol_a, resol_b) {
            (ChainResolution::Known(a), ChainResolution::Known(b)) => {
                if std::ptr::eq(a, b) {
                    SubstitutionResult::NoActionNeeded // Minor optimization, when we see referential equality
                } else {
                    SubstitutionResult::Recurse((a, b))
                }
            }
            (ChainResolution::Known(_), ChainResolution::Unknown(id)) => {
                let _ = substitutor.set_elem(id, a);
                SubstitutionResult::NoActionNeeded
            }
            (ChainResolution::Unknown(id), ChainResolution::Known(_))
            | (ChainResolution::Unknown(id), ChainResolution::Unknown(_)) => {
                let _ = substitutor.set_elem(id, b);
                SubstitutionResult::NoActionNeeded
            }
        }
    }

    /// `recursively_subsitute` must call [Self::substitute] on every child [UnifyableCell]
    pub fn substitute<'s>(
        &'s self,
        substitutor: &AppendOnlyAlloc<&'s Self, IDMarker>,
        recursively_subsitute: impl FnOnce(&'s T),
    ) {
        if let Some(known) = self.get() {
            recursively_subsitute(known);
        } else {
            match self.resolve_substitution_chain(substitutor) {
                ChainResolution::Known(known) => unsafe {
                    recursively_subsitute(known);

                    // Because we've done the recusive_substitute, that should've called `substitute` on every recursive [UnifyableCell], `known` can safely be cloned, it can't contain any Unallocateds anymore
                    let known_clone = known.clone();

                    *self.0.get() = Interior::Known(known_clone);
                },
                ChainResolution::Unknown(_) => {}
            }
        }
    }

    /// When constructing types regular Clone is not a good option. It'll crash due to [Interior::Unallocated]. Instead this version either allocates a new variable for it, or copies the existing substitution
    pub fn clone_type<'s>(&'s self, substitutor: &AppendOnlyAlloc<&'s Self, IDMarker>) -> Self {
        let ptr: *mut _ = self.0.get();
        let new_id = match unsafe { &*ptr } {
            Interior::Known(_) => substitutor.alloc(self),
            Interior::SubstitutesTo(id) => *id,
            Interior::Unallocated => {
                let slf_id = substitutor.alloc(self);
                unsafe {
                    *ptr = Interior::SubstitutesTo(slf_id);
                }
                slf_id
            }
        };
        Self(UnsafeCell::new(Interior::SubstitutesTo(new_id)))
    }
}

impl<T: Debug + Clone, IDMarker: UUIDMarker> Deref for UnifyableCell<T, IDMarker> {
    type Target = T;

    #[track_caller]
    fn deref(&self) -> &T {
        let content_ptr: *const Interior<T, IDMarker> = self.0.get();
        // SAFETY: In shared context, once we're [Interior::Known] that reference will never be invalidated.
        let content = unsafe { &*content_ptr };
        let_unwrap!(Interior::Known(v), content);
        v
    }
}

impl<T: Debug + Clone, IDMarker: UUIDMarker> DerefMut for UnifyableCell<T, IDMarker> {
    #[track_caller]
    fn deref_mut(&mut self) -> &mut T {
        let_unwrap!(Interior::Known(v), self.0.get_mut());
        v
    }
}

impl<T: Debug + Clone, IDMarker: UUIDMarker> From<T> for UnifyableCell<T, IDMarker> {
    fn from(known: T) -> Self {
        Self(UnsafeCell::new(Interior::Known(known)))
    }
}

impl<T: Debug + Clone, IDMarker: UUIDMarker> Clone for UnifyableCell<T, IDMarker> {
    fn clone(&self) -> Self {
        // We cast to a const pointer here instead, such that we never actually create a &mut that might conflict with another existing shared ref
        let content_ptr: *const Interior<T, IDMarker> = self.0.get();
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

pub enum ChainResolution<'s, T, IDMarker: UUIDMarker> {
    Known(&'s T),
    Unknown(UUID<IDMarker>),
}

pub enum SubstitutionResult<'s, T> {
    NoActionNeeded,
    Recurse((&'s T, &'s T)),
}

#[derive(Debug, Clone)]
enum PeanoTypeInner {
    Zero,
    Succ(Box<PeanoType>),
}
type PeanoType = UnifyableCell<PeanoTypeInner, PeanoVariableIDMarker>;

impl PeanoType {
    fn unify_peanos<'s>(
        l: &'s Self,
        r: &'s Self,
        substitutor: &AppendOnlyAlloc<&'s Self, PeanoVariableIDMarker>,
    ) -> bool {
        match Self::unify(l, r, substitutor) {
            SubstitutionResult::NoActionNeeded => true,
            SubstitutionResult::Recurse(content) => match content {
                (PeanoTypeInner::Zero, PeanoTypeInner::Zero) => true,
                (PeanoTypeInner::Succ(lc), PeanoTypeInner::Succ(rc)) => {
                    Self::unify_peanos(lc, rc, substitutor)
                }
                _ => false,
            },
        }
    }
    fn fully_substitute<'s>(
        &'s self,
        substitutor: &AppendOnlyAlloc<&'s Self, PeanoVariableIDMarker>,
    ) {
        self.substitute(substitutor, |inner| match inner {
            PeanoTypeInner::Zero => {}
            PeanoTypeInner::Succ(inner) => inner.fully_substitute(substitutor),
        })
    }
    fn count(&self) -> usize {
        match self.get() {
            Some(PeanoTypeInner::Zero) => 0,
            Some(PeanoTypeInner::Succ(inner)) => inner.count() + 1,
            None => panic!("Unknown in Peano!"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn add_to(mut cur: PeanoType, up_to: usize) -> PeanoType {
        for _ in 0..up_to {
            cur = PeanoTypeInner::Succ(Box::new(cur)).into();
        }

        cur
    }
    fn mk_peano(up_to: usize) -> PeanoType {
        add_to(PeanoTypeInner::Zero.into(), up_to)
    }

    #[test]
    fn test_peano_initial() {
        let a = mk_peano(4);
        let b = mk_peano(2);

        let unknown = PeanoType::UNKNOWN;

        unknown.set_initial(PeanoTypeInner::Zero);

        assert_eq!(a.count(), 4);
        assert_eq!(b.count(), 2);
        assert_eq!(unknown.count(), 0);
    }
    #[test]
    fn test_peano_unify() {
        let substitutor = AppendOnlyAlloc::new();

        let four = mk_peano(4);

        let a = PeanoType::UNKNOWN;
        let three_plus_a = add_to(a.clone_type(&substitutor), 3);

        PeanoType::unify_peanos(&four, &three_plus_a, &substitutor);

        a.fully_substitute(&substitutor);

        assert_eq!(a.count(), 1)
    }

    #[test]
    fn test_non_infinite_peano() {
        let substitutor = AppendOnlyAlloc::new();

        let a = PeanoType::UNKNOWN;
        let a_plus_zero = add_to(a.clone_type(&substitutor), 0);

        PeanoType::unify_peanos(&a, &a_plus_zero, &substitutor);

        a.fully_substitute(&substitutor);
    }

    #[test]
    fn test_infinite_peano() {
        let substitutor = AppendOnlyAlloc::new();

        let a = PeanoType::UNKNOWN;
        let a_plus_one = add_to(a.clone_type(&substitutor), 1);

        println!("Gets Stuck at Unify");
        PeanoType::unify_peanos(&a, &a_plus_one, &substitutor);

        println!("Gets Stuck at fully_substitute");
        a.fully_substitute(&substitutor);
    }
}
