use std::{
    collections::HashMap,
    ops::{Deref, DerefMut},
};

use ibig::IBig;
use sus_proc_macro::get_builtin_type;

use crate::{
    let_unwrap,
    prelude::*,
    typing::{abstract_type::AbstractInnerType, template::TVec},
    util::all_equal,
    value::Value,
};

use super::{
    abstract_type::AbstractRankedType,
    concrete_type::{ConcreteGlobalReference, ConcreteTemplateArg, ConcreteType},
    set_unifier::{
        DelayedErrorCollector, FullySubstitutable, SetUnifier, SetUnifierStore, Unifyable,
        UnifyableAlloc,
    },
    template::TemplateKind,
    type_inference::ConcreteTypeVariableIDMarker,
};

pub type UnifyableValue = Unifyable<Value, ConcreteTypeVariableIDMarker>;
pub type ValueUnifierAlloc = UnifyableAlloc<Value, ConcreteTypeVariableIDMarker>;
pub type ValueUnifierStore = SetUnifierStore<Value, ConcreteTypeVariableIDMarker>;
pub type ValueUnifier<'inst> = SetUnifier<'inst, Value, ConcreteTypeVariableIDMarker>;
pub type ValueErrorReporter<'inst> =
    DelayedErrorCollector<'inst, Value, ConcreteTypeVariableIDMarker>;

impl From<Value> for UnifyableValue {
    fn from(val: Value) -> Self {
        assert!(
            !matches!(val, Value::Unset),
            "Compiletime Value MUST be set before use in Type Unification"
        );
        Unifyable::Set(val)
    }
}

impl<'inst> ValueUnifier<'inst> {
    /// Unifies all [UnifyableValue] parameters contained in the [ConcreteType]s that are not used in subtyping relations.
    /// Parameters that are involved in subtyping relations (like int::MIN & int::MAX) are ignored. Retrieve these with
    /// [ConcreteType::retreive_unifyable_parameters]
    pub fn unify_concrete<const UNIFY_EVERYTHING: bool>(
        &mut self,
        from: &ConcreteType,
        to: &ConcreteType,
    ) -> bool {
        match (from, to) {
            (ConcreteType::Named(a), ConcreteType::Named(b)) => {
                assert_eq!(a.id, b.id);
                if !UNIFY_EVERYTHING && a.id == get_builtin_type!("int") {
                    // Int args are part of subtyping.
                    return true;
                }
                let mut success = true;
                for (_, a, b) in crate::alloc::zip_eq(&a.template_args, &b.template_args) {
                    success &= match a.and_by_ref(b) {
                        TemplateKind::Type((a, b)) => self.unify_concrete::<UNIFY_EVERYTHING>(a, b),
                        TemplateKind::Value((a, b)) => self.unify(a, b),
                    };
                }
                success
            }
            (ConcreteType::Array(a), ConcreteType::Array(b)) => {
                let (a_content, a_sz) = a.deref();
                let (b_content, b_sz) = b.deref();

                self.unify_concrete::<UNIFY_EVERYTHING>(a_content, b_content)
                    & self.unify(a_sz, b_sz)
            }
            (ConcreteType::Named(_), ConcreteType::Array(_))
            | (ConcreteType::Array(_), ConcreteType::Named(_)) => {
                unreachable!("Caught by abstract typecheck")
            }
        }
    }

