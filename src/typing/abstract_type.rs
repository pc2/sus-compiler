use std::ops::Deref;

use sus_proc_macro::get_builtin_type;

use crate::prelude::*;

use super::template::{Parameter, TVec};
use super::type_inference::{
    AbstractTypeSubstitutor, InnerTypeVariableID, PeanoVariableID, TypeUnifier, UnifyErrorReport,
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
}

pub const BOOL_TYPE: AbstractInnerType = AbstractInnerType::Named(get_builtin_type!("bool"));
pub const INT_TYPE: AbstractInnerType = AbstractInnerType::Named(get_builtin_type!("int"));

/// Performs Hindley-Milner typing during Flattening. (See [TypeSubstitutor])
///
/// 'A U 'x -> Substitute 'x = 'A
///
/// 'x U 'y -> Substitute 'x = 'y
pub struct FullTypeUnifier {
    pub template_type_names: FlatAlloc<String, TemplateIDMarker>,
    pub abstract_type_substitutor: TypeUnifier<AbstractTypeSubstitutor>,
}

impl FullTypeUnifier {
    pub fn new(parameters: &TVec<Parameter>, typing_alloc: AbstractTypeSubstitutor) -> Self {
        Self {
            template_type_names: map_to_type_names(parameters),
            abstract_type_substitutor: typing_alloc.into(),
        }
    }

    /// Returns the type of the content of the array
    pub fn rank_down<Report: UnifyErrorReport>(
        &mut self,
        arr_typ: &AbstractRankedType,
        span: Span,
        context: Report,
    ) -> AbstractRankedType {
        if let PeanoType::Succ(content_rank) = &arr_typ.rank {
            AbstractRankedType {
                inner: arr_typ.inner.clone(),
                rank: content_rank.deref().clone(),
            }
        } else {
            let content_rank = self
                .abstract_type_substitutor
                .rank_substitutor
                .alloc_unknown();
            let mut content_typ = AbstractRankedType {
                inner: arr_typ.inner.clone(),
                rank: PeanoType::Succ(Box::new(content_rank.clone())),
            };
            self.abstract_type_substitutor
                .unify_report_error(arr_typ, &content_typ, span, context);
            content_typ.rank = content_rank;
            content_typ
        }
    }

    /// Returns the output type. It happens that the operator rank is the output type's rank
    pub fn typecheck_unary_operator_abstr(
        &mut self,
        op: UnaryOperator,
        input_typ: &AbstractRankedType,
        span: Span,
    ) -> AbstractRankedType {
        let input_rank = input_typ.rank.clone();
        if op == UnaryOperator::Not {
            self.abstract_type_substitutor.unify_report_error(
                input_typ,
                &BOOL_TYPE.with_rank(input_rank.clone()),
                span,
                "! input",
            );

            BOOL_TYPE.with_rank(input_rank)
        } else if op == UnaryOperator::Negate {
            self.abstract_type_substitutor.unify_report_error(
                input_typ,
                &INT_TYPE.with_rank(input_rank.clone()),
                span,
                "unary - input",
            );
            INT_TYPE.with_rank(input_rank)
        } else {
            let reduction_type = match op {
                UnaryOperator::And => BOOL_TYPE,
                UnaryOperator::Or => BOOL_TYPE,
                UnaryOperator::Xor => BOOL_TYPE,
                UnaryOperator::Sum => INT_TYPE,
                UnaryOperator::Product => INT_TYPE,
                _ => unreachable!(),
            };
            let reduction_type = reduction_type.with_rank(input_rank.clone());
            self.abstract_type_substitutor.unify_report_error(
                input_typ,
                &reduction_type,
                span,
                "array reduction",
            );
            self.rank_down(&reduction_type, span, "array reduction")
        }
    }

    pub fn typecheck_binary_operator_abstr(
        &mut self,
        op: BinaryOperator,
        left_typ: &AbstractRankedType,
        right_typ: &AbstractRankedType,
        left_span: Span,
        right_span: Span,
    ) -> AbstractRankedType {
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
        let input_rank = left_typ.rank.clone();
        let exp_left = exp_left.with_rank(input_rank.clone());
        let exp_right = exp_right.with_rank(input_rank.clone());
        let out_typ = out_typ.with_rank(input_rank.clone());

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
        out_typ
    }
}
