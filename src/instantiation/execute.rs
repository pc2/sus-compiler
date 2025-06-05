//! Executes the generative code and produces a netlist from it
//!
//! Stops generating at the first error.
//!
//! As for typing, it only instantiates written types and leaves the rest for further typechecking.

use std::ops::{Deref, Index, IndexMut};

use crate::latency::CALCULATE_LATENCY_LATER;
use crate::linker::{GlobalUUID, IsExtern, LinkInfo};
use crate::prelude::*;
use crate::typing::abstract_type::{AbstractInnerType, AbstractRankedType, PeanoType};
use crate::typing::concrete_type::ConcreteTemplateArg;
use crate::typing::template::{GlobalReference, TVec, TemplateArg};
use crate::typing::written_type::WrittenType;
use crate::util::{unwrap_single_element, zip_eq};

use ibig::ops::{Abs, UnsignedAbs};
use ibig::{ibig, IBig, UBig};

use sus_proc_macro::get_builtin_const;

use crate::flattening::*;
use crate::value::{compute_binary_op, compute_unary_op, Value};

use crate::typing::{
    abstract_type::DomainType, concrete_type::ConcreteType, template::TemplateKind,
};

use super::*;

pub fn execute(
    link_info: &LinkInfo,
    linker: &Linker,
    working_on_template_args: &TVec<ConcreteTemplateArg>,
) -> Executed {
    let mut context = ExecutionContext {
        generation_state: GenerationState {
            link_info,
            generation_state: link_info
                .instructions
                .map(|(_, _)| SubModuleOrWire::Unnasigned),
        },
        type_substitutor: Default::default(),
        //type_value_substitutor: Default::default(),
        condition_stack: Vec::new(),
        wires: FlatAlloc::new(),
        submodules: FlatAlloc::new(),
        unique_name_producer: UniqueNames::new(),
        working_on_template_args,
        link_info,
        linker,
    };

    let execution_status = context.instantiate_code_block(link_info.instructions.id_range());

    Executed {
        wires: context.wires,
        submodules: context.submodules,
        type_var_alloc: context.type_substitutor,
        generation_state: context.generation_state.generation_state,
        execution_status,
    }
}

/// As with other contexts, this is the shared state we're lugging around while executing & typechecking a module.
struct ExecutionContext<'l> {
    wires: FlatAlloc<RealWire, WireIDMarker>,
    submodules: FlatAlloc<SubModule, SubModuleIDMarker>,
    type_substitutor: ValueUnifierAlloc,

    /// Used for Execution
    generation_state: GenerationState<'l>,
    unique_name_producer: UniqueNames,
    condition_stack: Vec<ConditionStackElem>,

    working_on_template_args: &'l TVec<ConcreteTemplateArg>,
    link_info: &'l LinkInfo,
    linker: &'l Linker,
}

macro_rules! caught_by_typecheck {
    ($arg:literal) => {
        panic!("{} should have been caught by typecheck!", $arg)
    };
    () => {
        panic!("Should have been caught by typecheck!")
    };
}

pub type ExecutionResult<T> = Result<T, (Span, String)>;

/// Every [crate::flattening::Instruction] has an associated value (See [SubModuleOrWire]).
/// They are either what this local name is currently referencing (either a wire instance or a submodule instance).
/// Or in the case of Generative values, the current value in the generative variable.
#[derive(Debug)]
struct GenerationState<'l> {
    generation_state: FlatAlloc<SubModuleOrWire, FlatIDMarker>,
    link_info: &'l LinkInfo,
}

impl GenerationState<'_> {
    fn span_of(&self, v: FlatID) -> Span {
        let instr = &self.link_info.instructions[v];
        match instr {
            Instruction::Declaration(d) => d.name_span,
            Instruction::Expression(expr) => expr.span,
            _ => unreachable!(),
        }
    }

    fn write_gen_variable(
        &self,
        target: &mut Value,
        conn_path: &[WireReferencePathElement],
        to_write: Value,
    ) -> ExecutionResult<()> {
        // must be an array, from earlier typechecking

        let mut cur_targets: Vec<(&mut Value, Value)> = vec![(target, to_write)];
        for path_elem in conn_path {
            let cur_targets_len = cur_targets.len();
            match path_elem {
                WireReferencePathElement::ArrayAccess {
                    idx,
                    bracket_span,
                    output_typ: _,
                } => {
                    let idx = self.get_generation_integer(*idx)?;
                    let Some(idx) = usize::try_from(idx).ok() else {
                        return Err((
                            bracket_span.inner_span(),
                            format!("Index {idx} must be > 0"),
                        ));
                    };
                    let old_targets =
                        std::mem::replace(&mut cur_targets, Vec::with_capacity(cur_targets_len));
                    for (to, _from) in old_targets {
                        // must be an array, from earlier typechecking
                        let Value::Array(t_values) = to else {
                            unreachable!()
                        };
                        let t_values_len = t_values.len();
                        let Some(p) = t_values.get_mut(idx) else {
                            return Err((
                                bracket_span.inner_span(),
                                format!("Index {idx} is out of bounds for this array of length {t_values_len}"),
                            ));
                        };
                        cur_targets.push((p, _from.clone()));
                    }
                }
                WireReferencePathElement::ArraySlice {
                    idx_a,
                    idx_b,
                    bracket_span,
                    output_typ: _,
                } => {
                    let start = match idx_a {
                        Some(idx) => self.get_generation_integer(*idx)?,
                        None => &ibig!(0),
                    };
                    let end = match idx_b {
                        Some(idx) => {
                            let end = self.get_generation_integer(*idx)?;
                            let Some(end) = usize::try_from(end).ok() else {
                                return Err((
                                    bracket_span.inner_span(),
                                    format!("End index {end} must be > 0"),
                                ));
                            };
                            Some(end)
                        }
                        None => None,
                    };

                    let Some(start) = usize::try_from(start).ok() else {
                        return Err((
                            bracket_span.inner_span(),
                            format!("Start index {start} must be > 0"),
                        ));
                    };
                    let capacity_elements = match end {
                        Some(end) => end - start,
                        None => 0, // inefficient, but fiddly to manage lifetimes to figure out the size here
                    };
                    let old_targets = std::mem::replace(
                        &mut cur_targets,
                        Vec::with_capacity(capacity_elements * cur_targets_len),
                    );
                    for (t, f) in old_targets {
                        // &mut [Value]
                        let Value::Array(t_values) = t else {
                            unreachable!()
                        }; // must be an array, from earlier typechecking
                           // Vec<Value>
                        let Value::Array(f_values) = f else {
                            unreachable!()
                        }; // must be an array, from earlier typechecking
                           //assert_eq!(end - start, f_values.len());
                        let t_values_len = t_values.len();
                        let p = match end {
                            Some(end) => {
                                let Some(g) = t_values.get_mut(start..end) else {
                                    return Err((
                                    bracket_span.inner_span(),
                                    format!("Slice {start}:{end} is out of bounds for this array of length {t_values_len}"),
                                ));
                                };
                                g
                            }
                            None => {
                                let Some(g) = t_values.get_mut(start..) else {
                                    return Err((
                                    bracket_span.inner_span(),
                                    format!("Slice {start}: is out of bounds for this array of length {t_values_len}"),
                                ));
                                };
                                g
                            }
                        };
                        for (tt, ff) in p.iter_mut().zip(f_values.iter()) {
                            cur_targets.push((tt, ff.clone()));
                        }
                    }
                }
                WireReferencePathElement::ArrayPartSelectDown {
                    idx_a,
                    width: idx_b,
                    bracket_span,
                    output_typ: _,
                }
                | WireReferencePathElement::ArrayPartSelectUp {
                    idx_a,
                    width: idx_b,
                    bracket_span,
                    output_typ: _,
                } => {
                    let start = self.get_generation_integer(*idx_a)?;
                    let idx_b = self.get_generation_integer(*idx_b)?;
                    let end = match path_elem {
                        WireReferencePathElement::ArraySlice { .. } => idx_b,
                        WireReferencePathElement::ArrayPartSelectDown { .. } => &(start - idx_b),
                        WireReferencePathElement::ArrayPartSelectUp { .. } => &(start + idx_b),
                        _ => unreachable!(),
                    };
                    let Some(end) = usize::try_from(end).ok() else {
                        return Err((
                            bracket_span.inner_span(),
                            format!("End index {end} must be > 0"),
                        ));
                    };

                    let Some(start) = usize::try_from(start).ok() else {
                        return Err((
                            bracket_span.inner_span(),
                            format!("Start index {start} must be > 0"),
                        ));
                    };
                    let old_targets = std::mem::replace(
                        &mut cur_targets,
                        Vec::with_capacity((end - start) * cur_targets_len),
                    );
                    for (t, f) in old_targets {
                        // &mut [Value]
                        let Value::Array(t_values) = t else {
                            unreachable!()
                        }; // must be an array, from earlier typechecking
                           // Vec<Value>
                        let Value::Array(f_values) = f else {
                            unreachable!()
                        }; // must be an array, from earlier typechecking
                        assert_eq!(end - start, f_values.len());
                        let t_values_len = t_values.len();
                        let Some(p) = t_values.get_mut(start..end) else {
                            return Err((
                                bracket_span.inner_span(),
                                format!("Slice {start}:{end} is out of bounds for this array of length {t_values_len}"),
                            ));
                        };
                        for (tt, ff) in p.iter_mut().zip(f_values.iter()) {
                            cur_targets.push((tt, ff.clone()));
                        }
                    }
                }
                WireReferencePathElement::Error => {}
            }
        }
        for (t, f) in cur_targets.into_boxed_slice() {
            *t = f;
        }
        Ok(())
    }
    fn get_generation_value(&self, v: FlatID) -> ExecutionResult<&Value> {
        let SubModuleOrWire::CompileTimeValue(vv) = &self.generation_state[v] else {
            unreachable!()
        };

        if let Value::Unset = vv {
            Err((self.span_of(v), "This variable is unset!".to_owned()))
        } else {
            Ok(vv)
        }
    }
    fn get_generation_integer(&self, idx: FlatID) -> ExecutionResult<&IBig> {
        let val = self.get_generation_value(idx)?;
        Ok(val.unwrap_integer())
    }
    fn get_generation_small_int<INT: for<'v> TryFrom<&'v IBig>>(
        &self,
        idx: FlatID,
    ) -> ExecutionResult<INT> {
        let val = self.get_generation_value(idx)?;
        let val_as_int = val.unwrap_integer();
        INT::try_from(val_as_int).map_err(|_| {
            (
                self.span_of(idx),
                format!(
                    "Value {val_as_int} does not fit in {}",
                    std::any::type_name::<INT>()
                ),
            )
        })
    }
}

