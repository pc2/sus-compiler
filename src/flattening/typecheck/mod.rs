mod domain_check;
mod lints;
mod type_check;

use crate::{
    flattening::typecheck::{domain_check::domain_check_all, type_check::typecheck_all_modules},
    linker::passes::ImmutableContext,
    typing::type_inference::{AbstractTypeSubstitutor, TypeSubstitutor, TypeUnifier},
};

use super::*;

use std::{cell::OnceCell, ops::Deref};

pub use lints::perform_lints;
pub fn typecheck_all(linker: &mut Linker) {
    typecheck_all_modules(linker);
    domain_check_all(linker);
}

struct DomainCheckingContext<'l> {
    globals: ImmutableContext<'l>,
    errors: &'l ErrorCollector<'l>,
    instructions: &'l FlatAlloc<Instruction, FlatIDMarker>,
    domain_checker: TypeUnifier<TypeSubstitutor<DomainType>>,
}

struct TypeCheckingContext<'l> {
    globals: ImmutableContext<'l>,
    errors: &'l ErrorCollector<'l>,
    instructions: &'l FlatAlloc<Instruction, FlatIDMarker>,
    type_checker: TypeUnifier<AbstractTypeSubstitutor>,
}

/// Basically equivalent to [std::cell::OnceCell], but implements [std::ops::Deref] and automatically unwraps
/// This file defines a OnceCell variant for use with typechecking
///
/// Because in typechecking, we will always set it to uninitialized in Flatten, set it to an initial value (&self) in typechecking, and then finalize the type in (&mut self)
#[derive(Debug)]
pub struct TyCell<T>(OnceCell<T>);

impl<T: std::fmt::Debug> TyCell<T> {
    pub fn new() -> Self {
        Self::default()
    }
    #[track_caller]
    fn get_mut(&mut self) -> &mut T {
        self.0.get_mut().unwrap()
    }
    /// Private because only typechecking should be allowed to set TyCells
    #[track_caller]
    fn set(&self, v: T) {
        self.0.set(v).unwrap();
    }
}

impl<T> Default for TyCell<T> {
    fn default() -> Self {
        Self(Default::default())
    }
}

impl<T> Deref for TyCell<T> {
    type Target = T;

    #[track_caller]
    fn deref(&self) -> &Self::Target {
        self.0.get().expect("Deref on an unfinished TyCell!")
    }
}

/*
// This delegated IntoIterator impl causes infinite recursion due to a bug in rustc. https://github.com/rust-lang/rust/issues/106512
// Right now, just defer to .iter()
impl<'a, T> IntoIterator for &'a TyCell<T>
where
    &'a T: IntoIterator,
{
    type Item = <&'a T as IntoIterator>::Item; // NOTE diff
    type IntoIter = <&'a T as IntoIterator>::IntoIter; // NOTE diff
    fn into_iter(self) -> Self::IntoIter {
        self.0.get().unwrap().into_iter()
    }
}
*/
