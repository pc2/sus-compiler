//! Implements the Hindley-Milner algorithm for Type Inference.

use std::cell::{OnceCell, RefCell};
use std::fmt::Debug;
use std::marker::PhantomData;
use std::ops::{BitAnd, Deref, DerefMut, Index};
use std::thread::panicking;

use crate::block_vector::{BlockVec, BlockVecIter};
use crate::errors::ErrorInfo;
use crate::prelude::*;

use crate::alloc::{UUIDAllocator, UUIDMarker, UUIDRange, UUID};
use crate::value::Value;

use super::abstract_type::PeanoType;
use super::abstract_type::{AbstractInnerType, DomainType};
use super::concrete_type::ConcreteType;

pub struct InnerTypeVariableIDMarker;
impl UUIDMarker for InnerTypeVariableIDMarker {
    const DISPLAY_NAME: &'static str = "type_variable_";
}
pub type InnerTypeVariableID = UUID<InnerTypeVariableIDMarker>;

pub struct PeanoVariableIDMarker;
impl UUIDMarker for PeanoVariableIDMarker {
    const DISPLAY_NAME: &'static str = "peano_variable_";
}
pub type PeanoVariableID = UUID<PeanoVariableIDMarker>;

pub struct DomainVariableIDMarker;
impl UUIDMarker for DomainVariableIDMarker {
    const DISPLAY_NAME: &'static str = "domain_variable_";
}
pub type DomainVariableID = UUID<DomainVariableIDMarker>;

pub struct ConcreteTypeVariableIDMarker;
impl UUIDMarker for ConcreteTypeVariableIDMarker {
    const DISPLAY_NAME: &'static str = "concrete_type_variable_";
}
pub type ConcreteTypeVariableID = UUID<ConcreteTypeVariableIDMarker>;

pub struct FailedUnification<MyType> {
    pub found: MyType,
    pub expected: MyType,
    pub span: Span,
    pub context: String,
    pub infos: Vec<ErrorInfo>,
}

/// Pretty big block size so for most typing needs we only need one
const BLOCK_SIZE: usize = 512;

/// Implements Hindley-Milner type inference
///
/// It actually already does eager inference where possible (through [Self::unify])
///
/// When eager inference is not possible, [DelayedConstraintsList] should be used
pub struct TypeSubstitutor<MyType: HindleyMilner<VariableIDMarker>, VariableIDMarker: UUIDMarker> {
    substitution_map: BlockVec<OnceCell<MyType>, BLOCK_SIZE>,
    failed_unifications: RefCell<Vec<FailedUnification<MyType>>>,
    _ph: PhantomData<VariableIDMarker>,
}

impl<'v, MyType: HindleyMilner<VariableIDMarker> + 'v, VariableIDMarker: UUIDMarker> IntoIterator
    for &'v TypeSubstitutor<MyType, VariableIDMarker>
{
    type Item = &'v OnceCell<MyType>;

    type IntoIter = BlockVecIter<'v, OnceCell<MyType>, BLOCK_SIZE>;

    fn into_iter(self) -> Self::IntoIter {
        self.substitution_map.iter()
    }
}

impl<MyType: HindleyMilner<VariableIDMarker>, VariableIDMarker: UUIDMarker>
    Index<UUID<VariableIDMarker>> for TypeSubstitutor<MyType, VariableIDMarker>
{
    type Output = OnceCell<MyType>;

    fn index(&self, index: UUID<VariableIDMarker>) -> &Self::Output {
        &self.substitution_map[index.get_hidden_value()]
    }
}

