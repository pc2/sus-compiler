mod concrete_typecheck;
mod execute;
mod final_checks;
mod unique_names;

use unique_names::UniqueNames;

use crate::prelude::*;
use crate::typing::template::TVec;
use crate::typing::type_inference::{ConcreteTypeVariableIDMarker, TypeSubstitutor};

use std::cell::OnceCell;
use std::{cell::RefCell, collections::HashMap, rc::Rc};

use crate::flattening::{BinaryOperator, Module, UnaryOperator};
use crate::{
    config,
    errors::{CompileError, ErrorStore},
    to_string::pretty_print_concrete_instance,
    value::Value,
};

use crate::typing::concrete_type::ConcreteType;

// Temporary value before proper latency is given
pub const CALCULATE_LATENCY_LATER: i64 = i64::MIN;

/// See [MultiplexerSource]
///
/// This is the post-instantiation equivalent of [crate::flattening::WireReferencePathElement]
#[derive(Debug, Clone)]
pub enum RealWirePathElem {
    ArrayAccess { span: BracketSpan, idx_wire: WireID },
}

impl RealWirePathElem {
    pub fn for_each_wire_in_path(path: &[RealWirePathElem], mut f: impl FnMut(WireID)) {
        for v in path {
            match v {
                RealWirePathElem::ArrayAccess { span: _, idx_wire } => {
                    f(*idx_wire);
                }
            }
        }
    }
}

/// One arm of a multiplexer. Each arm has an attached condition that is also stored here.
///
/// See [RealWireDataSource::Multiplexer]
#[derive(Debug)]
pub struct MultiplexerSource {
    pub to_path: Vec<RealWirePathElem>,
    pub num_regs: i64,
    pub from: WireID,
    pub condition: Box<[ConditionStackElem]>,
    pub original_connection: FlatID,
}

/// Where a [RealWire] gets its data, be it an operator, read-only value, constant, etc.
///
/// This is the post-instantiation equivalent of [crate::flattening::ExpressionSource]
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

/// An actual instantiated wire of an [InstantiatedModule] (See [InstantiatedModule::wires])
///
/// It can have a latency count and domain. All wires have a name, either the name they were given by the user, or a generated name like _1, _13
///
/// Generated from a [crate::flattening::Expression] instruction
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

/// See [SubModule]
///
/// This represents a port of such a submodule
#[derive(Debug)]
pub struct SubModulePort {
    pub maps_to_wire: WireID,
    pub name_refs: Vec<Span>,
}

/// An actual instantiated submodule of an [InstantiatedModule] (See [InstantiatedModule::submodules])
///
/// All submodules have a name, either the name they were given by the user, or a generated name like _1, _13
///
/// When generating RTL code, one submodule object generates a single submodule instantiation
///
/// Generated from a [crate::flattening::SubModuleInstance] instruction
#[derive(Debug)]
pub struct SubModule {
    pub original_instruction: FlatID,
    pub instance: OnceCell<Rc<InstantiatedModule>>,
    pub port_map: FlatAlloc<Option<SubModulePort>, PortIDMarker>,
    pub interface_call_sites: FlatAlloc<Vec<Span>, InterfaceIDMarker>,
    pub name: String,
    pub module_uuid: ModuleUUID,
    pub template_args: TVec<ConcreteType>,
}

/// Generated from [Module::ports]
#[derive(Debug)]
pub struct InstantiatedPort {
    pub wire: WireID,
    pub is_input: bool,
    pub absolute_latency: i64,
    pub typ: ConcreteType,
    pub domain: DomainID,
}

/// [InstantiatedModule] are the final product we're trying to produce with the compiler.
/// They amount to little more than a collection of wires, multiplexers and submodules.
///
/// With the submodules, they form a tree structure, of nested [InstantiatedModule] references.
///
/// Generated when instantiating a [Module]
#[derive(Debug)]
pub struct InstantiatedModule {
    /// Unique name involving all template arguments
    pub name: String,
    /// Used in code generation. Only contains characters allowed in SV and VHDL
    pub mangled_name: String,
    pub errors: ErrorStore,
    /// This matches the ports in [Module::ports]. Ports are not `None` when they are not part of this instantiation.
    pub interface_ports: FlatAlloc<Option<InstantiatedPort>, PortIDMarker>,
    pub wires: FlatAlloc<RealWire, WireIDMarker>,
    pub submodules: FlatAlloc<SubModule, SubModuleIDMarker>,
    /// See [GenerationState]
    pub generation_state: FlatAlloc<SubModuleOrWire, FlatIDMarker>,
}

