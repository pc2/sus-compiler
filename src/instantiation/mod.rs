use std::{cell::RefCell, cmp::max, iter::zip, ops::Deref, rc::Rc};

use num::BigInt;

use crate::{arena_alloc::{FlatAlloc, UUIDMarker, UUIDRange, UUID}, ast::{IdentifierType, InterfacePorts}, errors::ErrorCollector, file_position::Span, flattening::{BinaryOperator, ConnectionWritePathElement, ConnectionWritePathElementComputed, FlatID, FlatIDMarker, FlatIDRange, FlattenedModule, Instruction, UnaryOperator, WireInstance, WireSource, Write, WriteType}, instantiation::latency_algorithm::{convert_fanin_to_fanout, solve_latencies, FanInOut, LatencyCountingError}, linker::{Linker, NamedConstant}, list_of_lists::ListOfLists, typing::{ConcreteType, Type, BOOL_CONCRETE_TYPE, INT_CONCRETE_TYPE}, value::{compute_binary_op, compute_unary_op, Value}};

use self::latency_algorithm::SpecifiedLatency;

pub mod latency_algorithm;

#[derive(Debug,Clone,Copy,PartialEq,Eq,Hash)]
pub struct WireIDMarker;
impl UUIDMarker for WireIDMarker {const DISPLAY_NAME : &'static str = "wire_";}
pub type WireID = UUID<WireIDMarker>;

#[derive(Debug,Clone,Copy,PartialEq,Eq,Hash)]
pub struct SubModuleIDMarker;
impl UUIDMarker for SubModuleIDMarker {const DISPLAY_NAME : &'static str = "submodule_";}
pub type SubModuleID = UUID<SubModuleIDMarker>;

// Temporary value before proper latency is given
pub const CALCULATE_LATENCY_LATER : i64 = i64::MIN;

#[derive(Debug)]
pub struct ConnectFrom {
    pub num_regs : i64,
    pub from : WireID,
    pub condition : Option<WireID>,
    pub original_connection : FlatID
}

#[derive(Debug)]
pub enum ConnectToPathElem {
    MuxArrayWrite{idx_wire : WireID},
    ConstArrayWrite{idx : u64}
}

#[derive(Debug)]
pub struct MultiplexerSource {
    pub path : Vec<ConnectToPathElem>,
    pub from : ConnectFrom
}

impl MultiplexerSource {
    pub fn for_each_source<F : FnMut(WireID)>(&self, mut f : F) {
        f(self.from.from);
        for path_elem in &self.path {
            match path_elem {
                ConnectToPathElem::MuxArrayWrite { idx_wire } => {f(*idx_wire)}
                ConnectToPathElem::ConstArrayWrite { idx:_ } => {}
            }
        }
    }
}

#[derive(Debug)]
pub enum RealWireDataSource {
    ReadOnly,
    Multiplexer{is_state : Option<Value>, sources : Vec<MultiplexerSource>},
    UnaryOp{op : UnaryOperator, right : WireID},
    BinaryOp{op : BinaryOperator, left : WireID, right : WireID},
    ArrayAccess{arr : WireID, arr_idx : WireID},
    ConstArrayAccess{arr : WireID, arr_idx : u64},
    Constant{value : Value}
}

impl RealWireDataSource {
    fn iter_sources_with_min_latency<F : FnMut(WireID, i64)>(&self, f : &mut F) {
        match self {
            RealWireDataSource::ReadOnly => {}
            RealWireDataSource::Multiplexer { is_state: _, sources } => {
                for s in sources {
                    f(s.from.from, s.from.num_regs);
                    if let Some(c) = s.from.condition {
                        f(c, s.from.num_regs);
                    }
                }
            }
            RealWireDataSource::UnaryOp { op: _, right } => {
                f(*right, 0);
            }
            RealWireDataSource::BinaryOp { op: _, left, right } => {
                f(*left, 0);
                f(*right, 0);
            }
            RealWireDataSource::ArrayAccess { arr, arr_idx } => {
                f(*arr, 0);
                f(*arr_idx, 0);
            }
            RealWireDataSource::ConstArrayAccess { arr, arr_idx: _ } => {
                f(*arr, 0);
            }
            RealWireDataSource::Constant { value: _ } => {}
        }
    }
}

#[derive(Debug)]
pub struct RealWire {
    pub source : RealWireDataSource,
    pub original_wire : FlatID,
    pub typ : ConcreteType,
    pub name : Box<str>,
    // Before latency counting, non i64::MIN values specify specified latency
    pub absolute_latency : i64,
    pub needed_until : i64 // If needed only the same cycle it is generated, then this is absolue_latency.
}

#[derive(Debug)]
pub struct SubModule {
    pub original_flat : FlatID,
    pub instance : Rc<InstantiatedModule>,
    pub wires : InterfacePorts<WireID>,
    pub name : Box<str>
}

#[derive(Debug)]
pub struct InstantiatedModule {
    pub name : Box<str>, // Unique name involving all template arguments
    pub interface : InterfacePorts<WireID>, // Interface is only valid if all wires of the interface were valid
    pub wires : FlatAlloc<RealWire, WireIDMarker>,
    pub submodules : FlatAlloc<SubModule, SubModuleIDMarker>,
    pub generation_state : FlatAlloc<SubModuleOrWire, FlatIDMarker>
}

#[derive(Debug,Clone)]
pub enum SubModuleOrWire {
    SubModule(SubModuleID),
    Wire(WireID),
    CompileTimeValue(Value),
    // Variable doesn't exist yet
    Unnasigned
}

impl SubModuleOrWire {
    #[track_caller]
    pub fn extract_wire(&self) -> WireID {
        let Self::Wire(result) = self else {panic!("Failed wire extraction! Is {self:?} instead")};
        *result
    }
    #[track_caller]
    pub fn extract_generation_value(&self) -> &Value {
        let Self::CompileTimeValue(result) = self else {panic!("Failed GenerationValue extraction! Is {self:?} instead")};
        result
    }
}

fn write_gen_variable(mut target : &mut Value, conn_path : &[ConnectionWritePathElementComputed], to_write : Value) {
    for elem in conn_path {
        match elem {
            ConnectionWritePathElementComputed::ArrayIdx(idx) => {
                let Value::Array(a_box) = target else {unreachable!()};
                target = &mut a_box[*idx];
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
    errors : ErrorCollector,

    flattened : &'fl FlattenedModule,
    linker : &'l Linker,
}

impl<'fl, 'l> InstantiationContext<'fl, 'l> {
    fn get_generation_value(&self, v : FlatID) -> Option<&Value> {
        if let SubModuleOrWire::CompileTimeValue(vv) = &self.generation_state[v] {
            if let Value::Unset | Value::Error = vv {
                self.errors.error_basic(self.flattened.instructions[v].extract_wire().span, format!("This variable is set but it's {vv:?}!"));
                None
            } else {
                Some(vv)
            }
        } else {
            self.errors.error_basic(self.flattened.instructions[v].extract_wire().span, "This variable is not set at this point!");
            None
        }
    }
    fn extract_integer_from_value<'v, IntT : TryFrom<&'v BigInt>>(&self, val : &'v Value, span : Span) -> Option<IntT> {
        let val = val.extract_integer(); // Typecheck should cover this
        match IntT::try_from(val) {
            Ok(val) => Some(val),
            Err(_) => {
                self.errors.error_basic(span, format!("Generative integer does not fit in {}: {val}", std::any::type_name::<IntT>()));
                None
            }
        }
    }
    fn extract_integer_from_generative<'v, IntT : TryFrom<&'v BigInt>>(&'v self, idx : FlatID) -> Option<IntT> {
        let val = self.get_generation_value(idx)?;
        let span = self.flattened.instructions[idx].extract_wire().span;
        self.extract_integer_from_value(val, span)
    }

    fn concretize_type(&self, typ : &Type, span : Span) -> Option<ConcreteType> {
        match typ {
            Type::Error | Type::Unknown => unreachable!("Bad types should be caught in flattening: {}", typ.to_string(&self.linker.types)),
            Type::Named(id) => {
                Some(ConcreteType::Named(*id))
            }
            Type::Array(arr_box) => {
                let (arr_content_typ, arr_size_wire) = arr_box.deref();
                let inner_typ = self.concretize_type(arr_content_typ, span);
                let arr_size = self.extract_integer_from_generative(*arr_size_wire);
                Some(ConcreteType::Array(Box::new((inner_typ?, arr_size?))))
            }
        }
    }
    fn process_connection_to_wire(&mut self, to_path : &[ConnectionWritePathElement], from : ConnectFrom, wire_id : WireID) -> Option<()> {
        let mut new_path : Vec<ConnectToPathElem> = Vec::new();

        let mut write_to_typ = &self.wires[wire_id].typ;

        for pe in to_path {
            match pe {
                ConnectionWritePathElement::ArrayIdx{idx, bracket_span:_} => {
                    match &self.generation_state[*idx] {
                        SubModuleOrWire::SubModule(_) => unreachable!(),
                        SubModuleOrWire::Unnasigned => unreachable!(),
                        SubModuleOrWire::Wire(idx_wire) => {
                            assert!(self.wires[*idx_wire].typ == INT_CONCRETE_TYPE);
        
                            new_path.push(ConnectToPathElem::MuxArrayWrite{idx_wire : *idx_wire});
                        }
                        SubModuleOrWire::CompileTimeValue(_) => {
                            let idx = self.extract_integer_from_generative(*idx)?;
                            new_path.push(ConnectToPathElem::ConstArrayWrite{idx});
                        }
                    }
                    let ConcreteType::Array(new_write_to_typ) = write_to_typ else {unreachable!("Write to array cannot not be an array, should have been caught in Flattening")};
                    write_to_typ = &new_write_to_typ.deref().0;
                }
            }
        }

        let found_typ = &self.wires[from.from].typ;
        if write_to_typ != found_typ {
            let from_flattened_wire = self.flattened.instructions[self.wires[from.from].original_wire].extract_wire();
            let to_flattened_decl = self.flattened.instructions[self.wires[wire_id].original_wire].extract_wire_declaration();

            let found_typ_name = found_typ.to_string(&self.linker.types);
            let write_to_typ_name = write_to_typ.to_string(&self.linker.types);

            self.errors.error_with_info(from_flattened_wire.span, format!("Instantiation TypeError: Can't assign {found_typ_name} to {write_to_typ_name}"), vec![to_flattened_decl.make_declared_here(self.errors.file)]);
        }

        let RealWireDataSource::Multiplexer{is_state : _, sources} = &mut self.wires[wire_id].source else {unreachable!("Should only be a writeable wire here")};

        sources.push(MultiplexerSource{from, path : new_path});

        Some(())
    }
    fn convert_connection_path_to_known_values(&self, conn_path : &[ConnectionWritePathElement]) -> Option<Vec<ConnectionWritePathElementComputed>> {
        let mut result = Vec::new();
        result.reserve(conn_path.len());
        for p in conn_path {
            match p {
                ConnectionWritePathElement::ArrayIdx{idx, bracket_span:_} => {
                    let idx_val = self.extract_integer_from_generative(*idx)?;
                    result.push(ConnectionWritePathElementComputed::ArrayIdx(idx_val))
                }
            }
        }
        Some(result)
    }
    fn process_connection(&mut self, conn : &Write, original_connection : FlatID, condition : Option<WireID>) -> Option<()> {
        match conn.write_type {
            WriteType::Connection{num_regs, regs_span : _} => {
                match &self.generation_state[conn.to.root] {
                    SubModuleOrWire::SubModule(_) => unreachable!(),
                    SubModuleOrWire::Unnasigned => unreachable!(),
                    SubModuleOrWire::Wire(w) => { // Runtime wire
                        let deref_w = *w;
                        
                        let from = self.get_wire_or_constant_as_wire(conn.from)?;
                        let conn_from = ConnectFrom {
                            num_regs,
                            from,
                            condition,
                            original_connection
                        };
                        
                        self.process_connection_to_wire(&conn.to.path, conn_from, deref_w)?;
                    }
                    SubModuleOrWire::CompileTimeValue(_original_value) => { // Compiletime wire
                        let found_v = self.generation_state[conn.from].extract_generation_value().clone();
                        let cvt_path = self.convert_connection_path_to_known_values(&conn.to.path)?;
                        // Hack to get around the borrow rules here
                        let SubModuleOrWire::CompileTimeValue(v_writable) = &mut self.generation_state[conn.to.root] else {unreachable!()};
                        write_gen_variable(v_writable, &cvt_path, found_v);
                    }
                };
            }
            WriteType::Initial => {
                let found_v = self.get_generation_value(conn.from)?.clone();
                let cvt_path = self.convert_connection_path_to_known_values(&conn.to.path)?;
                // Hack to get around the borrow rules here
                let SubModuleOrWire::Wire(w) = &mut self.generation_state[conn.to.root] else {unreachable!()};
                let RealWireDataSource::Multiplexer{is_state : Some(initial_value), sources : _} = &mut self.wires[*w].source else {unreachable!()};
                write_gen_variable(initial_value, &cvt_path, found_v);
            }
        }
        Some(())
    }
    fn compute_compile_time(&self, wire_inst : &WireSource) -> Option<Value> {
        Some(match wire_inst {
            &WireSource::WireRead(from_wire) => {
                self.get_generation_value(from_wire)?.clone()
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
            &WireSource::ArrayAccess{arr, arr_idx} => {
                let Value::Array(arr_val) = self.get_generation_value(arr)? else {return None};
                let arr_idx_wire = self.flattened.instructions[arr_idx].extract_wire();
                let idx : usize = self.extract_integer_from_generative(arr_idx)?;
                if let Some(item) = arr_val.get(idx) {
                    item.clone()
                } else {
                    self.errors.error_basic(arr_idx_wire.span, format!("Compile-Time Array index is out of range: idx: {idx}, array size: {}", arr_val.len()));
                    return None
                }
            }
            WireSource::Constant(value) => value.clone(),
            WireSource::NamedConstant(id) => {
                let NamedConstant::Builtin{name:_, typ:_, val} = &self.linker.constants[*id];
                val.clone()
            }
        })
    }
    fn get_unique_name(&self) -> Box<str> {
        format!("_{}", self.wires.get_next_alloc_id().get_hidden_value()).into_boxed_str()
    }
    fn get_wire_or_constant_as_wire(&mut self, flat_id : FlatID) -> Option<WireID> {
        match &self.generation_state[flat_id] {
            SubModuleOrWire::SubModule(_) => unreachable!(),
            SubModuleOrWire::Unnasigned => unreachable!(),
            SubModuleOrWire::Wire(w) => Some(*w),
            SubModuleOrWire::CompileTimeValue(v) => {
                let value = v.clone();
                let Instruction::Wire(wire) = &self.flattened.instructions[flat_id] else {unreachable!()};
                let typ = self.concretize_type(&wire.typ, wire.span)?;
                let name = self.get_unique_name();
                Some(self.wires.alloc(RealWire{source : RealWireDataSource::Constant{value}, original_wire : flat_id, typ, name, absolute_latency : CALCULATE_LATENCY_LATER, needed_until : CALCULATE_LATENCY_LATER}))
            }
        }
    }
    fn wire_to_real_wire(&mut self, w: &WireInstance, typ : ConcreteType, original_wire : FlatID) -> Option<WireID> {
        let source = match &w.source {
            &WireSource::WireRead(from_wire) => {
                /*Assert*/ self.flattened.instructions[from_wire].extract_wire_declaration(); // WireReads must point to a NamedWire!
                return Some(self.generation_state[from_wire].extract_wire())
            }
            &WireSource::UnaryOp{op, right} => {
                let right = self.get_wire_or_constant_as_wire(right)?;
                RealWireDataSource::UnaryOp{op, right}
            }
            &WireSource::BinaryOp{op, left, right} => {
                let left = self.get_wire_or_constant_as_wire(left)?;
                let right = self.get_wire_or_constant_as_wire(right)?;
                RealWireDataSource::BinaryOp{op, left, right}
            }
            &WireSource::ArrayAccess{arr, arr_idx} => {
                let arr = self.get_wire_or_constant_as_wire(arr)?;
                match &self.generation_state[arr_idx] {
                    SubModuleOrWire::SubModule(_) => unreachable!(),
                    SubModuleOrWire::Unnasigned => unreachable!(),
                    SubModuleOrWire::Wire(w) => {
                        RealWireDataSource::ArrayAccess{arr, arr_idx: *w}
                    }
                    SubModuleOrWire::CompileTimeValue(_) => {
                        let arr_idx = self.extract_integer_from_generative(arr_idx)?;
                        RealWireDataSource::ConstArrayAccess{arr, arr_idx}
                    }
                }
            }
            WireSource::Constant(_) | WireSource::NamedConstant(_) => {
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
            let instance_to_add : SubModuleOrWire = match &self.flattened.instructions[original_wire] {
                Instruction::SubModule(submodule) => {
                    let Some(instance) = self.linker.instantiate(submodule.module_uuid) else {return None}; // Avoid error from submodule
                    let interface_real_wires = submodule.interface_ports.map(&mut |port, _is_input| {
                        self.generation_state[port].extract_wire()
                    });
                    SubModuleOrWire::SubModule(self.submodules.alloc(SubModule { original_flat: original_wire, instance, wires : interface_real_wires, name : submodule.name.clone()}))
                }
                Instruction::Declaration(wire_decl) => {
                    let typ = self.concretize_type(&wire_decl.typ, wire_decl.typ_expr.get_span())?;
                    if wire_decl.identifier_type.is_generative() {
                        let initial_value = typ.get_initial_val(self.linker);
                        assert!(initial_value.is_of_type(&typ));
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
                            if wire_decl.is_declared_in_this_module {
                                self.extract_integer_from_generative(*spec)?
                            } else {
                                CALCULATE_LATENCY_LATER
                            }
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
                    let typ = self.concretize_type(&w.typ, w.span)?;
                    if w.is_compiletime {
                        let value_computed = self.compute_compile_time(&w.source)?;
                        assert!(value_computed.is_of_type(&typ));
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
                    let if_condition_wire = self.flattened.instructions[stm.condition].extract_wire();
                    if if_condition_wire.is_compiletime {
                        let condition_val = self.get_generation_value(stm.condition)?;
                        let run_range = if condition_val.extract_bool() {
                            then_range
                        } else {
                            else_range
                        };
                        self.instantiate_flattened_module(run_range, condition);
                    } else {
                        let condition_wire = self.generation_state[stm.condition].extract_wire();
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
                    let start_val = self.get_generation_value(stm.start)?.extract_integer().clone();
                    let end_val = self.get_generation_value(stm.end)?.extract_integer().clone();
                    if start_val > end_val {
                        let start_flat = &self.flattened.instructions[stm.start].extract_wire();
                        let end_flat = &self.flattened.instructions[stm.end].extract_wire();
                        self.errors.error_basic(Span::new_overarching(start_flat.span, end_flat.span), format!("for loop range end is before begin: {start_val}:{end_val}"));
                        return None;
                    }

                    let mut current_val = start_val;

                    while current_val < end_val {
                        let SubModuleOrWire::CompileTimeValue(v) = &mut self.generation_state[stm.loop_var_decl] else {unreachable!()};
                        *v = Value::Integer(current_val.clone());
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

    fn make_fanins(&self) -> (ListOfLists<FanInOut>, Vec<SpecifiedLatency>) {
        let mut fanins : ListOfLists<FanInOut> = ListOfLists::new_with_groups_capacity(self.wires.len());
        let mut initial_latencies = Vec::new();
        
        // Wire to wire Fanin
        for (id, wire) in &self.wires {
            fanins.new_group();
            wire.source.iter_sources_with_min_latency(&mut |from, delta_latency| {
                fanins.push_to_last_group(FanInOut{other : from.get_hidden_value(), delta_latency});
            });

            // Submodules Fanin
            // This creates two way connections, from any input i to output o it creates a |o| - |i| length connection, and a -(|o| - |i|) backward connection. This fixes them to be an exact latency apart. 
            // This is O(lots) but doesn't matter, usually very few submodules. Fix this if needed
            for (_id, sub_mod) in &self.submodules {
                for (self_output, submodule_output) in zip(sub_mod.wires.outputs(), sub_mod.instance.interface.outputs()) {
                    if *self_output != id {continue}
                    for (self_input, submodule_input) in zip(sub_mod.wires.inputs(), sub_mod.instance.interface.inputs()) {
                        
                        let delta_latency = sub_mod.instance.wires[*submodule_output].absolute_latency - sub_mod.instance.wires[*submodule_input].absolute_latency;
        
                        fanins.push_to_last_group(FanInOut{other: self_input.get_hidden_value(), delta_latency});
                    }
                }
                // Also have to add inverse connections, such that the ports of the module are well and truly fixed together
                for (self_input, submodule_input) in zip(sub_mod.wires.inputs(), sub_mod.instance.interface.inputs()) {
                    if *self_input != id {continue}
                    for (self_output, submodule_output) in zip(sub_mod.wires.outputs(), sub_mod.instance.interface.outputs()) {
                        
                        let delta_latency = sub_mod.instance.wires[*submodule_output].absolute_latency - sub_mod.instance.wires[*submodule_input].absolute_latency;
        
                        fanins.push_to_last_group(FanInOut{other: self_output.get_hidden_value(), delta_latency : -delta_latency});
                    }
                }
            }

            if wire.absolute_latency != CALCULATE_LATENCY_LATER {
                initial_latencies.push(SpecifiedLatency { wire: id.get_hidden_value(), latency: wire.absolute_latency })
            }
        }

        (fanins, initial_latencies)
    }

    // Computes all latencies involved
    pub fn compute_latencies(&mut self, ports : &InterfacePorts<WireID>) -> Option<()> {
        let (fanins, initial_latencies) = self.make_fanins();
        
        // Process fanouts
        let fanouts = convert_fanin_to_fanout(&fanins);

        let inputs : Vec<usize> = ports.inputs().iter().map(|input| input.get_hidden_value()).collect();
        let outputs : Vec<usize> = ports.outputs().iter().map(|input| input.get_hidden_value()).collect();

        
        match solve_latencies(&fanins, &fanouts, &inputs, &outputs, initial_latencies) {
            Ok(latencies) => {
                for ((_id, wire), lat) in zip(self.wires.iter_mut(), latencies.iter()) {
                    wire.absolute_latency = *lat;
                    if *lat == CALCULATE_LATENCY_LATER {
                        if let Some(source_location) = self.flattened.instructions[wire.original_wire].get_location_of_module_part() {
                            self.errors.error_basic(source_location, format!("Latency Counting couldn't reach this node"));
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
                        let unique_write_instructions = filter_unique_write_flats(&writes_involved, &self.flattened.instructions);
                        let rest_of_message = format!(" part of a net-positive latency cycle of +{net_roundtrip_latency}\n\n{path_message}\nWhich conflicts with the starting latency");
                        
                        let mut did_place_error = false;
                        for wr in &unique_write_instructions {
                            match wr.write_type {
                                WriteType::Connection { num_regs, regs_span } => {
                                    if num_regs >= 1 {
                                        did_place_error = true;
                                        let this_register_plural = if num_regs == 1 {"This register is"} else {"These registers are"};
                                        self.errors.error_basic(regs_span, format!("{this_register_plural}{rest_of_message}"));
                                    }
                                }
                                WriteType::Initial => {unreachable!("Initial assignment can only be from compile-time constant. Cannot be part of latency loop. ")}
                            }
                        }
                        // Fallback if no register annotations used
                        if !did_place_error {
                            for wr in unique_write_instructions {
                                self.errors.error_basic(wr.to.span, format!("This write is{rest_of_message}"));
                            }
                        }
                    }
                    LatencyCountingError::IndeterminablePortLatency { bad_ports } => {
                        for port in bad_ports {
                            let port_decl = self.flattened.instructions[self.wires[WireID::from_hidden_value(port.0)].original_wire].extract_wire_declaration();
                            self.errors.error_basic(port_decl.name_span, format!("Cannot determine port latency. Options are {} and {}\nTry specifying an explicit latency or rework the module to remove this ambiguity", port.1, port.2));
                        }
                    }
                    LatencyCountingError::ConflictingSpecifiedLatencies { conflict_path } => {
                        let start_wire = &self.wires[WireID::from_hidden_value(conflict_path.first().unwrap().wire)];
                        let end_wire = &self.wires[WireID::from_hidden_value(conflict_path.last().unwrap().wire)];
                        let start_decl = self.flattened.instructions[start_wire.original_wire].extract_wire_declaration();
                        let end_decl = self.flattened.instructions[end_wire.original_wire].extract_wire_declaration();
                        let end_latency_decl = self.flattened.instructions[end_decl.latency_specifier.unwrap()].extract_wire();
                        

                        let writes_involved = gather_all_mux_inputs(&self.wires, &conflict_path);
                        let path_message = make_path_info_string(&writes_involved, start_wire.absolute_latency, &start_wire.name);
                        //assert!(!writes_involved.is_empty());

                        let end_name = &end_wire.name;
                        let specified_end_latency = end_wire.absolute_latency;
                        let reason = format!("Conflicting specified latency\n\n{path_message}\nBut this was specified as {end_name}'{specified_end_latency}");
                        self.errors.error_with_info(end_latency_decl.span, reason, vec![start_decl.make_declared_here(self.errors.file)]);
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

        Some(())
    }
    

    // Returns a proper interface if all ports involved did not produce an error. If a port did produce an error then returns None. 
    fn make_interface(&self) -> InterfacePorts<WireID> {
        let mut result = Vec::new();
        result.reserve(self.flattened.interface_ports.ports.len());
        for port in self.flattened.interface_ports.ports.iter() {
            match &self.generation_state[*port] {
                SubModuleOrWire::Wire(w) => {
                    result.push(*w)
                }
                SubModuleOrWire::Unnasigned => {
                    unreachable!() // Error building interface
                }
                _other => unreachable!() // interface wires cannot point to anything else
            }
        }
        InterfacePorts{ports : result.into_boxed_slice(), outputs_start : self.flattened.interface_ports.outputs_start}
    }


    fn instantiate_full(&mut self) -> Option<InterfacePorts<WireID>> {
        if self.flattened.errors.did_error.get() {
            return None;// Don't instantiate modules that already errored. Otherwise instantiator may crash
        }

        for (_id, inst) in &self.flattened.instructions {
            inst.for_each_embedded_type(&mut |typ,_span| {
                assert!(!typ.contains_error_or_unknown::<true,true>(), "Types brought into instantiation may not contain 'bad types': {typ:?} in {inst:?}");
            })
        }
        
    
        self.instantiate_flattened_module(self.flattened.instructions.id_range(), None)?;
        let interface = self.make_interface();
        self.compute_latencies(&interface)?;
        
        if self.errors.did_error.get() {
            return None
        }

        Some(interface)
    }
}


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
        //let Instruction::Declaration(decl) = &self.flattened.instructions[decl_id] else {unreachable!()};

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
        let original_write = instructions[w.mux_input.from.original_connection].extract_write();
        
        if !result.iter().any(|found_write| std::ptr::eq(*found_write, original_write)) {result.push(original_write)}
    }
    result
}



#[derive(Debug)]
pub struct InstantiationList {
    cache : RefCell<Vec<(Rc<InstantiatedModule>, ErrorCollector)>>
}

impl InstantiationList {
    pub fn new() -> Self {
        Self{cache : RefCell::new(Vec::new())}
    }

    pub fn instantiate(&self, name : &str, flattened : &FlattenedModule, linker : &Linker) -> Option<Rc<InstantiatedModule>> {
        let mut cache_borrow = self.cache.borrow_mut();
        
        // Temporary, no template arguments yet
        if cache_borrow.is_empty() {
            let mut context = InstantiationContext{
                generation_state : flattened.instructions.iter().map(|(_, _)| SubModuleOrWire::Unnasigned).collect(),
                wires : FlatAlloc::new(),
                submodules : FlatAlloc::new(),
                specified_latencies : Vec::new(),
                flattened : &flattened,
                linker : linker,
                errors : ErrorCollector::new(flattened.errors.file)
            };

            let interface = context.instantiate_full();
            let result = Rc::new(InstantiatedModule{
                name : name.to_owned().into_boxed_str(),
                wires : context.wires,
                submodules : context.submodules,
                interface : interface.unwrap_or(InterfacePorts::empty()), // Empty value. Invalid interface can't get accessed from result of this method, as that should have produced an error
                generation_state : context.generation_state
            });

            if context.errors.did_error.get() {
                cache_borrow.push((result, context.errors));
                return None;
            } else {
                cache_borrow.push((result.clone(), context.errors));
                return Some(result);
            }
        }
        
        let instance_id = 0; // Temporary, will always be 0 while not template arguments
        let instance = &cache_borrow[instance_id];
        if !instance.1.did_error.get() {
            Some(instance.0.clone())
        } else {
            None
        }
    }

    pub fn collect_errors(&self, errors : &ErrorCollector) {
        let cache_borrow = self.cache.borrow();
        for inst in cache_borrow.deref() {
            errors.ingest(&inst.1);
        }
    }

    pub fn clear_instances(&mut self) {
        self.cache.borrow_mut().clear()
    }

    // Also passes over invalid instances. Instance validity should not be assumed!
    // Only used for things like syntax highlighting
    pub fn for_each_instance<F : FnMut(&InstantiatedModule)>(&self, mut f : F) {
        let borrow = self.cache.borrow();
        for v in borrow.iter() {
            f(v.0.as_ref())
        }
    }
}
