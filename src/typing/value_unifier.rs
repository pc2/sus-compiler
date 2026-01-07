use crate::prelude::*;

use std::{collections::HashMap, fmt::Display, ops::Deref};

use ibig::IBig;
use sus_proc_macro::get_builtin_type;

use crate::{
    linker::LinkerGlobals,
    typing::{
        abstract_type::AbstractRankedType,
        abstract_type::{AbstractGlobalReference, AbstractInnerType},
        concrete_type::SubtypeRelation,
        concrete_type::{ConcreteGlobalReference, ConcreteTemplateArg, ConcreteType},
        template::TVec,
        template::TemplateKind,
        unifyable_cell::{
            ResolveError, SubTree, SubstituteRecurse, Substitutor, UniCell, Unifier, UnifierTop,
            UnifierTopInfo, UnifyRecurse, UnifyResult,
        },
    },
    value::Value,
};

#[derive(Default)]
pub struct ValueUnifier<'s> {
    value_substitutor: Substitutor<'s, Value, Self>,
    unifier_top_info: UnifierTopInfo<'s, Self>,
}

impl<'s> ValueUnifier<'s> {
    pub fn new() -> Self {
        Self {
            value_substitutor: Substitutor::new(),
            unifier_top_info: UnifierTopInfo::new(),
        }
    }
}

impl ConcreteType {
    pub fn display_substitute<'s, 'obj: 's>(
        &'obj self,
        globals: &'obj LinkerGlobals,
        unifier: &ValueUnifier<'s>,
    ) -> impl Display + 'obj {
        let _ = unifier.fully_substitute_recurse(self);
        self.display(globals)
    }
}

impl<'s> UnifierTop<'s> for ValueUnifier<'s> {
    fn execute_ready_constraints(&self) {
        self.value_substitutor.execute_ready_constraints(self);
    }
    fn get_unifier_info(&self) -> &UnifierTopInfo<'s, Self> {
        &self.unifier_top_info
    }
}

fn check_values_equal(a: &Value, b: &Value) -> UnifyResult {
    assert!(!a.contains_unset());
    assert!(!b.contains_unset());
    let eq = match (a, b) {
        (Value::Bool(a), Value::Bool(b)) => a == b,
        (Value::Integer(a), Value::Integer(b)) => a == b,
        (Value::Float(a), Value::Float(b)) => a == b,
        (Value::Double(a), Value::Double(b)) => a == b,
        (Value::String(a), Value::String(b)) => a == b,
        (Value::Array(a), Value::Array(b)) => a == b,
        (Value::Unset, _) | (_, Value::Unset) => {
            unreachable!("Unsets can never make it into the type checker!")
        }
        _ => unreachable!(
            "Abstract Typing Error in Unifier? Should have been caught by Abstract Type Checker!"
        ),
    };
    UnifyResult::from(eq)
}
impl<'unif, 's: 'unif> SubstituteRecurse<'unif, 's, Value> for ValueUnifier<'s> {
    fn fully_substitute_recurse(
        &'unif self,
        _: &'s Value,
    ) -> Result<(), ResolveError<'unif, 's, Self>> {
        Ok(()) // No recursion
    }
}
impl<'unif, 's: 'unif> UnifyRecurse<'unif, 's, Value> for ValueUnifier<'s> {
    fn unify_subtrees(&'unif self, a: &'s Value, b: &'s Value) -> UnifyResult {
        check_values_equal(a, b)
    }

    fn set_subtrees(&'unif self, a: &'s Value, b: Value) -> UnifyResult {
        check_values_equal(a, &b)
    }

    fn clone_known(&'unif self, known: &'s Value) -> Value {
        known.clone() // No recursion
    }
}
impl<'unif, 's: 'unif> Unifier<'unif, 's, Value> for ValueUnifier<'s> {
    fn get_substitutor(&'unif self) -> &'unif Substitutor<'s, Value, Self> {
        &self.value_substitutor
    }
    fn contains_subtree(&'unif self, _: &Value, _: SubTree<Value>) -> bool {
        false // No recursion
    }
}

