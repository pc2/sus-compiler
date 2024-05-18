

use std::{cmp::max, iter::zip, ops::Deref};

use num::BigInt;

use crate::{
    arena_alloc::{FlatAlloc, UUIDRange, UUID},
    errors::ErrorCollector,
    file_position::Span,
    flattening::{BinaryOperator, Declaration, FlatID, FlatIDMarker, FlatIDRange, IdentifierType, Instruction, Module, PortIDMarker, UnaryOperator, WireInstance, WireReference, WireReferencePathElement, WireReferenceRoot, WireSource, WriteModifiers},
    instantiation::latency_algorithm::{convert_fanin_to_fanout, solve_latencies, FanInOut, LatencyCountingError},
    linker::{Linker, NamedConstant},
    typing::{ConcreteType, WrittenType, BOOL_CONCRETE_TYPE, INT_CONCRETE_TYPE},
    value::{compute_binary_op, compute_unary_op, TypedValue, Value}
};

use self::list_of_lists::ListOfLists;

use super::*;

struct PathMuxSource<'s> {
    to_wire : &'s RealWire,
    to_latency : i64,
    mux_input : &'s MultiplexerSource
}
fn gather_all_mux_inputs<'w>(wires : &'w FlatAlloc<RealWire, WireIDMarker>, conflict_iter : &[SpecifiedLatency]) -> Vec<PathMuxSource<'w>> {
    let mut connection_list = Vec::new();
    for window in conflict_iter.windows(2) {
        let [from, to] = window else {unreachable!()};
        let from_wire_id = WireID::from_hidden_value(from.wire);
        //let from_wire = &self.wires[from_wire_id];
        let to_wire_id = WireID::from_hidden_value(to.wire);
        let to_wire = &wires[to_wire_id];
        let RealWireDataSource::Multiplexer { is_state:_, sources } = &to_wire.source else {continue}; // We can only name multiplexers

        //let decl_id = to_wire.original_instruction;
        //let Instruction::Declaration(decl) = &self.instructions[decl_id] else {unreachable!()};

        for s in sources {
            let mut predecessor_found = false;
            s.for_each_source(|source| {
                if source == from_wire_id {
                    predecessor_found = true;
                }
            });
            if predecessor_found {
                connection_list.push(PathMuxSource{to_wire, mux_input : s, to_latency : to.latency});
            }
        }
    }
    connection_list
}

fn write_path_elem_to_string(result : &mut String, decl_name : &str, to_absolute_latency : i64, prev_absolute_latency : i64) {
    use std::fmt::Write;

    let delta_latency = to_absolute_latency - prev_absolute_latency;

    let plus_sign = if delta_latency >= 0 {"+"} else {""};

    writeln!(result, "-> {decl_name}'{to_absolute_latency} ({plus_sign}{delta_latency})").unwrap();
}
fn make_path_info_string(writes : &[PathMuxSource<'_>], from_latency : i64, from_name : &str) -> String {
   let mut prev_decl_absolute_latency = from_latency;
    let mut result = format!("{from_name}'{prev_decl_absolute_latency}\n");

    for wr in writes {
        let decl_name = &wr.to_wire.name;

        let to_absolute_latency = wr.to_latency;
        
        write_path_elem_to_string(&mut result, &decl_name, to_absolute_latency, prev_decl_absolute_latency);

        prev_decl_absolute_latency = to_absolute_latency;
    }

    result
}

fn filter_unique_write_flats<'w>(writes : &'w [PathMuxSource<'w>], instructions : &'w FlatAlloc<Instruction, FlatIDMarker>) -> Vec<&'w crate::flattening::Write> {
    let mut result : Vec<&'w crate::flattening::Write> = Vec::new();
    for w in writes {
        let original_write = instructions[w.mux_input.from.original_connection].unwrap_write();
        
        if !result.iter().any(|found_write| std::ptr::eq(*found_write, original_write)) {result.push(original_write)}
    }
    result
}

type ExecutionResult<T> = Result<T, (Span, String)>;

macro_rules! caught_by_typecheck {
    ($arg:literal) => {unreachable!("{} should have been caught by typecheck!", $arg)};
    () => {unreachable!("Should have been caught by typecheck!")};
}

