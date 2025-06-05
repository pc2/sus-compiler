//! This file defines a OnceCell variant for use with typechecking
//!
//! Because in typechecking, we will always set it to uninitialized in Flatten, set it to an initial value (&self) in typechecking, and then finalize the type in (&mut self)

use std::{cell::OnceCell, ops::Deref};

#[derive(Debug)]
pub struct TyCell<T>(OnceCell<T>);

impl<T: std::fmt::Debug> TyCell<T> {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn set(&self, v: T) {
        self.0.set(v).unwrap();
    }
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

    fn deref(&self) -> &Self::Target {
        self.0.get().unwrap()
    }
}
