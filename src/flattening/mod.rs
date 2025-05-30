mod flatten;
mod initialization;
mod lints;
mod name_context;
mod parser;
mod typechecking;
mod walk;

use crate::alloc::UUIDAllocator;
use crate::prelude::*;
use crate::typing::abstract_type::{AbstractRankedType, DomainType, PeanoType};
use crate::typing::written_type::WrittenType;

use std::cell::OnceCell;
use std::ops::Deref;

use crate::latency::port_latency_inference::PortLatencyInferenceInfo;
pub use flatten::flatten_all_globals;
pub use initialization::gather_initial_file_data;
pub use lints::perform_lints;
pub use typechecking::typecheck_all_modules;

use crate::linker::{Documentation, LinkInfo};
use crate::value::Value;

use crate::typing::{abstract_type::FullType, template::GlobalReference};

/// Modules are compiled in 4 stages. All modules must pass through each stage before advancing to the next stage.
///
/// 1. Initialization: initial name resolution and port discovery. The Module objects themselves are constructed.
///
/// 2. Flattening:
///
///     2.1: Parsing: Parse source code to create instruction list.
///
///     2.2: Typecheck: Add typ variables to everything. [Declaration::typ], [Expression::typ] and [SubModuleInstance::local_interface_domains] are set in this stage.
///
/// 3. Instantiation: Actually run generative code and instantiate modules.
///
///     3.1: Execution
///     
///     3.2: Concrete Typecheck, Latency Counting
///
/// All Modules are stored in [Linker::modules] and indexed by [ModuleUUID]
#[derive(Debug)]
pub struct Module {
    /// Created in Stage 1: Initialization
    pub link_info: LinkInfo,

    /// Created in Stage 1: Initialization
    ///
    /// [Port::declaration_instruction] are set in Stage 2: Flattening
    ///
    /// Ports can only use domains in [Self::named_domains]
    pub ports: FlatAlloc<Port, PortIDMarker>,

    /// Created in Stage 2: Flattening
    pub latency_inference_info: PortLatencyInferenceInfo,

    /// Created in Stage 1: Initialization
    pub domains: FlatAlloc<DomainInfo, DomainIDMarker>,

    /// Created in Stage 1: Initialization
    pub interfaces: FlatAlloc<Interface, InterfaceIDMarker>,
}

impl Module {
    pub fn get_main_interface(&self) -> Option<(InterfaceID, &Interface)> {
        self.interfaces
            .iter()
            .find(|(_, interf)| interf.name == self.link_info.name)
    }

    pub fn get_port_decl(&self, port: PortID) -> &Declaration {
        let flat_port = self.ports[port].declaration_instruction;

        self.link_info.instructions[flat_port].unwrap_declaration()
    }

    /// Get a port by the given name. Reports non existing ports errors
    ///
    /// Prefer interfaces over ports in name conflicts
    pub fn get_port_or_interface_by_name(
        &self,
        name_span: Span,
        name: &str,
        errors: &ErrorCollector,
    ) -> Option<PortOrInterface> {
        for (id, data) in &self.interfaces {
            if data.name == name {
                return Some(PortOrInterface::Interface(id));
            }
        }
        for (id, data) in &self.ports {
            if data.name == name {
                return Some(PortOrInterface::Port(id));
            }
        }
        errors
            .error(
                name_span,
                format!(
                    "There is no port or interface of name '{name}' on module {}",
                    self.link_info.name
                ),
            )
            .info_obj(self);
        None
    }

    /// Temporary upgrade such that we can name the singular clock of the module, such that weirdly-named external module clocks can be used
    ///
    /// See #7
    pub fn get_clock_name(&self) -> &str {
        &self.domains.iter().next().unwrap().1.name
    }
}

impl LinkInfo {
    pub fn get_instruction_span(&self, instr_id: FlatID) -> Span {
        match &self.instructions[instr_id] {
            Instruction::SubModule(sm) => sm.module_ref.get_total_span(),
            Instruction::Declaration(decl) => decl.decl_span,
            Instruction::Expression(w) => w.span,
            Instruction::IfStatement(if_stmt) => self.get_instruction_span(if_stmt.condition),
            Instruction::ForStatement(for_stmt) => {
                self.get_instruction_span(for_stmt.loop_var_decl)
            }
        }
    }
}

