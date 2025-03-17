use sus_proc_macro::get_builtin_type_whole;

use crate::alloc::ArenaAllocator;
use crate::prelude::*;
use crate::value::Value;
use std::ops::Deref;

use super::template::{GlobalReference, Parameter, TVec};
use super::type_inference::{
    DomainVariableID, DomainVariableIDMarker, InnerTypeVariableID, InnerTypeVariableIDMarker,
    PeanoVariableID, PeanoVariableIDMarker, TypeSubstitutor, UnifyErrorReport,
};
use crate::flattening::{BinaryOperator, StructType, TypingAllocator, UnaryOperator, WrittenType};
use crate::to_string::map_to_type_names;

/// This contains only the information that can be type-checked before template instantiation.
///
/// Its most important components are the names and structure of types.
///
/// What isn't included are the parameters of types. So Array Sizes for example.
///
/// This is such that useful information can still be given for modules that haven't been instantiated.
///
/// Not to be confused with [WrittenType], which is the in-source text representation.
///
/// Not to be confused with [crate::typing::concrete_type::ConcreteType], which is the
/// post-instantiation type.
///
/// [AbstractType]s don't actually get converted to [crate::typing::concrete_type::ConcreteType]s.
/// Instead [crate::typing::concrete_type::ConcreteType] gets created from [WrittenType] directly.
#[derive(Debug, Clone)]
pub enum AbstractInnerType {
    Template(TemplateID),
    Named(WholeTypeUUID),
    Array(Box<AbstractRankedType>),
    /// Referencing [AbstractType::Unknown] is a strong code smell.
    /// It is likely you should use [TypeSubstitutor::unify_must_succeed] or [TypeSubstitutor::unify_report_error] instead
    ///
    /// It should only occur in creation `AbstractType::Unknown(self.type_substitutor.alloc())`
    Unknown(InnerTypeVariableID),
}

#[derive(Debug, Clone)]
pub struct AbstractRankedType {
    pub inner: AbstractInnerType,
    pub rank: PeanoType,
}

impl AbstractRankedType {
    pub const fn scalar(inner: AbstractInnerType) -> Self {
        Self {
            inner,
            rank: PeanoType::Zero,
        }
    }
    pub fn rank_up(&self) -> Self {
        Self {
            inner: self.inner.clone(),
            rank: PeanoType::Succ(Box::new(self.rank.clone())),
        }
    }
}

#[derive(Debug, Clone)]
pub enum PeanoType {
    Zero,
    Named(WholeTypeUUID),
    Succ(Box<PeanoType>),
    Unknown(PeanoVariableID),
}

pub const BOOL_TYPE: AbstractRankedType =
    AbstractRankedType::scalar(AbstractInnerType::Named(get_builtin_type_whole!("bool")));
pub const INT_TYPE: AbstractRankedType =
    AbstractRankedType::scalar(AbstractInnerType::Named(get_builtin_type_whole!("int")));

/// These represent (clock) domains. While clock domains are included under this umbrella, domains can use the same clock.
/// The use case for non-clock-domains is to separate Latency Counting domains. So different pipelines where it doesn't
/// necessarily make sense that their values are related by a fixed number of clock cycles.
///
/// Domains are resolved pre-instantiation, because dynamic domain merging doesn't seem like a valuable use case.
///
/// As a convenience, we make [DomainType::Generative] a special case for a domain.
///
/// The fun thing is that we can now use this domain info for syntax highlighting, giving wires in different domains a different color.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DomainType {
    /// Generative conflicts with nothing
    Generative,
    /// This object is a real wire. It corresponds to a certain (clock) domain. It can only affect wires in the same domain.
    Physical(DomainID),

    /// These are unified by Hindley-Milner unification
    ///
    /// They always point to non-generative domains.
    ///
    /// Referencing [DomainType::Unknown] is a strong code smell.
    /// It is likely you should use [TypeSubstitutor::unify_must_succeed] or [TypeSubstitutor::unify_report_error] instead
    ///
    /// It should only occur in creation `DomainType::Unknown(self.domain_substitutor.alloc())`
    Unknown(DomainVariableID),
}

