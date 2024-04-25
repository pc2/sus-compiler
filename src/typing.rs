use std::ops::Deref;

use crate::{arena_alloc::ArenaAllocator, errors::{error_info, ErrorCollector}, file_position::{BracketSpan, Span, SpanFile}, flattening::{BinaryOperator, FlatID, UnaryOperator}, linker::{get_builtin_type, Linkable, Linker, NamedType, TypeUUID, TypeUUIDMarker}, value::Value};

// These are 
#[derive(Debug, Clone)]
pub enum WrittenType {
    Error(Span),
    Named(Span, TypeUUID),
    Array(Span, Box<(WrittenType, FlatID, BracketSpan)>)
}

impl WrittenType {
    pub fn for_each_located_type<F : FnMut(Option<TypeUUID>, Span)>(&self, f : &mut F) {
        match self {
            WrittenType::Error(span) => {f(None, *span)}
            WrittenType::Named(span, id) => {f(Some(*id), *span)}
            WrittenType::Array(_span, arr_box) => {
                let (arr, _idx, _br_span) = arr_box.deref();
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
                    let (arr_typ, _idx, _br_span) = arr_box.deref();
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

    pub fn to_type(&self) -> AbstractType {
        match self {
            WrittenType::Error(_) => AbstractType::Error,
            WrittenType::Named(_, id) => AbstractType::Named(*id),
            WrittenType::Array(_, arr_box) => {
                let (elem_typ, _arr_idx, _br_span) = arr_box.deref();
                AbstractType::Array(Box::new(elem_typ.to_type()))
            }
        }
    }

    pub fn for_each_generative_input<F : FnMut(FlatID)>(&self, f : &mut F) {
        match self {
            WrittenType::Error(_) | WrittenType::Named(_, _) => {}
            WrittenType::Array(_span, arr_box) => {
                f(arr_box.deref().1)
            }
        }
    }

    pub fn to_string(&self, linker_types : &ArenaAllocator<NamedType, TypeUUIDMarker>) -> String {
        match self {
            WrittenType::Error(_) => {
                "{error}".to_owned()
            }
            WrittenType::Named(_, id) => {
                linker_types[*id].get_full_name()
            }
            WrittenType::Array(_, sub) => sub.deref().0.to_string(linker_types) + "[]",
        }
    }
}

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
    pub fn to_string(&self, linker_types : &ArenaAllocator<NamedType, TypeUUIDMarker>) -> String {
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
pub const BOOL_CONCRETE_TYPE : ConcreteType = ConcreteType::Named(get_builtin_type("bool"));
pub const INT_CONCRETE_TYPE : ConcreteType = ConcreteType::Named(get_builtin_type("int"));

pub fn typecheck_unary_operator(op : UnaryOperator, input_typ : &AbstractType, span : Span, linker_types : &ArenaAllocator<NamedType, TypeUUIDMarker>, errors : &ErrorCollector) -> AbstractType {
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
        if let Some(arr_content_typ) = typecheck_is_array_indexer(input_typ, span, linker_types, errors) {
            typecheck(arr_content_typ, span, &gather_type, &format!("{op} input"), linker_types, None, errors);
        }
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

/// Panics on Type Errors that should have been caught by [UnparametrizedType]
/// 
/// TODO Add checks for array sizes being equal etc. 
pub fn typecheck_concrete_unary_operator(op : UnaryOperator, input_typ : &ConcreteType, _span : Span, _linker_types : &ArenaAllocator<NamedType, TypeUUIDMarker>, _errors : &ErrorCollector) -> ConcreteType {
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
/// Panics on Type Errors that should have been caught by [UnparametrizedType]
/// 
/// TODO Add checks for array sizes being equal etc. 
pub fn typecheck_concrete_binary_operator(op : BinaryOperator, left_typ : &ConcreteType, right_typ : &ConcreteType, _span : Span, _linker_types : &ArenaAllocator<NamedType, TypeUUIDMarker>, _errors : &ErrorCollector) -> ConcreteType {
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
pub fn typecheck(found : &AbstractType, span : Span, expected : &AbstractType, context : &str, linker_types : &ArenaAllocator<NamedType, TypeUUIDMarker>, declared_here : Option<SpanFile>, errors : &ErrorCollector) {
    if !type_compare(expected, found) {
        let expected_name = expected.to_string(linker_types);
        let found_name = found.to_string(linker_types);
        let decl_here_info = if let Some(declared_here) = declared_here {
            vec![error_info(declared_here.0, declared_here.1, "Declared here")]
        } else {
            Vec::new()
        };
        errors.error_with_info(span, format!("Typing Error: {context} expects a {expected_name} but was given a {found_name}"), decl_here_info);
        assert!(expected_name != found_name, "{expected_name} != {found_name}");
    }
}
pub fn typecheck_is_array_indexer<'a>(arr_type : &'a AbstractType, span : Span, linker_types : &ArenaAllocator<NamedType, TypeUUIDMarker>, errors : &ErrorCollector) -> Option<&'a AbstractType> {
    let AbstractType::Array(arr_element_type) = arr_type else {
        let arr_type_name = arr_type.to_string(linker_types);
        errors.error_basic(span, format!("Typing Error: Attempting to index into this, but it is not of array type, instead found a {arr_type_name}"));
        return None;
    };
    Some(&arr_element_type.deref())
}

#[derive(Debug,Clone,PartialEq,Eq)]
pub enum ConcreteType {
    Named(TypeUUID),
    Array(Box<(ConcreteType, u64)>)
}

impl Into<AbstractType> for &ConcreteType {
    fn into(self) -> AbstractType {
        match self {
            ConcreteType::Named(name) => {
                AbstractType::Named(*name)
            }
            ConcreteType::Array(arr) => {

                let (sub, _sz) = arr.deref();
                let concrete_sub : AbstractType = sub.into();
                AbstractType::Array(Box::new(concrete_sub))
            }
        }
    }
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
            ConcreteType::Named(name) => {
                linker_types[*name].get_full_name()
            }
            ConcreteType::Array(arr_box) => {
                let (elem_typ, arr_size) = arr_box.deref();
                format!("{}[{}]", elem_typ.to_string(linker_types), arr_size)
            }
        }
    }
    pub fn down_array(&self) -> &ConcreteType {
        let ConcreteType::Array(arr_box) = self else {unreachable!()};
        let (sub, _sz) = arr_box.deref();
        sub
    }
}