/// Represents an opaque type in the compiler, like `int` or `bool`.
///
/// TODO: Structs #8
///
/// All Types are stored in [Linker::types] and indexed by [TypeUUID]
#[derive(Debug)]
pub struct StructType {
    /// Created in Stage 1: Initialization
    pub link_info: LinkInfo,

    /// Created in Stage 1: Initialization
    ///
    /// [StructField::declaration_instruction] are set in Stage 2: Flattening
    fields: FlatAlloc<StructField, FieldIDMarker>,
}

/// Global constant, like `true`, `false`, or user-defined constants (TODO #19)
///
/// All Constants are stored in [Linker::constants] and indexed by [ConstantUUID]
#[derive(Debug)]
pub struct NamedConstant {
    pub link_info: LinkInfo,
    pub output_decl: FlatID,
}

/// Represents a field in a struct
///
/// UNFINISHED
///
/// TODO: Structs #8
#[derive(Debug)]
pub struct StructField {
    #[allow(unused)]
    pub name: String,
    pub name_span: Span,
    #[allow(unused)]
    pub decl_span: Span,
    /// This is only set after flattening is done. Initially just [UUID::PLACEHOLDER]
    pub declaration_instruction: FlatID,
}

/// In SUS, when you write `my_submodule.abc`, `abc` could refer to an interface or to a port.
///
/// See [Module::get_port_or_interface_by_name]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PortOrInterface {
    Port(PortID),
    Interface(InterfaceID),
}

/// Information about a (clock) domain.
///
/// Right now this only contains the domain name, but when actual clock domains are implemented (#7),
/// this will contain information about the Clock.
#[derive(Debug, Clone)]
pub struct DomainInfo {
    pub name: String,
    /// May be [None] for the default `clk` domain
    pub name_span: Option<Span>,
}

/// With this struct, we convert the domains of a submodule, to their connecting domains in the containing module
#[derive(Clone, Copy)]
pub struct InterfaceToDomainMap<'linker> {
    pub local_domain_map: &'linker FlatAlloc<DomainType, DomainIDMarker>,
    pub domains: &'linker FlatAlloc<DomainInfo, DomainIDMarker>,
}

/// What kind of wire/value does this identifier represent?
///
/// We already know it's not a submodule
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IdentifierType {
    /// Local temporary
    /// ```sus
    /// int v = val
    /// ```
    Local,
    /// ```sus
    /// state int v
    /// ```
    State,
    /// ```sus
    /// gen int V = 3
    /// ```
    Generative,
}

impl IdentifierType {
    pub fn get_keyword(&self) -> &'static str {
        match self {
            IdentifierType::Local => "",
            IdentifierType::State => "state",
            IdentifierType::Generative => "gen",
        }
    }
    pub fn is_generative(&self) -> bool {
        *self == IdentifierType::Generative
    }
}

/// A port of a module. Not to be confused with [PortReference], which is a reference to a submodule port.
///
/// All ports must have a name
///
/// ```sus
/// module md {
///     interface beep : int a -> bool b, int[3] c
///
///     output int d
/// }
/// ```
///
/// Creates four ports: a, b, c, and d.
///
/// Ports can be part of interfaces, as is the case above, or are standalone, like d
///
/// ```sus
/// module md {
///     interface beep : int a -> bool b, int[3] c
/// }
/// ```
#[derive(Debug)]
pub struct Port {
    pub name: String,
    pub name_span: Span,
    pub decl_span: Span,
    pub is_input: bool,
    pub domain: DomainID,
    /// This is only set after flattening is done. Initially just [crate::alloc::UUID::PLACEHOLDER]
    pub declaration_instruction: FlatID,
}

/// An interface, like:
///
/// ```sus
/// module md {
///     interface beep : int a -> bool b, int[3] c
/// }
/// ```
///
/// So this struct represents an interface, which always can be called with a method-call notation:
///
/// ```sus
/// module use_md {
///     md x
///
///     bool xyz, int[3] pqr = x.beep(3)
/// }
/// ```
#[derive(Debug)]
pub struct Interface {
    pub name_span: Span,
    pub name: String,
    /// All the interface's ports have this domain too
    pub domain: DomainID,
    pub func_call_inputs: PortIDRange,
    pub func_call_outputs: PortIDRange,
}

