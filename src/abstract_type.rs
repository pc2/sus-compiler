use std::ops::{Deref, Index};

use crate::{errors::ErrorCollector, file_position::{Span, SpanFile}, flattening::{BinaryOperator, UnaryOperator}, linker::{get_builtin_type, Linkable, NamedType, TypeUUID}};

/// This contains only the information that can be easily type-checked. 
/// 
/// Its most important components are the names and structure of types. 
/// 
/// What isn't included are the parameters of types. So Array Sizes for example. 
#[derive(Debug, Clone)]
pub enum AbstractType {
    Error,
    Unknown,
    Named(TypeUUID),
    Array(Box<AbstractType>)
}

impl AbstractType {
    pub fn to_string<TypVec : Index<TypeUUID, Output = NamedType>>(&self, linker_types : &TypVec) -> String {
        match self {
            AbstractType::Error => {
                "{error}".to_owned()
            }
            AbstractType::Unknown => {
                "{unknown}".to_owned()
            }
            AbstractType::Named(id) => {
                linker_types[*id].get_full_name()
            }
            AbstractType::Array(sub) => sub.deref().to_string(linker_types) + "[]",
        }
    }
    pub fn contains_error_or_unknown<const CHECK_ERROR : bool, const CHECK_UNKNOWN : bool>(&self) -> bool {
        match self {
            AbstractType::Error => CHECK_ERROR,
            AbstractType::Unknown => CHECK_UNKNOWN,
            AbstractType::Named(_id) => false,
            AbstractType::Array(arr_box) => {
                arr_box.deref().contains_error_or_unknown::<CHECK_ERROR, CHECK_UNKNOWN>()
            }
        }
    }
}


pub const BOOL_TYPE : AbstractType = AbstractType::Named(get_builtin_type("bool"));
pub const INT_TYPE : AbstractType = AbstractType::Named(get_builtin_type("int"));
const ERROR_TYPE : AbstractType = AbstractType::Error;

pub fn typecheck_unary_operator<TypVec : Index<TypeUUID, Output = NamedType>>(op : UnaryOperator, input_typ : &AbstractType, span : Span, linker_types : &TypVec, errors : &ErrorCollector) -> AbstractType {
    if op == UnaryOperator::Not {
        typecheck(input_typ, span, &BOOL_TYPE, "! input", linker_types, None, errors);
        BOOL_TYPE
    } else if op == UnaryOperator::Negate {
        typecheck(input_typ, span, &INT_TYPE, "- input", linker_types, None, errors);
        INT_TYPE
    } else {
        let gather_type = match op {
            UnaryOperator::And => BOOL_TYPE,
            UnaryOperator::Or => BOOL_TYPE,
            UnaryOperator::Xor => BOOL_TYPE,
            UnaryOperator::Sum => INT_TYPE,
            UnaryOperator::Product => INT_TYPE,
            _ => unreachable!()
        };
        let arr_content_typ = typecheck_is_array_indexer(input_typ, span, linker_types, errors);
        typecheck(arr_content_typ, span, &gather_type, &format!("{op} input"), linker_types, None, errors);

        gather_type
    }
}
pub fn get_binary_operator_types(op : BinaryOperator) -> ((AbstractType, AbstractType), AbstractType) {
    match op {
        BinaryOperator::And => ((BOOL_TYPE, BOOL_TYPE), BOOL_TYPE),
        BinaryOperator::Or => ((BOOL_TYPE, BOOL_TYPE), BOOL_TYPE),
        BinaryOperator::Xor => ((BOOL_TYPE, BOOL_TYPE), BOOL_TYPE),
        BinaryOperator::Add => ((INT_TYPE, INT_TYPE), INT_TYPE),
        BinaryOperator::Subtract => ((INT_TYPE, INT_TYPE), INT_TYPE),
        BinaryOperator::Multiply => ((INT_TYPE, INT_TYPE), INT_TYPE),
        BinaryOperator::Divide => ((INT_TYPE, INT_TYPE), INT_TYPE),
        BinaryOperator::Modulo => ((INT_TYPE, INT_TYPE), INT_TYPE),
        BinaryOperator::Equals => ((INT_TYPE, INT_TYPE), BOOL_TYPE),
        BinaryOperator::NotEquals => ((INT_TYPE, INT_TYPE), BOOL_TYPE),
        BinaryOperator::GreaterEq => ((INT_TYPE, INT_TYPE), BOOL_TYPE),
        BinaryOperator::Greater => ((INT_TYPE, INT_TYPE), BOOL_TYPE),
        BinaryOperator::LesserEq => ((INT_TYPE, INT_TYPE), BOOL_TYPE),
        BinaryOperator::Lesser => ((INT_TYPE, INT_TYPE), BOOL_TYPE),
    }
}

fn type_compare(expected : &AbstractType, found : &AbstractType) -> bool {
    match (expected, found) {
        (AbstractType::Named(exp), AbstractType::Named(fnd)) => exp == fnd,
        (AbstractType::Array(exp), AbstractType::Array(fnd)) => {
            type_compare(&exp.deref(), &fnd.deref())
        }
        (AbstractType::Error, _) | (_, AbstractType::Error) => true, // Just assume correct, because the other side has an error
        (AbstractType::Unknown, _) | (_, AbstractType::Unknown) => todo!("Type Unification"),
        _ => false,
    }
}
pub fn typecheck<TypVec : Index<TypeUUID, Output = NamedType>>(found : &AbstractType, span : Span, expected : &AbstractType, context : &str, linker_types : &TypVec, declared_here : Option<SpanFile>, errors : &ErrorCollector) {
    if !type_compare(expected, found) {
        let expected_name = expected.to_string(linker_types);
        let found_name = found.to_string(linker_types);
        let err_ref = errors.error(span, format!("Typing Error: {context} expects a {expected_name} but was given a {found_name}"));
        if let Some(declared_here) = declared_here {
            err_ref.info(declared_here, "Declared here");
        }
        assert!(expected_name != found_name, "{expected_name} != {found_name}");
    }
}

pub fn typecheck_is_array_indexer<'a, TypVec : Index<TypeUUID, Output = NamedType>>(arr_type : &'a AbstractType, span : Span, linker_types : &TypVec, errors : &ErrorCollector) -> &'a AbstractType {
    let AbstractType::Array(arr_element_type) = arr_type else {
        let arr_type_name = arr_type.to_string(linker_types);
        errors.error(span, format!("Typing Error: Attempting to index into this, but it is not of array type, instead found a {arr_type_name}"));
        return &ERROR_TYPE;
    };
    &arr_element_type.deref()
}