fn write_gen_variable<'t, 'p>(mut target : &'t mut Value, conn_path : &'p [RealWirePathElem], to_write : Value) -> ExecutionResult<()> {
    for elem in conn_path {
        match elem {
            RealWirePathElem::ConstArrayWrite{span, idx} => {
                let Value::Array(a_box) = target else {caught_by_typecheck!("Non-array")};
                let array_len = a_box.len();
                let Some(tt) = usize::try_from(idx).ok().and_then(|pos| a_box.get_mut(pos)) else {
                    return Err((span.inner_span(), format!("Index {idx} is out of bounds for this array of size {}", array_len)))
                };
                target = tt
            }
            RealWirePathElem::MuxArrayWrite {span:_,  idx_wire:_ } => {
                caught_by_typecheck!("Non-generative array access");
            }
        }
    }
    *target = to_write;
    Ok(())
}
pub fn array_access(tv : &TypedValue, idx : &BigInt, span : BracketSpan) -> ExecutionResult<TypedValue> {
    let typ = tv.typ.down_array().clone();

    let Value::Array(arr) = &tv.value else {caught_by_typecheck!("Value must be an array")};

    if let Some(elem) = usize::try_from(idx).ok().and_then(|idx| arr.get(idx)) {
        Ok(TypedValue{typ, value : elem.clone()})
    } else {
        Err((span.outer_span(), format!("Compile-Time Array index is out of range: idx: {idx}, array size: {}", arr.len())))
    }
}

struct InstantiatedWireRef {
    root : RealWireRefRoot,
    path : Vec<RealWirePathElem>
}

/// Executes the generative code and produces a netlist from it
/// 
/// Stops generating at the first error. 
/// 
/// As for typing, it only instantiates written types and leaves the rest for further typechecking. 
struct InstantiationContext<'fl, 'l> {
    name : String,
    generation_state : FlatAlloc<SubModuleOrWire, FlatIDMarker>,
    wires : FlatAlloc<RealWire, WireIDMarker>,
    submodules : FlatAlloc<SubModule, SubModuleIDMarker>,
    specified_latencies : Vec<(WireID, i64)>,

    interface_ports : FlatAlloc<Option<InstantiatedPort>, PortIDMarker>,
    errors : ErrorCollector<'l>,

    md : &'fl Module,
    linker : &'l Linker,
}

impl<'fl, 'l> InstantiationContext<'fl, 'l> {
    fn span_of(&self, v : FlatID) -> Span {
        self.md.instructions[v].unwrap_wire().span
    }
    fn get_generation_value(&self, v : FlatID) -> ExecutionResult<&TypedValue> {
        if let SubModuleOrWire::CompileTimeValue(vv) = &self.generation_state[v] {
            if let Value::Unset | Value::Error = &vv.value {
                Err((self.span_of(v), format!("This variable is set but it's {vv:?}!")))
            } else {
                Ok(vv)
            }
        } else {
            Err((self.span_of(v), "This variable is not set at this point!".to_owned()))
        }
    }
    fn get_generation_integer(&self, idx : FlatID) -> ExecutionResult<&BigInt> {
        let val = self.get_generation_value(idx)?;
        Ok(val.unwrap_integer())
    }
    fn get_generation_small_int<INT : for<'v> TryFrom<&'v BigInt>>(&self, idx : FlatID) -> ExecutionResult<INT> {
        let val = self.get_generation_value(idx)?;
        let val_as_int = val.unwrap_integer();
        INT::try_from(val_as_int).map_err(|_| {
            (self.span_of(idx), format!("Value {val_as_int} does not fit in {}", std::any::type_name::<INT>()))
        })
    }

    /// Uses the current context to turn a [WrittenType] into a [ConcreteType]. 
    /// 
    /// Failures are fatal. 
    fn concretize_type(&self, typ : &WrittenType) -> ExecutionResult<ConcreteType> {
        Ok(match typ {
            WrittenType::Error(_) => unreachable!("Bad types should be caught in flattening: {}", typ.to_string(&self.linker.types)),
            WrittenType::Named(_, id) => {
                ConcreteType::Named(*id)
            }
            WrittenType::Array(_, arr_box) => {
                let (arr_content_typ, arr_size_wire, _bracket_span) = arr_box.deref();
                let inner_typ = self.concretize_type(arr_content_typ)?;
                let arr_size = self.get_generation_integer(*arr_size_wire)?;
                ConcreteType::Array(Box::new((inner_typ, ConcreteType::Value(Value::Integer(arr_size.clone())))))
            }
        })
    }

