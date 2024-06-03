//! Executes the generative code and produces a netlist from it
//! 
//! Stops generating at the first error. 
//! 
//! As for typing, it only instantiates written types and leaves the rest for further typechecking. 

use std::ops::{Deref, Index, IndexMut};

use num::BigInt;

use crate::{
    abstract_type::DomainType, arena_alloc::UUIDRange, concrete_type::{ConcreteType, BOOL_CONCRETE_TYPE, INT_CONCRETE_TYPE}, file_position::Span, flattening::{BinaryOperator, Declaration, FlatID, FlatIDRange, IdentifierType, Instruction, UnaryOperator, WireInstance, WireReference, WireReferencePathElement, WireReferenceRoot, WireSource, WriteModifiers, WrittenType}, linker::NamedConstant, value::{compute_binary_op, compute_unary_op, TypedValue, Value}
};

use super::*;

macro_rules! caught_by_typecheck {
    ($arg:literal) => {panic!("{} should have been caught by typecheck!", $arg)};
    () => {panic!("Should have been caught by typecheck!")};
}

pub type ExecutionResult<T> = Result<T, (Span, String)>;

impl<'fl> GenerationState<'fl> {
    fn span_of(&self, v : FlatID) -> Span {
        self.md.instructions[v].unwrap_wire().span
    }

    fn write_gen_variable(&self, mut target : &mut Value, conn_path : &[WireReferencePathElement], to_write : Value) -> ExecutionResult<()> {
        for elem in conn_path {
            match elem {
                &WireReferencePathElement::ArrayAccess{idx, bracket_span} => {
                    let idx = self.get_generation_integer(idx)?; // Caught by typecheck
                    let Value::Array(a_box) = target else {caught_by_typecheck!("Non-array")};
                    let array_len = a_box.len();
                    let Some(tt) = usize::try_from(idx).ok().and_then(|pos| a_box.get_mut(pos)) else {
                        return Err((bracket_span.inner_span(), format!("Index {idx} is out of bounds for this array of size {}", array_len)))
                    };
                    target = tt
                }
            }
        }
        *target = to_write;
        Ok(())
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
}

impl<'fl> Index<FlatID> for GenerationState<'fl> {
    type Output = SubModuleOrWire;

    fn index(&self, index: FlatID) -> &Self::Output {
        &self.generation_state[index]
    }
}

impl<'fl> IndexMut<FlatID> for GenerationState<'fl> {
    fn index_mut(&mut self, index: FlatID) -> &mut Self::Output {
        &mut self.generation_state[index]
    }
}

fn array_access(tv : &TypedValue, idx : &BigInt, span : BracketSpan) -> ExecutionResult<TypedValue> {
    let typ = tv.typ.down_array().clone();

    let Value::Array(arr) = &tv.value else {caught_by_typecheck!("Value must be an array")};

    if let Some(elem) = usize::try_from(idx).ok().and_then(|idx| arr.get(idx)) {
        Ok(TypedValue{typ, value : elem.clone()})
    } else {
        Err((span.outer_span(), format!("Compile-Time Array index is out of range: idx: {idx}, array size: {}", arr.len())))
    }
}

impl<'fl, 'l> InstantiationContext<'fl, 'l> {
    /// Uses the current context to turn a [WrittenType] into a [ConcreteType]. 
    /// 
    /// Failures are fatal. 
    fn concretize_type(&self, typ : &WrittenType) -> ExecutionResult<ConcreteType> {
        Ok(match typ {
            WrittenType::Error(_) => caught_by_typecheck!("Error Type"),
            WrittenType::Named(_, id) => {
                ConcreteType::Named(*id)
            }
            WrittenType::Array(_, arr_box) => {
                let (arr_content_typ, arr_size_wire, _bracket_span) = arr_box.deref();
                let inner_typ = self.concretize_type(arr_content_typ)?;
                let arr_size = self.generation_state.get_generation_integer(*arr_size_wire)?;
                ConcreteType::Array(Box::new((inner_typ, ConcreteType::Value(Value::Integer(arr_size.clone())))))
            }
        })
    }

