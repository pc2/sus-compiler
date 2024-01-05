use std::{rc::Rc, ops::Deref, cell::RefCell};

use num::BigInt;

use crate::{arena_alloc::{UUID, UUIDMarker, FlatAlloc}, ast::{Operator, IdentifierType, Span}, typing::{ConcreteType, Type}, flattening::{FlatID, Instantiation, FlatIDMarker, ConnectionWritePathElement, WireSource, WireInstance, Connection, ConnectionWritePathElementComputed, FlattenedModule}, errors::ErrorCollector, linker::{Linker, get_builtin_uuid}, value::{Value, compute_unary_op, compute_binary_op}};

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
pub enum StateInitialValue {
    Combinatorial,
    State{initial_value : Value}
}

#[derive(Debug)]
pub enum RealWireDataSource {
    ReadOnly,
    Multiplexer{is_state : StateInitialValue, sources : Vec<MultiplexerSource>},
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
    pub name : Box<str>
}

#[derive(Debug)]
pub struct SubModule {
    pub original_flat : FlatID,
    pub instance : Rc<InstantiatedModule>,
    pub wires : Vec<WireID>,
    pub name : Box<str>
}

#[derive(Debug)]
pub struct InstantiatedModule {
    pub name : Box<str>, // Unique name involving all template arguments
    pub interface : Option<Vec<WireID>>, // Interface is only valid if all wires of the interface were valid
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
    fn extract_submodule(&self) -> SubModuleID {
        let Self::SubModule(result) = self else {panic!("Failed SubModule extraction! Is {self:?} instead")};
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
    fn extract_integer_from_value<'v, IntT : TryFrom<&'v BigInt>>(&self, val : &'v Value, span : Span) -> Option<IntT> {
        let Value::Integer(val) = val else {self.errors.error_basic(span, format!("Value is not an int, it is {val:?} instead")); return None};
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
            Type::Error | Type::Unknown => unreachable!("Bad types should be caught in flattening: {}", typ.to_string(self.linker)),
            Type::Named(n) => {
                Some(ConcreteType::Named(*n))
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
    fn process_connection_to_wire(&mut self, to_path : &[ConnectionWritePathElement], from : ConnectFrom, wire_id : WireID) {
        let mut new_path : Vec<ConnectToPathElem> = Vec::new();

        let mut write_to_typ = &self.wires[wire_id].typ;

        for pe in to_path {
            match pe {
                ConnectionWritePathElement::ArrayIdx{idx, idx_span} => {
                    match &self.generation_state[*idx] {
                        SubModuleOrWire::SubModule(_) => unreachable!(),
                        SubModuleOrWire::Unnasigned => unreachable!(),
                        SubModuleOrWire::Wire(idx_wire) => {
                            assert!(self.wires[*idx_wire].typ == ConcreteType::Named(get_builtin_uuid("int")));
        
                            new_path.push(ConnectToPathElem::MuxArrayWrite{idx_wire : *idx_wire});
                        }
                        SubModuleOrWire::CompileTimeValue(v) => {
                            let Some(idx) = self.extract_integer_from_value(v, *idx_span) else {return};
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
    }
    fn convert_connection_path_to_known_values(&self, conn_path : &[ConnectionWritePathElement]) -> Option<Vec<ConnectionWritePathElementComputed>> {
        let mut result = Vec::new();
        result.reserve(conn_path.len());
        for p in conn_path {
            match p {
                ConnectionWritePathElement::ArrayIdx{idx, idx_span} => {
                    let Some(idx_val) = self.get_generation_value(*idx) else {return None};
                    let Some(idx_val) = self.extract_integer_from_value::<usize>(idx_val, *idx_span) else {return None};
                    result.push(ConnectionWritePathElementComputed::ArrayIdx(idx_val))
                }
            }
        }
        Some(result)
    }
    fn process_connection(&mut self, conn : &Connection, original_wire : FlatID) {
        match &self.generation_state[conn.to.root] {
            SubModuleOrWire::SubModule(_) => unreachable!(),
            SubModuleOrWire::Unnasigned => unreachable!(),
            SubModuleOrWire::Wire(w) => { // Runtime wire
                let deref_w = *w;
                
                let condition = conn.condition.map(|found_conn| self.generation_state[found_conn].extract_wire());

                let Some(from) = self.get_wire_or_constant_as_wire(conn.from) else {return;};
                let conn_from = ConnectFrom {
                    num_regs: conn.num_regs,
                    from,
                    condition,
                    original_wire
                };
                
                self.process_connection_to_wire(&conn.to.path, conn_from, deref_w);

                return;
            }
            SubModuleOrWire::CompileTimeValue(_original_value) => { // Compiletime wire
                let found_v = self.generation_state[conn.from].extract_generation_value().clone();
                let Some(cvt_path) = self.convert_connection_path_to_known_values(&conn.to.path) else {return};
                // Hack to get around the borrow rules here
                let SubModuleOrWire::CompileTimeValue(v_writable) = &mut self.generation_state[conn.to.root] else {unreachable!()};
                write_gen_variable(v_writable, &cvt_path, found_v);
            }
        };

    }
    fn get_generation_value(&self, v : FlatID) -> Option<&Value> {
        if let SubModuleOrWire::CompileTimeValue(vv) = &self.generation_state[v] {
            Some(vv)
        } else {
            self.errors.error_basic(self.flattened.instantiations[v].extract_wire().span, "This variable is not set at this point!");
            None
        }
    }
    fn compute_compile_time(&self, wire_inst : &WireSource) -> Option<Value> {
        Some(match wire_inst {
            &WireSource::WireRead{from_wire} => {
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
            WireSource::Constant{value} => value.clone()
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
                let Instantiation::Wire(WireInstance{typ, source : _, is_compiletime : _, span}) = &self.flattened.instantiations[flat_id] else {unreachable!()};
                let typ = self.concretize_type(typ, *span)?;
                let name = self.get_unique_name();
                Some(self.wires.alloc(RealWire{source : RealWireDataSource::Constant{value}, original_wire : flat_id, typ, name}))
            }
        }
    }
    fn wire_to_real_wire(&mut self, w: &WireInstance, typ : ConcreteType, original_wire : FlatID) -> Option<WireID> {
        let source = match &w.source {
            &WireSource::WireRead{from_wire} => {
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
            WireSource::Constant{value: _} => {
                unreachable!("Constant cannot be non-compile-time");
            }
        };
        let name = self.get_unique_name();
        Some(self.wires.alloc(RealWire{ name, typ, original_wire, source}))
    }
    fn instantiate_flattened_module(&mut self) {
        for (original_wire, inst) in &self.flattened.instantiations {
            let instance_to_add : SubModuleOrWire = match inst {
                Instantiation::SubModule(submodule) => {
                    let Some(instance) = self.linker.instantiate(submodule.module_uuid) else {continue}; // Avoid error from submodule
                    let interface_real_wires = submodule.local_wires.iter().map(|port| {
                        self.generation_state[*port].extract_wire()
                    }).collect();
                    SubModuleOrWire::SubModule(self.submodules.alloc(SubModule { original_flat: original_wire, instance, wires : interface_real_wires, name : submodule.name.clone()}))
                }
                Instantiation::WireDeclaration(wire_decl) => {
                    let Some(typ) = self.concretize_type(&wire_decl.typ, wire_decl.typ_span) else {
                        return; // Exit early, do not produce invalid wires in InstantiatedModule
                    };
                    if wire_decl.identifier_type == IdentifierType::Generative {
                        /*Do nothing (in fact re-initializes the wire to 'empty'), just corresponds to wire declaration*/
                        if wire_decl.read_only {
                            todo!("Modules can't be computed at compile time yet");
                        } 
                        let initial_value = typ.get_initial_val(self.linker);
                        assert!(initial_value.is_of_type(&typ));
                        SubModuleOrWire::CompileTimeValue(initial_value)
                    } else {
                        let source = if wire_decl.read_only {
                            RealWireDataSource::ReadOnly
                        } else {
                            // TODO initial value
                            let is_state = if wire_decl.identifier_type == IdentifierType::State{StateInitialValue::State{initial_value: Value::Unset}} else {StateInitialValue::Combinatorial};
                            RealWireDataSource::Multiplexer{is_state, sources : Vec::new()}
                        };
                        SubModuleOrWire::Wire(self.wires.alloc(RealWire{ name: wire_decl.name.clone(), typ, original_wire, source}))
                    }
                }
                Instantiation::Wire(w) => {
                    let Some(typ) = self.concretize_type(&w.typ, w.span) else {
                        return; // Exit early, do not produce invalid wires in InstantiatedModule
                    };
                    if w.is_compiletime {
                        let Some(value_computed) = self.compute_compile_time(&w.source) else {return};
                        assert!(value_computed.is_of_type(&typ));
                        SubModuleOrWire::CompileTimeValue(value_computed)
                    } else {
                        let Some(wire_found) = self.wire_to_real_wire(w, typ, original_wire) else {return};
                        SubModuleOrWire::Wire(wire_found)
                    }
                }
                Instantiation::Connection(conn) => {
                    self.process_connection(conn, original_wire);
                    continue;
                }
            };
            self.generation_state[original_wire] = instance_to_add;
        }
    }

    // Returns a proper interface if all ports involved did not produce an error. If a port did produce an error then returns None. 
    fn make_interface(&self) -> Option<Vec<WireID>> {
        let mut result = Vec::new();
        result.reserve(self.flattened.interface.interface_wires.len());
        for port in self.flattened.interface.interface_wires.iter() {
            match &self.generation_state[port.wire_id] {
                SubModuleOrWire::Wire(w) => {
                    result.push(*w)
                }
                SubModuleOrWire::Unnasigned => {
                    return None // Error building interface
                }
                _other => unreachable!() // interface wires cannot point to anything else
            }
        }
        Some(result)
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
        
            context.instantiate_flattened_module();
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