    fn instantiate_wire_ref(&self, wire_ref : &WireReference) -> ExecutionResult<InstantiatedWireRef> {
        // Later on, potentially allow module arrays
        let mut path = Vec::new();

        let root = match &wire_ref.root {
            &WireReferenceRoot::LocalDecl(decl_id, _) => {
                match &self.generation_state[decl_id] {
                    SubModuleOrWire::Wire(w) => RealWireRefRoot::Wire(*w),
                    SubModuleOrWire::CompileTimeValue(_) => RealWireRefRoot::Generative(decl_id),
                    SubModuleOrWire::SubModule(_) => unreachable!(),
                    SubModuleOrWire::Unnasigned => unreachable!(),
                }
            }
            WireReferenceRoot::NamedConstant(cst, _) => {
                let NamedConstant::Builtin{name:_, val} = &self.linker.constants[*cst];
                RealWireRefRoot::Constant(val.clone())
            }
            WireReferenceRoot::SubModulePort(port) => {
                let sm = &self.submodules[self.generation_state[port.submodule_flat].unwrap_submodule_instance()];
                RealWireRefRoot::Wire(sm.port_map[port.port])
            }
        };

        for v in &wire_ref.path {
            match v {
                &WireReferencePathElement::ArrayIdx{idx, bracket_span} => {
                    match &self.generation_state[idx] {
                        SubModuleOrWire::SubModule(_) => unreachable!(),
                        SubModuleOrWire::Unnasigned => unreachable!(),
                        &SubModuleOrWire::Wire(idx_wire) => {
                            assert!(self.wires[idx_wire].typ == INT_CONCRETE_TYPE);
        
                            path.push(RealWirePathElem::MuxArrayWrite{ span:bracket_span, idx_wire});
                        }
                        SubModuleOrWire::CompileTimeValue(cv) => {
                            path.push(RealWirePathElem::ConstArrayWrite{
                                idx : cv.value.unwrap_integer().clone(),
                                span : bracket_span
                            });
                        }
                    }
                }
            }
        }

        Ok(InstantiatedWireRef{root, path})
    }

