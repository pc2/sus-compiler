use num::BigInt;

use crate::prelude::*;
use std::ops::Deref;

use crate::linker::get_builtin_type;
use crate::
    value::Value
;

use super::type_inference::ConcreteTypeVariableID;

pub const BOOL_CONCRETE_TYPE: ConcreteType = ConcreteType::Named(get_builtin_type("bool"));
pub const INT_CONCRETE_TYPE: ConcreteType = ConcreteType::Named(get_builtin_type("int"));

/// A post-instantiation type. These fully define what wires should be generated for a given object. 
/// So as opposed to [crate::typing::abstract_type::AbstractType], type parameters are filled out with concrete values. 
/// 
/// Examples: `bool[3]`, `int #(MAX: 20)`
/// 
/// Not to be confused with [crate::typing::abstract_type::AbstractType] which represents pre-instantiation types,
/// or [crate::flattening::WrittenType] which represents the textual in-editor data. 
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ConcreteType {
    Named(TypeUUID),
    Value(Value),
    Array(Box<(ConcreteType, ConcreteType)>),
    /// Referencing [ConcreteType::Unknown] is a strong code smell. 
    /// It is likely you should use [crate::typing::type_inference::TypeSubstitutor::unify_must_succeed]
    /// or [crate::typing::type_inference::TypeSubstitutor::unify_report_error] instead
    /// 
    /// It should only occur in creation `ConcreteType::Unknown(self.type_substitutor.alloc())`
    Unknown(ConcreteTypeVariableID)
}

impl ConcreteType {
    #[track_caller]
    pub fn unwrap_value(&self) -> &Value {
        let ConcreteType::Value(v) = self else {
            unreachable!("unwrap_value on {self:?}")
        };
        v
    }
    pub fn down_array(&self) -> &ConcreteType {
        let ConcreteType::Array(arr_box) = self else {
            unreachable!("Must be an array! Is {self:?} instead")
        };
        let (sub, _sz) = arr_box.deref();
        sub
    }
    pub fn contains_unknown(&self) -> bool {
        match self {
            ConcreteType::Named(_) => false,
            ConcreteType::Value(_) => false,
            ConcreteType::Array(arr_box) => {
                let (arr_arr, arr_size) = arr_box.deref();
                arr_arr.contains_unknown() || arr_size.contains_unknown()
            }
            ConcreteType::Unknown(_) => true,
        }
    }
    /// Returns the size of this type in *wires*. So int #(MAX: 255) would return '8'
    /// 
    /// If it contains any Unknowns, then returns None
    pub fn sizeof(&self) -> Option<BigInt> {
        match self {
            ConcreteType::Named(uuid) => Some(Self::sizeof_named(*uuid).into()),
            ConcreteType::Value(_value) => unreachable!("Root of ConcreteType cannot be a value"),
            ConcreteType::Array(arr_box) => {
                let (typ, size) = arr_box.deref();

                let mut typ_sz = typ.sizeof()?;

                let ConcreteType::Value(arr_sz) = size else {return None};

                typ_sz *= arr_sz.unwrap_integer();

                Some(typ_sz)
            }
            ConcreteType::Unknown(_uuid) => None
        }
    }

    /// TODO #50 Ranged Int work & ConcreteGlobalReference should be integrated
    pub fn sizeof_named(id: TypeUUID) -> u64 {
        if id == get_builtin_type("int") {
            32 // TODO concrete int sizes
        } else if id == get_builtin_type("bool") {
            1
        } else {
            println!("TODO Named Structs Size");
            1 // todo!() // Named structs are not implemented yet
        }
    }
}
