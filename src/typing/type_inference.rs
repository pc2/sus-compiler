//! Implements the Hindley-Milner algorithm for Type Inference. 

use std::cell::{OnceCell, RefCell};
use std::fmt::Debug;
use std::marker::PhantomData;
use std::ops::Index;

use crate::block_vector::BlockVec;
use crate::prelude::*;

use crate::alloc::{UUIDAllocator, UUIDMarker, UUID};

use super::abstract_type::AbstractType;
use super::abstract_type::DomainType;

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

pub struct FailedUnification<MyType> {
    pub found: MyType,
    pub expected: MyType,
    pub span: Span,
    pub context: &'static str
}

pub struct TypeSubstitutor<MyType : HindleyMilner<VariableIDMarker>, VariableIDMarker : UUIDMarker> {
    substitution_map: BlockVec<OnceCell<MyType>, 512>, // Pretty big block size so we usually just need one
    failed_unifications: RefCell<Vec<FailedUnification<MyType>>>,
    _ph: PhantomData<VariableIDMarker>
}

impl<MyType : HindleyMilner<VariableIDMarker>, VariableIDMarker : UUIDMarker> Index<UUID<VariableIDMarker>> for TypeSubstitutor<MyType, VariableIDMarker> {
    type Output = OnceCell<MyType>;

    fn index(&self, index: UUID<VariableIDMarker>) -> &Self::Output {
        &self.substitution_map[index.get_hidden_value()]
    }
}

impl<MyType : HindleyMilner<VariableIDMarker>+Clone, VariableIDMarker : UUIDMarker> TypeSubstitutor<MyType, VariableIDMarker> {
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
                    HindleyMilnerInfo::TypeVar(_) => self.unify(b, a),
                    HindleyMilnerInfo::MatchesAny => true
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
            HindleyMilnerInfo::MatchesAny => true
        }
    }
    pub fn unify_must_succeed(&self, a: &MyType, b: &MyType) {
        assert!(self.unify(a, b), "This unification cannot fail. Usually because we're unifying with a Written Type");
    }
    pub fn unify_report_error(&self, found: &MyType, expected: &MyType, span: Span, context: &'static str) {
        if !self.unify(found, expected) {
            self.failed_unifications.borrow_mut().push(FailedUnification {
                found: found.clone(),
                expected: expected.clone(),
                span,
                context
            });
        }
    }
    pub fn extract_errors(&mut self) -> Vec<FailedUnification<MyType>> {
        self.failed_unifications.replace(Vec::new())
    }
}

impl<MyType: HindleyMilner<VariableIDMarker>, VariableIDMarker: UUIDMarker> Drop for TypeSubstitutor<MyType, VariableIDMarker> {
    fn drop(&mut self) {
        assert!(self.failed_unifications.borrow().is_empty(), "Errors were not extracted before dropping!");
    }
}

#[derive(Debug, Clone, Copy)]
pub enum HindleyMilnerInfo<TypeFuncIdent, VariableIDMarker : UUIDMarker> {
    /// Just a marker. Use [HindleyMilner::unify_all_args]
    TypeFunc(TypeFuncIdent),
    TypeVar(UUID<VariableIDMarker>),
    /// Used for errors. Just returning Ok(()) prevents type errors from propagating
    MatchesAny
}

pub trait HindleyMilner<VariableIDMarker: UUIDMarker> : Sized {
    type TypeFuncIdent : Eq;

    fn get_hm_info(&self) -> HindleyMilnerInfo<Self::TypeFuncIdent, VariableIDMarker>;

    /// Iterate through all arguments and unify them
    /// 
    /// If any pair couldn't be unified, return false
    /// 
    /// This is never called by the user, only by [unify]
    fn unify_all_args<F : FnMut(&Self, &Self) -> bool>(left : &Self, right : &Self, unify : &mut F) -> bool;

    /// Has to be implemented per 
    fn fully_substitute(&mut self, substitutor: &TypeSubstitutor<Self, VariableIDMarker>);
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AbstractTypeHMInfo {
    Template(TemplateID),
    Named(TypeUUID),
    Array
}

impl HindleyMilner<TypeVariableIDMarker> for AbstractType {
    type TypeFuncIdent = AbstractTypeHMInfo;

    fn get_hm_info(&self) -> HindleyMilnerInfo<AbstractTypeHMInfo, TypeVariableIDMarker> {
        match self {
            AbstractType::Error => HindleyMilnerInfo::MatchesAny,
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

    fn fully_substitute(&mut self, substitutor: &TypeSubstitutor<Self, TypeVariableIDMarker>) {
        match self {
            AbstractType::Error => {}
            AbstractType::Template(_) => {} // Template Name is included in get_hm_info
            AbstractType::Named(_) => {} // Name is included in get_hm_info
            AbstractType::Array(arr_typ) => {
                arr_typ.fully_substitute(substitutor);
            },
            AbstractType::Unknown(var) => {
                *self = substitutor.substitution_map[var.get_hidden_value()].get().expect("No unknown type variables can be left").clone();
                self.fully_substitute(substitutor);
            }
        }
    }
}

impl HindleyMilner<DomainVariableIDMarker> for DomainType {
    type TypeFuncIdent = DomainID;

    fn get_hm_info(&self) -> HindleyMilnerInfo<DomainID, DomainVariableIDMarker> {
        match self {
            DomainType::Generative => unreachable!("No explicit comparisons with Generative Possible. These should be filtered out first"),
            DomainType::Physical(domain_id) => HindleyMilnerInfo::TypeFunc(*domain_id),
            DomainType::DomainVariable(var) => HindleyMilnerInfo::TypeVar(*var)
        }
    }

    fn unify_all_args<F : FnMut(&Self, &Self) -> bool>(_left : &Self, _right : &Self, _unify : &mut F) -> bool {
        // No sub-args
        true
    }

    fn fully_substitute(&mut self, substitutor: &TypeSubstitutor<Self, DomainVariableIDMarker>) {
        match self {
            DomainType::Generative | DomainType::Physical(_) => {} // Do nothing, These are done already
            DomainType::DomainVariable(var) => {
                *self = substitutor.substitution_map[var.get_hidden_value()].get().unwrap().clone();
                self.fully_substitute(substitutor);
            }
        }
    }
}