    /// Gathers values for subtype relations for a's parameters
    fn unify_gather_subtype_relations<'a>(
        &mut self,
        a: &'a ConcreteType,
        b: &'a ConcreteType,
        source_gather: &mut SubTypeSourceGatherer<'_, 'a>,
    ) -> bool {
        let mut unify_success = true;
        match (a, b) {
            (ConcreteType::Named(global_ref_a), ConcreteType::Named(global_ref_b)) => {
                match all_equal([global_ref_a.id, global_ref_b.id]) {
                    get_builtin_type!("int") => {
                        let [min_a, max_a] = global_ref_a.template_args.cast_to_unifyable_array();
                        let [min_b, max_b] = global_ref_b.template_args.cast_to_unifyable_array();
                        source_gather.add_relation(ValueUnificationRelation::Min, min_a, min_b);
                        source_gather.add_relation(ValueUnificationRelation::Max, max_a, max_b);
                    }
                    _ => {
                        for (_, arg_a, arg_b) in crate::alloc::zip_eq(
                            &global_ref_a.template_args,
                            &global_ref_b.template_args,
                        ) {
                            unify_success &= match arg_a.and_by_ref(arg_b) {
                                TemplateKind::Type((t_a, t_b)) => {
                                    self.unify_gather_subtype_relations(t_a, t_b, source_gather)
                                }
                                TemplateKind::Value((v_a, v_b)) => self.unify(v_a, v_b),
                            }
                        }
                    }
                }
            }
            (ConcreteType::Array(arr_box_a), ConcreteType::Array(arr_box_b)) => {
                let (content_a, sz_a) = arr_box_a.deref();
                let (content_b, sz_b) = arr_box_b.deref();
                unify_success &= self.unify(sz_a, sz_b);
                self.unify_gather_subtype_relations(content_a, content_b, source_gather);
            }
            _ => unreachable!("Caught by typecheck"),
        }
        unify_success
    }

    /// In type_iter: The first type represents the target, the second type represents the source
    pub fn create_subtype_constraint(
        &mut self,
        type_iter: impl IntoIterator<Item = (&'inst ConcreteType, &'inst ConcreteType)>,
    ) {
        let type_iter = type_iter.into_iter();
        let expected_num_targets = type_iter.size_hint().0;

        let mut source_gather_hashmap =
            HashMap::<*const UnifyableValue, CommonSubtypeRelation<'inst>>::new();

        for (to_typ, from_typ) in type_iter {
            let mut source_gather = SubTypeSourceGatherer {
                source_gather: &mut source_gather_hashmap,
                expected_num_targets,
            };
            // Errors are reported by final_checks
            let _ = self.unify_gather_subtype_relations(to_typ, from_typ, &mut source_gather);
        }

        for var_sources in source_gather_hashmap.into_values() {
            match var_sources.target {
                // Set means that it's specified! Because it was placed there directly by execute. Known values due to unifying are [Unifyable::Unknown] pointing to [KnownValue::Known]
                // Errors are reported by final_checks
                Unifyable::Set(_) => {}
                Unifyable::Unknown(_) => {
                    let reservation = self.reserve_constraint(var_sources.sources.iter().copied());
                    self.place_reserved_constraint(reservation, move |unifier| {
                        let source_iter = var_sources
                            .sources
                            .into_iter()
                            .map(|src| unifier.unwrap_known(src).unwrap_integer());

                        let common_subtype = match var_sources.relation {
                            ValueUnificationRelation::Min => source_iter.min(),
                            ValueUnificationRelation::Max => source_iter.max(),
                        };
                        // We can simply unwrap, because a source only appears in the HashMap if it's actually encountered, and thus at least one other var matches with it!
                        let common_subtype = common_subtype.unwrap().clone();

                        // Values used in subtyping relations are always resolved in a forward direction (so a value b that depends on value a only gets resolved after a is resolved)
                        // That's why we can safely call unwrap()
                        unifier
                            .set(var_sources.target, Value::Integer(common_subtype))
                            .unwrap();
                    });
                }
            }
        }
    }
}

pub struct CommonSubtypeRelation<'t> {
    pub target: &'t UnifyableValue,
    pub relation: ValueUnificationRelation,
    pub sources: Vec<&'t UnifyableValue>,
}

impl ValueUnifierAlloc {
    pub fn make_array_of(&mut self, content_typ: ConcreteType) -> ConcreteType {
        ConcreteType::Array(Box::new((content_typ, self.alloc_unknown())))
    }
    fn mk_int_maybe(&mut self, v: Option<IBig>) -> TemplateKind<ConcreteType, UnifyableValue> {
        TemplateKind::Value(match v {
            Some(v) => Value::Integer(v).into(),
            None => self.alloc_unknown(),
        })
    }
    /// Creates a new `int #(int MIN, int MAX)`. The resulting int can have a value from `MIN` to `MAX-1`
    pub fn new_int_type(&mut self, min: Option<IBig>, max: Option<IBig>) -> ConcreteType {
        let template_args =
            FlatAlloc::from_vec(vec![self.mk_int_maybe(min), self.mk_int_maybe(max)]);

        ConcreteType::Named(ConcreteGlobalReference {
            id: get_builtin_type!("int"),
            template_args,
        })
    }
}