/// To be passed to [TypeSubstitutor::unify_report_error]
pub trait UnifyErrorReport {
    fn report(self) -> (String, Vec<ErrorInfo>);
}
impl UnifyErrorReport for &str {
    fn report(self) -> (String, Vec<ErrorInfo>) {
        (self.to_string(), Vec::new())
    }
}
impl<F: FnOnce() -> (String, Vec<ErrorInfo>)> UnifyErrorReport for F {
    fn report(self) -> (String, Vec<ErrorInfo>) {
        self()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UnifyResult {
    Success,
    NoMatchingTypeFunc,
    NoInfiniteTypes,
}
impl BitAnd for UnifyResult {
    type Output = UnifyResult;

    fn bitand(self, rhs: Self) -> Self::Output {
        if self == UnifyResult::Success {
            rhs
        } else {
            self
        }
    }
}

impl<MyType: HindleyMilner<VariableIDMarker> + Clone + Debug, VariableIDMarker: UUIDMarker> Default
    for TypeSubstitutor<MyType, VariableIDMarker>
{
    fn default() -> Self {
        Self::new()
    }
}

impl<MyType: HindleyMilner<VariableIDMarker> + Clone + Debug, VariableIDMarker: UUIDMarker>
    TypeSubstitutor<MyType, VariableIDMarker>
{
    pub fn new() -> Self {
        Self {
            substitution_map: BlockVec::new(),
            failed_unifications: RefCell::new(Vec::new()),
            _ph: PhantomData,
        }
    }

    pub fn init(variable_alloc: &UUIDAllocator<VariableIDMarker>) -> Self {
        Self {
            substitution_map: variable_alloc
                .into_iter()
                .map(|_| OnceCell::new())
                .collect(),
            failed_unifications: RefCell::new(Vec::new()),
            _ph: PhantomData,
        }
    }

    pub fn alloc(&self) -> UUID<VariableIDMarker> {
        UUID::from_hidden_value(self.substitution_map.alloc(OnceCell::new()))
    }

    pub fn id_range(&self) -> UUIDRange<VariableIDMarker> {
        UUIDRange::new_with_length(self.substitution_map.len())
    }

    fn does_typ_reference_var_recurse_with_substitution(
        &self,
        does_this: &MyType,
        reference_this: UUID<VariableIDMarker>,
    ) -> bool {
        let mut does_it_reference_it = false;
        does_this.for_each_unknown(&mut |v: UUID<VariableIDMarker>| {
            if v == reference_this {
                does_it_reference_it = true;
            } else if let Some(found_substitution) = self[v].get() {
                does_it_reference_it |= self.does_typ_reference_var_recurse_with_substitution(
                    found_substitution,
                    reference_this,
                );
            }
        });
        does_it_reference_it
    }

    fn try_fill_empty_var<'s>(
        &'s self,
        empty_var: UUID<VariableIDMarker>,
        mut replace_with: &'s MyType,
    ) -> UnifyResult {
        assert!(self[empty_var].get().is_none());

        // 1-deep Unknowns should be dug out, becuase they don't create infinite types
        while let HindleyMilnerInfo::TypeVar(unknown_synonym) = replace_with.get_hm_info() {
            if let Some(found_subst) = self[unknown_synonym].get() {
                replace_with = found_subst;
            } else if unknown_synonym == empty_var {
                return UnifyResult::Success;
            } else {
                assert!(self[empty_var].set(replace_with.clone()).is_ok());
                return UnifyResult::Success;
            }
        }

        if self.does_typ_reference_var_recurse_with_substitution(replace_with, empty_var) {
            UnifyResult::NoInfiniteTypes
        } else {
            assert!(self[empty_var].set(replace_with.clone()).is_ok());
            UnifyResult::Success
        }
    }

    /// Returns false if the types couldn't be unified
    /// Unification is loosely based on this video: https://www.youtube.com/watch?v=KNbRLTLniZI
    ///
    /// The main change is that I don't keep a substitution list,
    /// but immediately apply substitutions to [Self::substitution_map]
    #[must_use]
    fn unify(&self, a: &MyType, b: &MyType) -> UnifyResult {
        let result = match (a.get_hm_info(), b.get_hm_info(), a, b) {
            (HindleyMilnerInfo::TypeVar(a_var), HindleyMilnerInfo::TypeVar(b_var), _, _) => {
                if a_var == b_var {
                    UnifyResult::Success // Same var, all ok
                } else {
                    match (self[a_var].get(), self[b_var].get()) {
                        (None, None) => {
                            assert!(self[a_var].set(b.clone()).is_ok());
                            UnifyResult::Success
                        }
                        (None, Some(subs_b)) => self.try_fill_empty_var(a_var, subs_b),
                        (Some(subs_a), None) => self.try_fill_empty_var(b_var, subs_a),
                        (Some(subs_a), Some(subs_b)) => self.unify(subs_a, subs_b),
                    }
                }
            }
            (HindleyMilnerInfo::TypeFunc(tf_a), HindleyMilnerInfo::TypeFunc(tf_b), _, _) => {
                if tf_a != tf_b {
                    UnifyResult::NoMatchingTypeFunc
                } else {
                    MyType::unify_all_args(a, b, &mut |arg_a, arg_b| self.unify(arg_a, arg_b))
                }
            }
            (HindleyMilnerInfo::TypeFunc(_), HindleyMilnerInfo::TypeVar(v), tf, _)
            | (HindleyMilnerInfo::TypeVar(v), HindleyMilnerInfo::TypeFunc(_), _, tf) => {
                if let Some(subs) = self[v].get() {
                    self.unify(subs, tf)
                } else {
                    self.try_fill_empty_var(v, tf)
                }
            }
        };

        // Very expensive, only enable if there are issues
        //#[cfg(debug_assertions)]
        //self.check_no_unknown_loop();
        result
    }
    pub fn unify_must_succeed(&self, a: &MyType, b: &MyType) {
        assert!(
            self.unify(a, b) == UnifyResult::Success,
            "This unification cannot fail. Usually because we're unifying with a Written Type"
        );
    }
    pub fn unify_report_error<Report: UnifyErrorReport>(
        &self,
        found: &MyType,
        expected: &MyType,
        span: Span,
        reporter: Report,
    ) {
        let unify_result = self.unify(found, expected);
        if unify_result != UnifyResult::Success {
            let (mut context, infos) = reporter.report();
            if unify_result == UnifyResult::NoInfiniteTypes {
                context.push_str(": Creating Infinite Types is Forbidden!");
            }
            self.failed_unifications
                .borrow_mut()
                .push(FailedUnification {
                    found: found.clone(),
                    expected: expected.clone(),
                    span,
                    context,
                    infos,
                });
        }
    }
    pub fn extract_errors(&mut self) -> Vec<FailedUnification<MyType>> {
        self.failed_unifications.replace(Vec::new())
    }

    pub fn iter(&self) -> BlockVecIter<'_, OnceCell<MyType>, BLOCK_SIZE> {
        self.into_iter()
    }

