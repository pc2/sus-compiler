use std::{cell::UnsafeCell, fmt::Debug};

use crate::{let_unwrap, typing::type_inference::UnifyResult};

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
    fn replace_substitution_with_known(&self, originally_substituted_to: Option<usize>, known: T) {
        // SAFETY: We're currently not yet Interior::Known, so we can safely replace our internals with the new known value.
        // To catch logic errors, we explicitly check that we did originally point to the given substitution
        unsafe {
            let interior_ptr: *mut Interior<T> = self.0.get();
            match *interior_ptr {
                Interior::Known(_) => unreachable!(),
                Interior::SubstitutesTo(id) => {
                    assert_eq!(Some(id), originally_substituted_to);
                    *interior_ptr = Interior::Known(known);
                }
                Interior::Unallocated => {
                    assert_eq!(None, originally_substituted_to);
                    *interior_ptr = Interior::Known(known);
                }
            }
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
                    "Not fully known substitutables can't be Cloned at all! Use [Unifier::clone_obj] or [Unifier::clone_prototype] to make clones."
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

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub struct VarID(usize);

#[derive(Debug)]
pub enum ChainResolution<T> {
    Known(T),
    /// If no ID is provided, then that means this chain is a [Interior::Unallocated], and is therefore globally unique.
    Unknown(Option<VarID>),
}

/// This struct bookkeeps the extra state for a Hindley Mindley Union-Find algorithm. It contains the counterparts to [UniCell]'s [Interior::SubstitutesTo]'s ID field.
/// All references are to [UniCell]s in the field. If a new value needs to be injected into the graph of [UniCell]s, then it should be [UniCell::set_initial].
///
/// To use, you should implement [Unifier], which should include a [Substitutor] field.
///
/// For examples see [PeanoUnifier]
///
/// The [Unifier] trait gets a bit complicated, as it has many members that need to be implemented,
/// but this complicatedness allows it to be able to be used for more complex multi-substitutor unifiers.
/// `&mut self` is passed down manually, and so is able to be presented at the interface.
/// Everything the user needs for unification should be part of their custom [Unifier]-implementing struct.
#[derive(Debug)]
pub struct Substitutor<'s, T: Debug>(Vec<&'s UniCell<T>>);

impl<'s, T: Debug> Substitutor<'s, T> {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    /// Creates a new substitution map that points to the passed-in object.
    /// Returns the ID of this map.
    fn alloc(&mut self, obj: &'s UniCell<T>) -> usize {
        let idx = self.0.len();
        self.0.push(obj);
        idx
    }

    /// `ptr` must be [Interior::SubstitutesTo]
    fn resolve_chain_recurse(&mut self, ptr: *mut Interior<T>, ptr_target: usize) -> usize {
        unsafe {
            let target = self.0[ptr_target];

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
    pub fn resolve_substitution_chain(&mut self, obj: &'s UniCell<T>) -> ChainResolution<&'s T> {
        unsafe {
            let interior_ptr: *mut Interior<T> = obj.0.get();
            match &*interior_ptr {
                Interior::Known(known) => ChainResolution::Known(known),
                Interior::SubstitutesTo(to_id) => {
                    let final_target_id = self.resolve_chain_recurse(interior_ptr, *to_id);
                    let final_target = self.0[final_target_id];
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
}

/// By implementing [Unifier], you gain access to:
/// - [Unifier::unify]
/// - [Unifier::fully_substitute]
/// - [Unifier::clone_obj]
/// - [Unifier::clone_prototype]
pub trait Unifier<'s, T: Debug> {
    /// Your Unifier should have a field `substitutor: Substitutor<'s, T>`, return it here.
    fn get_substitutor_mut(&mut self) -> &mut Substitutor<'s, T>;

    /// `contains_subtree` is used to prevent infinite types.
    /// It must be implemented using [Substitutor::resolve_substitution_chain] to iterate through its subtrees.
    /// If a subtree is found that contains the given pointer it must return true.
    fn contains_subtree(&mut self, in_obj: &'s T, subtree_var: VarID) -> bool;

    /// `unify_subtrees` should recursively call [Unifier::unify] for every pair of subtrees.
    /// If some irreconcilable difference is found it should return [UnifyResult::Failure].
    /// Otherwise return the binary AND of subtree unifications.
    /// Regardless of failure, all subtrees should be unified for best possible type error information.
    /// You as the user should never return [UnifyResult::FailureInfiniteTypes]
    fn unify_subtrees(&mut self, left: &'s T, right: &'s T) -> UnifyResult;

    /// `recursively_subsitute` must call [Unifier::substitute] on every child [UniCell]
    fn recursively_subsitute(&mut self, obj: &'s T) -> bool;

    /// Should call [Unifier::clone_obj] on all the sub-fields of this object.
    fn clone_obj_recurse(&mut self, obj: &'s T) -> T;

    /// `clone_prototype_recurse` must recursively call [Unifier::clone_prototype] on parts of T that it clones.
    fn clone_prototype_recurse(&mut self, obj: &'s T) -> T;

    fn unify(&mut self, a: &'s UniCell<T>, b: &'s UniCell<T>) -> UnifyResult {
        let subs = self.get_substitutor_mut();

        let resol_a = subs.resolve_substitution_chain(a);
        let resol_b = subs.resolve_substitution_chain(b);

        match (resol_a, resol_b) {
            (ChainResolution::Known(a), ChainResolution::Known(b)) => {
                if std::ptr::eq(a, b) {
                    UnifyResult::Success // Minor but common optimization, when we see referential equality we can immediately exit
                } else {
                    self.unify_subtrees(a, b)
                }
            }
            (ChainResolution::Known(known), ChainResolution::Unknown(id)) => {
                if let Some(id) = id {
                    if self.contains_subtree(known, id) {
                        // Always have to check contains_subtree. Could be that a contains b which was uninit
                        return UnifyResult::FailureInfiniteTypes;
                    }
                }
                let known_clone = self.clone_obj_recurse(known);
                b.replace_substitution_with_known(id.map(|v| v.0), known_clone);
                if let Some(id) = id {
                    let subs = self.get_substitutor_mut();
                    subs.0[id.0] = a;
                }
                UnifyResult::Success
            }
            (ChainResolution::Unknown(id), ChainResolution::Known(known)) => {
                if let Some(id) = id {
                    if self.contains_subtree(known, id) {
                        // Always have to check contains_subtree. Could be that a contains b which was uninit
                        return UnifyResult::FailureInfiniteTypes;
                    }
                }
                let known_clone = self.clone_obj_recurse(known);
                a.replace_substitution_with_known(id.map(|v| v.0), known_clone);
                if let Some(id) = id {
                    let subs = self.get_substitutor_mut();
                    subs.0[id.0] = b;
                }
                UnifyResult::Success
            }
            (ChainResolution::Unknown(id_a), ChainResolution::Unknown(id_b)) => {
                match (id_a, id_b) {
                    (None, None) => {
                        let id_a = subs.alloc(a);
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
                        subs.0[id_a.0] = b;
                    }
                }
                UnifyResult::Success
            }
        }
    }

    /// Returns `true` if substitution was successful (the resulting type is now fully known and clone-able)
    fn fully_substitute(&mut self, obj: &'s UniCell<T>) -> bool {
        if let Some(known) = obj.get() {
            self.recursively_subsitute(known)
        } else {
            let subs = self.get_substitutor_mut();
            match subs.resolve_substitution_chain(obj) {
                ChainResolution::Known(known) => {
                    self.recursively_subsitute(known);

                    // Because we've done the recusive_substitute,
                    // that should've called `substitute` on every recursive [UniCell],
                    // `known` can safely be cloned, it can't contain any Unallocateds anymore
                    let known_clone = self.clone_obj_recurse(known);

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
    fn clone_obj(&mut self, obj: &'s UniCell<T>) -> UniCell<T> {
        unsafe {
            let subs = self.get_substitutor_mut();
            let interior_ptr: *mut Interior<T> = obj.0.get();
            let new_interior = match &*interior_ptr {
                Interior::Known(known) => {
                    let known_clone = self.clone_obj_recurse(known);
                    Interior::Known(known_clone)
                }
                Interior::SubstitutesTo(id) => Interior::SubstitutesTo(*id),
                Interior::Unallocated => {
                    let slf_id = subs.alloc(obj);
                    *interior_ptr = Interior::SubstitutesTo(slf_id);
                    Interior::SubstitutesTo(slf_id)
                }
            };
            UniCell(UnsafeCell::new(new_interior))
        }
    }

    /// Custom Clone function exclusively for cloning just-created types, that have no unification relations yet.
    /// It means that any variables that have been created without being [UniCell::set_initial] will become new distinct variables from the original.
    /// May panic if [Unifier::unify] has ever been called on it.
    fn clone_prototype(&mut self, obj: &'s UniCell<T>) -> UniCell<T> {
        unsafe {
            let interior_ptr: *mut Interior<T> = obj.0.get();
            let new_interior = match &*interior_ptr {
                Interior::Known(known) => Interior::Known(self.clone_prototype_recurse(known)),
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

#[derive(Debug, Default)]
struct PeanoUnifier<'inst> {
    substitutor: Substitutor<'inst, PeanoType>,
}

impl<'inst> PeanoUnifier<'inst> {
    pub fn new() -> Self {
        Self {
            substitutor: Substitutor::new(),
        }
    }
}

impl<'inst> Unifier<'inst, PeanoType> for PeanoUnifier<'inst> {
    fn get_substitutor_mut(&mut self) -> &mut Substitutor<'inst, PeanoType> {
        &mut self.substitutor
    }

    fn contains_subtree(&mut self, in_obj: &'inst PeanoType, target: VarID) -> bool {
        match in_obj {
            PeanoType::Zero => false,
            PeanoType::Succ(subtree_cell) => {
                match self.substitutor.resolve_substitution_chain(subtree_cell) {
                    ChainResolution::Known(known) => self.contains_subtree(known, target),
                    ChainResolution::Unknown(var) => var == Some(target),
                }
            }
        }
    }

    fn unify_subtrees(&mut self, lc: &'inst PeanoType, rc: &'inst PeanoType) -> UnifyResult {
        match (lc, rc) {
            (PeanoType::Zero, PeanoType::Zero) => UnifyResult::Success,
            (PeanoType::Succ(lc), PeanoType::Succ(rc)) => self.unify(lc, rc),
            _ => UnifyResult::Failure,
        }
    }

    fn recursively_subsitute(&mut self, inner: &'inst PeanoType) -> bool {
        match inner {
            PeanoType::Zero => true,
            PeanoType::Succ(inner) => self.fully_substitute(inner),
        }
    }

    fn clone_obj_recurse(&mut self, inner: &'inst PeanoType) -> PeanoType {
        match inner {
            PeanoType::Zero => PeanoType::Zero,
            PeanoType::Succ(succ) => PeanoType::Succ(Box::new(self.clone_obj(succ))),
        }
    }

    fn clone_prototype_recurse(&mut self, inner: &'inst PeanoType) -> PeanoType {
        match inner {
            PeanoType::Zero => PeanoType::Zero,
            PeanoType::Succ(succ) => PeanoType::Succ(Box::new(self.clone_prototype(succ))),
        }
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
        let mut substitutor = PeanoUnifier::new();

        let four = mk_peano_cell(4);

        let a = PeanoType::UNKNOWN;
        let three_plus_a = add_to_cell(substitutor.clone_obj(&a), 3);

        substitutor.unify(&four, &three_plus_a);

        substitutor.fully_substitute(&a);

        assert_eq!(a.unwrap().count(), 1)
    }

    #[test]
    fn test_non_infinite_peano() {
        let mut substitutor = PeanoUnifier::new();

        let a = PeanoType::UNKNOWN;
        let a_plus_zero = add_to_cell(substitutor.clone_obj(&a), 0);

        assert_eq!(substitutor.unify(&a, &a_plus_zero), UnifyResult::Success);
        assert_eq!(substitutor.unify(&a_plus_zero, &a), UnifyResult::Success);

        substitutor.fully_substitute(&a);
    }

    #[test]
    fn test_invalid_unification() {
        let mut substitutor = PeanoUnifier::new();

        let three = mk_peano_cell(3);
        let four = mk_peano_cell(4);

        assert_eq!(substitutor.unify(&three, &four), UnifyResult::Failure);
        assert_eq!(substitutor.unify(&four, &three), UnifyResult::Failure);

        substitutor.fully_substitute(&three);
        substitutor.fully_substitute(&four);
    }

    #[test]
    fn test_infinite_peano() {
        let mut substitutor = PeanoUnifier::new();

        let a = PeanoType::UNKNOWN;
        let a_plus_one = add_to_cell(substitutor.clone_obj(&a), 1);

        assert_eq!(
            substitutor.unify(&a, &a_plus_one),
            UnifyResult::FailureInfiniteTypes
        );
        assert_eq!(
            substitutor.unify(&a_plus_one, &a),
            UnifyResult::FailureInfiniteTypes
        );

        substitutor.fully_substitute(&a);
    }

    #[test]
    fn test_peano_equivalence_simple() {
        let mut substitutor = PeanoUnifier::new();

        let one = mk_peano_cell(1);
        let two = mk_peano_cell(2);
        let one_plus_three = add_to_cell(substitutor.clone_obj(&one), 3);
        let two_plus_two = add_to_cell(substitutor.clone_obj(&two), 2);

        // 2+2 == 1+3
        assert_eq!(
            substitutor.unify(&two_plus_two, &one_plus_three),
            UnifyResult::Success
        );
    }

    #[test]
    fn test_peano_multiple_variables_chain() {
        let mut substitutor = PeanoUnifier::new();

        let x = PeanoType::UNKNOWN;
        let y = PeanoType::UNKNOWN;
        let z = PeanoType::UNKNOWN;

        // x = 2, y = x + 1, z = y + 1
        x.set_initial(PeanoType::Zero);
        let x_plus_2 = add_to_cell(substitutor.clone_obj(&x), 2);
        let y_val = add_to_cell(substitutor.clone_obj(&x), 1);
        let z_val = add_to_cell(substitutor.clone_obj(&y), 1);

        // Unify y with x+1, z with y+1, and z with x+2
        assert_eq!(substitutor.unify(&y, &y_val), UnifyResult::Success);
        assert_eq!(substitutor.unify(&z, &z_val), UnifyResult::Success);
        assert_eq!(substitutor.unify(&z, &x_plus_2), UnifyResult::Success);

        substitutor.fully_substitute(&x);
        substitutor.fully_substitute(&y);
        substitutor.fully_substitute(&z);

        assert_eq!(x.unwrap().count(), 0);
        assert_eq!(y.unwrap().count(), 1);
        assert_eq!(z.unwrap().count(), 2);
    }

    #[test]
    fn test_peano_complex_substitution_graph() {
        let mut substitutor = PeanoUnifier::new();

        let a = PeanoType::UNKNOWN;
        let b = PeanoType::UNKNOWN;
        let c = PeanoType::UNKNOWN;

        // a = 2, b = a + 2, c = b + 1
        a.set_initial(PeanoType::Zero);
        let b_val = add_to_cell(substitutor.clone_obj(&a), 2);
        let c_val = add_to_cell(substitutor.clone_obj(&b), 1);

        assert_eq!(substitutor.unify(&b, &b_val), UnifyResult::Success);
        assert_eq!(substitutor.unify(&c, &c_val), UnifyResult::Success);

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

        let mut substitutor = PeanoUnifier::new();

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

                    let _ = substitutor.unify(a, b);
                }
                2 => {
                    // Fully substitute something
                    let a = cells.choose(&mut rng).unwrap();

                    if substitutor.fully_substitute(a) {
                        // Can clone values after a successful substitute
                        let _a_clone = a.clone();
                    }
                }
                _ => unreachable!(),
            }
        }
    }
}
