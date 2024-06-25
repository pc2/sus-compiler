
use std::ops::{Deref, Index};

use crate::{abstract_type::AbstractType, errors::ErrorCollector, file_position::Span, flattening::{BinaryOperator, UnaryOperator}, linker::{get_builtin_type, NamedType, TypeUUID}, value::Value};

pub const BOOL_CONCRETE_TYPE : ConcreteType = ConcreteType::Named(get_builtin_type("bool"));
pub const INT_CONCRETE_TYPE : ConcreteType = ConcreteType::Named(get_builtin_type("int"));

#[derive(Debug,Clone,PartialEq,Eq)]
pub enum ConcreteType {
    Named(TypeUUID),
    Value(Value),
    Array(Box<(ConcreteType, ConcreteType)>),
    Unknown,
    Error
}

impl Into<AbstractType> for &ConcreteType {
    fn into(self) -> AbstractType {
        match self {
            ConcreteType::Named(name) => {
                AbstractType::Named(*name)
            }
            ConcreteType::Value(_) => {
                unreachable!("Turning a ConcreteType::Value into an AbstractType");
            }
            ConcreteType::Array(arr) => {
                let (sub, _sz) = arr.deref();
                let concrete_sub : AbstractType = sub.into();
                AbstractType::Array(Box::new(concrete_sub))
            }
            ConcreteType::Unknown => AbstractType::Unknown,
            ConcreteType::Error => AbstractType::Error
        }
    }
}


/// Panics on Type Errors that should have been caught by [AbstractType]
/// 
/// TODO Add checks for array sizes being equal etc. 
pub fn get_unary_operator_expected_output(op : UnaryOperator, input_typ : &ConcreteType) -> ConcreteType {
    let gather_type = match op {
        UnaryOperator::Not => {
            assert_eq!(*input_typ, BOOL_CONCRETE_TYPE);
            return BOOL_CONCRETE_TYPE
        }
        UnaryOperator::Negate => {
            assert_eq!(*input_typ, INT_CONCRETE_TYPE);
            return INT_CONCRETE_TYPE
        }
        UnaryOperator::And => BOOL_CONCRETE_TYPE,
        UnaryOperator::Or => BOOL_CONCRETE_TYPE,
        UnaryOperator::Xor => BOOL_CONCRETE_TYPE,
        UnaryOperator::Sum => INT_CONCRETE_TYPE,
        UnaryOperator::Product => INT_CONCRETE_TYPE
    };
    assert_eq!(input_typ.down_array(), &gather_type);
    gather_type
}

/// Panics on Type Errors that should have been caught by [AbstractType]
/// 
/// TODO Add checks for array sizes being equal etc. 
pub fn get_binary_operator_expected_output(op : BinaryOperator, left_typ : &ConcreteType, right_typ : &ConcreteType) -> ConcreteType {
    let ((in_left, in_right), out) = match op {
        BinaryOperator::And => ((BOOL_CONCRETE_TYPE, BOOL_CONCRETE_TYPE), BOOL_CONCRETE_TYPE),
        BinaryOperator::Or => ((BOOL_CONCRETE_TYPE, BOOL_CONCRETE_TYPE), BOOL_CONCRETE_TYPE),
        BinaryOperator::Xor => ((BOOL_CONCRETE_TYPE, BOOL_CONCRETE_TYPE), BOOL_CONCRETE_TYPE),
        BinaryOperator::Add => ((INT_CONCRETE_TYPE, INT_CONCRETE_TYPE), INT_CONCRETE_TYPE),
        BinaryOperator::Subtract => ((INT_CONCRETE_TYPE, INT_CONCRETE_TYPE), INT_CONCRETE_TYPE),
        BinaryOperator::Multiply => ((INT_CONCRETE_TYPE, INT_CONCRETE_TYPE), INT_CONCRETE_TYPE),
        BinaryOperator::Divide => ((INT_CONCRETE_TYPE, INT_CONCRETE_TYPE), INT_CONCRETE_TYPE),
        BinaryOperator::Modulo => ((INT_CONCRETE_TYPE, INT_CONCRETE_TYPE), INT_CONCRETE_TYPE),
        BinaryOperator::Equals => ((INT_CONCRETE_TYPE, INT_CONCRETE_TYPE), BOOL_CONCRETE_TYPE),
        BinaryOperator::NotEquals => ((INT_CONCRETE_TYPE, INT_CONCRETE_TYPE), BOOL_CONCRETE_TYPE),
        BinaryOperator::GreaterEq => ((INT_CONCRETE_TYPE, INT_CONCRETE_TYPE), BOOL_CONCRETE_TYPE),
        BinaryOperator::Greater => ((INT_CONCRETE_TYPE, INT_CONCRETE_TYPE), BOOL_CONCRETE_TYPE),
        BinaryOperator::LesserEq => ((INT_CONCRETE_TYPE, INT_CONCRETE_TYPE), BOOL_CONCRETE_TYPE),
        BinaryOperator::Lesser => ((INT_CONCRETE_TYPE, INT_CONCRETE_TYPE), BOOL_CONCRETE_TYPE),
    };

    assert_eq!(*left_typ, in_left);
    assert_eq!(*right_typ, in_right);

    out
}    

