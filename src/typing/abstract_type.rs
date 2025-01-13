use crate::alloc::ArenaAllocator;
use crate::prelude::*;
use crate::value::Value;
use std::ops::Deref;

use super::template::{GlobalReference, TemplateAbstractTypes, Parameters};
use super::type_inference::{DomainVariableID, DomainVariableIDMarker, TypeSubstitutor, TypeVariableID, TypeVariableIDMarker, UnifyErrorReport};
use crate::flattening::{BinaryOperator, StructType, TypingAllocator, UnaryOperator, WrittenType};
use crate::linker::get_builtin_type;
use crate::to_string::map_to_type_names;

/// This contains only the information that can be easily type-checked.
///
/// Its most important components are the names and structure of types.
///
/// What isn't included are the parameters of types. So Array Sizes for example.
#[derive(Debug, Clone)]
pub enum AbstractType {
    Template(TemplateID),
    Named(TypeUUID),
    Array(Box<AbstractType>),
    /// Referencing [AbstractType::Unknown] is a strong code smell. 
    /// It is likely you should use [TypeSubstitutor::unify] instead
    /// 
    /// It should only occur in creation `AbstractType::Unknown(self.type_substitutor.alloc())`
    Unknown(TypeVariableID),
}

pub const BOOL_TYPE: AbstractType = AbstractType::Named(get_builtin_type("bool"));
pub const INT_TYPE: AbstractType = AbstractType::Named(get_builtin_type("int"));

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
    /// It is likely you should use [TypeSubstitutor::unify] instead
    /// 
    /// It should only occur in creation `DomainType::Unknown(self.domain_substitutor.alloc())`
    Unknown(DomainVariableID)
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
    pub typ: AbstractType,
    pub domain: DomainType,
}

/// Performs Hindley-Milner typing during Flattening. (See [TypeSubstitutor])
/// 
/// 'A U 'x -> Substitute 'x = 'A
///
/// 'x U 'y -> Substitute 'x = 'y
pub struct TypeUnifier {
    pub template_type_names: FlatAlloc<String, TemplateIDMarker>,
    pub type_substitutor: TypeSubstitutor<AbstractType, TypeVariableIDMarker>,
    pub domain_substitutor: TypeSubstitutor<DomainType, DomainVariableIDMarker>,
}

impl TypeUnifier {
    pub fn new(
        parameters: &Parameters,
        typing_alloc: TypingAllocator
    ) -> Self {
        Self {
            template_type_names: map_to_type_names(parameters),
            type_substitutor: TypeSubstitutor::init(&typing_alloc.type_variable_alloc),
            domain_substitutor: TypeSubstitutor::init(&typing_alloc.domain_variable_alloc)
        }
    }

    pub fn alloc_typ_variable(&self) -> TypeVariableID {
        self.type_substitutor.alloc()
    }

    #[allow(dead_code)]
    pub fn alloc_domain_variable(&self) -> DomainVariableID {
        self.domain_substitutor.alloc()
    }

    /// This should always be what happens first to a given variable. 
    /// 
    /// Therefore it should be impossible that one of the internal unifications ever fails
    pub fn unify_with_written_type_must_succeed(&self, wr_typ: &WrittenType, typ: &AbstractType) {
        match wr_typ {
            WrittenType::Error(_span) => {} // Already an error, don't unify
            WrittenType::TemplateVariable(_span, argument_id) => {
                self.type_substitutor.unify_must_succeed(&typ, &AbstractType::Template(*argument_id));
            }
            WrittenType::Named(global_reference) => {
                self.type_substitutor.unify_must_succeed(&typ, &AbstractType::Named(global_reference.id));
            }
            WrittenType::Array(_span, array_content_and_size) =>  {
                let (arr_content, _size_flat, _array_bracket_span) = array_content_and_size.deref();

                let arr_content_variable = AbstractType::Unknown(self.alloc_typ_variable());

                self.type_substitutor.unify_must_succeed(typ, &AbstractType::Array(Box::new(arr_content_variable.clone())));

                Self::unify_with_written_type_must_succeed(self, arr_content, &arr_content_variable);
            }
        }
    }