    fn instantiate_port_wire_ref_root(&self, port : PortID, submodule_instr : FlatID) -> RealWireRefRoot {
        let sm = &self.submodules[self.generation_state[submodule_instr].unwrap_submodule_instance()];
        RealWireRefRoot::Wire{wire_id : sm.port_map[port], preamble : Vec::new()}
    }

    // Points to the wire in the hardware that corresponds to the root of this. 
    fn determine_wire_ref_root(&self, wire_ref_root : &WireReferenceRoot) -> RealWireRefRoot {
        match wire_ref_root {
            &WireReferenceRoot::LocalDecl(decl_id, _) => {
                match &self.generation_state[decl_id] {
                    SubModuleOrWire::Wire(w) => RealWireRefRoot::Wire{wire_id : *w, preamble : Vec::new()},
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
                return self.instantiate_port_wire_ref_root(port.port, port.submodule_flat);
            }
        }
    }

    /// [Self::determine_wire_ref_root] may have included a preamble path already, this must be built upon by this function
    fn instantiate_wire_ref_path(&mut self, mut preamble : Vec<RealWirePathElem>, path : &[WireReferencePathElement], domain : DomainID) -> Vec<RealWirePathElem> {
        for v in path {
            match v {
                &WireReferencePathElement::ArrayAccess{idx, bracket_span} => {
                    let idx_wire = self.get_wire_or_constant_as_wire(idx, domain);
                    assert_eq!(self.wires[idx_wire].typ, INT_CONCRETE_TYPE, "Caught by typecheck");
                    preamble.push(RealWirePathElem::ArrayAccess{ span:bracket_span, idx_wire});
                }
            }
        }

        preamble
    }

    fn instantiate_write_to_wire(&mut self, write_to_wire : WireID, to_path : Vec<RealWirePathElem>, from : WireID, num_regs : i64, original_instruction : FlatID, condition : Option<WireID>) {
        let from = ConnectFrom {
            num_regs,
            from,
            condition,
            original_connection : original_instruction
        };

        let RealWireDataSource::Multiplexer{is_state : _, sources} = &mut self.wires[write_to_wire].source else {caught_by_typecheck!("Should only be a writeable wire here")};

        sources.push(MultiplexerSource{from, to_path});
    }

    fn instantiate_connection(&mut self, target_wire_ref : &WireReference, write_modifiers : &WriteModifiers, conn_from : FlatID, original_connection : FlatID, condition : Option<WireID>) -> ExecutionResult<()> {
        match write_modifiers {
            WriteModifiers::Connection{num_regs, regs_span : _} => {
                match self.determine_wire_ref_root(&target_wire_ref.root) {
                    RealWireRefRoot::Wire{wire_id: target_wire, preamble} => {
                        let domain = self.wires[target_wire].domain;
                        let from = self.get_wire_or_constant_as_wire(conn_from, domain);
                        let instantiated_path = self.instantiate_wire_ref_path(preamble, &target_wire_ref.path, domain);
                        self.instantiate_write_to_wire(target_wire, instantiated_path, from, *num_regs, original_connection, condition);
                    }
                    RealWireRefRoot::Generative(target_decl) => {
                        let found_v = self.generation_state[conn_from].unwrap_generation_value().clone();

                        let SubModuleOrWire::CompileTimeValue(v_writable) = &mut self.generation_state[target_decl] else {unreachable!()};
                        let mut new_val = std::mem::replace(&mut v_writable.value, Value::Error);
                        self.generation_state.write_gen_variable(&mut new_val, &target_wire_ref.path, found_v.value)?;

                        let SubModuleOrWire::CompileTimeValue(v_writable) = &mut self.generation_state[target_decl] else {unreachable!()};
                        v_writable.value = new_val;
                    }
                    RealWireRefRoot::Constant(_cst) => {
                        caught_by_typecheck!("Cannot assign to constants");
                    },
                }
            }
            WriteModifiers::Initial{initial_kw_span : _} => {
                let found_v = self.generation_state.get_generation_value(conn_from)?.clone();

                let root_wire = self.generation_state[target_wire_ref.root.unwrap_local_decl()].unwrap_wire();
                let RealWireDataSource::Multiplexer{is_state : Some(initial_value), sources : _} = &mut self.wires[root_wire].source else {caught_by_typecheck!()};
                self.generation_state.write_gen_variable(initial_value, &target_wire_ref.path, found_v.value)?;
            }
        }
        Ok(())
    }
    fn compute_compile_time_wireref(&self, wire_ref : &WireReference) -> ExecutionResult<TypedValue> {
        let mut work_on_value : TypedValue = match &wire_ref.root {
            &WireReferenceRoot::LocalDecl(decl_id, _span) => {
                self.generation_state.get_generation_value(decl_id)?.clone()
            }
            WireReferenceRoot::NamedConstant(const_id, _span) => {
                self.linker.constants[*const_id].get_value().clone()
            }
            &WireReferenceRoot::SubModulePort(_) => {
                todo!("Don't yet support compile time functions")
            }
        };

        for path_elem in &wire_ref.path {
            work_on_value = match path_elem {
                &WireReferencePathElement::ArrayAccess{idx, bracket_span} => {
                    let idx = self.generation_state.get_generation_integer(idx)?;

                    array_access(&work_on_value, idx, bracket_span)?
                }
            }
        }

        Ok(work_on_value)
    }
    fn compute_compile_time(&mut self, wire_inst : &WireInstance) -> ExecutionResult<TypedValue> {
        Ok(match &wire_inst.source {
            WireSource::WireRef(wire_ref) => {
                self.compute_compile_time_wireref(wire_ref)?
            }
            &WireSource::UnaryOp{op, right} => {
                let right_val = self.generation_state.get_generation_value(right)?;
                compute_unary_op(op, right_val)
            }
            &WireSource::BinaryOp{op, left, right} => {
                let left_val = self.generation_state.get_generation_value(left)?;
                let right_val = self.generation_state.get_generation_value(right)?;
                compute_binary_op(left_val, op, right_val)
            }
            WireSource::Constant(value) => TypedValue::from_value(value.clone())
        })
    }
    fn get_unique_name(&self) -> String {
        format!("_{}", self.wires.get_next_alloc_id().get_hidden_value())
    }
    fn alloc_wire_for_const(&mut self, value : TypedValue, original_instruction : FlatID, domain : DomainID) -> WireID {
        let name = self.get_unique_name();
        self.wires.alloc(RealWire{source : RealWireDataSource::Constant{value : value.value}, original_instruction, domain, typ : value.typ, name, absolute_latency : CALCULATE_LATENCY_LATER, needed_until : CALCULATE_LATENCY_LATER})
    }
    fn get_wire_or_constant_as_wire(&mut self, original_instruction : FlatID, domain : DomainID) -> WireID {
        match &self.generation_state[original_instruction] {
            SubModuleOrWire::SubModule(_) => unreachable!(),
            SubModuleOrWire::Unnasigned => unreachable!(),
            SubModuleOrWire::Wire(w) => *w,
            SubModuleOrWire::CompileTimeValue(v) => {
                let value = v.clone();

                self.alloc_wire_for_const(value, original_instruction, domain)
            }
        }
    }
    fn get_wire_ref_root_as_wire(&mut self, wire_ref_root : &WireReferenceRoot, original_instruction : FlatID, domain : DomainID) -> (WireID, Vec<RealWirePathElem>) {
        let root = self.determine_wire_ref_root(wire_ref_root);
        match root {
            RealWireRefRoot::Wire{wire_id, preamble} => (wire_id, preamble),
            RealWireRefRoot::Generative(decl_id) => {
                let value = self.generation_state[decl_id].unwrap_generation_value().clone();
                (self.alloc_wire_for_const(value, decl_id, domain), Vec::new())
            }
            RealWireRefRoot::Constant(value) => (self.alloc_wire_for_const(value, original_instruction, domain), Vec::new())
        }
    }
    fn wire_to_real_wire(&mut self, w: &WireInstance, original_instruction : FlatID, domain : DomainID) -> ExecutionResult<WireID> {
        let source = match &w.source {
            WireSource::WireRef(wire_ref) => {
                let (root_wire, path_preamble) = self.get_wire_ref_root_as_wire(&wire_ref.root, original_instruction, domain);
                let path = self.instantiate_wire_ref_path(path_preamble, &wire_ref.path, domain);

                if path.is_empty() { // Little optimization reduces instructions
                    return Ok(root_wire)
                }

                RealWireDataSource::Select { root: root_wire, path }
            }
            &WireSource::UnaryOp{op, right} => {
                let right = self.get_wire_or_constant_as_wire(right, domain);
                RealWireDataSource::UnaryOp{op, right}
            }
            &WireSource::BinaryOp{op, left, right} => {
                let left = self.get_wire_or_constant_as_wire(left, domain);
                let right = self.get_wire_or_constant_as_wire(right, domain);
                RealWireDataSource::BinaryOp{op, left, right}
            }
            WireSource::Constant(_) => {
                unreachable!("Constant cannot be non-compile-time");
            }
        };
        let name = self.get_unique_name();
        Ok(self.wires.alloc(RealWire{name, typ : ConcreteType::Unknown, original_instruction, domain, source, absolute_latency : CALCULATE_LATENCY_LATER, needed_until : CALCULATE_LATENCY_LATER}))
    }
    fn extend_condition(&mut self, condition : Option<WireID>, additional_condition : WireID, original_instruction : FlatID, domain : DomainID) -> WireID {
        if let Some(condition) = condition {
            self.wires.alloc(RealWire{
                typ : BOOL_CONCRETE_TYPE,
                name : self.get_unique_name(),
                original_instruction,
                domain,
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
                self.generation_state.get_generation_small_int(*spec)?
            } else {
                CALCULATE_LATENCY_LATER
            };
            let wire_id = self.wires.alloc(RealWire{
                name: wire_decl.name.clone(),
                typ,
                original_instruction,
                domain : wire_decl.typ.domain.unwrap_physical(),
                source,
                absolute_latency,
                needed_until : CALCULATE_LATENCY_LATER
            });
            if let Some(lat_spec_flat) = wire_decl.latency_specifier {
                let specified_absolute_latency : i64 = self.generation_state.get_generation_small_int(lat_spec_flat)?;
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
                    
                    let new_submodule_id = self.submodules.get_next_alloc_id();
                    let name = if let Some((name, _span)) = &submodule.name {
                        name.clone()
                    } else {
                        self.get_unique_name()
                    };
                    let port_map = sub_module.ports.iter().map(|(port_id, port_data)| {
                        let source = if port_data.identifier_type.unwrap_is_input() {
                            RealWireDataSource::Multiplexer { is_state: None, sources: Vec::new() }
                        } else {
                            RealWireDataSource::OutPort { sub_module_id: new_submodule_id, port_id }
                        };
                        let domain = submodule.local_interface_domains[port_data.interface];
                        self.wires.alloc(RealWire {
                            source,
                            original_instruction,
                            domain,
                            typ : ConcreteType::Unknown,
                            name: format!("{}_{}", name, port_data.name),
                            absolute_latency: CALCULATE_LATENCY_LATER,
                            needed_until: CALCULATE_LATENCY_LATER
                        })
                    }).collect();
                    SubModuleOrWire::SubModule(self.submodules.alloc(SubModule { original_instruction, instance : None, port_map, name, module_uuid : submodule.module_uuid}))
                }
                Instruction::Declaration(wire_decl) => {
                    self.instantiate_declaration(wire_decl, original_instruction)?
                }
                Instruction::Wire(w) => {
                    match w.typ.domain {
                        DomainType::Generative => {
                            let value_computed = self.compute_compile_time(w)?;
                            SubModuleOrWire::CompileTimeValue(value_computed)
                        }
                        DomainType::Physical(domain) => {
                            let wire_found = self.wire_to_real_wire(w, original_instruction, domain)?;
                            SubModuleOrWire::Wire(wire_found)
                        }
                    }
                }
                Instruction::Write(conn) => {
                    self.instantiate_connection(&conn.to, &conn.write_modifiers, conn.from, original_instruction, condition)?;
                    continue;
                }
                Instruction::FuncCall(fc) => {
                    let submod_id = self.generation_state[fc.submodule_instruction].unwrap_submodule_instance();
                    let original_submod_instr = &self.md.instructions[fc.submodule_instruction].unwrap_submodule();
                    let domain = original_submod_instr.local_interface_domains[fc.submodule_interface];
                    for (port, arg) in std::iter::zip(fc.func_call_inputs.iter(), fc.arguments.iter()) {
                        let from = self.get_wire_or_constant_as_wire(*arg, domain);
                        let submod_instance = &self.submodules[submod_id]; // Separately grab the same submodule every time because we take a &mut in for get_wire_or_constant_as_wire
                        let port_wire = submod_instance.port_map[port];
                        self.instantiate_write_to_wire(port_wire, Vec::new(), from, 0, original_instruction, condition);
                    }

                    continue;
                }
                Instruction::IfStatement(stm) => {
                    let then_range = UUIDRange(stm.then_start, stm.then_end_else_start);
                    let else_range = UUIDRange(stm.then_end_else_start, stm.else_end);
                    let if_condition_wire = self.md.instructions[stm.condition].unwrap_wire();
                    match if_condition_wire.typ.domain {
                        DomainType::Generative => {
                            let condition_val = self.generation_state.get_generation_value(stm.condition)?;
                            let run_range = if condition_val.unwrap_bool() {
                                then_range
                            } else {
                                else_range
                            };
                            self.instantiate_code_block(run_range, condition)?;
                        }
                        DomainType::Physical(domain) => {
                            let condition_wire = self.generation_state[stm.condition].unwrap_wire();
                            let then_cond = self.extend_condition(condition, condition_wire, original_instruction, domain);
                            self.instantiate_code_block(then_range, Some(then_cond))?;
                            
                            if !else_range.is_empty() {
                                let else_condition_bool = self.wires.alloc(RealWire{
                                    typ : BOOL_CONCRETE_TYPE,
                                    name : self.get_unique_name(),
                                    original_instruction,
                                    domain,
                                    source : RealWireDataSource::UnaryOp{
                                        op : UnaryOperator::Not,
                                        right : condition_wire
                                    },
                                    absolute_latency : CALCULATE_LATENCY_LATER, needed_until : CALCULATE_LATENCY_LATER
                                });
                                let else_cond = self.extend_condition(condition, else_condition_bool, original_instruction, domain);
                                self.instantiate_code_block(else_range, Some(else_cond))?;
                            }
                        }
                    }
                    instruction_range.skip_to(stm.else_end);
                    continue;
                }
                Instruction::ForStatement(stm) => {
                    // TODO Non integer for loops?
                    let start_val = self.generation_state.get_generation_value(stm.start)?.unwrap_integer().clone();
                    let end_val = self.generation_state.get_generation_value(stm.end)?.unwrap_integer().clone();
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

    pub fn execute_module(&mut self) -> ExecutionResult<()> {
        let result = self.instantiate_code_block(self.md.instructions.id_range(), None);
        self.make_interface();
        result
    }
}
