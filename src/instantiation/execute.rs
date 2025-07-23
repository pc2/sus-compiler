//! Executes the generative code and produces a netlist from it
//!
//! Stops generating at the first error.
//!
//! As for typing, it only instantiates written types and leaves the rest for further typechecking.

use std::borrow::Cow;
use std::ops::{Deref, Index, IndexMut, Range};

use crate::latency::CALCULATE_LATENCY_LATER;
use crate::let_unwrap;
use crate::linker::IsExtern;
use crate::linker::{GlobalUUID, LinkInfo};
use crate::prelude::*;
use crate::typing::abstract_type::{AbstractInnerType, AbstractRankedType, PeanoType};
use crate::typing::concrete_type::ConcreteTemplateArg;
use crate::typing::domain_type::DomainType;
use crate::typing::template::TVec;
use crate::util::{unwrap_single_element, zip_eq};

use ibig::{IBig, UBig};

use sus_proc_macro::get_builtin_const;

use crate::flattening::*;
use crate::value::{compute_binary_op, compute_unary_op, Value};

use crate::typing::{concrete_type::ConcreteType, template::TemplateKind};

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
                .map(|(_, _)| SubModuleOrWire::Unassigned),
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

enum GenerativeWireRefPathElem {
    ArrayAccess {
        idx: IBig,
        span: Span,
    },
    Slice {
        from: Option<IBig>,
        to: Option<IBig>,
        span: Span,
    },
}

