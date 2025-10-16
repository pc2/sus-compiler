use std::{
    collections::HashMap,
    ops::{Deref, DerefMut},
};

use ibig::IBig;
use sus_proc_macro::get_builtin_type;

use crate::{
    let_unwrap,
    prelude::*,
    typing::{
        abstract_type::{AbstractGlobalReference, AbstractInnerType},
        concrete_type::SubtypeRelation,
        template::TVec,
    },
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
    /// Unifies all [UnifyableValue] parameters contained in the [ConcreteType]. This includes values in subtyping relations [SubtypeRelation::Min] and [SubtypeRelation::Max].
    pub fn unify_concrete_all(&mut self, from: &ConcreteType, to: &ConcreteType) -> bool {
        let mut success = true;
        ConcreteType::co_iterate_parameters(from, to, &mut |f, t, _relation| {
            success &= self.unify(f, t);
        });
        success
    }

    /// Unifies all [UnifyableValue] parameters contained in the [ConcreteType]. This only includes [SubtypeRelation::Exact]
    pub fn unify_concrete_only_exact(&mut self, from: &ConcreteType, to: &ConcreteType) -> bool {
        let mut success = true;
        ConcreteType::co_iterate_parameters(from, to, &mut |f, t, relation| {
            if relation == SubtypeRelation::Exact {
                success &= self.unify(f, t);
            }
        });
        success
    }

    /// Gathers values for subtype relations for a's parameters
    fn unify_gather_subtype_relations<'a>(
        &mut self,
        a: &'a ConcreteType,
        b: &'a ConcreteType,
        source_gather: &mut SubTypeSourceGatherer<'_, 'a>,
    ) {
        ConcreteType::co_iterate_parameters(a, b, &mut |a, b, relation| match relation {
            SubtypeRelation::Exact => {
                // Errors are reported by final_checks
                let _ = self.unify(a, b);
            }
            SubtypeRelation::Min => source_gather.add_relation(ValueUnificationRelation::Min, a, b),
            SubtypeRelation::Max => source_gather.add_relation(ValueUnificationRelation::Max, a, b),
        });
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
            self.unify_gather_subtype_relations(to_typ, from_typ, &mut source_gather);
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

                        unifier
                            .set(var_sources.target, Value::Integer(common_subtype))
                            .expect("Values used in subtyping relations are always resolved in a forward direction (so a value b that depends on value a only gets resolved after a is resolved) That's why we can safely assert");
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

impl Value {
    /// Traverses the Value, to create a [ConcreteType] for it, guided by the abstract type given.
    /// So '1' becomes `ConcreteType::Named(ConcreteGlobalReference{id: get_builtin_type!("int"), ...}})`,
    /// but `Value::Array([])` becomes `ConcreteType::Array((ConcreteType::Unknown, 0))`
    ///
    /// Panics when arrays contain mutually incompatible types
    pub fn concretize_type(
        &self,
        _linker: &Linker,
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
            AbstractInnerType::Named(AbstractGlobalReference {
                id: get_builtin_type!("bool"),
                ..
            }) => {
                self.get_tensor_size_recursive(0, array_depth, &mut tensor_sizes, &mut |v| {
                    match v {
                        Value::Bool(_) => {}
                        Value::Integer(_) | Value::Float(_) | Value::Double(_) => {
                            unreachable!("Caught by abstract typecheck");
                        }
                        Value::Array(_) => {
                            unreachable!("All arrays handled by get_tensor_size_recursive");
                        }
                        Value::Unset => {
                            return Err("This compile-time constant contains Unset".into());
                        }
                    }
                    Ok(())
                })?;
                ConcreteType::Named(ConcreteGlobalReference {
                    id: get_builtin_type!("bool"),
                    template_args: FlatAlloc::new(),
                })
            }
            AbstractInnerType::Named(AbstractGlobalReference {
                id: get_builtin_type!("float"),
                ..
            }) => {
                self.get_tensor_size_recursive(0, array_depth, &mut tensor_sizes, &mut |v| {
                    match v {
                        Value::Float(_) => {}
                        Value::Integer(_) | Value::Bool(_) | Value::Double(_) => {
                            unreachable!("Caught by abstract typecheck");
                        }
                        Value::Array(_) => {
                            unreachable!("All arrays handled by get_tensor_size_recursive");
                        }
                        Value::Unset => {
                            return Err("This compile-time constant contains Unset".into());
                        }
                    }
                    Ok(())
                })?;
                ConcreteType::Named(ConcreteGlobalReference {
                    id: get_builtin_type!("float"),
                    template_args: FlatAlloc::new(),
                })
            }
            AbstractInnerType::Named(AbstractGlobalReference {
                id: get_builtin_type!("double"),
                ..
            }) => {
                self.get_tensor_size_recursive(0, array_depth, &mut tensor_sizes, &mut |v| {
                    match v {
                        Value::Double(_) => {}
                        Value::Integer(_) | Value::Bool(_) | Value::Float(_) => {
                            unreachable!("Caught by abstract typecheck");
                        }
                        Value::Array(_) => {
                            unreachable!("All arrays handled by get_tensor_size_recursive");
                        }
                        Value::Unset => {
                            return Err("This compile-time constant contains Unset".into());
                        }
                    }
                    Ok(())
                })?;
                ConcreteType::Named(ConcreteGlobalReference {
                    id: get_builtin_type!("double"),
                    template_args: FlatAlloc::new(),
                })
            }
            AbstractInnerType::Named(AbstractGlobalReference {
                id: get_builtin_type!("int"),
                ..
            }) => {
                let mut min_max: Option<(&IBig, &IBig)> = None;

                self.get_tensor_size_recursive(0, array_depth, &mut tensor_sizes, &mut |v| {
                    match v {
                        Value::Integer(v) => {
                            if let Some((min, max)) = &mut min_max {
                                if v < min {
                                    *min = v;
                                }
                                if v > max {
                                    *max = v;
                                }
                            } else {
                                min_max = Some((v, v))
                            }
                        }
                        Value::Bool(_) | Value::Float(_) | Value::Double(_) => {
                            unreachable!("Caught by abstract typecheck");
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

                let template_args: TVec<ConcreteTemplateArg> =
                    FlatAlloc::from_vec(if let Some((min, max)) = min_max {
                        vec![
                            TemplateKind::Value(Value::Integer(min.clone()).into()),
                            TemplateKind::Value(Value::Integer(max + 1).into()),
                        ]
                    } else {
                        vec![
                            TemplateKind::Value(value_alloc.alloc_unknown()),
                            TemplateKind::Value(value_alloc.alloc_unknown()),
                        ]
                    });
                ConcreteType::Named(ConcreteGlobalReference {
                    id: get_builtin_type!("int"),
                    template_args,
                })
            }
            AbstractInnerType::Named(AbstractGlobalReference { .. }) => {
                return Err("TODO: Structs".to_string()); // todo!("Structs")
            }
            AbstractInnerType::Unknown(_) => unreachable!("Caught by typecheck"),
            AbstractInnerType::Interface(_, _) | AbstractInnerType::LocalInterface(_) => {
                unreachable!(
                    "Interfaces can't be concretized, should have been caught by typecheck!"
                )
            }
        };

        assert!(tensor_sizes.len() <= array_depth);
        let mut array_size_vars: Vec<UnifyableValue> = Vec::with_capacity(array_depth);
        for sz in tensor_sizes {
            array_size_vars.push(Value::Integer(IBig::from(sz)).into());
        }
        // Because we might encounter zero sized arrays, we don't actually know sizes under there
        // Fill remaining array slots with Unknown
        while array_size_vars.len() < array_depth {
            array_size_vars.push(value_alloc.alloc_unknown());
        }
        Ok(content_typ.stack_arrays(array_size_vars))
    }
    fn get_tensor_size_recursive<'v>(
        &'v self,
        depth: usize,
        max_depth: usize,
        tensor_sizes: &mut Vec<usize>,
        elem_fn: &mut impl FnMut(&'v Value) -> Result<(), String>,
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
    fn can_fully_substitute(&self, substitutor: &ValueUnifierStore) -> bool {
        match self {
            ConcreteType::Named(global_ref) => {
                global_ref.template_args.can_fully_substitute(substitutor)
            }
            ConcreteType::Array(arr) => {
                let (content, sz) = arr.deref();
                content.can_fully_substitute(substitutor) & sz.can_fully_substitute(substitutor)
            }
        }
    }
}

impl FullySubstitutable<Value, ConcreteTypeVariableIDMarker> for TVec<ConcreteTemplateArg> {
    fn fully_substitute(&mut self, substitutor: &ValueUnifierStore) -> bool {
        self.iter_mut().all(|(_, arg)| match arg {
            TemplateKind::Type(t) => t.fully_substitute(substitutor),
            TemplateKind::Value(v) => v.fully_substitute(substitutor),
        })
    }
    fn can_fully_substitute(&self, substitutor: &ValueUnifierStore) -> bool {
        self.iter().all(|(_, arg)| match arg {
            TemplateKind::Type(t) => t.can_fully_substitute(substitutor),
            TemplateKind::Value(v) => v.can_fully_substitute(substitutor),
        })
    }
}

impl ConcreteType {
    pub fn get_value_args<const N: usize>(&self, expected: TypeUUID) -> [&UnifyableValue; N] {
        let_unwrap!(ConcreteType::Named(typ), &self);
        assert_eq!(typ.id, expected);
        typ.template_args
            .cast_to_array::<N>()
            .each_ref()
            .map(|v| v.unwrap_value())
    }
    pub fn display_substitute(&self, linker: &Linker, substitutor: &ValueUnifierStore) -> String {
        let mut typ_copy = self.clone();
        typ_copy.fully_substitute(substitutor);
        let as_display = typ_copy.display(linker);
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
