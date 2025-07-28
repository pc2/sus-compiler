mod flatten;
mod initialization;
mod name_context;
mod parser;
pub mod typecheck;
mod walk;

use crate::flattening::typecheck::TyCell;
use crate::prelude::*;
use crate::typing::abstract_type::{AbstractGlobalReference, AbstractRankedType, PeanoType};
use crate::typing::domain_type::DomainType;

use std::cell::{Cell, OnceCell};

use crate::latency::port_latency_inference::PortLatencyInferenceInfo;
pub use flatten::flatten_all_globals;
pub use initialization::gather_initial_file_data;

use crate::linker::{Documentation, LinkInfo};
use crate::value::Value;

use crate::typing::template::{TVec, TemplateArg, TemplateKind};

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

    /// Created in Stage 2: Initialization
    ///
    /// Ports can only use domains in [Self::domains]
    ///
    /// These are used by instantiation. They directly correspond to the ports that are actually used in the generated code
    pub ports: FlatAlloc<Port, PortIDMarker>,

    /// Created in Stage 2: Flattening
    pub latency_inference_info: PortLatencyInferenceInfo,

    /// Created in Stage 2: Initialization
    pub domains: FlatAlloc<DomainInfo, DomainIDMarker>,

    /// Created in Stage 2: Initialization
    ///
    /// Used for resolving the names. These shouldn't really occur in Instantiation
    pub interfaces: FlatAlloc<Interface, InterfaceIDMarker>,
}

impl Module {
    /// Temporary upgrade such that we can name the singular clock of the module, such that weirdly-named external module clocks can be used
    ///
    /// See #7
    pub fn get_clock_name(&self) -> &str {
        &self.domains.iter().next().unwrap().1.name
    }
    pub fn get_fn_interface(&self, interface_id: InterfaceID) -> &InterfaceDeclaration {
        let interface = &self.interfaces[interface_id];
        let_unwrap!(
            Some(InterfaceDeclKind::Interface(i)),
            interface.declaration_instruction
        );
        self.link_info.instructions[i].unwrap_interface()
    }

