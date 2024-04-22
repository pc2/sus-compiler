use std::ops::Deref;

use num::BigInt;

use crate::{flattening::{BinaryOperator, UnaryOperator}, typing::{ConcreteType, AbstractType, BOOL_CONCRETE_TYPE, BOOL_TYPE, INT_CONCRETE_TYPE, INT_TYPE}};

#[derive(Debug,Clone,PartialEq,Eq)]
pub enum Value {
    Bool(bool),
    Integer(BigInt),
    Array(Box<[Value]>),
    Unset,
    Error
}

impl Value {
    pub fn get_type_of_constant(&self) -> AbstractType {
        match self {
            Value::Bool(_) => BOOL_TYPE,
            Value::Integer(_) => INT_TYPE,
            Value::Array(_b) => {
                unreachable!("Can't express arrays as constants (yet?)");
                /*let content_typ = if let Some(b_first) = b.first() {
                    b_first.get_type()
                } else {
                    Type::Invalid
                }
                Type::Array(Box::new((content_typ, b.len())))*/
            }
            Value::Unset => AbstractType::Error,
            Value::Error => AbstractType::Error,
        }
    }
    pub fn get_concrete_type_of_constant(&self) -> ConcreteType {
        match self {
            Value::Bool(_) => BOOL_CONCRETE_TYPE,
            Value::Integer(_) => INT_CONCRETE_TYPE,
            Value::Array(_b) => {
                unreachable!("Can't express arrays as constants (yet?)");
                /*let content_typ = if let Some(b_first) = b.first() {
                    b_first.get_type()
                } else {
                    Type::Invalid
                }
                Type::Array(Box::new((content_typ, b.len())))*/
            }
            Value::Unset | Value::Error => unreachable!()
        }
    }
    pub fn is_of_type(&self, typ : &ConcreteType) -> bool {
        match (self, typ) {
            (Self::Integer(_), typ) if *typ == INT_CONCRETE_TYPE => true,
            (Self::Bool(_), typ) if *typ == BOOL_CONCRETE_TYPE => true,
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

    pub fn is_valid(&self) -> bool {
        match self {
            Value::Unset | Value::Error => false,
            _other => true
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

    pub fn to_string(&self) -> String {
        match self {
            Value::Bool(b) => if *b {"1'b1"} else {"1'b0"}.to_owned(),
            Value::Integer(v) => v.to_string(),
            Value::Array(arr_box) => {
                let mut result = "[".to_owned();
                for v in arr_box.iter() {
                    result.push_str(&v.to_string());
                    result.push_str(", ");
                }
                result.push(']');
                result
            }
            Value::Unset => "Value::Unset".to_owned(),
            Value::Error => "Value::Error".to_owned(),
        }
    }
}

pub fn compute_unary_op(op : UnaryOperator, v : &Value) -> Value {
    if *v == Value::Error {
        return Value::Error
    }
    match op {
        UnaryOperator::Or => {
            todo!("Array Values")
        }
        UnaryOperator::And => {
            todo!("Array Values")
        }
        UnaryOperator::Xor => {
            todo!("Array Values")
        }
        UnaryOperator::Not => {
            let Value::Bool(b) = v else {panic!()};
            Value::Bool(!*b)
        }
        UnaryOperator::Sum => {
            todo!("Array Values")
        }
        UnaryOperator::Product => {
            todo!("Array Values")
        }
        UnaryOperator::Negate => {
            let Value::Integer(v) = v else {panic!()};
            Value::Integer(-v)
        }
    }
}

pub fn compute_binary_op(left : &Value, op : BinaryOperator, right : &Value) -> Value {
    if *left == Value::Error || *right == Value::Error {
        return Value::Error
    }
    match op {
        BinaryOperator::Equals => Value::Bool(left == right),
        BinaryOperator::NotEquals => Value::Bool(left != right),
        BinaryOperator::GreaterEq => Value::Bool(left.extract_integer() >= right.extract_integer()),
        BinaryOperator::Greater => Value::Bool(left.extract_integer() > right.extract_integer()),
        BinaryOperator::LesserEq => Value::Bool(left.extract_integer() <= right.extract_integer()),
        BinaryOperator::Lesser => Value::Bool(left.extract_integer() < right.extract_integer()),
        BinaryOperator::Add => Value::Integer(left.extract_integer() + right.extract_integer()),
        BinaryOperator::Subtract => Value::Integer(left.extract_integer() - right.extract_integer()),
        BinaryOperator::Multiply => Value::Integer(left.extract_integer() * right.extract_integer()),
        BinaryOperator::Divide => Value::Integer(left.extract_integer() / right.extract_integer()),
        BinaryOperator::Modulo => Value::Integer(left.extract_integer() % right.extract_integer()),
        BinaryOperator::And => Value::Bool(left.extract_bool() & right.extract_bool()),
        BinaryOperator::Or => Value::Bool(left.extract_bool() & right.extract_bool()),
        BinaryOperator::Xor => Value::Bool(left.extract_bool() & right.extract_bool()),
        //BinaryOperator::ShiftLeft => todo!(), // Still a bit iffy about shift operator inclusion
        //BinaryOperator::ShiftRight => todo!()
    }
}
