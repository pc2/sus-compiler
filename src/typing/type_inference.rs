//! Implements the Hindley-Milner algorithm for Type Inference.

use std::cell::OnceCell;
use std::fmt::{Debug, Display, Formatter};
use std::ops::{BitAnd, BitAndAssign, Deref, DerefMut};

use crate::errors::ErrorInfo;
use crate::{let_unwrap, prelude::*};

use crate::alloc::{get2_mut, UUIDMarker, UUID};

use super::abstract_type::{AbstractInnerType, PeanoType};
use super::abstract_type::{AbstractRankedType, DomainType};

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

#[derive(Debug)]
pub struct FailedUnification<MyType> {
    pub found: MyType,
    pub expected: MyType,
    pub span: Span,
    pub context: String,
    pub infos: Vec<ErrorInfo>,
}

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

    fn unify_must_succeed(&mut self, a: &Self::MyType, b: &Self::MyType) {
        assert!(
            self.unify_total(a, b) == UnifyResult::Success,
            "This unification cannot fail. Usually because we're unifying with a Written Type: {a:?} <-> {b:?}"
        );
    }

    /// Has to be implemented separately per type
    ///
    /// Returns true when no Unknowns remain
    fn fully_substitute(&self, typ: &mut Self::MyType) -> bool;
}

impl Substitutor for TypeSubstitutor<DomainType> {
    type MyType = DomainType;

    fn fully_substitute(&self, typ: &mut DomainType) -> bool {
        match typ {
            DomainType::Generative | DomainType::Physical(_) => true, // Do nothing, These are done already
            DomainType::Unknown(var) => {
                *typ = *self[*var].get().expect("It's impossible for domain variables to remain, as any unset domain variable would have been replaced with a new physical domain");
                self.fully_substitute(typ)
            }
        }
    }

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

    fn fully_substitute(&self, typ: &mut PeanoType) -> bool {
        match typ {
            PeanoType::Succ(t) => self.fully_substitute(t),
            PeanoType::Zero => true,
            PeanoType::Unknown(var) => {
                let Some(replacement) = self[*var].get() else {
                    return false;
                };
                assert!(!std::ptr::eq(typ, replacement));
                *typ = replacement.clone();
                self.fully_substitute(typ)
            }
        }
    }
}
impl TypeSubstitutor<PeanoType> {
    pub fn alloc_unknown(&mut self) -> PeanoType {
        PeanoType::Unknown(self.alloc(OnceCell::new()))
    }
}

impl Substitutor for AbstractTypeSubstitutor {
    type MyType = AbstractRankedType;

    fn fully_substitute(&self, typ: &mut AbstractRankedType) -> bool {
        let inner_success = match &mut typ.inner {
            AbstractInnerType::Named(_) | AbstractInnerType::Template(_) => true, // Template Name & Name is included in get_hm_info
            AbstractInnerType::Unknown(var) => {
                if let Some(replacement) = self.inner_substitutor[*var].get() {
                    assert!(!std::ptr::eq(&typ.inner, replacement));
                    typ.inner = replacement.clone();
                    self.fully_substitute(typ)
                } else {
                    false
                }
            }
        };
        let rank_success = self.rank_substitutor.fully_substitute(&mut typ.rank);
        inner_success & rank_success
    }

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

enum KnownValue<T, ID> {
    Unknown(Vec<ID>),
    Known(T),
}

pub struct SetUnifier<T: Eq + Clone, IDMarker> {
    ptrs: FlatAlloc<usize, IDMarker>,
    known_values: Vec<KnownValue<T, UUID<IDMarker>>>,
}
impl<T: Eq + Clone, IDMarker> Default for SetUnifier<T, IDMarker> {
    fn default() -> Self {
        Self {
            ptrs: Default::default(),
            known_values: Default::default(),
        }
    }
}

/// Referencing [Unifyable::Unknown] is a strong code smell.
/// It is likely you should use [crate::typing::type_inference::TypeSubstitutor::unify_must_succeed]
/// or [crate::typing::type_inference::TypeSubstitutor::unify_report_error] instead
///
/// It should only occur in creation `Unifyable::Unknown(self.type_substitutor.alloc())`
pub enum Unifyable<T, IDMarker: UUIDMarker> {
    Set(T),
    Unknown(UUID<IDMarker>),
}

impl<T: Eq + Clone, IDMarker: UUIDMarker> Unifyable<T, IDMarker> {
    pub fn is_unknown(&self) -> bool {
        match self {
            Unifyable::Set(_) => false,
            Unifyable::Unknown(_) => true,
        }
    }
    pub fn get_substitution<'s>(&'s self, unifier: &'s SetUnifier<T, IDMarker>) -> Option<&'s T> {
        match self {
            Unifyable::Set(v) => Some(v),
            Unifyable::Unknown(id) => match &unifier.known_values[unifier.ptrs[*id]] {
                KnownValue::Unknown(_) => None,
                KnownValue::Known(new_v) => Some(new_v),
            },
        }
    }
    pub fn substitute(&mut self, unifier: &SetUnifier<T, IDMarker>) -> bool {
        match self {
            Unifyable::Set(_) => true,
            Unifyable::Unknown(id) => match &unifier.known_values[unifier.ptrs[*id]] {
                KnownValue::Unknown(_) => false,
                KnownValue::Known(new_v) => {
                    *self = Unifyable::Set(new_v.clone());
                    true
                }
            },
        }
    }
    pub fn unwrap_set(&self) -> &T {
        match self {
            Unifyable::Set(s) => s,
            Unifyable::Unknown(_) => panic!("unwrap_set not allowed to be Unknown"),
        }
    }
}

impl<T, IDMarker: UUIDMarker> Deref for Unifyable<T, IDMarker> {
    type Target = T;