impl Index<FlatID> for GenerationState<'_> {
    type Output = SubModuleOrWire;

    fn index(&self, index: FlatID) -> &Self::Output {
        &self.generation_state[index]
    }
}

impl IndexMut<FlatID> for GenerationState<'_> {
    fn index_mut(&mut self, index: FlatID) -> &mut Self::Output {
        &mut self.generation_state[index]
    }
}

fn array_access<'v>(
    arr_val: &'v Value,
    idx: &IBig,
    span: BracketSpan,
) -> ExecutionResult<&'v Value> {
    let Value::Array(arr) = arr_val else {
        caught_by_typecheck!("Value must be an array")
    };

    if let Some(elem) = usize::try_from(idx).ok().and_then(|idx| arr.get(idx)) {
        Ok(elem)
    } else {
        Err((
            span.outer_span(),
            format!(
                "Compile-Time Array index is out of range: idx: {idx}, array size: {}",
                arr.len()
            ),
        ))
    }
}

fn array_slice(
    arr_val: &mut Value,
    idx_a: &IBig,
    idx_b: Option<&IBig>,
    span: BracketSpan,
) -> ExecutionResult<Vec<Value>> {
    let Value::Array(a_box) = arr_val else {
        caught_by_typecheck!("Non-array")
    };

    let array_len = a_box.len();

    let Some(idx_a) = usize::try_from(idx_a).ok() else {
        return Err((
            span.inner_span(),
            format!(
                "Index {idx_a} is out of bounds for this array of size {}",
                array_len
            ),
        ));
    };

    let idx_b = match idx_b {
        Some(idx_b) => {
            let Some(g) = usize::try_from(idx_b).ok() else {
                return Err((
                    span.inner_span(),
                    format!(
                        "Index {idx_b} is out of bounds for this array of size {}",
                        array_len
                    ),
                ));
            };
            g
        }
        None => array_len - 1,
    };

    // make right sized empty vector of values
    let mut values = Vec::<Value>::with_capacity(idx_b - idx_a);

    for idx in idx_a..idx_b {
        let Some(tt) = a_box.get_mut(idx) else {
            return Err((
                span.inner_span(),
                format!(
                    "Index {idx} would be out of bounds for this array of size {}",
                    array_len
                ),
            ));
        };

        values.push(tt.clone());
    }

    Ok(values)
}

fn add_to_small_set<T: Eq>(set_vec: &mut Vec<T>, elem: T) {
    if !set_vec.contains(&elem) {
        set_vec.push(elem);
    }
}

fn must_be_positive(v: &IBig, subject: &str) -> Result<UBig, String> {
    UBig::try_from(v).map_err(|_| format!("{subject} must be positive! Found {v}"))
}
/// n! / (n - k)!
fn falling_factorial(mut n: UBig, k: &UBig) -> UBig {
    let num_terms = u64::try_from(k).unwrap();
    let mut result = ibig::ubig!(1);
    for _ in 0..num_terms {
        result *= &n;
        n -= 1usize;
    }
    result
}
fn factorial(mut n: UBig) -> UBig {
    let as_usize = usize::try_from(&n).unwrap();
    for v in 2..as_usize {
        n *= v;
    }
    n
}

/// Temporary intermediary struct
///
/// See [WireReferenceRoot]
#[derive(Debug, Clone)]
enum RealWireRefRoot<'t> {
    /// The preamble isn't really used yet, but it's there for when we have submodule arrays (soon)
    Wire {
        wire_id: WireID,
        preamble: Vec<RealWirePathElem>,
    },
    Generative(FlatID),
    Constant(Value, &'t AbstractRankedType),
}

trait Concretizer {
    fn get_type(&mut self, id: TemplateID) -> ConcreteType;
    fn get_value(&mut self, expr: FlatID) -> ExecutionResult<UnifyableValue>;
    fn alloc_unknown(&mut self) -> UnifyableValue;
}

