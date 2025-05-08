use std::ops::Deref;

use ibig::IBig;

use crate::flattening::{BinaryOperator, UnaryOperator};

use crate::typing::concrete_type::{ConcreteType, BOOL_CONCRETE_TYPE, INT_CONCRETE_TYPE};
use crate::typing::type_inference::{Substitutor, TypeSubstitutor};

/// Top type for any kind of compiletime value while executing.
///
/// These are used during execution ([crate::instantiation::execute])
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Value {
    Bool(bool),
    Integer(IBig),
    Array(Vec<Value>),
    /// The initial [Value] a variable has, before it's been set. (translates to `'x` don't care)
    Unset,
    Error,
}

impl ConcreteType {
    /// On the road to implementing subtyping. Takes in a list of types,
    /// and computes the smallest supertype that all list elements can coerce to.
    /// TODO integrate into Hindley-Milner more closely
    fn get_smallest_common_supertype(
        list: &[Self],
        type_substitutor: &mut TypeSubstitutor<ConcreteType>,
    ) -> Option<Self> {
        let mut iter = list.iter();

        let first = iter.next()?.clone();

        for elem in iter {
            type_substitutor.unify_must_succeed(&first, elem);
        }

        Some(first)
    }
}

impl Value {
    /// Traverses the Value, to create a best-effort [ConcreteType] for it.
    /// So '1' becomes [INT_CONCRETE_TYPE],
    /// but `Value::Array([])` becomes `ConcreteType::Array((ConcreteType::Unknown, 0))`
    ///
    /// Panics when arrays contain mutually incompatible types
    pub fn get_type(&self, type_substitutor: &mut TypeSubstitutor<ConcreteType>) -> ConcreteType {
        match self {
            Value::Bool(_) => BOOL_CONCRETE_TYPE,
            Value::Integer(_) => INT_CONCRETE_TYPE,
            Value::Array(arr) => {
                let typs_arr: Vec<ConcreteType> = arr
                    .iter()
                    .map(|elem| elem.get_type(type_substitutor))
                    .collect();

                let shared_supertype =
                    ConcreteType::get_smallest_common_supertype(&typs_arr, type_substitutor)
                        .unwrap_or_else(|| type_substitutor.alloc_unknown());

                ConcreteType::Array(Box::new((
                    shared_supertype,
                    ConcreteType::Value(Value::Integer(arr.len().into())),
                )))
            }
            Value::Unset => type_substitutor.alloc_unknown(),
            Value::Error => unreachable!("{self:?}"),
        }
    }

    pub fn contains_errors_or_unsets(&self) -> bool {
        match self {
            Value::Bool(_) | Value::Integer(_) => false,
            Value::Array(values) => values.iter().any(|v| v.contains_errors_or_unsets()),
            Value::Unset | Value::Error => true,
        }
    }

    #[track_caller]
    pub fn unwrap_integer(&self) -> &IBig {
        let Self::Integer(i) = self else {
            panic!("{:?} is not an integer!", self)
        };
        i
    }

    #[track_caller]
    pub fn unwrap_int<IntT: for<'i> TryFrom<&'i IBig>>(&self) -> IntT {
        let Self::Integer(i) = self else {
            panic!("{:?} is not an integer!", self)
        };
        IntT::try_from(i).ok().unwrap()
    }

    #[track_caller]
    pub fn unwrap_bool(&self) -> bool {
        let Self::Bool(b) = self else {
            panic!("{:?} is not a bool!", self)
        };
        *b
    }

    pub fn unwrap_array(&self) -> &[Value] {
        let Self::Array(arr) = self else {
            panic!("{:?} is not an array!", self)
        };
        arr
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
                let arr_size: usize = arr_size.unwrap_value().unwrap_int();
                let mut arr = Vec::new();
                if arr_size > 0 {
                    let content_typ = arr_typ.get_initial_val();
                    arr.resize(arr_size, content_typ);
                }
                Value::Array(arr)
            }
            ConcreteType::Value(_) | ConcreteType::Unknown(_) => unreachable!(),
        }
    }
}
