use std::{rc::Rc, ops::Deref, cell::RefCell};

use num::BigInt;

use crate::{arena_alloc::{UUID, UUIDMarker, FlatAlloc, UUIDRange}, ast::{Operator, IdentifierType, Span, InterfacePorts}, typing::{ConcreteType, Type, BOOL_CONCRETE_TYPE, INT_CONCRETE_TYPE}, flattening::{Write, ConnectionWritePathElement, ConnectionWritePathElementComputed, FlatID, FlatIDMarker, FlatIDRange, FlattenedModule, Instantiation, WireInstance, WireSource, WriteType}, errors::ErrorCollector, linker::{Linker, NamedConstant}, value::{Value, compute_unary_op, compute_binary_op}, tokenizer::kw};

pub mod latency_algorithm;
pub mod latency;

#[derive(Debug,Clone,Copy,PartialEq,Eq,Hash)]
pub struct WireIDMarker;
impl UUIDMarker for WireIDMarker {const DISPLAY_NAME : &'static str = "wire_";}
pub type WireID = UUID<WireIDMarker>;

#[derive(Debug,Clone,Copy,PartialEq,Eq,Hash)]
pub struct SubModuleIDMarker;
impl UUIDMarker for SubModuleIDMarker {const DISPLAY_NAME : &'static str = "submodule_";}
pub type SubModuleID = UUID<SubModuleIDMarker>;

#[derive(Debug)]
pub struct ConnectFrom {
    pub num_regs : i64,
    pub from : WireID,
    pub condition : Option<WireID>,
    pub original_wire : FlatID
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

#[derive(Debug)]
pub enum RealWireDataSource {
    ReadOnly,
    Multiplexer{is_state : Option<Value>, sources : Vec<MultiplexerSource>},
    UnaryOp{op : Operator, right : WireID},
    BinaryOp{op : Operator, left : WireID, right : WireID},
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
                        f(c, 0);
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
    pub latency_specifier : Option<u64>
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
    pub interface : Option<InterfacePorts<WireID>>, // Interface is only valid if all wires of the interface were valid
    pub wires : FlatAlloc<RealWire, WireIDMarker>,
    pub submodules : FlatAlloc<SubModule, SubModuleIDMarker>,
    pub errors : ErrorCollector,
}

#[derive(Debug,Clone)]
enum SubModuleOrWire {
    SubModule(SubModuleID),
    Wire(WireID),
    CompileTimeValue(Value),
    // Variable doesn't exist yet
    Unnasigned
}

impl SubModuleOrWire {
    #[track_caller]
    fn extract_wire(&self) -> WireID {
        let Self::Wire(result) = self else {panic!("Failed wire extraction! Is {self:?} instead")};
        *result
    }
    #[track_caller]
    fn extract_generation_value(&self) -> &Value {
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
    errors : ErrorCollector,

    flattened : &'fl FlattenedModule,
    linker : &'l Linker,
}

impl<'fl, 'l> InstantiationContext<'fl, 'l> {
    fn get_generation_value(&self, v : FlatID) -> Option<&Value> {
        if let SubModuleOrWire::CompileTimeValue(vv) = &self.generation_state[v] {
            if let Value::Unset | Value::Error = vv {
                self.errors.error_basic(self.flattened.instantiations[v].extract_wire().span, format!("This variable is set but it's {vv:?}!"));
                None
            } else {
                Some(vv)
            }
        } else {
            self.errors.error_basic(self.flattened.instantiations[v].extract_wire().span, "This variable is not set at this point!");
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
    fn concretize_type(&self, typ : &Type, span : Span) -> Option<ConcreteType> {
        match typ {
            Type::Error | Type::Unknown => unreachable!("Bad types should be caught in flattening: {}", typ.to_string(&self.linker.types)),
            Type::Named{id, span : _} => {
                Some(ConcreteType::Named(*id))
            }
            Type::Array(arr_box) => {
                let (arr_content_typ, arr_size_wire) = arr_box.deref();
                let inner_typ = self.concretize_type(arr_content_typ, span)?;
                let size_val = &self.generation_state[*arr_size_wire];
                let cv = size_val.extract_generation_value();
                let arr_usize = self.extract_integer_from_value(cv, span)?;
                Some(ConcreteType::Array(Box::new((inner_typ, arr_usize))))
            }
        }
    }
    fn process_connection_to_wire(&mut self, to_path : &[ConnectionWritePathElement], from : ConnectFrom, wire_id : WireID) -> Option<()> {
        let mut new_path : Vec<ConnectToPathElem> = Vec::new();

        let mut write_to_typ = &self.wires[wire_id].typ;

        for pe in to_path {
            match pe {
                ConnectionWritePathElement::ArrayIdx{idx, idx_span} => {
                    match &self.generation_state[*idx] {
                        SubModuleOrWire::SubModule(_) => unreachable!(),
                        SubModuleOrWire::Unnasigned => unreachable!(),
                        SubModuleOrWire::Wire(idx_wire) => {
                            assert!(self.wires[*idx_wire].typ == INT_CONCRETE_TYPE);
        
                            new_path.push(ConnectToPathElem::MuxArrayWrite{idx_wire : *idx_wire});
                        }
                        SubModuleOrWire::CompileTimeValue(v) => {
                            let idx = self.extract_integer_from_value(v, *idx_span)?;
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
            // todo!();
            //TODO
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
                ConnectionWritePathElement::ArrayIdx{idx, idx_span} => {
                    let idx_val = self.get_generation_value(*idx)?;
                    let idx_val = self.extract_integer_from_value::<usize>(idx_val, *idx_span)?;
                    result.push(ConnectionWritePathElementComputed::ArrayIdx(idx_val))
                }
            }
        }
        Some(result)
    }
    fn process_connection(&mut self, conn : &Write, original_wire : FlatID, condition : Option<WireID>) -> Option<()> {
        match conn.write_type {
            WriteType::Connection{num_regs} => {
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
                            original_wire
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
                let arr_idx_val = self.get_generation_value(arr_idx)?;
                let arr_idx_wire = self.flattened.instantiations[arr_idx].extract_wire();
                let idx : usize = self.extract_integer_from_value(arr_idx_val, arr_idx_wire.span)?;
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
                let Instantiation::Wire(wire) = &self.flattened.instantiations[flat_id] else {unreachable!()};
                let typ = self.concretize_type(&wire.typ, wire.span)?;
                let name = self.get_unique_name();
                Some(self.wires.alloc(RealWire{source : RealWireDataSource::Constant{value}, original_wire : flat_id, typ, name, latency_specifier : None}))
            }
        }
    }
    fn wire_to_real_wire(&mut self, w: &WireInstance, typ : ConcreteType, original_wire : FlatID) -> Option<WireID> {
        let source = match &w.source {
            &WireSource::WireRead(from_wire) => {
                /*Assert*/ self.flattened.instantiations[from_wire].extract_wire_declaration(); // WireReads must point to a NamedWire!
                return Some(self.generation_state[from_wire].extract_wire())
            }
            &WireSource::UnaryOp{op, right} => {
                let right = self.get_wire_or_constant_as_wire(right)?;
                RealWireDataSource::UnaryOp{op: op, right}
            }
            &WireSource::BinaryOp{op, left, right} => {
                let left = self.get_wire_or_constant_as_wire(left)?;
                let right = self.get_wire_or_constant_as_wire(right)?;
                RealWireDataSource::BinaryOp{op: op, left, right}
            }
            &WireSource::ArrayAccess{arr, arr_idx} => {
                let arr = self.get_wire_or_constant_as_wire(arr)?;
                match &self.generation_state[arr_idx] {
                    SubModuleOrWire::SubModule(_) => unreachable!(),
                    SubModuleOrWire::Unnasigned => unreachable!(),
                    SubModuleOrWire::Wire(w) => {
                        RealWireDataSource::ArrayAccess{arr, arr_idx: *w}
                    }
                    SubModuleOrWire::CompileTimeValue(v) => {
                        let arr_idx_wire = self.flattened.instantiations[arr_idx].extract_wire();
                        let arr_idx = self.extract_integer_from_value(v, arr_idx_wire.span)?;
                        RealWireDataSource::ConstArrayAccess{arr, arr_idx}
                    }
                }
            }
            WireSource::Constant(_) | WireSource::NamedConstant(_) => {
                unreachable!("Constant cannot be non-compile-time");
            }
        };
        let name = self.get_unique_name();
        Some(self.wires.alloc(RealWire{name, typ, original_wire, source, latency_specifier : None}))
    }
    fn extend_condition(&mut self, condition : Option<WireID>, additional_condition : WireID, original_wire : FlatID) -> WireID {
        if let Some(condition) = condition {
            self.wires.alloc(RealWire{
                typ : BOOL_CONCRETE_TYPE,
                name : self.get_unique_name(),
                original_wire,
                source : RealWireDataSource::BinaryOp{
                    op: Operator{op_typ : kw("&")},
                    left : condition,
                    right : additional_condition
                },
                latency_specifier : None})
        } else {
            additional_condition
        }
    }
    fn instantiate_flattened_module(&mut self, flat_range : FlatIDRange, condition : Option<WireID>) -> Option<()> {
        let mut instruction_range = flat_range.into_iter();
        while let Some(original_wire) = instruction_range.next() {
            let instance_to_add : SubModuleOrWire = match &self.flattened.instantiations[original_wire] {
                Instantiation::SubModule(submodule) => {
                    let Some(instance) = self.linker.instantiate(submodule.module_uuid) else {return None}; // Avoid error from submodule
                    let interface_real_wires = submodule.interface_ports.map(&mut |port, _is_input| {
                        self.generation_state[port].extract_wire()
                    });
                    SubModuleOrWire::SubModule(self.submodules.alloc(SubModule { original_flat: original_wire, instance, wires : interface_real_wires, name : submodule.name.clone()}))
                }
                Instantiation::Declaration(wire_decl) => {
                    let typ = self.concretize_type(&wire_decl.typ, wire_decl.typ_span)?;
                    if wire_decl.identifier_type == IdentifierType::Generative {
                        /*Do nothing (in fact re-initializes the wire to 'empty'), just corresponds to wire declaration*/
                        /*if wire_decl.read_only { // Don't know why this check is *here*
                            todo!("Modules can't be computed at compile time yet");
                        }*/
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
                        let latency_specifier = if let Some(lat_spec_flat) = wire_decl.latency_specifier {
                            let val = self.get_generation_value(lat_spec_flat)?;
                            Some(self.extract_integer_from_value(val, self.flattened.instantiations[lat_spec_flat].extract_wire().span)?)
                        } else {
                            None
                        };
                        SubModuleOrWire::Wire(self.wires.alloc(RealWire{name: wire_decl.name.clone(), typ, original_wire, source, latency_specifier}))
                    }
                }
                Instantiation::Wire(w) => {
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
                Instantiation::Write(conn) => {
                    self.process_connection(conn, original_wire, condition);
                    continue;
                }
                Instantiation::IfStatement(stm) => {
                    let then_range = UUIDRange(stm.then_start, stm.then_end_else_start);
                    let else_range = UUIDRange(stm.then_end_else_start, stm.else_end);
                    let if_condition_wire = self.flattened.instantiations[stm.condition].extract_wire();
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
                                    op : Operator{op_typ : kw("!")},
                                    right : condition_wire
                                },
                                latency_specifier : None
                            });
                            let else_cond = self.extend_condition(condition, else_condition_bool, original_wire);
                            self.instantiate_flattened_module(else_range, Some(else_cond));
                        }
                    }
                    instruction_range.skip_to(stm.else_end);
                    continue;
                }
                Instantiation::ForStatement(stm) => {
                    // TODO Non integer for loops?
                    let start_val = self.get_generation_value(stm.start)?.extract_integer().clone();
                    let end_val = self.get_generation_value(stm.end)?.extract_integer().clone();
                    if start_val > end_val {
                        let start_flat = &self.flattened.instantiations[stm.start].extract_wire();
                        let end_flat = &self.flattened.instantiations[stm.end].extract_wire();
                        self.errors.error_basic(Span(start_flat.span.0, end_flat.span.1), format!("for loop range end is before begin: {start_val}:{end_val}"));
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

    // Returns a proper interface if all ports involved did not produce an error. If a port did produce an error then returns None. 
    fn make_interface(&self) -> Option<InterfacePorts<WireID>> {
        let mut result = Vec::new();
        result.reserve(self.flattened.interface_ports.ports.len());
        for port in self.flattened.interface_ports.ports.iter() {
            match &self.generation_state[*port] {
                SubModuleOrWire::Wire(w) => {
                    result.push(*w)
                }
                SubModuleOrWire::Unnasigned => {
                    return None // Error building interface
                }
                _other => unreachable!() // interface wires cannot point to anything else
            }
        }
        Some(InterfacePorts{ports : result.into_boxed_slice(), outputs_start : self.flattened.interface_ports.outputs_start})
    }
}



#[derive(Debug)]
pub struct InstantiationList {
    cache : RefCell<Vec<Rc<InstantiatedModule>>>
}

impl InstantiationList {
    pub fn new() -> Self {
        Self{cache : RefCell::new(Vec::new())}
    }

    pub fn instantiate(&self, name : &str, flattened : &FlattenedModule, linker : &Linker) -> Option<Rc<InstantiatedModule>> {
        if flattened.errors.did_error.get() {
            return None;// Don't instantiate modules that already errored. Otherwise instantiator may crash
        }

        let mut cache_borrow = self.cache.borrow_mut();
        
        // Temporary, no template arguments yet
        if cache_borrow.is_empty() {
            for (_id, inst) in &flattened.instantiations {
                inst.for_each_embedded_type(&mut |typ,_span| {
                    assert!(!typ.contains_error_or_unknown::<true,true>(), "Types brought into instantiation may not contain 'bad types': {typ:?} in {inst:?}");
                })
            }    
            let mut context = InstantiationContext{
                generation_state : flattened.instantiations.iter().map(|(_, _)| SubModuleOrWire::Unnasigned).collect(),
                wires : FlatAlloc::new(),
                submodules : FlatAlloc::new(),
                flattened : &flattened,
                linker : linker,
                errors : ErrorCollector::new(flattened.errors.file)
            };
        
            context.instantiate_flattened_module(flattened.instantiations.id_range(), None);
            let interface = context.make_interface();
            
            cache_borrow.push(Rc::new(InstantiatedModule{
                name : name.to_owned().into_boxed_str(),
                wires : context.wires,
                submodules : context.submodules,
                interface,
                errors : context.errors
            }));
        }
        
        let instance_id = 0; // Temporary, will always be 0 while not template arguments
        Some(cache_borrow[instance_id].clone())
    }

    pub fn collect_errors(&self, errors : &ErrorCollector) {
        let cache_borrow = self.cache.borrow();
        for inst in cache_borrow.deref() {
            errors.ingest(&inst.errors);
        }
    }

    pub fn clear_instances(&mut self) {
        self.cache.borrow_mut().clear()
    }
}
