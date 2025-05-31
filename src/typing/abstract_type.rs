use sus_proc_macro::get_builtin_type;

use crate::prelude::*;

use super::template::{Parameter, TVec};
use super::type_inference::{
    AbstractTypeSubstitutor, DomainVariableID, InnerTypeVariableID, PeanoVariableID, Substitutor,
    TypeSubstitutor, TypeUnifier, UnifyErrorReport,
};
use crate::flattening::{BinaryOperator, UnaryOperator};
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
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AbstractInnerType {
    Template(TemplateID),
    Named(TypeUUID),
    /// Referencing [AbstractType::Unknown] is a strong code smell.
    /// It is likely you should use [TypeSubstitutor::unify_must_succeed] or [TypeSubstitutor::unify_report_error] instead
    ///
    /// It should only occur in creation `AbstractType::Unknown(self.type_substitutor.alloc())`
    Unknown(InnerTypeVariableID),
}

impl AbstractInnerType {
    pub fn scalar(self) -> AbstractRankedType {
        AbstractRankedType {
            inner: self,
            rank: PeanoType::Zero,
        }
    }
    pub fn with_rank(self, rank: PeanoType) -> AbstractRankedType {
        AbstractRankedType { inner: self, rank }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
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
    pub fn rank_up(self) -> Self {
        Self {
            inner: self.inner,
            rank: PeanoType::Succ(Box::new(self.rank)),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PeanoType {
    Zero,
    Succ(Box<PeanoType>),
    Unknown(PeanoVariableID),
}

impl PeanoType {
    pub fn count(&self) -> Option<usize> {
        match self {
            PeanoType::Zero => Some(0),
            PeanoType::Succ(inner) => Some(inner.count()? + 1),
            PeanoType::Unknown(_) => None,
        }
    }
    pub fn count_unwrap(&self) -> usize {
        let Some(cnt) = self.count() else {
            panic!("Peano Number {self:?} still contains Unknown!");
        };
        cnt
    }
    pub fn from_natural(count: usize) -> Self {
        if count == 0 {
            PeanoType::Zero
        } else {
            PeanoType::Succ(Box::new(PeanoType::from_natural(count - 1)))
        }
    }
}

pub const BOOL_TYPE: AbstractInnerType = AbstractInnerType::Named(get_builtin_type!("bool"));
pub const INT_TYPE: AbstractInnerType = AbstractInnerType::Named(get_builtin_type!("int"));

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
    /// Used to quickly combine domains with each other. NOT A SUBSTITUTE FOR UNIFICATION.
    pub fn combine_with(&mut self, other: DomainType) {
        if *self == DomainType::Generative {
            *self = other;
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
pub struct FullTypeUnifier {
    pub template_type_names: FlatAlloc<String, TemplateIDMarker>,
    pub abstract_type_substitutor: TypeUnifier<AbstractTypeSubstitutor>,
    pub domain_substitutor: TypeUnifier<TypeSubstitutor<DomainType>>,
}

impl FullTypeUnifier {
    pub fn new(
        parameters: &TVec<Parameter>,
        typing_alloc: (AbstractTypeSubstitutor, TypeSubstitutor<DomainType>),
    ) -> Self {
        Self {
            template_type_names: map_to_type_names(parameters),
            abstract_type_substitutor: typing_alloc.0.into(),
            domain_substitutor: typing_alloc.1.into(),
        }
    }

    pub fn unify_must_succeed(&mut self, ta: &FullType, tb: &FullType) {
        self.abstract_type_substitutor
            .unify_must_succeed(&ta.typ, &tb.typ);

        if !ta.domain.is_generative() && !tb.domain.is_generative() {
            self.domain_substitutor
                .unify_must_succeed(&ta.domain, &tb.domain);
        }
    }

    pub fn typecheck_unary_operator_abstr(
        &mut self,
        op: UnaryOperator,
        op_rank: &PeanoType,
        input_typ: &AbstractRankedType,
        span: Span,
        output_typ: &AbstractRankedType,
    ) {
        if op == UnaryOperator::Not {
            self.abstract_type_substitutor.unify_report_error(
                input_typ,
                &BOOL_TYPE.with_rank(op_rank.clone()),
                span,
                "! input",
            );

            self.abstract_type_substitutor.unify_report_error(
                output_typ,
                &BOOL_TYPE.with_rank(op_rank.clone()),
                span,
                "! output",
            );
        } else if op == UnaryOperator::Negate {
            self.abstract_type_substitutor.unify_report_error(
                input_typ,
                &INT_TYPE.with_rank(op_rank.clone()),
                span,
                "unary - input",
            );
            self.abstract_type_substitutor.unify_report_error(
                output_typ,
                &INT_TYPE.with_rank(op_rank.clone()),
                span,
                "unary - output",
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
                output_typ,
                &reduction_type.with_rank(op_rank.clone()),
                span,
                "array reduction",
            );
            {
                let this = &mut *self;
                let output_typ = output_typ.clone();
                this.abstract_type_substitutor.unify_report_error(
                    input_typ,
                    &output_typ.rank_up(),
                    span,
                    "array access",
                );
            };
        }
    }

    #[allow(clippy::too_many_arguments)]
    pub fn typecheck_binary_operator_abstr(
        &mut self,
        op: BinaryOperator,
        op_rank: &PeanoType,
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
        let exp_left = exp_left.with_rank(op_rank.clone());
        let exp_right = exp_right.with_rank(op_rank.clone());
        let out_typ = out_typ.with_rank(op_rank.clone());

        self.abstract_type_substitutor.unify_report_error(
            left_typ,
            &exp_left,
            left_span,
            "binop left side",
        );
        self.abstract_type_substitutor.unify_report_error(
            right_typ,
            &exp_right,
            right_span,
            "binop right side",
        );

        self.abstract_type_substitutor.unify_report_error(
            output_typ,
            &out_typ,
            Span::new_overarching(left_span, right_span),
            "binop output",
        );
    }

    // ===== Both =====

    pub fn unify_domains<Context: UnifyErrorReport>(
        &mut self,
        from_domain: &DomainType,
        to_domain: &DomainType,
        span: Span,
        context: Context,
    ) {
        // The case of writes to generatives from non-generatives should be fully covered by flattening
        if !from_domain.is_generative() && !to_domain.is_generative() {
            self.domain_substitutor
                .unify_report_error(from_domain, to_domain, span, context);
        }
    }

    pub fn unify_write_to_abstract<Context: UnifyErrorReport>(
        &mut self,
        found: &AbstractRankedType,
        expected: &AbstractRankedType,
        span: Span,
        context: Context,
    ) {
        self.abstract_type_substitutor
            .unify_report_error(found, expected, span, context);
    }

    pub fn unify_write_to<Context: UnifyErrorReport + Clone>(
        &mut self,
        found_typ: &AbstractRankedType,
        found_domain: &DomainType,
        expected: &FullType,
        span: Span,
        context: Context,
    ) {
        self.unify_write_to_abstract(found_typ, &expected.typ, span, context.clone());
        self.unify_domains(found_domain, &expected.domain, span, context);
    }
}
