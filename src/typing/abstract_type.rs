use crate::alloc::ArenaAllocator;
use crate::prelude::*;
use crate::value::Value;

use std::ops::Deref;

use super::template::TemplateInputs;
use super::type_inference::{DomainVariableID, DomainVariableIDMarker, TypeSubstitutor, TypeVariableID, TypeVariableIDMarker};
use crate::flattening::{BinaryOperator, Instruction, StructType, TypingAllocator, UnaryOperator, WrittenType};
use crate::linker::get_builtin_type;
use crate::to_string::map_to_type_names;

/// This contains only the information that can be easily type-checked.
///
/// Its most important components are the names and structure of types.
///
/// What isn't included are the parameters of types. So Array Sizes for example.
#[derive(Debug, Clone)]
pub enum AbstractType {
    Error,
    Unknown(TypeVariableID),
    Template(TemplateID),
    Named(TypeUUID),
    Array(Box<AbstractType>),
}

impl AbstractType {
    pub fn contains_error_or_unknown<const CHECK_ERROR: bool, const CHECK_UNKNOWN: bool>(
        &self,
    ) -> bool {
        match self {
            AbstractType::Error => CHECK_ERROR,
            AbstractType::Unknown(_) => CHECK_UNKNOWN,
            AbstractType::Template(_id) => false,
            AbstractType::Named(_id) => false,
            AbstractType::Array(arr_box) => arr_box
                .deref()
                .contains_error_or_unknown::<CHECK_ERROR, CHECK_UNKNOWN>(),
        }
    }
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
    DomainVariable(DomainVariableID)
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
            DomainType::DomainVariable(_) => false,
        }
    }
}

#[derive(Debug, Clone)]
pub struct FullType {
    pub typ: AbstractType,
    pub domain: DomainType,
}

/// Unification of domains?
///
/// 'A U 'x -> 'x = 'A
///
/// 'x U 'y -> 'x = 'y
pub struct TypeUnifier<'linker_file_texts, 'errs> {
    pub template_type_names: FlatAlloc<String, TemplateIDMarker>,
    pub errors: &'errs ErrorCollector<'linker_file_texts>,
    pub type_substitutor: TypeSubstitutor<AbstractType, TypeVariableIDMarker>,
    pub domain_substitutor: TypeSubstitutor<DomainType, DomainVariableIDMarker>,
}

impl<'linker_file_texts, 'errs> TypeUnifier<'linker_file_texts, 'errs> {
    pub fn new(
        template_inputs: &TemplateInputs,
        errors: &'errs ErrorCollector<'linker_file_texts>,
        typing_alloc: TypingAllocator
    ) -> Self {
        Self {
            template_type_names: map_to_type_names(template_inputs),
            errors,
            type_substitutor: TypeSubstitutor::init(&typing_alloc.type_variable_alloc),
            domain_substitutor: TypeSubstitutor::init(&typing_alloc.domain_variable_alloc)
        }
    }

    pub fn alloc_typ_variable(&self) -> TypeVariableID {
        self.type_substitutor.alloc()
    }

    pub fn alloc_domain_variable(&self) -> DomainVariableID {
        self.domain_substitutor.alloc()
    }

    /// This should always be what happens first to a given variable. 
    /// 
    /// Therefore it should be impossible that one of the internal unifications ever fails
    pub fn unify_with_written_type(&mut self, instructions: &FlatAlloc<Instruction, FlatIDMarker>, wr_typ: &WrittenType, typ: &AbstractType) {
        match wr_typ {
            WrittenType::Error(_span) => {} // Already an error, don't unify
            WrittenType::Template(_span, uuid) => {
                self.type_substitutor.unify_must_succeed(&typ, &AbstractType::Template(*uuid));
            }
            WrittenType::Named(global_reference) => {
                if !global_reference.template_args.is_empty() {
                    todo!("Type template arguments")
                }

                self.type_substitutor.unify_must_succeed(&typ, &AbstractType::Named(global_reference.id));
            }
            WrittenType::Array(_span, array_content_and_size) =>  {
                let (arr_content, size_flat, array_bracket_span) = array_content_and_size.deref();

                let size_instr = instructions[*size_flat].unwrap_wire();
                
                // Of course, the user can still make mistakes
                if !size_instr.typ.domain.is_generative() {
                    self.errors.error(array_bracket_span.inner_span(), "The size of arrays must be a generative value. (gen)");
                }
                
                let arr_content_variable = AbstractType::Unknown(self.alloc_typ_variable());

                self.type_substitutor.unify_must_succeed(typ, &AbstractType::Array(Box::new(arr_content_variable.clone())));

                Self::unify_with_written_type(self, instructions, arr_content, &arr_content_variable);
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

    /// Unify the given full type [found] with the expected type [expected]. 
    pub fn typecheck_and_generative<const MUST_BE_GENERATIVE: bool>(
        &self,
        found: &FullType,
        expected: &AbstractType,
        span: Span,
        context: &'static str
    ) {
        self.type_substitutor.unify_report_error(&found.typ, &expected, span, context);

        if MUST_BE_GENERATIVE && found.domain != DomainType::Generative {
            self.errors
                .error(span, format!("A generative value is required in {context}"));
        }
    }

    pub fn unify_domains(&self, from_domain: &DomainType, to_domain: &DomainType, span: Span, context: &'static str) {
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

    pub fn typecheck_write_to(
        &self,
        found: &FullType,
        expected: &FullType,
        span: Span,
        context: &'static str
    ) {
        self.type_substitutor.unify_report_error(&found.typ, &expected.typ, span, context);
        self.unify_domains(&found.domain, &expected.domain, span, context);
    }

    pub fn finalize_type(&mut self, types: &ArenaAllocator<StructType, TypeUUIDMarker>, typ: &mut FullType, span: Span) {
        use super::type_inference::HindleyMilner;

        typ.domain.fully_substitute(&self.domain_substitutor).unwrap();
        if typ.typ.fully_substitute(&self.type_substitutor).is_err() {
            let typ_as_string = typ.typ.to_string(types, &self.template_type_names);
            self.errors.error(span, format!("Could not fully figure out the type of this object. {typ_as_string}"));
        }
    }
}
