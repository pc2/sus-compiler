use std::ops::Deref;

use crate::{arena_alloc::ArenaAllocator, ast::Operator, errors::ErrorCollector, file_position::Span, flattening::FlatID, linker::{get_builtin_type, Linkable, Linker, NamedType, TypeUUID, TypeUUIDMarker}, tokenizer::kw, value::Value};

// These are 
#[derive(Debug, Clone)]
pub enum WrittenType {
    Error(Span),
    Named(Span, TypeUUID),
    Array(Span, Box<(WrittenType, FlatID)>)
}

impl WrittenType {
    pub fn for_each_located_type<F : FnMut(Option<TypeUUID>, Span)>(&self, f : &mut F) {
        match self {
            WrittenType::Error(span) => {f(None, *span)}
            WrittenType::Named(span, id) => {f(Some(*id), *span)}
            WrittenType::Array(_span, arr_box) => {
                let (arr, _idx) = arr_box.deref();
                arr.for_each_located_type(f);
            }
        }
    }

    pub fn get_span(&self) -> Span {
        match self {
            WrittenType::Error(span) | WrittenType::Named(span, _) | WrittenType::Array(span, _) => *span
        }
    }

    pub fn get_deepest_selected(&self, position : usize) -> Option<&WrittenType> {
        let span = self.get_span();
        if span.contains_pos(position) {
            match self {
                WrittenType::Error(_span) | WrittenType::Named(_span, _) => {}
                WrittenType::Array(_span, arr_box) => {
                    let (arr_typ, _idx) = arr_box.deref();
                    let sub = arr_typ.get_deepest_selected(position);
                    if sub.is_some() {
                        return sub;
                    }
                }
            }
            Some(self)
        } else {
            None
        }
    }

    pub fn to_type(&self) -> Type {
        match self {
            WrittenType::Error(_) => Type::Error,
            WrittenType::Named(_, id) => Type::Named(*id),
            WrittenType::Array(_, arr_box) => {
                let (elem_typ, arr_idx) = arr_box.deref();
                Type::Array(Box::new((elem_typ.to_type(), *arr_idx)))
            }
        }
    }
}

// Types contain everything that cannot be expressed at runtime
#[derive(Debug, Clone)]
pub enum Type {
    Error,
    Unknown,
    Named(TypeUUID),
    /*Contains a wireID pointing to a constant expression for the array size, 
    but doesn't actually take size into account for type checking as that would
    make type checking too difficult. Instead delay until proper instantiation
    to check array sizes, as then we have concrete numbers*/
    Array(Box<(Type, FlatID)>)
}

impl Type {
    pub fn to_string(&self, linker_types : &ArenaAllocator<NamedType, TypeUUIDMarker>) -> String {
        match self {
            Type::Error => {
                "{error}".to_owned()
            }
            Type::Unknown => {
                "{unknown}".to_owned()
            }
            Type::Named(id) => {
                linker_types[*id].get_full_name()
            }
            Type::Array(sub) => sub.deref().0.to_string(linker_types) + "[]",
        }
    }
    pub fn for_each_generative_input<F : FnMut(FlatID)>(&self, f : &mut F) {
        match self {
            Type::Error | Type::Unknown | Type::Named(_) => {}
            Type::Array(arr_box) => {
                f(arr_box.deref().1)
            }
        }
    }
    pub fn contains_error_or_unknown<const CHECK_ERROR : bool, const CHECK_UNKNOWN : bool>(&self) -> bool {
        match self {
            Type::Error => CHECK_ERROR,
            Type::Unknown => CHECK_UNKNOWN,
            Type::Named(_id) => false,
            Type::Array(arr_box) => {
                arr_box.deref().0.contains_error_or_unknown::<CHECK_ERROR, CHECK_UNKNOWN>()
            }
        }
    }
}


pub const BOOL_TYPE : Type = Type::Named(get_builtin_type("bool"));
pub const INT_TYPE : Type = Type::Named(get_builtin_type("int"));
pub const BOOL_CONCRETE_TYPE : ConcreteType = ConcreteType::Named(get_builtin_type("bool"));
pub const INT_CONCRETE_TYPE : ConcreteType = ConcreteType::Named(get_builtin_type("int"));

