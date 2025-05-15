use ibig::IBig;

use super::{
    abstract_type::AbstractRankedType, concrete_type::ConcreteTemplateArg,
    written_type::WrittenType,
};
use crate::{let_unwrap, prelude::*};

/// See [TVec]. All circumstances handling Templates need to handle both Types and Values.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
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
}

impl<T> TemplateKind<T, T> {
    pub fn unwrap_identical(self) -> T {
        match self {
            TemplateKind::Type(t) | TemplateKind::Value(t) => t,
        }
    }
}

/// References any [crate::flattening::Module], [crate::flattening::StructType], or [crate::flattening::NamedConstant],
/// and includes any template arguments.
///
/// As an example, this is the struct in charge of representing:
/// ```sus
/// FIFO #(DEPTH : 32, T : type int)
/// ```
#[derive(Debug)]
pub struct GlobalReference<ID> {
    pub name_span: Span,
    pub id: ID,
    pub template_args: TVec<AbstractTemplateArg>,
    pub template_span: Option<BracketSpan>,
}

impl<ID> GlobalReference<ID> {
    pub fn get_total_span(&self) -> Span {
        let mut result = self.name_span;
        if let Some(template_span) = self.template_span {
            result = Span::new_overarching(result, template_span.outer_span());
        }
        result
    }
}

pub type AbstractTemplateArg = TemplateKind<TemplateArg<WrittenType>, TemplateArg<FlatID>>;

impl AbstractTemplateArg {
    pub fn map_is_provided(&self) -> Option<(Span, Span, TemplateKind<&WrittenType, &FlatID>)> {
        match self {
            TemplateKind::Type(TemplateArg::Provided {
                name_span,
                value_span,
                arg,
                ..
            }) => Some((*name_span, *value_span, TemplateKind::Type(arg))),
            TemplateKind::Value(TemplateArg::Provided {
                name_span,
                value_span,
                arg,
                ..
            }) => Some((*name_span, *value_span, TemplateKind::Value(arg))),
            TemplateKind::Type(TemplateArg::NotProvided { .. }) => None,
            TemplateKind::Value(TemplateArg::NotProvided { .. }) => None,
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

/// An argument passed to a template parameter.
///
/// See [GlobalReference]
///
/// Not to be confused with [Parameter], which it is passed into.
///
/// When instantiated, this becomes a [ConcreteTemplateArg]
#[derive(Debug)]
pub enum TemplateArg<T> {
    Provided {
        name_span: Span,
        value_span: Span,
        arg: T,
        abs_typ: AbstractRankedType,
    },
    NotProvided {
        abs_typ: AbstractRankedType,
    },
}

impl<T> TemplateArg<T> {
    pub fn get_abstract_typ(&self) -> &AbstractRankedType {
        match self {
            TemplateArg::Provided { abs_typ, .. } | TemplateArg::NotProvided { abs_typ } => abs_typ,
        }
    }
    pub fn get_abstract_typ_mut(&mut self) -> &mut AbstractRankedType {
        match self {
            TemplateArg::Provided { abs_typ, .. } | TemplateArg::NotProvided { abs_typ } => abs_typ,
        }
    }
}

/// A convienent type alias for all places where lists of template args are needed
pub type TVec<T> = FlatAlloc<T, TemplateIDMarker>;

pub fn for_each_generative_input_in_template_args(
    template_args: &TVec<AbstractTemplateArg>,
    f: &mut impl FnMut(FlatID),
) {
    for (_id, t_arg) in template_args {
        match t_arg {
            TemplateKind::Type(TemplateArg::Provided { arg: wr_typ, .. }) => {
                wr_typ.for_each_generative_input(f)
            }
            TemplateKind::Value(TemplateArg::Provided { arg: val, .. }) => f(*val),
            TemplateKind::Type(TemplateArg::NotProvided { .. })
            | TemplateKind::Value(TemplateArg::NotProvided { .. }) => {}
        }
    }
}

impl TVec<ConcreteTemplateArg> {
    pub fn cast_to_int_array<const N: usize>(&self) -> [&IBig; N] {
        self.map_to_array(|_, v| v.unwrap_value().unwrap_value().unwrap_integer())
    }
}