struct LocalTypeConcretizer<'substitutor, 'linker> {
    template_args: &'linker TVec<ConcreteTemplateArg>,
    generation_state: &'linker GenerationState<'linker>,
    type_substitutor: &'substitutor mut ValueUnifierAlloc,
}
impl Concretizer for LocalTypeConcretizer<'_, '_> {
    fn get_type(&mut self, id: TemplateID) -> ConcreteType {
        self.template_args[id].unwrap_type().clone()
    }
    fn get_value(&mut self, expr: FlatID) -> ExecutionResult<UnifyableValue> {
        Ok(self
            .generation_state
            .get_generation_value(expr)?
            .clone()
            .into())
    }

    fn alloc_unknown(&mut self) -> UnifyableValue {
        self.type_substitutor.alloc_unknown()
    }
}
struct SubModuleTypeConcretizer<'substitutor, 'linker> {
    submodule_template_args: &'linker TVec<ConcreteTemplateArg>,
    instructions: &'linker FlatAlloc<Instruction, FlatIDMarker>,
    type_substitutor: &'substitutor mut ValueUnifierAlloc,
}
impl Concretizer for SubModuleTypeConcretizer<'_, '_> {
    fn get_type(&mut self, id: TemplateID) -> ConcreteType {
        self.submodule_template_args[id].unwrap_type().clone()
    }

    /// Part of Template Value Inference.
    ///
    /// Specifically, for code like this:
    ///
    /// ```sus
    /// module add_all #(int Size) {
    ///     input int[Size] arr // We're targeting the 'Size' within the array size
    ///     output int total
    /// }
    /// ```
    fn get_value(&mut self, expr: FlatID) -> ExecutionResult<UnifyableValue> {
        let expr = self.instructions[expr].unwrap_expression();
        Ok(match &expr.source {
            ExpressionSource::WireRef(wr) => {
                if !wr.path.is_empty() {
                    return Ok(self.type_substitutor.alloc_unknown());
                } // Must be a plain, no fuss reference to a de
                let WireReferenceRoot::LocalDecl(wire_declaration) = &wr.root else {
                    return Ok(self.type_substitutor.alloc_unknown());
                };
                let template_arg_decl = self.instructions[*wire_declaration].unwrap_declaration();
                let DeclarationKind::GenerativeInput(template_id) = &template_arg_decl.decl_kind
                else {
                    return Ok(self.type_substitutor.alloc_unknown());
                };
                self.submodule_template_args[*template_id]
                    .unwrap_value()
                    .clone()
            }
            ExpressionSource::Constant(cst) => cst.clone().into(),
            _ => self.type_substitutor.alloc_unknown(),
        })
    }

    fn alloc_unknown(&mut self) -> UnifyableValue {
        self.type_substitutor.alloc_unknown()
    }
}

fn concretize_type_recurse(
    linker: &Linker,
    inner: &AbstractInnerType,
    rank: &PeanoType,
    wr_typ: Option<&WrittenType>,
    concretizer: &mut impl Concretizer,
) -> ExecutionResult<ConcreteType> {
    Ok(match rank {
        PeanoType::Zero => match inner {
            AbstractInnerType::Template(id) => concretizer.get_type(*id),
            AbstractInnerType::Named(name) => {
                let template_params = &linker.types[*name].link_info.template_parameters;
                let template_args = match wr_typ {
                    Some(WrittenType::Named(wr_named)) => {
                        assert_eq!(wr_named.id, *name);
                        wr_named.template_args.try_map(|(_, arg)| {
                            Ok(match arg {
                                TemplateKind::Type(_) => {
                                    todo!("Abstract Type Args aren't yet supported!")
                                }
                                TemplateKind::Value(TemplateArg::Provided { arg, .. }) => {
                                    TemplateKind::Value(concretizer.get_value(*arg)?)
                                }
                                TemplateKind::Value(TemplateArg::NotProvided { .. }) => {
                                    TemplateKind::Value(concretizer.alloc_unknown())
                                }
                            })
                        })?
                    }
                    Some(_) => unreachable!("Can't get Array from Non-Array WrittenType!"), // TODO Fix with Let bindings (#57)
                    None => template_params.map(|(_, arg)| match &arg.kind {
                        TemplateKind::Type(_) => {
                            todo!("Abstract Type Args aren't yet supported!")
                        }
                        TemplateKind::Value(_) => TemplateKind::Value(concretizer.alloc_unknown()),
                    }),
                };

                ConcreteType::Named(ConcreteGlobalReference {
                    id: *name,
                    template_args,
                })
            }
            AbstractInnerType::Unknown(_) => {
                unreachable!("Should have been resolved already!")
            }
        },
        PeanoType::Succ(one_down) => {
            let (new_wr_typ, size) = match wr_typ {
                Some(WrittenType::Array(_span, arr)) => {
                    let (content, arr_size, _) = arr.deref();
                    (Some(content), concretizer.get_value(*arr_size)?)
                }
                Some(_) => unreachable!("Impossible: Can't get Array from Non-Array WrittenType!"), // TODO Fix with Let bindings (#57)
                None => (None, concretizer.alloc_unknown()),
            };
            ConcreteType::Array(Box::new((
                concretize_type_recurse(linker, inner, one_down, new_wr_typ, concretizer)?,
                size,
            )))
        }
        PeanoType::Unknown(_) => {
            caught_by_typecheck!("No PeanoType::Unknown should be left in execute!")
        }
    })
}

impl<'l> ExecutionContext<'l> {
    fn alloc_array_dimensions_stack(&mut self, peano_type: &PeanoType) -> Vec<UnifyableValue> {
        (0..peano_type.count().unwrap())
            .map(|_| self.type_substitutor.alloc_unknown())
            .collect()
    }
    /// Uses the current context to turn a [WrittenType] into a [ConcreteType].
    ///
    /// Failures are fatal.
    fn concretize_type(
        &mut self,
        abs: &AbstractRankedType,
        wr_typ: &WrittenType,
    ) -> ExecutionResult<ConcreteType> {
        let mut concretizer = LocalTypeConcretizer {
            template_args: self.working_on_template_args,
            generation_state: &self.generation_state,
            type_substitutor: &mut self.type_substitutor,
        };
        concretize_type_recurse(
            self.linker,
            &abs.inner,
            &abs.rank,
            Some(wr_typ),
            &mut concretizer,
        )
    }
    /// Uses the current context to turn a [AbstractRankedType] into a [ConcreteType].
    ///
    /// Failures as impossible as we don't need to read from [Self::generation_state]
    fn concretize_type_no_written_reference(&mut self, abs: &AbstractRankedType) -> ConcreteType {
        let mut concretizer = LocalTypeConcretizer {
            template_args: self.working_on_template_args,
            generation_state: &self.generation_state,
            type_substitutor: &mut self.type_substitutor,
        };
        concretize_type_recurse(self.linker, &abs.inner, &abs.rank, None, &mut concretizer).unwrap()
    }
    /// Uses the current context to turn a [WrittenType] from a [SubModule] into a [ConcreteType].
    ///
    /// Cannot fail, since we're not using [Self::generation_state]
    fn concretize_submodule_port_type(
        type_substitutor: &mut ValueUnifierAlloc,
        linker: &Linker,
        submodule_port: &Port,
        submodule_template_args: &TVec<ConcreteTemplateArg>,
        submodule_link_info: &LinkInfo,
    ) -> ConcreteType {
        let submodule_decl = submodule_link_info.instructions
            [submodule_port.declaration_instruction]
            .unwrap_declaration();
        let mut concretizer = SubModuleTypeConcretizer {
            submodule_template_args,
            instructions: &submodule_link_info.instructions,
            type_substitutor,
        };
        concretize_type_recurse(
            linker,
            &submodule_decl.typ.typ.inner,
            &submodule_decl.typ.typ.rank,
            Some(&submodule_decl.typ_expr),
            &mut concretizer,
        )
        .unwrap()
    }

