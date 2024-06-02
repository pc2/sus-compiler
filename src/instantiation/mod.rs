
mod latency_algorithm;
mod execute;
mod list_of_lists;
mod typecheck;
mod latency_count;

use std::{cell::RefCell, ops::Deref, rc::Rc};

use num::BigInt;

use crate::{
    arena_alloc::{FlatAlloc, UUIDMarker, UUID}, concrete_type::ConcreteType, config, errors::{CompileError, ErrorCollector, ErrorStore}, file_position::BracketSpan, flattening::{BinaryOperator, FlatID, FlatIDMarker, Module, PortID, PortIDMarker, UnaryOperator}, linker::{Linker, ModuleUUID}, value::{TypedValue, Value}
};

use self::latency_algorithm::SpecifiedLatency;

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
    ConstArrayWrite{span : BracketSpan, idx : BigInt}
}

impl RealWirePathElem {
    fn for_each_wire_in_path<F : FnMut(WireID)>(path : &[RealWirePathElem], mut f : F) {
        for v in path {
            match v {
                RealWirePathElem::MuxArrayWrite { span:_, idx_wire } => {
                    f(*idx_wire);
                }
                RealWirePathElem::ConstArrayWrite { span:_, idx:_ } => {}
            }
        }
    }
}


#[derive(Debug)]
pub struct MultiplexerSource {
    pub to_path : Vec<RealWirePathElem>,
    pub from : ConnectFrom
}

#[derive(Debug)]
pub enum RealWireDataSource {
    ReadOnly,
    OutPort{sub_module_id : SubModuleID, port_id : PortID},
    Multiplexer{is_state : Option<Value>, sources : Vec<MultiplexerSource>},
    UnaryOp{op : UnaryOperator, right : WireID},
    BinaryOp{op : BinaryOperator, left : WireID, right : WireID},
    Select{root : WireID, path : Vec<RealWirePathElem>},
    Constant{value : Value}
}

#[derive(Debug)]
pub struct RealWire {
    pub source : RealWireDataSource,
    pub original_instruction : FlatID,
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
    pub original_instruction : FlatID,
    pub instance : Option<Rc<InstantiatedModule>>,
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
    pub errors : ErrorStore,
    /// This matches the ports in [Module::module_ports]. Ports are not None when they are not part of this instantiation. 
    pub interface_ports : FlatAlloc<Option<InstantiatedPort>, PortIDMarker>,
    pub wires : FlatAlloc<RealWire, WireIDMarker>,
    pub submodules : FlatAlloc<SubModule, SubModuleIDMarker>,
    pub generation_state : FlatAlloc<SubModuleOrWire, FlatIDMarker>,
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

impl RealWireRefRoot {
    #[track_caller]
    pub fn unwrap_wire(&self) -> WireID {
        let Self::Wire(w) = self else {unreachable!("RealWireRefRoot::unwrap_wire")};
        *w
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

    pub fn instantiate(&self, md : &Module, linker : &Linker) -> Option<Rc<InstantiatedModule>> {
        let mut cache_borrow = self.cache.borrow_mut();
        
        // Temporary, no template arguments yet
        if cache_borrow.is_empty() {
            let result = perform_instantiation(md, linker);

            if config().debug_print_module_contents {
                println!("[[Instantiated {}]]", &result.name);
                for (id, w) in &result.wires {
                    println!("{id:?} -> {w:?}");
                }
                for (id, sm) in &result.submodules {
                    println!("SubModule {id:?}: {sm:?}");
                }
            }

            cache_borrow.push(Rc::new(result));
        }
        
        let instance_id = 0; // Temporary, will always be 0 while not template arguments
        let instance_rc = &cache_borrow[instance_id];
        if !instance_rc.errors.did_error {
            Some(instance_rc.clone())
        } else {
            None
        }
    }

    pub fn for_each_error<F : FnMut(&CompileError)>(&self, func : &mut F) {
        let cache_borrow = self.cache.borrow();
        for inst in cache_borrow.deref() {
            for err in &inst.errors {
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
            f(v.as_ref())
        }
    }
}


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

    fn instantiate_submodules(&mut self) -> bool {
        let mut success = true;
        for (_sm_id, sm) in &mut self.submodules {
            let sub_module = &self.linker.modules[sm.module_uuid];
            if let Some(instance) = sub_module.instantiations.instantiate(sub_module, self.linker) {
                for (port_id, port) in &instance.interface_ports {
                    let wire = &mut self.wires[sm.port_map[port_id]];
                    wire.typ = if let Some(instance_data) = port {
                        instance_data.typ.clone()
                    } else {
                        // TODO report ports being used when not enabled
                        ConcreteType::Error
                    };
                }
                sm.instance = Some(instance);
            } else {
                self.errors.error(self.md.instructions[sm.original_instruction].unwrap_submodule().module_name_span, "Error instantiating submodule");
                success = false;
            };
        }
        success
    }
}

fn perform_instantiation(md : &Module, linker : &Linker) -> InstantiatedModule {
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
        context.errors.set_did_error();
        return context.extract();
    }

    println!("Instantiating {}", md.link_info.name);

    if let Err(e) = context.execute_module() {
        context.errors.error(e.0, e.1);

        return context.extract();
    }



    if config().debug_print_module_contents {
        println!("[[Executed {}]]", &context.name);
        for (id, w) in &context.wires {
            println!("{id:?} -> {w:?}");
        }
        for (id, sm) in &context.submodules {
            println!("SubModule {id:?}: {sm:?}");
        }
    }


    println!("Instantiating submodules for {}", md.link_info.name);
    if !context.instantiate_submodules() {
        return context.extract();
    }

    println!("Concrete Typechecking {}", md.link_info.name);
    context.typecheck();

    println!("Latency Counting {}", md.link_info.name);
    context.compute_latencies();

    context.extract()
}
