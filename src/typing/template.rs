use super::{abstract_type::AbstractType, concrete_type::ConcreteType};
use crate::flattening::WrittenType;
use crate::{prelude::*, value::Value};
use std::fmt::Display;

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
    pub template_args: TemplateArgs,
    pub template_arg_types: TemplateAbstractTypes,
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

/// The template parameters of an object ([crate::flattening::Module], [crate::flattening::StructType], or [crate::flattening::NamedConstant])
///
/// See [crate::linker::LinkInfo]
///
/// Not to be confused with [TemplateArg], which is the argument passed to this parameter.
#[derive(Debug)]
pub struct Parameter {
    pub name: String,
    pub name_span: Span,
    pub kind: ParameterKind,
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

/// See [Parameter]
///
/// Must match the [TemplateArgKind] that is passed
#[derive(Debug)]
pub enum ParameterKind {
    Type(TypeParameterKind),
    Generative(GenerativeParameterKind),
}

impl ParameterKind {
    #[track_caller]
    pub fn unwrap_type(&self) -> &TypeParameterKind {
        let Self::Type(t) = self else {
            unreachable!("ParameterKind::unwrap_type on {self:?}")
        };
        t
    }
    #[track_caller]
    pub fn unwrap_value(&self) -> &GenerativeParameterKind {
        let Self::Generative(v) = self else {
            unreachable!("ParameterKind::unwrap_value on {self:?}")
        };
        v
    }
}

/// An argument passed to a template parameter.
///
/// See [GlobalReference]
///
/// Not to be confused with [Parameter], which it is passed into.
///
/// When instantiated, this becomes a [ConcreteTemplateArg]
#[derive(Debug)]
pub struct TemplateArg {
    pub name_span: Span,
    pub value_span: Span,
    pub kind: TemplateArgKind,
}

/// See [TemplateArg]
///
/// The argument kind passed to [ParameterKind], which it must match
#[derive(Debug)]
pub enum TemplateArgKind {
    Type(WrittenType),
    Value(FlatID),
}

impl TemplateArgKind {
    #[track_caller]
    pub fn unwrap_type(&self) -> &WrittenType {
        let Self::Type(t) = self else {
            unreachable!("TemplateArgKind::unwrap_type on {self:?}")
        };
        t
    }
    #[track_caller]
    pub fn unwrap_value(&self) -> FlatID {
        let Self::Value(v) = self else {
            unreachable!("TemplateArgKind::unwrap_value on {self:?}")
        };
        *v
    }
}

/// Tracks how we arrived at this value through type inference. (See [crate::typing::type_inference])
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum HowDoWeKnowTheTemplateArg {
    Given,
    Inferred,
}

impl Display for HowDoWeKnowTheTemplateArg {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            HowDoWeKnowTheTemplateArg::Given => "given",
            HowDoWeKnowTheTemplateArg::Inferred => "inferred",
        })
    }
}

// --------------------------------------------------
// IMPORTANT: To those seeking to refactor this struct: don't.
// It may look like [ConcreteTemplateArg::Value] duplicates [ConcreteType::Value], or that the whole thing could be replaced by [ConcreteType] itself.
// But the [ConcreteTemplateArg::Type] and [ConcreteTemplateArg::Value] cases differentiate often enough, and semantically are more similar to [ParameterKind].
// Attempting to merge these uses with [ConcreteType] internals only leads to confusion.
// --------------------------------------------------

/// Represents the value we're passing into a template argument.
///
/// It is the instantiated variant of [TemplateArg]
///
/// And it is passed to a [crate::flattening::Module], [crate::flattening::StructType], or [crate::flattening::NamedConstant]'s [Parameter]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ConcreteTemplateArg {
    Type(ConcreteType, HowDoWeKnowTheTemplateArg),
    Value(Value, HowDoWeKnowTheTemplateArg),
    /// It has not been explicitly provided,
    /// yet [crate::typing::type_inference] may replace this value when it can figure it out from the context
    NotProvided,
}

impl ConcreteTemplateArg {
    #[track_caller]
    pub fn unwrap_type(&self) -> &ConcreteType {
        let Self::Type(t, _) = self else {
            unreachable!()
        };
        t
    }
    #[track_caller]
    pub fn unwrap_value(&self) -> &Value {
        let Self::Value(v, _) = self else {
            unreachable!()
        };
        v
    }
}

/// See [TemplateArg]
pub type TemplateArgs = FlatAlloc<Option<TemplateArg>, TemplateIDMarker>;
/// Applies to both Template Type args and Template Value args.
///
/// For Types this is the Type, for Values this is unified with the parameter declaration type
pub type TemplateAbstractTypes = FlatAlloc<AbstractType, TemplateIDMarker>;
/// See [Parameter]
pub type Parameters = FlatAlloc<Parameter, TemplateIDMarker>;
/// See [ConcreteTemplateArg]
pub type ConcreteTemplateArgs = FlatAlloc<ConcreteTemplateArg, TemplateIDMarker>;