    fn process_connection(&mut self, wire_ref_inst : InstantiatedWireRef, write_modifiers : &WriteModifiers, conn_from : FlatID, original_connection : FlatID, condition : Option<WireID>) -> ExecutionResult<()> {
        match write_modifiers {
            WriteModifiers::Connection{num_regs, regs_span : _} => {
                match &wire_ref_inst.root {
                    RealWireRefRoot::Wire(write_to_wire) => {
                        let from = ConnectFrom {
                            num_regs : *num_regs,
                            from : self.get_wire_or_constant_as_wire(conn_from),
                            condition,
                            original_connection
                        };

                        let RealWireDataSource::Multiplexer{is_state : _, sources} = &mut self.wires[*write_to_wire].source else {caught_by_typecheck!("Should only be a writeable wire here")};

                        sources.push(MultiplexerSource{from, path : wire_ref_inst.path});
                    }
                    RealWireRefRoot::Generative(decl_id) => {
                        let found_v = self.generation_state[conn_from].unwrap_generation_value().clone();

                        let SubModuleOrWire::CompileTimeValue(v_writable) = &mut self.generation_state[*decl_id] else {caught_by_typecheck!()};
                        write_gen_variable(&mut v_writable.value, &wire_ref_inst.path, found_v.value)?;
                    }
                    RealWireRefRoot::Constant(_) => {
                        unreachable!("Cannot assign to constants. This should have been caught in Abstract Typecheck!")
                    },
                }
            }
            WriteModifiers::Initial{initial_kw_span : _} => {
                let found_v = self.get_generation_value(conn_from)?.clone();

                let RealWireDataSource::Multiplexer{is_state : Some(initial_value), sources : _} = &mut self.wires[wire_ref_inst.root.unwrap_wire()].source else {caught_by_typecheck!()};
                write_gen_variable(initial_value, &wire_ref_inst.path, found_v.value)?;
            }
        }
        Ok(())
    }
    fn compute_compile_time_wireref(&self, wire_ref_inst : InstantiatedWireRef) -> ExecutionResult<TypedValue> {
        let mut work_on_value = match &wire_ref_inst.root {
            &RealWireRefRoot::Generative(decl_id) => {
                self.get_generation_value(decl_id)?.clone()
            }
            RealWireRefRoot::Constant(cst) => {
                cst.clone()
            }
            RealWireRefRoot::Wire(_) => {
                caught_by_typecheck!("Write to a non-generative wire at compiletime.")
            }
        };

        for path_elem in &wire_ref_inst.path {
            work_on_value = match path_elem {
                RealWirePathElem::ConstArrayWrite { span, idx } => {
                    array_access(&work_on_value, &idx, *span)?
                }
                RealWirePathElem::MuxArrayWrite { span, idx_wire:_ } => {
                    span.outer_span().debug();
                    caught_by_typecheck!("Write to a non-generative wire at compiletime.")
                }
            }
        }

        Ok(work_on_value)
    }
    fn compute_compile_time(&self, wire_inst : &WireInstance) -> ExecutionResult<TypedValue> {
        Ok(match &wire_inst.source {
            WireSource::WireRef(wire_ref) => {
                let wire_ref_instance = self.instantiate_wire_ref(wire_ref)?;
                self.compute_compile_time_wireref(wire_ref_instance)?
            }
            &WireSource::UnaryOp{op, right} => {
                let right_val = self.get_generation_value(right)?;
                compute_unary_op(op, right_val)
            }
            &WireSource::BinaryOp{op, left, right} => {
                let left_val = self.get_generation_value(left)?;
                let right_val = self.get_generation_value(right)?;
                compute_binary_op(left_val, op, right_val)
            }
            WireSource::Constant(value) => TypedValue::from_value(value.clone())
        })
    }
    fn get_unique_name(&self) -> String {
        format!("_{}", self.wires.get_next_alloc_id().get_hidden_value())
    }
    fn alloc_wire_for_const(&mut self, value : TypedValue, original_instruction : FlatID) -> WireID {
        let name = self.get_unique_name();
        self.wires.alloc(RealWire{source : RealWireDataSource::Constant{value : value.value}, original_instruction, typ : value.typ, name, absolute_latency : CALCULATE_LATENCY_LATER, needed_until : CALCULATE_LATENCY_LATER})
    }
    fn get_wire_or_constant_as_wire(&mut self, original_instruction : FlatID) -> WireID {
        match &self.generation_state[original_instruction] {
            SubModuleOrWire::SubModule(_) => unreachable!(),
            SubModuleOrWire::Unnasigned => unreachable!(),
            SubModuleOrWire::Wire(w) => *w,
            SubModuleOrWire::CompileTimeValue(v) => {
                let value = v.clone();

                self.alloc_wire_for_const(value, original_instruction)
            }
        }
    }
    fn get_wire_ref_root_as_wire(&mut self, root : RealWireRefRoot, original_instruction : FlatID) -> WireID {
        match root {
            RealWireRefRoot::Wire(w) => w,
            RealWireRefRoot::Generative(decl_id) => {
                let value = self.generation_state[decl_id].unwrap_generation_value().clone();
                self.alloc_wire_for_const(value, decl_id)
            }
            RealWireRefRoot::Constant(value) => self.alloc_wire_for_const(value, original_instruction)
        }
    }
    fn wire_to_real_wire(&mut self, w: &WireInstance, original_instruction : FlatID) -> ExecutionResult<WireID> {
        let source = match &w.source {
            WireSource::WireRef(wire_ref) => {
                let inst = self.instantiate_wire_ref(wire_ref)?;
                let root_wire = self.get_wire_ref_root_as_wire(inst.root, original_instruction);

                if inst.path.is_empty() { // Little optimization reduces instructions
                    return Ok(root_wire)
                }

                RealWireDataSource::Select { root: root_wire, path : inst.path }
            }
            &WireSource::UnaryOp{op, right} => {
                let right = self.get_wire_or_constant_as_wire(right);
                RealWireDataSource::UnaryOp{op, right}
            }
            &WireSource::BinaryOp{op, left, right} => {
                let left = self.get_wire_or_constant_as_wire(left);
                let right = self.get_wire_or_constant_as_wire(right);
                RealWireDataSource::BinaryOp{op, left, right}
            }
            WireSource::Constant(_) => {
                unreachable!("Constant cannot be non-compile-time");
            }
        };
        let name = self.get_unique_name();
        Ok(self.wires.alloc(RealWire{name, typ : ConcreteType::Unknown, original_instruction, source, absolute_latency : CALCULATE_LATENCY_LATER, needed_until : CALCULATE_LATENCY_LATER}))
    }
    fn extend_condition(&mut self, condition : Option<WireID>, additional_condition : WireID, original_instruction : FlatID) -> WireID {
        if let Some(condition) = condition {
            self.wires.alloc(RealWire{
                typ : BOOL_CONCRETE_TYPE,
                name : self.get_unique_name(),
                original_instruction,
                source : RealWireDataSource::BinaryOp{
                    op: BinaryOperator::And,
                    left : condition,
                    right : additional_condition
                },
                absolute_latency : CALCULATE_LATENCY_LATER, needed_until : CALCULATE_LATENCY_LATER})
        } else {
            additional_condition
        }
    }

    pub fn get_initial_val(&self, typ : &ConcreteType) -> Value {
        match typ {
            ConcreteType::Named(_name) => {
                Value::Unset
            }
            ConcreteType::Array(arr) => {
                let (arr_typ, arr_size) = arr.deref();
                let arr_size = arr_size.unwrap_value().unwrap_usize();
                let mut arr = Vec::new();
                if arr_size > 0 {
                    let content_typ = self.get_initial_val(arr_typ);
                    arr.resize(arr_size as usize, content_typ);
                }
                Value::Array(arr.into_boxed_slice())
            }
            ConcreteType::Value(_) | ConcreteType::Unknown | ConcreteType::Error => unreachable!()
        }
    }

    pub fn get_initial_typed_val(&self, typ : ConcreteType) -> TypedValue {
        TypedValue{value : self.get_initial_val(&typ), typ}
    }

