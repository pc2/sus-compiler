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

    fn unify_all_args<F : FnMut(&Self, &Self) -> Result<(), ()>>(left : &Self, right : &Self, unify : &mut F) -> Result<(), ()>;

    fn fully_substitute(&self, substitution_map: &FlatAlloc<OnceCell<Self>, TypeVariableID>) -> Self;/* {
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
fn unify<TypeFuncID : Eq, MyType : HindleyMilner<TypeFuncID>+Clone+std::fmt::Debug>(substitution_map: &FlatAlloc<OnceCell<MyType>, TypeVariableIDMarker>, a: &MyType, b: &MyType) -> Result<(), ()> {
    match a.get_hm_info() {
        HindleyMilnerInfo::TypeFunc(tf_a) => {
            match b.get_hm_info() {
                HindleyMilnerInfo::TypeFunc(tf_b) => {
                    if tf_a != tf_b {
                        return Err(());
                    }
                    MyType::unify_all_args(a, b, &mut |arg_a, arg_b| {
                        unify(substitution_map, arg_a, arg_b)
                    })
                }
                HindleyMilnerInfo::TypeVar(_) => unify(substitution_map, b, a),
                HindleyMilnerInfo::MatchesAny => Ok(())
            }
        }
        HindleyMilnerInfo::TypeVar(var) => {
            let typ_cell = &substitution_map[var];
            if let Some(found) = typ_cell.get() {
                unify(substitution_map, found, b)
            } else {
                typ_cell.set(b.clone()).unwrap();
                Ok(())
            }
        }
        HindleyMilnerInfo::MatchesAny => Ok(())
    }
}

/*
use super::abstract_type::AbstractType;

impl<'falloc, TypeFuncID : Eq> HindleyMilner<TypeFuncID> for AbstractType {
    fn get_hm_info(&self) -> HindleyMilnerInfo<TypeFuncID> {
        match self {
            AbstractType::Error => HindleyMilnerInfo::MatchesAny,
            AbstractType::Unknown => todo!(),
            AbstractType::Template(_) => todo!(),
            AbstractType::Named(_) => todo!(),
            AbstractType::Array(_) => todo!(),
        }
    }

    fn unify_all_args<F : FnMut(&Self, &Self) -> Result<(), ()>>(left : &Self, right : &Self, unify : &mut F) -> Result<(), ()> {
        match (left, right) {
            (AbstractType::Named(na), AbstractType::Named(nb)) => Ok(()), // Already covered by get_hm_info
            (AbstractType::Array(arr_typ), AbstractType::Array(arr_typ)) => unify(substitution_map, a, b),
            (_, _) => unreachable!("All others should have been eliminated by get_hm_info check")
        }
    }

    fn fully_substitute(&self, substitution_map: &mut FlatAlloc<Option<&Self>, TypeVariableID>) -> Self {
        match self {
            HindleyMilnerInfo::TypeFunc(tf, tf_args) => HindleyMilnerInfo::TypeFunc(*tf, tf_args.map(|(_, arg)| self.fully_substitute(substitution_map, arg))),
            HindleyMilnerInfo::TypeVar(v) => {
                let substituted = substitution_map[*v].expect("This variable wasn't properly substituted");
                self.fully_substitute(substitution_map, substituted)
            }
        }
    }
}
*/