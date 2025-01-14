//! Implements the Hindley-Milner algorithm for Type Inference. 

use std::cell::{OnceCell, RefCell};
use std::fmt::Debug;
use std::marker::PhantomData;
use std::ops::{Deref, DerefMut, Index};
use std::thread::panicking;

use crate::block_vector::{BlockVec, BlockVecIter};
use crate::errors::ErrorInfo;
use crate::prelude::*;

use crate::alloc::{UUIDAllocator, UUIDMarker, UUID};
use crate::value::Value;

use super::abstract_type::AbstractType;
use super::abstract_type::DomainType;
use super::concrete_type::ConcreteType;

pub struct TypeVariableIDMarker;
impl UUIDMarker for TypeVariableIDMarker {
    const DISPLAY_NAME: &'static str = "type_variable_";
}
pub type TypeVariableID = UUID<TypeVariableIDMarker>;

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
    pub infos: Vec<ErrorInfo>
}

/// Pretty big block size so for most typing needs we only need one
const BLOCK_SIZE : usize = 512;

/// Implements Hindley-Milner type inference
/// 
/// It actually already does eager inference where possible (through [Self::unify])
/// 
/// When eager inference is not possible, [DelayedConstraintsList] should be used
pub struct TypeSubstitutor<MyType : HindleyMilner<VariableIDMarker>, VariableIDMarker : UUIDMarker> {
    substitution_map: BlockVec<OnceCell<MyType>, BLOCK_SIZE>,
    failed_unifications: RefCell<Vec<FailedUnification<MyType>>>,
    _ph: PhantomData<VariableIDMarker>
}

impl<'v, MyType: HindleyMilner<VariableIDMarker> + 'v, VariableIDMarker: UUIDMarker> IntoIterator for &'v TypeSubstitutor<MyType, VariableIDMarker> {
    type Item = &'v OnceCell<MyType>;

    type IntoIter = BlockVecIter<'v, OnceCell<MyType>, BLOCK_SIZE>;

    fn into_iter(self) -> Self::IntoIter {
        self.substitution_map.iter()
    }
}

impl<MyType : HindleyMilner<VariableIDMarker>, VariableIDMarker : UUIDMarker> Index<UUID<VariableIDMarker>> for TypeSubstitutor<MyType, VariableIDMarker> {
    type Output = OnceCell<MyType>;

    fn index(&self, index: UUID<VariableIDMarker>) -> &Self::Output {
        &self.substitution_map[index.get_hidden_value()]
    }
}

/// To be passed to [TypeSubstitutor::unify_report_error]
pub trait UnifyErrorReport {
    fn report(self) -> (String, Vec<ErrorInfo>);
}
impl<'s> UnifyErrorReport for &'s str {
    fn report(self) -> (String, Vec<ErrorInfo>) {
        (self.to_string(), Vec::new())
    }
}
impl<F : Fn() -> (String, Vec<ErrorInfo>)> UnifyErrorReport for F {
    fn report(self) -> (String, Vec<ErrorInfo>) {
        self()
    }
}



impl<MyType : HindleyMilner<VariableIDMarker>+Clone, VariableIDMarker : UUIDMarker> TypeSubstitutor<MyType, VariableIDMarker> {
    pub fn new() -> Self {
        Self {
            substitution_map: BlockVec::new(),
            failed_unifications: RefCell::new(Vec::new()),
            _ph: PhantomData
        }
    }
    
    pub fn init(variable_alloc : &UUIDAllocator<VariableIDMarker>) -> Self {
        Self {
            substitution_map: variable_alloc.into_iter().map(|_| OnceCell::new()).collect(),
            failed_unifications: RefCell::new(Vec::new()),
            _ph: PhantomData
        }
    }

    pub fn alloc(&self) -> UUID<VariableIDMarker> {
        UUID::from_hidden_value(self.substitution_map.alloc(OnceCell::new()))
    }

    /// Returns false if the types couldn't be unified
    #[must_use]
    fn unify(&self, a: &MyType, b: &MyType) -> bool {
        match a.get_hm_info() {
            HindleyMilnerInfo::TypeFunc(tf_a) => {
                match b.get_hm_info() {
                    HindleyMilnerInfo::TypeFunc(tf_b) => {
                        if tf_a != tf_b {
                            return false;
                        }
                        MyType::unify_all_args(a, b, &mut |arg_a, arg_b| {
                            self.unify(arg_a, arg_b)
                        })
                    }
                    HindleyMilnerInfo::TypeVar(_) => self.unify(b, a)
                }
            }
            HindleyMilnerInfo::TypeVar(var) => {
                if let HindleyMilnerInfo::TypeVar(var2) = b.get_hm_info() {
                    if var == var2 {
                        return true;
                    }
                }
                let typ_cell = &self.substitution_map[var.get_hidden_value()];
                if let Some(found) = typ_cell.get() {
                    self.unify(found, b)
                } else {
                    assert!(typ_cell.set(b.clone()).is_ok());
                    true
                }
            }
        }
    }
    pub fn unify_must_succeed(&self, a: &MyType, b: &MyType) {
        assert!(self.unify(a, b), "This unification cannot fail. Usually because we're unifying with a Written Type");
    }
    pub fn unify_report_error<Report : UnifyErrorReport>(&self, found: &MyType, expected: &MyType, span: Span, reporter: Report) {
        if !self.unify(found, expected) {
            let (context, infos) = reporter.report();
            self.failed_unifications.borrow_mut().push(FailedUnification {
                found: found.clone(),
                expected: expected.clone(),
                span,
                context,
                infos
            });
        }
    }
    pub fn extract_errors(&mut self) -> Vec<FailedUnification<MyType>> {
        self.failed_unifications.replace(Vec::new())
    }

