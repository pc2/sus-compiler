use std::ops::Deref;

use num::BigInt;

use crate::flattening::{BinaryOperator, UnaryOperator};

use crate::typing::{
    abstract_type::{AbstractType, FullType, DomainType, BOOL_TYPE, INT_TYPE},
    concrete_type::{ConcreteType, BOOL_CONCRETE_TYPE, INT_CONCRETE_TYPE}
};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Value {
    Bool(bool),
    Integer(BigInt),
    Array(Box<[Value]>),
    Unset,
    Error
}

impl Value {
    pub fn get_type_of_constant(&self) -> FullType {
        FullType {
            typ : match self {
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
            },
            domain : DomainType::Generative
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
                if arr_slice.len() != arr_size_typ.unwrap_value().unwrap_usize() {
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
    pub fn unwrap_integer(&self) -> &BigInt {
        let Self::Integer(i) = self else {panic!("{:?} is not an integer!", self)};
        i
    }

    #[track_caller]
    pub fn unwrap_usize(&self) -> usize {
        let Self::Integer(i) = self else {panic!("{:?} is not an integer!", self)};
        use num::ToPrimitive;
        i.to_usize().expect("Integer too large? Program crash")
    }

    #[track_caller]
    pub fn unwrap_bool(&self) -> bool {
        let Self::Bool(b) = self else {panic!("{:?} is not a bool!", self)};
        *b
    }
}

pub fn compute_unary_op(op : UnaryOperator, v : &TypedValue) -> TypedValue {
    if v.value == Value::Error {
        unreachable!("unary op on Value::Error!")
        //return TypedValue{typ : , value : Value::Error}
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
            assert_eq!(v.typ, BOOL_CONCRETE_TYPE);
            let Value::Bool(b) = &v.value else {unreachable!("Only not bool supported, should be caught by abstract typecheck")};
            TypedValue::make_bool(!*b)
        }
        UnaryOperator::Sum => {
            todo!("Array Values")
        }
        UnaryOperator::Product => {
            todo!("Array Values")
        }
        UnaryOperator::Negate => {
            assert_eq!(v.typ, INT_CONCRETE_TYPE);
            let Value::Integer(v) = &v.value else {panic!()};
            TypedValue::make_integer(-v)
        }
    }
}

pub fn compute_binary_op(left : &TypedValue, op : BinaryOperator, right : &TypedValue) -> TypedValue {
    if left.value == Value::Error || right.value == Value::Error {
        unreachable!("binary op on Value::Error!")
        //return Value::Error
    }
    let lv = &left.value;
    let rv = &right.value;
    match op {
        BinaryOperator::Equals => TypedValue::make_bool(lv == rv),
        BinaryOperator::NotEquals => TypedValue::make_bool(lv != rv),
        BinaryOperator::GreaterEq => TypedValue::make_bool(lv.unwrap_integer() >= rv.unwrap_integer()),
        BinaryOperator::Greater => TypedValue::make_bool(lv.unwrap_integer() > rv.unwrap_integer()),
        BinaryOperator::LesserEq => TypedValue::make_bool(lv.unwrap_integer() <= rv.unwrap_integer()),
        BinaryOperator::Lesser => TypedValue::make_bool(lv.unwrap_integer() < rv.unwrap_integer()),
        BinaryOperator::Add => TypedValue::make_integer(lv.unwrap_integer() + rv.unwrap_integer()),
        BinaryOperator::Subtract => TypedValue::make_integer(lv.unwrap_integer() - rv.unwrap_integer()),
        BinaryOperator::Multiply => TypedValue::make_integer(lv.unwrap_integer() * rv.unwrap_integer()),
        BinaryOperator::Divide => TypedValue::make_integer(lv.unwrap_integer() / rv.unwrap_integer()),
        BinaryOperator::Modulo => TypedValue::make_integer(lv.unwrap_integer() % rv.unwrap_integer()),
        BinaryOperator::And => TypedValue::make_bool(lv.unwrap_bool() & rv.unwrap_bool()),
        BinaryOperator::Or => TypedValue::make_bool(lv.unwrap_bool() & rv.unwrap_bool()),
        BinaryOperator::Xor => TypedValue::make_bool(lv.unwrap_bool() & rv.unwrap_bool()),
        //BinaryOperator::ShiftLeft => todo!(), // Still a bit iffy about shift operator inclusion
        //BinaryOperator::ShiftRight => todo!()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TypedValue {
    pub value : Value,
    pub typ : ConcreteType
}

impl TypedValue {
    pub fn make_bool(b : bool) -> Self {
        Self{typ : BOOL_CONCRETE_TYPE, value : Value::Bool(b)}
    }
    pub fn make_integer(i : BigInt) -> Self {
        Self{typ : INT_CONCRETE_TYPE, value : Value::Integer(i)}
    }
    /// panics if the value can't be typed. 
    pub fn from_value(value : Value) -> Self {
        Self{typ : value.get_concrete_type_of_constant(), value}
    }

    #[track_caller]
    pub fn unwrap_integer(&self) -> &BigInt {
        self.value.unwrap_integer()
    }

    #[track_caller]
    pub fn unwrap_bool(&self) -> bool {
        self.value.unwrap_bool()
    }
}
