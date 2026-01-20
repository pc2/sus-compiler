use crate::prelude::*;
use crate::typing::unifyable_cell::UniCell;

mod builtins;
mod concrete_typecheck;
mod execute;
mod final_checks;
mod instantiator;
mod unique_names;

pub use instantiator::Instantiator;

use ibig::IBig;
use unique_names::UniqueNames;

use crate::{
    errors::ErrorStore,
    flattening::{
        BinaryOperator, Direction, Expression, ExpressionSource, GlobalReference, Instruction,
        Module, PartSelectDirection, UnaryOperator, WireReference, WireReferenceRoot,
    },
    instantiation::concrete_typecheck::ModuleTypingSuperContext,
    latency::{AbsLat, InferenceFailure},
    linker::{LinkInfo, LinkerGlobals},
    typing::{
        concrete_type::{ConcreteGlobalReference, ConcreteType, IntBounds},
        template::TVec,
    },
    value::Value,
};

use std::cell::{OnceCell, RefCell};
use std::collections::HashSet;
use std::fmt::Write;
use std::rc::Rc;

/// In valid programs, this becomes [PartialBound::Known] after concrete typecheck
#[derive(Debug, Clone)]
pub enum PartialBound {
    Known(IBig, IBig),
    From(IBig),
    To(IBig),
    WholeSlice,
}

impl PartialBound {
    pub fn unwrap_valid(&self) -> IntBounds<&IBig> {
        let_unwrap!(Self::Known(from, to), self);
        IntBounds { from, to }
    }
    pub fn unwrap_width(&self) -> IBig {
        let_unwrap!(Self::Known(from, to), self);
        to - from
    }
}

/// See [MultiplexerSource]
///
/// This is the post-instantiation equivalent of [crate::flattening::WireReferencePathElement]
#[derive(Debug, Clone)]
pub enum RealWirePathElem {
    Index {
        span: BracketSpan,
        idx_wire: WireID,
    },
    ConstIndex {
        span: BracketSpan,
        idx: IBig,
    },
    PartSelect {
        span: BracketSpan,
        from_wire: WireID,
        width: IBig,
        direction: PartSelectDirection,
    },
    Slice {
        span: BracketSpan,
        bounds: PartialBound,
    },
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
    pub write_span: Span,
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
        rank: Vec<UniCell<Value>>,
        right: WireID,
    },
    BinaryOp {
        op: BinaryOperator,
        rank: Vec<UniCell<Value>>,
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IsPort {
    PlainWire,
    Port(PortID, Direction),
    SubmodulePort(SubModuleID, PortID, Direction),
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
    pub specified_latency: AbsLat,
    /// The computed latencies after latency counting
    pub absolute_latency: AbsLat,
    pub is_port: IsPort,
}
impl RealWire {
    pub fn get_span(&self, link_info: &LinkInfo) -> Span {
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
    pub refers_to: ConcreteGlobalReference<ModuleUUID>,
    pub last_infer_values: RefCell<TVec<Vec<InferenceResult>>>,
    pub port_map: FlatAlloc<Option<SubModulePort>, PortIDMarker>,
    pub interface_call_sites: FlatAlloc<Vec<Span>, InterfaceIDMarker>,
    pub name: String,
}
impl SubModule {
    fn get_span(&self, link_info: &LinkInfo) -> Span {
        match &link_info.instructions[self.original_instruction] {
            Instruction::SubModule(sub_module_instance) => sub_module_instance.name_span,
            Instruction::Expression(Expression {
                source:
                    ExpressionSource::WireRef(WireReference {
                        root: WireReferenceRoot::NamedModule(_),
                        ..
                    }),
                span,
                ..
            }) => *span,
            _ => unreachable!(),
        }
    }
    fn get_original_global_ref<'linker>(
        &self,
        instructions: &'linker FlatAlloc<Instruction, FlatIDMarker>,
    ) -> &'linker GlobalReference<ModuleUUID> {
        match &instructions[self.original_instruction] {
            Instruction::SubModule(sm) => &sm.module_ref,
            Instruction::Expression(Expression {
                source:
                    ExpressionSource::WireRef(WireReference {
                        root: WireReferenceRoot::NamedModule(md_ref),
                        ..
                    }),
                ..
            }) => md_ref,
            _ => unreachable!(),
        }
    }
}