    /// This should always be what happens first to a given variable. 
    /// 
    /// Therefore it should be impossible that one of the internal unifications ever fails
    pub fn unify_with_written_type_substitute_templates_must_succeed(&self, wr_typ: &WrittenType, typ: &AbstractType, template_type_args: &TemplateAbstractTypes) {
        match wr_typ {
            WrittenType::Error(_span) => {} // Already an error, don't unify
            WrittenType::TemplateVariable(_span, argument_id) => {
                self.type_substitutor.unify_must_succeed(&typ, &template_type_args[*argument_id]);
            }
            WrittenType::Named(global_reference) => {
                self.type_substitutor.unify_must_succeed(&typ, &AbstractType::Named(global_reference.id));
            }
            WrittenType::Array(_span, array_content_and_size) =>  {
                let (arr_content, _size_flat, _array_bracket_span) = array_content_and_size.deref();

                let arr_content_variable = AbstractType::Unknown(self.alloc_typ_variable());

                self.type_substitutor.unify_must_succeed(typ, &AbstractType::Array(Box::new(arr_content_variable.clone())));

                Self::unify_with_written_type_substitute_templates_must_succeed(self, arr_content, &arr_content_variable, template_type_args);
            }
        }
    }

    /// TODO make WrittenValue compared to Value to encode Spans
    pub fn unify_with_constant(&mut self, typ: &AbstractType, value: &Value, value_span: Span) {
        match value {
            Value::Bool(_) => self.type_substitutor.unify_report_error(typ, &BOOL_TYPE, value_span, "bool constant"),
            Value::Integer(_big_int) => self.type_substitutor.unify_report_error(typ, &INT_TYPE, value_span, "int constant"),
            Value::Array(arr) => {
                let arr_content_variable = AbstractType::Unknown(self.alloc_typ_variable());
                self.type_substitutor.unify_report_error(typ, &AbstractType::Array(Box::new(arr_content_variable.clone())), value_span, "array constant");
                
                for v in arr.deref() {
                    self.unify_with_constant(&arr_content_variable, v, value_span);
                }
            }
            Value::Error | Value::Unset => {} // Already an error, don't unify
        }
    }

    // Unifies arr_type with output_typ[]
    pub fn unify_with_array_of(
        &self,
        arr_type: &AbstractType,
        output_typ: AbstractType,
        arr_span: Span,
    ) {
        self.type_substitutor.unify_report_error(arr_type, &AbstractType::Array(Box::new(output_typ)), arr_span, "array access");
    }

    pub fn typecheck_unary_operator_abstr(
        &self,
        op: UnaryOperator,
        input_typ: &AbstractType,
        span: Span,
        output_typ: &AbstractType,
    ) {
        if op == UnaryOperator::Not {
            self.type_substitutor.unify_report_error(input_typ, &BOOL_TYPE, span, "! input");
            self.type_substitutor.unify_report_error(output_typ, &BOOL_TYPE, span, "! output");
        } else if op == UnaryOperator::Negate {
            self.type_substitutor.unify_report_error(input_typ, &INT_TYPE, span, "unary - input");
            self.type_substitutor.unify_report_error(output_typ, &INT_TYPE, span, "unary - output");
        } else {
            let reduction_type = match op {
                UnaryOperator::And => BOOL_TYPE,
                UnaryOperator::Or => BOOL_TYPE,
                UnaryOperator::Xor => BOOL_TYPE,
                UnaryOperator::Sum => INT_TYPE,
                UnaryOperator::Product => INT_TYPE,
                _ => unreachable!(),
            };
            self.type_substitutor.unify_report_error(output_typ, &reduction_type, span, "array reduction");
            self.unify_with_array_of(input_typ, output_typ.clone(), span);
        }
    }