    fn instantiate_port_wire_ref_root(
        &mut self,
        port: PortID,
        submodule_instr: FlatID,
        port_name_span: Option<Span>,
    ) -> RealWireRefRoot<'l> {
        let submod_id = self.generation_state[submodule_instr].unwrap_submodule_instance();
        let wire_id = self.get_submodule_port(submod_id, port, port_name_span);
        RealWireRefRoot::Wire {
            wire_id,
            preamble: Vec::new(),
        }
    }

    fn evaluate_builtin_constant(
        &self,
        cst_ref: &ConcreteGlobalReference<ConstantUUID>,
    ) -> Result<Value, String> {
        match cst_ref.id {
            get_builtin_const!("true") => Ok(Value::Bool(true)),
            get_builtin_const!("false") => Ok(Value::Bool(false)),
            get_builtin_const!("clog2") => {
                let [val] = cst_ref.template_args.cast_to_int_array();
                if val > &ibig::ibig!(0) {
                    let val = UBig::try_from(val - 1).unwrap();
                    Ok(Value::Integer(IBig::from(val.bit_len())))
                } else {
                    Err(format!(
                        "clog2 argument must be strictly positive! Found {val}"
                    ))
                }
            }
            get_builtin_const!("pow2") => {
                let [exponent] = cst_ref.template_args.cast_to_int_array();
                if let Ok(exp) = usize::try_from(exponent) {
                    let mut result = ibig::ubig!(0);
                    result.set_bit(exp);
                    Ok(Value::Integer(result.into()))
                } else {
                    Err(format!("pow2 exponent must be >= 0, found {exponent}"))
                }
            }
            get_builtin_const!("pow") => {
                let [base, exponent] = cst_ref.template_args.cast_to_int_array();
                if let Ok(exp) = usize::try_from(exponent) {
                    Ok(Value::Integer(base.pow(exp)))
                } else {
                    Err(format!("pow exponent must be >= 0, found {exponent}"))
                }
            }
            get_builtin_const!("factorial") => {
                let [n] = cst_ref.template_args.cast_to_int_array();
                let n = must_be_positive(n, "factorial parameter")?;

                Ok(Value::Integer(factorial(n).into()))
            }
            get_builtin_const!("falling_factorial") => {
                let [n, k] = cst_ref.template_args.cast_to_int_array();
                let n = must_be_positive(n, "comb n parameter")?;
                let k = must_be_positive(k, "comb k parameter")?;

                if k > n {
                    return Err(format!("comb assertion failed: k <= n. Found n={n}, k={k}"));
                }

                Ok(Value::Integer(falling_factorial(n, &k).into()))
            }
            get_builtin_const!("comb") => {
                let [n, k] = cst_ref.template_args.cast_to_int_array();
                let n = must_be_positive(n, "comb n parameter")?;
                let k = must_be_positive(k, "comb k parameter")?;

                if k > n {
                    return Err(format!("comb assertion failed: k <= n. Found n={n}, k={k}"));
                }

                Ok(Value::Integer(
                    (falling_factorial(n, &k) / factorial(k)).into(),
                ))
            }
            get_builtin_const!("assert") => {
                let [condition] = cst_ref.template_args.cast_to_array();

                if condition.unwrap_value().unwrap_bool() {
                    Ok(Value::Bool(true))
                } else {
                    Err("Assertion failed".into())
                }
            }
            get_builtin_const!("sizeof") => {
                let [concrete_typ] = cst_ref.template_args.cast_to_array();

                if let Some(typ_sz) = concrete_typ.unwrap_type().sizeof() {
                    Ok(Value::Integer(typ_sz))
                } else {
                    Err("This is an incomplete type".into())
                }
            }
            get_builtin_const!("__crash_compiler") => {
                panic!(
                    "__crash_compiler Intentional ICE. This is for debugging the compiler and LSP."
                )
            }
            other => unreachable!("{other:?} is not a known builtin constant"),
        }
    }

    fn get_named_constant_value(
        &mut self,
        cst_ref: &GlobalReference<ConstantUUID>,
    ) -> ExecutionResult<Value> {
        let concrete_ref = self.execute_global_ref(cst_ref)?;

        let linker_cst = &self.linker.constants[cst_ref.id];
        if !concrete_ref.is_final() {
            let mut resulting_error = String::from("For executing compile-time constants, all arguments must be fully specified. In this case, the arguments ");
            for (id, arg) in &concrete_ref.template_args {
                if arg.contains_unknown() {
                    use std::fmt::Write;
                    write!(
                        resulting_error,
                        "'{}', ",
                        &linker_cst.link_info.template_parameters[id].name
                    )
                    .unwrap();
                }
            }
            resulting_error.pop();
            resulting_error.pop();
            resulting_error.push_str(" were not specified");

            return Err((cst_ref.get_total_span(), resulting_error));
        }

        if linker_cst.link_info.is_extern == IsExtern::Builtin {
            cst_ref.get_total_span().debug();
            self.evaluate_builtin_constant(&concrete_ref)
                .map_err(|e| (cst_ref.get_total_span(), e))
        } else {
            todo!("Custom Constants");
        }
    }

    // Points to the wire in the hardware that corresponds to the root of this.
    fn determine_wire_ref_root(
        &mut self,
        wire_ref: &'l WireReference,
    ) -> ExecutionResult<RealWireRefRoot<'l>> {
        Ok(match &wire_ref.root {
            &WireReferenceRoot::LocalDecl(decl_id) => match &self.generation_state[decl_id] {
                SubModuleOrWire::Wire(w) => RealWireRefRoot::Wire {
                    wire_id: *w,
                    preamble: Vec::new(),
                },
                SubModuleOrWire::CompileTimeValue(_) => RealWireRefRoot::Generative(decl_id),
                SubModuleOrWire::SubModule(_) => unreachable!(),
                SubModuleOrWire::Unnasigned => unreachable!(),
            },
            WireReferenceRoot::NamedConstant(cst) => RealWireRefRoot::Constant(
                self.get_named_constant_value(cst)?,
                &wire_ref.root_typ.typ,
            ),
            WireReferenceRoot::SubModulePort(port) => {
                return Ok(self.instantiate_port_wire_ref_root(
                    port.port,
                    port.submodule_decl,
                    port.port_name_span,
                ));
            }
            WireReferenceRoot::Error => unreachable!(),
        })
    }

    /// [Self::determine_wire_ref_root] may have included a preamble path already, this must be built upon by this function
    fn instantiate_wire_ref_path(
        &mut self,
        mut preamble: Vec<RealWirePathElem>,
        path: &[WireReferencePathElement],
        domain: DomainID,
    ) -> ExecutionResult<Vec<RealWirePathElem>> {
        for v in path {
            match v {
                &WireReferencePathElement::ArrayAccess {
                    idx,
                    bracket_span,
                    output_typ: _,
                } => {
                    let idx_wire = self.get_wire_or_constant_as_wire(idx, domain)?;
                    preamble.push(RealWirePathElem::ArrayAccess {
                        span: bracket_span,
                        idx_wire,
                    });
                }
                WireReferencePathElement::ArraySlice {
                    idx_a,
                    idx_b,
                    bracket_span,
                    output_typ: _,
                } => {
                    let idx_a = match idx_a {
                        Some(idx_a) => {
                            let idx_wire_a = self.get_wire_or_constant_as_wire(*idx_a, domain)?;
                            SliceIndex::Wire(idx_wire_a)
                        }
                        None => SliceIndex::Unknown(self.type_substitutor.alloc_unknown()),
                    };

                    let idx_b = match idx_b {
                        Some(idx_b) => {
                            let idx_wire_b = self.get_wire_or_constant_as_wire(*idx_b, domain)?;
                            SliceIndex::Wire(idx_wire_b)
                        }
                        None => SliceIndex::Unknown(self.type_substitutor.alloc_unknown()),
                    };

                    preamble.push(RealWirePathElem::ArraySlice {
                        span: *bracket_span,
                        idx_a_wire: idx_a,
                        idx_b_wire: idx_b,
                    });
                }
                WireReferencePathElement::ArrayPartSelectDown {
                    idx_a,
                    width: idx_b,
                    bracket_span,
                    output_typ: _,
                }
                | WireReferencePathElement::ArrayPartSelectUp {
                    idx_a,
                    width: idx_b,
                    bracket_span,
                    output_typ: _,
                } => {
                    let idx_wire_a = self.get_wire_or_constant_as_wire(*idx_a, domain)?;

                    let idx_wire_b = self.get_wire_or_constant_as_wire(*idx_b, domain)?;

                    preamble.push(match v {
                        WireReferencePathElement::ArrayPartSelectDown { .. } => {
                            RealWirePathElem::ArrayPartSelectDown {
                                span: *bracket_span,
                                idx_a_wire: idx_wire_a,
                                width_wire: idx_wire_b,
                            }
                        }
                        WireReferencePathElement::ArrayPartSelectUp { .. } => {
                            RealWirePathElem::ArrayPartSelectUp {
                                span: *bracket_span,
                                idx_a_wire: idx_wire_a,
                                width_wire: idx_wire_b,
                            }
                        }
                        _ => unreachable!(),
                    });
                }
                WireReferencePathElement::Error => {}
            }
        }

        Ok(preamble)
    }

    fn instantiate_write_to_wire(
        &mut self,
        write_to_wire: WireID,
        to_path: Vec<RealWirePathElem>,
        from: WireID,
        num_regs: i64,
        wr_ref: WriteReference,
    ) {
        let RealWireDataSource::Multiplexer {
            is_state: _,
            sources,
        } = &mut self.wires[write_to_wire].source
        else {
            caught_by_typecheck!("Should only be a writeable wire here")
        };

        sources.push(MultiplexerSource {
            to_path,
            num_regs,
            from,
            condition: self.condition_stack.clone().into_boxed_slice(),
            wr_ref,
        });
    }

    fn write_non_generative(
        &mut self,
        write_to: &'l WriteTo,
        from: WireID,
        wr_ref: WriteReference,
    ) -> ExecutionResult<()> {
        let_unwrap!(
            WriteModifiers::Connection {
                num_regs,
                regs_span: _,
            },
            &write_to.write_modifiers
        );
        let_unwrap!(
            RealWireRefRoot::Wire {
                wire_id: target_wire,
                preamble,
            },
            self.determine_wire_ref_root(&write_to.to)?
        );
        let domain = self.wires[target_wire].domain;
        let instantiated_path =
            self.instantiate_wire_ref_path(preamble, &write_to.to.path, domain)?;
        self.instantiate_write_to_wire(target_wire, instantiated_path, from, *num_regs, wr_ref);
        Ok(())
    }

    fn write_generative(
        &mut self,
        write_to: &'l WriteTo,
        value: Value,
        original_expression: FlatID,
    ) -> ExecutionResult<()> {
        match &write_to.write_modifiers {
            WriteModifiers::Connection {
                num_regs,
                regs_span: _,
            } => match self.determine_wire_ref_root(&write_to.to)? {
                RealWireRefRoot::Wire {
                    wire_id: target_wire,
                    preamble,
                } => {
                    let domain = self.wires[target_wire].domain;
                    let from = self.alloc_wire_for_const(
                        value,
                        write_to.to.get_output_typ(),
                        original_expression,
                        domain,
                        self.link_info.instructions[original_expression]
                            .unwrap_expression()
                            .span,
                    )?;
                    let instantiated_path =
                        self.instantiate_wire_ref_path(preamble, &write_to.to.path, domain)?;
                    self.instantiate_write_to_wire(
                        target_wire,
                        instantiated_path,
                        from,
                        *num_regs,
                        WriteReference {
                            original_expression,
                            write_idx: 0,
                        },
                    );
                }
                RealWireRefRoot::Generative(target_decl) => {
                    let SubModuleOrWire::CompileTimeValue(v_writable) =
                        &mut self.generation_state[target_decl]
                    else {
                        unreachable!()
                    };
                    let mut new_val = std::mem::replace(v_writable, Value::Unset);
                    self.generation_state.write_gen_variable(
                        &mut new_val,
                        &write_to.to.path,
                        value,
                    )?;

                    let SubModuleOrWire::CompileTimeValue(v_writable) =
                        &mut self.generation_state[target_decl]
                    else {
                        unreachable!()
                    };
                    *v_writable = new_val;
                }
                RealWireRefRoot::Constant(_cst, _) => {
                    caught_by_typecheck!("Cannot assign to constants");
                }
            },
            WriteModifiers::Initial { initial_kw_span: _ } => {
                let root_wire =
                    self.generation_state[write_to.to.root.unwrap_local_decl()].unwrap_wire();
                let RealWireDataSource::Multiplexer {
                    is_state: Some(initial_value),
                    sources: _,
                } = &mut self.wires[root_wire].source
                else {
                    caught_by_typecheck!()
                };
                self.generation_state.write_gen_variable(
                    initial_value,
                    &write_to.to.path,
                    value,
                )?;
            }
        }
        Ok(())
    }
    fn alloc_wire_for_const(
        &mut self,
        value: Value,
        abs_typ: &AbstractRankedType,
        original_instruction: FlatID,
        domain: DomainID,
        const_span: Span,
    ) -> ExecutionResult<WireID> {
        if value.contains_unset() {
            return Err((const_span, format!("This compile-time value was not fully resolved by the time it needed to be converted to a wire: {value}")));
        }
        Ok(self.wires.alloc(RealWire {
            typ: value
                .concretize_type(
                    self.linker,
                    abs_typ,
                    self.working_on_template_args,
                    &mut self.type_substitutor,
                )
                .map_err(|msg| (const_span, msg))?,
            source: RealWireDataSource::Constant { value },
            original_instruction,
            domain,
            name: self.unique_name_producer.get_unique_name(""),
            specified_latency: CALCULATE_LATENCY_LATER,
            absolute_latency: CALCULATE_LATENCY_LATER,
        }))
    }
    fn get_wire_or_constant_as_wire(
        &mut self,
        original_instruction: FlatID,
        domain: DomainID,
    ) -> ExecutionResult<WireID> {
        match &self.generation_state[original_instruction] {
            SubModuleOrWire::SubModule(_) => unreachable!(),
            SubModuleOrWire::Unnasigned => unreachable!(),
            SubModuleOrWire::Wire(w) => Ok(*w),
            SubModuleOrWire::CompileTimeValue(v) => {
                let value = v.clone();
                let original_expr =
                    self.link_info.instructions[original_instruction].unwrap_subexpression();
                self.alloc_wire_for_const(
                    value,
                    original_expr.typ,
                    original_instruction,
                    domain,
                    original_expr.span,
                )
            }
        }
    }

    /// Allocates ports on first use, to see which ports are used, and to determine instantiation based on this
    fn get_submodule_port(
        &mut self,
        sub_module_id: SubModuleID,
        port_id: PortID,
        port_name_span: Option<Span>,
    ) -> WireID {
        let submod_instance = &mut self.submodules[sub_module_id]; // Separately grab the same submodule every time because we take a &mut in for get_wire_or_constant_as_wire
        let wire_found = &mut submod_instance.port_map[port_id];

        if let Some(wire_found) = wire_found {
            if let Some(sp) = port_name_span {
                // Deduplicate these spans, so we don't produce overly huge errors, nor allocate more memory than needed
                add_to_small_set(&mut wire_found.name_refs, sp);
            }
            wire_found.maps_to_wire
        } else {
            let submod_md = &self.linker.modules[submod_instance.refers_to.id];
            let port_data = &submod_md.ports[port_id];
            let submodule_instruction = self.link_info.instructions
                [submod_instance.original_instruction]
                .unwrap_submodule();
            let source = if port_data.is_input {
                RealWireDataSource::Multiplexer {
                    is_state: None,
                    sources: Vec::new(),
                }
            } else {
                RealWireDataSource::ReadOnly
            };
            let domain = submodule_instruction.local_interface_domains[port_data.domain];
            let typ = Self::concretize_submodule_port_type(
                &mut self.type_substitutor,
                self.linker,
                port_data,
                &submod_instance.refers_to.template_args,
                &submod_md.link_info,
            );
            let new_wire = self.wires.alloc(RealWire {
                source,
                original_instruction: submod_instance.original_instruction,
                domain: domain.unwrap_physical(),
                typ,
                name: self
                    .unique_name_producer
                    .get_unique_name(format!("{}_{}", submod_instance.name, port_data.name)),
                specified_latency: CALCULATE_LATENCY_LATER,
                absolute_latency: CALCULATE_LATENCY_LATER,
            });

            let name_refs = if let Some(sp) = port_name_span {
                vec![sp]
            } else {
                Vec::new()
            };

            *wire_found = Some(SubModulePort {
                maps_to_wire: new_wire,
                name_refs,
            });
            new_wire
        }
    }

    fn get_wire_ref_root_as_wire(
        &mut self,
        wire_ref: &'l WireReference,
        original_instruction: FlatID,
        domain: DomainID,
    ) -> ExecutionResult<(WireID, Vec<RealWirePathElem>)> {
        let root = self.determine_wire_ref_root(wire_ref)?;
        Ok(match root {
            RealWireRefRoot::Wire { wire_id, preamble } => (wire_id, preamble),
            RealWireRefRoot::Generative(decl_id) => {
                let decl = self.link_info.instructions[decl_id].unwrap_declaration();
                let value = self.generation_state[decl_id]
                    .unwrap_generation_value()
                    .clone();
                (
                    self.alloc_wire_for_const(
                        value,
                        &decl.typ.typ,
                        decl_id,
                        domain,
                        wire_ref.root_span,
                    )?,
                    Vec::new(),
                )
            }
            RealWireRefRoot::Constant(value, typ) => (
                self.alloc_wire_for_const(
                    value,
                    typ,
                    original_instruction,
                    domain,
                    wire_ref.root_span,
                )?,
                Vec::new(),
            ),
        })
    }

    fn expression_to_real_wire(
        &mut self,
        expression: &'l Expression,
        original_instruction: FlatID,
        domain: DomainID,
    ) -> ExecutionResult<Vec<WireID>> {
        let source = match &expression.source {
            ExpressionSource::WireRef(wire_ref) => {
                let (root_wire, path_preamble) =
                    self.get_wire_ref_root_as_wire(wire_ref, original_instruction, domain)?;
                let path = self.instantiate_wire_ref_path(path_preamble, &wire_ref.path, domain)?;

                if path.is_empty() {
                    // Little optimization reduces instructions
                    return Ok(vec![root_wire]);
                }

                RealWireDataSource::Select {
                    root: root_wire,
                    path,
                }
            }
            ExpressionSource::UnaryOp { op, rank, right } => {
                let right = self.get_wire_or_constant_as_wire(*right, domain)?;
                RealWireDataSource::UnaryOp {
                    op: *op,
                    rank: self.alloc_array_dimensions_stack(rank),
                    right,
                }
            }
            ExpressionSource::BinaryOp {
                op,
                rank,
                left,
                right,
            } => {
                let left = self.get_wire_or_constant_as_wire(*left, domain)?;
                let right = self.get_wire_or_constant_as_wire(*right, domain)?;
                RealWireDataSource::BinaryOp {
                    op: *op,
                    rank: self.alloc_array_dimensions_stack(rank),
                    left,
                    right,
                }
            }
            ExpressionSource::FuncCall(fc) => {
                let submod_id = self.generation_state[fc.interface_reference.submodule_decl]
                    .unwrap_submodule_instance();
                let original_submod_instr = self.link_info.instructions
                    [fc.interface_reference.submodule_decl]
                    .unwrap_submodule();
                let submod_md = &self.linker.modules[original_submod_instr.module_ref.id];
                let interface = &submod_md.interfaces[fc.interface_reference.submodule_interface];
                let submod_interface_domain = interface.domain;
                let domain = original_submod_instr.local_interface_domains[submod_interface_domain]
                    .unwrap_physical();

                add_to_small_set(
                    &mut self.submodules[submod_id].interface_call_sites
                        [fc.interface_reference.submodule_interface],
                    fc.interface_reference.interface_span,
                );

                for (write_idx, (port, arg)) in
                    zip_eq(interface.func_call_inputs, &fc.arguments).enumerate()
                {
                    let from = self.get_wire_or_constant_as_wire(*arg, domain)?;
                    let port_wire = self.get_submodule_port(submod_id, port, None);
                    self.instantiate_write_to_wire(
                        port_wire,
                        Vec::new(),
                        from,
                        0,
                        WriteReference {
                            original_expression: original_instruction,
                            write_idx,
                        },
                    );
                }

                return Ok(interface
                    .func_call_outputs
                    .iter()
                    .map(|port_id| self.get_submodule_port(submod_id, port_id, None))
                    .collect());
            }
            ExpressionSource::ArrayConstruct(arr) => {
                let mut array_wires = Vec::with_capacity(arr.len());
                for v_id in arr {
                    let wire_id = self.get_wire_or_constant_as_wire(*v_id, domain)?;
                    array_wires.push(wire_id);
                }
                RealWireDataSource::ConstructArray { array_wires }
            }
            ExpressionSource::Constant(_) => {
                unreachable!("Constant cannot be non-compile-time");
            }
            ExpressionSource::Range { .. } => {
                unreachable!("Range cannot be non-compile-time");
            }
        };
        let typ = self
            .concretize_type_no_written_reference(expression.as_single_output_expr().unwrap().typ);
        Ok(vec![self.wires.alloc(RealWire {
            name: self.unique_name_producer.get_unique_name(""),
            typ,
            original_instruction,
            domain,
            source,
            specified_latency: CALCULATE_LATENCY_LATER,
            absolute_latency: CALCULATE_LATENCY_LATER,
        })])
    }

    fn get_specified_latency(&mut self, spec_lat: Option<FlatID>) -> ExecutionResult<i64> {
        Ok(if let Some(spec) = &spec_lat {
            self.generation_state.get_generation_small_int(*spec)?
        } else {
            CALCULATE_LATENCY_LATER
        })
    }

    fn instantiate_declaration(
        &mut self,
        wire_decl: &Declaration,
        original_instruction: FlatID,
    ) -> ExecutionResult<SubModuleOrWire> {
        let typ = self.concretize_type(&wire_decl.typ.typ, &wire_decl.typ_expr)?;

        Ok(if wire_decl.identifier_type == IdentifierType::Generative {
            let value: Value =
                if let DeclarationKind::GenerativeInput(template_id) = wire_decl.decl_kind {
                    // Only for template arguments, we must initialize their value to the value they've been assigned in the template instantiation
                    self.working_on_template_args[template_id]
                        .unwrap_value()
                        .unwrap_set()
                        .clone()
                } else {
                    // Empty initial value
                    typ.get_initial_val()
                };
            SubModuleOrWire::CompileTimeValue(value)
        } else {
            let source = if wire_decl.read_only {
                RealWireDataSource::ReadOnly
            } else {
                let is_state = if wire_decl.identifier_type == IdentifierType::State {
                    Some(typ.get_initial_val())
                } else {
                    None
                };
                RealWireDataSource::Multiplexer {
                    is_state,
                    sources: Vec::new(),
                }
            };

            let specified_latency = self.get_specified_latency(wire_decl.latency_specifier)?;
            let wire_id = self.wires.alloc(RealWire {
                name: self.unique_name_producer.get_unique_name(&wire_decl.name),
                typ,
                original_instruction,
                domain: wire_decl.typ.domain.unwrap_physical(),
                source,
                specified_latency,
                absolute_latency: CALCULATE_LATENCY_LATER,
            });
            SubModuleOrWire::Wire(wire_id)
        })
    }

    fn execute_global_ref<ID: Copy + Into<GlobalUUID>>(
        &mut self,
        global_ref: &GlobalReference<ID>,
    ) -> ExecutionResult<ConcreteGlobalReference<ID>> {
        let template_args = global_ref.template_args.try_map(
            |(_, arg)| -> ExecutionResult<ConcreteTemplateArg> {
                Ok(match arg {
                    TemplateKind::Type(arg) => TemplateKind::Type(match arg {
                        TemplateArg::Provided { arg, abs_typ, .. } => {
                            self.concretize_type(abs_typ, arg)?
                        }
                        TemplateArg::NotProvided { abs_typ } => {
                            self.concretize_type_no_written_reference(abs_typ)
                        }
                    }),
                    TemplateKind::Value(arg) => TemplateKind::Value({
                        match arg {
                            TemplateArg::Provided { arg, .. } => self
                                .generation_state
                                .get_generation_value(*arg)?
                                .clone()
                                .into(),
                            TemplateArg::NotProvided { .. } => {
                                self.type_substitutor.alloc_unknown()
                            }
                        }
                    }),
                })
            },
        )?;
        Ok(ConcreteGlobalReference {
            id: global_ref.id,
            template_args,
        })
    }

    fn compute_compile_time_wireref(&mut self, wire_ref: &WireReference) -> ExecutionResult<Value> {
        let mut work_on_value: Value = match &wire_ref.root {
            &WireReferenceRoot::LocalDecl(decl_id) => {
                self.generation_state.get_generation_value(decl_id)?.clone()
            }
            WireReferenceRoot::NamedConstant(cst) => self.get_named_constant_value(cst)?,
            &WireReferenceRoot::SubModulePort(_) => {
                todo!("Don't yet support compile time functions")
            }
            WireReferenceRoot::Error => caught_by_typecheck!(),
        };

        for path_elem in &wire_ref.path {
            work_on_value = match path_elem {
                &WireReferencePathElement::ArrayAccess {
                    idx,
                    bracket_span,
                    output_typ: _,
                } => {
                    let idx = self.generation_state.get_generation_integer(idx)?;

                    array_access(&work_on_value, idx, bracket_span)?.clone()
                }
                &WireReferencePathElement::ArraySlice {
                    idx_a,
                    idx_b,
                    bracket_span,
                    output_typ: _,
                } => {
                    let idx_a = match idx_a {
                        Some(idx_a) => self.generation_state.get_generation_integer(idx_a)?,
                        None => &ibig!(0),
                    };
                    let idx_b = match idx_b {
                        Some(idx_b) => Some(self.generation_state.get_generation_integer(idx_b)?),
                        None => None,
                    };
                    Value::Array(
                        array_slice(&mut work_on_value, idx_a, idx_b, bracket_span)?.clone(),
                    )
                }
                &WireReferencePathElement::ArrayPartSelectDown {
                    idx_a,
                    width: idx_b,
                    bracket_span,
                    output_typ: _,
                }
                | &WireReferencePathElement::ArrayPartSelectUp {
                    idx_a,
                    width: idx_b,
                    bracket_span,
                    output_typ: _,
                } => {
                    let idx_a = self.generation_state.get_generation_integer(idx_a)?;
                    let idx_b = self.generation_state.get_generation_integer(idx_b)?;
                    let idx_end = match path_elem {
                        &WireReferencePathElement::ArrayPartSelectDown { .. } => &(idx_a - idx_b),
                        &WireReferencePathElement::ArrayPartSelectUp { .. } => &(idx_a + idx_b),
                        _ => unreachable!(),
                    };
                    Value::Array(
                        array_slice(&mut work_on_value, idx_a, Some(idx_end), bracket_span)?
                            .clone(),
                    )
                }
                WireReferencePathElement::Error => {
                    return Err((wire_ref.root_span, "todo".to_owned()))
                }
            }
        }

        Ok(work_on_value)
    }
    fn compute_compile_time(&mut self, expr: &Expression) -> ExecutionResult<Value> {
        fn duplicate_for_all_array_ranks<const SZ: usize>(
            values: &[&Value; SZ],
            rank: usize,
            f: &mut impl FnMut(&[&Value; SZ]) -> Result<Value, String>,
        ) -> Result<Value, String> {
            if rank == 0 {
                f(values)
            } else {
                let all_arrs: [_; SZ] = std::array::from_fn(|i| values[i].unwrap_array());

                let len = all_arrs[0].len();
                if !all_arrs.iter().all(|a| len == a.len()) {
                    let lens: [String; SZ] = std::array::from_fn(|i| all_arrs[i].len().to_string());
                    return Err(format!(
                        "Higher Rank array operation's arrays don't match in size: {}",
                        lens.join(", ")
                    ));
                }
                let mut results = Vec::with_capacity(len);
                for j in 0..len {
                    let values_parts: [_; SZ] = std::array::from_fn(|i| &all_arrs[i][j]);
                    results.push(duplicate_for_all_array_ranks(&values_parts, rank - 1, f)?);
                }
                Ok(Value::Array(results))
            }
        }

        Ok(match &expr.source {
            ExpressionSource::WireRef(wire_ref) => {
                self.compute_compile_time_wireref(wire_ref)?.clone()
            }
            ExpressionSource::UnaryOp { op, rank, right } => {
                let right_val = self.generation_state.get_generation_value(*right)?;
                duplicate_for_all_array_ranks(&[right_val], rank.count().unwrap(), &mut |[v]| {
                    Ok(compute_unary_op(*op, v))
                })
                .unwrap()
            }
            ExpressionSource::BinaryOp {
                op,
                rank,
                left,
                right,
            } => {
                let left_val = self.generation_state.get_generation_value(*left)?;
                let right_val = self.generation_state.get_generation_value(*right)?;

                duplicate_for_all_array_ranks(
                    &[left_val, right_val],
                    rank.count().unwrap(),
                    &mut |[l, r]| {
                        match op {
                            BinaryOperator::Divide | BinaryOperator::Modulo => {
                                if right_val.unwrap_integer() == &ibig::ibig!(0) {
                                    return Err(format!(
                                        "Divide or Modulo by zero: {} / 0",
                                        l.unwrap_integer()
                                    ));
                                }
                            }
                            _ => {}
                        }

                        Ok(compute_binary_op(l, *op, r))
                    },
                )
                .map_err(|reason| (expr.span, reason))?
            }
            ExpressionSource::Range { start, end } => {
                let start_val = self
                    .generation_state
                    .get_generation_value(*start)?
                    .unwrap_integer();
                let end_val = self
                    .generation_state
                    .get_generation_value(*end)?
                    .unwrap_integer();

                let length: usize = match (end_val - start_val).abs().unsigned_abs().try_into() {
                    Ok(n) => n,
                    Err(_) => {
                        let max = usize::MAX;
                        panic!("Range larger than {max}")
                    }
                };

                let mut result = Vec::with_capacity(length);
                let mut value = start_val.clone();
                for _ in 0..length {
                    result.push(Value::Integer(value.clone()));
                    if end_val > start_val {
                        value += 1;
                    } else {
                        value -= 1;
                    }
                }
                Value::Array(result)
            }
            ExpressionSource::FuncCall(_) => {
                todo!("Func Calls cannot yet be executed at compiletime")
            }
            ExpressionSource::ArrayConstruct(arr) => {
                let mut result = Vec::with_capacity(arr.len());
                for v_id in arr {
                    let val = self.generation_state.get_generation_value(*v_id)?;
                    result.push(val.clone());
                }
                Value::Array(result)
            }
            ExpressionSource::Constant(value) => value.clone(),
        })
    }

    fn instantiate_code_block(&mut self, block_range: FlatIDRange) -> ExecutionResult<()> {
        let mut instruction_range = block_range.into_iter();
        while let Some(original_instruction) = instruction_range.next() {
            let instr = &self.link_info.instructions[original_instruction];
            self.link_info
                .get_instruction_span(original_instruction)
                .debug();
            let instance_to_add: SubModuleOrWire = match instr {
                Instruction::SubModule(submodule) => {
                    let sub_module = &self.linker.modules[submodule.module_ref.id];

                    let name_origin = if let Some((name, _span)) = &submodule.name {
                        name
                    } else {
                        ""
                    };
                    let port_map = sub_module.ports.map(|_| None);
                    let interface_call_sites = sub_module.interfaces.map(|_| Vec::new());

                    let refers_to = self.execute_global_ref(&submodule.module_ref)?;

                    SubModuleOrWire::SubModule(self.submodules.alloc(SubModule {
                        original_instruction,
                        instance: OnceCell::new(),
                        refers_to,
                        port_map,
                        interface_call_sites,
                        name: self.unique_name_producer.get_unique_name(name_origin),
                    }))
                }
                Instruction::Declaration(wire_decl) => {
                    self.instantiate_declaration(wire_decl, original_instruction)?
                }
                Instruction::Expression(expr) => {
                    match expr.domain {
                        DomainType::Generative => {
                            let value_computed = self.compute_compile_time(expr)?;
                            match &expr.output {
                                ExpressionOutput::SubExpression(_full_type) => {} // Simply returning value_computed is enough
                                ExpressionOutput::MultiWrite(write_tos) => {
                                    if let Some(single_write) = write_tos.first() {
                                        self.write_generative(
                                            single_write,
                                            value_computed.clone(), // We do an extra clone, maybe not needed, such that we can show the value in GenerationState
                                            original_instruction,
                                        )?;
                                    }
                                }
                            }
                            SubModuleOrWire::CompileTimeValue(value_computed)
                        }
                        DomainType::Physical(domain) => {
                            let output_wires =
                                self.expression_to_real_wire(expr, original_instruction, domain)?;
                            match &expr.output {
                                ExpressionOutput::SubExpression(_full_type) => {
                                    let single_wire = unwrap_single_element(output_wires);
                                    SubModuleOrWire::Wire(single_wire)
                                }
                                ExpressionOutput::MultiWrite(write_tos) => {
                                    if write_tos.is_empty() {
                                        continue; // See no errors on zero outputs (#79)
                                    }
                                    for (write_idx, (expr_output, write)) in
                                        zip_eq(output_wires, write_tos).enumerate()
                                    {
                                        self.write_non_generative(
                                            write,
                                            expr_output,
                                            WriteReference {
                                                original_expression: original_instruction,
                                                write_idx,
                                            },
                                        )?;
                                    }
                                    continue;
                                }
                            }
                        }
                        DomainType::Unknown(_) => caught_by_typecheck!(),
                    }
                }
                Instruction::IfStatement(stm) => {
                    if stm.is_generative {
                        let condition_val =
                            self.generation_state.get_generation_value(stm.condition)?;
                        let run_range = if condition_val.unwrap_bool() {
                            stm.then_block
                        } else {
                            stm.else_block
                        };
                        self.instantiate_code_block(run_range)?;
                    } else {
                        let condition_wire = self.generation_state[stm.condition].unwrap_wire();
                        self.condition_stack.push(ConditionStackElem {
                            condition_wire,
                            inverse: false,
                        });
                        self.instantiate_code_block(stm.then_block)?;

                        if !stm.else_block.is_empty() {
                            self.condition_stack.last_mut().unwrap().inverse = true;
                            self.instantiate_code_block(stm.else_block)?;
                        }

                        // Get rid of the condition
                        let _ = self.condition_stack.pop().unwrap();
                    }
                    instruction_range.skip_to(stm.else_block.1);
                    continue;
                }
                Instruction::ForStatement(stm) => {
                    // TODO Non integer for loops?
                    let start_val = self
                        .generation_state
                        .get_generation_value(stm.start)?
                        .unwrap_integer()
                        .clone();
                    let end_val = self
                        .generation_state
                        .get_generation_value(stm.end)?
                        .unwrap_integer()
                        .clone();
                    if start_val > end_val {
                        let start_flat =
                            &self.link_info.instructions[stm.start].unwrap_expression();
                        let end_flat = &self.link_info.instructions[stm.end].unwrap_expression();
                        return Err((
                            Span::new_overarching(start_flat.span, end_flat.span),
                            format!("for loop range end is before begin: {start_val}:{end_val}"),
                        ));
                    }

                    let mut current_val = start_val;

                    while current_val < end_val {
                        let SubModuleOrWire::CompileTimeValue(v) =
                            &mut self.generation_state[stm.loop_var_decl]
                        else {
                            unreachable!()
                        };
                        *v = Value::Integer(current_val.clone());
                        current_val += 1;
                        self.instantiate_code_block(stm.loop_body)?;
                    }

                    instruction_range.skip_to(stm.loop_body.1);
                    continue;
                }
            };
            self.generation_state[original_instruction] = instance_to_add;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::instantiation::execute::factorial;

    #[test]
    fn test_factorial() {
        let a = ibig::ubig!(7);

        assert_eq!(factorial(a), ibig::ubig!(5040))
    }
    #[test]
    fn test_falling_factorial() {
        let a = ibig::ubig!(20);
        let b = ibig::ubig!(15);

        let a_factorial = factorial(a.clone());
        let a_b_factorial = factorial(&a - &b);

        assert_eq!(
            falling_factorial(a.clone(), &b),
            a_factorial / a_b_factorial
        )
    }
}
