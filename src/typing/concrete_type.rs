use ibig::IBig;
use ibig::UBig;
use sus_proc_macro::get_builtin_type;

use crate::instantiation::RealWirePathElem;
use crate::linker::GlobalUUID;
use crate::prelude::*;
use crate::util::all_equal;
use std::ops::Deref;

use crate::value::Value;

use super::template::TVec;

use super::template::TemplateKind;
use super::value_unifier::UnifyableValue;

pub type ConcreteTemplateArg = TemplateKind<ConcreteType, UnifyableValue>;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ConcreteGlobalReference<ID> {
    pub id: ID,
    pub template_args: TVec<ConcreteTemplateArg>,
}

impl ConcreteTemplateArg {
    pub fn contains_unknown(&self) -> bool {
        match self {
            TemplateKind::Type(t) => t.contains_unknown(),
            TemplateKind::Value(v) => v.is_unknown(),
        }
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
    Array(Box<(ConcreteType, UnifyableValue)>),
}

impl ConcreteType {
    pub const BOOL: ConcreteType = ConcreteType::Named(ConcreteGlobalReference {
        id: get_builtin_type!("bool"),
        template_args: FlatAlloc::new(),
    });

    pub fn stack_arrays_usize(self, tensor_sizes: &[usize]) -> Self {
        let mut result = self;
        for s in tensor_sizes.iter().rev() {
            result = ConcreteType::Array(Box::new((result, Value::Integer(IBig::from(*s)).into())));
        }
        result
    }
    pub fn stack_arrays(self, tensor_sizes: &[UnifyableValue]) -> Self {
        let mut result = self;
        for s in tensor_sizes.iter().rev() {
            result = ConcreteType::Array(Box::new((result, s.clone())));
        }
        result
    }
    #[track_caller]
    pub fn unwrap_named(&self) -> &ConcreteGlobalReference<TypeUUID> {
        let ConcreteType::Named(v) = self else {
            unreachable!("unwrap_named")
        };
        v
    }
    #[track_caller]
    pub fn unwrap_array(&self) -> &(ConcreteType, UnifyableValue) {
        let ConcreteType::Array(arr_box) = self else {
            unreachable!("unwrap_array")
        };
        arr_box
    }
    #[track_caller]
    pub fn unwrap_integer_bounds(&self) -> (&IBig, &IBig) {
        let ConcreteType::Named(v) = self else {
            unreachable!("unwrap_integer_bounds")
        };
        assert_eq!(v.id, get_builtin_type!("int"));
        let [min, max] = v.template_args.cast_to_int_array();
        (min, max)
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
            ConcreteType::Array(arr_box) => {
                let (arr_arr, arr_size) = arr_box.deref();
                arr_arr.contains_unknown() || arr_size.is_unknown()
            }
        }
    }
    /// Requires all parameters to be known and already substituted!
    ///
    /// a return value of true means that `self` can be assigned to `other`
    pub fn is_subtype_of(&self, other: &Self) -> bool {
        match (self, other) {
            (ConcreteType::Named(a), ConcreteType::Named(b)) => {
                assert_eq!(a.id, b.id);
                match all_equal([a.id, b.id]) {
                    get_builtin_type!("int") => {
                        let [a_min, a_max] = a.template_args.cast_to_int_array();
                        let [b_min, b_max] = b.template_args.cast_to_int_array();

                        (a_min >= b_min) && (a_max <= b_max)
                    }
                    _ => {
                        crate::alloc::zip_eq(&a.template_args, &b.template_args).all(|(_, a, b)| {
                            match a.and_by_ref(b) {
                                TemplateKind::Type((a, b)) => a.is_subtype_of(b),
                                TemplateKind::Value((a, b)) => a.unwrap_set() == b.unwrap_set(),
                            }
                        })
                    }
                }
            }
            (ConcreteType::Array(arr_a), ConcreteType::Array(arr_b)) => {
                let (a, sz_a) = arr_a.deref();
                let (b, sz_b) = arr_b.deref();

                a.is_subtype_of(b) && (sz_a.unwrap_integer() == sz_b.unwrap_integer())
            }
            _ => unreachable!(),
        }
    }
    /// Returns the size of this type in *wires*. So int #(MAX: 255) would return '8'
    ///
    /// If it contains any Unknowns, then returns None
    pub fn sizeof(&self) -> Option<IBig> {
        match self {
            ConcreteType::Named(reference) => Some(Self::sizeof_named(reference).into()),
            ConcreteType::Array(arr_box) => {
                let (typ, size) = arr_box.deref();

                let mut typ_sz = typ.sizeof()?;

                typ_sz *= size.unwrap_integer();

                Some(typ_sz)
            }
        }
    }

    /// Returns the width of the packed bit vector
    pub fn can_be_represented_as_packed_bits(&self) -> Option<u64> {
        match self {
            ConcreteType::Named(name) => Some(Self::sizeof_named(name)),
            ConcreteType::Array(arr_box) => {
                let (content, sz) = arr_box.deref();

                if let ConcreteType::Named(ConcreteGlobalReference {
                    id: get_builtin_type!("bool"),
                    template_args: _,
                }) = content
                {
                    Some(sz.unwrap_int())
                } else {
                    None
                }
            }
        }
    }

