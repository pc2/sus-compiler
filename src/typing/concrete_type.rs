use crate::prelude::*;
use std::ops::Deref;

use crate::linker::get_builtin_type;
use crate::value::Value;

use super::template::ConcreteTemplateArgs;
use super::type_inference::ConcreteTypeVariableID;

pub const BOOL_CONCRETE_TYPE: ConcreteType = ConcreteType::Named(ConcreteGlobalReference {
    id: get_builtin_type("bool"),
    template_args: FlatAlloc::new(),
});

pub const INT_CONCRETE_TYPE: ConcreteType = ConcreteType::Named(ConcreteGlobalReference {
    id: get_builtin_type("int"),
    template_args: FlatAlloc::new(),
});

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ConcreteGlobalReference<ID> {
    pub id: ID,
    pub template_args: ConcreteTemplateArgs,
}

/// A post-instantiation type. These fully define what wires should be generated for a given object. 
/// So as opposed to [crate::typing::abstract_type::AbstractType], type parameters are filled out with concrete values. 
/// 
/// Examples: `bool[3]`, `int #(MAX: 20)`
/// 
/// Not to be confused with [crate::typing::abstract_type::AbstractType] which represents pre-instantiation types,
/// or [crate::flattening::WrittenType] which represents the textual in-editor data. 
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ConcreteType {
    Named(ConcreteGlobalReference<TypeUUID>),
    Value(Value),
    Array(Box<(ConcreteType, ConcreteType)>),
    /// Referencing [ConcreteType::Unknown] is a strong code smell. 
    /// It is likely you should use [crate::typing::type_inference::TypeSubstitutor::unify_must_succeed]
    /// or [crate::typing::type_inference::TypeSubstitutor::unify_report_error] instead
    ///
    /// It should only occur in creation `ConcreteType::Unknown(self.type_substitutor.alloc())`
    Unknown(ConcreteTypeVariableID),
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
}