    pub fn iter(&self) -> BlockVecIter<'_, OnceCell<MyType>, BLOCK_SIZE> {
        self.into_iter()
    }
}

impl<MyType: HindleyMilner<VariableIDMarker>, VariableIDMarker: UUIDMarker> Drop for TypeSubstitutor<MyType, VariableIDMarker> {
    fn drop(&mut self) {
        if !panicking() {
            assert!(self.failed_unifications.borrow().is_empty(), "Errors were not extracted before dropping!");
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
pub enum HindleyMilnerInfo<TypeFuncIdent, VariableIDMarker : UUIDMarker> {
    /// Just a marker. Use [HindleyMilner::unify_all_args]
    TypeFunc(TypeFuncIdent),
    TypeVar(UUID<VariableIDMarker>)
}

/// Implements Hindley-Milner type unification for various types in the SUS language
/// 
/// Unification is roughly based on this video: https://www.youtube.com/watch?v=KNbRLTLniZI
/// The other HM videos are also highly recommended to understand this
/// 
/// Though this implementation does eager unification as much as possible, while unifications that cannot
/// be performed eagerly are handled by [DelayedConstraintsList]. 
pub trait HindleyMilner<VariableIDMarker: UUIDMarker> : Sized {
    type TypeFuncIdent<'slf> : Eq where Self : 'slf;

    fn get_hm_info<'slf>(&'slf self) -> HindleyMilnerInfo<Self::TypeFuncIdent<'slf>, VariableIDMarker>;

    /// Iterate through all arguments and unify them
    /// 
    /// If any pair couldn't be unified, return false
    /// 
    /// This is never called by the user, only by [TypeSubstitutor::unify]
    fn unify_all_args<F : FnMut(&Self, &Self) -> bool>(left : &Self, right : &Self, unify : &mut F) -> bool;

    /// Has to be implemented separately per type
    /// 
    /// Returns true when no Unknowns remain
    fn fully_substitute(&mut self, substitutor: &TypeSubstitutor<Self, VariableIDMarker>) -> bool;
}

/// [HindleyMilnerInfo] `TypeFuncIdent` for [AbstractType]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AbstractTypeHMInfo {
    Template(TemplateID),
    Named(TypeUUID),
    Array
}

impl HindleyMilner<TypeVariableIDMarker> for AbstractType {
    type TypeFuncIdent<'slf> = AbstractTypeHMInfo;

    fn get_hm_info(&self) -> HindleyMilnerInfo<AbstractTypeHMInfo, TypeVariableIDMarker> {
        match self {
            AbstractType::Unknown(var_id) => HindleyMilnerInfo::TypeVar(*var_id),
            AbstractType::Template(template_id) => HindleyMilnerInfo::TypeFunc(AbstractTypeHMInfo::Template(*template_id)),
            AbstractType::Named(named_id) => HindleyMilnerInfo::TypeFunc(AbstractTypeHMInfo::Named(*named_id)),
            AbstractType::Array(_) => HindleyMilnerInfo::TypeFunc(AbstractTypeHMInfo::Array),
        }
    }

    fn unify_all_args<F : FnMut(&Self, &Self) -> bool>(left : &Self, right : &Self, unify : &mut F) -> bool {
        match (left, right) {
            (AbstractType::Template(na), AbstractType::Template(nb)) => {assert!(*na == *nb); true}, // Already covered by get_hm_info
            (AbstractType::Named(na), AbstractType::Named(nb)) => {assert!(*na == *nb); true}, // Already covered by get_hm_info
            (AbstractType::Array(arr_typ), AbstractType::Array(arr_typ_2)) => unify(arr_typ, arr_typ_2),
            (_, _) => unreachable!("All others should have been eliminated by get_hm_info check")
        }
    }