    pub fn sizeof_named(type_ref: &ConcreteGlobalReference<TypeUUID>) -> u64 {
        match type_ref.id {
            get_builtin_type!("int") => {
                let [min, max] = type_ref.template_args.cast_to_int_array();
                get_int_bitwidth(min, max)
            }
            get_builtin_type!("bool") => 1,
            get_builtin_type!("float") => 32,
            _other => todo!("Other Named Structs are not implemented yet"),
        }
    }

    pub fn walk_rank(&self, rank: usize) -> &ConcreteType {
        let mut typ = self;
        for _ in 0..rank {
            typ = &typ.unwrap_array().0;
        }
        typ
    }
    pub fn walk_path(&self, path: &[RealWirePathElem]) -> &ConcreteType {
        let mut cur_typ = self;
        for p in path {
            match p {
                RealWirePathElem::ArrayAccess { .. } => {
                    cur_typ = &cur_typ.unwrap_array().0;
                }
                RealWirePathElem::ArrayPartSelectDown { .. }
                | RealWirePathElem::ArrayPartSelectUp { .. }
                | RealWirePathElem::ArraySlice { .. } => {}
            }
        }

        cur_typ
    }
}

impl ConcreteType {
    pub fn is_valid(&self) -> bool {
        match self {
            ConcreteType::Named(global_ref) => {
                if !global_ref.find_invalid_template_args().is_empty() {
                    return false;
                }

                if global_ref.id == get_builtin_type!("int") {
                    let [min, max] = global_ref.template_args.cast_to_int_array();
                    if min > max {
                        return false;
                    }
                }

                true
            }
            ConcreteType::Array(arr_box) => {
                let (content, size) = arr_box.deref();

                if size.is_unknown() {
                    return false;
                }

                let size = size.unwrap_integer();
                content.is_valid() && size >= &IBig::from(0) && size < &IBig::from(10000000)
            }
        }
    }
}

impl<ID: Into<GlobalUUID> + Copy> ConcreteGlobalReference<ID> {
    pub fn find_invalid_template_args(&self) -> Vec<TemplateID> {
        let mut failures = Vec::new();
        for (id, arg) in &self.template_args {
            let is_okay = match arg {
                TemplateKind::Type(t) => t.is_valid(),
                TemplateKind::Value(v) => !v.is_unknown(),
            };
            if !is_okay {
                failures.push(id);
            }
        }
        failures
    }

    pub fn report_if_errors(&self, linker: &Linker, context: &str) -> Result<(), String> {
        let error_parameters = self.find_invalid_template_args();
        if !error_parameters.is_empty() {
            let mut resulting_error = format!("{context} The arguments ");
            for id in error_parameters {
                use std::fmt::Write;
                write!(
                    resulting_error,
                    "'{}', ",
                    &linker.globals[self.id.into()].template_parameters[id].name
                )
                .unwrap();
            }

            resulting_error.pop();
            resulting_error.pop();
            resulting_error.push_str(" were not valid");

            Err(resulting_error)
        } else {
            Ok(())
        }
    }
}

pub fn get_int_bitwidth(min: &IBig, max: &IBig) -> u64 {
    assert!(
        min <= max,
        "Integer Min is not less than max! Min: {min}, Max: {max}"
    );
    if min < &IBig::from(0) {
        let min_abs: UBig = UBig::try_from(-min).unwrap() - 1;

        let bits_for_min = min_abs.bit_len();

        let bits_for_max = if max > &IBig::from(0) {
            let max = UBig::try_from(max).unwrap();

            max.bit_len()
        } else {
            0
        };

        (usize::max(bits_for_min, bits_for_max) + 1) as u64
    } else {
        let max = UBig::try_from(max).unwrap();

        u64::max(max.bit_len() as u64, 1) // Can't have 0-width wires
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bound_to_bits() {
        assert_eq!(get_int_bitwidth(&IBig::from(-1), &IBig::from(0)), 1);
        assert_eq!(get_int_bitwidth(&IBig::from(-2), &IBig::from(0)), 2);
        assert_eq!(get_int_bitwidth(&IBig::from(-1), &IBig::from(1)), 2);
        assert_eq!(get_int_bitwidth(&IBig::from(-2), &IBig::from(2)), 3);
        assert_eq!(get_int_bitwidth(&IBig::from(2), &IBig::from(8)), 4);
        assert_eq!(get_int_bitwidth(&IBig::from(-1000), &IBig::from(0)), 11);
        assert_eq!(get_int_bitwidth(&IBig::from(-2000), &IBig::from(-1000)), 12);
        assert_eq!(get_int_bitwidth(&IBig::from(-256), &IBig::from(255)), 9);
        assert_eq!(get_int_bitwidth(&IBig::from(0), &IBig::from(255)), 8);
        assert_eq!(get_int_bitwidth(&IBig::from(20), &IBig::from(256)), 9);
        assert_eq!(get_int_bitwidth(&IBig::from(0), &IBig::from(0)), 1); // Temporary fix, such that we never generate Length 0 wires (#86)
    }
}