    #[track_caller]
    fn deref(&self) -> &T {
        let Self::Set(v) = self else {
            unreachable!("Attempting to Deref a not-Set Unifyable!")
        };
        v
    }
}

impl<T: Display, IDMarker: UUIDMarker> Display for Unifyable<T, IDMarker> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Unifyable::Set(v) => v.fmt(f),
            Unifyable::Unknown(id) => f.write_fmt(format_args!("{id:?}")),
        }
    }
}

impl<T: Debug, IDMarker: UUIDMarker> Debug for Unifyable<T, IDMarker> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Unifyable::Set(v) => v.fmt(f),
            Unifyable::Unknown(id) => f.write_fmt(format_args!("{id:?}")),
        }
    }
}

impl<T: Clone, IDMarker: UUIDMarker> Clone for Unifyable<T, IDMarker> {
    fn clone(&self) -> Self {
        match self {
            Self::Set(arg0) => Self::Set(arg0.clone()),
            Self::Unknown(arg0) => Self::Unknown(*arg0),
        }
    }
}
impl<T: PartialEq + Debug, IDMarker: UUIDMarker> PartialEq for Unifyable<T, IDMarker> {
    fn eq(&self, other: &Self) -> bool {
        let_unwrap!(Self::Set(a), self);
        let_unwrap!(Self::Set(b), other);
        a.eq(b)
    }
}
impl<T: Eq + Debug, IDMarker: UUIDMarker> Eq for Unifyable<T, IDMarker> {}
impl<T: std::hash::Hash + Debug, IDMarker: UUIDMarker> std::hash::Hash for Unifyable<T, IDMarker> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        let_unwrap!(Self::Set(a), self);
        a.hash(state);
    }
}

impl<T: Eq + Clone, IDMarker: UUIDMarker> SetUnifier<T, IDMarker> {
    pub fn unify(&mut self, a: &Unifyable<T, IDMarker>, b: &Unifyable<T, IDMarker>) -> bool {
        match (a, b) {
            (Unifyable::Set(a), Unifyable::Set(b)) => a == b,
            (Unifyable::Set(v), Unifyable::Unknown(var))
            | (Unifyable::Unknown(var), Unifyable::Set(v)) => {
                let k = &mut self.known_values[self.ptrs[*var]];
                if let KnownValue::Known(k) = k {
                    k == v
                } else {
                    *k = KnownValue::Known(v.clone());
                    true
                }
            }
            (Unifyable::Unknown(idx_a), Unifyable::Unknown(idx_b)) => {
                let idx_a = self.ptrs[*idx_a];
                let idx_b = self.ptrs[*idx_b];
                let (old_vector, to) = match get2_mut(&mut self.known_values, idx_a, idx_b) {
                    Some((KnownValue::Unknown(a_v), KnownValue::Unknown(b_v))) => {
                        if a_v.len() > b_v.len() {
                            a_v.extend_from_slice(b_v);
                            (b_v, idx_a)
                        } else {
                            b_v.extend_from_slice(a_v);
                            (a_v, idx_b)
                        }
                    }
                    Some((KnownValue::Unknown(v), KnownValue::Known(_))) => (v, idx_a),
                    Some((KnownValue::Known(_), KnownValue::Unknown(v))) => (v, idx_b),
                    Some((KnownValue::Known(x), KnownValue::Known(y))) => return x == y,
                    None => return true,
                };

                for v in std::mem::take(old_vector) {
                    self.ptrs[v] = to;
                }

                true
            }
        }
    }

    pub fn alloc_unknown(&mut self) -> Unifyable<T, IDMarker> {
        let new_ptr = self.ptrs.alloc(self.known_values.len());
        self.known_values.push(KnownValue::Unknown(vec![new_ptr]));
        Unifyable::Unknown(new_ptr)
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