impl DomainType {
    pub fn unwrap_physical(&self) -> DomainID {
        let Self::Physical(w) = self else {
            unreachable!()
        };
        *w
    }
    pub fn is_generative(&self) -> bool {
        match self {
            DomainType::Generative => true,
            DomainType::Physical(_) => false,
            DomainType::Unknown(_) => false,
        }
    }
}

/// Represents all typing information needed in the Flattening Stage.
///
/// At the time being, this consists of the structural type ([AbstractType]), IE, if it's an `int`, `bool`, or `int[]`
/// And the domain ([DomainType]), which tracks part of what (clock) domain this wire is.
#[derive(Debug, Clone)]
pub struct FullType {
    pub typ: AbstractRankedType,
    pub domain: DomainType,
}

/// Performs Hindley-Milner typing during Flattening. (See [TypeSubstitutor])
///
/// 'A U 'x -> Substitute 'x = 'A
///
/// 'x U 'y -> Substitute 'x = 'y
pub struct TypeUnifier {
    pub template_type_names: FlatAlloc<String, TemplateIDMarker>,
    pub type_substitutor: TypeSubstitutor<AbstractInnerType, InnerTypeVariableIDMarker>,
    pub peano_substitutor: TypeSubstitutor<PeanoType, PeanoVariableIDMarker>,
    pub domain_substitutor: TypeSubstitutor<DomainType, DomainVariableIDMarker>,
    //pub type_map: HashMap<Type
}

impl TypeUnifier {
    pub fn new(parameters: &TVec<Parameter>, typing_alloc: TypingAllocator) -> Self {
        Self {
            template_type_names: map_to_type_names(parameters),
            type_substitutor: TypeSubstitutor::init(&typing_alloc.type_variable_alloc),
            peano_substitutor: TypeSubstitutor::init(&typing_alloc.peano_variable_alloc),
            domain_substitutor: TypeSubstitutor::init(&typing_alloc.domain_variable_alloc),
        }
    }

    pub fn alloc_typ_variable(&self) -> InnerTypeVariableID {
        self.type_substitutor.alloc()
    }

    pub fn alloc_peano_variable(&self) -> PeanoVariableID {
        self.peano_substitutor.alloc()
    }

    #[allow(dead_code)]
    pub fn alloc_domain_variable(&self) -> DomainVariableID {
        self.domain_substitutor.alloc()
    }

