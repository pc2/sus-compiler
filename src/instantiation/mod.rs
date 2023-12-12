use std::{rc::Rc, ops::Deref, cell::RefCell};

use crate::{arena_alloc::{UUID, UUIDMarker, FlatAlloc}, ast::{Value, Operator, Module}, typing::{ConcreteType, Type}, flattening::{FlatID, Instantiation, FlatIDMarker, ConnectionWrite, ConnectionWritePathElement, InterfacePort}, errors::ErrorCollector, linker::{Linker, get_builtin_uuid, NamedUUID}};

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
    num_regs : i64,
    from : WireID,
    condition : WireID
}

#[derive(Debug)]
pub enum ConnectToPathElem {
    ArrayConnection{idx_wire : WireID}
}

#[derive(Debug)]
pub struct MultiplexerSource {
    pub path : Vec<ConnectToPathElem>,
    pub from : ConnectFrom
}

#[derive(Debug)]
pub enum RealWireDataSource {
    ReadOnly,
    Multiplexer{sources : Vec<MultiplexerSource>},
    UnaryOp{op : Operator, right : WireID},
    BinaryOp{op : Operator, left : WireID, right : WireID},
    ArrayAccess{arr : WireID, arr_idx : WireID},
    Constant{value : Value}
}

impl RealWireDataSource {
    fn iter_sources_with_min_latency<F : FnMut(WireID, i64) -> ()>(&self, mut f : F) {
        match self {
            RealWireDataSource::ReadOnly => {}
            RealWireDataSource::Multiplexer { sources } => {
                for s in sources {
                    f(s.from.from, s.from.num_regs);
                    f(s.from.condition, 0);
                }
            }
            RealWireDataSource::UnaryOp { op, right } => {
                f(*right, 0);
            }
            RealWireDataSource::BinaryOp { op, left, right } => {
                f(*left, 0);
                f(*right, 0);
            }
            RealWireDataSource::ArrayAccess { arr, arr_idx } => {
                f(*arr, 0);
                f(*arr_idx, 0);
            }
            RealWireDataSource::Constant { value } => {}
        }
    }
}

#[derive(Debug)]
pub struct RealWire {
    source : RealWireDataSource,
    original_wire : FlatID,
    typ : ConcreteType,
    latency : i64
}

#[derive(Debug,Clone,Copy)]
pub struct InstantiatedInterfacePort {
    pub id : WireID,
    pub is_input : bool,
    pub absolute_latency : i64
}

#[derive(Debug)]
pub struct SubModule {
    original_flat : FlatID,
    instance : Rc<InstantiatedModule>,
    wires : Vec<WireID>
}

#[derive(Debug)]
pub struct InstantiatedModule {
    pub interface : Vec<InstantiatedInterfacePort>,
    pub wires : FlatAlloc<RealWire, WireIDMarker>,
    pub submodules : FlatAlloc<SubModule, SubModuleIDMarker>,
    pub errors : ErrorCollector,
}

#[derive(Clone,Copy)]
enum SubModuleOrWire {
    SubModule(SubModuleID),
    Wire(WireID),
    Unnasigned
}

impl SubModuleOrWire {
    fn extract_wire(&self) -> WireID {
        let Self::Wire(result) = self else {panic!("Failed wire extraction!")};
        *result
    }
    fn extract_submodule(&self) -> SubModuleID {
        let Self::SubModule(result) = self else {panic!("Failed SubModule extraction!")};
        *result
    }
}

struct InstantiationContext<'fl, 'l> {
    instance_map : FlatAlloc<SubModuleOrWire, FlatIDMarker>,
    wires : FlatAlloc<RealWire, WireIDMarker>,
    submodules : FlatAlloc<SubModule, SubModuleIDMarker>,
    interface : Vec<InstantiatedInterfacePort>,
    errors : ErrorCollector,

    module : &'fl Module,
    linker : &'l Linker,
}