impl Interface {
    pub fn all_ports(&self) -> PortIDRange {
        assert_eq!(self.func_call_inputs.1, self.func_call_outputs.0);
        PortIDRange::new(self.func_call_inputs.0, self.func_call_outputs.1)
    }
}

/// An element in a [WireReference] path. Could be array accesses, slice accesses, field accesses, etc
///
/// When executing, this turns into [crate::instantiation::RealWirePathElem]
#[derive(Debug, Clone)]
pub enum WireReferencePathElement {
    ArrayAccess {
        idx: FlatID,
        bracket_span: BracketSpan,
        output_typ: AbstractRankedType,
    },
}

/// The root of a [WireReference]. Basically where the wire reference starts.
///
/// This can be a local declaration, a global constant, the port of a submodule.
#[derive(Debug)]
pub enum WireReferenceRoot {
    /// ```sus
    /// int local_var
    /// local_var = 3
    /// ```
    ///
    /// [FlatID] points to [Instruction::Declaration]
    LocalDecl(FlatID),
    /// ```sus
    /// bool b = true // root is global constant `true`
    /// ```
    NamedConstant(GlobalReference<ConstantUUID>),
    /// ```sus
    /// FIFO local_submodule
    /// local_submodule.data_in = 3 // root is local_submodule.data_in (yes, both)
    /// ```
    SubModulePort(PortReference),
    /// Used to conveniently represent errors
    Error,
}

impl WireReferenceRoot {
    pub fn get_root_flat(&self) -> Option<FlatID> {
        match self {
            WireReferenceRoot::LocalDecl(f) => Some(*f),
            WireReferenceRoot::NamedConstant(_) => None,
            WireReferenceRoot::SubModulePort(port) => Some(port.submodule_decl),
            WireReferenceRoot::Error => None,
        }
    }
    #[track_caller]
    pub fn unwrap_local_decl(&self) -> FlatID {
        let Self::LocalDecl(decl) = self else {
            unreachable!()
        };
        *decl
    }
}

/// References to wires or generative variables.
///
/// Basically, this struct encapsulates all expressions that can be written to, like field and array accesses.
///
/// [Expression] covers anything that can not be written to.
///
/// Example: `myModule.port[a][b:c]`. (`myModule.port` is the [Self::root], `[a][b:c]` are two parts of the [Self::path])
#[derive(Debug)]
pub struct WireReference {
    pub root: WireReferenceRoot,
    pub path: Vec<WireReferencePathElement>,
    pub root_typ: FullType,
    pub root_span: Span,
}

impl WireReference {
    pub fn is_error(&self) -> bool {
        matches!(&self.root, WireReferenceRoot::Error)
    }
    pub fn get_output_typ(&self) -> &AbstractRankedType {
        if let Some(last) = self.path.last() {
            match last {
                WireReferencePathElement::ArrayAccess { output_typ, .. } => output_typ,
            }
        } else {
            &self.root_typ.typ
        }
    }
}

/// In a [Write], this represents what kind of write it is, based on keywords `reg` or `initial`
#[derive(Debug)]
pub enum WriteModifiers {
    /// A regular write to a local wire (can include latency registers) or generative variable
    /// ```sus
    /// int v
    /// reg reg v = a * 3
    /// ```
    Connection { num_regs: i64, regs_span: Span },
    /// Set the initial value of a `state` register
    /// ```sus
    /// state int count
    /// initial count = 3 // Must be generative
    /// ```
    Initial { initial_kw_span: Span },
}

impl WriteModifiers {
    pub fn requires_generative(&self) -> bool {
        match self {
            Self::Connection { .. } => false,
            Self::Initial { .. } => true,
        }
    }
}

/// A part of [Expression] that refers to an assignment
///
/// ```sus
/// module md {
///     int x = 3 // first write
///
///     int b, int c = someFunc(3) // Two writes, one to b, one to c
/// }
/// ```
#[derive(Debug)]
pub struct WriteTo {
    /// Invalid [WireReference] is possible.
    pub to: WireReference,
    pub to_span: Span,
    pub write_modifiers: WriteModifiers,
}

/// -x
///
/// See [crate::value::compute_unary_op]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UnaryOperator {
    /// Horizontal & on arrays
    And,
    /// Horizontal | on arrays
    Or,
    /// Horizontal ^ on arrays
    Xor,
    /// ! on booleans
    Not,
    /// Horizontal + on arrays
    Sum,
    /// Horizontal * on arrays
    Product,
    /// - on integers
    Negate,
}

