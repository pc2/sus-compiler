use ibig::IBig;
use ibig::UBig;
use sus_proc_macro::get_builtin_type;

use crate::linker::GlobalUUID;
use crate::linker::LinkerGlobals;
use crate::prelude::*;
use crate::to_string::display_join;
use crate::typing::abstract_type::AbstractGlobalReference;
use crate::typing::abstract_type::AbstractInnerType;
use crate::typing::abstract_type::AbstractRankedType;
use crate::typing::abstract_type::PeanoType;
use crate::typing::unifyable_cell::UniCell;
use crate::util::all_equal;
use crate::value::Value;
use std::ops::Deref;

use super::template::TVec;

use super::template::TemplateKind;

pub type ConcreteTemplateArg = TemplateKind<ConcreteType, UniCell<Value>>;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ConcreteGlobalReference<ID> {
    pub id: ID,
    pub template_args: TVec<ConcreteTemplateArg>,
}

impl ConcreteTemplateArg {
    pub fn contains_unknown(&self) -> bool {
        match self {
            TemplateKind::Type(t) => t.contains_unknown(),
            TemplateKind::Value(v) => v.get().is_none(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SubtypeRelation {
    /// For any param
    Exact,
    /// Only integers. Take the min of values assigned
    Min,
    /// Only integers. Take the max of values assigned
    Max,
}

/// A post-instantiation type. These fully define what wires should be generated for a given object.
/// So as opposed to [crate::typing::abstract_type::AbstractRankedType], type parameters are filled out with concrete values.
///
/// Examples: `bool[3]`, `int #(TO: 20)`
///
/// Not to be confused with [crate::typing::abstract_type::AbstractRankedType] which represents pre-instantiation types,
/// or [crate::flattening::WrittenType] which represents the textual in-editor data.
#[derive(Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum ConcreteType {
    Named(ConcreteGlobalReference<TypeUUID>),
    Array(Box<(ConcreteType, UniCell<Value>)>),
}

impl std::fmt::Debug for ConcreteType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Named(global_ref) => {
                let name = global_ref.id;
                let template_args =
                    display_join(", ", global_ref.template_args.iter(), |f, (arg_id, arg)| {
                        write!(f, "{arg_id:?}: ")?;
                        match arg {
                            TemplateKind::Type(t) => {
                                write!(f, "type {t:?}")?;
                            }
                            TemplateKind::Value(v) => {
                                write!(f, "{v:?}")?;
                            }
                        }
                        Ok(())
                    });
                write!(f, "{name:?} #({template_args})")
            }
            Self::Array(arr_box) => {
                let (content, sz) = arr_box.deref();
                write!(f, "{content:?}[{sz:?}]")
            }
        }
    }
}

impl ConcreteType {
    pub const BOOL: ConcreteType = ConcreteType::Named(ConcreteGlobalReference {
        id: get_builtin_type!("bool"),
        template_args: FlatAlloc::new(),
    });

