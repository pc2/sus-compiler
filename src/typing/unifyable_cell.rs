use std::{cell::UnsafeCell, fmt::Debug};

use crate::{append_only_vec::AppendOnlyVec, let_unwrap};

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

enum ChainResolution<'s, T> {
    Known(&'s T),
    Unknown(usize),
}

pub enum SubstitutionResult<'s, T> {
    NoActionNeeded,
    Recurse((&'s T, &'s T)),
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
    fn resolve_substitution_chain(&self, typ: &'s UniCell<T>) -> ChainResolution<'s, T> {
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

    /// If unification of the content is required, returns Some(a_inner, b_inner)
    pub fn unify(&self, a: &'s UniCell<T>, b: &'s UniCell<T>) -> SubstitutionResult<'s, T> {
        let resol_a = self.resolve_substitution_chain(a);
        let resol_b = self.resolve_substitution_chain(b);

        match (resol_a, resol_b) {
            (ChainResolution::Known(a), ChainResolution::Known(b)) => {
                if std::ptr::eq(a, b) {
                    SubstitutionResult::NoActionNeeded // Minor optimization, when we see referential equality
                } else {
                    SubstitutionResult::Recurse((a, b))
                }
            }
            (ChainResolution::Known(_), ChainResolution::Unknown(id)) => {
                let _ = self.0.set_elem(id, a);
                SubstitutionResult::NoActionNeeded
            }
            (ChainResolution::Unknown(id), ChainResolution::Known(_))
            | (ChainResolution::Unknown(id), ChainResolution::Unknown(_)) => {
                let _ = self.0.set_elem(id, b);
                SubstitutionResult::NoActionNeeded
            }
        }
    }

    /// `recursively_subsitute` must call [Self::substitute] on every child [UniCell]
    pub fn substitute(&self, typ: &'s UniCell<T>, recursively_subsitute: impl FnOnce(&'s T)) {
        if let Some(known) = typ.get() {
            recursively_subsitute(known);
        } else {
            match self.resolve_substitution_chain(typ) {
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
    pub const UNKNOWN: UniCell<PeanoType> = UniCell::<PeanoType>::UNKNOWN;

    fn unify_peanos<'s>(
        l: &'s UniCell<Self>,
        r: &'s UniCell<Self>,
        substitutor: &Substitutor<'s, PeanoType>,
    ) -> bool {
        match substitutor.unify(l, r) {
            SubstitutionResult::NoActionNeeded => true,
            SubstitutionResult::Recurse(content) => match content {
                (PeanoType::Zero, PeanoType::Zero) => true,
                (PeanoType::Succ(lc), PeanoType::Succ(rc)) => {
                    Self::unify_peanos(lc, rc, substitutor)
                }
                _ => false,
            },
        }
    }
}

impl UniCell<PeanoType> {
    fn fully_substitute<'s>(&'s self, substitutor: &Substitutor<'s, PeanoType>) {
        substitutor.substitute(self, |inner| match inner {
            PeanoType::Zero => {}
            PeanoType::Succ(inner) => inner.fully_substitute(substitutor),
        })
    }
    fn count(&self) -> usize {
        match self.get() {
            Some(PeanoType::Zero) => 0,
            Some(PeanoType::Succ(inner)) => inner.count() + 1,
            None => panic!("Unknown in Peano!"),
        }
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

        assert_eq!(a.count(), 4);
        assert_eq!(b.count(), 2);
        assert_eq!(unknown.count(), 0);
    }
    #[test]
    fn test_peano_unify() {
        let substitutor = Substitutor::new();

        let four = mk_peano(4);

        let a = PeanoType::UNKNOWN;
        let three_plus_a = add_to(substitutor.clone_type(&a), 3);

        PeanoType::unify_peanos(&four, &three_plus_a, &substitutor);

        a.fully_substitute(&substitutor);

        assert_eq!(a.count(), 1)
    }

    #[test]
    fn test_non_infinite_peano() {
        let substitutor = Substitutor::new();

        let a = PeanoType::UNKNOWN;
        let a_plus_zero = add_to(substitutor.clone_type(&a), 0);

        PeanoType::unify_peanos(&a, &a_plus_zero, &substitutor);

        a.fully_substitute(&substitutor);
    }

    #[test]
    fn test_infinite_peano() {
        let substitutor = Substitutor::new();

        let a = PeanoType::UNKNOWN;
        let a_plus_one = add_to(substitutor.clone_type(&a), 1);

        println!("Gets Stuck at Unify");
        PeanoType::unify_peanos(&a, &a_plus_one, &substitutor);

        println!("Gets Stuck at fully_substitute");
        a.fully_substitute(&substitutor);
    }
}