    fn instantiate_declaration(&mut self, wire_decl: &Declaration, original_instruction: FlatID) -> ExecutionResult<SubModuleOrWire> {
        let typ = self.concretize_type(&wire_decl.typ_expr)?;
        
        Ok(if wire_decl.identifier_type == IdentifierType::Generative {
            let initial_value = self.get_initial_typed_val(typ);
            SubModuleOrWire::CompileTimeValue(initial_value)
        } else {
            let source = if wire_decl.read_only {
                RealWireDataSource::ReadOnly
            } else {
                let is_state = if wire_decl.identifier_type == IdentifierType::State {
                    Some(Value::Unset)
                } else {
                    None
                };
                RealWireDataSource::Multiplexer{is_state, sources : Vec::new()}
            };

            let absolute_latency : i64 = if let Some(spec) = &wire_decl.latency_specifier {
                self.get_generation_small_int(*spec)?
            } else {
                CALCULATE_LATENCY_LATER
            };
            let wire_id = self.wires.alloc(RealWire{name: wire_decl.name.clone(), typ, original_instruction, source, absolute_latency, needed_until : CALCULATE_LATENCY_LATER});
            if let Some(lat_spec_flat) = wire_decl.latency_specifier {
                let specified_absolute_latency : i64 = self.get_generation_small_int(lat_spec_flat)?;
                self.specified_latencies.push((wire_id, specified_absolute_latency));
            }
            SubModuleOrWire::Wire(wire_id)
        })
    }

    fn instantiate_code_block(&mut self, block_range : FlatIDRange, condition : Option<WireID>) -> ExecutionResult<()> {
        let mut instruction_range = block_range.into_iter();
        while let Some(original_instruction) = instruction_range.next() {
            let instr = &self.md.instructions[original_instruction];
            self.md.get_instruction_span(original_instruction);
            let instance_to_add : SubModuleOrWire = match instr {
                Instruction::SubModule(submodule) => {
                    let sub_module = &self.linker.modules[submodule.module_uuid];
                    let Some(instance) = sub_module.instantiations.instantiate(sub_module, self.linker) else {return Err((submodule.module_name_span, "Error instantiating submodule".to_owned()))}; // Avoid error from submodule

                    let port_map = std::iter::zip(instance.interface_ports.iter(), sub_module.ports.iter()).map(|((_, instance_data), (_, port_data))| {
                        let typ = if let Some(instance_data) = instance_data {
                            instance_data.typ.clone()
                        } else {
                            ConcreteType::Error
                        };
                        self.wires.alloc(RealWire {
                            source: RealWireDataSource::Multiplexer { is_state: None, sources: Vec::new() },
                            original_instruction,
                            typ,
                            name: port_data.name.clone(),
                            absolute_latency: CALCULATE_LATENCY_LATER,
                            needed_until: CALCULATE_LATENCY_LATER
                        })
                    }).collect();
                    let name = if let Some((name, _span)) = &submodule.name {
                        name.clone()
                    } else {
                        self.get_unique_name()
                    };
                    SubModuleOrWire::SubModule(self.submodules.alloc(SubModule { original_flat: original_instruction, instance, port_map, name, module_uuid : submodule.module_uuid}))
                }
                Instruction::Declaration(wire_decl) => {
                    self.instantiate_declaration(wire_decl, original_instruction)?
                }
                Instruction::Wire(w) => {
                    if w.is_compiletime {
                        let value_computed = self.compute_compile_time(w)?;
                        SubModuleOrWire::CompileTimeValue(value_computed)
                    } else {
                        let wire_found = self.wire_to_real_wire(w, original_instruction)?;
                        SubModuleOrWire::Wire(wire_found)
                    }
                }
                Instruction::Write(conn) => {
                    let to_inst = self.instantiate_wire_ref(&conn.to)?;
                    self.process_connection(to_inst, &conn.write_modifiers, conn.from, original_instruction, condition)?;
                    continue;
                }
                Instruction::IfStatement(stm) => {
                    let then_range = UUIDRange(stm.then_start, stm.then_end_else_start);
                    let else_range = UUIDRange(stm.then_end_else_start, stm.else_end);
                    let if_condition_wire = self.md.instructions[stm.condition].unwrap_wire();
                    if if_condition_wire.is_compiletime {
                        let condition_val = self.get_generation_value(stm.condition)?;
                        let run_range = if condition_val.unwrap_bool() {
                            then_range
                        } else {
                            else_range
                        };
                        self.instantiate_code_block(run_range, condition)?;
                    } else {
                        let condition_wire = self.generation_state[stm.condition].unwrap_wire();
                        let then_cond = self.extend_condition(condition, condition_wire, original_instruction);
                        self.instantiate_code_block(then_range, Some(then_cond))?;

                        if !else_range.is_empty() {
                            let else_condition_bool = self.wires.alloc(RealWire{
                                typ : BOOL_CONCRETE_TYPE,
                                name : self.get_unique_name(),
                                original_instruction,
                                source : RealWireDataSource::UnaryOp{
                                    op : UnaryOperator::Not,
                                    right : condition_wire
                                },
                                absolute_latency : CALCULATE_LATENCY_LATER, needed_until : CALCULATE_LATENCY_LATER
                            });
                            let else_cond = self.extend_condition(condition, else_condition_bool, original_instruction);
                            self.instantiate_code_block(else_range, Some(else_cond))?;
                        }
                    }
                    instruction_range.skip_to(stm.else_end);
                    continue;
                }
                Instruction::ForStatement(stm) => {
                    // TODO Non integer for loops?
                    let start_val = self.get_generation_value(stm.start)?.unwrap_integer().clone();
                    let end_val = self.get_generation_value(stm.end)?.unwrap_integer().clone();
                    if start_val > end_val {
                        let start_flat = &self.md.instructions[stm.start].unwrap_wire();
                        let end_flat = &self.md.instructions[stm.end].unwrap_wire();
                        return Err((Span::new_overarching(start_flat.span, end_flat.span), format!("for loop range end is before begin: {start_val}:{end_val}")));
                    }

                    let mut current_val = start_val;

                    while current_val < end_val {
                        let SubModuleOrWire::CompileTimeValue(v) = &mut self.generation_state[stm.loop_var_decl] else {unreachable!()};
                        *v = TypedValue::make_integer(current_val.clone());
                        current_val += 1;
                        self.instantiate_code_block(stm.loop_body, condition)?;
                    }

                    instruction_range.skip_to(stm.loop_body.1);
                    continue;
                }
            };
            self.generation_state[original_instruction] = instance_to_add;
        }
        Ok(())
    }
    