pub fn typecheck_unary_operator(op : Operator, input_typ : &Type, span : Span, linker_types : &ArenaAllocator<NamedType, TypeUUIDMarker>, errors : &ErrorCollector) -> Type {
    if op.op_typ == kw("!") {
        typecheck(input_typ, span, &BOOL_TYPE, "! input", linker_types, errors);
        BOOL_TYPE
    } else if op.op_typ == kw("-") {
        typecheck(input_typ, span, &INT_TYPE, "- input", linker_types, errors);
        INT_TYPE
    } else {
        let gather_type = match op.op_typ {
            x if x == kw("&") => BOOL_TYPE,
            x if x == kw("|") => BOOL_TYPE,
            x if x == kw("^") => BOOL_TYPE,
            x if x == kw("+") => INT_TYPE,
            x if x == kw("*") => INT_TYPE,
            _ => unreachable!()
        };
        if let Some(arr_content_typ) = typecheck_is_array_indexer(input_typ, span, linker_types, errors) {
            typecheck(arr_content_typ, span, &gather_type, &format!("{op} input"), linker_types, errors);
        }
        gather_type
    }
}
pub fn get_binary_operator_types(op : Operator) -> ((Type, Type), Type) {
    match op.op_typ {
        x if x == kw("&") => ((BOOL_TYPE, BOOL_TYPE), BOOL_TYPE),
        x if x == kw("|") => ((BOOL_TYPE, BOOL_TYPE), BOOL_TYPE),
        x if x == kw("^") => ((BOOL_TYPE, BOOL_TYPE), BOOL_TYPE),
        x if x == kw("+") => ((INT_TYPE, INT_TYPE), INT_TYPE),
        x if x == kw("-") => ((INT_TYPE, INT_TYPE), INT_TYPE),
        x if x == kw("*") => ((INT_TYPE, INT_TYPE), INT_TYPE),
        x if x == kw("/") => ((INT_TYPE, INT_TYPE), INT_TYPE),
        x if x == kw("%") => ((INT_TYPE, INT_TYPE), INT_TYPE),
        x if x == kw("==") => ((INT_TYPE, INT_TYPE), BOOL_TYPE),
        x if x == kw("!=") => ((INT_TYPE, INT_TYPE), BOOL_TYPE),
        x if x == kw(">=") => ((INT_TYPE, INT_TYPE), BOOL_TYPE),
        x if x == kw("<=") => ((INT_TYPE, INT_TYPE), BOOL_TYPE),
        x if x == kw(">") => ((INT_TYPE, INT_TYPE), BOOL_TYPE),
        x if x == kw("<") => ((INT_TYPE, INT_TYPE), BOOL_TYPE),
        _ => unreachable!()
    }
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
pub fn typecheck(found : &Type, span : Span, expected : &Type, context : &str, linker_types : &ArenaAllocator<NamedType, TypeUUIDMarker>, errors : &ErrorCollector) {
    if !type_compare(expected, found) {
        let expected_name = expected.to_string(linker_types);
        let found_name = found.to_string(linker_types);
        errors.error_basic(span, format!("Typing Error: {context} expects a {expected_name} but was given a {found_name}"));
        assert!(expected_name != found_name, "{expected_name} != {found_name}");
    }
}
pub fn typecheck_is_array_indexer<'a>(arr_type : &'a Type, span : Span, linker_types : &ArenaAllocator<NamedType, TypeUUIDMarker>, errors : &ErrorCollector) -> Option<&'a Type> {
    let Type::Array(arr_element_type) = arr_type else {
        let arr_type_name = arr_type.to_string(linker_types);
        errors.error_basic(span, format!("Typing Error: Attempting to index into this, but it is not of array type, instead found a {arr_type_name}"));
        return None;
    };
    Some(&arr_element_type.deref().0)
}

#[derive(Debug,Clone,PartialEq,Eq)]
pub enum ConcreteType {
    Named(TypeUUID),
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
    pub fn to_string(&self, linker_types : &ArenaAllocator<NamedType, TypeUUIDMarker>) -> String {
        match self {
            ConcreteType::Named(id) => {
                linker_types[*id].get_full_name()
            }
            ConcreteType::Array(sub) => {
                let (elem_typ, arr_size) = sub.deref();
                format!("{}[{}]", elem_typ.to_string(linker_types), arr_size)
            }
        }
    }
}
