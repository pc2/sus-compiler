//! This file defines a OnceCell variant for use with typechecking
//!
//! Because in typechecking, we will always set it to uninitialized in Flatten, set it to an initial value (&self) in typechecking, and then finalize the type in (&mut self)

use std::{cell::OnceCell, ops::Deref};

#[derive(Debug)]
/// Basically equivalent to [std::cell::OnceCell], but implements [std::ops::Deref] and automatically unwraps
pub struct TyCell<T>(OnceCell<T>);

impl<T: std::fmt::Debug> TyCell<T> {
    pub fn new() -> Self {
        Self::default()
    }
    #[track_caller]
    pub fn set(&self, v: T) {
        self.0.set(v).unwrap();
    }
    #[track_caller]
    pub fn get_mut(&mut self) -> &mut T {
        self.0.get_mut().unwrap()
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
        self.0.get().unwrap()
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