/// See [GenerationState]
#[derive(Debug, Clone)]
pub enum SubModuleOrWire {
    SubModule(SubModuleID),
    Wire(WireID),
    CompileTimeValue(Value),
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
    pub fn unwrap_generation_value(&self) -> &Value {
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

/// Stored per module [Module].
/// With this you can instantiate a module for different sets of template arguments.
/// It caches the instantiations that have been made, such that they need not be repeated.
///
/// Also, with incremental builds (#49) this will be a prime area for investigation
#[derive(Debug)]
pub struct InstantiationCache {
    cache: RefCell<HashMap<TVec<ConcreteType>, Rc<InstantiatedModule>>>,
}

impl Default for InstantiationCache {
    fn default() -> Self {
        Self::new()
    }
}

impl InstantiationCache {
    pub fn new() -> Self {
        Self {
            cache: RefCell::new(HashMap::new()),
        }
    }

    pub fn instantiate(
        &self,
        md: &Module,
        linker: &Linker,
        template_args: TVec<ConcreteType>,
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

    pub fn for_each_error(&self, func: &mut impl FnMut(&CompileError)) {
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
    pub fn for_each_instance(
        &self,
        mut f: impl FnMut(&TVec<ConcreteType>, &Rc<InstantiatedModule>),
    ) {
        let borrow = self.cache.borrow();
        for (k, v) in borrow.iter() {
            f(k, v)
        }
    }
}

/// Every [crate::flattening::Instruction] has an associated value (See [SubModuleOrWire]).
/// They are either what this local name is currently referencing (either a wire instance or a submodule instance).
/// Or in the case of Generative values, the current value in the generative variable.
#[derive(Debug)]
struct GenerationState<'fl> {
    generation_state: FlatAlloc<SubModuleOrWire, FlatIDMarker>,
    md: &'fl Module,
}

/// Runtime conditions applied to a [crate::flattening::Write]
///
/// ```sus
/// state int a
/// when x {
///   a = 3
/// } else {
///   a = 2
/// }
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ConditionStackElem {
    pub condition_wire: WireID,
    /// When this is an else-branch
    pub inverse: bool,
}

/// As with other contexts, this is the shared state we're lugging around while executing & typechecking a module.
pub struct InstantiationContext<'fl, 'l> {
    pub name: String,
    pub wires: FlatAlloc<RealWire, WireIDMarker>,
    pub submodules: FlatAlloc<SubModule, SubModuleIDMarker>,

    pub type_substitutor: TypeSubstitutor<ConcreteType, ConcreteTypeVariableIDMarker>,

    /// Used for Execution
    generation_state: GenerationState<'fl>,
    unique_name_producer: UniqueNames,
    condition_stack: Vec<ConditionStackElem>,

    pub interface_ports: FlatAlloc<Option<InstantiatedPort>, PortIDMarker>,
    pub errors: ErrorCollector<'l>,

    pub template_args: &'fl TVec<ConcreteType>,
    pub md: &'fl Module,
    pub linker: &'l Linker,
}

/// Mangle the module name for use in code generation
fn mangle_name(str: &str) -> String {
    let mut result = String::with_capacity(str.len());
    for c in str.chars() {
        if c.is_whitespace() || c == ':' {
            continue;
        }
        result.push(if c.is_alphanumeric() { c } else { '_' });
    }
    result.trim_matches('_').to_owned()
}

impl InstantiationContext<'_, '_> {
    fn extract(self) -> InstantiatedModule {
        InstantiatedModule {
            mangled_name: mangle_name(&self.name),
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
    template_args: &TVec<ConcreteType>,
) -> InstantiatedModule {
    let mut context = InstantiationContext {
        name: pretty_print_concrete_instance(&md.link_info, template_args, &linker.whole_types),
        generation_state: GenerationState {
            md,
            generation_state: md
                .link_info
                .instructions
                .map(|(_, _)| SubModuleOrWire::Unnasigned),
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

    println!("Checking array accesses {}", md.link_info.name);
    context.check_array_accesses();

    context.extract()
}