    fn make_interface(&mut self) {
        for (port_id, port) in &self.md.ports {
            let port_decl_id = port.declaration_instruction;
            if let SubModuleOrWire::Wire(wire_id) = &self.generation_state[port_decl_id] {
                let wire = &self.wires[*wire_id];
                self.interface_ports[port_id] = Some(InstantiatedPort{
                    wire : *wire_id,
                    is_input: port.identifier_type.unwrap_is_input(),
                    absolute_latency: CALCULATE_LATENCY_LATER,
                    typ: wire.typ.clone()
                })
            }
        }
    }

    fn extract(self) -> InstantiatedModule {
        InstantiatedModule {
            name : self.name,
            wires : self.wires,
            submodules : self.submodules,
            interface_ports : self.interface_ports,
            generation_state : self.generation_state,
            errors : self.errors.into_storage()
        }
    }



    // ===== Typechecking =====

    fn walk_type_along_path(&self, mut cur_typ : ConcreteType, path : &[RealWirePathElem]) -> ConcreteType {
        for p in path {
            match p {
                RealWirePathElem::MuxArrayWrite { span:_, idx_wire:_ } | RealWirePathElem::ConstArrayWrite { span:_, idx:_ } => {
                    cur_typ = cur_typ.down_array().clone();
                }
            }
        }

        cur_typ
    }

    fn typecheck(&mut self) {
        for this_wire_id in self.wires.id_range() {
            let this_wire = &self.wires[this_wire_id];
            let span = self.md.get_instruction_span(this_wire.original_instruction);
            span.debug();

            match &this_wire.source {
                RealWireDataSource::ReadOnly => {}
                RealWireDataSource::Multiplexer { is_state:_, sources:_ } => {} // Do muxes later. 
                &RealWireDataSource::UnaryOp { op, right } => {
                    let right_typ = self.wires[right].typ.clone();
                    self.wires[this_wire_id].typ.typecheck_concrete_unary_operator(op, &right_typ, span, &self.linker.types, &self.errors);
                }
                &RealWireDataSource::BinaryOp { op, left, right } => {
                    let left_typ = self.wires[left].typ.clone();
                    let right_typ = self.wires[right].typ.clone();
                    self.wires[this_wire_id].typ.typecheck_concrete_binary_operator(op, &left_typ, &right_typ, span, &self.linker.types, &self.errors);
                }
                RealWireDataSource::Select { root, path } => {
                    let found_typ = self.walk_type_along_path(self.wires[*root].typ.clone(), path);
                    self.wires[this_wire_id].typ.check_or_update_type(&found_typ, span, &self.linker.types, &self.errors);
                }
                RealWireDataSource::Constant { value } => {
                    assert!(value.is_of_type(&this_wire.typ), "Assigned type to a constant should already be of the type");
                }
            };
        }

        // Do typechecking of Multiplexers afterwards, because typechecker isn't so smart right now. 
        for this_wire_id in self.wires.id_range() {
            let this_wire = &self.wires[this_wire_id];
            let span = self.md.get_instruction_span(this_wire.original_instruction);
            span.debug();

            if let RealWireDataSource::Multiplexer { is_state, sources } = &this_wire.source {
                if let Some(is_state) = is_state {
                    assert!(is_state.is_of_type(&this_wire.typ));
                }
                for s in sources {
                    let source_typ = &self.wires[s.from.from].typ;
                    let destination_typ = self.walk_type_along_path(self.wires[this_wire_id].typ.clone(), &s.path);
                    destination_typ.check_type(&source_typ, span, &self.linker.types, &self.errors);
                }
            };
        }
    }

