mod execute;
mod latency_algorithm;
mod latency_count;
mod list_of_lists;
mod concrete_typecheck;
mod unique_names;

use unique_names::UniqueNames;

use crate::prelude::*;
use crate::typing::type_inference::{ConcreteTypeVariableIDMarker, TypeSubstitutor};

use std::cell::OnceCell;
use std::{cell::RefCell, collections::HashMap, rc::Rc};

use crate::flattening::{BinaryOperator, Module, UnaryOperator};
use crate::{
    config,
    errors::{CompileError, ErrorStore},
    to_string::pretty_print_concrete_instance,
    value::{TypedValue, Value},
};

use crate::typing::{
    concrete_type::ConcreteType,
    template::ConcreteTemplateArgs,
};

use self::latency_algorithm::SpecifiedLatency;

// Temporary value before proper latency is given
pub const CALCULATE_LATENCY_LATER: i64 = i64::MIN;

#[derive(Debug)]
pub struct ConnectFrom {
    pub num_regs: i64,
    pub from: WireID,
    pub condition: Box<[ConditionStackElem]>,
    pub original_connection: FlatID,
}

#[derive(Debug, Clone)]
pub enum RealWirePathElem {
    ArrayAccess { span: BracketSpan, idx_wire: WireID },
}

impl RealWirePathElem {
    fn for_each_wire_in_path<F: FnMut(WireID)>(path: &[RealWirePathElem], mut f: F) {
        for v in path {
            match v {
                RealWirePathElem::ArrayAccess { span: _, idx_wire } => {
                    f(*idx_wire);
                }
            }
        }
    }
}

#[derive(Debug)]
pub struct MultiplexerSource {
    pub to_path: Vec<RealWirePathElem>,
    pub from: ConnectFrom,
}

#[derive(Debug)]
pub enum RealWireDataSource {
    ReadOnly,
    Multiplexer {
        is_state: Option<Value>,
        sources: Vec<MultiplexerSource>,
    },
    UnaryOp {
        op: UnaryOperator,
        right: WireID,
    },
    BinaryOp {
        op: BinaryOperator,
        left: WireID,
        right: WireID,
    },
    Select {
        root: WireID,
        path: Vec<RealWirePathElem>,
    },
    Constant {
        value: Value,
    },
}

#[derive(Debug)]
pub struct RealWire {
    pub source: RealWireDataSource,
    /// If it's a port of a module, then this must be the submodule
    pub original_instruction: FlatID,
    pub typ: ConcreteType,
    pub name: String,
    pub domain: DomainID,
    /// non i64::MIN values specify specified latency
    pub specified_latency: i64,
    /// The computed latencies after latency counting
    pub absolute_latency: i64,
}

#[derive(Debug)]
pub struct UsedPort {
    pub maps_to_wire: WireID,
    pub name_refs: Vec<Span>,
}

#[derive(Debug)]
pub struct SubModule {
    pub original_instruction: FlatID,
    pub instance: OnceCell<Rc<InstantiatedModule>>,
    pub port_map: FlatAlloc<Option<UsedPort>, PortIDMarker>,
    pub interface_call_sites: FlatAlloc<Vec<Span>, InterfaceIDMarker>,
    pub name: String,
    pub module_uuid: ModuleUUID,
    pub template_args: ConcreteTemplateArgs,
}

#[derive(Debug)]
pub struct InstantiatedPort {
    pub wire: WireID,
    pub is_input: bool,
    pub absolute_latency: i64,
    pub typ: ConcreteType,
    pub domain: DomainID,
}

#[derive(Debug)]
pub struct InstantiatedModule {
    /// Unique name involving all template arguments
    pub name: String,
    pub errors: ErrorStore,
    /// This matches the ports in [Module::module_ports]. Ports are not None when they are not part of this instantiation.
    pub interface_ports: FlatAlloc<Option<InstantiatedPort>, PortIDMarker>,
    pub wires: FlatAlloc<RealWire, WireIDMarker>,
    pub submodules: FlatAlloc<SubModule, SubModuleIDMarker>,
    pub generation_state: FlatAlloc<SubModuleOrWire, FlatIDMarker>,
}

#[derive(Debug, Clone)]
pub enum SubModuleOrWire {
    SubModule(SubModuleID),
    Wire(WireID),
    CompileTimeValue(TypedValue),
    // Variable doesn't exist yet
    Unnasigned,
}

impl SubModuleOrWire {
    #[track_caller]
    pub fn unwrap_wire(&self) -> WireID {
        let Self::Wire(result) = self else {
            unreachable!("SubModuleOrWire::unwrap_wire failed! Is {self:?} instead")
        };
        *result
    }
    #[track_caller]
    pub fn unwrap_generation_value(&self) -> &TypedValue {
        let Self::CompileTimeValue(result) = self else {
            unreachable!("SubModuleOrWire::unwrap_generation_value failed! Is {self:?} instead")
        };
        result
    }
    #[track_caller]
    pub fn unwrap_submodule_instance(&self) -> SubModuleID {
        let Self::SubModule(result) = self else {
            unreachable!("SubModuleOrWire::unwrap_submodule_instance failed! Is {self:?} instead")
        };
        *result
    }
}