impl Value {
    /// Returns None for Unset
    pub fn get_type_id(&self) -> TypeUUID {
        match self {
            Value::Bool(_) => get_builtin_type!("bool"),
            Value::Integer(_) => get_builtin_type!("int"),
            Value::Array(_) => unreachable!("Value::get_type_abs is only ever used for terminal Values, because any array instantiations would be Expression::ArrayConstruct"),
            Value::Unset => unreachable!(),
        }
    }

    /// Traverses the Value, to create a [ConcreteType] for it, guided by the abstract type given.
    /// So '1' becomes `ConcreteType::Named(ConcreteGlobalReference{id: get_builtin_type!("int"), ...}})`,
    /// but `Value::Array([])` becomes `ConcreteType::Array((ConcreteType::Unknown, 0))`
    ///
    /// Panics when arrays contain mutually incompatible types
    pub fn concretize_type(
        &self,
        linker: &Linker,
        abs_typ: &AbstractRankedType,
        template_args: &TVec<ConcreteTemplateArg>,
        value_alloc: &mut ValueUnifierAlloc,
    ) -> Result<ConcreteType, String> {
        let array_depth = abs_typ.rank.count().unwrap();
        let mut tensor_sizes = Vec::with_capacity(array_depth);

        let content_typ = match &abs_typ.inner {
            AbstractInnerType::Template(template_id) => {
                self.get_tensor_size_recursive(0, array_depth, &mut tensor_sizes, &mut |_| Ok(()))?;
                template_args[*template_id].unwrap_type().clone()
            }
            AbstractInnerType::Named(content_typ_id) => {
                let mut result_args: Option<TVec<ConcreteTemplateArg>> = None;

                self.get_tensor_size_recursive(0, array_depth, &mut tensor_sizes, &mut |v| {
                    match v {
                        Value::Bool(_) => {
                            assert_eq!(*content_typ_id, get_builtin_type!("bool"));
                        }
                        Value::Integer(v) => {
                            assert_eq!(*content_typ_id, get_builtin_type!("int"));
                            if let Some(args) = &mut result_args {
                                let [min, max] = args.cast_to_int_array_mut();
                                if v < min {
                                    *min = v.clone();
                                }
                                if v > max {
                                    *max = v.clone();
                                }
                            } else {
                                result_args = Some(TVec::from_vec(vec![
                                    TemplateKind::Value(Value::Integer(v.clone()).into()),
                                    TemplateKind::Value(Value::Integer(v.clone()).into()),
                                ]))
                            }
                        }
                        Value::Array(_) => {
                            unreachable!("All arrays handled by get_tensor_size_recursive")
                        }
                        Value::Unset => {
                            return Err("This compile-time constant contains Unset".into());
                        }
                    }
                    Ok(())
                })?;

                ConcreteType::Named(ConcreteGlobalReference {
                    id: *content_typ_id,
                    template_args: match result_args {
                        Some(args) => args,
                        None => linker.types[*content_typ_id].link_info.template_parameters.map(|(_, param)| match &param.kind {
                            TemplateKind::Type(_) => todo!("Should extract type info from AbstractRankedType with specified args instead!"),
                            TemplateKind::Value(_) => TemplateKind::Value(value_alloc.alloc_unknown())
                        }),
                    },
                })
            }
            AbstractInnerType::Unknown(_) => unreachable!("Caught by typecheck"),
        };

        Ok(content_typ.stack_arrays_usize(&tensor_sizes))
    }
    fn get_tensor_size_recursive(
        &self,
        depth: usize,
        max_depth: usize,
        tensor_sizes: &mut Vec<usize>,
        elem_fn: &mut impl FnMut(&Value) -> Result<(), String>,
    ) -> Result<(), String> {
        if depth == max_depth {
            elem_fn(self)
        } else {
            let Value::Array(values) = self else {
                unreachable!()
            };
            if let Some(sz) = tensor_sizes.get(depth) {
                if *sz != values.len() {
                    return Err("Value is a Jagged Tensor. This is not allowed!".into());
                }
            } else {
                assert!(tensor_sizes.len() == depth);
                tensor_sizes.push(values.len());
            }
            for v in values {
                v.get_tensor_size_recursive(depth + 1, max_depth, tensor_sizes, elem_fn)?;
            }
            Ok(())
        }
    }
}