/// x * y
///
/// See [crate::value::compute_binary_op]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BinaryOperator {
    And,
    Or,
    Xor,
    Add,
    //ShiftLeft,
    //ShiftRight,
    Subtract,
    Multiply,
    Divide,
    Modulo,
    Equals,
    NotEquals,
    Greater,
    GreaterEq,
    Lesser,
    LesserEq,
}

/// A reference to a port within a submodule.
/// Not to be confused with [Port], which is the declaration of the port itself in the [Module]
#[derive(Debug, Clone, Copy)]
pub struct PortReference {
    pub submodule_decl: FlatID,
    pub port: PortID,
    pub is_input: bool,
    /// Only set if the port is named as an explicit field. If the port name is implicit, such as in the function call syntax, then it is not present.
    pub port_name_span: Option<Span>,
    /// Even this can be implicit. In the inline function call instantiation syntax there's no named submodule. my_mod(a, b, c)
    ///
    /// Finally, if [Self::port_name_span].is_none(), then for highlighting and renaming, this points to a duplicate of a Function Call
    pub submodule_name_span: Option<Span>,
}

/// See [Expression]
#[derive(Debug)]
pub enum ExpressionSource {
    WireRef(WireReference), // Used to add a span to the reference of a wire.
    FuncCall(FuncCall),
    UnaryOp {
        op: UnaryOperator,
        /// Operators automatically parallelize across arrays
        rank: PeanoType,
        right: FlatID,
    },
    BinaryOp {
        op: BinaryOperator,
        /// Operators automatically parallelize across arrays
        rank: PeanoType,
        left: FlatID,
        right: FlatID,
    },
    ArrayConstruct(Vec<FlatID>),
    Constant(Value),
}
/// [FuncCall]s (and potentially, in the future, other things) can have multiple outputs.
/// We make the distinction between [SubExpression] that can only represent one output, and [MultiWrite], which can represent multiple outputs.
/// Workarounds like putting multiple outputs together in a tuple would not work, because:
/// - The function call syntax is just a convenient syntax sugar for connecting multiple inputs and outputs simultaneously.
///     We want to conceptually keep the signals separate. Both input and output signals, while keeping the function call syntax that programmers are used to.
/// - Forcing all outputs together into one type would bind them together for latency counting, which we don't want
/// - We refuse to have tuple types
#[derive(Debug)]
pub enum ExpressionOutput {
    SubExpression(AbstractRankedType),
    MultiWrite(Vec<WriteTo>),
}
/// An [Instruction] that represents a single expression in the program. Like ((3) + (x))
///
/// See [ExpressionSource]
///
/// On instantiation, creates [crate::instantiation::RealWire] when non-generative
#[derive(Debug)]
pub struct Expression {
    pub span: Span,
    pub source: ExpressionSource,
    /// Means [Self::source] can be computed at compiletime, not that [Self::output] neccesarily requires a generative result
    pub domain: DomainType,

    /// If [None], then this function returns a single result like a normal expression
    /// If Some(outputs), then this function is a dead-end expression, and does it's outputs manually
    pub output: ExpressionOutput,
}

impl Expression {
    pub fn as_single_output_expr(&self) -> Option<SingleOutputExpression> {
        let typ = match &self.output {
            ExpressionOutput::SubExpression(typ) => typ,
            ExpressionOutput::MultiWrite(write_tos) => {
                let [single_write] = write_tos.as_slice() else {
                    return None;
                };
                single_write.to.get_output_typ()
            }
        };
        Some(SingleOutputExpression {
            typ,
            domain: self.domain,
            span: self.span,
            source: &self.source,
        })
    }
    pub fn is_error(&self) -> bool {
        matches!(
            &self.source,
            ExpressionSource::WireRef(WireReference {
                root: WireReferenceRoot::Error,
                ..
            })
        )
    }
}

/// Little helper struct that tells us what kind of declaration it is.
/// Is it a Port, Template argument, A struct field, or just a regular temporary?
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DeclarationKind {
    NotPort,
    StructField { field_id: FieldID },
    RegularPort { is_input: bool, port_id: PortID },
    GenerativeInput(TemplateID),
}

