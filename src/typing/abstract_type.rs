use sus_proc_macro::get_builtin_type;

use crate::alloc::ArenaAllocator;
use crate::prelude::*;
use crate::value::Value;
use std::fmt::{self, Display, Formatter};
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
    Named(TypeUUID),
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
    Template(TemplateID),
    Succ(Box<PeanoType>),
    Unknown(PeanoVariableID),
}

impl PeanoType {
    pub fn as_integer_must_succeed(&self) -> usize {
        match self {
            PeanoType::Zero => 0,
            PeanoType::Succ(inner) => inner.as_integer_must_succeed() + 1,
            _ => panic!("Could not convert {self:?} Peano type to integer"),
        }
    }
}

pub const BOOL_TYPE: AbstractRankedType =
    AbstractRankedType::scalar(AbstractInnerType::Named(get_builtin_type!("bool")));
pub const INT_TYPE: AbstractRankedType =
    AbstractRankedType::scalar(AbstractInnerType::Named(get_builtin_type!("int")));

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

impl Display for PeanoType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            PeanoType::Zero => write!(f, ""),
            PeanoType::Template(id) => write!(f, "<[T]>"),
            PeanoType::Succ(inner) => write!(f, "{}[]", inner.to_string()),
            PeanoType::Unknown(id) => write!(f, "<[...]>"),
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
    pub abstract_type_substitutor: TypeSubstitutor<AbstractInnerType, InnerTypeVariableIDMarker>,
    pub peano_substitutor: TypeSubstitutor<PeanoType, PeanoVariableIDMarker>,
    pub domain_substitutor: TypeSubstitutor<DomainType, DomainVariableIDMarker>,
}

impl TypeUnifier {
    pub fn new(parameters: &TVec<Parameter>, typing_alloc: TypingAllocator) -> Self {
        let p = TypeSubstitutor::init(&typing_alloc.peano_variable_alloc);
        p.alloc();
        // Allocate the identifier for the Peano variable with identity 0, the 0 variable. Needed so that Zero
        // can be a HM type var rather than a type func.
        Self {
            template_type_names: map_to_type_names(parameters),
            abstract_type_substitutor: TypeSubstitutor::init(
                &typing_alloc.inner_type_variable_alloc,
            ),
            peano_substitutor: p,
            domain_substitutor: TypeSubstitutor::init(&typing_alloc.domain_variable_alloc),
        }
    }

