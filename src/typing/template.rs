use super::{abstract_type::AbstractRankedType, written_type::WrittenType};
use crate::{let_unwrap, prelude::*};

/// See [TVec]. All circumstances handling Templates need to handle both Types and Values.
#[derive(Debug)]
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
    pub template_args: TVec<Option<TemplateArg>>,
    pub template_arg_types: TVec<AbstractRankedType>,
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
pub struct TemplateArg {
    pub name_span: Span,
    pub value_span: Span,
    pub kind: TemplateKind<WrittenType, FlatID>,
}

/// A convienent type alias for all places where lists of template args are needed
pub type TVec<T> = FlatAlloc<T, TemplateIDMarker>;

pub fn for_each_generative_input_in_template_args(
    template_args: &TVec<Option<TemplateArg>>,
    f: &mut impl FnMut(FlatID),
) {
    for (_id, t_arg) in template_args.iter_valids() {
        match &t_arg.kind {
            TemplateKind::Type(typ) => typ.for_each_generative_input(f),
            TemplateKind::Value(val) => f(*val),
        }
    }
}