impl DeclarationKind {
    /// Basically an unwrap to see if this [Declaration] refers to a [Port], and returns `Some(is_input)` if so.
    pub fn is_io_port(&self) -> Option<bool> {
        if let DeclarationKind::RegularPort {
            is_input,
            port_id: _,
        } = self
        {
            Some(*is_input)
        } else {
            None
        }
    }
    pub fn implies_read_only(&self) -> bool {
        match self {
            DeclarationKind::NotPort => false,
            DeclarationKind::StructField { field_id: _ } => false,
            DeclarationKind::RegularPort {
                is_input,
                port_id: _,
            } => *is_input,
            DeclarationKind::GenerativeInput(_) => true,
        }
    }
}

/// An [Instruction] that represents a declaration of a new local variable.
///
/// It can be referenced by a [WireReferenceRoot::LocalDecl]
///
/// A Declaration Instruction always corresponds to a new entry in the [self::name_context::LocalVariableContext].
#[derive(Debug)]
pub struct Declaration {
    pub typ_expr: WrittenType,
    pub typ: FullType,
    pub decl_span: Span,
    pub name_span: Span,
    pub name: String,
    pub declaration_runtime_depth: OnceCell<usize>,
    /// Variables are read_only when they may not be controlled by the current block of code.
    /// This is for example, the inputs of the current module, or the outputs of nested modules.
    /// But could also be the iterator of a for loop.
    pub read_only: bool,
    /// If the program text already covers the write, then lsp stuff on this declaration shouldn't use it.
    pub declaration_itself_is_not_written_to: bool,
    pub decl_kind: DeclarationKind,
    pub identifier_type: IdentifierType,
    pub latency_specifier: Option<FlatID>,
    pub documentation: Documentation,
}

/// An [Instruction] that represents a instantiation of a submodule.
///
/// It can be referenced by a [WireReferenceRoot::SubModulePort]
///
/// A SubModuleInstance Instruction always corresponds to a new entry in the [self::name_context::LocalVariableContext].
///
/// When instantiating, creates a [crate::instantiation::SubModule]
#[derive(Debug)]
pub struct SubModuleInstance {
    pub module_ref: GlobalReference<ModuleUUID>,
    /// Name is not always present in source code. Such as in inline function call syntax: my_mod(a, b, c)
    pub name: Option<(String, Span)>,
    /// Maps each of the module's local domains to the domain that it is used in.
    ///
    /// These are *always* [DomainType::Physical] (of course, start out as [DomainType::Unknown] before typing)
    pub local_interface_domains: FlatAlloc<DomainType, DomainIDMarker>,
    pub documentation: Documentation,
}

impl SubModuleInstance {
    pub fn get_name<'o, 's: 'o, 'l: 'o>(&'s self, corresponding_module: &'l Module) -> &'o str {
        if let Some((n, _span)) = &self.name {
            n
        } else {
            &corresponding_module.link_info.name
        }
    }
    /// If it is named, then return the [Span] of the name, otherwise return the span of the module ref
    pub fn get_most_relevant_span(&self) -> Span {
        if let Some((_name, span)) = &self.name {
            *span
        } else {
            self.module_ref.get_total_span()
        }
    }
}

/// See [FuncCallInstruction]
#[derive(Debug)]
pub struct ModuleInterfaceReference {
    pub submodule_decl: FlatID,
    pub submodule_interface: InterfaceID,

    /// If this is None, that means the submodule was declared implicitly. Hence it could also be used at compiletime
    pub name_span: Option<Span>,

    /// Best-effort span for the interface that is called. [my_mod<abc>](), my_mod<abc> mm; [mm]() or mm.[my_interface]()
    ///
    /// if interface_span == name_span then no specific interface is selected, so the main interface is used
    pub interface_span: Span,
}