    fn fully_substitute(&mut self, substitutor: &TypeSubstitutor<Self, TypeVariableIDMarker>) -> bool {
        match self {
            AbstractType::Named(_) | AbstractType::Template(_) => true, // Template Name & Name is included in get_hm_info
            AbstractType::Array(arr_typ) => {
                arr_typ.fully_substitute(substitutor)
            },
            AbstractType::Unknown(var) => {
                let Some(replacement) = substitutor.substitution_map[var.get_hidden_value()].get() else {return false};
                *self = replacement.clone();
                self.fully_substitute(substitutor)
            }
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

    fn unify_all_args<F : FnMut(&Self, &Self) -> bool>(_left : &Self, _right : &Self, _unify : &mut F) -> bool {
        // No sub-args
        true
    }

    /// For domains, always returns true. Or rather it should, since any leftover unconnected domains should be assigned an ID of their own by the type checker
    fn fully_substitute(&mut self, substitutor: &TypeSubstitutor<Self, DomainVariableIDMarker>) -> bool {
        match self {
            DomainType::Generative | DomainType::Physical(_) => true, // Do nothing, These are done already
            DomainType::Unknown(var) => {
                *self = substitutor.substitution_map[var.get_hidden_value()].get().expect("It's impossible for domain variables to remain, as any unset domain variable would have been replaced with a new physical domain").clone();
                self.fully_substitute(substitutor)
            }
        }
    }
}


/// [HindleyMilnerInfo] `TypeFuncIdent` for [ConcreteType]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ConcreteTypeHMInfo<'slf> {
    Named(TypeUUID),
    Value(&'slf Value),
    Array
}


impl HindleyMilner<ConcreteTypeVariableIDMarker> for ConcreteType {
    type TypeFuncIdent<'slf> = ConcreteTypeHMInfo<'slf>;

    fn get_hm_info(&self) -> HindleyMilnerInfo<ConcreteTypeHMInfo, ConcreteTypeVariableIDMarker> {
        match self {
            ConcreteType::Unknown(var_id) => HindleyMilnerInfo::TypeVar(*var_id),
            ConcreteType::Named(named_id) => HindleyMilnerInfo::TypeFunc(ConcreteTypeHMInfo::Named(named_id.id)),
            ConcreteType::Value(v) => HindleyMilnerInfo::TypeFunc(ConcreteTypeHMInfo::Value(v)),
            ConcreteType::Array(_) => HindleyMilnerInfo::TypeFunc(ConcreteTypeHMInfo::Array),
        }
    }

    fn unify_all_args<F : FnMut(&Self, &Self) -> bool>(left : &Self, right : &Self, unify : &mut F) -> bool {
        match (left, right) {
            (ConcreteType::Named(na), ConcreteType::Named(nb)) => {assert!(*na == *nb); true} // Already covered by get_hm_info
            (ConcreteType::Value(v_1), ConcreteType::Value(v_2)) => {assert!(*v_1 == *v_2); true} // Already covered by get_hm_info
            (ConcreteType::Array(arr_typ_1), ConcreteType::Array(arr_typ_2)) => {
                let (arr_typ_1_arr, arr_typ_1_sz) = arr_typ_1.deref();
                let (arr_typ_2_arr, arr_typ_2_sz) = arr_typ_2.deref();
                unify(arr_typ_1_arr, arr_typ_2_arr) & unify(arr_typ_1_sz, arr_typ_2_sz)
            }
            (_, _) => unreachable!("All others should have been eliminated by get_hm_info check")
        }
    }

    fn fully_substitute(&mut self, substitutor: &TypeSubstitutor<Self, ConcreteTypeVariableIDMarker>) -> bool {
        match self {
            ConcreteType::Named(_) | ConcreteType::Value(_) => true, // Don't need to do anything, this is already final
            ConcreteType::Array(arr_typ) => {
                let (arr_typ, arr_sz) = arr_typ.deref_mut();
                arr_typ.fully_substitute(substitutor) &&
                arr_sz.fully_substitute(substitutor)
            },
            ConcreteType::Unknown(var) => {
                let Some(replacement) = substitutor.substitution_map[var.get_hidden_value()].get() else {return false};
                *self = replacement.clone();
                self.fully_substitute(substitutor)
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
    Progress,
    /// No progress was made, if all constraints return [DelayedConstraintStatus::NoProgress] then type resolution deadlocked and cannot finish. 
    NoProgress
}

/// Implement this for any typing constraints that can't be resolved immediately. 
/// 
/// See [DelayedConstraintsList]
pub trait DelayedConstraint<T> {
    fn try_apply(&mut self, shared_object : &mut T) -> DelayedConstraintStatus;
    fn report_could_not_resolve_error(&self, shared_object : &T);
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
    pub fn push<C : DelayedConstraint<T> + 'static>(&mut self, constraint: C) {
        self.0.push(Box::new(constraint));
    }

    /// Will keep looping over the list of constraints, and try to apply them. 
    /// 
    /// Calls [DelayedConstraint::report_could_not_resolve_error] on all constraints that weren't resolved
    pub fn resolve_delayed_constraints(mut self, shared_object: &mut T) {
        while self.0.len() > 0 {
            let mut progress_made = false;
            self.0.retain_mut(|constraint| {
                match constraint.try_apply(shared_object) {
                    DelayedConstraintStatus::Resolved => {progress_made = true; false}
                    DelayedConstraintStatus::Progress => {progress_made = true; true}
                    DelayedConstraintStatus::NoProgress => true
                }
            });
            if !progress_made {
                for constraint in std::mem::replace(&mut self.0, Vec::new()) {
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