/// Generated from [Module::ports]
#[derive(Debug)]
pub struct InstantiatedPort {
    pub wire: WireID,
    pub direction: Direction,
    pub absolute_latency: AbsLat,
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
    Unassigned,
}

impl SubModuleOrWire {
    #[track_caller]
    pub fn unwrap_wire(&self) -> WireID {
        let Self::Wire(result) = self else {
            unreachable!("SubModuleOrWire::unwrap_wire failed! Is {self:?} instead")
        };
        *result
    }
    #[allow(unused)]
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
            RealWirePathElem::Index { span: _, idx_wire } => {
                f(*idx_wire);
            }
            RealWirePathElem::PartSelect { from_wire, .. } => {
                f(*from_wire);
            }
            RealWirePathElem::Slice { .. } | RealWirePathElem::ConstIndex { .. } => {}
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

#[derive(Debug, Clone)]
pub enum InferenceResult {
    /// Means the inference candidate can be discarded
    PortNotUsed,
    /// Means the port is valid, but the target couldn't be computed. Invalidates [ValueInferStrategy::Min] and [ValueInferStrategy::Max]
    NotFound,
    /// See [InferenceFailure::BadProblem]
    LatencyBadProblem,
    /// See [InferenceFailure::NotReached]
    LatencyNotReached,
    /// See [InferenceFailure::Poison]
    LatencyPoison {
        submod: SubModuleID,
        /// Input port
        port_from: PortID,
        /// Output port
        port_to: PortID,
    },
    /// Valid value! Can be used for inferring
    Found(IBig),
}

struct Executed {
    wires: FlatAlloc<RealWire, WireIDMarker>,
    submodules: FlatAlloc<SubModule, SubModuleIDMarker>,
    generation_state: FlatAlloc<SubModuleOrWire, FlatIDMarker>,
    execution_status: Result<(), (Span, String)>,
}

pub struct ModuleTypingContext<'l> {
    pub name: String,
    pub mangled_name: String,
    pub global_ref: Rc<ConcreteGlobalReference<ModuleUUID>>,
    pub wires: FlatAlloc<RealWire, WireIDMarker>,
    pub submodules: FlatAlloc<SubModule, SubModuleIDMarker>,
    pub generation_state: FlatAlloc<SubModuleOrWire, FlatIDMarker>,
    pub globals: &'l LinkerGlobals,
    /// Yes I know it's redundant, but it's easier to both have link_info and md
    pub md: &'l Module,
    pub link_info: &'l LinkInfo,
    pub errors: ErrorCollector<'l>,
}

impl<'l> ModuleTypingContext<'l> {
    fn into_instantiated_module(self) -> InstantiatedModule {
        if crate::debug::is_enabled("dot-dependency-graph") {
            crate::dev_aid::dot_graphs::display_generated_hardware_structure(&self);
        }

        // A non-error instance must be fully valid!
        if !self.errors.did_error() {
            for (_, w) in &self.wires {
                assert!(w.typ.is_valid());
                assert!(w.absolute_latency.get().is_some());
            }
            for (_, sm) in &self.submodules {
                assert!(sm.refers_to.find_invalid_template_args().is_empty());
                assert!(sm.instance.get().is_some());
            }
        }

        let interface_ports = self.md.ports.map(|(port_id, port)| {
            let port_decl_id = port.declaration_instruction;
            let SubModuleOrWire::Wire(wire_id) = &self.generation_state[port_decl_id] else {
                return None;
            };
            let wire = &self.wires[*wire_id];
            assert_eq!(wire.is_port, IsPort::Port(port_id, port.direction));
            Some(InstantiatedPort {
                wire: *wire_id,
                direction: port.direction,
                absolute_latency: wire.absolute_latency,
                domain: wire.domain,
            })
        });

        InstantiatedModule {
            global_ref: self.global_ref,
            name: self.name,
            mangled_name: self.mangled_name,
            errors: self.errors.into_storage(),
            interface_ports,
            wires: self.wires,
            submodules: self.submodules,
            generation_state: self.generation_state,
        }
    }
}
