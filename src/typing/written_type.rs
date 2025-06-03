use std::ops::Deref;

use crate::prelude::*;

use super::{
    abstract_type::{AbstractInnerType, AbstractRankedType},
    template::{
        get_type_arg_for, GlobalReference, Parameter, TVec, TemplateKind, WrittenTemplateArg,
    },
    type_inference::AbstractTypeSubstitutor,
};

/// The textual representation of a type expression in the source code.
///
/// Not to be confused with [crate::typing::abstract_type::AbstractType] which is for working with types in the flattening stage,
/// or [crate::typing::concrete_type::ConcreteType], which is for working with types post instantiation.
#[derive(Debug)]
pub enum WrittenType {
    Error(Span),
    TemplateVariable(Span, TemplateID),
    Named(GlobalReference<TypeUUID>),
    Array(Span, Box<(WrittenType, FlatID, BracketSpan)>),
}

impl WrittenType {
    pub fn get_span(&self) -> Span {
        match self {
            WrittenType::Error(total_span)
            | WrittenType::TemplateVariable(total_span, ..)
            | WrittenType::Array(total_span, _) => *total_span,
            WrittenType::Named(global_ref) => global_ref.get_total_span(),
        }
    }
}

impl AbstractTypeSubstitutor {
    /// This should always be what happens first to a given variable.
    ///
    /// Therefore it should be impossible that one of the internal unifications ever fails
    pub fn written_to_abstract_type(&mut self, wr_typ: &WrittenType) -> AbstractRankedType {
        match wr_typ {
            WrittenType::Error(_span) => self.alloc_unknown(),
            WrittenType::TemplateVariable(_span, argument_id) => {
                AbstractInnerType::Template(*argument_id).scalar()
            }

            WrittenType::Named(global_reference) => {
                AbstractInnerType::Named(global_reference.id).scalar()
            }
            WrittenType::Array(_span, array_content_and_size) => {
                let (arr_content_type, _size_flat, _array_bracket_span) =
                    array_content_and_size.deref();

                let content_typ = self.written_to_abstract_type(arr_content_type);

                content_typ.rank_up()
            }
        }
    }

    /// This should always be what happens first to a given variable.
    ///
    /// Therefore it should be impossible that one of the internal unifications ever fails
    ///
    /// template_type_args applies to both Template Type args and Template Value args.
    ///
    /// For Types this is the Type, for Values this is unified with the parameter declaration type
    pub fn written_to_abstract_type_substitute_templates(
        &mut self,
        wr_typ: &WrittenType,
        template_args: &TVec<AbstractRankedType>,
    ) -> AbstractRankedType {
        match wr_typ {
            WrittenType::Error(_span) => self.alloc_unknown(),
            WrittenType::TemplateVariable(_span, argument_id) => {
                template_args[*argument_id].clone()
            }
            WrittenType::Named(global_reference) => {
                AbstractInnerType::Named(global_reference.id).scalar()
            }
            WrittenType::Array(_span, array_content_and_size) => {
                let (arr_content_type, _size_flat, _array_bracket_span) =
                    array_content_and_size.deref();

                let content_typ = self
                    .written_to_abstract_type_substitute_templates(arr_content_type, template_args);

                content_typ.rank_up()
            }
        }
    }

    pub fn written_template_args_to_abstract(
        &mut self,
        params: &TVec<Parameter>,
        written_args: &[WrittenTemplateArg],
    ) -> TVec<AbstractRankedType> {
        params.map(|(id, param)| match &param.kind {
            TemplateKind::Type(_) => {
                if let Some(wr_typ) = get_type_arg_for(written_args, id) {
                    self.written_to_abstract_type(wr_typ)
                } else {
                    self.alloc_unknown()
                }
            }
            TemplateKind::Value(_) => self.alloc_unknown(),
        })
    }
}