    /// This should always be what happens first to a given variable.
    ///
    /// Therefore it should be impossible that one of the internal unifications ever fails
    pub fn unify_with_written_type_must_succeed(
        &self,
        wr_typ: &WrittenType,
        typ: &AbstractRankedType,
    ) {
        match wr_typ {
            WrittenType::Error(_span) => {} // Already an error, don't unify
            WrittenType::TemplateVariable(_span, argument_id) => {
                self.type_substitutor
                    .unify_must_succeed(&typ.inner, &AbstractInnerType::Template(*argument_id));
            }
            WrittenType::Named(global_reference) => {
                self.type_substitutor
                    .unify_must_succeed(&typ.inner, &AbstractInnerType::Named(global_reference.id));
            }
            WrittenType::Array(_span, array_content_and_size) => {
                unreachable!("no array written type unifying yet"); // todo this

                /*let (arr_content, _size_flat, _array_bracket_span) = array_content_and_size.deref();

                let arr_content_variable = AbstractType::Unknown(self.alloc_typ_variable());

                self.type_substitutor.unify_must_succeed(
                    &typ.inner,
                    &AbstractType::Array(Box::new(arr_content_variable.clone())),
                );

                Self::unify_with_written_type_must_succeed(
                    self,
                    arr_content,
                    &arr_content_variable,
                );*/
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
    pub fn unify_with_written_type_substitute_templates_must_succeed(
        &self,
        wr_typ: &WrittenType,
        typ: &AbstractRankedType,
        template_type_args: &TVec<AbstractRankedType>,
    ) {
        match wr_typ {
            WrittenType::Error(_span) => {} // Already an error, don't unify
            WrittenType::TemplateVariable(_span, argument_id) => {
                self.type_substitutor
                    .unify_must_succeed(&typ.inner, &template_type_args[*argument_id].inner);

                self.peano_substitutor
                    .unify_must_succeed(&typ.rank, &template_type_args[*argument_id].rank);
            }
            WrittenType::Named(global_reference) => {
                self.type_substitutor
                    .unify_must_succeed(&typ.inner, &AbstractInnerType::Named(global_reference.id));
                self.peano_substitutor
                    .unify_must_succeed(&typ.rank, &PeanoType::Named(global_reference.id));
            }
            WrittenType::Array(_span, array_content_and_size) => {
                unreachable!("no array written type unifying yet"); // todo this

                /*
                let (arr_content, _size_flat, _array_bracket_span) = array_content_and_size.deref();

                let arr_content_variable = AbstractRankedType::Unknown(self.alloc_typ_variable());

                self.type_substitutor.unify_must_succeed(
                    typ,
                    &AbstractRankedType::Array(Box::new(arr_content_variable.clone())),
                );

                Self::unify_with_written_type_substitute_templates_must_succeed(
                    self,
                    arr_content,
                    &arr_content_variable,
                    template_type_args,
                );*/
            }
        }
    }

    /// TODO make WrittenValue compared to Value to encode Spans
    pub fn unify_with_constant(
        &mut self,
        typ: &AbstractRankedType,
        value: &Value,
        value_span: Span,
    ) {
        match value {
            // todo: also unify with rank
            Value::Bool(_) => self.type_substitutor.unify_report_error(
                &typ.inner,
                &BOOL_TYPE.inner,
                value_span,
                &"bool constant",
            ),
            Value::Integer(_big_int) => self.type_substitutor.unify_report_error(
                &typ.inner,
                &INT_TYPE.inner,
                value_span,
                &"int constant",
            ),
            Value::Array(arr) => {
                let arr_content_variable = AbstractRankedType {
                    inner: AbstractInnerType::Unknown(self.type_substitutor.alloc()),
                    rank: PeanoType::Unknown(self.peano_substitutor.alloc()),
                };

                for v in arr.deref() {
                    self.unify_with_constant(&arr_content_variable.rank_up(), v, value_span);
                }
                /*
                self.type_substitutor.unify_report_error(
                    typ,
                    &AbstractRankedType::Array(Box::new(arr_content_variable.clone())),
                    value_span,
                    "array constant",
                );*/
            }
            Value::Error | Value::Unset => {} // Already an error, don't unify
        }
    }

    // Unifies arr_type with output_typ[]
    pub fn unify_with_array_of(
        &self,
        arr_type: &AbstractRankedType,
        output_typ: AbstractRankedType,
        arr_span: Span,
    ) {
        self.type_substitutor.unify_report_error(
            &arr_type.inner,
            &Box::new(output_typ.inner),
            arr_span,
            &"array access",
        );

        self.peano_substitutor.unify_report_error(
            &arr_type.rank,
            &PeanoType::Succ(Box::new(output_typ.rank)),
            arr_span,
            &"array access",
        );
    }

    pub fn typecheck_unary_operator_abstr(
        &self,
        op: UnaryOperator,
        input_typ: &AbstractRankedType,
        span: Span,
        output_typ: &AbstractRankedType,
    ) {
        // todo: unify ranks
        if op == UnaryOperator::Not {
            self.type_substitutor.unify_report_error(
                &input_typ.inner,
                &BOOL_TYPE.inner,
                span,
                &"! input",
            );
            self.type_substitutor.unify_report_error(
                &output_typ.inner,
                &BOOL_TYPE.inner,
                span,
                &"! output",
            );
        } else if op == UnaryOperator::Negate {
            self.type_substitutor.unify_report_error(
                &input_typ.inner,
                &INT_TYPE.inner,
                span,
                &"unary - input",
            );
            self.type_substitutor.unify_report_error(
                &output_typ.inner,
                &INT_TYPE.inner,
                span,
                &"unary - output",
            );
        } else {
            let reduction_type = match op {
                UnaryOperator::And => BOOL_TYPE,
                UnaryOperator::Or => BOOL_TYPE,
                UnaryOperator::Xor => BOOL_TYPE,
                UnaryOperator::Sum => INT_TYPE,
                UnaryOperator::Product => INT_TYPE,
                _ => unreachable!(),
            };
            self.type_substitutor.unify_report_error(
                &output_typ.inner,
                &reduction_type.inner,
                span,
                &"array reduction",
            );
            self.unify_with_array_of(input_typ, output_typ.clone(), span);
        }
    }

    pub fn typecheck_binary_operator_abstr(
        &self,
        op: BinaryOperator,
        left_typ: &AbstractRankedType,
        right_typ: &AbstractRankedType,
        left_span: Span,
        right_span: Span,
        output_typ: &AbstractRankedType,
    ) {
        let (exp_left, exp_right, out_typ) = match op {
            BinaryOperator::And => (BOOL_TYPE, BOOL_TYPE, BOOL_TYPE),
            BinaryOperator::Or => (BOOL_TYPE, BOOL_TYPE, BOOL_TYPE),
            BinaryOperator::Xor => (BOOL_TYPE, BOOL_TYPE, BOOL_TYPE),
            BinaryOperator::Add => (INT_TYPE, INT_TYPE, INT_TYPE),
            BinaryOperator::Subtract => (INT_TYPE, INT_TYPE, INT_TYPE),
            BinaryOperator::Multiply => (INT_TYPE, INT_TYPE, INT_TYPE),
            BinaryOperator::Divide => (INT_TYPE, INT_TYPE, INT_TYPE),
            BinaryOperator::Modulo => (INT_TYPE, INT_TYPE, INT_TYPE),
            BinaryOperator::Equals => (INT_TYPE, INT_TYPE, BOOL_TYPE),
            BinaryOperator::NotEquals => (INT_TYPE, INT_TYPE, BOOL_TYPE),
            BinaryOperator::GreaterEq => (INT_TYPE, INT_TYPE, BOOL_TYPE),
            BinaryOperator::Greater => (INT_TYPE, INT_TYPE, BOOL_TYPE),
            BinaryOperator::LesserEq => (INT_TYPE, INT_TYPE, BOOL_TYPE),
            BinaryOperator::Lesser => (INT_TYPE, INT_TYPE, BOOL_TYPE),
        };

        self.type_substitutor.unify_report_error(
            &left_typ.inner,
            &exp_left.inner,
            left_span,
            &"binop left side",
        );

        // todo: unify ranks!!
        self.type_substitutor.unify_report_error(
            &right_typ.inner,
            &exp_right.inner,
            right_span,
            &"binop right side",
        );
        self.type_substitutor.unify_report_error(
            &output_typ.inner,
            &out_typ.inner,
            Span::new_overarching(left_span, right_span),
            &"binop output",
        );
    }

    // ===== Both =====

    pub fn unify_domains<Context: UnifyErrorReport>(
        &self,
        from_domain: &DomainType,
        to_domain: &DomainType,
        span: Span,
        context: Context,
    ) {
        // The case of writes to generatives from non-generatives should be fully covered by flattening
        if !from_domain.is_generative() && !to_domain.is_generative() {
            self.domain_substitutor
                .unify_report_error(from_domain, to_domain, span, &context);
        }
    }

    pub fn typecheck_unary_operator(
        &self,
        op: UnaryOperator,
        input_typ: &FullType,
        output_typ: &FullType,
        span: Span,
    ) {
        self.typecheck_unary_operator_abstr(op, &input_typ.typ, span, &output_typ.typ);
        self.unify_domains(&input_typ.domain, &output_typ.domain, span, "unary op");
    }

    pub fn typecheck_binary_operator(
        &self,
        op: BinaryOperator,
        left_typ: &FullType,
        right_typ: &FullType,
        left_span: Span,
        right_span: Span,
        output_typ: &FullType,
    ) {
        self.typecheck_binary_operator_abstr(
            op,
            &left_typ.typ,
            &right_typ.typ,
            left_span,
            right_span,
            &output_typ.typ,
        );
        self.unify_domains(
            &left_typ.domain,
            &output_typ.domain,
            left_span,
            "binop left",
        );
        self.unify_domains(
            &right_typ.domain,
            &output_typ.domain,
            right_span,
            "binop right",
        );
    }

    pub fn typecheck_array_access(
        &self,
        arr_type: &AbstractRankedType,
        idx_type: &AbstractRankedType,
        arr_span: Span,
        idx_span: Span,
        output_typ: &AbstractRankedType,
    ) {
        // Must unify arr_type's inner type with output_typ's inner type,
        // arr_type's rank type with succ(output_typ's ranked type) (and idx_type with int)

        self.type_substitutor.unify_report_error(
            &idx_type.inner,
            &INT_TYPE.inner,
            idx_span,
            &"array index",
        );
        self.peano_substitutor.unify_report_error(
            &idx_type.rank,
            &INT_TYPE.rank,
            idx_span,
            &"array index",
        );

        self.unify_with_array_of(arr_type, output_typ.clone(), arr_span);
    }

    pub fn typecheck_write_to_abstract<Context: UnifyErrorReport>(
        &self,
        found: &AbstractRankedType,
        expected: &AbstractRankedType,
        span: Span,
        context: &Context,
    ) {
        self.type_substitutor
            .unify_report_error(&found.inner, &expected.inner, span, context);

        self.peano_substitutor
            .unify_report_error(&found.rank, &expected.rank, span, context);
    }

    pub fn typecheck_write_to<Context: UnifyErrorReport + Clone>(
        &self,
        found: &FullType,
        expected: &FullType,
        span: Span,
        context: Context,
    ) {
        self.typecheck_write_to_abstract(&found.typ, &expected.typ, span, &mut context.clone());
        self.unify_domains(&found.domain, &expected.domain, span, context);
    }

    pub fn finalize_domain_type(&mut self, typ_domain: &mut DomainType) {
        use super::type_inference::HindleyMilner;
        assert!(typ_domain.fully_substitute(&self.domain_substitutor));
    }

    pub fn finalize_abstract_type(
        &mut self,
        inner_types: &ArenaAllocator<StructType, WholeTypeUUIDMarker>,
        rank_types: &ArenaAllocator<StructType, WholeTypeUUIDMarker>,
        typ: &mut AbstractRankedType,
        span: Span,
        errors: &ErrorCollector,
    ) {
        use super::type_inference::HindleyMilner;
        if !(typ.inner.fully_substitute(&self.type_substitutor)
            && typ.rank.fully_substitute(&self.peano_substitutor))
        {
            let typ_as_string = typ.display(inner_types, rank_types, &self.template_type_names);
            errors.error(
                span,
                format!("Could not fully figure out the type of this object. {typ_as_string}"),
            );
        }
    }

    pub fn finalize_type(
        &mut self,
        inner_types: &ArenaAllocator<StructType, WholeTypeUUIDMarker>,
        rank_types: &ArenaAllocator<StructType, WholeTypeUUIDMarker>,
        typ: &mut FullType,
        span: Span,
        errors: &ErrorCollector,
    ) {
        self.finalize_domain_type(&mut typ.domain);
        self.finalize_abstract_type(inner_types, rank_types, &mut typ.typ, span, errors);
    }

    pub fn finalize_global_ref<ID>(
        &mut self,
        inner_types: &ArenaAllocator<StructType, WholeTypeUUIDMarker>,
        rank_types: &ArenaAllocator<StructType, WholeTypeUUIDMarker>,
        global_ref: &mut GlobalReference<ID>,
        errors: &ErrorCollector,
    ) {
        let global_ref_span = global_ref.get_total_span();
        for (_template_id, template_type) in &mut global_ref.template_arg_types {
            self.finalize_abstract_type(
                inner_types,
                rank_types,
                template_type,
                global_ref_span,
                errors,
            );
        }
    }
}
