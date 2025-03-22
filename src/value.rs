use std::ops::Deref;

use num::BigInt;

use crate::flattening::{BinaryOperator, UnaryOperator};

use crate::typing::{
    concrete_type::{ConcreteType, BOOL_CONCRETE_TYPE, INT_CONCRETE_TYPE},
    type_inference::{ConcreteTypeVariableIDMarker, TypeSubstitutor},
};

/// Top type for any kind of compiletime value while executing.
///
/// These are used during execution ([crate::instantiation::execute])
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Value {
    Bool(bool),
    Integer(BigInt),
    Array(Box<[Value]>),
    /// The initial [Value] a variable has, before it's been set. (translates to `'x` don't care)
    Unset,
    Error,
}

impl Value {
    /// Traverses the Value, to create a best-effort [ConcreteType] for it.
    /// So '1' becomes [INT_CONCRETE_TYPE],
    /// but `Value::Array([])` becomes `ConcreteType::Array(ConcreteType::Unknown)`
    ///
    /// Panics when arrays contain mutually incompatible types
    pub fn get_type_best_effort(
        &self,
        type_substitutor: &mut TypeSubstitutor<ConcreteType, ConcreteTypeVariableIDMarker>,
    ) -> ConcreteType {
        match self {
            Value::Bool(_) => BOOL_CONCRETE_TYPE,
            Value::Integer(_) => INT_CONCRETE_TYPE,
            Value::Array(arr) => {
                let mut arr_iter = arr.iter();
                let Some(fst) = arr_iter.next() else {
                    return ConcreteType::Unknown(type_substitutor.alloc());
                };
                let typ = fst.get_type_best_effort(type_substitutor);

                for other in arr_iter {
                    // Assert the types are correct
                    assert!(other.is_of_type(&typ));
                }

                ConcreteType::Array(Box::new((
                    typ,
                    ConcreteType::Value(Value::Integer(arr.len().into())),
                )))
            }
            Value::Unset | Value::Error => unreachable!(),
        }
    }
    pub fn is_of_type(&self, typ: &ConcreteType) -> bool {
        match (self, typ) {
            (Self::Integer(_), typ) if *typ == INT_CONCRETE_TYPE => true,
            (Self::Bool(_), typ) if *typ == BOOL_CONCRETE_TYPE => true,
            (Self::Array(arr_slice), ConcreteType::Array(arr_typ_box)) => {
                let (arr_content_typ, arr_size_typ) = arr_typ_box.deref();
                if arr_slice.len() != arr_size_typ.unwrap_value().get_int::<usize>().unwrap() {
                    return false;
                }
                for v in arr_slice.iter() {
                    if !v.is_of_type(arr_content_typ) {
                        return false;
                    }
                }
                true
            }
            (Self::Unset, _) => true,
            (Self::Error, _) => true,
            _other => false,
        }
    }

    #[track_caller]
    pub fn unwrap_integer(&self) -> &BigInt {
        let Self::Integer(i) = self else {
            panic!("{:?} is not an integer!", self)
        };
        i
    }

    #[track_caller]
    pub fn get_int<IntT: for<'i> TryFrom<&'i BigInt>>(&self) -> Option<IntT> {
        let Self::Integer(i) = self else {
            panic!("{:?} is not an integer!", self)
        };
        IntT::try_from(i).ok()
    }

    #[track_caller]
    pub fn unwrap_bool(&self) -> bool {
        let Self::Bool(b) = self else {
            panic!("{:?} is not a bool!", self)
        };
        *b
    }
}

pub fn compute_unary_op(op: UnaryOperator, v: &Value) -> Value {
    if *v == Value::Error {
        unreachable!("unary op on Value::Error!")
        //return Value::Error
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
            let Value::Bool(b) = v else {
                unreachable!("Only not bool supported, should be caught by abstract typecheck")
            };
            Value::Bool(!*b)
        }
        UnaryOperator::Sum => {
            todo!("Array Values")
        }
        UnaryOperator::Product => {
            todo!("Array Values")
        }
        UnaryOperator::Negate => {
            let Value::Integer(v) = v else { panic!() };
            Value::Integer(-v)
        }
    }
}

pub fn compute_binary_op(left: &Value, op: BinaryOperator, right: &Value) -> Value {
    if *left == Value::Error || *right == Value::Error {
        unreachable!("binary op on Value::Error!")
        //return Value::Error
    }
    match op {
        BinaryOperator::Equals => Value::Bool(left == right),
        BinaryOperator::NotEquals => Value::Bool(left != right),
        BinaryOperator::GreaterEq => Value::Bool(left.unwrap_integer() >= right.unwrap_integer()),
        BinaryOperator::Greater => Value::Bool(left.unwrap_integer() > right.unwrap_integer()),
        BinaryOperator::LesserEq => Value::Bool(left.unwrap_integer() <= right.unwrap_integer()),
        BinaryOperator::Lesser => Value::Bool(left.unwrap_integer() < right.unwrap_integer()),
        BinaryOperator::Add => Value::Integer(left.unwrap_integer() + right.unwrap_integer()),
        BinaryOperator::Subtract => Value::Integer(left.unwrap_integer() - right.unwrap_integer()),
        BinaryOperator::Multiply => Value::Integer(left.unwrap_integer() * right.unwrap_integer()),
        BinaryOperator::Divide => Value::Integer(left.unwrap_integer() / right.unwrap_integer()),
        BinaryOperator::Modulo => Value::Integer(left.unwrap_integer() % right.unwrap_integer()),
        BinaryOperator::And => Value::Bool(left.unwrap_bool() & right.unwrap_bool()),
        BinaryOperator::Or => Value::Bool(left.unwrap_bool() & right.unwrap_bool()),
        BinaryOperator::Xor => Value::Bool(left.unwrap_bool() & right.unwrap_bool()),
        //BinaryOperator::ShiftLeft => todo!(), // Still a bit iffy about shift operator inclusion
        //BinaryOperator::ShiftRight => todo!()
    }
}

impl ConcreteType {
    pub fn get_initial_val(&self) -> Value {
        match self {
            ConcreteType::Named(_name) => Value::Unset,
            ConcreteType::Array(arr) => {
                let (arr_typ, arr_size) = arr.deref();
                let arr_size: usize = arr_size.unwrap_value().get_int().unwrap();
                let mut arr = Vec::new();
                if arr_size > 0 {
                    let content_typ = arr_typ.get_initial_val();
                    arr.resize(arr_size, content_typ);
                }
                Value::Array(arr.into_boxed_slice())
            }
            ConcreteType::Value(_) | ConcreteType::Unknown(_) => unreachable!(),
        }
    }
}