    pub fn alloc_typ_variable(&self) -> InnerTypeVariableID {
        self.abstract_type_substitutor.alloc()
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
                self.abstract_type_substitutor
                    .unify_must_succeed(&typ.inner, &AbstractInnerType::Template(*argument_id));
                self.peano_substitutor
                    .unify_must_succeed(&typ.rank, &PeanoType::Template(*argument_id));
            }
            WrittenType::Named(global_reference) => {
                self.abstract_type_substitutor
                    .unify_must_succeed(&typ.inner, &AbstractInnerType::Named(global_reference.id));
                self.peano_substitutor
                    .unify_must_succeed(&typ.rank, &PeanoType::Zero);
            }
            WrittenType::Array(_span, array_content_and_size) => {
                let (arr_content_type, _size_flat, _array_bracket_span) =
                    array_content_and_size.deref();

                let arr_content_variable = AbstractRankedType {
                    inner: AbstractInnerType::Unknown(self.alloc_typ_variable()),
                    rank: PeanoType::Unknown(self.alloc_peano_variable()),
                };

                self.abstract_type_substitutor
                    .unify_must_succeed(&typ.inner, &Box::new(arr_content_variable.inner.clone()));

                self.peano_substitutor
                    .unify_must_succeed(&typ.rank, &arr_content_variable.rank_up().rank);

                Self::unify_with_written_type_must_succeed(
                    self,
                    arr_content_type,
                    &arr_content_variable,
                );
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
                self.abstract_type_substitutor
                    .unify_must_succeed(&typ.inner, &template_type_args[*argument_id].inner);

                self.peano_substitutor
                    .unify_must_succeed(&typ.rank, &template_type_args[*argument_id].rank);
            }
            WrittenType::Named(global_reference) => {
                self.abstract_type_substitutor
                    .unify_must_succeed(&typ.inner, &AbstractInnerType::Named(global_reference.id));
                self.peano_substitutor
                    .unify_must_succeed(&typ.rank, &PeanoType::Zero);
            }
            WrittenType::Array(_span, array_content_and_size) => {
                let (arr_content_type, _size_flat, _array_bracket_span) =
                    array_content_and_size.deref();

                let arr_content_variable = AbstractRankedType {
                    inner: AbstractInnerType::Unknown(self.alloc_typ_variable()),
                    rank: PeanoType::Unknown(self.alloc_peano_variable()),
                };

                self.abstract_type_substitutor
                    .unify_must_succeed(&typ.inner, &Box::new(arr_content_variable.inner.clone()));

                self.peano_substitutor
                    .unify_must_succeed(&typ.rank, &arr_content_variable.rank_up().rank);

                Self::unify_with_written_type_substitute_templates_must_succeed(
                    self,
                    arr_content_type,
                    &arr_content_variable,
                    template_type_args,
                );
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
            Value::Bool(_) => {
                self.abstract_type_substitutor.unify_report_error(
                    &typ.inner,
                    &BOOL_TYPE.inner,
                    value_span,
                    &"bool constant",
                );
                self.peano_substitutor.unify_report_error(
                    &typ.rank,
                    &BOOL_TYPE.rank,
                    value_span,
                    &"bool constant rank",
                );
            }
            Value::Integer(_big_int) => {
                self.abstract_type_substitutor.unify_report_error(
                    &typ.inner,
                    &INT_TYPE.inner,
                    value_span,
                    &"int constant",
                );
                self.peano_substitutor.unify_report_error(
                    &typ.rank,
                    &INT_TYPE.rank,
                    value_span,
                    &"int constant rank",
                );
            }
            Value::Array(arr) => {
                let arr_content_variable = AbstractRankedType {
                    inner: AbstractInnerType::Unknown(self.abstract_type_substitutor.alloc()),
                    rank: PeanoType::Unknown(self.peano_substitutor.alloc()),
                };

                for v in arr.deref() {
                    self.unify_with_constant(&arr_content_variable, v, value_span);
                }

                self.abstract_type_substitutor.unify_report_error(
                    &typ.inner,
                    &arr_content_variable.inner,
                    value_span,
                    &"array literal",
                );

                self.peano_substitutor.unify_report_error(
                    &typ.rank_up().rank,
                    &arr_content_variable.rank,
                    value_span,
                    &"array literal rank",
                );
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
        self.abstract_type_substitutor.unify_report_error(
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
        if op == UnaryOperator::Not {
            self.abstract_type_substitutor.unify_report_error(
                &input_typ.inner,
                &BOOL_TYPE.inner,
                span,
                &"! input",
            );
            self.peano_substitutor.unify_report_error(
                &input_typ.rank,
                &BOOL_TYPE.rank,
                span,
                &"! input rank",
            );

            self.abstract_type_substitutor.unify_report_error(
                &output_typ.inner,
                &BOOL_TYPE.inner,
                span,
                &"! output",
            );
            self.peano_substitutor.unify_report_error(
                &output_typ.rank,
                &BOOL_TYPE.rank,
                span,
                &"! output rank",
            );
        } else if op == UnaryOperator::Negate {
            self.abstract_type_substitutor.unify_report_error(
                &input_typ.inner,
                &INT_TYPE.inner,
                span,
                &"unary - input",
            );
            self.peano_substitutor.unify_report_error(
                &input_typ.rank,
                &INT_TYPE.rank,
                span,
                &"unary - input rank",
            );
            self.abstract_type_substitutor.unify_report_error(
                &output_typ.inner,
                &INT_TYPE.inner,
                span,
                &"unary - output",
            );
            self.peano_substitutor.unify_report_error(
                &output_typ.rank,
                &INT_TYPE.rank,
                span,
                &"unary - output rank",
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
            self.abstract_type_substitutor.unify_report_error(
                &output_typ.inner,
                &reduction_type.inner,
                span,
                &"array reduction",
            );
            self.peano_substitutor.unify_report_error(
                &output_typ.rank,
                &reduction_type.rank,
                span,
                &"array reduction rank",
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

        self.abstract_type_substitutor.unify_report_error(
            &left_typ.inner,
            &exp_left.inner,
            left_span,
            &"binop left side",
        );
        self.abstract_type_substitutor.unify_report_error(
            &right_typ.inner,
            &exp_right.inner,
            right_span,
            &"binop right side",
        );

        self.abstract_type_substitutor.unify_report_error(
            &output_typ.inner,
            &out_typ.inner,
            Span::new_overarching(left_span, right_span),
            &"binop output",
        );
        self.peano_substitutor.unify_report_error(
            &output_typ.rank,
            &left_typ.rank,
            left_span,
            &"binop left rank",
        );
        self.peano_substitutor.unify_report_error(
            &output_typ.rank,
            &right_typ.rank,
            right_span,
            &"binop right rank",
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

        self.abstract_type_substitutor.unify_report_error(
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
        self.abstract_type_substitutor.unify_report_error(
            &found.inner,
            &expected.inner,
            span,
            context,
        );

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
        self.typecheck_write_to_abstract(&found.typ, &expected.typ, span, &context);
        self.unify_domains(&found.domain, &expected.domain, span, context);
    }

    pub fn finalize_domain_type(&mut self, typ_domain: &mut DomainType) {
        use super::type_inference::HindleyMilner;
        assert!(typ_domain.fully_substitute(&self.domain_substitutor));
    }

    pub fn finalize_abstract_type(
        &mut self,
        linker_types: &ArenaAllocator<StructType, TypeUUIDMarker>,
        typ: &mut AbstractRankedType,
        span: Span,
        errors: &ErrorCollector,
    ) {
        use super::type_inference::HindleyMilner;
        if !(typ.inner.fully_substitute(&self.abstract_type_substitutor)
            && typ.rank.fully_substitute(&self.peano_substitutor))
        {
            let typ_as_string = typ.display(linker_types, &self.template_type_names);
            errors.error(
                span,
                format!("Could not fully figure out the type of this object. {typ_as_string}"),
            );
        }
    }

    pub fn finalize_type(
        &mut self,
        linker_types: &ArenaAllocator<StructType, TypeUUIDMarker>,
        typ: &mut FullType,
        span: Span,
        errors: &ErrorCollector,
    ) {
        self.finalize_domain_type(&mut typ.domain);
        self.finalize_abstract_type(linker_types, &mut typ.typ, span, errors);
    }

    pub fn finalize_global_ref<ID>(
        &mut self,
        linker_types: &ArenaAllocator<StructType, TypeUUIDMarker>,
        global_ref: &mut GlobalReference<ID>,
        errors: &ErrorCollector,
    ) {
        let global_ref_span = global_ref.get_total_span();
        for (_template_id, template_type) in &mut global_ref.template_arg_types {
            self.finalize_abstract_type(linker_types, template_type, global_ref_span, errors);
        }
    }
}
