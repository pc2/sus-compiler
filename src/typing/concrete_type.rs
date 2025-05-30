use ibig::ibig;
use ibig::ops::Abs;
use ibig::IBig;
use ibig::UBig;
use sus_proc_macro::get_builtin_type;

use crate::instantiation::RealWirePathElem;
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

impl<ID> ConcreteGlobalReference<ID> {
    /// Means that the ConcreteGlobalReference contains no Unknowns
    ///
    /// If true, then this is a unique ID for a specific instantiated object
    pub fn is_final(&self) -> bool {
        !self.template_args.iter().any(|(_, v)| v.contains_unknown())
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

    pub fn sizeof_named(type_ref: &ConcreteGlobalReference<TypeUUID>) -> u64 {
        match type_ref.id {
            get_builtin_type!("int") => {
                let [min, max] = type_ref.template_args.cast_to_int_array();
                bound_to_bits(min, &(max - 1))
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
            }
        }

        cur_typ
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
