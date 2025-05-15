use std::ops::Deref;

use ibig::IBig;

use crate::alloc::FlatAlloc;
use crate::flattening::{BinaryOperator, UnaryOperator};
use crate::typing::template::TemplateKind;
use sus_proc_macro::get_builtin_type;

use crate::typing::concrete_type::{ConcreteGlobalReference, ConcreteType, BOOL_CONCRETE_TYPE};
use crate::typing::type_inference::{
    ConcreteTypeVariableID, ConcreteTypeVariableIDMarker, Substitutor, TypeSubstitutor, Unifyable,
};

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
    Unknown(ConcreteTypeVariableID),
}

impl Unifyable for Value {
    type IDMarker = ConcreteTypeVariableIDMarker;
    fn get_unknown(&self) -> Option<ConcreteTypeVariableID> {
        if let Value::Unknown(var) = self {
            Some(*var)
        } else {
            None
        }
    }

    fn new_unknown(id: ConcreteTypeVariableID) -> Self {
        Value::Unknown(id)
    }
}

/*impl ConcreteType {
    fn update_smallest_common_supertype(&mut self, other: &Self) -> Option<()> {
        match (self, other) {
            (_, ConcreteType::Unknown(_)) | (ConcreteType::Unknown(_), _) => None,
            (ConcreteType::Named(left), ConcreteType::Named(right)) => {
                assert_eq!(left.id, right.id);
                if left.id == get_builtin_type!("int") {
                    if let (
                        [TemplateKind::Value(ConcreteType::Value(Value::Integer(left_min))), TemplateKind::Value(ConcreteType::Value(Value::Integer(left_max)))],
                        [TemplateKind::Value(ConcreteType::Value(Value::Integer(right_min))), TemplateKind::Value(ConcreteType::Value(Value::Integer(right_max)))],
                    ) = (
                        left.template_args.cast_to_array_mut(),
                        right.template_args.cast_to_array(),
                    ) {
                        if right_min < left_min {
                            *left_min = right_min.clone();
                        }
                        if right_max > left_max {
                            *left_max = right_max.clone();
                        }
                        Some(())
                    } else {
                        None
                    }
                } else {
                    for (_, left_arg, right_arg) in
                        zip_eq(left.template_args.iter_mut(), right.template_args.iter())
                    {
                        left_arg.update_smallest_common_supertype(right_arg)?;
                    }
                    Some(())
                }
            }
            (ConcreteType::Array(left), ConcreteType::Array(right)) => {
                let (left_content, left_size) = left.deref_mut();
                let (right_content, right_size) = right.deref();
                left_size.update_smallest_common_supertype(right_size)?;
                left_content.update_smallest_common_supertype(right_content)
            }
            (ConcreteType::Value(left), ConcreteType::Value(right)) => {
                (left == right).then_some(())
            }
            _ => unreachable!("Caught by typecheck"),
        }
    }
    /// On the road to implementing subtyping. Takes in a list of types,
    /// and computes the smallest supertype that all list elements can coerce to.
    /// TODO integrate into Hindley-Milner more closely
    fn get_smallest_common_supertype(
        mut iter: impl Iterator<Item = Self>,
        type_substitutor: &mut TypeSubstitutor<ConcreteType>,
    ) -> Option<Self> {
        let mut first = iter.next()?;
        let _ = type_substitutor.fully_substitute(&mut first);

        for mut elem in iter {
            let _ = type_substitutor.fully_substitute(&mut elem);
            first.update_smallest_common_supertype(&elem)?;
        }

        Some(first)
    }
}*/

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
                (_, Value::Unset) => {
                    return Err("This compile-time constant contains Unset".into());
                }
                (_, Value::Unknown(_)) => {
                    panic!("get_type on Value::Unknown!");
                }
            };
            Ok(())
        })?;

        let content_typ = match result_range {
            Some(ValuesRange::Bool) => BOOL_CONCRETE_TYPE,
            Some(ValuesRange::Int { min, max }) => ConcreteType::Named(ConcreteGlobalReference {
                id: get_builtin_type!("int"),
                template_args: FlatAlloc::from_vec(vec![
                    TemplateKind::Value(ConcreteType::new_int(min)),
                    TemplateKind::Value(ConcreteType::new_int(max + 1)),
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

    pub fn contains_errors_or_unsets(&self) -> bool {
        match self {
            Value::Bool(_) | Value::Integer(_) => false,
            Value::Array(values) => values.iter().any(|v| v.contains_errors_or_unsets()),
            Value::Unset => true,
            Value::Unknown(_) => {
                panic!("contains_errors_or_unsets on Value::Unknown!");
            }
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
