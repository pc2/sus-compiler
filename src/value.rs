use std::ops::Deref;

use ibig::IBig;

use crate::alloc::FlatAlloc;
use crate::flattening::{BinaryOperator, UnaryOperator};
use sus_proc_macro::get_builtin_type;

use crate::typing::concrete_type::{ConcreteGlobalReference, ConcreteType, BOOL_CONCRETE_TYPE};
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
    /// So '1' becomes `ConcreteType::Named(ConcreteGlobalReference{id: get_builtin_type!("int"), ...}})`,
    /// but `Value::Array([])` becomes `ConcreteType::Array((ConcreteType::Unknown, 0))`
    ///
    /// Panics when arrays contain mutually incompatible types
    pub fn get_type(
        &self,
        type_substitutor: &mut TypeSubstitutor<ConcreteType>,
    ) -> Result<ConcreteType, String> {
        let mut tensor_sizes = Vec::new();

        enum ValuesRange {
            Bool,
            Int { min: IBig, max: IBig },
        }

        let mut result_range: Option<ValuesRange> = None;

        self.get_tensor_size_recursive(0, &mut tensor_sizes, &mut |v| {
            match (&mut result_range, v) {
                (None, Value::Bool(_)) => result_range = Some(ValuesRange::Bool),
                (Some(ValuesRange::Bool), Value::Bool(_)) => {} // OK
                (None, Value::Integer(v)) => {
                    result_range = Some(ValuesRange::Int {
                        min: v.clone(),
                        max: v.clone(),
                    })
                }
                (Some(ValuesRange::Int { min, max }), Value::Integer(v)) => {
                    if v < min {
                        *min = v.clone();
                    }
                    if v > max {
                        *max = v.clone();
                    }
                }
                (Some(_), Value::Bool(_)) | (Some(_), Value::Integer(_)) => {
                    unreachable!("Differing types is caught by abstract typecheck!")
                }
                (_, Value::Array(_)) => {
                    unreachable!("All arrays handled by get_tensor_size_recursive")
                }
                (_, Value::Unset) | (_, Value::Error) => {
                    return Err("This compile-time constant contains a Unset or Error".into());
                }
            };
            Ok(())
        })?;

        let content_typ = match result_range {
            Some(ValuesRange::Bool) => BOOL_CONCRETE_TYPE,
            Some(ValuesRange::Int { min, max }) => ConcreteType::Named(ConcreteGlobalReference {
                id: get_builtin_type!("int"),
                template_args: FlatAlloc::from_vec(vec![
                    ConcreteType::new_int(min),
                    ConcreteType::new_int(max + 1),
                ]),
            }),
            None => type_substitutor.alloc_unknown(),
        };

        Ok(content_typ.new_arrays_of(&tensor_sizes))
    }
    fn get_tensor_size_recursive(
        &self,
        depth: usize,
        tensor_sizes: &mut Vec<usize>,
        elem_fn: &mut impl FnMut(&Value) -> Result<(), String>,
    ) -> Result<(), String> {
        if let Value::Array(values) = self {
            if let Some(sz) = tensor_sizes.get(depth) {
                if *sz != values.len() {
                    return Err("Value is a Jagged Tensor. This is not allowed!".into());
                }
            } else {
                assert!(tensor_sizes.len() == depth);
                tensor_sizes.push(values.len());
            }
            for v in values {
                v.get_tensor_size_recursive(depth + 1, tensor_sizes, elem_fn)?;
            }
            Ok(())
        } else {
            elem_fn(self)
        }
    }
    pub fn is_of_type(&self, typ: &ConcreteType) -> bool {
        match (self, typ) {
            (Self::Integer(_), ConcreteType::Named(global_ref)) => {
                global_ref.id == get_builtin_type!("int")
            }
            (Self::Bool(_), typ) if *typ == BOOL_CONCRETE_TYPE => true,
            (Self::Array(arr_slice), ConcreteType::Array(arr_typ_box)) => {
                let (arr_content_typ, arr_size_typ) = arr_typ_box.deref();
                if IBig::from(arr_slice.len()) != *arr_size_typ.unwrap_value().unwrap_integer() {
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
