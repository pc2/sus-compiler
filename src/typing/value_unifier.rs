use std::ops::{Deref, DerefMut};

use ibig::IBig;
use sus_proc_macro::get_builtin_type;

use crate::{
    alloc::zip_eq,
    prelude::*,
    typing::{abstract_type::AbstractInnerType, template::TVec},
    value::Value,
};

use super::{
    abstract_type::AbstractRankedType,
    concrete_type::{ConcreteGlobalReference, ConcreteTemplateArg, ConcreteType},
    template::TemplateKind,
    type_inference::{
        ConcreteTypeVariableIDMarker, SetUnifier, Substitutor, TypeUnifier, UnifyResult, Unifyable,
    },
};

pub type UnifyableValue = Unifyable<Value, ConcreteTypeVariableIDMarker>;
pub type ValueSetUnifier = SetUnifier<Value, ConcreteTypeVariableIDMarker>;
pub type ValueUnifier = TypeUnifier<ValueSetUnifier>;

impl From<Value> for UnifyableValue {
    fn from(val: Value) -> Self {
        assert!(
            !matches!(val, Value::Unset),
            "Compiletime Value MUST be set before use in Type Unification"
        );
        Unifyable::Set(val)
    }
}

impl Substitutor for ValueSetUnifier {
    type MyType = ConcreteType;

    fn unify_total(&mut self, from: &ConcreteType, to: &ConcreteType) -> UnifyResult {
        match (from, to) {
            (ConcreteType::Named(a), ConcreteType::Named(b)) => {
                assert_eq!(a.id, b.id);
                let mut success = UnifyResult::Success;
                for (_, a, b) in zip_eq(&a.template_args, &b.template_args) {
                    success &= match a.and_by_ref(b) {
                        TemplateKind::Type((a, b)) => self.unify_total(a, b),
                        TemplateKind::Value((a, b)) => {
                            if self.unify(a, b) {
                                UnifyResult::Success
                            } else {
                                UnifyResult::NoMatchingTypeFunc
                            }
                        }
                    };
                }
                success
            }
            (ConcreteType::Array(a), ConcreteType::Array(b)) => {
                let (a_content, a_sz) = a.deref();
                let (b_content, b_sz) = b.deref();

                self.unify_total(a_content, b_content)
                    & if self.unify(a_sz, b_sz) {
                        UnifyResult::Success
                    } else {
                        UnifyResult::NoMatchingTypeFunc
                    }
            }
            (ConcreteType::Named(_), ConcreteType::Array(_))
            | (ConcreteType::Array(_), ConcreteType::Named(_)) => {
                unreachable!("Caught by abstract typecheck")
            }
        }
    }

    fn fully_substitute(&self, typ: &mut ConcreteType) -> bool {
        match typ {
            ConcreteType::Named(name) => {
                let mut success = true;
                for (_, arg) in &mut name.template_args {
                    success &= match arg {
                        TemplateKind::Type(t) => self.fully_substitute(t),
                        TemplateKind::Value(v) => v.substitute(self),
                    };
                }
                success
            }
            ConcreteType::Array(arr) => {
                let (content, sz) = arr.deref_mut();
                self.fully_substitute(content) & sz.substitute(self)
            }
        }
    }
}

impl ValueSetUnifier {
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
        unifier: &mut ValueUnifier,
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
                                let vp = v + 1;
                                if vp > *max {
                                    *max = vp;
                                }
                            } else {
                                result_args = Some(TVec::from_vec(vec![
                                    TemplateKind::Value(Value::Integer(v.clone()).into()),
                                    TemplateKind::Value(Value::Integer(v + 1).into()),
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
                            TemplateKind::Value(_) => TemplateKind::Value(unifier.alloc_unknown())
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

impl ConcreteType {
    pub fn fully_substitute(&mut self, unifier: &ValueUnifier) -> bool {
        match self {
            ConcreteType::Named(concrete_global_reference) => concrete_global_reference
                .template_args
                .iter_mut()
                .all(|(_, arg)| arg.fully_substitute(unifier)),
            ConcreteType::Array(arr_box) => {
                let (content, sz) = arr_box.deref_mut();
                content.fully_substitute(unifier) & sz.substitute(unifier)
            }
        }
    }
    pub fn try_fully_substitute(&self, unifier: &ValueUnifier) -> Option<Self> {
        let mut self_clone = self.clone();
        if self_clone.fully_substitute(unifier) {
            Some(self_clone)
        } else {
            None
        }
    }
}

impl ConcreteTemplateArg {
    pub fn fully_substitute(&mut self, unifier: &ValueUnifier) -> bool {
        match self {
            TemplateKind::Type(t) => t.fully_substitute(unifier),
            TemplateKind::Value(v) => v.substitute(unifier),
        }
    }
}
