use std::ops::Deref;

use num::BigInt;

use crate::{typing::{Type, ConcreteType}, linker::get_builtin_uuid, ast::Operator, tokenizer::kw};

#[derive(Debug,Clone,PartialEq,Eq)]
pub enum Value {
    Bool(bool),
    Integer(BigInt),
    Array(Box<[Value]>),
    Unset,
    Error
}

impl Value {
    pub fn get_type_of_constant(&self) -> Type {
        match self {
            Value::Bool(_) => Type::Named(get_builtin_uuid("bool")),
            Value::Integer(_) => Type::Named(get_builtin_uuid("int")),
            Value::Array(_b) => {
                unreachable!("Can't express arrays as constants (yet?)");
                /*let content_typ = if let Some(b_first) = b.first() {
                    b_first.get_type()
                } else {
                    Type::Invalid
                }
                Type::Array(Box::new((content_typ, b.len())))*/
            }
            Value::Unset => Type::Error,
            Value::Error => Type::Error,
        }
    }
    pub fn is_of_type(&self, typ : &ConcreteType) -> bool {
        match (self, typ) {
            (Self::Integer(_), ConcreteType::Named(name)) if *name == get_builtin_uuid("int") => true,
            (Self::Bool(_), ConcreteType::Named(name)) if *name == get_builtin_uuid("bool") => true,
            (Self::Array(arr_slice), ConcreteType::Array(arr_typ_box)) => {
                let (arr_content_typ, arr_size_typ) = arr_typ_box.deref();
                if arr_slice.len() != *arr_size_typ as usize {
                    return false;
                }
                for v in arr_slice.iter() {
                    if !v.is_of_type(arr_content_typ) {
                        return false;
                    }
                }
                true
            },
            (Self::Unset, _) => true,
            (Self::Error, _) => true,
            _other => false
        }
    }

    #[track_caller]
    pub fn extract_integer(&self) -> &BigInt {
        let Self::Integer(i) = self else {panic!("{:?} is not an integer!", self)};
        i
    }

    #[track_caller]
    pub fn extract_bool(&self) -> bool {
        let Self::Bool(b) = self else {panic!("{:?} is not a bool!", self)};
        *b
    }
}

pub fn compute_unary_op(op : Operator, v : &Value) -> Value {
    if *v == Value::Error {
        return Value::Error
    }
    match op.op_typ {
        typ if typ == kw("|") => {
            todo!("Array Values")
        }
        typ if typ == kw("&") => {
            todo!("Array Values")
        }
        typ if typ == kw("^") => {
            todo!("Array Values")
        }
        typ if typ == kw("-") => {
            let Value::Integer(v) = v else {panic!()};
            Value::Integer(-v)
        }
        typ if typ == kw("+") => {
            todo!("Array Values")
        }
        typ if typ == kw("*") => {
            todo!("Array Values")
        }
        typ if typ == kw("!") => {
            let Value::Bool(b) = v else {panic!()};
            Value::Bool(!*b)
        }
        _other => unreachable!()
    }
}

pub fn compute_binary_op(left : &Value, op : Operator, right : &Value) -> Value {
    if *left == Value::Error || *right == Value::Error {
        return Value::Error
    }
    match op.op_typ {
        typ if typ == kw("<=") => Value::Bool(left.extract_integer() <= right.extract_integer()),
        typ if typ == kw(">=") => Value::Bool(left.extract_integer() >= right.extract_integer()),
        typ if typ == kw("<")  => Value::Bool(left.extract_integer() < right.extract_integer()),
        typ if typ == kw(">")  => Value::Bool(left.extract_integer() > right.extract_integer()),
        typ if typ == kw("==") => Value::Bool(left == right),
        typ if typ == kw("!=") => Value::Bool(left != right),
        typ if typ == kw("<<") => todo!(), // Still a bit iffy about shift operator inclusion
        typ if typ == kw(">>") => todo!(),
        typ if typ == kw("+")  => Value::Integer(left.extract_integer() + right.extract_integer()),
        typ if typ == kw("-")  => Value::Integer(left.extract_integer() - right.extract_integer()),
        typ if typ == kw("*")  => Value::Integer(left.extract_integer() * right.extract_integer()),
        typ if typ == kw("/")  => Value::Integer(left.extract_integer() / right.extract_integer()),
        typ if typ == kw("%")  => Value::Integer(left.extract_integer() % right.extract_integer()),
        typ if typ == kw("&")  => Value::Bool(left.extract_bool() & right.extract_bool()),
        typ if typ == kw("|")  => Value::Bool(left.extract_bool() & right.extract_bool()),
        typ if typ == kw("^")  => Value::Bool(left.extract_bool() & right.extract_bool()),
        _other => unreachable!()
    }
}
