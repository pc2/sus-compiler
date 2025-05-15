use std::ops::Deref;

use crate::prelude::*;

use super::{
    abstract_type::{AbstractInnerType, AbstractRankedType},
    template::{
        for_each_generative_input_in_template_args, AbstractTemplateArg, GlobalReference, TVec,
    },
    type_inference::{AbstractTypeSubstitutor, Substitutor},
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
    pub fn for_each_generative_input(&self, f: &mut impl FnMut(FlatID)) {
        match self {
            WrittenType::Error(_) | WrittenType::TemplateVariable(_, _) => {}
            WrittenType::Named(name) => {
                for_each_generative_input_in_template_args(&name.template_args, f)
            }
            WrittenType::Array(_span, arr_box) => {
                use std::ops::Deref;
                f(arr_box.deref().1)
            }
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
        template_args: &TVec<AbstractTemplateArg>,
    ) -> AbstractRankedType {
        match wr_typ {
            WrittenType::Error(_span) => self.alloc_unknown(),
            WrittenType::TemplateVariable(_span, argument_id) => template_args[*argument_id]
                .unwrap_type()
                .get_abstract_typ()
                .clone(),
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
}