    /// Used for sanity-checking. The graph of Unknown nodes must be non-cyclical, such that we don't create infinite types
    ///
    /// Implements https://en.wikipedia.org/wiki/Tarjan%27s_strongly_connected_components_algorithm
    pub fn check_no_unknown_loop(&self) {
        #[derive(Clone, Copy)]
        struct NodeInfo {
            is_not_part_of_loop: bool,
            is_part_of_stack: bool,
        }
        fn is_node_infinite_loop<
            MyType: HindleyMilner<VariableIDMarker> + Clone + Debug,
            VariableIDMarker: UUIDMarker,
        >(
            slf: &TypeSubstitutor<MyType, VariableIDMarker>,
            node_in_path: &mut FlatAlloc<NodeInfo, VariableIDMarker>,
            unknown_id: UUID<VariableIDMarker>,
        ) -> bool {
            // This is the core check we're doing. If this triggers then we have an infinite loop
            if node_in_path[unknown_id].is_not_part_of_loop {
                // Early exit, so we don't do the same thing over and over
                return false;
            }

            let mut is_infinite_loop = false;

            if let Some(substitutes_to) = slf[unknown_id].get() {
                if node_in_path[unknown_id].is_part_of_stack {
                    is_infinite_loop = true;
                } else {
                    node_in_path[unknown_id].is_part_of_stack = true;
                    substitutes_to.for_each_unknown(&mut |id| {
                        if !is_infinite_loop && is_node_infinite_loop(slf, node_in_path, id) {
                            is_infinite_loop = true;
                        }
                    });
                    node_in_path[unknown_id].is_part_of_stack = false;
                }

                if is_infinite_loop {
                    eprintln!("{unknown_id:?} => {substitutes_to:?}");
                }
            }
            node_in_path[unknown_id].is_not_part_of_loop = true;
            is_infinite_loop
        }

        let mut node_in_path: FlatAlloc<NodeInfo, VariableIDMarker> = FlatAlloc::with_size(
            self.substitution_map.len(),
            NodeInfo {
                is_not_part_of_loop: false,
                is_part_of_stack: false,
            },
        );

        for id in self.id_range() {
            if !node_in_path[id].is_not_part_of_loop
                && is_node_infinite_loop(self, &mut node_in_path, id)
            {
                panic!(
                    "Cyclic Type Substitution Found! See Above. On node {id:?} => {:?}",
                    self[id]
                )
            }
        }
    }
}

