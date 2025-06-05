//! Implements the Hindley-Milner algorithm for Type Inference.

use std::cell::OnceCell;
use std::fmt::Debug;
use std::ops::{BitAnd, BitAndAssign, Deref, DerefMut};

use crate::errors::ErrorInfo;
use crate::prelude::*;

use crate::alloc::{UUIDMarker, UUID};
use crate::typing::domain_type::DomainType;

use super::abstract_type::AbstractRankedType;
use super::abstract_type::{AbstractInnerType, PeanoType};

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
#[allow(unused)]
pub type ConcreteTypeVariableID = UUID<ConcreteTypeVariableIDMarker>;

/// Implements Hindley-Milner type inference
///
/// It actually already does eager inference where possible (through [Self::unify])
///
/// When eager inference is not possible, [DelayedConstraintsList] should be used
pub type TypeSubstitutor<MyType> =
    FlatAlloc<OnceCell<MyType>, <MyType as HindleyMilner>::VariableIDMarker>;

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
impl BitAndAssign for UnifyResult {
    fn bitand_assign(&mut self, rhs: Self) {
        if *self == UnifyResult::Success {
            *self = rhs;
        }
    }
}

impl<MyType: HindleyMilner> TypeSubstitutor<MyType> {
    fn does_typ_reference_var_recurse_with_substitution(
        &self,
        does_this: &MyType,
        reference_this: UUID<MyType::VariableIDMarker>,
    ) -> bool {
        let mut does_it_reference_it = false;
        does_this.for_each_unknown(&mut |v: UUID<MyType::VariableIDMarker>| {
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
        empty_var: UUID<MyType::VariableIDMarker>,
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

    /// Used for sanity-checking. The graph of Unknown nodes must be non-cyclical, such that we don't create infinite types
    ///
    /// Implements https://en.wikipedia.org/wiki/Tarjan%27s_strongly_connected_components_algorithm
    pub fn check_no_unknown_loop(&self)
    where
        MyType: Debug,
    {
        #[derive(Clone, Copy)]
        struct NodeInfo {
            is_not_part_of_loop: bool,
            is_part_of_stack: bool,
        }
        fn is_node_infinite_loop<MyType: HindleyMilner + Clone + Debug>(
            slf: &TypeSubstitutor<MyType>,
            node_in_path: &mut FlatAlloc<NodeInfo, MyType::VariableIDMarker>,
            unknown_id: UUID<MyType::VariableIDMarker>,
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

        let mut node_in_path: FlatAlloc<NodeInfo, MyType::VariableIDMarker> = FlatAlloc::with_size(
            self.len(),
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
pub trait HindleyMilner: Sized + Clone {
    type TypeFuncIdent<'slf>: Eq
    where
        Self: 'slf;

    type VariableIDMarker: UUIDMarker;

    fn get_hm_info(&self) -> HindleyMilnerInfo<Self::TypeFuncIdent<'_>, Self::VariableIDMarker>;

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

    /// Recursively called for each Unknown that is part of this. Used by [TypeSubstitutor::check_no_unknown_loop]
    fn for_each_unknown(&self, f: &mut impl FnMut(UUID<Self::VariableIDMarker>));

    fn contains_unknown(&self, var: UUID<Self::VariableIDMarker>) -> bool {
        let mut contains_var = false;
        self.for_each_unknown(&mut |v: UUID<Self::VariableIDMarker>| {
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

impl HindleyMilner for AbstractInnerType {
    type TypeFuncIdent<'slf> = AbstractTypeHMInfo;
    type VariableIDMarker = InnerTypeVariableIDMarker;

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

impl HindleyMilner for PeanoType {
    type TypeFuncIdent<'slf> = PeanoTypeHMInfo;
    type VariableIDMarker = PeanoVariableIDMarker;

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

    fn for_each_unknown(&self, f: &mut impl FnMut(PeanoVariableID)) {
        match self {
            PeanoType::Zero => {}
            PeanoType::Succ(typ) => typ.for_each_unknown(f),
            PeanoType::Unknown(uuid) => f(*uuid),
        }
    }
}

impl HindleyMilner for DomainType {
    type TypeFuncIdent<'slf> = DomainID;
    type VariableIDMarker = DomainVariableIDMarker;

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

    fn for_each_unknown(&self, f: &mut impl FnMut(DomainVariableID)) {
        match self {
            DomainType::Generative | DomainType::Physical(_) => {}
            DomainType::Unknown(uuid) => f(*uuid),
        }
    }
}

pub trait Substitutor {
    type MyType: Clone + Debug;
    fn unify_total(&mut self, from: &Self::MyType, to: &Self::MyType) -> UnifyResult;
}

impl Substitutor for TypeSubstitutor<DomainType> {
    type MyType = DomainType;

    fn unify_total(&mut self, from: &DomainType, to: &DomainType) -> UnifyResult {
        self.unify(from, to)
    }
}

impl TypeSubstitutor<DomainType> {
    pub fn alloc_unknown(&mut self) -> DomainType {
        DomainType::Unknown(self.alloc(OnceCell::new()))
    }
}

#[derive(Debug, Default)]
pub struct AbstractTypeSubstitutor {
    pub inner_substitutor: TypeSubstitutor<AbstractInnerType>,
    pub rank_substitutor: TypeSubstitutor<PeanoType>,
}

impl Substitutor for TypeSubstitutor<PeanoType> {
    type MyType = PeanoType;

    fn unify_total(&mut self, from: &PeanoType, to: &PeanoType) -> UnifyResult {
        self.unify(from, to)
    }
}
impl TypeSubstitutor<PeanoType> {
    pub fn alloc_unknown(&mut self) -> PeanoType {
        PeanoType::Unknown(self.alloc(OnceCell::new()))
    }
}

impl Substitutor for AbstractTypeSubstitutor {
    type MyType = AbstractRankedType;

    fn unify_total(&mut self, from: &AbstractRankedType, to: &AbstractRankedType) -> UnifyResult {
        self.inner_substitutor.unify(&from.inner, &to.inner)
            & self.rank_substitutor.unify(&from.rank, &to.rank)
    }
}

impl AbstractTypeSubstitutor {
    pub fn alloc_unknown(&mut self) -> AbstractRankedType {
        AbstractRankedType {
            inner: AbstractInnerType::Unknown(self.inner_substitutor.alloc(OnceCell::new())),
            rank: PeanoType::Unknown(self.rank_substitutor.alloc(OnceCell::new())),
        }
    }
}

impl DomainType {
    pub fn fully_substitute(&mut self, substitutor: &TypeSubstitutor<Self>) -> bool {
        match self {
            DomainType::Generative | DomainType::Physical(_) => true, // Do nothing, These are done already
            DomainType::Unknown(var) => {
                *self = *substitutor[*var].get().expect("It's impossible for domain variables to remain, as any unset domain variable would have been replaced with a new physical domain");
                self.fully_substitute(substitutor)
            }
        }
    }
}

impl PeanoType {
    pub fn fully_substitute(&mut self, substitutor: &TypeSubstitutor<Self>) -> bool {
        match self {
            PeanoType::Succ(t) => t.fully_substitute(substitutor),
            PeanoType::Zero => true,
            PeanoType::Unknown(var) => {
                let Some(replacement) = substitutor[*var].get() else {
                    return false;
                };
                assert!(!std::ptr::eq(self, replacement));
                *self = replacement.clone();
                self.fully_substitute(substitutor)
            }
        }
    }
}

impl AbstractRankedType {
    pub fn fully_substitute(&mut self, substitutor: &AbstractTypeSubstitutor) -> bool {
        let inner_success = match &mut self.inner {
            AbstractInnerType::Named(_) | AbstractInnerType::Template(_) => true, // Template Name & Name is included in get_hm_info
            AbstractInnerType::Unknown(var) => {
                if let Some(replacement) = substitutor.inner_substitutor[*var].get() {
                    assert!(!std::ptr::eq(&self.inner, replacement));
                    self.inner = replacement.clone();
                    self.fully_substitute(substitutor)
                } else {
                    false
                }
            }
        };
        let rank_success = self.rank.fully_substitute(&substitutor.rank_substitutor);
        inner_success & rank_success
    }
}

#[derive(Debug)]
pub struct FailedUnification<MyType> {
    pub found: MyType,
    pub expected: MyType,
    pub span: Span,
    pub context: String,
    pub infos: Vec<ErrorInfo>,
}

pub struct TypeUnifier<S: Substitutor> {
    substitutor: S,
    failed_unifications: Vec<FailedUnification<S::MyType>>,
}

impl<S: Substitutor> From<S> for TypeUnifier<S> {
    fn from(substitutor: S) -> Self {
        Self {
            substitutor,
            failed_unifications: Vec::new(),
        }
    }
}

impl<S: Substitutor> TypeUnifier<S> {
    pub fn unify_report_error<Report: UnifyErrorReport>(
        &mut self,
        found: &S::MyType,
        expected: &S::MyType,
        span: Span,
        reporter: Report,
    ) {
        let unify_result = self.substitutor.unify_total(found, expected);
        if unify_result != UnifyResult::Success {
            let (mut context, infos) = reporter.report();
            if unify_result == UnifyResult::NoInfiniteTypes {
                context.push_str(": Creating Infinite Types is Forbidden!");
            }
            self.failed_unifications.push(FailedUnification {
                found: found.clone(),
                expected: expected.clone(),
                span,
                context,
                infos,
            });
        }
    }
    pub fn extract_errors(&mut self) -> Vec<FailedUnification<S::MyType>> {
        std::mem::take(&mut self.failed_unifications)
    }
}

impl<S: Substitutor + Default> Default for TypeUnifier<S> {
    fn default() -> Self {
        Self {
            substitutor: Default::default(),
            failed_unifications: Default::default(),
        }
    }
}

impl<S: Substitutor> Deref for TypeUnifier<S> {
    type Target = S;

    fn deref(&self) -> &Self::Target {
        &self.substitutor
    }
}

impl<S: Substitutor> DerefMut for TypeUnifier<S> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.substitutor
    }
}

impl<S: Substitutor> TypeUnifier<S> {}

impl<S: Substitutor> Drop for TypeUnifier<S> {
    fn drop(&mut self) {
        if !std::thread::panicking() {
            assert!(
                self.failed_unifications.is_empty(),
                "Errors were not extracted before dropping!"
            );
        }
    }
}