impl<'fl, 'l> InstantiationContext<'fl, 'l> {
    fn compute_constant(&self, wire : FlatID) -> Value {
        let Instantiation::Constant { typ : _, value } = &self.module.flattened.instantiations[wire] else {todo!()};
        value.clone()
    }
    fn concretize_type(&self, typ : &Type) -> ConcreteType {
        match typ {
            Type::Named(n) => {
                ConcreteType::Named(*n)
            },
            Type::Array(arr_box) => {
                let (arr_content_typ, arr_size_wire) = arr_box.deref();
                let inner_typ = self.concretize_type(arr_content_typ);
                let Value::Integer(v) = self.compute_constant(*arr_size_wire) else {panic!("Not an int, should have been solved beforehand!")};
                let arr_usize = u64::try_from(v).expect("Array size should be small enough");
                ConcreteType::Array(Box::new((inner_typ, arr_usize)))
            },
        }
    }
    fn process_connection(&mut self, to : &ConnectionWrite, from : ConnectFrom) {
        let mut new_path : Vec<ConnectToPathElem> = Vec::new();

        let mut write_to_typ = &self.wires[self.instance_map[to.root].extract_wire()].typ;
        
        for pe in &to.path {
            match pe {
                ConnectionWritePathElement::ArrayIdx(arr_idx) => {
                    let idx_wire = self.instance_map[arr_idx.0].extract_wire();

                    assert!(self.wires[idx_wire].typ == ConcreteType::Named(get_builtin_uuid("int")));

                    new_path.push(ConnectToPathElem::ArrayConnection { idx_wire });

                    let ConcreteType::Array(new_write_to_typ) = write_to_typ else {unreachable!("Cannot not be an array")};
                    write_to_typ = &new_write_to_typ.deref().0;
                }
            }
        }

        let found_typ = &self.wires[from.from].typ;
        if write_to_typ != found_typ {
            todo!();
        }

        let RealWire{latency : _, typ : _, original_wire: _, source : RealWireDataSource::Multiplexer { sources }} = &mut self.wires[self.instance_map[to.root].extract_wire()] else {unreachable!("Should only be a writeable wire here")};

        sources.push(MultiplexerSource{from, path : new_path})
    }
    fn add_wire(&mut self, typ : &Type, original_wire : FlatID, source : RealWireDataSource) -> SubModuleOrWire {
        SubModuleOrWire::Wire(self.wires.alloc(RealWire{ latency : i64::MIN /* Invalid */, typ : self.concretize_type(typ), original_wire, source}))
    }
    fn compute_absolute_latencies(&mut self) {

        

        
    }


    fn instantiate_flattened_module(&mut self) {
        for (original_wire, inst) in &self.module.flattened.instantiations {
            let instance_to_add : SubModuleOrWire = match inst {
                Instantiation::SubModule{module_uuid, typ_span, interface_wires} => {
                    let instance = self.linker.instantiate(*module_uuid);
                    let interface_real_wires = interface_wires.iter().map(|port| {
                        self.instance_map[port.id].extract_wire()
                    }).collect();
                    SubModuleOrWire::SubModule(self.submodules.alloc(SubModule { original_flat: original_wire, instance, wires : interface_real_wires}))
                },
                Instantiation::PlainWire{read_only, typ, decl_id} => {
                    let source = if *read_only {
                        RealWireDataSource::ReadOnly
                    } else {
                        RealWireDataSource::Multiplexer {sources : Vec::new()}
                    };
                    self.add_wire(typ, original_wire, source)
                },
                Instantiation::UnaryOp{typ, op, right} => {
                    self.add_wire(typ, original_wire, RealWireDataSource::UnaryOp{op: *op, right: self.instance_map[right.0].extract_wire() })
                },
                Instantiation::BinaryOp{typ, op, left, right} => {
                    self.add_wire(typ, original_wire, RealWireDataSource::BinaryOp{op: *op, left: self.instance_map[left.0].extract_wire(), right: self.instance_map[right.0].extract_wire() })
                },
                Instantiation::ArrayAccess{typ, arr, arr_idx} => {
                    self.add_wire(typ, original_wire, RealWireDataSource::ArrayAccess{arr: self.instance_map[arr.0].extract_wire(), arr_idx: self.instance_map[arr_idx.0].extract_wire() })
                },
                Instantiation::Constant{typ, value} => {
                    self.add_wire(typ, original_wire, RealWireDataSource::Constant{value : value.clone() })
                },
                Instantiation::Error => {unreachable!()},
            };
            self.instance_map[original_wire] = instance_to_add;
        }
        for conn in &self.module.flattened.connections {
            let condition = if conn.condition != UUID::INVALID {
                self.instance_map[conn.condition].extract_wire()
            } else {
                UUID::INVALID
            };
            let conn_from = ConnectFrom {
                num_regs: conn.num_regs,
                from: self.instance_map[conn.from.0].extract_wire(), // TODO Span?
                condition,
            };
            
            self.process_connection(&conn.to, conn_from);
        }
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

    pub fn instantiate(&self, module : &Module, linker : &Linker) -> Rc<InstantiatedModule> {
        let mut cache_borrow = self.cache.borrow_mut();
        
        // Temporary, no template arguments yet
        if cache_borrow.is_empty() {
            let mut context = InstantiationContext{
                instance_map : module.flattened.instantiations.iter().map(|(_, _)| SubModuleOrWire::Unnasigned).collect(),
                wires : FlatAlloc::new(),
                submodules : FlatAlloc::new(),
                interface : Vec::new(),
                module : module,
                linker : linker,
                errors : ErrorCollector::new(module.flattened.errors.file)
            };
        
            context.instantiate_flattened_module();
            
            cache_borrow.push(Rc::new(InstantiatedModule{
                wires : context.wires,
                submodules : context.submodules,
                interface : context.interface,
                errors : context.errors
            }));
        }
        
        let instance_id = 0; // Temporary, will always be 0 while not template arguments
        cache_borrow[instance_id].clone()
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
