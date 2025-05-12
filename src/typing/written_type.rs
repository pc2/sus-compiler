use std::ops::Deref;

use crate::{
    flattening::{DeclarationKind, ExpressionSource, WireReferenceRoot},
    linker::LinkInfo,
    prelude::*,
};

use super::{
    abstract_type::{AbstractInnerType, AbstractRankedType},
    concrete_type::{ConcreteGlobalReference, ConcreteType},
    template::{
        for_each_generative_input_in_template_args, GlobalReference, TVec, TemplateArgKind,
    },
    type_inference::{AbstractTypeSubstitutor, Substitutor, TypeSubstitutor},
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
        template_type_args: &TVec<AbstractRankedType>,
    ) -> AbstractRankedType {
        match wr_typ {
            WrittenType::Error(_span) => self.alloc_unknown(),
            WrittenType::TemplateVariable(_span, argument_id) => {
                template_type_args[*argument_id].clone()
            }
            WrittenType::Named(global_reference) => {
                AbstractInnerType::Named(global_reference.id).scalar()
            }
            WrittenType::Array(_span, array_content_and_size) => {
                let (arr_content_type, _size_flat, _array_bracket_span) =
                    array_content_and_size.deref();

                let content_typ = self.written_to_abstract_type_substitute_templates(
                    arr_content_type,
                    template_type_args,
                );

                content_typ.rank_up()
            }
        }
    }
}

impl TypeSubstitutor<ConcreteType> {
    pub fn concretize_written_type_with_possible_template_args(
        &mut self,
        written_typ: &WrittenType,
        template_args: &TVec<ConcreteType>,
        link_info: &LinkInfo,
    ) -> ConcreteType {
        match written_typ {
            WrittenType::Error(_span) => self.alloc_unknown(),
            WrittenType::TemplateVariable(_span, uuid) => template_args[*uuid].clone(),
            WrittenType::Named(global_reference) => {
                let object_template_args: TVec<ConcreteType> =
                    global_reference
                        .template_args
                        .map(|(_arg_id, arg)| -> ConcreteType {
                            if let Some(arg) = arg {
                                match &arg.kind {
                                    TemplateArgKind::Type(arg_wr_typ) => self
                                        .concretize_written_type_with_possible_template_args(
                                            arg_wr_typ,
                                            template_args,
                                            link_info,
                                        ),
                                    TemplateArgKind::Value(uuid) => {
                                        if let Some(found_template_arg) =
                                            can_expression_be_value_inferred(link_info, *uuid)
                                        {
                                            template_args[found_template_arg].clone()
                                        } else {
                                            self.alloc_unknown()
                                        }
                                    }
                                }
                            } else {
                                self.alloc_unknown()
                            }
                        });

                ConcreteType::Named(ConcreteGlobalReference {
                    id: global_reference.id,
                    template_args: object_template_args,
                })
            }
            WrittenType::Array(_span, arr_box) => {
                let (arr_content_wr, arr_idx_id, _arr_brackets) = arr_box.deref();

                let arr_content_concrete = self
                    .concretize_written_type_with_possible_template_args(
                        arr_content_wr,
                        template_args,
                        link_info,
                    );
                let arr_idx_concrete = if let Some(found_template_arg) =
                    can_expression_be_value_inferred(link_info, *arr_idx_id)
                {
                    template_args[found_template_arg].clone()
                } else {
                    self.alloc_unknown()
                };

                ConcreteType::Array(Box::new((arr_content_concrete, arr_idx_concrete)))
            }
        }
    }
}

/// Part of Template Value Inference.
///
/// Specifically, for code like this:
///
/// ```sus
/// module add_all #(int Size) {
///     input int[Size] arr // We're targeting the 'Size' within the array size
///     output int total
/// }
/// ```
fn can_expression_be_value_inferred(link_info: &LinkInfo, expr_id: FlatID) -> Option<TemplateID> {
    let expr = link_info.instructions[expr_id].unwrap_expression();
    let ExpressionSource::WireRef(wr) = &expr.source else {
        return None;
    };
    if !wr.path.is_empty() {
        return None;
    } // Must be a plain, no fuss reference to a de
    let WireReferenceRoot::LocalDecl(wire_declaration, _span) = &wr.root else {
        return None;
    };
    let template_arg_decl = link_info.instructions[*wire_declaration].unwrap_declaration();
    let DeclarationKind::GenerativeInput(template_id) = &template_arg_decl.decl_kind else {
        return None;
    };
    Some(*template_id)
}