impl<'s> ValueUnifier<'s> {
    /// Unifies all [UniCell<Value>] parameters contained in the [ConcreteType]. This includes values in subtyping relations [SubtypeRelation::Min] and [SubtypeRelation::Max].
    pub fn unify_concrete_all(&self, from: &'s ConcreteType, to: &'s ConcreteType) -> bool {
        let mut success = true;
        ConcreteType::co_iterate_parameters(from, to, &mut |f, t, _relation| {
            success &= self.unify(f, t) == UnifyResult::Success;
        });
        success
    }

    /// Unifies all [UniCell<Value>] parameters contained in the [ConcreteType]. This only includes [SubtypeRelation::Exact]
    pub fn unify_concrete_only_exact(&self, from: &'s ConcreteType, to: &'s ConcreteType) -> bool {
        let mut success = true;
        ConcreteType::co_iterate_parameters(from, to, &mut |f, t, relation| {
            if relation == SubtypeRelation::Exact {
                success &= self.unify(f, t) == UnifyResult::Success;
            }
        });
        success
    }

    /// Gathers values for subtype relations for a's parameters
    fn unify_gather_subtype_relations(
        &self,
        a: &'s ConcreteType,
        b: &'s ConcreteType,
        source_gather: &mut SubTypeSourceGatherer<'_, 's>,
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
        &self,
        type_iter: impl IntoIterator<Item = (&'s ConcreteType, &'s ConcreteType)>,
    ) {
        let type_iter = type_iter.into_iter();
        let expected_num_targets = type_iter.size_hint().0;

        let mut source_gather_hashmap =
            HashMap::<*const UniCell<Value>, CommonSubtypeRelation<'s>>::new();

        for (to_typ, from_typ) in type_iter {
            let mut source_gather = SubTypeSourceGatherer {
                source_gather: &mut source_gather_hashmap,
                expected_num_targets,
            };
            self.unify_gather_subtype_relations(to_typ, from_typ, &mut source_gather);
        }

        for var_sources in source_gather_hashmap.into_values() {
            // Set means that it's specified! Because it was placed there directly by execute. Known values due to unifying are [Unifyable::Unknown] pointing to [KnownValue::Known]
            // Errors are reported by final_checks
            if var_sources.target.get().is_none() {
                self.delayed_constraint(move |unifier| {
                    let mut source_iter = var_sources.sources.iter();
                    let mut common_subtype = unifier.resolve(source_iter.next().unwrap())?.unwrap_integer();

                    for source in source_iter {
                        let v = unifier.resolve(*source)?.unwrap_integer();
                        match var_sources.relation {
                            ValueUnificationRelation::Min if v < common_subtype => {
                                common_subtype = v;
                            }
                            ValueUnificationRelation::Max if v > common_subtype => {
                                common_subtype = v;
                            }
                            _ => {}
                        }
                    }

                    unifier
                        .set(var_sources.target, Value::Integer(common_subtype.clone()))
                        .expect("Values used in subtyping relations are always resolved in a forward direction (so a value b that depends on value a only gets resolved after a is resolved) That's why we can safely assert");
                    Ok(())
                });
            }
        }
    }
}

pub struct CommonSubtypeRelation<'t> {
    pub target: &'t UniCell<Value>,
    pub relation: ValueUnificationRelation,
    pub sources: Vec<&'t UniCell<Value>>,
}