    // ===== Latencies =====

    fn make_fanins(&self) -> (ListOfLists<FanInOut>, Vec<SpecifiedLatency>) {
        let mut fanins : ListOfLists<FanInOut> = ListOfLists::new_with_groups_capacity(self.wires.len());
        let mut initial_latencies = Vec::new();
        
        // Wire to wire Fanin
        for (id, wire) in &self.wires {
            fanins.new_group();
            wire.source.iter_sources_with_min_latency(|from, delta_latency| {
                fanins.push_to_last_group(FanInOut{other : from.get_hidden_value(), delta_latency});
            });

            // Submodules Fanin
            // This creates two way connections, from any input i to output o it creates a |o| - |i| length connection, and a -(|o| - |i|) backward connection. This fixes them to be an exact latency apart. 
            // This is O(lots) but doesn't matter, usually very few submodules. Fix this if needed
            for (_id, sub_mod) in &self.submodules {
                for (port_id, self_wire) in &sub_mod.port_map {
                    // Can assign to the wire, too keep in line with ListOfLists build order
                    if *self_wire != id {continue}

                    // Skip non-instantiated ports
                    let Some(port_in_submodule) = &sub_mod.instance.interface_ports[port_id] else {continue};

                    for (other_port_id, other_port_in_submodule) in sub_mod.instance.interface_ports.iter_valids() {
                        if other_port_in_submodule.is_input == !other_port_in_submodule.is_input {
                            // Valid input/output or output/input pair. Apply delta absolute latency

                            let mut delta_latency = other_port_in_submodule.absolute_latency - port_in_submodule.absolute_latency;

                            if port_in_submodule.is_input {
                                delta_latency = -delta_latency;
                            }

                            let other_wire_in_self = sub_mod.port_map[other_port_id];

                            fanins.push_to_last_group(FanInOut{other: other_wire_in_self.get_hidden_value(), delta_latency});
                        }
                    }
                }
            }

            if wire.absolute_latency != CALCULATE_LATENCY_LATER {
                initial_latencies.push(SpecifiedLatency { wire: id.get_hidden_value(), latency: wire.absolute_latency })
            }
        }

        (fanins, initial_latencies)
    }

