

use std::{cmp::max, iter::zip, ops::Deref};

use num::BigInt;

use crate::{
    arena_alloc::{FlatAlloc, UUIDRange, UUID},
    compiler_top::instantiate,
    errors::{ErrorCollector, ErrorStore},
    file_position::Span,
    flattening::{PortIDMarker, BinaryOperator, FlatID, FlatIDMarker, FlatIDRange, IdentifierType, Instruction, Module, UnaryOperator, WireInstance, WireReference, WireReferencePathElement, WireReferenceRoot, WireSource, Write, WriteModifiers},
    instantiation::latency_algorithm::{convert_fanin_to_fanout, solve_latencies, FanInOut, LatencyCountingError},
    linker::{Linker, NamedConstant},
    typing::{typecheck_concrete_binary_operator, typecheck_concrete_unary_operator, ConcreteType, WrittenType, BOOL_CONCRETE_TYPE, INT_CONCRETE_TYPE},
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

        //let decl_id = to_wire.original_wire;
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



impl RealWireRefRoot {
    fn get_concrete_type<'out, 'slf : 'out, 'ctx : 'out>(&'slf self, ctx : &'ctx InstantiationContext) -> &'out ConcreteType {
        match self {
            RealWireRefRoot::Wire(w) => {
                &ctx.wires[*w].typ
            }
            RealWireRefRoot::Generative(f) => {
                &ctx.generation_state[*f].unwrap_generation_value().typ
            }
            RealWireRefRoot::Constant(cv) => {
                &cv.typ
            }
        }
    }

    fn get_concrete_type_with_path<'out, 'slf : 'out, 'ctx : 'out>(&'slf self, ctx : &'ctx InstantiationContext, path : &[RealWirePathElem]) -> &'out ConcreteType {
        let mut cur_typ = self.get_concrete_type(ctx);

        for p in path {
            match p {
                RealWirePathElem::MuxArrayWrite { span:_, idx_wire:_ } | RealWirePathElem::ConstArrayWrite { span:_, idx:_ } => {
                    cur_typ = cur_typ.down_array();
                }
            }
        }

        cur_typ
    }
}



fn write_gen_variable<'t, 'p>(mut target : &'t mut Value, conn_path : &'p [RealWirePathElem], to_write : Value) {
    for elem in conn_path {
        match elem {
            RealWirePathElem::ConstArrayWrite{span:_, idx: Some(idx)} => {
                let Value::Array(a_box) = target else {unreachable!()};
                target = &mut a_box[*idx];
            }
            RealWirePathElem::ConstArrayWrite{span:_, idx: None} | RealWirePathElem::MuxArrayWrite {span:_,  idx_wire:_ } => {
                // No errors are thrown here, because the user should have been informed by generative-checking code beforehand
                // Could not compute this index
                return;
            }
        }
    }
    *target = to_write;
}

struct InstantiationContext<'fl, 'l> {
    generation_state : FlatAlloc<SubModuleOrWire, FlatIDMarker>,
    wires : FlatAlloc<RealWire, WireIDMarker>,
    submodules : FlatAlloc<SubModule, SubModuleIDMarker>,
    specified_latencies : Vec<(WireID, i64)>,
    errors : ErrorCollector<'l>,

    md : &'fl Module,
    linker : &'l Linker,
}

impl<'fl, 'l> InstantiationContext<'fl, 'l> {
    fn get_generation_value(&self, v : FlatID) -> Option<&TypedValue> {
        if let SubModuleOrWire::CompileTimeValue(vv) = &self.generation_state[v] {
            if let Value::Unset | Value::Error = &vv.value {
                self.errors.error(self.md.instructions[v].unwrap_wire().span, format!("This variable is set but it's {vv:?}!"));
                None
            } else {
                Some(vv)
            }
        } else {
            self.errors.error(self.md.instructions[v].unwrap_wire().span, "This variable is not set at this point!");
            None
        }
    }
    fn extract_integer_from_value<'v, IntT : TryFrom<&'v BigInt>>(&self, val : &'v Value, span : Span) -> Option<IntT> {
        let val = val.unwrap_integer(); // Typecheck should cover this
        match IntT::try_from(val) {
            Ok(val) => Some(val),
            Err(_) => {
                self.errors.error(span, format!("Generative integer does not fit in {}: {val}", std::any::type_name::<IntT>()));
                None
            }
        }
    }
    fn extract_integer_from_generative<'v, IntT : TryFrom<&'v BigInt>>(&'v self, idx : FlatID) -> Option<IntT> {
        let val = self.get_generation_value(idx)?;
        let span = self.md.instructions[idx].unwrap_wire().span;
        self.extract_integer_from_value(&val.value, span)
    }

    fn concretize_type(&self, typ : &WrittenType) -> ConcreteType {
        match typ {
            WrittenType::Error(_) => unreachable!("Bad types should be caught in flattening: {}", typ.to_string(&self.linker.types)),
            WrittenType::Named(_, id) => {
                ConcreteType::Named(*id)
            }
            WrittenType::Array(_, arr_box) => {
                let (arr_content_typ, arr_size_wire, _bracket_span) = arr_box.deref();
                let inner_typ = self.concretize_type(arr_content_typ);
                let arr_size = self.extract_integer_from_generative(*arr_size_wire);
                ConcreteType::Array(Box::new((inner_typ, arr_size)))
            }
        }
    }

    fn realize_wire_ref(&self, wire_ref : &WireReference) -> (RealWireRefRoot, Vec<RealWirePathElem>) {
        // Later on, potentially allow module arrays
        let mut real_path = Vec::new();

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
        
                            real_path.push(RealWirePathElem::MuxArrayWrite{ span:bracket_span, idx_wire});
                        }
                        SubModuleOrWire::CompileTimeValue(cv) => {
                            let span = self.md.instructions[idx].unwrap_wire().span;
                            real_path.push(RealWirePathElem::ConstArrayWrite{
                                idx : self.extract_integer_from_value(&cv.value, span),
                                span : bracket_span
                            });
                        }
                    }
                }
            }
        }

        (root, real_path)
    }

    fn process_connection_to_wire(&mut self, to_wire_ref : &WireReference, from : ConnectFrom, wire_id : WireID) -> Option<()> {
        let (root, path) = self.realize_wire_ref(to_wire_ref);

        let write_to_typ = root.get_concrete_type_with_path(self, &path);
        
        let found_typ = &self.wires[from.from].typ;
        if write_to_typ != found_typ {
            let from_flattened_wire = self.md.instructions[self.wires[from.from].original_wire].unwrap_wire();
            let to_flattened_decl = self.md.instructions[self.wires[wire_id].original_wire].unwrap_wire_declaration();

            let found_typ_name = found_typ.to_string(&self.linker.types);
            let write_to_typ_name = write_to_typ.to_string(&self.linker.types);

            self.errors
                .error(from_flattened_wire.span, format!("Instantiation TypeError: Can't assign {found_typ_name} to {write_to_typ_name}"))
                .info_obj_same_file(to_flattened_decl);
        }

        let RealWireDataSource::Multiplexer{is_state : _, sources} = &mut self.wires[wire_id].source else {unreachable!("Should only be a writeable wire here")};

        sources.push(MultiplexerSource{from, path});

        Some(())
    }
    fn process_connection(&mut self, conn : &Write, original_connection : FlatID, condition : Option<WireID>) -> Option<()> {
        let (root, real_path) = self.realize_wire_ref(&conn.to);
        
        match conn.write_modifiers {
            WriteModifiers::Connection{num_regs, regs_span : _} => {
                match &root {
                    RealWireRefRoot::Wire(write_to_wire) => {
                        let from = self.get_wire_or_constant_as_wire(conn.from);
                        let conn_from = ConnectFrom {
                            num_regs,
                            from,
                            condition,
                            original_connection
                        };

                        self.process_connection_to_wire(&conn.to, conn_from, *write_to_wire)?;
                    }
                    RealWireRefRoot::Generative(decl_id) => {
                        let found_v = self.generation_state[conn.from].unwrap_generation_value().clone();
                        // Hack to get around the borrow rules here
                        let SubModuleOrWire::CompileTimeValue(v_writable) = &mut self.generation_state[*decl_id] else {unreachable!()};
                        write_gen_variable(&mut v_writable.value, &real_path, found_v.value);
                    }
                    RealWireRefRoot::Constant(_) => {
                        unreachable!("Cannot assign to constants. This should have been caught in Abstract Typecheck!")
                    },
                }
            }
            WriteModifiers::Initial{initial_kw_span : _} => {
                let found_v = self.get_generation_value(conn.from)?.clone();
                // Hack to get around the borrow rules here
                let SubModuleOrWire::Wire(w) = &mut self.generation_state[conn.to.root.unwrap_decl()] else {unreachable!()};
                let RealWireDataSource::Multiplexer{is_state : Some(initial_value), sources : _} = &mut self.wires[*w].source else {unreachable!()};
                write_gen_variable(initial_value, &real_path, found_v.value);
            }
        }
        Some(())
    }
    fn compute_compile_time_wireref(&self, wire_ref : &WireReference) -> Option<TypedValue> {
        let (root, path) = self.realize_wire_ref(wire_ref);

        let mut work_on_value = match &root {
            &RealWireRefRoot::Generative(decl_id) => {
                self.get_generation_value(decl_id)?.clone()
            }
            RealWireRefRoot::Constant(cst) => {
                cst.clone()
            }
            RealWireRefRoot::Wire(_) => {
                unreachable!("Trying to write to a non-generative wire at compiletime. Should have been caught before!")
            }
        };

        for path_elem in &path {
            work_on_value = match path_elem {
                &RealWirePathElem::ConstArrayWrite { span, idx : Some(computed_index)} => {
                    work_on_value.array_access(computed_index, span, &self.errors)?
                }
                RealWirePathElem::MuxArrayWrite { span:_, idx_wire:_ } | RealWirePathElem::ConstArrayWrite { span:_, idx : None} => {
                    return None; // Error already reported. 
                }
            }
        }

        Some(work_on_value)
    }
    fn compute_compile_time(&self, wire_inst : &WireInstance) -> Option<TypedValue> {
        Some(match &wire_inst.source {
            WireSource::WireRef(wire_ref) => {
                self.compute_compile_time_wireref(wire_ref)?
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
    fn alloc_wire_for_const(&mut self, value : TypedValue, original_wire : FlatID) -> WireID {
        let name = self.get_unique_name();
        self.wires.alloc(RealWire{source : RealWireDataSource::Constant{value : value.value}, original_wire, typ : value.typ, name, absolute_latency : CALCULATE_LATENCY_LATER, needed_until : CALCULATE_LATENCY_LATER})
    }
    fn get_wire_or_constant_as_wire(&mut self, original_wire : FlatID) -> WireID {
        match &self.generation_state[original_wire] {
            SubModuleOrWire::SubModule(_) => unreachable!(),
            SubModuleOrWire::Unnasigned => unreachable!(),
            SubModuleOrWire::Wire(w) => *w,
            SubModuleOrWire::CompileTimeValue(v) => {
                let value = v.clone();

                self.alloc_wire_for_const(value, original_wire)
            }
        }
    }
    fn get_wire_ref_root_as_wire(&mut self, root : RealWireRefRoot, original_wire : FlatID) -> WireID {
        match root {
            RealWireRefRoot::Wire(w) => w,
            RealWireRefRoot::Generative(decl_id) => {
                let value = self.generation_state[decl_id].unwrap_generation_value().clone();
                self.alloc_wire_for_const(value, decl_id)
            }
            RealWireRefRoot::Constant(value) => self.alloc_wire_for_const(value, original_wire)
        }
    }
    fn wire_to_real_wire(&mut self, w: &WireInstance, typ : ConcreteType, original_wire : FlatID) -> Option<WireID> {
        let source = match &w.source {
            WireSource::WireRef(wire_ref) => {
                let (root, path) = self.realize_wire_ref(wire_ref);
                let root_wire = self.get_wire_ref_root_as_wire(root, original_wire);

                if path.is_empty() { // Little optimization reduces instructions
                    return Some(root_wire)
                }

                RealWireDataSource::Select { root: root_wire, path }
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
        Some(self.wires.alloc(RealWire{name, typ, original_wire, source, absolute_latency : CALCULATE_LATENCY_LATER, needed_until : CALCULATE_LATENCY_LATER}))
    }
    fn extend_condition(&mut self, condition : Option<WireID>, additional_condition : WireID, original_wire : FlatID) -> WireID {
        if let Some(condition) = condition {
            self.wires.alloc(RealWire{
                typ : BOOL_CONCRETE_TYPE,
                name : self.get_unique_name(),
                original_wire,
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

    fn instantiate_flattened_module(&mut self, flat_range : FlatIDRange, condition : Option<WireID>) -> Option<()> {
        let mut instruction_range = flat_range.into_iter();
        while let Some(original_wire) = instruction_range.next() {
            let instance_to_add : SubModuleOrWire = match &self.md.instructions[original_wire] {
                Instruction::SubModule(submodule) => {
                    let Some(instance) = instantiate(&self.linker, submodule.module_uuid) else {return None}; // Avoid error from submodule

                    let port_map = instance.interface_ports.iter().map(|(_id, port_data)| {
                        let wire_in_instance = &instance.wires[port_data.wire];
                        self.wires.alloc(RealWire {
                            source: RealWireDataSource::Multiplexer { is_state: None, sources: Vec::new() },
                            original_wire,
                            typ: wire_in_instance.typ.clone(),
                            name: wire_in_instance.name.clone(),
                            absolute_latency: CALCULATE_LATENCY_LATER,
                            needed_until: CALCULATE_LATENCY_LATER
                        })
                    }).collect();
                    let name = if let Some((name, _span)) = &submodule.name {
                        name.clone()
                    } else {
                        self.get_unique_name()
                    };
                    SubModuleOrWire::SubModule(self.submodules.alloc(SubModule { original_flat: original_wire, instance, port_map, name, module_uuid : submodule.module_uuid}))
                }
                Instruction::Declaration(wire_decl) => {
                    let typ = self.concretize_type(&wire_decl.typ_expr);
                    if wire_decl.identifier_type.is_generative() {
                        let initial_value = typ.into_initial_typed_val(self.linker);
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
                        let absolute_latency = if let Some(spec) = &wire_decl.latency_specifier {
                            self.extract_integer_from_generative(*spec)?
                        } else {
                            CALCULATE_LATENCY_LATER
                        };
                        let wire_id = self.wires.alloc(RealWire{name: wire_decl.name.clone(), typ, original_wire, source, absolute_latency, needed_until : CALCULATE_LATENCY_LATER});
                        if let Some(lat_spec_flat) = wire_decl.latency_specifier {
                            let specified_absolute_latency : i64 = self.extract_integer_from_generative(lat_spec_flat)?;
                            self.specified_latencies.push((wire_id, specified_absolute_latency));
                        }
                        SubModuleOrWire::Wire(wire_id)
                    }
                }
                Instruction::Wire(w) => {
                    let typ = match &w.source {
                        WireSource::WireRef(wire_ref) => {
                            let (root, path) = self.realize_wire_ref(wire_ref);

                            root.get_concrete_type_with_path(self, &path).clone()
                        }
                        &WireSource::UnaryOp { op, right } => {
                            let right_typ = self.get_current_concrete_type(right);
                            typecheck_concrete_unary_operator(op, &right_typ, w.span, &self.linker.types, &self.errors)
                        }
                        &WireSource::BinaryOp { op, left, right } => {
                            let left_typ = self.get_current_concrete_type(left);
                            let right_typ = self.get_current_concrete_type(right);
                            typecheck_concrete_binary_operator(op, &left_typ, &right_typ, w.span, &self.linker.types, &self.errors)
                        }
                        WireSource::Constant(v) => v.get_concrete_type_of_constant(),
                    };
                    if w.is_compiletime {
                        let value_computed = self.compute_compile_time(w)?;
                        assert_eq!(value_computed.typ, typ);
                        SubModuleOrWire::CompileTimeValue(value_computed)
                    } else {
                        let wire_found = self.wire_to_real_wire(w, typ, original_wire)?;
                        SubModuleOrWire::Wire(wire_found)
                    }
                }
                Instruction::Write(conn) => {
                    self.process_connection(conn, original_wire, condition);
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
                        self.instantiate_flattened_module(run_range, condition);
                    } else {
                        let condition_wire = self.generation_state[stm.condition].unwrap_wire();
                        let then_cond = self.extend_condition(condition, condition_wire, original_wire);
                        self.instantiate_flattened_module(then_range, Some(then_cond));

                        if !else_range.is_empty() {
                            let else_condition_bool = self.wires.alloc(RealWire{
                                typ : BOOL_CONCRETE_TYPE,
                                name : self.get_unique_name(),
                                original_wire,
                                source : RealWireDataSource::UnaryOp{
                                    op : UnaryOperator::Not,
                                    right : condition_wire
                                },
                                absolute_latency : CALCULATE_LATENCY_LATER, needed_until : CALCULATE_LATENCY_LATER
                            });
                            let else_cond = self.extend_condition(condition, else_condition_bool, original_wire);
                            self.instantiate_flattened_module(else_range, Some(else_cond));
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
                        self.errors.error(Span::new_overarching(start_flat.span, end_flat.span), format!("for loop range end is before begin: {start_val}:{end_val}"));
                        return None;
                    }

                    let mut current_val = start_val;

                    while current_val < end_val {
                        let SubModuleOrWire::CompileTimeValue(v) = &mut self.generation_state[stm.loop_var_decl] else {unreachable!()};
                        *v = TypedValue::make_integer(current_val.clone());
                        current_val += 1;
                        self.instantiate_flattened_module(stm.loop_body, condition);
                    }

                    instruction_range.skip_to(stm.loop_body.1);
                    continue;
                }
            };
            self.generation_state[original_wire] = instance_to_add;
        }
        Some(())
    }

    fn get_current_concrete_type(&self, w: FlatID) -> &ConcreteType {
        match &self.generation_state[w] {
            SubModuleOrWire::Wire(w_idx) => &self.wires[*w_idx].typ,
            SubModuleOrWire::CompileTimeValue(cv) => &cv.typ,
            SubModuleOrWire::SubModule(_) => unreachable!("Cannot get concrete type of submodule"),
            SubModuleOrWire::Unnasigned => unreachable!("Concrete type of Unassigned, should have been caught in abstract typecheck?"),
        }
    }
    
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

                    let port_in_submodule = &sub_mod.instance.interface_ports[port_id];

                    for (other_port_id, other_port_in_submodule) in &sub_mod.instance.interface_ports {
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
    
    fn make_instantiated_ports(&self) -> FlatAlloc<InstantiatedPort, PortIDMarker> {
        let result = self.md.module_ports.ports.iter().map(|(port_id, port)| {
            let port_decl_id = self.md.module_ports.ports[port_id].declaration_instruction;
            let wire_id = self.generation_state[port_decl_id].unwrap_wire();
            let wire = &self.wires[wire_id];
            
            InstantiatedPort{ wire: wire_id, is_input: port.identifier_type.unwrap_is_input(), absolute_latency: CALCULATE_LATENCY_LATER, typ : wire.typ.clone()}
        }).collect();
        result
    }

    // Returns a proper interface if all ports involved did not produce an error. If a port did produce an error then returns None. 
    // Computes all latencies involved
    fn compute_latencies(&mut self, ) -> Option<FlatAlloc<InstantiatedPort, PortIDMarker>> {
        let interface = self.make_instantiated_ports();

        let (fanins, initial_latencies) = self.make_fanins();
        
        // Process fanouts
        let fanouts = convert_fanin_to_fanout(&fanins);

        let mut inputs = Vec::new();
        let mut outputs = Vec::new();

        for (_id, p) in &interface {
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
                        if let Some(source_location) = self.md.instructions[wire.original_wire].get_location_of_module_part() {
                            self.errors.error(source_location, format!("Latency Counting couldn't reach this node"));
                        }
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
                            let port_decl = self.md.instructions[self.wires[WireID::from_hidden_value(port.0)].original_wire].unwrap_wire_declaration();
                            self.errors.error(port_decl.name_span, format!("Cannot determine port latency. Options are {} and {}\nTry specifying an explicit latency or rework the module to remove this ambiguity", port.1, port.2));
                        }
                    }
                    LatencyCountingError::ConflictingSpecifiedLatencies { conflict_path } => {
                        let start_wire = &self.wires[WireID::from_hidden_value(conflict_path.first().unwrap().wire)];
                        let end_wire = &self.wires[WireID::from_hidden_value(conflict_path.last().unwrap().wire)];
                        let start_decl = self.md.instructions[start_wire.original_wire].unwrap_wire_declaration();
                        let end_decl = self.md.instructions[end_wire.original_wire].unwrap_wire_declaration();
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
        }?;

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
        let mut interface = interface;
        for (_id, port) in &mut interface {
            port.absolute_latency = self.wires[port.wire].absolute_latency;
        }

        Some(interface)
    }
    
    fn instantiate_full(&mut self) -> Option<FlatAlloc<InstantiatedPort, PortIDMarker>> {
        if self.md.link_info.errors.did_error {
            return None;// Don't instantiate modules that already errored. Otherwise instantiator may crash
        }

        for (_id, inst) in &self.md.instructions {
            inst.for_each_embedded_type(|typ,_span| {
                assert!(!typ.contains_error_or_unknown::<true,true>(), "Types brought into instantiation may not contain 'bad types': {typ:?} in {inst:?}");
            })
        }
        
    
        self.instantiate_flattened_module(self.md.instructions.id_range(), None)?;
        let interface = self.compute_latencies();
        
        if self.errors.did_error() {
            return None
        }

        interface
    }
}

pub fn perform_instantiation(name : &str, module : &Module, linker : &Linker) -> (InstantiatedModule, ErrorStore) {
    let mut context = InstantiationContext{
        generation_state : module.instructions.iter().map(|(_, _)| SubModuleOrWire::Unnasigned).collect(),
        wires : FlatAlloc::new(),
        submodules : FlatAlloc::new(),
        specified_latencies : Vec::new(),
        md : module,
        linker : linker,
        errors : ErrorCollector::new_empty(module.link_info.file, &linker.files)
    };

    let interface = context.instantiate_full();
    (InstantiatedModule{
        name : name.to_owned(),
        wires : context.wires,
        submodules : context.submodules,
        interface_ports : interface.unwrap_or(FlatAlloc::new()), // Empty value. Invalid interface can't get accessed from result of this method, as that should have produced an error
        generation_state : context.generation_state
    }, context.errors.into_storage())
}