fn make_array_bounds<'v>(
    from_maybe: Option<IBig>,
    to_maybe: Option<IBig>,
    mut values: impl Iterator<Item = &'v Value>,
    span: Span,
) -> ExecutionResult<Range<usize>> {
    if let Some(first) = values.next() {
        let_unwrap!(Value::Array(arr), first);

        let arr_sz = arr.len();

        let is_dynamic_range = from_maybe.is_none() || to_maybe.is_none();

        let from = from_maybe.unwrap_or_else(|| IBig::from(0));
        let to = to_maybe.unwrap_or_else(|| IBig::from(arr_sz));

        if from > to {
            return Err((span, format!("Slice {from}:{to} has a negative length.")));
        }

        let (from_valid, to_valid) = match (usize::try_from(&from), usize::try_from(&to)) {
            (Ok(from), Ok(to)) if to <= arr_sz => (from, to), // && from >= 0, but it's usize
            _ => {
                let e = format!(
                    "Slice {from}:{to} is out of bounds. The size of this array is {arr_sz}"
                );
                return Err((span, e));
            }
        };

        for v in values {
            let_unwrap!(Value::Array(arr), v);

            let other_arr_sz = arr.len();

            if is_dynamic_range && other_arr_sz != arr_sz {
                let e = "Using a variable index on a jagged array".to_string();
                return Err((span, e));
            }

            if to_valid > other_arr_sz {
                let e = format!(
                    "Slice {from}:{to} is out of bounds. The size of this array is {other_arr_sz}"
                );
                return Err((span, e));
            }
        }

        Ok(from_valid..to_valid)
    } else {
        Ok(0..0)
    }
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
        fn array_access<'t>(
            tgt_ref: &'t mut Value,
            idx: &IBig,
            span: Span,
        ) -> (ExecutionResult<()>, &'t mut Value) {
            let idx_as_usize = usize::try_from(idx).ok();

            let Value::Array(tgt_arr) = tgt_ref else {
                unreachable!()
            };
            let arr_sz = tgt_arr.len();

            if idx_as_usize.and_then(|idx| tgt_arr.get_mut(idx)).is_some() {
                // Once we know we're safe, we have to do the little dance again, such that this time we *consume* tgt_ref
                let Value::Array(tgt_arr) = tgt_ref else {
                    unreachable!()
                };
                (Ok(()), &mut tgt_arr[idx_as_usize.unwrap()])
            } else {
                let err = Err((
                    span,
                    format!("Index {idx} out of bounds for array of size {arr_sz}"),
                ));
                (err, tgt_ref)
            }
        }

        // must be an array, from earlier typechecking

        let mut cur_targets: Vec<(&mut Value, Value)> = vec![(target, to_write)];
        let executed_path = self.execute_path(conn_path)?;

        for path_elem in executed_path {
            match path_elem {
                GenerativeWireRefPathElem::ArrayAccess { idx, span } => {
                    for target in &mut cur_targets {
                        replace_with::replace_with_or_abort_and_return(&mut target.0, |tgt| {
                            array_access(tgt, &idx, span)
                        })?;
                    }
                }
                GenerativeWireRefPathElem::Slice { from, to, span } => {
                    let slice =
                        make_array_bounds(from, to, cur_targets.iter().map(|t| &*t.0), span)?;

                    let new_len = cur_targets.len() * slice.len();

                    let mut new_targets = Vec::with_capacity(new_len);

                    for (target, value) in cur_targets {
                        let_unwrap!(Value::Array(target), target);
                        let Value::Array(value) = value else {
                            unreachable!()
                        };

                        let slice_len = slice.len();
                        let from_len = value.len();
                        if from_len != slice_len {
                            let from = slice.start;
                            let to = slice.end;
                            return Err((span, format!("Attempting to write to this slice {from}:{to} (length {slice_len}) with an array of length {from_len}.")));
                        }
                        for new_pair in zip_eq(&mut target[slice.clone()], value.into_iter()) {
                            new_targets.push(new_pair)
                        }
                    }

                    cur_targets = new_targets;
                }
            }
        }

        for (t, f) in cur_targets {
            *t = f;
        }
        Ok(())
    }
    fn read_from_path(
        &self,
        value: &Value,
        conn_path: &[WireReferencePathElement],
    ) -> ExecutionResult<Value> {
        let executed_path = self.execute_path(conn_path)?;

        let mut flattened_result_tensor: Vec<&Value> = vec![value];
        let mut create_array_layers = Vec::new();

        // First we expand the result tensor by digging down
        for p in executed_path {
            match p {
                GenerativeWireRefPathElem::ArrayAccess { idx, span } => {
                    for vp in &mut flattened_result_tensor {
                        let_unwrap!(Value::Array(arr), *vp);
                        let arr_sz = arr.len();
                        let Some(v) = usize::try_from(&idx).ok().and_then(|idx| arr.get(idx))
                        else {
                            return Err((
                                span,
                                format!("Index {idx} out of bounds for array of size {arr_sz}"),
                            ));
                        };
                        *vp = v;
                    }
                }
                GenerativeWireRefPathElem::Slice { from, to, span } => {
                    let slice =
                        make_array_bounds(from, to, flattened_result_tensor.iter().copied(), span)?;

                    let mut new_value_parts =
                        Vec::with_capacity(flattened_result_tensor.len() * slice.len());

                    for vp in &mut flattened_result_tensor {
                        let_unwrap!(Value::Array(arr), *vp);

                        for a in &arr[slice.clone()] {
                            new_value_parts.push(a);
                        }
                    }

                    create_array_layers.push(slice.len());
                    flattened_result_tensor = new_value_parts;
                }
            }
        }

        // If we've created a zero-sized tensor, this prevents a div-by-zero error downstream
        if flattened_result_tensor.is_empty() {
            return Ok(Value::Array(Vec::new()));
        }
        // Then we re-consitute the array until we have one element again
        let mut flattened_result_tensor: Vec<Value> =
            flattened_result_tensor.into_iter().cloned().collect();
        for dimension_len in create_array_layers.into_iter().rev() {
            let num_sub_tensors = flattened_result_tensor.len() / dimension_len;
            assert_eq!(flattened_result_tensor.len() % dimension_len, 0);

            let mut result_iter = flattened_result_tensor.into_iter();
            flattened_result_tensor = (0..num_sub_tensors)
                .map(|_| {
                    Value::Array(
                        (0..dimension_len)
                            .map(|_| result_iter.next().unwrap())
                            .collect(),
                    )
                })
                .collect();

            assert!(result_iter.next().is_none());
        }

        Ok(unwrap_single_element(flattened_result_tensor))
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

    fn execute_path(
        &self,
        path: &[WireReferencePathElement],
    ) -> ExecutionResult<Vec<GenerativeWireRefPathElem>> {
        let mut resulting_path = Vec::with_capacity(path.len());
        for p in path {
            let new_elem = match p {
                WireReferencePathElement::FieldAccess { refers_to, .. } => {
                    match refers_to.get().unwrap() {
                        PathElemRefersTo::Interface(_) => {
                            unreachable!("Not possible in generative context!")
                        }
                    }
                }
                WireReferencePathElement::ArrayAccess {
                    idx, bracket_span, ..
                } => {
                    let idx = self.get_generation_integer(*idx)?.clone();
                    GenerativeWireRefPathElem::ArrayAccess {
                        idx,
                        span: bracket_span.inner_span(),
                    }
                }
                WireReferencePathElement::ArraySlice {
                    from,
                    to,
                    bracket_span,
                    ..
                } => {
                    let from = if let Some(from) = from {
                        Some(self.get_generation_integer(*from)?.clone())
                    } else {
                        None
                    };
                    let to = if let Some(to) = to {
                        Some(self.get_generation_integer(*to)?.clone())
                    } else {
                        None
                    };
                    GenerativeWireRefPathElem::Slice {
                        from,
                        to,
                        span: bracket_span.inner_span(),
                    }
                }
                WireReferencePathElement::ArrayPartSelect {
                    from,
                    width,
                    bracket_span,
                    direction,
                    ..
                } => {
                    let from = self.get_generation_integer(*from)?;
                    let width = self.get_generation_integer(*width)?;

                    let (from, to) = match direction {
                        PartSelectDirection::Up => (Some(from.clone()), Some(from + width - 1)),
                        PartSelectDirection::Down => (Some(from - width + 1), Some(from.clone())),
                    };
                    GenerativeWireRefPathElem::Slice {
                        from,
                        to,
                        span: bracket_span.inner_span(),
                    }
                }
            };
            resulting_path.push(new_elem);
        }
        Ok(resulting_path)
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

struct InterfaceWires {
    condition_wire: Option<WireID>,
    inputs: Vec<WireID>,
    outputs: Vec<WireID>,
    interface_span: Span,
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
                let DeclarationKind::TemplateParameter(template_id) = &template_arg_decl.decl_kind
                else {
                    return Ok(self.type_substitutor.alloc_unknown());
                };
                self.submodule_template_args[*template_id]
                    .unwrap_value()
                    .clone()
            }
            ExpressionSource::Literal(cst) => cst.clone().into(),
            _ => self.type_substitutor.alloc_unknown(),
        })
    }

    fn alloc_unknown(&mut self) -> UnifyableValue {
        self.type_substitutor.alloc_unknown()
    }
}