    // Returns a proper interface if all ports involved did not produce an error. If a port did produce an error then returns None. 
    // Computes all latencies involved
    fn compute_latencies(&mut self) {
        let (fanins, initial_latencies) = self.make_fanins();
        
        // Process fanouts
        let fanouts = convert_fanin_to_fanout(&fanins);

        let mut inputs = Vec::new();
        let mut outputs = Vec::new();

        for (_id, p) in self.interface_ports.iter_valids() {
            if p.is_input {
                inputs.push(p.wire.get_hidden_value());
            } else {
                outputs.push(p.wire.get_hidden_value());
            }
        }
        
        match solve_latencies(&fanins, &fanouts, &inputs, &outputs, initial_latencies) {
            Ok(latencies) => {
                for ((_id, wire), lat) in zip(self.wires.iter_mut(), latencies.iter()) {
                    wire.absolute_latency = *lat;
                    if *lat == CALCULATE_LATENCY_LATER {
                        let source_location = self.md.get_instruction_span(wire.original_instruction);
                        self.errors.error(source_location, format!("Latency Counting couldn't reach this node"));
                    }
                }
                Some(())
            }
            Err(err) => {
                match err {
                    LatencyCountingError::NetPositiveLatencyCycle { conflict_path, net_roundtrip_latency } => {
                        let writes_involved = gather_all_mux_inputs(&self.wires, &conflict_path);
                        assert!(!writes_involved.is_empty());
                        let (first_write, later_writes) = writes_involved.split_first().unwrap();
                        let first_write_desired_latency = first_write.to_latency + net_roundtrip_latency;
                        let mut path_message = make_path_info_string(later_writes, first_write.to_latency, &first_write.to_wire.name);
                        write_path_elem_to_string(&mut path_message, &first_write.to_wire.name, first_write_desired_latency, writes_involved.last().unwrap().to_latency);
                        let unique_write_instructions = filter_unique_write_flats(&writes_involved, &self.md.instructions);
                        let rest_of_message = format!(" part of a net-positive latency cycle of +{net_roundtrip_latency}\n\n{path_message}\nWhich conflicts with the starting latency");
                        
                        let mut did_place_error = false;
                        for wr in &unique_write_instructions {
                            match wr.write_modifiers {
                                WriteModifiers::Connection { num_regs, regs_span } => {
                                    if num_regs >= 1 {
                                        did_place_error = true;
                                        let this_register_plural = if num_regs == 1 {"This register is"} else {"These registers are"};
                                        self.errors.error(regs_span, format!("{this_register_plural}{rest_of_message}"));
                                    }
                                }
                                WriteModifiers::Initial{initial_kw_span : _} => {unreachable!("Initial assignment can only be from compile-time constant. Cannot be part of latency loop. ")}
                            }
                        }
                        // Fallback if no register annotations used
                        if !did_place_error {
                            for wr in unique_write_instructions {
                                self.errors.error(wr.to.span, format!("This write is{rest_of_message}"));
                            }
                        }
                    }
                    LatencyCountingError::IndeterminablePortLatency { bad_ports } => {
                        for port in bad_ports {
                            let port_decl = self.md.instructions[self.wires[WireID::from_hidden_value(port.0)].original_instruction].unwrap_wire_declaration();
                            self.errors.error(port_decl.name_span, format!("Cannot determine port latency. Options are {} and {}\nTry specifying an explicit latency or rework the module to remove this ambiguity", port.1, port.2));
                        }
                    }
                    LatencyCountingError::ConflictingSpecifiedLatencies { conflict_path } => {
                        let start_wire = &self.wires[WireID::from_hidden_value(conflict_path.first().unwrap().wire)];
                        let end_wire = &self.wires[WireID::from_hidden_value(conflict_path.last().unwrap().wire)];
                        let start_decl = self.md.instructions[start_wire.original_instruction].unwrap_wire_declaration();
                        let end_decl = self.md.instructions[end_wire.original_instruction].unwrap_wire_declaration();
                        let end_latency_decl = self.md.instructions[end_decl.latency_specifier.unwrap()].unwrap_wire();
                        

                        let writes_involved = gather_all_mux_inputs(&self.wires, &conflict_path);
                        let path_message = make_path_info_string(&writes_involved, start_wire.absolute_latency, &start_wire.name);
                        //assert!(!writes_involved.is_empty());

                        let end_name = &end_wire.name;
                        let specified_end_latency = end_wire.absolute_latency;
                        self.errors
                            .error(end_latency_decl.span, format!("Conflicting specified latency\n\n{path_message}\nBut this was specified as {end_name}'{specified_end_latency}"))
                            .info_obj_same_file(start_decl);
                    }
                }
                None
            }
        };

        // Compute needed_untils
        for id in self.wires.id_range() {
            let wire = &self.wires[id];
            let mut needed_until = wire.absolute_latency;
            for target_fanout in &fanouts[id.get_hidden_value()] {
                let target_wire = &self.wires[UUID::from_hidden_value(target_fanout.other)];

                needed_until = max(needed_until, target_wire.absolute_latency);
            }
            self.wires[id].needed_until = needed_until;
        }

        // Finally update interface absolute latencies
        for (_id, port) in self.interface_ports.iter_valids_mut() {
            port.absolute_latency = self.wires[port.wire].absolute_latency;
        }
    }
}

pub fn perform_instantiation(md : &Module, linker : &Linker) -> InstantiatedModule {
    let mut context = InstantiationContext{
        name : md.link_info.name.clone(),
        generation_state : md.instructions.iter().map(|(_, _)| SubModuleOrWire::Unnasigned).collect(),
        wires : FlatAlloc::new(),
        submodules : FlatAlloc::new(),
        specified_latencies : Vec::new(),
        interface_ports : md.ports.iter().map(|_| None).collect(),
        errors : ErrorCollector::new_empty(md.link_info.file, &linker.files),
        md,
        linker : linker
    };

    // Don't instantiate modules that already errored. Otherwise instantiator may crash
    if md.link_info.errors.did_error {
        println!("Not Instantiating {} due to flattening errors", md.link_info.name);
        return context.extract();
    }

    println!("Instantiating {}", md.link_info.name);

    let r = context.instantiate_code_block(context.md.instructions.id_range(), None);
    context.make_interface();

    if let Err(e) = r {
        context.errors.error(e.0, e.1);

        return context.extract();
    }

    println!("Concrete Typechecking and Latency Counting {}", md.link_info.name);
    context.typecheck();
    context.compute_latencies();

    context.extract()
}
