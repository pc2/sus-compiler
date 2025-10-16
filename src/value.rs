use std::ops::Deref;

use ibig::IBig;
use ordered_float::NotNan;

use sus_proc_macro::get_builtin_type;

use crate::flattening::{BinaryOperator, UnaryOperator};

use crate::typing::concrete_type::{ConcreteTemplateArg, ConcreteType};
use crate::typing::set_unifier::Unifyable;

/// Top type for any kind of compiletime value while executing.
///
/// These are used during execution ([crate::instantiation::execute])
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Value {
    Bool(bool),
    Integer(IBig),
    /// Temporary, one day we'll have Rationals here instead
    Float(NotNan<f32>),
    /// Temporary, one day we'll have Rationals here instead
    Double(NotNan<f64>),
    String(String),
    Array(Vec<Value>),
    /// The initial [Value] a variable has, before it's been set. (translates to `'x` don't care)
    Unset,
}
impl PartialOrd for Value {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for Value {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        use Value::*;
        match (self, other) {
            (Bool(a), Bool(b)) => a.cmp(b),
            (Integer(a), Integer(b)) => a.cmp(b),
            (Float(a), Float(b)) => a.cmp(b),
            (Double(a), Double(b)) => a.cmp(b),
            (Array(a), Array(b)) => a.cmp(b),
            _ => unreachable!("Should have been caught by typecheck"),
        }
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
    pub fn contains_unset(&self) -> bool {
        match self {
            Value::Bool(_)
            | Value::Integer(_)
            | Value::Float(_)
            | Value::Double(_)
            | Value::String(_) => false,
            Value::Array(values) => values.iter().any(|v| v.contains_unset()),
            Value::Unset => true,
        }
    }
    pub fn is_unset(&self) -> bool {
        match self {
            Value::Unset => true,
            Value::Array(values) => values.iter().all(|v| v.is_unset()),
            Value::Bool(_)
            | Value::Integer(_)
            | Value::Float(_)
            | Value::Double(_)
            | Value::String(_) => false,
        }
    }

    #[track_caller]
    pub fn unwrap_integer(&self) -> &IBig {
        let Self::Integer(i) = self else {
            panic!("{self:?} is not an integer!")
        };
        i
    }

    #[track_caller]
    pub fn unwrap_int<IntT: for<'i> TryFrom<&'i IBig>>(&self) -> IntT {
        let Self::Integer(i) = self else {
            panic!("{self:?} is not an integer!")
        };
        IntT::try_from(i).ok().unwrap()
    }

    #[track_caller]
    pub fn unwrap_bool(&self) -> bool {
        let Self::Bool(b) = self else {
            panic!("{self:?} is not a bool!")
        };
        *b
    }

    pub fn unwrap_array(&self) -> &[Value] {
        let Self::Array(arr) = self else {
            panic!("{self:?} is not an array!")
        };
        arr
    }

    /// Requires `typ` to be fully substituted
    ///
    /// Allows the existense of [Value::Unset]
    pub fn is_of_type(&self, typ: &ConcreteType) -> bool {
        match self {
            Value::Bool(_) => typ.unwrap_named().id == get_builtin_type!("bool"),
            Value::Float(_) => typ.unwrap_named().id == get_builtin_type!("float"),
            Value::Double(_) => typ.unwrap_named().id == get_builtin_type!("double"),
            Value::String(_) => typ.unwrap_named().id == get_builtin_type!("string"),
            Value::Integer(v) => {
                let bounds = typ.unwrap_int_bounds();
                v >= bounds.from && v < bounds.to
            }
            Value::Array(values) => {
                let (content, sz) = typ.unwrap_array_known_size();
                values.len() == usize::try_from(sz).unwrap()
                    && values.iter().all(|v| v.is_of_type(content))
            }
            Value::Unset => true,
        }
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
                let arr_size: usize = arr_size.unwrap_int();
                let mut arr = Vec::new();
                if arr_size > 0 {
                    let content_typ = arr_typ.get_initial_val();
                    arr.resize(arr_size, content_typ);
                }
                Value::Array(arr)
            }
        }
    }
}

impl From<IBig> for Value {
    fn from(value: IBig) -> Self {
        Value::Integer(value)
    }
}

impl From<bool> for Value {
    fn from(value: bool) -> Self {
        Value::Bool(value)
    }
}

impl From<Vec<Value>> for Value {
    fn from(value: Vec<Value>) -> Self {
        Value::Array(value)
    }
}

impl From<IBig> for ConcreteTemplateArg {
    fn from(value: IBig) -> Self {
        ConcreteTemplateArg::Value(Unifyable::Set(Value::Integer(value)))
    }
}

impl From<bool> for ConcreteTemplateArg {
    fn from(value: bool) -> Self {
        ConcreteTemplateArg::Value(Unifyable::Set(Value::Bool(value)))
    }
}

impl From<Vec<Value>> for ConcreteTemplateArg {
    fn from(value: Vec<Value>) -> Self {
        ConcreteTemplateArg::Value(Unifyable::Set(Value::Array(value)))
    }
}
