//! Implements the Hindley-Milner algorithm for Type Inference. 

use std::cell::OnceCell;

use itertools::Itertools;

use crate::prelude::*;

use crate::alloc::{UUIDMarker, UUID};

pub struct TypeVariableIDMarker;
impl UUIDMarker for TypeVariableIDMarker {
    const DISPLAY_NAME: &'static str = "type_variable_";
}
pub type TypeVariableID = UUID<TypeVariableIDMarker>;

#[derive(Debug, Clone, Copy)]
enum HindleyMilnerInfo<TypeFuncIdent> {
    /// Just a marker. Use [HindleyMilner::type_func_for_each_matching]
    TypeFunc(TypeFuncIdent),
    TypeVar(TypeVariableID),
    /// Used for errors. Just returning Ok(()) prevents type errors from propagating
    MatchesAny
}

pub trait HindleyMilner<TypeFuncIdent : Eq> : Sized {
    fn get_hm_info(&self) -> HindleyMilnerInfo<TypeFuncIdent>;

    /// Iterate through all arguments and unify them
    /// 
    /// If any pair couldn't be unified, return false
    fn unify_all_args<F : FnMut(&Self, &Self) -> bool>(left : &Self, right : &Self, unify : &mut F) -> bool;

    fn fully_substitute(&mut self, substitution_map: &FlatAlloc<OnceCell<Self>, TypeVariableIDMarker>);/* {
        match a.as_hm_info() {
            HindleyMilnerInfo::TypeFunc(tf, tf_args) => HindleyMilnerInfo::TypeFunc(*tf, tf_args.map(|(_, arg)| fully_substitute(substitution_map, arg))),
            HindleyMilnerInfo::TypeVar(v) => {
                let substituted = substitution_map[v].expect("This variable wasn't properly substituted");
                fully_substitute(substitution_map, substituted)
            }
        }
    }*/
}

/// Returns false if the types couldn't be unified
fn unify<TypeFuncID : Eq, MyType : HindleyMilner<TypeFuncID>+Clone+std::fmt::Debug>(substitution_map: &FlatAlloc<OnceCell<MyType>, TypeVariableIDMarker>, a: &MyType, b: &MyType) -> bool {
    match a.get_hm_info() {
        HindleyMilnerInfo::TypeFunc(tf_a) => {
            match b.get_hm_info() {
                HindleyMilnerInfo::TypeFunc(tf_b) => {
                    if tf_a != tf_b {
                        return false;
                    }
                    MyType::unify_all_args(a, b, &mut |arg_a, arg_b| {
                        unify(substitution_map, arg_a, arg_b)
                    })
                }
                HindleyMilnerInfo::TypeVar(_) => unify(substitution_map, b, a),
                HindleyMilnerInfo::MatchesAny => true
            }
        }
        HindleyMilnerInfo::TypeVar(var) => {
            let typ_cell = &substitution_map[var];
            if let Some(found) = typ_cell.get() {
                unify(substitution_map, found, b)
            } else {
                typ_cell.set(b.clone()).unwrap();
                true
            }
        }
        HindleyMilnerInfo::MatchesAny => true
    }
}


use super::abstract_type::AbstractType;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum AbstractTypeHMInfo {
    Template(TemplateID),
    Named(TypeUUID),
    Array
}

impl HindleyMilner<AbstractTypeHMInfo> for AbstractType {
    fn get_hm_info(&self) -> HindleyMilnerInfo<AbstractTypeHMInfo> {
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

    fn fully_substitute(&mut self, substitution_map: &FlatAlloc<OnceCell<Self>, TypeVariableIDMarker>) {
        match self {
            AbstractType::Error => {}
            AbstractType::Template(_) => {}
            AbstractType::Named(_) => {}
            AbstractType::Array(arr_typ) => {
                arr_typ.fully_substitute(substitution_map);
            },
            AbstractType::Unknown(var) => {
                *self = substitution_map[*var].get().expect("No unknown type variables can be left").clone();
                self.fully_substitute(substitution_map);
            }
        }
    }
}