impl FullySubstitutable<Value, ConcreteTypeVariableIDMarker> for ConcreteType {
    fn gather_all_substitutables<'slf>(&'slf self, gather_here: &mut Vec<&'slf UnifyableValue>) {
        match self {
            ConcreteType::Named(global_ref) => global_ref
                .template_args
                .gather_all_substitutables(gather_here),
            ConcreteType::Array(arr) => {
                let (content, sz) = arr.deref();
                gather_here.push(sz);
                content.gather_all_substitutables(gather_here);
            }
        }
    }
    fn fully_substitute(&mut self, substitutor: &ValueUnifierStore) -> bool {
        match self {
            ConcreteType::Named(global_ref) => {
                global_ref.template_args.fully_substitute(substitutor)
            }
            ConcreteType::Array(arr) => {
                let (content, sz) = arr.deref_mut();
                content.fully_substitute(substitutor) & sz.fully_substitute(substitutor)
            }
        }
    }
}

impl FullySubstitutable<Value, ConcreteTypeVariableIDMarker> for TVec<ConcreteTemplateArg> {
    fn gather_all_substitutables<'slf>(&'slf self, gather_here: &mut Vec<&'slf UnifyableValue>) {
        for (_, arg) in self {
            match arg {
                TemplateKind::Type(t) => t.gather_all_substitutables(gather_here),
                TemplateKind::Value(v) => gather_here.push(v),
            }
        }
    }
    fn fully_substitute(&mut self, substitutor: &ValueUnifierStore) -> bool {
        self.iter_mut().all(|(_, arg)| match arg {
            TemplateKind::Type(t) => t.fully_substitute(substitutor),
            TemplateKind::Value(v) => v.fully_substitute(substitutor),
        })
    }
}

impl ConcreteType {
    pub fn get_value_args<const N: usize>(&self, expected: TypeUUID) -> [&UnifyableValue; N] {
        let_unwrap!(ConcreteType::Named(typ), &self);
        assert_eq!(typ.id, expected);
        typ.template_args
            .cast_to_array::<N>()
            .map(|v| v.unwrap_value())
    }
    pub fn set_named_template_args<const N: usize>(
        &self,
        expected: TypeUUID,
        unifier: &mut ValueUnifier<'_>,
        args: [impl Into<Value>; N],
    ) -> bool {
        let_unwrap!(ConcreteType::Named(typ), &self);
        assert_eq!(typ.id, expected);
        crate::util::zip_eq(typ.template_args.iter(), args)
            .all(|((_, to_unify), arg)| unifier.set(to_unify.unwrap_value(), arg.into()).is_ok())
    }
    pub fn new_named_with_args<const N: usize>(
        id: TypeUUID,
        args: [impl Into<ConcreteTemplateArg>; N],
    ) -> Self {
        ConcreteType::Named(ConcreteGlobalReference {
            id,
            template_args: FlatAlloc::from_vec(args.map(|a| a.into()).to_vec()),
        })
    }

    pub fn display_substitute(&self, linker: &Linker, substitutor: &ValueUnifierStore) -> String {
        let mut typ_copy = self.clone();
        typ_copy.fully_substitute(substitutor);
        let as_display = typ_copy.display(&linker.types, true);
        as_display.to_string()
    }
}

struct SubTypeSourceGatherer<'hm, 'a> {
    source_gather: &'hm mut HashMap<*const UnifyableValue, CommonSubtypeRelation<'a>>,
    expected_num_targets: usize,
}

impl<'hm, 'inst> SubTypeSourceGatherer<'hm, 'inst> {
    fn add_relation(
        &mut self,
        relation: ValueUnificationRelation,
        target: &'inst UnifyableValue,
        value: &'inst UnifyableValue,
    ) {
        let list = match self.source_gather.entry(target) {
            std::collections::hash_map::Entry::Occupied(occupied_entry) => {
                let occ = occupied_entry.into_mut();
                assert!(occ.relation == relation);
                occ
            }
            std::collections::hash_map::Entry::Vacant(vacant_entry) => {
                vacant_entry.insert(CommonSubtypeRelation {
                    target,
                    relation,
                    sources: Vec::with_capacity(self.expected_num_targets),
                })
            }
        };
        list.sources.push(value);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ValueUnificationRelation {
    Min,
    Max,
}
