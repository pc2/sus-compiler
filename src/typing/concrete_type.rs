use ibig::IBig;
use sus_proc_macro::get_builtin_type;

use crate::alloc::zip_eq;
use crate::linker::GlobalUUID;
use crate::prelude::*;
use std::ops::Deref;

use crate::value::Value;

use super::template::TVec;

use super::type_inference::ConcreteTypeVariableID;

pub const BOOL_CONCRETE_TYPE: ConcreteType = ConcreteType::Named(ConcreteGlobalReference {
    id: get_builtin_type!("bool"),
    template_args: FlatAlloc::new(),
});

pub const INT_CONCRETE_TYPE: ConcreteType = ConcreteType::Named(ConcreteGlobalReference {
    id: get_builtin_type!("int"),
    template_args: FlatAlloc::new(),
});

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ConcreteGlobalReference<ID> {
    pub id: ID,
    pub template_args: TVec<ConcreteType>,
}

impl<ID> ConcreteGlobalReference<ID> {
    /// Means that the ConcreteGlobalReference contains no Unknowns
    ///
    /// If true, then this is a unique ID for a specific instantiated object
    pub fn is_final(&self) -> bool {
        !self.template_args.iter().any(|(_, v)| v.contains_unknown())
    }
    pub fn pretty_print_concrete_instance(&self, linker: &Linker) -> String
    where
        ID: Into<GlobalUUID> + Copy,
    {
        let target_link_info = linker.get_link_info(self.id.into());
        assert!(self.template_args.len() == target_link_info.template_parameters.len());
        let object_full_name = target_link_info.get_full_name();
        if self.template_args.is_empty() {
            return format!("{object_full_name} #()");
        }
        use std::fmt::Write;
        let mut result = format!("{object_full_name} #(\n");
        for (_id, arg, arg_in_target) in
            zip_eq(&self.template_args, &target_link_info.template_parameters)
        {
            write!(result, "    {}: ", arg_in_target.name).unwrap();
            match arg {
                ConcreteType::Named(_) | ConcreteType::Array(_) => {
                    writeln!(result, "type {},", arg.display(&linker.types)).unwrap();
                }
                ConcreteType::Value(value) => {
                    writeln!(result, "{value},").unwrap();
                }
                ConcreteType::Unknown(_) => {
                    writeln!(result, "/* Could not infer */").unwrap();
                }
            }
        }
        result.push(')');
        result
    }
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
            unreachable!("unwrap_value on {self:?}")
        };
        v
    }
    pub fn contains_unknown(&self) -> bool {
        match self {
            ConcreteType::Named(global_ref) => global_ref
                .template_args
                .iter()
                .any(|concrete_template_arg| concrete_template_arg.1.contains_unknown()),
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
    pub fn sizeof(&self) -> Option<IBig> {
        match self {
            ConcreteType::Named(reference) => Some(Self::sizeof_named(reference).into()),
            ConcreteType::Value(_value) => unreachable!("Root of ConcreteType cannot be a value"),
            ConcreteType::Array(arr_box) => {
                let (typ, size) = arr_box.deref();

                let mut typ_sz = typ.sizeof()?;

                let ConcreteType::Value(arr_sz) = size else {
                    return None;
                };

                typ_sz *= arr_sz.unwrap_integer();

                Some(typ_sz)
            }
            ConcreteType::Unknown(_uuid) => None,
        }
    }

    /// TODO #50 Ranged Int work should be integrated
    pub fn sizeof_named(type_ref: &ConcreteGlobalReference<TypeUUID>) -> u64 {
        match type_ref.id {
            get_builtin_type!("int") => 32, // TODO concrete int sizes
            get_builtin_type!("bool") => 1,
            get_builtin_type!("float") => 32,
            _other => todo!("Other Named Structs are not implemented yet"),
        }
    }
}