impl ConcreteType {
    #[track_caller]
    pub fn unwrap_value(&self) -> &Value {
        let ConcreteType::Value(v) = self else {unreachable!("unwrap_value")};
        v
    }
    pub fn down_array(&self) -> &ConcreteType {
        let ConcreteType::Array(arr_box) = self else {unreachable!("Must be an array!")};
        let (sub, _sz) = arr_box.deref();
        sub
    }

    pub fn type_compare(&self, found : &ConcreteType) -> bool {
        match (self, found) {
            (ConcreteType::Named(exp), ConcreteType::Named(fnd)) => exp == fnd,
            (ConcreteType::Array(exp), ConcreteType::Array(fnd)) => {
                let (target_arr_typ, target_arr_size) = exp.deref();
                let (found_arr_typ, found_arr_size) = fnd.deref();
                target_arr_typ.type_compare(found_arr_typ) && target_arr_size.type_compare(found_arr_size)
            }
            (ConcreteType::Value(lv), ConcreteType::Value(rv)) => lv == rv,
            (ConcreteType::Error, _) | (_, ConcreteType::Error) => true, // Just assume correct, because the other side has an error
            (ConcreteType::Unknown, _) | (_, ConcreteType::Unknown) => todo!("Type Unification {self:?} {found:?}"),
            _ => false,
        }
    }
    pub fn check_type<TypVec : Index<TypeUUID, Output = NamedType>>(&self, source_type : &ConcreteType, span : Span, linker_types : &TypVec, errors : &ErrorCollector) {
        if !self.type_compare(source_type) {
            errors.error(span, format!("Concrete Type Error! Expected {} but found {}", self.to_string(linker_types), source_type.to_string(linker_types)));
        }
    }

    pub fn check_or_update_type<TypVec : Index<TypeUUID, Output = NamedType>>(&mut self, source_type : &ConcreteType, span : Span, linker_types : &TypVec, errors : &ErrorCollector) {
        if *self == ConcreteType::Unknown {
            *self = source_type.clone();
        } else {
            self.check_type(source_type, span, linker_types, errors);
        }
    }

    pub fn typecheck_concrete_unary_operator<TypVec : Index<TypeUUID, Output = NamedType>>(&mut self, op : UnaryOperator, input_typ : &ConcreteType, span : Span, linker_types : &TypVec, errors : &ErrorCollector) {
        let expected = get_unary_operator_expected_output(op, input_typ);

        self.check_or_update_type(&expected, span, linker_types, errors);
    }
    pub fn typecheck_concrete_binary_operator<TypVec : Index<TypeUUID, Output = NamedType>>(&mut self, op : BinaryOperator, left_typ : &ConcreteType, right_typ : &ConcreteType, span : Span, linker_types : &TypVec, errors : &ErrorCollector) {
        let expected = get_binary_operator_expected_output(op, left_typ, right_typ);

        self.check_or_update_type(&expected, span, linker_types, errors);
    }
}