    pub fn get_port_for_decl(&self, decl_id: FlatID) -> (PortID, Direction) {
        let decl = self.link_info.instructions[decl_id].unwrap_declaration();
        let_unwrap!(
            DeclarationKind::Port {
                direction,
                port_id,
                ..
            },
            decl.decl_kind
        );
        (port_id, direction)
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

/// Represents a field in a struct
///
/// UNFINISHED
///
/// TODO: Structs #8
#[derive(Debug)]
pub struct StructField {
    #[allow(unused)]
    pub name: String,
    #[allow(unused)]
    pub name_span: Span,
    #[allow(unused)]
    pub decl_span: Span,

    pub declaration_instruction: FlatID,
}

/// Global constant, like `true`, `false`, or user-defined constants (TODO #19)
///
/// All Constants are stored in [Linker::constants] and indexed by [ConstantUUID]
#[derive(Debug)]
pub struct NamedConstant {
    pub link_info: LinkInfo,
    pub output_decl: FlatID,
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
    pub direction: Direction,
    pub domain: DomainID,
    /// Points to a [Declaration]
    pub declaration_instruction: FlatID,
    pub latency_specifier: Option<FlatID>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Direction {
    Input,
    Output,
}
impl Direction {
    pub fn invert(self) -> Direction {
        match self {
            Direction::Input => Direction::Output,
            Direction::Output => Direction::Input,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum InterfaceKind {
    RegularInterface,
    Action(PortID),
    Trigger(PortID),
}

impl InterfaceKind {
    pub fn is_conditional(&self) -> bool {
        match self {
            InterfaceKind::RegularInterface => false,
            InterfaceKind::Action(_) | InterfaceKind::Trigger(_) => true,
        }
    }
    pub fn as_string(&self) -> &'static str {
        match self {
            InterfaceKind::RegularInterface => "interface",
            InterfaceKind::Action(_) => "action",
            InterfaceKind::Trigger(_) => "trigger",
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum InterfaceDeclKind {
    Interface(FlatID),
    SinglePort(FlatID),
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
    pub domain: Option<DomainID>,
    pub declaration_instruction: Option<InterfaceDeclKind>,
}

#[derive(Debug, Clone, Copy)]
pub enum PathElemRefersTo {
    Interface(InterfaceID),
}

/// An element in a [WireReference] path. Could be array accesses, slice accesses, field accesses, etc
///
/// When executing, this turns into [crate::instantiation::RealWirePathElem]
#[derive(Debug)]
pub enum WireReferencePathElement {
    FieldAccess {
        name: String,
        name_span: Span,
        refers_to: OnceCell<PathElemRefersTo>,
        input_typ: TyCell<AbstractRankedType>,
    },
    ArrayAccess {
        idx: FlatID,
        bracket_span: BracketSpan,
        input_typ: TyCell<AbstractRankedType>,
    },
    ArraySlice {
        from: Option<FlatID>,
        to: Option<FlatID>,
        bracket_span: BracketSpan,
        input_typ: TyCell<AbstractRankedType>,
    },
    ArrayPartSelect {
        from: FlatID,
        width: FlatID,
        bracket_span: BracketSpan,
        input_typ: TyCell<AbstractRankedType>,
        direction: PartSelectDirection,
    },
}

impl WireReferencePathElement {
    pub fn get_span(&self) -> Span {
        match self {
            WireReferencePathElement::FieldAccess { name_span, .. } => *name_span,
            WireReferencePathElement::ArrayAccess { bracket_span, .. }
            | WireReferencePathElement::ArraySlice { bracket_span, .. }
            | WireReferencePathElement::ArrayPartSelect { bracket_span, .. } => {
                bracket_span.outer_span()
            }
        }
    }
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
    /// FIFO fifo
    /// fifo = 3
    /// ```
    ///
    /// [FlatID] points to [Instruction::SubModule]
    LocalSubmodule(FlatID),
    /// ```sus
    /// trigger xyz: int a
    /// when something {
    ///     xyz(5)
    /// }
    /// ```
    ///
    /// [FlatID] points to [Instruction::Interface]
    LocalInterface(FlatID),
    /// ```sus
    /// bool b = true // root is global constant `true`
    /// ```
    NamedConstant(GlobalReference<ConstantUUID>),
    /// ```sus
    /// Repeat(...) // root is global constant `Repeat`
    /// ```
    NamedModule(GlobalReference<ModuleUUID>),
    /// Used to conveniently represent errors
    Error,
}

impl WireReferenceRoot {
    pub fn get_root_flat(&self) -> Option<FlatID> {
        match self {
            WireReferenceRoot::LocalDecl(f) => Some(*f),
            WireReferenceRoot::LocalSubmodule(f) => Some(*f),
            WireReferenceRoot::LocalInterface(f) => Some(*f),
            WireReferenceRoot::NamedConstant(_) => None,
            WireReferenceRoot::NamedModule(_) => None,
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
    pub output_typ: TyCell<AbstractRankedType>,
    pub root_span: Span,
}

impl WireReference {
    pub fn is_error(&self) -> bool {
        matches!(&self.root, WireReferenceRoot::Error)
    }
    pub fn get_root_typ(&self) -> &AbstractRankedType {
        if let Some(first) = self.path.first() {
            match first {
                WireReferencePathElement::ArrayAccess { input_typ, .. }
                | WireReferencePathElement::FieldAccess { input_typ, .. }
                | WireReferencePathElement::ArrayPartSelect { input_typ, .. }
                | WireReferencePathElement::ArraySlice { input_typ, .. } => input_typ,
            }
        } else {
            &self.output_typ
        }
    }
    pub fn get_total_span(&self) -> Span {
        if let Some(last) = self.path.last() {
            Span::new_overarching(self.root_span, last.get_span())
        } else {
            self.root_span
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
    pub target_domain: Cell<DomainType>,
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PartSelectDirection {
    Up,
    Down,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SliceType {
    Normal,
    PartSelect(PartSelectDirection),
}

/// See [Expression]
#[derive(Debug)]
pub enum ExpressionSource {
    WireRef(WireReference), // Used to add a span to the reference of a wire.
    FuncCall(FuncCall),
    UnaryOp {
        op: UnaryOperator,
        /// Operators automatically parallelize across arrays
        rank: TyCell<PeanoType>,
        right: FlatID,
    },
    BinaryOp {
        op: BinaryOperator,
        /// Operators automatically parallelize across arrays
        rank: TyCell<PeanoType>,
        left: FlatID,
        right: FlatID,
    },
    ArrayConstruct(Vec<FlatID>),
    Literal(Value),
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
    SubExpression(TyCell<AbstractRankedType>),
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
    pub parent_condition: Option<ParentCondition>,
    pub source: ExpressionSource,
    /// Means [Self::source] can be computed at compiletime, not that [Self::output] neccesarily requires a generative result
    pub domain: Cell<DomainType>,

    /// If [None], then this function returns a single result like a normal expression
    /// If Some(outputs), then this function is a dead-end expression, and does it's outputs manually
    pub output: ExpressionOutput,
}

impl Expression {
    pub fn as_single_output_expr(&self) -> Option<SingleOutputExpression<'_>> {
        let typ = match &self.output {
            ExpressionOutput::SubExpression(typ) => typ,
            ExpressionOutput::MultiWrite(write_tos) => {
                let [single_write] = write_tos.as_slice() else {
                    return None;
                };
                &single_write.to.output_typ
            }
        };
        Some(SingleOutputExpression {
            typ,
            domain: self.domain.get(),
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
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum DeclarationKind {
    RegularWire {
        is_state: bool,
        read_only: bool,
    },
    StructField(FieldID),
    Port {
        direction: Direction,
        is_state: bool,
        port_id: PortID,
        parent_interface: InterfaceID,
    },
    ConditionalBinding {
        when_id: FlatID,
        direction: Direction,
        is_state: bool,
    },
    RegularGenerative {
        read_only: bool,
    },
    TemplateParameter(TemplateID),
}

impl DeclarationKind {
    /// Basically an unwrap to see if this [Declaration] refers to a [Port], and returns `Some(direction)` if so.
    pub fn is_io_port(&self) -> Option<Direction> {
        if let DeclarationKind::Port { direction, .. } = self {
            Some(*direction)
        } else {
            None
        }
    }
    pub fn is_read_only(&self) -> bool {
        match self {
            DeclarationKind::RegularWire { read_only, .. } => *read_only,
            DeclarationKind::ConditionalBinding { direction, .. } => *direction == Direction::Input,
            DeclarationKind::StructField(_) => false,
            DeclarationKind::Port { direction, .. } => *direction == Direction::Input,
            DeclarationKind::RegularGenerative { read_only } => *read_only,
            DeclarationKind::TemplateParameter(_) => true,
        }
    }
    pub fn is_generative(&self) -> bool {
        match self {
            DeclarationKind::RegularWire { .. }
            | DeclarationKind::ConditionalBinding { .. }
            | DeclarationKind::StructField(_)
            | DeclarationKind::Port { .. } => false,
            DeclarationKind::RegularGenerative { .. } | DeclarationKind::TemplateParameter(..) => {
                true
            }
        }
    }
    pub fn is_state(&self) -> bool {
        match self {
            DeclarationKind::RegularWire { is_state, .. }
            | DeclarationKind::Port { is_state, .. }
            | DeclarationKind::ConditionalBinding { is_state, .. } => *is_state,
            DeclarationKind::StructField(_)
            | DeclarationKind::RegularGenerative { .. }
            | DeclarationKind::TemplateParameter(..) => false,
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
    pub parent_condition: Option<ParentCondition>,
    pub typ_expr: WrittenType,
    pub typ: TyCell<AbstractRankedType>,
    pub domain: Cell<DomainType>,
    pub decl_span: Span,
    pub name_span: Span,
    pub name: String,
    /// If the program text already covers the write, then lsp stuff on this declaration shouldn't use it.
    pub declaration_itself_is_not_written_to: bool,
    pub decl_kind: DeclarationKind,
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
    pub parent_condition: Option<ParentCondition>,
    pub module_ref: GlobalReference<ModuleUUID>,

    pub name: String,
    pub name_span: Span,
    /// Maps each of the module's local domains to the domain that it is used in.
    ///
    /// These are *always* [DomainType::Physical] (of course, start out as [DomainType::Unknown] before typing)
    pub local_domain_map: TyCell<FlatAlloc<DomainType, DomainIDMarker>>,
    pub typ: TyCell<AbstractRankedType>,
    pub documentation: Documentation,
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
    pub func_wire_ref: FlatID,

    /// Points to a list of [Expression]
    pub arguments: Vec<FlatID>,

    pub arguments_span: BracketSpan,
}

impl FuncCall {
    pub fn could_be_at_compile_time(&self) -> bool {
        todo!("self.name_span.is_none() but also other requirements, like if the module is a function")
    }
}

/// References any [crate::flattening::Module], [crate::flattening::StructType], or [crate::flattening::NamedConstant],
/// and includes any template arguments.
///
/// As an example, this is the struct in charge of representing:
/// ```sus
/// FIFO #(DEPTH : 32, T : type int)
/// ```
#[derive(Debug)]
pub struct GlobalReference<ID> {
    pub name_span: Span,
    pub id: ID,
    pub template_args: Vec<WrittenTemplateArg>,
    pub template_arg_types: TyCell<TVec<TemplateKind<AbstractRankedType, ()>>>,
    pub template_span: Option<BracketSpan>,
}

impl<ID: Copy> GlobalReference<ID> {
    pub fn get_total_span(&self) -> Span {
        let mut result = self.name_span;
        if let Some(template_span) = self.template_span {
            result = Span::new_overarching(result, template_span.outer_span());
        }
        result
    }

    pub fn as_abstract_global_ref(&self) -> AbstractGlobalReference<ID> {
        AbstractGlobalReference {
            id: self.id,
            template_arg_types: self.template_arg_types.clone(),
        }
    }

    pub fn resolve_template_args(&self, errors: &ErrorCollector, target: &LinkInfo) {
        let full_object_name = target.get_full_name();

        let mut previous_uses: TVec<Option<Span>> = target.template_parameters.map(|_| None);

        for arg in &self.template_args {
            let name = &arg.name;
            if let Some(refers_to) = target
                .template_parameters
                .find(|_, param| param.name == arg.name)
            {
                arg.refers_to.set(refers_to).unwrap();
            }

            if let Some(&refer_to) = arg.refers_to.get() {
                let param = &target.template_parameters[refer_to];

                match (&param.kind, &arg.kind) {
                    (TemplateKind::Value(_), Some(TemplateKind::Type(_))) => {
                        errors
                            .error(
                                arg.name_span,
                                format!(
                                "'{name}' is not a value. `type` keyword cannot be used for values"
                            ),
                            )
                            .info((param.name_span, target.file), "Declared here");
                    }
                    (TemplateKind::Type(_), Some(TemplateKind::Value(_))) => {
                        errors
                            .error(arg.name_span, format!("'{name}' is not a type. To use template type arguments use the `type` keyword like `T: type int[123]`"))
                            .info((param.name_span, target.file), "Declared here");
                    }
                    _ => {}
                }

                if let Some(prev_use) = previous_uses[refer_to] {
                    errors
                        .error(
                            arg.name_span,
                            format!("'{name}' has already been defined previously"),
                        )
                        .info_same_file(prev_use, format!("'{name}' specified here previously"));
                } else {
                    previous_uses[refer_to] = Some(arg.name_span);
                }
            } else {
                errors
                    .error(
                        arg.name_span,
                        format!("'{name}' is not a valid template argument of {full_object_name}"),
                    )
                    .info_obj(target);
            }
        }
    }
    pub fn get_arg_for(&self, id: TemplateID) -> Option<&WrittenTemplateArg> {
        self.template_args
            .iter()
            .find(|arg| arg.refers_to.get().copied() == Some(id))
    }
    pub fn get_type_arg_for(&self, id: TemplateID) -> Option<&WrittenType> {
        let arg = self.get_arg_for(id)?;
        let Some(TemplateKind::Type(t)) = &arg.kind else {
            return None;
        };
        Some(t)
    }
    pub fn get_value_arg_for(&self, id: TemplateID) -> Option<FlatID> {
        let arg = self.get_arg_for(id)?;
        let Some(TemplateKind::Value(v)) = &arg.kind else {
            return None;
        };
        Some(*v)
    }
}

#[derive(Debug)]
pub struct WrittenTemplateArg {
    pub name: String,
    pub name_span: Span,
    pub value_span: Span,
    pub refers_to: OnceCell<TemplateID>,
    pub kind: Option<TemplateKind<WrittenType, FlatID>>,
}

pub type AbstractTemplateArg = TemplateKind<TemplateArg<WrittenType>, TemplateArg<FlatID>>;

impl AbstractTemplateArg {
    pub fn map_is_provided(&self) -> Option<(Span, Span, TemplateKind<&WrittenType, &FlatID>)> {
        match self {
            TemplateKind::Type(TemplateArg::Provided {
                name_span,
                value_span,
                arg,
                ..
            }) => Some((*name_span, *value_span, TemplateKind::Type(arg))),
            TemplateKind::Value(TemplateArg::Provided {
                name_span,
                value_span,
                arg,
                ..
            }) => Some((*name_span, *value_span, TemplateKind::Value(arg))),
            TemplateKind::Type(TemplateArg::NotProvided { .. }) => None,
            TemplateKind::Value(TemplateArg::NotProvided { .. }) => None,
        }
    }
}

/// The textual representation of a type expression in the source code.
///
/// Not to be confused with [crate::typing::abstract_type::AbstractType] which is for working with types in the flattening stage,
/// or [crate::typing::concrete_type::ConcreteType], which is for working with types post instantiation.
#[derive(Debug)]
pub enum WrittenType {
    Error(Span),
    TemplateVariable(Span, TemplateID),
    Named(GlobalReference<TypeUUID>),
    Array(Span, Box<(WrittenType, FlatID, BracketSpan)>),
}

impl WrittenType {
    pub fn get_span(&self) -> Span {
        match self {
            WrittenType::Error(total_span)
            | WrittenType::TemplateVariable(total_span, ..)
            | WrittenType::Array(total_span, _) => *total_span,
            WrittenType::Named(global_ref) => global_ref.get_total_span(),
        }
    }
}

/// A control-flow altering [Instruction] to represent compiletime and runtime if & when statements.
#[derive(Debug)]
pub struct IfStatement {
    pub if_keyword_span: Span,
    pub parent_condition: Option<ParentCondition>,
    pub condition: FlatID,
    pub is_generative: bool,
    pub then_block: FlatIDRange,
    pub else_block: FlatIDRange,
    pub bindings_read_only: Vec<FlatID>,
    pub bindings_writable: Vec<FlatID>,
    pub conditional_bindings_span: Option<Span>,
}

/// A control-flow altering [Instruction] to represent compiletime looping on a generative index
#[derive(Debug)]
pub struct ForStatement {
    pub parent_condition: Option<ParentCondition>,
    pub loop_var_decl: FlatID,
    pub start: FlatID,
    pub end: FlatID,
    pub loop_body: FlatIDRange,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ParentCondition {
    pub parent_when: FlatID,
    pub is_else_branch: bool,
}

#[derive(Debug)]
pub struct InterfaceDeclaration {
    pub parent_condition: Option<ParentCondition>,
    pub name: String,
    pub name_span: Span,
    pub decl_span: Span,
    pub interface_kw_span: Span,
    pub documentation: Documentation,
    pub latency_specifier: Option<FlatID>,
    pub is_local: bool,
    pub interface_id: InterfaceID,
    pub interface_kind: InterfaceKind,
    /// These and [Self::outputs] are respective to the function-call syntax!
    ///
    /// Do not be confused by [InterfaceKind::Trigger], where [Self::inputs] corresponds to module Output ports!
    pub inputs: Vec<FlatID>,
    /// See [Self::inputs]
    pub outputs: Vec<FlatID>,
    pub then_block: FlatIDRange,
    pub else_block: FlatIDRange,
    pub domain: DomainType,
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
    Interface(InterfaceDeclaration),
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
    pub fn unwrap_subexpression(&self) -> SingleOutputExpression<'_> {
        let expr = self.unwrap_expression();
        let ExpressionOutput::SubExpression(typ) = &expr.output else {
            unreachable!("unwrap_subexpression on not a SubExpression")
        };
        SingleOutputExpression {
            typ,
            domain: expr.domain.get(),
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
    pub fn unwrap_interface(&self) -> &InterfaceDeclaration {
        let Self::Interface(interf) = self else {
            panic!("unwrap_declaration on not a Declaration! Found {self:?}")
        };
        interf
    }
    #[track_caller]
    pub fn unwrap_submodule(&self) -> &SubModuleInstance {
        let Self::SubModule(sm) = self else {
            panic!("unwrap_submodule on not a SubModule! Found {self:?}")
        };
        sm
    }
    #[track_caller]
    pub fn unwrap_if(&self) -> &IfStatement {
        let Self::IfStatement(ii) = self else {
            panic!("unwrap_if on not a IfStatement! Found {self:?}")
        };
        ii
    }

    pub fn get_parent_condition(&self) -> Option<ParentCondition> {
        match self {
            Instruction::SubModule(SubModuleInstance {
                parent_condition, ..
            })
            | Instruction::Declaration(Declaration {
                parent_condition, ..
            })
            | Instruction::Expression(Expression {
                parent_condition, ..
            })
            | Instruction::IfStatement(IfStatement {
                parent_condition, ..
            })
            | Instruction::ForStatement(ForStatement {
                parent_condition, ..
            })
            | Instruction::Interface(InterfaceDeclaration {
                parent_condition, ..
            }) => *parent_condition,
        }
    }
    pub fn get_span(&self) -> Span {
        match self {
            Instruction::SubModule(sub_module_instance) => sub_module_instance.name_span,
            Instruction::Declaration(declaration) => declaration.name_span,
            Instruction::Interface(act_trig) => act_trig.name_span,
            Instruction::Expression(expression) => expression.span,
            Instruction::IfStatement(_) | Instruction::ForStatement(_) => {
                unreachable!("{self:?} is control flow! Shouldn't ask it's span")
            }
        }
    }
    pub fn get_name(&self) -> &str {
        match self {
            Instruction::Declaration(declaration) => &declaration.name,
            Instruction::Interface(interface_declaration) => &interface_declaration.name,
            Instruction::SubModule(submod) => &submod.name,
            Instruction::Expression(_)
            | Instruction::IfStatement(_)
            | Instruction::ForStatement(_) => unreachable!("{self:?} is not nameable!"),
        }
    }
    pub fn get_latency_specifier(&self) -> Option<FlatID> {
        match self {
            Instruction::Declaration(declaration) => declaration.latency_specifier,
            Instruction::Interface(interface) => interface.latency_specifier,
            Instruction::SubModule(_)
            | Instruction::Expression(_)
            | Instruction::IfStatement(_)
            | Instruction::ForStatement(_) => {
                unreachable!("{self:?} Cannot have Latency Specifier!")
            }
        }
    }
}