impl Value {
    /// Traverses the Value, to create a [ConcreteType] for it, guided by the abstract type given.
    /// So '1' becomes `ConcreteType::Named(ConcreteGlobalReference{id: get_builtin_type!("int"), ...}})`,
    /// but `Value::Array([])` becomes `ConcreteType::Array((ConcreteType::Unknown, 0))`
    ///
    /// Panics when arrays contain mutually incompatible types
    pub fn concretize_type(
        &self,
        _linker: &LinkerGlobals,
        abs_typ: &AbstractRankedType,
        template_args: &TVec<ConcreteTemplateArg>,
    ) -> Result<ConcreteType, String> {
        let array_depth = abs_typ.rank.count();
        let mut tensor_sizes = Vec::with_capacity(array_depth);

        let content_typ = match abs_typ.inner.unwrap() {
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
                        Value::Integer(_)
                        | Value::Float(_)
                        | Value::Double(_)
                        | Value::String(_) => {
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
                        Value::Integer(_)
                        | Value::Bool(_)
                        | Value::Double(_)
                        | Value::String(_) => {
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
                        Value::Integer(_) | Value::Bool(_) | Value::Float(_) | Value::String(_) => {
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
                id: get_builtin_type!("string"),
                ..
            }) => {
                self.get_tensor_size_recursive(0, array_depth, &mut tensor_sizes, &mut |v| {
                    match v {
                        Value::String(_) => {}
                        Value::Integer(_) | Value::Bool(_) | Value::Float(_) | Value::Double(_) => {
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
                    id: get_builtin_type!("string"),
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
                        Value::Bool(_) | Value::Float(_) | Value::Double(_) | Value::String(_) => {
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
                            TemplateKind::Value(Value::UNKNOWN),
                            TemplateKind::Value(Value::UNKNOWN),
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
            AbstractInnerType::Interface(_, _) | AbstractInnerType::LocalInterface(_) => {
                unreachable!(
                    "Interfaces can't be concretized, should have been caught by typecheck!"
                )
            }
        };

        assert!(tensor_sizes.len() <= array_depth);
        let mut array_size_vars: Vec<UniCell<Value>> = Vec::with_capacity(array_depth);
        for sz in tensor_sizes {
            array_size_vars.push(Value::Integer(IBig::from(sz)).into());
        }
        // Because we might encounter zero sized arrays, we don't actually know sizes under there
        // Fill remaining array slots with Unknown
        while array_size_vars.len() < array_depth {
            array_size_vars.push(Value::UNKNOWN);
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

impl<'unif, 's: 'unif> SubstituteRecurse<'unif, 's, ConcreteType> for ValueUnifier<'s> {
    fn fully_substitute_recurse(
        &'unif self,
        v: &'s ConcreteType,
    ) -> Result<(), ResolveError<'unif, 's, Self>> {
        match v {
            ConcreteType::Named(global_ref) => {
                self.fully_substitute_recurse(&global_ref.template_args)
            }
            ConcreteType::Array(arr) => {
                let (content, sz) = arr.deref();
                self.fully_substitute_recurse(content)
                    .and(self.fully_substitute(sz))
            }
        }
    }
}
impl<'unif, 's: 'unif> SubstituteRecurse<'unif, 's, TVec<ConcreteTemplateArg>>
    for ValueUnifier<'s>
{
    fn fully_substitute_recurse(
        &'unif self,
        v: &'s TVec<ConcreteTemplateArg>,
    ) -> Result<(), ResolveError<'unif, 's, Self>> {
        let mut total_result = Ok(());
        for (_, arg) in v {
            total_result = total_result.and(match arg {
                TemplateKind::Type(t) => self.fully_substitute_recurse(t),
                TemplateKind::Value(v) => self.fully_substitute(v),
            })
        }
        total_result
    }
}

impl<'unif, 's: 'unif, ID> SubstituteRecurse<'unif, 's, ConcreteGlobalReference<ID>>
    for ValueUnifier<'s>
{
    fn fully_substitute_recurse(
        &'unif self,
        v: &'s ConcreteGlobalReference<ID>,
    ) -> Result<(), ResolveError<'unif, 's, Self>> {
        self.fully_substitute_recurse(&v.template_args)
    }
}

impl ConcreteType {
    pub fn get_value_args<const N: usize>(&self, expected: TypeUUID) -> [&UniCell<Value>; N] {
        let_unwrap!(ConcreteType::Named(typ), &self);
        assert_eq!(typ.id, expected);
        typ.template_args
            .cast_to_array::<N>()
            .each_ref()
            .map(|v| v.unwrap_value())
    }
}

struct SubTypeSourceGatherer<'hm, 'a> {
    source_gather: &'hm mut HashMap<*const UniCell<Value>, CommonSubtypeRelation<'a>>,
    expected_num_targets: usize,
}

impl<'hm, 's> SubTypeSourceGatherer<'hm, 's> {
    fn add_relation(
        &mut self,
        relation: ValueUnificationRelation,
        target: &'s UniCell<Value>,
        value: &'s UniCell<Value>,
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