impl<MyType: HindleyMilner<VariableIDMarker>, VariableIDMarker: UUIDMarker> Drop
    for TypeSubstitutor<MyType, VariableIDMarker>
{
    fn drop(&mut self) {
        if !panicking() {
            assert!(
                self.failed_unifications.borrow().is_empty(),
                "Errors were not extracted before dropping!"
            );
        }
    }
}

/// See [HindleyMilner]
///
/// `TypeFuncIdent` is a value object, that is a "terminal". Basically, it's an atom that can either be equal or not.
///
/// Usually this is type names: `int` `bool`, or "array of" without the containing type.
///
/// Basically, when unifying, `int` isn't equal to "array of", and thus a type error is generated
///
/// This enum itself then is either such a terminal, or a type variable that can be unified (IE substituted)
#[derive(Debug, Clone, Copy)]
pub enum HindleyMilnerInfo<TypeFuncIdent, VariableIDMarker: UUIDMarker> {
    /// Just a marker. Use [HindleyMilner::unify_all_args]
    TypeFunc(TypeFuncIdent),
    TypeVar(UUID<VariableIDMarker>),
}

/// Implements Hindley-Milner type unification for various types in the SUS language
///
/// Unification is roughly based on this video: https://www.youtube.com/watch?v=KNbRLTLniZI
/// The other HM videos are also highly recommended to understand this
///
/// Though this implementation does eager unification as much as possible, while unifications that cannot
/// be performed eagerly are handled by [DelayedConstraintsList].
pub trait HindleyMilner<VariableIDMarker: UUIDMarker>: Sized {
    type TypeFuncIdent<'slf>: Eq
    where
        Self: 'slf;

    fn get_hm_info(&self) -> HindleyMilnerInfo<Self::TypeFuncIdent<'_>, VariableIDMarker>;

    /// Iterate through all arguments and unify them
    ///
    /// If any pair couldn't be unified, return false
    ///
    /// This is never called by the user, only by [TypeSubstitutor::unify]
    fn unify_all_args<F: FnMut(&Self, &Self) -> UnifyResult>(
        left: &Self,
        right: &Self,
        unify: &mut F,
    ) -> UnifyResult;

    /// Has to be implemented separately per type
    ///
    /// Returns true when no Unknowns remain
    fn fully_substitute(&mut self, substitutor: &TypeSubstitutor<Self, VariableIDMarker>) -> bool;

    /// Recursively called for each Unknown that is part of this. Used by [TypeSubstitutor::check_no_unknown_loop]
    fn for_each_unknown(&self, f: &mut impl FnMut(UUID<VariableIDMarker>));

    fn contains_unknown(&self, var: UUID<VariableIDMarker>) -> bool {
        let mut contains_var = false;
        self.for_each_unknown(&mut |v: UUID<VariableIDMarker>| {
            if v == var {
                contains_var = true;
            }
        });
        contains_var
    }
}

/// [HindleyMilnerInfo] `TypeFuncIdent` for [AbstractType]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AbstractTypeHMInfo {
    Template(TemplateID),
    Named(TypeUUID),
}

impl HindleyMilner<InnerTypeVariableIDMarker> for AbstractInnerType {
    type TypeFuncIdent<'slf> = AbstractTypeHMInfo;

