use std::ops::Deref;

use crate::{ast::{Operator, Span}, linker::{get_builtin_uuid, NamedUUID, Linker, Linkable}, tokenizer::kw, flattening::FlatID, errors::ErrorCollector, value::Value};

// Types contain everything that cannot be expressed at runtime
#[derive(Debug, Clone)]
pub enum Type {
    Error,
    Unknown,
    Named(NamedUUID),
    /*Contains a wireID pointing to a constant expression for the array size, 
    but doesn't actually take size into account for type checking as that would
    make type checking too difficult. Instead delay until proper instantiation
    to check array sizes, as then we have concrete numbers*/
    Array(Box<(Type, FlatID)>)
}

impl Type {
    pub fn to_string(&self, linker : &Linker) -> String {
        match self {
            Type::Error => {
                "{error}".to_owned()
            }
            Type::Unknown => {
                "{unknown}".to_owned()
            }
            Type::Named(n) => {
                linker.links[*n].get_full_name()
            }
            Type::Array(sub) => sub.deref().0.to_string(linker) + "[]",
        }
    }
    pub fn get_root(&self) -> Option<NamedUUID> {
        match self {
            Type::Error => None,
            Type::Unknown => None,
            Type::Named(name) => Some(*name),
            Type::Array(sub) => sub.0.get_root(),
        }
    }
    pub fn for_each_generative_input<F : FnMut(FlatID)>(&self, f : &mut F) {
        match self {
            Type::Error => {}
            Type::Unknown => {}
            Type::Named(_) => {}
            Type::Array(arr_box) => {
                f(arr_box.deref().1)
            }
        }
    }
    pub fn contains_error_or_unknown<const CHECK_ERROR : bool, const CHECK_UNKNOWN : bool>(&self) -> bool {
        match self {
            Type::Error => CHECK_ERROR,
            Type::Unknown => CHECK_UNKNOWN,
            Type::Named(_) => false,
            Type::Array(arr_box) => {
                arr_box.deref().0.contains_error_or_unknown::<CHECK_ERROR, CHECK_UNKNOWN>()
            }
        }
    }
}

pub fn typecheck_unary_operator(op : Operator, input_typ : &Type, span : Span, linker : &Linker, errors : &ErrorCollector) -> Type {
    const BOOL : Type = Type::Named(get_builtin_uuid("bool"));
    const INT : Type = Type::Named(get_builtin_uuid("int"));
    
    if op.op_typ == kw("!") {
        typecheck(input_typ, span, &BOOL, "! input", linker, errors);
        BOOL
    } else if op.op_typ == kw("-") {
        typecheck(input_typ, span, &INT, "- input", linker, errors);
        INT
    } else {
        let gather_type = match op.op_typ {
            x if x == kw("&") => BOOL,
            x if x == kw("|") => BOOL,
            x if x == kw("^") => BOOL,
            x if x == kw("+") => INT,
            x if x == kw("*") => INT,
            _ => unreachable!()
        };
        if let Some(arr_content_typ) = typecheck_is_array_indexer(input_typ, span, linker, errors) {
            typecheck(arr_content_typ, span, &gather_type, &format!("{op} input"), linker, errors);
        }
        gather_type
    }
}
pub fn get_binary_operator_types(op : Operator) -> ((Type, Type), Type) {
    const BOOL : NamedUUID = get_builtin_uuid("bool");
    const INT : NamedUUID = get_builtin_uuid("int");
    
    let (a, b, o) = match op.op_typ {
        x if x == kw("&") => (BOOL, BOOL, BOOL),
        x if x == kw("|") => (BOOL, BOOL, BOOL),
        x if x == kw("^") => (BOOL, BOOL, BOOL),
        x if x == kw("+") => (INT, INT, INT),
        x if x == kw("-") => (INT, INT, INT),
        x if x == kw("*") => (INT, INT, INT),
        x if x == kw("/") => (INT, INT, INT),
        x if x == kw("%") => (INT, INT, INT),
        x if x == kw("==") => (INT, INT, BOOL),
        x if x == kw("!=") => (INT, INT, BOOL),
        x if x == kw(">=") => (INT, INT, BOOL),
        x if x == kw("<=") => (INT, INT, BOOL),
        x if x == kw(">") => (INT, INT, BOOL),
        x if x == kw("<") => (INT, INT, BOOL),
        _ => unreachable!()
    };
    ((Type::Named(a), Type::Named(b)), Type::Named(o))
}

fn type_compare(expected : &Type, found : &Type) -> bool {
    match (expected, found) {
        (Type::Named(exp), Type::Named(fnd)) => exp == fnd,
        (Type::Array(exp), Type::Array(fnd)) => {
            type_compare(&exp.deref().0, &fnd.deref().0)
        }
        (Type::Error, _) | (_, Type::Error) => true, // Just assume correct, because the other side has an error
        (Type::Unknown, _) | (_, Type::Unknown) => todo!("Type Unification"),
        _ => false,
    }
}
pub fn typecheck(found : &Type, span : Span, expected : &Type, context : &str, linker : &Linker, errors : &ErrorCollector) {
    if !type_compare(expected, found) {
        let expected_name = expected.to_string(linker);
        let found_name = found.to_string(linker);
        errors.error_basic(span, format!("Typing Error: {context} expects a {expected_name} but was given a {found_name}"));
        assert!(expected_name != found_name, "{expected_name} != {found_name}");
    }
}
pub fn typecheck_is_array_indexer<'a>(arr_type : &'a Type, span : Span, linker : &Linker, errors : &ErrorCollector) -> Option<&'a Type> {
    let Type::Array(arr_element_type) = arr_type else {
        let arr_type_name = arr_type.to_string(linker);
        errors.error_basic(span, format!("Typing Error: Attempting to index into this, but it is not of array type, instead found a {arr_type_name}"));
        return None;
    };
    Some(&arr_element_type.deref().0)
}

#[derive(Debug,Clone,PartialEq,Eq)]
pub enum ConcreteType {
    Named(NamedUUID),
    Array(Box<(ConcreteType, u64)>)
}

impl ConcreteType {
    pub fn get_initial_val(&self, linker : &Linker) -> Value {
        match self {
            ConcreteType::Named(_name) => {
                Value::Unset
            }
            ConcreteType::Array(arr) => {
                let (arr_typ, arr_size) = arr.deref();
                let mut arr = Vec::new();
                if *arr_size > 0 {
                    let content_typ = arr_typ.get_initial_val(linker);
                    arr.resize(*arr_size as usize, content_typ);
                }
                Value::Array(arr.into_boxed_slice())
            }
        }
    }
}