use crate::prelude::*;
use ibig::IBig;

use super::{concrete_type::ConcreteTemplateArg, value_unifier::UnifyableValue};
use crate::{typing::set_unifier::Unifyable, value::Value};

/// See [TVec]. All circumstances handling Templates need to handle both Types and Values.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum TemplateKind<T, V> {
    Type(T),
    Value(V),
}

impl<T: std::fmt::Debug, V: std::fmt::Debug> TemplateKind<T, V> {
    #[track_caller]
    pub fn unwrap_type(&self) -> &T {
        let_unwrap!(Self::Type(t), self);
        t
    }
    #[track_caller]
    pub fn unwrap_value(&self) -> &V {
        let_unwrap!(Self::Value(t), self);
        t
    }
    #[track_caller]
    pub fn unwrap_type_mut(&mut self) -> &mut T {
        let_unwrap!(Self::Type(t), self);
        t
    }
    #[track_caller]
    pub fn unwrap_value_mut(&mut self) -> &mut V {
        let_unwrap!(Self::Value(t), self);
        t
    }
}
impl<T, V> TemplateKind<T, V> {
    pub fn as_ref(&self) -> TemplateKind<&T, &V> {
        match self {
            TemplateKind::Type(t) => TemplateKind::Type(t),
            TemplateKind::Value(v) => TemplateKind::Value(v),
        }
    }
    pub fn as_mut(&mut self) -> TemplateKind<&mut T, &mut V> {
        match self {
            TemplateKind::Type(t) => TemplateKind::Type(t),
            TemplateKind::Value(v) => TemplateKind::Value(v),
        }
    }
    pub fn and<T2, V2>(self, other: TemplateKind<T2, V2>) -> TemplateKind<(T, T2), (V, V2)> {
        match (self, other) {
            (TemplateKind::Type(t1), TemplateKind::Type(t2)) => TemplateKind::Type((t1, t2)),
            (TemplateKind::Value(v1), TemplateKind::Value(v2)) => TemplateKind::Value((v1, v2)),
            (TemplateKind::Type(_), TemplateKind::Value(_))
            | (TemplateKind::Value(_), TemplateKind::Type(_)) => {
                unreachable!("Cannot combine Type and Value template args!")
            }
        }
    }
    pub fn and_by_ref<'s, T2, V2>(
        &'s self,
        other: &'s TemplateKind<T2, V2>,
    ) -> TemplateKind<(&'s T, &'s T2), (&'s V, &'s V2)> {
        match (self, other) {
            (TemplateKind::Type(t1), TemplateKind::Type(t2)) => TemplateKind::Type((t1, t2)),
            (TemplateKind::Value(v1), TemplateKind::Value(v2)) => TemplateKind::Value((v1, v2)),
            (TemplateKind::Type(_), TemplateKind::Value(_))
            | (TemplateKind::Value(_), TemplateKind::Type(_)) => {
                unreachable!("Cannot combine Type and Value template args!")
            }
        }
    }
    pub fn and_by_ref_mut<'s, T2, V2>(
        &'s mut self,
        other: &'s TemplateKind<T2, V2>,
    ) -> TemplateKind<(&'s mut T, &'s T2), (&'s mut V, &'s V2)> {
        match (self, other) {
            (TemplateKind::Type(t1), TemplateKind::Type(t2)) => TemplateKind::Type((t1, t2)),
            (TemplateKind::Value(v1), TemplateKind::Value(v2)) => TemplateKind::Value((v1, v2)),
            (TemplateKind::Type(_), TemplateKind::Value(_))
            | (TemplateKind::Value(_), TemplateKind::Type(_)) => {
                unreachable!("Cannot combine Type and Value template args!")
            }
        }
    }
}

impl<T> TemplateKind<T, T> {
    pub fn unwrap_identical(self) -> T {
        match self {
            TemplateKind::Type(t) | TemplateKind::Value(t) => t,
        }
    }
}

/// The template parameters of an object ([crate::flattening::Module], [crate::flattening::StructType], or [crate::flattening::NamedConstant])
///
/// See [crate::linker::LinkInfo]
///
/// Not to be confused with [TemplateArg], which is the argument passed to this parameter.
#[derive(Debug)]
pub struct Parameter {
    pub name: String,
    pub name_span: Span,
    pub kind: TemplateKind<TypeParameterKind, GenerativeParameterKind>,
}

/// See [Parameter]
#[derive(Debug)]
pub struct GenerativeParameterKind {
    pub decl_span: Span,
    /// Set at the end of Flattening
    pub declaration_instruction: FlatID,
}

/// See [Parameter]
#[derive(Debug)]
pub struct TypeParameterKind {}

/// A convienent type alias for all places where lists of template args are needed
pub type TVec<T> = FlatAlloc<T, TemplateIDMarker>;

impl TVec<ConcreteTemplateArg> {
    pub fn cast_to_unifyable_array<const N: usize>(&self) -> [&UnifyableValue; N] {
        self.cast_to_array().each_ref().map(|v| v.unwrap_value())
    }
    pub fn cast_to_unifyable_array_mut<const N: usize>(&mut self) -> [&mut UnifyableValue; N] {
        self.cast_to_array_mut()
            .each_mut()
            .map(|v| v.unwrap_value_mut())
    }
    pub fn cast_to_int_array<const N: usize>(&self) -> [&IBig; N] {
        self.cast_to_array().each_ref().map(|v| {
            let_unwrap!(TemplateKind::Value(Unifyable::Set(Value::Integer(i))), v);
            i
        })
    }
    pub fn cast_to_int_array_mut<const N: usize>(&mut self) -> [&mut IBig; N] {
        self.cast_to_array_mut().each_mut().map(|v| {
            let_unwrap!(TemplateKind::Value(Unifyable::Set(Value::Integer(i))), v);
            i
        })
    }
}