#[derive(Debug, Clone)]
pub enum RealWireRefRoot {
    /// The preamble isn't really used yet, but it's there for when we have submodule arrays (soon)
    Wire {
        wire_id: WireID,
        preamble: Vec<RealWirePathElem>,
    },
    Generative(FlatID),
    Constant(TypedValue),
}

#[derive(Debug)]
pub struct InstantiationList {
    cache: RefCell<HashMap<ConcreteTemplateArgs, Rc<InstantiatedModule>>>,
}

impl InstantiationList {
    pub fn new() -> Self {
        Self {
            cache: RefCell::new(HashMap::new()),
        }
    }

    pub fn instantiate(
        &self,
        md: &Module,
        linker: &Linker,
        template_args: ConcreteTemplateArgs,
    ) -> Option<Rc<InstantiatedModule>> {
        let cache_borrow = self.cache.borrow();

        // Temporary, no template arguments yet
        let instance = if let Some(found) = cache_borrow.get(&template_args) {
            found.clone()
        } else {
            std::mem::drop(cache_borrow);

            let result = perform_instantiation(md, linker, &template_args);

            if config().should_print_for_debug(config().debug_print_module_contents, &result.name) {
                println!("[[Instantiated {}]]", result.name);
                for (id, w) in &result.wires {
                    println!("{id:?} -> {w:?}");
                }
                for (id, sm) in &result.submodules {
                    println!("SubModule {id:?}: {sm:?}");
                }
            }

            let result_ref = Rc::new(result);
            assert!(self
                .cache
                .borrow_mut()
                .insert(template_args, result_ref.clone())
                .is_none());
            result_ref
        };

        if !instance.errors.did_error {
            Some(instance.clone())
        } else {
            None
        }
    }

    pub fn for_each_error<F: FnMut(&CompileError)>(&self, func: &mut F) {
        let cache_borrow = self.cache.borrow();
        for inst in cache_borrow.values() {
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
    pub fn for_each_instance<'s, F: FnMut(&ConcreteTemplateArgs, &Rc<InstantiatedModule>)>(
        &self,
        mut f: F,
    ) {
        let borrow = self.cache.borrow();
        for (k, v) in borrow.iter() {
            f(k, &v)
        }
    }
}

#[derive(Debug)]
struct GenerationState<'fl> {
    generation_state: FlatAlloc<SubModuleOrWire, FlatIDMarker>,
    md: &'fl Module,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ConditionStackElem {
    pub condition_wire : WireID,
    pub inverse : bool
}

struct InstantiationContext<'fl, 'l> {
    name: String,
    generation_state: GenerationState<'fl>,
    wires: FlatAlloc<RealWire, WireIDMarker>,
    submodules: FlatAlloc<SubModule, SubModuleIDMarker>,

    type_substitutor: TypeSubstitutor<ConcreteType, ConcreteTypeVariableIDMarker>,

    // Used for Execution
    unique_name_producer: UniqueNames,
    condition_stack : Vec<ConditionStackElem>,

    interface_ports: FlatAlloc<Option<InstantiatedPort>, PortIDMarker>,
    errors: ErrorCollector<'l>,

    template_args: &'fl ConcreteTemplateArgs,
    md: &'fl Module,
    linker: &'l Linker,
}

impl<'fl, 'l> InstantiationContext<'fl, 'l> {
    fn extract(self) -> InstantiatedModule {
        InstantiatedModule {
            name: self.name,
            wires: self.wires,
            submodules: self.submodules,
            interface_ports: self.interface_ports,
            generation_state: self.generation_state.generation_state,
            errors: self.errors.into_storage(),
        }
    }
}

fn perform_instantiation(
    md: &Module,
    linker: &Linker,
    template_args: &ConcreteTemplateArgs,
) -> InstantiatedModule {
    let mut context = InstantiationContext {
        name: pretty_print_concrete_instance(&md.link_info, template_args, &linker.types),
        generation_state: GenerationState {
            md,
            generation_state: md.link_info.instructions.map(|(_, _)| SubModuleOrWire::Unnasigned),
        },
        type_substitutor: TypeSubstitutor::new(),
        condition_stack: Vec::new(),
        wires: FlatAlloc::new(),
        submodules: FlatAlloc::new(),
        interface_ports: md.ports.map(|_| None),
        errors: ErrorCollector::new_empty(md.link_info.file, &linker.files),
        unique_name_producer: UniqueNames::new(),
        template_args,
        md,
        linker,
    };

    // Don't instantiate modules that already errored. Otherwise instantiator may crash
    if md.link_info.errors.did_error {
        println!(
            "Not Instantiating {} due to flattening errors",
            md.link_info.name
        );
        context.errors.set_did_error();
        return context.extract();
    }

    println!("Instantiating {}", md.link_info.name);

    if let Err(e) = context.execute_module() {
        context.errors.error(e.0, e.1);

        return context.extract();
    }

    if config().should_print_for_debug(config().debug_print_module_contents, &context.name) {
        println!("[[Executed {}]]", &context.name);
        for (id, w) in &context.wires {
            println!("{id:?} -> {w:?}");
        }
        for (id, sm) in &context.submodules {
            println!("SubModule {id:?}: {sm:?}");
        }
    }

    println!("Concrete Typechecking {}", md.link_info.name);
    context.typecheck();

    println!("Latency Counting {}", md.link_info.name);
    context.compute_latencies();

    context.extract()
}