fn concretize_global_ref<ID: Copy + Into<GlobalUUID>>(
    linker: &Linker,
    global_ref: &GlobalReference<ID>,
    concretizer: &mut impl Concretizer,
) -> ExecutionResult<ConcreteGlobalReference<ID>> {
    let target: &LinkInfo = &linker.globals[global_ref.id.into()];
    let template_args = target.template_parameters.try_map2(
        &global_ref.template_arg_types,
        |(param_id, param, abs_typ)| -> ExecutionResult<ConcreteTemplateArg> {
            Ok(match &param.kind {
                TemplateKind::Type(_) => {
                    let wr_typ = global_ref.get_type_arg_for(param_id);
                    let abs_typ = abs_typ.unwrap_type();
                    TemplateKind::Type(concretize_type_recurse(
                        linker,
                        &abs_typ.inner,
                        &abs_typ.rank,
                        wr_typ,
                        concretizer,
                    )?)
                }
                TemplateKind::Value(_) => {
                    TemplateKind::Value(if let Some(v) = global_ref.get_value_arg_for(param_id) {
                        concretizer.get_value(v)?
                    } else {
                        concretizer.alloc_unknown()
                    })
                }
            })
        },
    )?;
    Ok(ConcreteGlobalReference {
        id: global_ref.id,
        template_args,
    })
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
                let target = &linker.types[name.id].link_info;
                ConcreteType::Named(match wr_typ {
                    Some(WrittenType::Named(wr_named)) => {
                        assert_eq!(wr_named.id, name.id);
                        concretize_global_ref(linker, wr_named, concretizer)?
                    }
                    Some(_) => unreachable!("Can't get Array from Non-Array WrittenType!"), // TODO Fix with Let bindings (#57)
                    None => ConcreteGlobalReference {
                        id: name.id,
                        template_args: target.template_parameters.map(|(_, arg)| match &arg.kind {
                            TemplateKind::Type(_) => {
                                todo!("Abstract Type Args aren't yet supported!")
                            }
                            TemplateKind::Value(_) => {
                                TemplateKind::Value(concretizer.alloc_unknown())
                            }
                        }),
                    },
                })
            }
            AbstractInnerType::Unknown(_) => {
                unreachable!("Should have been resolved already!")
            }
            AbstractInnerType::Interface(_, _) | AbstractInnerType::LocalInterface(_) => unreachable!("Cannot concretize an interface type. Only proper wire types are concretizeable! Should have been caught by typecheck!")
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

    fn execute_global_ref<ID: Copy + Into<GlobalUUID>>(
        &mut self,
        global_ref: &GlobalReference<ID>,
    ) -> ExecutionResult<ConcreteGlobalReference<ID>> {
        let mut concretizer = LocalTypeConcretizer {
            template_args: self.working_on_template_args,
            generation_state: &self.generation_state,
            type_substitutor: &mut self.type_substitutor,
        };
        concretize_global_ref(self.linker, global_ref, &mut concretizer)
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
    ) -> (ConcreteType, bool) {
        match &submodule_link_info.instructions[submodule_port.declaration_instruction] {
            Instruction::Declaration(submodule_decl) => {
                let mut concretizer = SubModuleTypeConcretizer {
                    submodule_template_args,
                    instructions: &submodule_link_info.instructions,
                    type_substitutor,
                };
                let typ = concretize_type_recurse(
                    linker,
                    &submodule_decl.typ.inner,
                    &submodule_decl.typ.rank,
                    Some(&submodule_decl.typ_expr),
                    &mut concretizer,
                )
                .unwrap();
                (typ, false)
            }
            Instruction::Interface(interface_decl) => match interface_decl.interface_kind {
                InterfaceKind::RegularInterface => {
                    unreachable!("Non-conditional interfaces can't have condition")
                }
                InterfaceKind::Action(_) | InterfaceKind::Trigger(_) => (ConcreteType::BOOL, true),
            },
            _ => unreachable!("Ports can only point to Declaration or InterfaceDeclaration"),
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
        let linker_cst = &self.linker.constants[cst_ref.id];
        let concrete_ref = self.execute_global_ref(cst_ref)?;

        concrete_ref
            .report_if_errors(
                self.linker,
                "For executing compile-time constants, all arguments must be fully specified.",
            )
            .map_err(|e| (cst_ref.get_total_span(), e))?;

        if linker_cst.link_info.is_extern == IsExtern::Builtin {
            cst_ref.get_total_span().debug();
            self.evaluate_builtin_constant(&concrete_ref)
                .map_err(|e| (cst_ref.get_total_span(), e))
        } else {
            todo!("Custom Constants");
        }
    }

    fn execute_wire_ref_path(
        &mut self,
        wire_ref: &'l WireReference,
        domain: DomainID,
    ) -> ExecutionResult<(InterfaceID, Span, Vec<RealWirePathElem>)> {
        let mut interface_found = (InterfaceID::MAIN_INTERFACE, wire_ref.root_span);
        let mut path = Vec::new();
        for p in &wire_ref.path {
            match p {
                WireReferencePathElement::ArrayAccess {
                    idx, bracket_span, ..
                } => {
                    let idx_wire = self.get_wire_or_constant_as_wire(*idx, domain)?;
                    path.push(RealWirePathElem::Index {
                        span: *bracket_span,
                        idx_wire,
                    });
                }
                WireReferencePathElement::FieldAccess {
                    name_span,
                    refers_to,
                    ..
                } => match refers_to.get().unwrap() {
                    PathElemRefersTo::Interface(interface) => {
                        interface_found = (*interface, *name_span);
                    }
                },
                WireReferencePathElement::ArraySlice {
                    from,
                    to,
                    bracket_span,
                    ..
                } => {
                    let bounds = match (from, to) {
                        (None, None) => PartialBound::WholeSlice,
                        (None, Some(to)) => PartialBound::To(
                            self.generation_state.get_generation_integer(*to)?.clone(),
                        ),
                        (Some(from), None) => PartialBound::From(
                            self.generation_state.get_generation_integer(*from)?.clone(),
                        ),
                        (Some(from), Some(to)) => PartialBound::Known(
                            self.generation_state.get_generation_integer(*from)?.clone(),
                            self.generation_state.get_generation_integer(*to)?.clone(),
                        ),
                    };

                    path.push(RealWirePathElem::Slice {
                        span: *bracket_span,
                        bounds,
                    });
                }
                WireReferencePathElement::ArrayPartSelect {
                    from,
                    width,
                    bracket_span,
                    direction,
                    ..
                } => {
                    let width = self.generation_state.get_generation_integer(*width)?;
                    let span = *bracket_span;

                    match &self.generation_state[*from] {
                        SubModuleOrWire::SubModule(_) => unreachable!(),
                        SubModuleOrWire::Unassigned => unreachable!(),
                        &SubModuleOrWire::Wire(from_wire) => {
                            path.push(RealWirePathElem::PartSelect {
                                span,
                                from_wire,
                                width: width.clone(),
                                direction: *direction,
                            });
                        }
                        SubModuleOrWire::CompileTimeValue(from) => {
                            let from = from.unwrap_integer().clone();
                            let to = &from + width;
                            path.push(RealWirePathElem::Slice {
                                span,
                                bounds: PartialBound::Known(from, to),
                            });
                        }
                    }
                }
            }
        }
        Ok((interface_found.0, interface_found.1, path))
    }
    // Points to the wire in the hardware that corresponds to the root of this.
    fn wire_ref_to_real_path(
        &mut self,
        wire_ref: &'l WireReference,
        original_instruction: FlatID,
        domain: DomainID,
    ) -> ExecutionResult<(WireID, Vec<RealWirePathElem>)> {
        self.link_info.instructions[original_instruction]
            .get_span()
            .debug();
        let (port_interface, port_span, path) = self.execute_wire_ref_path(wire_ref, domain)?;
        let wire_id = match &wire_ref.root {
            &WireReferenceRoot::LocalDecl(decl_id) => {
                let _ = self.link_info.instructions[decl_id].unwrap_declaration();
                self.get_wire_or_constant_as_wire(decl_id, domain)?
            }
            WireReferenceRoot::LocalSubmodule(submod_id) => {
                let submod = self.link_info.instructions[*submod_id].unwrap_submodule();
                let submod_md = &self.linker.modules[submod.module_ref.id];
                let_unwrap!(
                    Some(InterfaceDeclKind::SinglePort(port_decl)),
                    submod_md.interfaces[port_interface].declaration_instruction
                );
                let port_decl = submod_md.link_info.instructions[port_decl].unwrap_declaration();
                let_unwrap!(
                    DeclarationKind::Port {
                        port_id,
                        domain,
                        ..
                    },
                    port_decl.decl_kind
                );
                let domain = submod.local_domain_map[domain].unwrap_physical();
                let submod_id = self.generation_state[*submod_id].unwrap_submodule_instance();
                self.get_submodule_port(submod_id, port_id, Some(port_span), domain)
            }
            WireReferenceRoot::NamedConstant(cst) => {
                let value = self.get_named_constant_value(cst)?;

                self.alloc_wire_for_const(
                    value,
                    wire_ref.get_root_typ(),
                    original_instruction,
                    domain,
                    wire_ref.root_span,
                )?
            }
            WireReferenceRoot::LocalInterface(_) | WireReferenceRoot::NamedModule(_) => {
                caught_by_typecheck!("Can't turn an inline module into a wire")
            }
            WireReferenceRoot::Error => caught_by_typecheck!(),
        };
        Ok((wire_id, path))
    }

    fn instantiate_write_to_wire(
        &mut self,
        write_to_wire: WireID,
        to_path: Vec<RealWirePathElem>,
        from: WireID,
        num_regs: i64,
        write_span: Span,
    ) {
        let target_wire = &mut self.wires[write_to_wire];

        let RealWireDataSource::Multiplexer {
            is_state: _,
            sources,
        } = &mut target_wire.source
        else {
            caught_by_typecheck!("Should only be a writeable wire here")
        };

        sources.push(MultiplexerSource {
            to_path,
            num_regs,
            from,
            condition: self.condition_stack.clone().into_boxed_slice(),
            write_span,
        });
    }

    fn write_non_generative(
        &mut self,
        write_to: &'l WriteTo,
        original_instruction: FlatID,
        from: WireID,
        write_span: Span,
        domain: DomainID,
    ) -> ExecutionResult<()> {
        let_unwrap!(
            WriteModifiers::Connection {
                num_regs,
                regs_span: _,
            },
            &write_to.write_modifiers
        );
        let (target_wire, path) =
            self.wire_ref_to_real_path(&write_to.to, original_instruction, domain)?;

        self.instantiate_write_to_wire(target_wire, path, from, *num_regs, write_span);
        Ok(())
    }

    fn write_generative(&mut self, write_to: &'l WriteTo, value: Value) -> ExecutionResult<()> {
        let root_decl_id = write_to.to.root.unwrap_local_decl();
        match &write_to.write_modifiers {
            WriteModifiers::Connection { .. } => {
                let_unwrap!(
                    SubModuleOrWire::CompileTimeValue(v_writable),
                    &mut self.generation_state[root_decl_id]
                );

                let mut new_val = std::mem::replace(v_writable, Value::Unset);
                self.generation_state
                    .write_gen_variable(&mut new_val, &write_to.to.path, value)?;

                let_unwrap!(
                    SubModuleOrWire::CompileTimeValue(v_writable),
                    &mut self.generation_state[root_decl_id]
                );
                *v_writable = new_val;
            }
            WriteModifiers::Initial { initial_kw_span: _ } => {
                let root_wire = self.generation_state[root_decl_id].unwrap_wire();
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
            is_port: None,
        }))
    }
    fn alloc_bool(&mut self, v: bool, original_instruction: FlatID, domain: DomainID) -> WireID {
        self.wires.alloc(RealWire {
            typ: ConcreteType::BOOL,
            source: RealWireDataSource::Constant {
                value: Value::Bool(v),
            },
            original_instruction,
            domain,
            name: self.unique_name_producer.get_unique_name(""),
            specified_latency: CALCULATE_LATENCY_LATER,
            absolute_latency: CALCULATE_LATENCY_LATER,
            is_port: None,
        })
    }

    /// Converts constants to wires implicitly.
    fn get_wire_or_constant_as_wire(
        &mut self,
        original_instruction: FlatID,
        domain: DomainID,
    ) -> ExecutionResult<WireID> {
        match &self.generation_state[original_instruction] {
            SubModuleOrWire::SubModule(_) => unreachable!(),
            SubModuleOrWire::Unassigned => unreachable!(),
            SubModuleOrWire::Wire(w) => Ok(*w),
            SubModuleOrWire::CompileTimeValue(v) => {
                let value = v.clone();
                let (typ, span) = match &self.link_info.instructions[original_instruction] {
                    Instruction::Declaration(decl) => (decl.typ.deref(), decl.name_span),
                    Instruction::Expression(expr) => {
                        let expr = expr.as_single_output_expr().unwrap();
                        (expr.typ, expr.span)
                    }
                    _ => unreachable!(),
                };
                self.alloc_wire_for_const(value, typ, original_instruction, domain, span)
            }
        }
    }

    /// Allocates ports on first use, to see which ports are used, and to determine instantiation based on this
    fn get_submodule_port(
        &mut self,
        sub_module_id: SubModuleID,
        port_id: PortID,
        port_name_span: Option<Span>,
        domain: DomainID,
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
            let write_span = submod_instance.get_span(self.link_info);
            let source = match port_data.direction {
                Direction::Input => RealWireDataSource::Multiplexer {
                    is_state: None,
                    sources: Vec::new(),
                },
                Direction::Output => RealWireDataSource::ReadOnly,
            };
            let (typ, is_condition) = Self::concretize_submodule_port_type(
                &mut self.type_substitutor,
                self.linker,
                port_data,
                &submod_instance.refers_to.template_args,
                &submod_md.link_info,
            );
            let original_instruction = submod_instance.original_instruction;
            let new_wire = self.wires.alloc(RealWire {
                source,
                original_instruction,
                domain,
                typ,
                name: self
                    .unique_name_producer
                    .get_unique_name(format!("_{}_{}", submod_instance.name, port_data.name)),
                specified_latency: CALCULATE_LATENCY_LATER,
                absolute_latency: CALCULATE_LATENCY_LATER,
                is_port: None,
            });

            if is_condition && port_data.direction == Direction::Input {
                let false_wire = self.alloc_bool(false, original_instruction, domain);
                let_unwrap!(
                    RealWireDataSource::Multiplexer { sources, .. },
                    &mut self.wires[new_wire].source
                );
                sources.push(MultiplexerSource {
                    to_path: Vec::new(),
                    num_regs: 0,
                    from: false_wire,
                    condition: Box::new([]),
                    write_span,
                });
            }

            let name_refs = if let Some(sp) = port_name_span {
                vec![sp]
            } else {
                Vec::new()
            };

            self.submodules[sub_module_id].port_map[port_id] = Some(SubModulePort {
                maps_to_wire: new_wire,
                name_refs,
            });
            new_wire
        }
    }
    fn get_submodule_interface(
        &mut self,
        submod_id: SubModuleID,
        interface_id: InterfaceID,
        interface_span: Span,
        domain: DomainID,
    ) -> InterfaceWires {
        add_to_small_set(
            &mut self.submodules[submod_id].interface_call_sites[interface_id],
            interface_span,
        );
        let md = &self.linker.modules[self.submodules[submod_id].refers_to.id];
        let interface = &md.interfaces[interface_id];
        let_unwrap!(
            Some(InterfaceDeclKind::Interface(interface_id)),
            interface.declaration_instruction
        );

        let interface = md.link_info.instructions[interface_id].unwrap_interface();

        let condition_wire = match interface.interface_kind {
            InterfaceKind::Action(condition_port) | InterfaceKind::Trigger(condition_port) => Some(
                self.get_submodule_port(submod_id, condition_port, Some(interface_span), domain),
            ),
            InterfaceKind::RegularInterface => None,
        };

        let inputs = interface
            .inputs
            .iter()
            .map(|decl_id| {
                let (port, direction) = md.get_port_for_decl(*decl_id);
                // Triggers have Outputs as their "function input"
                // assert_eq!(direction, Direction::Input);
                self.get_submodule_port(submod_id, port, None, domain)
            })
            .collect();
        let outputs = interface
            .outputs
            .iter()
            .map(|decl_id| {
                let (port, direction) = md.get_port_for_decl(*decl_id);
                // Triggers have Inputs as their "function output"
                // assert_eq!(direction, Direction::Output);
                self.get_submodule_port(submod_id, port, None, domain)
            })
            .collect();

        InterfaceWires {
            condition_wire,
            inputs,
            outputs,
            interface_span,
        }
    }

    fn get_interface(
        &mut self,
        interface_ref: &'l WireReference,
        original_instruction: FlatID,
        domain: DomainID,
    ) -> ExecutionResult<InterfaceWires> {
        match &interface_ref.root {
            WireReferenceRoot::LocalSubmodule(submod_decl_id) => {
                let submod_id = self.generation_state[*submod_decl_id].unwrap_submodule_instance();

                let (interface, name_span, path) =
                    self.execute_wire_ref_path(interface_ref, domain)?;

                Ok(self.get_submodule_interface(submod_id, interface, name_span, domain))
            }
            WireReferenceRoot::NamedModule(module_ref) => {
                let submod_id = self.instantiate_submodule(module_ref, "", original_instruction)?;

                assert!(interface_ref.path.is_empty());
                Ok(self.get_submodule_interface(
                    submod_id,
                    InterfaceID::MAIN_INTERFACE,
                    module_ref.get_total_span(),
                    domain,
                ))
            }
            WireReferenceRoot::LocalInterface(interface_decl) => {
                let interface = self.link_info.instructions[*interface_decl].unwrap_interface();
                if !interface_ref.path.is_empty() {
                    todo!("Can't yet work with sub-interfaces");
                }

                let condition_wire = match interface.interface_kind {
                    InterfaceKind::RegularInterface => {
                        interface.decl_span.debug();
                        unreachable!("Can't call interfaces locally")
                    }
                    InterfaceKind::Action(_) => unreachable!("Can't call actions locally"),
                    InterfaceKind::Trigger(_trigger_port) => {
                        Some(self.generation_state[*interface_decl].unwrap_wire())
                    }
                };

                let interface = self.link_info.instructions[*interface_decl].unwrap_interface();
                let inputs = interface
                    .inputs
                    .iter()
                    .map(|input_decl| self.generation_state[*input_decl].unwrap_wire())
                    .collect();
                let outputs = interface
                    .outputs
                    .iter()
                    .map(|input_decl| self.generation_state[*input_decl].unwrap_wire())
                    .collect();

                Ok(InterfaceWires {
                    condition_wire,
                    inputs,
                    outputs,
                    interface_span: interface_ref.root_span,
                })
            }
            WireReferenceRoot::LocalDecl(_)
            | WireReferenceRoot::NamedConstant(_)
            | WireReferenceRoot::Error => caught_by_typecheck!(),
        }
    }

    fn expression_to_real_wire(
        &mut self,
        expression: &'l Expression,
        original_instruction: FlatID,
        domain: DomainID,
    ) -> ExecutionResult<Vec<WireID>> {
        let source = match &expression.source {
            ExpressionSource::WireRef(wire_ref) => {
                let (root_wire, path) =
                    self.wire_ref_to_real_path(wire_ref, original_instruction, domain)?;

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
                let func_expr =
                    self.link_info.instructions[fc.func_wire_ref].unwrap_subexpression();
                let_unwrap!(ExpressionSource::WireRef(f_wr), &func_expr.source);
                let func_interface = self.get_interface(f_wr, original_instruction, domain)?;

                if let Some(condition) = func_interface.condition_wire {
                    let true_wire = self.alloc_bool(true, original_instruction, domain);

                    self.instantiate_write_to_wire(
                        condition,
                        Vec::new(),
                        true_wire,
                        0,
                        func_interface.interface_span,
                    );
                }

                for (port_wire, arg) in zip_eq(&func_interface.inputs, &fc.arguments) {
                    let arg_span = self.link_info.instructions[*arg].get_span();
                    let from = self.get_wire_or_constant_as_wire(*arg, domain)?;
                    self.instantiate_write_to_wire(*port_wire, Vec::new(), from, 0, arg_span);
                }

                return Ok(func_interface.outputs);
            }
            ExpressionSource::ArrayConstruct(arr) => {
                let mut array_wires = Vec::with_capacity(arr.len());
                for v_id in arr {
                    let wire_id = self.get_wire_or_constant_as_wire(*v_id, domain)?;
                    array_wires.push(wire_id);
                }
                RealWireDataSource::ConstructArray { array_wires }
            }
            ExpressionSource::Literal(_) => {
                unreachable!("Constant cannot be non-compile-time");
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
            is_port: None,
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
        let typ = self.concretize_type(&wire_decl.typ, &wire_decl.typ_expr)?;

        Ok(if wire_decl.decl_kind.is_generative() {
            let value: Value =
                if let DeclarationKind::TemplateParameter(template_id) = wire_decl.decl_kind {
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
            let source = if wire_decl.decl_kind.is_io_port() == Some(Direction::Input) {
                RealWireDataSource::ReadOnly
            } else {
                let is_state = if wire_decl.decl_kind.is_state() {
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

            let is_port = if let DeclarationKind::Port { direction, .. } = &wire_decl.decl_kind {
                Some(*direction)
            } else {
                None
            };

            let wire_id = self.wires.alloc(RealWire {
                name: self.unique_name_producer.get_unique_name(&wire_decl.name),
                typ,
                original_instruction,
                domain: wire_decl.domain.get().unwrap_physical(),
                source,
                specified_latency,
                absolute_latency: CALCULATE_LATENCY_LATER,
                is_port,
            });
            SubModuleOrWire::Wire(wire_id)
        })
    }

    fn compute_compile_time_wireref(&mut self, wire_ref: &WireReference) -> ExecutionResult<Value> {
        let work_on_value = match &wire_ref.root {
            WireReferenceRoot::LocalDecl(decl_id) => {
                Cow::Borrowed(self.generation_state.get_generation_value(*decl_id)?)
            }
            WireReferenceRoot::NamedConstant(cst) => {
                Cow::Owned(self.get_named_constant_value(cst)?)
            }
            WireReferenceRoot::LocalSubmodule(_)
            | WireReferenceRoot::NamedModule(_)
            | WireReferenceRoot::LocalInterface(_) => {
                todo!("Don't yet support compile time functions")
            }
            WireReferenceRoot::Error => caught_by_typecheck!(),
        };

        self.generation_state
            .read_from_path(&work_on_value, &wire_ref.path)
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
            ExpressionSource::Literal(value) => value.clone(),
        })
    }

    fn instantiate_submodule(
        &mut self,
        module_ref: &GlobalReference<ModuleUUID>,
        name_origin: &str,
        original_instruction: FlatID,
    ) -> ExecutionResult<SubModuleID> {
        let sub_module = &self.linker.modules[module_ref.id];

        let port_map = sub_module.ports.map(|_| None);
        let interface_call_sites = sub_module.interfaces.map(|_| Vec::new());

        let refers_to = self.execute_global_ref(module_ref)?;

        Ok(self.submodules.alloc(SubModule {
            original_instruction,
            instance: OnceCell::new(),
            refers_to,
            port_map,
            interface_call_sites,
            name: self.unique_name_producer.get_unique_name(name_origin),
        }))
    }

    fn instantiate_expression(
        &mut self,
        expr: &'l Expression,
        original_instruction: FlatID,
    ) -> ExecutionResult<SubModuleOrWire> {
        if let ExpressionOutput::SubExpression(typ) = &expr.output {
            if typ.inner.is_interface() {
                // Interface execution is up to whoever calls it
                return Ok(SubModuleOrWire::Unassigned);
            }
        }
        Ok(match expr.domain.get() {
            DomainType::Generative => {
                let value_computed = self.compute_compile_time(expr)?;
                match &expr.output {
                    ExpressionOutput::SubExpression(_full_type) => {} // Simply returning value_computed is enough
                    ExpressionOutput::MultiWrite(write_tos) => {
                        if let Some(single_write) = write_tos.first() {
                            match single_write.target_domain.get() {
                                DomainType::Generative => {
                                    self.write_generative(
                                        single_write,
                                        value_computed.clone(), // We do an extra clone, maybe not needed, such that we can show the value in GenerationState
                                    )?;
                                }
                                DomainType::Physical(domain) => {
                                    let value_as_wire = self.alloc_wire_for_const(
                                        value_computed.clone(),
                                        &single_write.to.output_typ,
                                        original_instruction,
                                        domain,
                                        expr.span,
                                    )?;
                                    self.write_non_generative(
                                        single_write,
                                        original_instruction,
                                        value_as_wire,
                                        single_write.to_span,
                                        domain,
                                    )?;
                                }
                                DomainType::Unknown(_) => caught_by_typecheck!(),
                            }
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
                            return Ok(SubModuleOrWire::Unassigned); // See no errors on zero outputs (#79)
                        }
                        for (expr_output, write) in zip_eq(output_wires, write_tos) {
                            self.write_non_generative(
                                write,
                                original_instruction,
                                expr_output,
                                write.to_span,
                                domain,
                            )?;
                        }
                        SubModuleOrWire::Unassigned
                    }
                }
            }
            DomainType::Unknown(_) => caught_by_typecheck!(),
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
                    SubModuleOrWire::SubModule(self.instantiate_submodule(
                        &submodule.module_ref,
                        &submodule.name,
                        original_instruction,
                    )?)
                }
                Instruction::Declaration(wire_decl) => {
                    self.instantiate_declaration(wire_decl, original_instruction)?
                }
                Instruction::Expression(expr) => {
                    self.instantiate_expression(expr, original_instruction)?
                }
                Instruction::IfStatement(if_stm) => {
                    if if_stm.is_generative {
                        let condition_val = self
                            .generation_state
                            .get_generation_value(if_stm.condition)?;
                        let run_range = if condition_val.unwrap_bool() {
                            if_stm.then_block
                        } else {
                            if_stm.else_block
                        };
                        self.instantiate_code_block(run_range)?;
                    } else {
                        let condition_expr =
                            self.link_info.instructions[if_stm.condition].unwrap_subexpression();

                        if condition_expr.typ.inner.is_interface() {
                            let wr_expr = self.link_info.instructions[if_stm.condition]
                                .unwrap_subexpression();
                            let_unwrap!(ExpressionSource::WireRef(interface), &wr_expr.source);
                            let domain = wr_expr.domain.unwrap_physical();
                            let trig_interface =
                                self.get_interface(interface, original_instruction, domain)?;

                            self.condition_stack.push(ConditionStackElem {
                                condition_wire: trig_interface.condition_wire.unwrap(),
                                inverse: false,
                            });

                            self.instantiate_code_block(if_stm.then_block)?;

                            for (port_wire, binding) in
                                zip_eq(&trig_interface.inputs, &if_stm.bindings_read_only)
                            {
                                let binding_span = self.link_info.instructions[*binding].get_span();
                                let binding = self.generation_state[*binding].unwrap_wire();
                                self.instantiate_write_to_wire(
                                    binding,
                                    Vec::new(),
                                    *port_wire,
                                    0,
                                    binding_span,
                                );
                            }

                            for (port_wire, binding) in
                                zip_eq(&trig_interface.outputs, &if_stm.bindings_writable)
                            {
                                let binding_span = self.link_info.instructions[*binding].get_span();
                                let binding = self.generation_state[*binding].unwrap_wire();
                                self.instantiate_write_to_wire(
                                    *port_wire,
                                    Vec::new(),
                                    binding,
                                    0,
                                    binding_span,
                                );
                            }
                        } else {
                            let condition_wire =
                                self.generation_state[if_stm.condition].unwrap_wire();
                            self.condition_stack.push(ConditionStackElem {
                                condition_wire,
                                inverse: false,
                            });
                            self.instantiate_code_block(if_stm.then_block)?;
                        }
                        if !if_stm.else_block.is_empty() {
                            self.condition_stack.last_mut().unwrap().inverse = true;
                            self.instantiate_code_block(if_stm.else_block)?;
                        }

                        // Get rid of the condition
                        let _ = self.condition_stack.pop().unwrap();
                    }
                    instruction_range.skip_to(if_stm.else_block.1);
                    continue;
                }
                Instruction::Interface(interface) => {
                    if interface.interface_kind.is_conditional() {
                        let specified_latency =
                            self.get_specified_latency(interface.latency_specifier)?;

                        let is_port = match interface.interface_kind {
                            InterfaceKind::RegularInterface => unreachable!(),
                            InterfaceKind::Action(_) => Some(Direction::Input),
                            InterfaceKind::Trigger(_) => Some(Direction::Output),
                        };

                        let source = match is_port {
                            Some(Direction::Input) => RealWireDataSource::ReadOnly,
                            Some(Direction::Output) | None => RealWireDataSource::Multiplexer {
                                is_state: None,
                                sources: Vec::new(),
                            },
                        };
                        let condition_wire = self.wires.alloc(RealWire {
                            name: self.unique_name_producer.get_unique_name(&interface.name),
                            typ: ConcreteType::BOOL,
                            original_instruction,
                            domain: interface.domain.unwrap_physical(),
                            source,
                            specified_latency,
                            absolute_latency: CALCULATE_LATENCY_LATER,
                            is_port,
                        });

                        self.condition_stack.push(ConditionStackElem {
                            condition_wire,
                            inverse: false,
                        });
                        self.instantiate_code_block(interface.then_block)?;

                        if !interface.else_block.is_empty() {
                            self.condition_stack.last_mut().unwrap().inverse = true;
                            self.instantiate_code_block(interface.else_block)?;
                        }

                        // Get rid of the condition
                        let _ = self.condition_stack.pop().unwrap();

                        instruction_range.skip_to(interface.else_block.1);

                        SubModuleOrWire::Wire(condition_wire)
                    } else {
                        SubModuleOrWire::Unassigned
                    }
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

            if crate::debug::is_enabled("print-generation-state") {
                println!("After running {original_instruction:?}");
                for (id, g) in &self.generation_state.generation_state {
                    println!("{id:?}: {g:?}");
                }
            }
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