    fn get_hm_info(&self) -> HindleyMilnerInfo<AbstractTypeHMInfo, InnerTypeVariableIDMarker> {
        match self {
            AbstractInnerType::Unknown(var_id) => HindleyMilnerInfo::TypeVar(*var_id),
            AbstractInnerType::Template(template_id) => {
                HindleyMilnerInfo::TypeFunc(AbstractTypeHMInfo::Template(*template_id))
            }
            AbstractInnerType::Named(named_id) => {
                HindleyMilnerInfo::TypeFunc(AbstractTypeHMInfo::Named(*named_id))
            }
        }
    }

    fn unify_all_args<F: FnMut(&Self, &Self) -> UnifyResult>(
        left: &Self,
        right: &Self,
        _: &mut F,
    ) -> UnifyResult {
        match (left, right) {
            (AbstractInnerType::Template(na), AbstractInnerType::Template(nb)) => {
                assert!(*na == *nb);
                UnifyResult::Success
            } // Already covered by get_hm_info
            (AbstractInnerType::Named(na), AbstractInnerType::Named(nb)) => {
                assert!(*na == *nb);
                UnifyResult::Success
            } // Already covered by get_hm_info
            (_, _) => unreachable!("All others should have been eliminated by get_hm_info check"),
        }
    }

    fn fully_substitute(
        &mut self,
        substitutor: &TypeSubstitutor<Self, InnerTypeVariableIDMarker>,
    ) -> bool {
        match self {
            AbstractInnerType::Named(_) | AbstractInnerType::Template(_) => true, // Template Name & Name is included in get_hm_info
            AbstractInnerType::Unknown(var) => {
                let Some(replacement) = substitutor.substitution_map[var.get_hidden_value()].get()
                else {
                    return false;
                };
                assert!(!std::ptr::eq(self, replacement));
                *self = replacement.clone();
                self.fully_substitute(substitutor)
            }
        }
    }

