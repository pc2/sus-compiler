
mod latency_algorithm;
mod execute;

use std::{cell::RefCell, ops::Deref, rc::Rc};

use crate::{
    arena_alloc::{FlatAlloc, UUIDMarker, UUID},
    errors::{CompileError, ErrorStore},
    file_position::BracketSpan,
    flattening::{PortIDMarker, BinaryOperator, FlatID, FlatIDMarker, Module, UnaryOperator},
    linker::{Linker, ModuleUUID},
    typing::ConcreteType,
    value::{TypedValue, Value}
};

use self::{execute::perform_instantiation, latency_algorithm::SpecifiedLatency};

pub struct WireIDMarker;
impl UUIDMarker for WireIDMarker {const DISPLAY_NAME : &'static str = "wire_";}
pub type WireID = UUID<WireIDMarker>;

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
pub enum RealWirePathElem {
    MuxArrayWrite{span : BracketSpan, idx_wire : WireID},
    ConstArrayWrite{span : BracketSpan, idx : Option<usize>} // Maybe errored
}

#[derive(Debug)]
pub struct MultiplexerSource {
    pub path : Vec<RealWirePathElem>,
    pub from : ConnectFrom
}

impl MultiplexerSource {
    pub fn for_each_source<F : FnMut(WireID)>(&self, mut f : F) {
        f(self.from.from);
        for path_elem in &self.path {
            match path_elem {
                RealWirePathElem::MuxArrayWrite { span:_, idx_wire } => {f(*idx_wire)}
                RealWirePathElem::ConstArrayWrite { span:_, idx:_ } => {}
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
    Select{root : WireID, path : Vec<RealWirePathElem>},
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
            RealWireDataSource::Select { root, path } => {
                f(*root, 0);
                for v in path {
                    match v {
                        RealWirePathElem::MuxArrayWrite { span:_, idx_wire } => {
                            f(*idx_wire, 0);
                        }
                        RealWirePathElem::ConstArrayWrite { span:_, idx:_ } => {}
                    }
                }
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
    pub name : String,
    /// Before latency counting, non i64::MIN values specify specified latency
    pub absolute_latency : i64,
    /// Is used to add implicit registers to wires that are used longer than one cycle. 
    /// 
    /// If needed only the same cycle it is generated, then this is equal to [RealWire::absolute_latency].
    pub needed_until : i64
}

#[derive(Debug)]
pub struct SubModule {
    pub original_flat : FlatID,
    pub instance : Rc<InstantiatedModule>,
    pub port_map : FlatAlloc<WireID, PortIDMarker>,
    pub name : String,
    pub module_uuid : ModuleUUID
}

#[derive(Debug)]
pub struct InstantiatedPort {
    pub wire : WireID,
    pub is_input : bool,
    pub absolute_latency : i64,
    pub typ : ConcreteType
}

#[derive(Debug)]
pub struct InstantiatedModule {
    /// Unique name involving all template arguments
    pub name : String,
    /// Interface is only valid if all wires of the interface were valid
    /// 
    /// Each port has a bool associated with it. It is true if 
    pub interface_ports : FlatAlloc<InstantiatedPort, PortIDMarker>,
    pub wires : FlatAlloc<RealWire, WireIDMarker>,
    pub submodules : FlatAlloc<SubModule, SubModuleIDMarker>,
    pub generation_state : FlatAlloc<SubModuleOrWire, FlatIDMarker>
}

#[derive(Debug, Clone)]
pub enum SubModuleOrWire {
    SubModule(SubModuleID),
    Wire(WireID),
    CompileTimeValue(TypedValue),
    // Variable doesn't exist yet
    Unnasigned
}

impl SubModuleOrWire {
    #[track_caller]
    pub fn unwrap_wire(&self) -> WireID {
        let Self::Wire(result) = self else {unreachable!("SubModuleOrWire::unwrap_wire failed! Is {self:?} instead")};
        *result
    }
    #[track_caller]
    pub fn unwrap_generation_value(&self) -> &TypedValue {
        let Self::CompileTimeValue(result) = self else {unreachable!("SubModuleOrWire::unwrap_generation_value failed! Is {self:?} instead")};
        result
    }
    #[track_caller]
    pub fn unwrap_submodule_instance(&self) -> SubModuleID {
        let Self::SubModule(result) = self else {unreachable!("SubModuleOrWire::unwrap_submodule_instance failed! Is {self:?} instead")};
        *result
    }
}

#[derive(Debug, Clone)]
pub enum RealWireRefRoot {
    Wire(WireID),
    Generative(FlatID),
    Constant(TypedValue)
}

#[derive(Debug)]
pub struct InstantiationList {
    cache : RefCell<Vec<(Rc<InstantiatedModule>, ErrorStore)>>
}

impl InstantiationList {
    pub fn new() -> Self {
        Self{cache : RefCell::new(Vec::new())}
    }

    pub fn instantiate(&self, name : &str, module : &Module, linker : &Linker) -> Option<Rc<InstantiatedModule>> {
        let mut cache_borrow = self.cache.borrow_mut();
        
        // Temporary, no template arguments yet
        if cache_borrow.is_empty() {
            let (result, instantiation_errors) = perform_instantiation(name, module, linker);
            cache_borrow.push((Rc::new(result), instantiation_errors));
        }
        
        let instance_id = 0; // Temporary, will always be 0 while not template arguments
        let instance = &cache_borrow[instance_id];
        if !instance.1.did_error {
            Some(instance.0.clone())
        } else {
            None
        }
    }

    pub fn for_each_error<F : FnMut(&CompileError)>(&self, func : &mut F) {
        let cache_borrow = self.cache.borrow();
        for inst in cache_borrow.deref() {
            for err in &inst.1 {
                func(err)
            }
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
