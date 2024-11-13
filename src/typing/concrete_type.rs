use crate::prelude::*;

use std::ops::Deref;

use crate::linker::get_builtin_type;
use crate::
    value::Value
;

use super::type_inference::ConcreteTypeVariableID;

pub const BOOL_CONCRETE_TYPE: ConcreteType = ConcreteType::Named(get_builtin_type("bool"));
pub const INT_CONCRETE_TYPE: ConcreteType = ConcreteType::Named(get_builtin_type("int"));

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ConcreteType {
    Named(TypeUUID),
    Value(Value),
    Array(Box<(ConcreteType, ConcreteType)>),
    Unknown(ConcreteTypeVariableID)
}

impl ConcreteType {
    #[track_caller]
    pub fn unwrap_value(&self) -> &Value {
        let ConcreteType::Value(v) = self else {
            unreachable!("unwrap_value")
        };
        v
    }
    pub fn down_array(&self) -> &ConcreteType {
        let ConcreteType::Array(arr_box) = self else {
            unreachable!("Must be an array!")
        };
        let (sub, _sz) = arr_box.deref();
        sub
    }
}