    pub fn stack_arrays(self, tensor_sizes: Vec<UniCell<Value>>) -> Self {
        let mut result = self;
        for s in tensor_sizes.into_iter().rev() {
            result = ConcreteType::Array(Box::new((result, s)));
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
    pub fn unwrap_array(&self) -> &(ConcreteType, UniCell<Value>) {
        let ConcreteType::Array(arr_box) = self else {
            unreachable!("unwrap_array")
        };
        arr_box
    }
    #[track_caller]
    pub fn unwrap_array_known_size(&self) -> (&ConcreteType, &IBig) {
        let (arr, sz) = self.unwrap_array();
        (arr, sz.unwrap_integer())
    }
    pub fn contains_unknown(&self) -> bool {
        match self {
            ConcreteType::Named(global_ref) => global_ref
                .template_args
                .iter()
                .any(|concrete_template_arg| concrete_template_arg.1.contains_unknown()),
            ConcreteType::Array(arr_box) => {
                let (arr_arr, arr_size) = arr_box.deref();
                arr_arr.contains_unknown() || arr_size.get().is_none()
            }
        }
    }
    pub fn co_iterate_parameters<'a>(
        a: &'a Self,
        b: &'a Self,
        f: &mut impl FnMut(&'a UniCell<Value>, &'a UniCell<Value>, SubtypeRelation),
    ) {
        match (a, b) {
            (ConcreteType::Named(a), ConcreteType::Named(b)) => match all_equal([a.id, b.id]) {
                get_builtin_type!("int") => {
                    let a_bounds = a.unwrap_int_bounds_unknown();
                    let b_bounds = b.unwrap_int_bounds_unknown();

                    f(a_bounds.from, b_bounds.from, SubtypeRelation::Min);
                    f(a_bounds.to, b_bounds.to, SubtypeRelation::Max);
                }
                _ => {
                    for (_, a, b) in crate::alloc::zip_eq(&a.template_args, &b.template_args) {
                        match a.and_by_ref(b) {
                            TemplateKind::Type((a, b)) => Self::co_iterate_parameters(a, b, f),
                            TemplateKind::Value((a, b)) => f(a, b, SubtypeRelation::Exact),
                        }
                    }
                }
            },
            (ConcreteType::Array(arr_a), ConcreteType::Array(arr_b)) => {
                let (a, sz_a) = arr_a.deref();
                let (b, sz_b) = arr_b.deref();

                f(sz_a, sz_b, SubtypeRelation::Exact);

                Self::co_iterate_parameters(a, b, f);
            }
            (a, b) => unreachable!(
                "Non-matching concretetype shape? Should have been caught by abstract typecheck! {a:?}, {b:?}"
            ),
        }
    }
    /// Requires all parameters to be known and already substituted!
    ///
    /// a return value of true means that `self` can be assigned to `other`
    pub fn is_subtype_of(&self, other: &Self) -> bool {
        let mut total_is_subtype = true;
        Self::co_iterate_parameters(self, other, &mut |a, b, relation| match relation {
            SubtypeRelation::Exact => {
                total_is_subtype &= a == b;
            }
            SubtypeRelation::Min => {
                if a.unwrap_integer() < b.unwrap_integer() {
                    total_is_subtype = false;
                }
            }
            SubtypeRelation::Max => {
                if a.unwrap_integer() > b.unwrap_integer() {
                    total_is_subtype = false;
                }
            }
        });
        total_is_subtype
    }
    /// Requires all parameters to be known and already substituted!
    ///
    /// a return value of true means that `self` can be assigned to `other`, and `other` can be assigned to `self`
    pub fn is_identical_to(&self, other: &Self) -> bool {
        let mut total_is_identical = true;
        Self::co_iterate_parameters(self, other, &mut |a, b, _relation| {
            total_is_identical &= a == b
        });
        total_is_identical
    }
    /// Returns the size of this type in *wires*. So int #(TO: 256) would return '8'
    pub fn sizeof(&self) -> UBig {
        match self {
            ConcreteType::Named(reference) => Self::sizeof_named(reference).into(),
            ConcreteType::Array(arr_box) => {
                let (typ, size) = arr_box.deref();

                let mut typ_sz = typ.sizeof();

                typ_sz *= UBig::try_from(size.unwrap_integer()).unwrap();

                typ_sz
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
                let bounds = type_ref.unwrap_int_bounds();
                bounds.bitwidth()
            }
            get_builtin_type!("bool") => 1,
            get_builtin_type!("float") => 32,
            get_builtin_type!("double") => 64,
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

    pub fn to_abstract(&self) -> AbstractRankedType {
        match self {
            ConcreteType::Named(named_typ) => {
                let inner = AbstractInnerType::Named(AbstractGlobalReference {
                    id: named_typ.id,
                    template_arg_types: named_typ.template_args.map(|(_, arg)| match arg {
                        TemplateKind::Type(t) => TemplateKind::Type(t.to_abstract()),
                        TemplateKind::Value(_) => TemplateKind::Value(()),
                    }),
                });
                AbstractRankedType {
                    inner,
                    rank: PeanoType::Zero,
                }
            }
            ConcreteType::Array(arr_box) => {
                let (content, _size) = arr_box.deref();
                content.to_abstract().rank_up()
            }
        }
    }

    pub fn is_valid(&self) -> bool {
        match self {
            ConcreteType::Named(global_ref) => {
                if !global_ref.find_invalid_template_args().is_empty() {
                    return false;
                }

                if global_ref.id == get_builtin_type!("int") {
                    let bounds = global_ref.unwrap_int_bounds();
                    if !bounds.is_valid_non_empty() {
                        return false;
                    }
                }

                true
            }
            ConcreteType::Array(arr_box) => {
                let (content, size) = arr_box.deref();

                if size.get().is_none() {
                    return false;
                }

                let size = size.unwrap_integer();
                content.is_valid() && size >= &IBig::from(0)
            }
        }
    }

    #[track_caller]
    pub fn unwrap_int_bounds(&self) -> IntBounds<&IBig> {
        let ConcreteType::Named(v) = self else {
            unreachable!("unwrap_integer_bounds")
        };
        v.unwrap_int_bounds()
    }

    #[track_caller]
    pub fn unwrap_int_bounds_unknown(&self) -> IntBounds<&UniCell<Value>> {
        let ConcreteType::Named(v) = self else {
            unreachable!("unwrap_integer_bounds_unknown")
        };
        v.unwrap_int_bounds_unknown()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
/// Inclusive lower bound, exclusive upper bound
pub struct IntBounds<T> {
    pub from: T,
    pub to: T,
}

impl IntBounds<&'_ IBig> {
    pub fn is_valid(self) -> bool {
        self.from <= self.to
    }
    pub fn is_valid_non_empty(self) -> bool {
        self.from < self.to
    }
    pub fn is_empty(self) -> bool {
        assert!(self.is_valid());
        self.from == self.to
    }
    pub fn is_signed(self) -> bool {
        assert!(self.is_valid_non_empty());
        self.from < &IBig::from(0)
    }
    pub fn bitwidth(self) -> u64 {
        assert!(!self.is_empty(), "{self}");
        let min = self.from;
        let max = self.to - IBig::from(1);
        if min < &IBig::from(0) {
            let min_abs: UBig = UBig::try_from(-min).unwrap() - 1;

            let bits_for_min = min_abs.bit_len();

            let bits_for_max = if max > IBig::from(0) {
                let max = UBig::try_from(max).unwrap();

                max.bit_len()
            } else {
                0
            };

            (usize::max(bits_for_min, bits_for_max) + 1) as u64
        } else {
            let max = UBig::try_from(max).unwrap();

            max.bit_len() as u64
        }
    }
    pub fn contains(self, idx: &IBig) -> bool {
        assert!(self.is_valid());
        idx >= self.from && idx < self.to
    }
    pub fn contains_bounds(self, other: IntBounds<&IBig>) -> bool {
        assert!(self.is_valid());
        other.from >= self.from && other.to <= self.to
    }
}

impl std::fmt::Display for IntBounds<&'_ IBig> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let IntBounds { from, to } = self;
        write!(f, "{from}:{to}")
    }
}

impl ConcreteGlobalReference<TypeUUID> {
    #[track_caller]
    pub fn unwrap_int_bounds(&self) -> IntBounds<&IBig> {
        assert_eq!(self.id, get_builtin_type!("int"));
        let [from, to] = self.template_args.cast_to_int_array();
        IntBounds { from, to }
    }

    #[track_caller]
    pub fn unwrap_int_bounds_unknown(&self) -> IntBounds<&UniCell<Value>> {
        assert_eq!(self.id, get_builtin_type!("int"));
        let [from, to] = self.template_args.cast_to_unifyable_array();
        IntBounds { from, to }
    }
}

impl<ID: Into<GlobalUUID> + Copy> ConcreteGlobalReference<ID> {
    pub fn find_invalid_template_args(&self) -> Vec<TemplateID> {
        let mut failures = Vec::new();
        for (id, arg) in &self.template_args {
            let is_okay = match arg {
                TemplateKind::Type(t) => t.is_valid(),
                TemplateKind::Value(v) => v.get().is_some(),
            };
            if !is_okay {
                failures.push(id);
            }
        }
        failures
    }

    pub fn report_if_errors(&self, globals: &LinkerGlobals, context: &str) -> Result<(), String> {
        let error_parameters = self.find_invalid_template_args();
        if !error_parameters.is_empty() {
            let error_params_disp = display_join(", ", &error_parameters, |f, id| {
                let param_name = &globals[self.id.into()].parameters[*id].name;
                write!(f, "'{param_name}'")
            });
            Err(format!(
                "{context}. The arguments {error_params_disp} were not valid",
            ))
        } else {
            Ok(())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[rustfmt::skip]
    fn test_bound_to_bits() {
        assert_eq!(IntBounds{from: &IBig::from(-1), to: &IBig::from(1)}.bitwidth(), 1);
        assert_eq!(IntBounds{from: &IBig::from(-2), to: &IBig::from(1)}.bitwidth(), 2);
        assert_eq!(IntBounds{from: &IBig::from(-1), to: &IBig::from(2)}.bitwidth(), 2);
        assert_eq!(IntBounds{from: &IBig::from(-2), to: &IBig::from(3)}.bitwidth(), 3);
        assert_eq!(IntBounds{from: &IBig::from(2), to: &IBig::from(9)}.bitwidth(), 4);
        assert_eq!(IntBounds{from: &IBig::from(-1000), to: &IBig::from(1)}.bitwidth(), 11);
        assert_eq!(IntBounds{from: &IBig::from(-2000), to: &IBig::from(-999)}.bitwidth(), 12);
        assert_eq!(IntBounds{from: &IBig::from(-256), to: &IBig::from(256)}.bitwidth(), 9);
        assert_eq!(IntBounds{from: &IBig::from(0), to: &IBig::from(256)}.bitwidth(), 8);
        assert_eq!(IntBounds{from: &IBig::from(20), to: &IBig::from(257)}.bitwidth(), 9);
        assert_eq!(IntBounds{from: &IBig::from(0), to: &IBig::from(1)}.bitwidth(), 0); // Zero sized wires are now possible
    }
}
