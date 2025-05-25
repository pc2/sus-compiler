mod concrete_typecheck;
mod execute;
mod final_checks;
pub mod instantiation_cache;
mod unique_names;

use unique_names::UniqueNames;

use crate::latency::CALCULATE_LATENCY_LATER;
use crate::linker::LinkInfo;
use crate::prelude::*;
use crate::typing::value_unifier::{UnifyableValue, UnifyableValueAlloc};

use std::cell::OnceCell;
use std::rc::Rc;

use crate::flattening::{BinaryOperator, Module, Port, UnaryOperator};
use crate::{errors::ErrorStore, value::Value};

use crate::typing::concrete_type::{ConcreteGlobalReference, ConcreteType};

/// See [MultiplexerSource]
///
/// This is the post-instantiation equivalent of [crate::flattening::WireReferencePathElement]
#[derive(Debug, Clone)]
pub enum RealWirePathElem {
    ArrayAccess { span: BracketSpan, idx_wire: WireID },
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
        rank: Vec<UnifyableValue>,
        right: WireID,
    },
    BinaryOp {
        op: BinaryOperator,
        rank: Vec<UnifyableValue>,
        left: WireID,
        right: WireID,
    },
    Select {
        root: WireID,
        path: Vec<RealWirePathElem>,
    },
    ConstructArray {
        array_wires: Vec<WireID>,
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
impl RealWire {
    fn get_span(&self, link_info: &LinkInfo) -> Span {
        link_info.instructions[self.original_instruction].get_span()
    }
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
    pub refers_to: Rc<ConcreteGlobalReference<ModuleUUID>>,
    pub port_map: FlatAlloc<Option<SubModulePort>, PortIDMarker>,
    pub interface_call_sites: FlatAlloc<Vec<Span>, InterfaceIDMarker>,
    pub name: String,
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
    pub global_ref: Rc<ConcreteGlobalReference<ModuleUUID>>,
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

/// Iteration of contained [WireID]s
pub trait ForEachContainedWire {
    fn for_each_wire(&self, f: &mut impl FnMut(WireID));
}

impl<E: ForEachContainedWire> ForEachContainedWire for [E] {
    fn for_each_wire(&self, f: &mut impl FnMut(WireID)) {
        for e in self {
            e.for_each_wire(f);
        }
    }
}

impl ForEachContainedWire for WireID {
    fn for_each_wire(&self, f: &mut impl FnMut(WireID)) {
        f(*self)
    }
}

impl ForEachContainedWire for RealWirePathElem {
    fn for_each_wire(&self, f: &mut impl FnMut(WireID)) {
        match self {
            RealWirePathElem::ArrayAccess { span: _, idx_wire } => {
                f(*idx_wire);
            }
        }
    }
}

impl ForEachContainedWire for ConditionStackElem {
    fn for_each_wire(&self, f: &mut impl FnMut(WireID)) {
        f(self.condition_wire);
    }
}

impl ForEachContainedWire for MultiplexerSource {
    fn for_each_wire(&self, f: &mut impl FnMut(WireID)) {
        self.to_path.for_each_wire(f);
        self.condition.for_each_wire(f);
        f(self.from);
    }
}

impl ForEachContainedWire for RealWireDataSource {
    fn for_each_wire(&self, f: &mut impl FnMut(WireID)) {
        match self {
            RealWireDataSource::ReadOnly => {}
            RealWireDataSource::Multiplexer { sources, .. } => {
                sources.for_each_wire(f);
            }
            RealWireDataSource::UnaryOp { right, .. } => f(*right),
            RealWireDataSource::BinaryOp { left, right, .. } => {
                f(*left);
                f(*right)
            }
            RealWireDataSource::Select { root, path } => {
                f(*root);
                path.for_each_wire(f);
            }
            RealWireDataSource::ConstructArray { array_wires } => {
                array_wires.for_each_wire(f);
            }
            RealWireDataSource::Constant { value: _ } => {}
        }
    }
}

struct Executed {
    wires: FlatAlloc<RealWire, WireIDMarker>,
    submodules: FlatAlloc<SubModule, SubModuleIDMarker>,
    type_substitutor: UnifyableValueAlloc,
    generation_state: FlatAlloc<SubModuleOrWire, FlatIDMarker>,
    execution_status: Result<(), (Span, String)>,
}

struct Typechecked {
    wires: FlatAlloc<RealWire, WireIDMarker>,
    submodules: FlatAlloc<SubModule, SubModuleIDMarker>,
    generation_state: FlatAlloc<SubModuleOrWire, FlatIDMarker>,
    errors: ErrorStore,
}

impl Typechecked {
    fn make_interface(
        &self,
        ports: &FlatAlloc<Port, PortIDMarker>,
    ) -> FlatAlloc<Option<InstantiatedPort>, PortIDMarker> {
        ports.map(|(_, port)| {
            let port_decl_id = port.declaration_instruction;
            if let SubModuleOrWire::Wire(wire_id) = &self.generation_state[port_decl_id] {
                let wire = &self.wires[*wire_id];
                Some(InstantiatedPort {
                    wire: *wire_id,
                    is_input: port.is_input,
                    absolute_latency: CALCULATE_LATENCY_LATER,
                    typ: wire.typ.clone(),
                    domain: wire.domain,
                })
            } else {
                None
            }
        })
    }

    fn to_instantiated_module(
        self,
        md: &Module,
        linker: &Linker,
        global_ref: Rc<ConcreteGlobalReference<ModuleUUID>>,
    ) -> InstantiatedModule {
        let interface_ports = self.make_interface(&md.ports);
        let name = global_ref.display(linker, false).to_string();
        let mangled_name = mangle_name(&name);
        InstantiatedModule {
            global_ref,
            name,
            mangled_name,
            errors: self.errors,
            interface_ports,
            wires: self.wires,
            submodules: self.submodules,
            generation_state: self.generation_state,
        }
    }
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