    fn for_each_unknown(&self, f: &mut impl FnMut(InnerTypeVariableID)) {
        match self {
            AbstractInnerType::Template(_) | AbstractInnerType::Named(_) => {}
            AbstractInnerType::Unknown(uuid) => f(*uuid),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PeanoTypeHMInfo {
    Successor,
    Zero,
}

impl HindleyMilner<PeanoVariableIDMarker> for PeanoType {
    type TypeFuncIdent<'slf> = PeanoTypeHMInfo;

    fn get_hm_info(&self) -> HindleyMilnerInfo<PeanoTypeHMInfo, PeanoVariableIDMarker> {
        match self {
            PeanoType::Unknown(var_id) => HindleyMilnerInfo::TypeVar(*var_id),
            PeanoType::Succ(_) => HindleyMilnerInfo::TypeFunc(PeanoTypeHMInfo::Successor),
            PeanoType::Zero => HindleyMilnerInfo::TypeFunc(PeanoTypeHMInfo::Zero),
        }
    }

    fn unify_all_args<F: FnMut(&Self, &Self) -> UnifyResult>(
        left: &Self,
        right: &Self,
        unify: &mut F,
    ) -> UnifyResult {
        match (left, right) {
            (PeanoType::Zero, PeanoType::Zero) => UnifyResult::Success,
            (PeanoType::Succ(na), PeanoType::Succ(nb)) => unify(na, nb),
            (_, _) => unreachable!(
                "All others ({:?}, {:?}) should have been eliminated by get_hm_info check",
                left, right
            ),
        }
    }

    fn fully_substitute(
        &mut self,
        substitutor: &TypeSubstitutor<Self, PeanoVariableIDMarker>,
    ) -> bool {
        match self {
            PeanoType::Succ(typ) => typ.fully_substitute(substitutor),
            PeanoType::Zero => true,
            PeanoType::Unknown(var) => {
                let Some(replacement) = substitutor.substitution_map[var.get_hidden_value()].get()
                else {
                    return false;
                };
                assert!(!std::ptr::eq(self, replacement));
                *self = replacement.clone();
                self.fully_substitute(substitutor)
            }
        }
    }

    fn for_each_unknown(&self, f: &mut impl FnMut(PeanoVariableID)) {
        match self {
            PeanoType::Zero => {}
            PeanoType::Succ(typ) => typ.for_each_unknown(f),
            PeanoType::Unknown(uuid) => f(*uuid),
        }
    }
}

impl HindleyMilner<DomainVariableIDMarker> for DomainType {
    type TypeFuncIdent<'slf> = DomainID;

    fn get_hm_info(&self) -> HindleyMilnerInfo<DomainID, DomainVariableIDMarker> {
        match self {
            DomainType::Generative => unreachable!("No explicit comparisons with Generative Possible. These should be filtered out first"),
            DomainType::Physical(domain_id) => HindleyMilnerInfo::TypeFunc(*domain_id),
            DomainType::Unknown(var) => HindleyMilnerInfo::TypeVar(*var)
        }
    }

    fn unify_all_args<F: FnMut(&Self, &Self) -> UnifyResult>(
        _left: &Self,
        _right: &Self,
        _unify: &mut F,
    ) -> UnifyResult {
        // No sub-args
        UnifyResult::Success
    }

    /// For domains, always returns true. Or rather it should, since any leftover unconnected domains should be assigned an ID of their own by the type checker
    fn fully_substitute(
        &mut self,
        substitutor: &TypeSubstitutor<Self, DomainVariableIDMarker>,
    ) -> bool {
        match self {
            DomainType::Generative | DomainType::Physical(_) => true, // Do nothing, These are done already
            DomainType::Unknown(var) => {
                *self = *substitutor.substitution_map[var.get_hidden_value()].get().expect("It's impossible for domain variables to remain, as any unset domain variable would have been replaced with a new physical domain");
                self.fully_substitute(substitutor)
            }
        }
    }

    fn for_each_unknown(&self, f: &mut impl FnMut(DomainVariableID)) {
        match self {
            DomainType::Generative | DomainType::Physical(_) => {}
            DomainType::Unknown(uuid) => f(*uuid),
        }
    }
}

/// [HindleyMilnerInfo] `TypeFuncIdent` for [ConcreteType]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ConcreteTypeHMInfo<'slf> {
    Named(TypeUUID),
    Value(&'slf Value),
    Array,
}

impl HindleyMilner<ConcreteTypeVariableIDMarker> for ConcreteType {
    type TypeFuncIdent<'slf> = ConcreteTypeHMInfo<'slf>;

    fn get_hm_info(&self) -> HindleyMilnerInfo<ConcreteTypeHMInfo, ConcreteTypeVariableIDMarker> {
        match self {
            ConcreteType::Unknown(var_id) => HindleyMilnerInfo::TypeVar(*var_id),
            ConcreteType::Named(named_id) => {
                HindleyMilnerInfo::TypeFunc(ConcreteTypeHMInfo::Named(named_id.id))
            }
            ConcreteType::Value(v) => HindleyMilnerInfo::TypeFunc(ConcreteTypeHMInfo::Value(v)),
            ConcreteType::Array(_) => HindleyMilnerInfo::TypeFunc(ConcreteTypeHMInfo::Array),
        }
    }

    fn unify_all_args<F: FnMut(&Self, &Self) -> UnifyResult>(
        left: &Self,
        right: &Self,
        unify: &mut F,
    ) -> UnifyResult {
        match (left, right) {
            (ConcreteType::Named(na), ConcreteType::Named(nb)) => {
                assert!(*na == *nb);
                UnifyResult::Success
            } // Already covered by get_hm_info
            (ConcreteType::Value(v_1), ConcreteType::Value(v_2)) => {
                assert!(*v_1 == *v_2);
                UnifyResult::Success
            } // Already covered by get_hm_info
            (ConcreteType::Array(arr_typ_1), ConcreteType::Array(arr_typ_2)) => {
                let (arr_typ_1_arr, arr_typ_1_sz) = arr_typ_1.deref();
                let (arr_typ_2_arr, arr_typ_2_sz) = arr_typ_2.deref();
                unify(arr_typ_1_arr, arr_typ_2_arr) & unify(arr_typ_1_sz, arr_typ_2_sz)
            }
            (_, _) => unreachable!("All others should have been eliminated by get_hm_info check"),
        }
    }

    fn fully_substitute(
        &mut self,
        substitutor: &TypeSubstitutor<Self, ConcreteTypeVariableIDMarker>,
    ) -> bool {
        match self {
            ConcreteType::Named(_) | ConcreteType::Value(_) => true, // Don't need to do anything, this is already final
            ConcreteType::Array(arr_typ) => {
                let (arr_typ, arr_sz) = arr_typ.deref_mut();
                arr_typ.fully_substitute(substitutor) && arr_sz.fully_substitute(substitutor)
            }
            ConcreteType::Unknown(var) => {
                let Some(replacement) = substitutor.substitution_map[var.get_hidden_value()].get()
                else {
                    return false;
                };
                *self = replacement.clone();
                self.fully_substitute(substitutor)
            }
        }
    }

    fn for_each_unknown(&self, f: &mut impl FnMut(ConcreteTypeVariableID)) {
        match self {
            ConcreteType::Named(_) | ConcreteType::Value(_) => {}
            ConcreteType::Unknown(uuid) => f(*uuid),
            ConcreteType::Array(arr_typ) => {
                let (arr_typ, arr_sz) = arr_typ.deref();
                arr_typ.for_each_unknown(f);
                arr_sz.for_each_unknown(f);
            }
        }
    }
}

/// See [DelayedConstraintsList::resolve_delayed_constraints]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DelayedConstraintStatus {
    /// The constraint can be removed
    Resolved,
    /// Progress was made, (potentially enabling other parts to continue), but the constraint cannot be removed
    #[allow(unused)]
    Progress,
    /// No progress was made, if all constraints return [DelayedConstraintStatus::NoProgress] then type resolution deadlocked and cannot finish.
    NoProgress,
}

/// Implement this for any typing constraints that can't be resolved immediately.
///
/// See [DelayedConstraintsList]
pub trait DelayedConstraint<T> {
    fn try_apply(&mut self, shared_object: &mut T) -> DelayedConstraintStatus;
    fn report_could_not_resolve_error(&self, shared_object: &T);
}

/// This is for unification of constraints that may not be resolveable right away
///
/// Such as struct field access. vec.x cannot resolve the type of x before the type of vec has been resolved
///
/// The given function should only make changes when it can be successfully resolved
///
/// When the constraint has been resolved, it should return 'true'
///
/// For convenience, a &mut T is provided such that a shared mutable object can be used
pub struct DelayedConstraintsList<T>(Vec<Box<dyn DelayedConstraint<T>>>);

impl<T> DelayedConstraintsList<T> {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    /// Add a constraint
    pub fn push<C: DelayedConstraint<T> + 'static>(&mut self, constraint: C) {
        self.0.push(Box::new(constraint));
    }

    /// Will keep looping over the list of constraints, and try to apply them.
    ///
    /// Calls [DelayedConstraint::report_could_not_resolve_error] on all constraints that weren't resolved
    pub fn resolve_delayed_constraints(mut self, shared_object: &mut T) {
        while !self.0.is_empty() {
            let mut progress_made = false;
            self.0
                .retain_mut(|constraint| match constraint.try_apply(shared_object) {
                    DelayedConstraintStatus::Resolved => {
                        progress_made = true;
                        false
                    }
                    DelayedConstraintStatus::Progress => {
                        progress_made = true;
                        true
                    }
                    DelayedConstraintStatus::NoProgress => true,
                });
            if !progress_made {
                for constraint in std::mem::take(&mut self.0) {
                    constraint.report_could_not_resolve_error(shared_object);
                }
                return; // Exit
            }
        }
    }
}

impl<T> Drop for DelayedConstraintsList<T> {
    fn drop(&mut self) {
        if !std::thread::panicking() {
            assert_eq!(self.0.len(), 0, "DelayedConstraintsList was not resolved.");
        }
    }
}