    pub fn typecheck_binary_operator_abstr(
        &self,
        op: BinaryOperator,
        left_typ: &AbstractType,
        right_typ: &AbstractType,
        left_span: Span,
        right_span: Span,
        output_typ: &AbstractType,
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
            left_typ,
            &exp_left,
            left_span,
            "binop left side"
        );
        self.type_substitutor.unify_report_error(
            right_typ,
            &exp_right,
            right_span,
            "binop right side"
        );
        self.type_substitutor.unify_report_error(
            output_typ,
            &out_typ,
            Span::new_overarching(left_span, right_span),
            "binop output"
        );
    }

    // ===== Both =====

    pub fn unify_domains<Context: UnifyErrorReport>(
        &self,
        from_domain: &DomainType,
        to_domain: &DomainType,
        span: Span,
        context: Context
    ) {
        // The case of writes to generatives from non-generatives should be fully covered by flattening
        if !from_domain.is_generative() && !to_domain.is_generative() {
            self.domain_substitutor.unify_report_error(&from_domain, &to_domain, span, context);
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
        self.typecheck_binary_operator_abstr(op, &left_typ.typ, &right_typ.typ, left_span, right_span, &output_typ.typ);
        self.unify_domains(&left_typ.domain, &output_typ.domain, left_span, "binop left");
        self.unify_domains(&right_typ.domain, &output_typ.domain, right_span, "binop right");
    }

    pub fn typecheck_array_access(
        &self,
        arr_type: &AbstractType,
        idx_type: &AbstractType,
        arr_span: Span,
        idx_span: Span,
        output_typ: &AbstractType,
    ) {
        self.type_substitutor.unify_report_error(&idx_type, &INT_TYPE, idx_span, "array index");
        self.unify_with_array_of(&arr_type, output_typ.clone(), arr_span);
    }

    pub fn typecheck_write_to_abstract<Context: UnifyErrorReport>(
        &self,
        found: &AbstractType,
        expected: &AbstractType,
        span: Span,
        context: Context
    ) {
        self.type_substitutor.unify_report_error(&found, &expected, span, context);
    }

    pub fn typecheck_write_to<Context: UnifyErrorReport+Clone>(
        &self,
        found: &FullType,
        expected: &FullType,
        span: Span,
        context: Context
    ) {
        self.typecheck_write_to_abstract(&found.typ, &expected.typ, span, context.clone());
        self.unify_domains(&found.domain, &expected.domain, span, context);
    }

    pub fn finalize_domain_type(&mut self, typ_domain: &mut DomainType) {
        use super::type_inference::HindleyMilner;
        assert!(typ_domain.fully_substitute(&self.domain_substitutor) == true);
    }

    pub fn finalize_abstract_type(&mut self, types: &ArenaAllocator<StructType, TypeUUIDMarker>, typ: &mut AbstractType, span: Span, errors: &ErrorCollector) {
        use super::type_inference::HindleyMilner;
        if typ.fully_substitute(&self.type_substitutor) == false {
            let typ_as_string = typ.display(types, &self.template_type_names);
            errors.error(span, format!("Could not fully figure out the type of this object. {typ_as_string}"));
        }
    }

    pub fn finalize_type(&mut self, types: &ArenaAllocator<StructType, TypeUUIDMarker>, typ: &mut FullType, span: Span, errors: &ErrorCollector) {
        self.finalize_domain_type(&mut typ.domain);
        self.finalize_abstract_type(types, &mut typ.typ, span, errors);
    }

    pub fn finalize_global_ref<ID>(&mut self, types: &ArenaAllocator<StructType, TypeUUIDMarker>, global_ref: &mut GlobalReference<ID>, errors: &ErrorCollector) {
        let global_ref_span = global_ref.get_total_span();
        for (_template_id, template_type) in &mut global_ref.template_arg_types {
            self.finalize_abstract_type(types, template_type, global_ref_span, errors);
        }
    }
}
