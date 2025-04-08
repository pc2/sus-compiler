use ibig::ibig;
use ibig::ops::Abs;
use ibig::IBig;
use ibig::UBig;
use sus_proc_macro::get_builtin_type;

use crate::alloc::zip_eq;
use crate::alloc::UUID;
use crate::flattening::StructType;
use crate::linker::LinkInfo;
use crate::prelude::*;
use std::ops::{Deref, Index};

use crate::value::Value;

use super::template::TVec;

use super::type_inference::ConcreteTypeVariableID;
use super::type_inference::ConcreteTypeVariableIDMarker;
use super::type_inference::HindleyMilner;
use super::type_inference::TypeSubstitutor;

pub const BOOL_CONCRETE_TYPE: ConcreteType = ConcreteType::Named(ConcreteGlobalReference {
    id: get_builtin_type!("bool"),
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
    pub fn pretty_print_concrete_instance(
        &self,
        target_link_info: &LinkInfo,
        linker_types: &impl Index<TypeUUID, Output = StructType>,
    ) -> String {
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
                    writeln!(result, "type {},", arg.display(linker_types)).unwrap();
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
    pub fn new_int(int: IBig) -> Self {
        Self::Value(Value::Integer(int))
    }
    #[track_caller]
    pub fn unwrap_value(&self) -> &Value {
        let ConcreteType::Value(v) = self else {
            unreachable!("unwrap_value on {self:?}")
        };
        v
    }
    #[track_caller]
    pub fn unwrap_named(&self) -> &ConcreteGlobalReference<TypeUUID> {
        let ConcreteType::Named(v) = self else {
            unreachable!("unwrap_named")
        };
        v
    }
    #[track_caller]
    pub fn unwrap_array(&self) -> &(ConcreteType, ConcreteType) {
        let ConcreteType::Array(arr_box) = self else {
            unreachable!("unwrap_array")
        };
        arr_box
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

    pub fn sizeof_named(type_ref: &ConcreteGlobalReference<TypeUUID>) -> u64 {
        match type_ref.id {
            get_builtin_type!("int") => {
                let min = type_ref.template_args[UUID::from_hidden_value(0)]
                    .unwrap_value()
                    .unwrap_integer();
                let max = type_ref.template_args[UUID::from_hidden_value(1)]
                    .unwrap_value()
                    .unwrap_integer()
                    - ibig!(1);
                bound_to_bits(min, &max)
            }
            get_builtin_type!("bool") => 1,
            get_builtin_type!("float") => 32,
            _other => todo!("Other Named Structs are not implemented yet"),
        }
    }
    pub fn try_fully_substitute(
        &self,
        substitutor: &TypeSubstitutor<Self, ConcreteTypeVariableIDMarker>,
    ) -> Option<Self> {
        let mut self_clone = self.clone();
        if self_clone.fully_substitute(substitutor) {
            Some(self_clone)
        } else {
            None
        }
    }
    /// Returns the inclusive bounds of an int. An int #(MIN: 0, MAX: 15) will return (0, 14)
    pub fn get_bounds(&self) -> (IBig, IBig) {
        let min = self.unwrap_named().template_args[UUID::from_hidden_value(0)]
            .unwrap_value()
            .unwrap_integer();
        let max = self.unwrap_named().template_args[UUID::from_hidden_value(1)]
            .unwrap_value()
            .unwrap_integer();
        (min.clone(), max - 1)
    }
}

fn bits_negative(value: &IBig) -> u64 {
    (UBig::try_from(value.abs() - 1).unwrap().bit_len() + 1)
        .try_into()
        .unwrap()
}

fn bits_positive(value: &IBig) -> u64 {
    (UBig::try_from(value).unwrap().bit_len() + 1)
        .try_into()
        .unwrap()
}

fn bound_to_bits(min: &IBig, max: &IBig) -> u64 {
    [min, max]
        .iter()
        .map(|&num| {
            if num >= &ibig!(0) {
                bits_positive(num)
            } else {
                bits_negative(num)
            }
        })
        .max()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_bits_negative() {
        assert_eq!(bits_negative(&ibig!(-1)), 1);
        assert_eq!(bits_negative(&ibig!(-2)), 2);
        assert_eq!(bits_negative(&ibig!(-3)), 3);
        assert_eq!(bits_negative(&ibig!(-4)), 3);
        assert_eq!(bits_negative(&ibig!(-5)), 4);
        assert_eq!(bits_negative(&ibig!(-8)), 4);
        assert_eq!(bits_negative(&ibig!(-9)), 5);
        assert_eq!(bits_negative(&ibig!(-16)), 5);
    }
    #[test]
    fn test_bits_positive() {
        assert_eq!(bits_positive(&ibig!(0)), 1);
        assert_eq!(bits_positive(&ibig!(1)), 2);
        assert_eq!(bits_positive(&ibig!(2)), 3);
        assert_eq!(bits_positive(&ibig!(3)), 3);
        assert_eq!(bits_positive(&ibig!(4)), 4);
        assert_eq!(bits_positive(&ibig!(7)), 4);
        assert_eq!(bits_positive(&ibig!(8)), 5);
        assert_eq!(bits_positive(&ibig!(15)), 5);
        assert_eq!(bits_positive(&ibig!(16)), 6);
        assert_eq!(bits_positive(&ibig!(31)), 6);
    }
    #[test]
    fn test_bound_to_bits() {
        assert_eq!(bound_to_bits(&ibig!(-1), &ibig!(0)), 1);
        assert_eq!(bound_to_bits(&ibig!(-2), &ibig!(0)), 2);
        assert_eq!(bound_to_bits(&ibig!(-1), &ibig!(1)), 2);
        assert_eq!(bound_to_bits(&ibig!(-2), &ibig!(2)), 3);
        assert_eq!(bound_to_bits(&ibig!(2), &ibig!(8)), 5);
        assert_eq!(bound_to_bits(&ibig!(-1000), &ibig!(0)), 11);
        assert_eq!(bound_to_bits(&ibig!(-2000), &ibig!(-1000)), 12);
        assert_eq!(bound_to_bits(&ibig!(-256), &ibig!(255)), 9);
    }
}
