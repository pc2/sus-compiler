use std::{cell::UnsafeCell, fmt::Debug};

use crate::{append_only_vec::AppendOnlyVec, let_unwrap, typing::type_inference::UnifyResult};

/// Basically a [std::cell::OnceCell] for type checking. We implement it safely by maintaining the following invariant:
///
/// - [UniCell] starts out [UniCell::UNKNOWN]. No interior references can be taken in this state. (But the type variable we refer to *can* be updated)
/// - At some point, it is set to [Interior::Known]. After this point references to this interior value can be taken.
///   Afterwards, we can *never* reset a [Interior::Known] back to an [Interior::Unallocated] or [Interior::SubstitutesTo], or mess with it in any mutable way. (Panics when trying otherwise)
pub struct UniCell<T: Debug>(UnsafeCell<Interior<T>>);
enum Interior<T: Debug> {
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

    pub fn get(&self) -> Option<&T> {
        // SAFETY: In shared context, once we're [Interior::Known] that reference will never be invalidated.
        unsafe {
            // We never actually create a &mut that might conflict with another existing shared ref
            let interior_ptr: *const Interior<T> = self.0.get();
            match &*interior_ptr {
                Interior::Known(known) => Some(known),
                Interior::SubstitutesTo(_) | Interior::Unallocated => None,
            }
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
    pub fn unwrap(&self) -> &T {
        // SAFETY: In shared context, once we're [Interior::Known] that reference will never be invalidated.
        unsafe {
            let interior_ptr: *const Interior<T> = self.0.get();
            let_unwrap!(Interior::Known(v), &*interior_ptr);
            v
        }
    }
    #[track_caller]
    pub fn unwrap_mut(&mut self) -> &mut T {
        let_unwrap!(Interior::Known(v), self.0.get_mut());
        v
    }

    pub fn is_unallocated(&self) -> bool {
        // SAFETY: In shared context, once we're [Interior::Known] that reference will never be invalidated.
        unsafe {
            // We cast to a const pointer here instead, such that we never actually create a &mut that might conflict with another existing shared ref
            let interior_ptr: *const Interior<T> = self.0.get();
            match &*interior_ptr {
                Interior::Known(_) | Interior::SubstitutesTo(_) => false,
                Interior::Unallocated => true,
            }
        }
    }

    /// Panics if [Substitutor::unify] has ever been called on this
    ///
    /// So only allowed if [Self::is_unallocated]
    pub fn set_initial(&self, v: T) {
        // SAFETY: Either this is already Interior::Known, in which case we panic before messing with a shared ref, OR this is not Interior::Known, and therefore no interior shared references have been given out
        //
        // Careful here! If we use the &mut for the check, then we've technically created UB between here and the panic. That's why we do a regular shared reborrow, and only create the mutable ref when we're sure no shares exist
        unsafe {
            let interior_ptr: *mut Interior<T> = self.0.get();

            if !matches!(&*interior_ptr, Interior::Unallocated) {
                unreachable!(
                    "`set_initial({v:?})` on a UniCell that's not in the Unallocated state ({:?})!",
                    &*interior_ptr
                );
            }

            // SAFETY: Only now do we actually use the ptr mutably
            *interior_ptr = Interior::Known(v)
        }
    }

    /// Links a [Interior::Unallocated] to a given value.
    fn init_uninit_to(&self, id: usize) {
        // SAFETY: Shared references to &'s T can only exist when [Interior::Known] is selected.
        // But for this call it must be [Interior::Unallocated].
        // We are being very careful not to create a &mut before we've asserted we're [Interior::Unallocated]
        unsafe {
            let interior_ptr: *mut Interior<T> = self.0.get();
            assert!(matches!(&*interior_ptr, Interior::Unallocated));
            *interior_ptr = Interior::SubstitutesTo(id);
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
        // SAFETY: In shared context, once we're [Interior::Known] that reference will never be invalidated.
        unsafe {
            // We cast to a const pointer here instead, such that we never actually create a &mut that might conflict with another existing shared ref
            let interior_ptr: *const Interior<T> = self.0.get();
            let Interior::Known(known) = &*interior_ptr else {
                panic!(
                    "Not fully known substitutables can't be Cloned at all! Use [Substitutor::clone_obj] or [Substitutor::clone_prototype] to make clones."
                )
            };
            let known_clone = known.clone();
            Self(UnsafeCell::new(Interior::Known(known_clone)))
        }
    }
}

impl<T: Debug> Debug for UniCell<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        unsafe {
            let interior_ptr: *const Interior<T> = self.0.get();
            f.debug_tuple("UniCell").field(&*interior_ptr).finish()
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

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
pub struct VarID(usize);

pub enum ChainResolution<T> {
    Known(T),
    /// If no ID is provided, then that means this chain is a [Interior::Unallocated], and is therefore globally unique.
    Unknown(Option<VarID>),
}

/// This struct bookkeeps the extra state for a Hindley Mindley Union-Find algorithm. It contains the counterparts to [UniCell]'s [Interior::SubstitutesTo]'s ID field.
/// All references are to [UniCell]s in the field. If a new value needs to be injected into the graph of [UniCell]s, then it should be [UniCell::set_initial].
///
/// To use, you should make custom wrappers around:
/// - [Self::unify]
/// - [Self::get_with_substitution]
/// - [Self::substitute]
/// - [Self::clone_obj]
/// - [Self::clone_prototype]
///
/// For examples see `impl<'s> Substitutor<'s, PeanoType>`
///
/// The reason we don't do a custom trait, but rather require these wrappers, is that a trait makes using nested substitutors more cumbersome.
/// We'd have to pass "extra data" (the nested substitutors) down the call hierarchy manually.
///
/// [Substitutor] references are *shared* on purpose (I've tried to replace them with &mut many times before).
/// The reason is that shared refs allow for more ergonomic recursive implementations of [Self::unify] and friends.
/// If we're building a [Substitutor] wrapper that includes more data (like delayed constraints for instance), then
/// going through the trouble with &mut refs is not worth it. Passing it along the call stack is also no bueno,
/// we'd have to pass the unifier itself, plus whatever extra data the user wants to attach to it. Lots of complexity for nothing.
pub struct Substitutor<'s, T: Clone + Debug> {
    substitutor: AppendOnlyVec<&'s UniCell<T>>,
}

impl<'s, T: Clone + Debug> Substitutor<'s, T> {
    pub fn new() -> Self {
        Self {
            substitutor: AppendOnlyVec::new(),
        }
    }

    /// Creates a new substitution map that points to the passed-in object.
    /// Returns the ID of this map.
    fn alloc(&self, obj: &'s UniCell<T>) -> usize {
        let idx = self.substitutor.len();
        self.substitutor.push(obj);
        idx
    }

    /// `ptr` must be [Interior::SubstitutesTo]
    fn resolve_chain_recurse(&self, ptr: *mut Interior<T>, ptr_target: usize) -> usize {
        unsafe {
            let target = self.substitutor.copy_elem(ptr_target);

            let target_ptr: *mut Interior<T> = target.0.get();
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
                Interior::Unallocated => unreachable!(
                    "Cannot be reached from get_with_subsitutution, it's in the SubstitutesTo branch"
                ),
            }
        }
    }

    /// Resolves a possibly extensive chain of substitutions to a single node.
    ///
    /// Result is either:
    /// - [Interior::Known] is then of course a known value.
    /// - [Interior::SubstitutesTo] that points to itself, signifying no substitution known yet
    pub fn get_with_substitution(&self, obj: &'s UniCell<T>) -> ChainResolution<&'s T> {
        unsafe {
            let interior_ptr: *mut Interior<T> = obj.0.get();
            match &*interior_ptr {
                Interior::Known(known) => ChainResolution::Known(known),
                Interior::SubstitutesTo(to_id) => {
                    let final_target_id = self.resolve_chain_recurse(interior_ptr, *to_id);
                    let final_target = self.substitutor.copy_elem(final_target_id);
                    match &*final_target.0.get() {
                        Interior::Known(known) => ChainResolution::Known(known),
                        Interior::SubstitutesTo(final_target_id_copy) => {
                            assert_eq!(*final_target_id_copy, final_target_id);
                            ChainResolution::Unknown(Some(VarID(final_target_id)))
                        }
                        Interior::Unallocated => unreachable!(),
                    }
                }
                Interior::Unallocated => ChainResolution::Unknown(None),
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
        contains_subtree: impl FnOnce(&'s T, VarID) -> bool,
        unify_subtrees: impl FnOnce(&'s T, &'s T) -> UnifyResult,
    ) -> UnifyResult {
        let resol_a = self.get_with_substitution(a);
        let resol_b = self.get_with_substitution(b);

        match (resol_a, resol_b) {
            (ChainResolution::Known(a), ChainResolution::Known(b)) => {
                if std::ptr::eq(a, b) {
                    UnifyResult::Success // Minor but common optimization, when we see referential equality we can immediately exit
                } else {
                    unify_subtrees(a, b)
                }
            }
            (ChainResolution::Known(known), ChainResolution::Unknown(id)) => {
                let id = id.unwrap_or_else(|| {
                    let id_b = self.alloc(b);
                    b.init_uninit_to(id_b);
                    VarID(id_b)
                });
                if contains_subtree(known, id) {
                    // Always have to check contains_subtree. Could be that a contains b which was uninit
                    UnifyResult::FailureInfiniteTypes
                } else {
                    let _ = self.substitutor.set_elem(id.0, a);
                    UnifyResult::Success
                }
            }
            (ChainResolution::Unknown(id), ChainResolution::Known(known)) => {
                let id = id.unwrap_or_else(|| {
                    let id_a = self.alloc(a);
                    a.init_uninit_to(id_a);
                    VarID(id_a)
                });
                if contains_subtree(known, id) {
                    UnifyResult::FailureInfiniteTypes
                } else {
                    let _ = self.substitutor.set_elem(id.0, b);
                    UnifyResult::Success
                }
            }
            (ChainResolution::Unknown(id_a), ChainResolution::Unknown(id_b)) => {
                match (id_a, id_b) {
                    (None, None) => {
                        let id_a = self.alloc(a);
                        a.init_uninit_to(id_a);
                        b.init_uninit_to(id_a);
                    }
                    (None, Some(id_b)) => {
                        a.init_uninit_to(id_b.0);
                    }
                    (Some(id_a), None) => {
                        b.init_uninit_to(id_a.0);
                    }
                    (Some(id_a), Some(_id_b)) => {
                        let _ = self.substitutor.set_elem(id_a.0, b);
                    }
                }
                UnifyResult::Success
            }
        }
    }

    /// `recursively_subsitute` must call [Self::substitute] on every child [UniCell]
    ///
    /// Returns `true` if substitution was successful (the resulting type is now fully known and clone-able)
    pub fn substitute(
        &self,
        obj: &'s UniCell<T>,
        recursively_subsitute: impl FnOnce(&'s T) -> bool,
    ) -> bool {
        if let Some(known) = obj.get() {
            recursively_subsitute(known)
        } else {
            match self.get_with_substitution(obj) {
                ChainResolution::Known(known) => {
                    recursively_subsitute(known);

                    // Because we've done the recusive_substitute,
                    // that should've called `substitute` on every recursive [UniCell],
                    // `known` can safely be cloned, it can't contain any Unallocateds anymore
                    let known_clone = known.clone();

                    unsafe {
                        *obj.0.get() = Interior::Known(known_clone);
                    }

                    true
                }
                ChainResolution::Unknown(_) => false,
            }
        }
    }

    /// When constructing types regular Clone is not a good option. It'll crash due to [Interior::Unallocated].
    /// Instead this version either allocates a new variable for it, or copies the existing substitution.
    pub fn clone_obj(&self, obj: &'s UniCell<T>) -> UniCell<T> {
        unsafe {
            let interior_ptr: *mut Interior<T> = obj.0.get();
            let new_interior = match &*interior_ptr {
                Interior::Known(_) => {
                    let id = self.alloc(obj);
                    Interior::SubstitutesTo(id)
                }
                Interior::SubstitutesTo(id) => Interior::SubstitutesTo(*id),
                Interior::Unallocated => {
                    let slf_id = self.alloc(obj);
                    *interior_ptr = Interior::SubstitutesTo(slf_id);
                    Interior::SubstitutesTo(slf_id)
                }
            };
            UniCell(UnsafeCell::new(new_interior))
        }
    }

    /// Custom Clone function exclusively for cloning just-created types, that have no unification relations yet.
    /// It means that any variables that have been created without being [UniCell::set_initial] will become new distinct variables from the original.
    /// May panic if [Self::unify] has ever been called on it.
    ///
    /// `clone_recurse` must recursively call [Self::clone_prototype] on parts of T that it clones.
    pub fn clone_prototype(
        &self,
        obj: &'s UniCell<T>,
        clone_recurse: impl FnOnce(&'s T) -> T,
    ) -> UniCell<T> {
        unsafe {
            let interior_ptr: *mut Interior<T> = obj.0.get();
            let new_interior = match &*interior_ptr {
                Interior::Known(known) => Interior::Known(clone_recurse(known)),
                Interior::SubstitutesTo(_) => unreachable!(
                    "[Substitutor::clone_prototype] on a type that has had [Substitutor::unify] called on it before"
                ),
                Interior::Unallocated => Interior::Unallocated,
            };
            UniCell(UnsafeCell::new(new_interior))
        }
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

impl<'s> Substitutor<'s, PeanoType> {
    fn contains_subtree(&self, find_in: &'s PeanoType, target: VarID) -> bool {
        match find_in {
            PeanoType::Zero => false,
            PeanoType::Succ(subtree_cell) => match self.get_with_substitution(subtree_cell) {
                ChainResolution::Known(known) => self.contains_subtree(known, target),
                ChainResolution::Unknown(var) => var == Some(target),
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
    /// Returns `true` if fully_subsitutite succeeded, and thus the resulting type is now *known* and is clone-able.
    fn fully_substitute(&self, obj: &'s UniCell<PeanoType>) -> bool {
        self.substitute(obj, |inner| match inner {
            PeanoType::Zero => true,
            PeanoType::Succ(inner) => self.fully_substitute(inner),
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
        let substitutor = Substitutor::new();

        let four = mk_peano_cell(4);

        let a = PeanoType::UNKNOWN;
        let three_plus_a = add_to_cell(substitutor.clone_obj(&a), 3);

        substitutor.unify_peanos(&four, &three_plus_a);

        substitutor.fully_substitute(&a);

        assert_eq!(a.unwrap().count(), 1)
    }

    #[test]
    fn test_non_infinite_peano() {
        let substitutor = Substitutor::new();

        let a = PeanoType::UNKNOWN;
        let a_plus_zero = add_to_cell(substitutor.clone_obj(&a), 0);

        assert_eq!(
            substitutor.unify_peanos(&a, &a_plus_zero),
            UnifyResult::Success
        );
        assert_eq!(
            substitutor.unify_peanos(&a_plus_zero, &a),
            UnifyResult::Success
        );

        substitutor.fully_substitute(&a);
    }

    #[test]
    fn test_invalid_unification() {
        let substitutor = Substitutor::default();

        let three = mk_peano_cell(3);
        let four = mk_peano_cell(4);

        assert_eq!(
            substitutor.unify_peanos(&three, &four),
            UnifyResult::Failure
        );
        assert_eq!(
            substitutor.unify_peanos(&four, &three),
            UnifyResult::Failure
        );

        substitutor.fully_substitute(&three);
        substitutor.fully_substitute(&four);
    }

    #[test]
    fn test_infinite_peano() {
        let substitutor = Substitutor::new();

        let a = PeanoType::UNKNOWN;
        let a_plus_one = add_to_cell(substitutor.clone_obj(&a), 1);

        assert_eq!(
            substitutor.unify_peanos(&a, &a_plus_one),
            UnifyResult::FailureInfiniteTypes
        );
        assert_eq!(
            substitutor.unify_peanos(&a_plus_one, &a),
            UnifyResult::FailureInfiniteTypes
        );

        substitutor.fully_substitute(&a);
    }

    #[test]
    fn test_peano_equivalence_simple() {
        let substitutor = Substitutor::new();

        let one = mk_peano_cell(1);
        let two = mk_peano_cell(2);
        let one_plus_three = add_to_cell(substitutor.clone_obj(&one), 3);
        let two_plus_two = add_to_cell(substitutor.clone_obj(&two), 2);

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
        let x_plus_2 = add_to_cell(substitutor.clone_obj(&x), 2);
        let y_val = add_to_cell(substitutor.clone_obj(&x), 1);
        let z_val = add_to_cell(substitutor.clone_obj(&y), 1);

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
        let b_val = add_to_cell(substitutor.clone_obj(&a), 2);
        let c_val = add_to_cell(substitutor.clone_obj(&b), 1);

        assert_eq!(substitutor.unify_peanos(&b, &b_val), UnifyResult::Success);
        assert_eq!(substitutor.unify_peanos(&c, &c_val), UnifyResult::Success);

        substitutor.fully_substitute(&a);
        substitutor.fully_substitute(&b);
        substitutor.fully_substitute(&c);

        assert_eq!(a.unwrap().count(), 0);
        assert_eq!(b.unwrap().count(), 2);
        assert_eq!(c.unwrap().count(), 3);
    }

    /// Just a stress test to cover all possible code paths. To check under miri that everything is alright.
    #[test]
    fn stress_test_for_miri() {
        use rand::prelude::IndexedRandom;
        use rand::{Rng, SeedableRng};

        use typed_arena::Arena;

        let arena = Arena::new();
        let mut rng = rand::rngs::StdRng::seed_from_u64(42);

        // Create a bunch of unknowns
        let cells: Vec<UniCell<PeanoType>> = (0..100).map(|_| PeanoType::UNKNOWN).collect();

        let mut all_peanos_pool: Vec<&UniCell<PeanoType>> = cells.iter().collect();

        let substitutor = Substitutor::new();

        // Randomly set some initial values
        for cell in cells.iter().take(10) {
            cell.set_initial(mk_peano(rng.random_range(0..5)));
        }

        for _ in 0..1000 {
            match rng.random_range(0..3) {
                0 => {
                    // Add a computed successor
                    let ontu = cells.choose(&mut rng).unwrap();
                    let add_count = rng.random_range(0..5);
                    let new_cell = arena.alloc(add_to_cell(substitutor.clone_obj(ontu), add_count));
                    all_peanos_pool.push(&*new_cell);
                }
                1 => {
                    // Unify two peanos
                    let a = cells.choose(&mut rng).unwrap();
                    let b = cells.choose(&mut rng).unwrap();

                    let _ = substitutor.unify_peanos(a, b);
                }
                2 => {
                    // Fully substitute something
                    let a = cells.choose(&mut rng).unwrap();

                    if substitutor.fully_substitute(a) {
                        // Can clone values after a successful fully_substitute
                        let _a_clone = a.clone();
                    }
                }
                _ => unreachable!(),
            }
        }
    }
}