/// An [Expression] that represents the calling on an interface of a [SubModuleInstance].
/// It is the connecting of multiple input ports, and output ports on a submodule in one statement.
///
/// Multiple outputs (or zero outputs) are only supported for non-subexpressions.
///
/// See [ExpressionOutput]
///
/// Function calls can come in three forms:
///
/// ```sus
/// module xor {
///     interface xor : bool a, bool b -> bool c
/// }
///
/// module fifo #(T) {
///     interface push : bool push, T data
///     interface pop : bool pop -> bool valid, T data
/// }
///
/// module use_modules {
///     // We can use functions inline
///     bool x = xor(true, false)
///
///     // Declare the submodule explicitly
///     xor xor_inst
///     bool y = xor_inst(true, false)
///
///     // Or access interfaces explicitly
///     fifo my_fifo
///     bool z, int data = my_fifo.pop()
///
///     // Finally, if a function returns a single argument, we can call it inline in an expression:
///     bool w = true | xor(true, false)
/// }
/// ```
#[derive(Debug)]
pub struct FuncCall {
    pub interface_reference: ModuleInterfaceReference,

    /// Points to a list of [Expression]
    pub arguments: Vec<FlatID>,

    pub arguments_span: BracketSpan,
}

impl FuncCall {
    pub fn could_be_at_compile_time(&self) -> bool {
        todo!("self.name_span.is_none() but also other requirements, like if the module is a function")
    }
}

/// A control-flow altering [Instruction] to represent compiletime and runtime if & when statements.
#[derive(Debug)]
pub struct IfStatement {
    pub condition: FlatID,
    pub is_generative: bool,
    pub then_block: FlatIDRange,
    pub else_block: FlatIDRange,
}

/// A control-flow altering [Instruction] to represent compiletime looping on a generative index
#[derive(Debug)]
pub struct ForStatement {
    pub loop_var_decl: FlatID,
    pub start: FlatID,
    pub end: FlatID,
    pub loop_body: FlatIDRange,
}

/// When a module has been parsed and flattened, it is turned into a large list of instructions,
/// These are stored in [LinkInfo::instructions]`: FlatAlloc<Instruction, FlatIDMarker>`
///
/// Instructions are indexed with [FlatID]
///
/// One may ask: Why have [Expression], [WrittenType], etc refer to others by [FlatID], instead of a recursive datastructure?
/// The reason is that later representations, such as [crate::instantiation::RealWire] and other structures can still refer to intermediate parts of expressions
/// They can simply refer to the [FlatID] of these instructions, instead of some convoluted other representation.
///
/// When executing, the instructions are processed in order. Control flow instructions like [IfStatement] and [ForStatement] can cause the executor to repeat or skip sections.
#[derive(Debug)]
pub enum Instruction {
    SubModule(SubModuleInstance),
    Declaration(Declaration),
    Expression(Expression),
    IfStatement(IfStatement),
    ForStatement(ForStatement),
}

/// Used as a convenient shorthand for [ExpressionOutput::SubExpression], to replace old uses of [Expression]
#[derive(Debug, Clone, Copy)]
pub struct SingleOutputExpression<'e> {
    pub typ: &'e AbstractRankedType,
    pub domain: DomainType,
    pub span: Span,
    pub source: &'e ExpressionSource,
}

impl Instruction {
    #[track_caller]
    pub fn unwrap_expression(&self) -> &Expression {
        let Self::Expression(expr) = self else {
            panic!("unwrap_expression on not a expression! Found {self:?}")
        };
        expr
    }
    #[track_caller]
    pub fn unwrap_subexpression(&self) -> SingleOutputExpression {
        let expr = self.unwrap_expression();
        let ExpressionOutput::SubExpression(typ) = &expr.output else {
            unreachable!("unwrap_subexpression on not a SubExpression")
        };
        SingleOutputExpression {
            typ,
            domain: expr.domain,
            span: expr.span,
            source: &expr.source,
        }
    }
    #[track_caller]
    pub fn unwrap_declaration(&self) -> &Declaration {
        let Self::Declaration(decl) = self else {
            panic!("unwrap_declaration on not a Declaration! Found {self:?}")
        };
        decl
    }
    #[track_caller]
    pub fn unwrap_submodule(&self) -> &SubModuleInstance {
        let Self::SubModule(sm) = self else {
            panic!("unwrap_submodule on not a SubModule! Found {self:?}")
        };
        sm
    }

    pub fn get_span(&self) -> Span {
        match self {
            Instruction::SubModule(sub_module_instance) => {
                sub_module_instance.get_most_relevant_span()
            }
            Instruction::Declaration(declaration) => declaration.decl_span,
            Instruction::Expression(expression) => expression.span,
            Instruction::IfStatement(_) => unreachable!(),
            Instruction::ForStatement(_) => unreachable!(),
        }
    }
}
