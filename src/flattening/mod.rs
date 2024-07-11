
mod name_context;
mod initialization;
mod typechecking;
mod parse;
mod walk;

use std::ops::Deref;

pub use parse::flatten_all_modules;
pub use initialization::gather_initial_file_data;
pub use typechecking::typecheck_all_modules;

use crate::{
    arena_alloc::{FlatAlloc, UUIDMarker, UUIDRange, UUID}, errors::ErrorCollector, file_position::{BracketSpan, FileText, Span}, instantiation::InstantiationList, parser::Documentation, value::Value
};
use crate::linker::{ConstantUUID, LinkInfo, ModuleUUID, TypeUUID};

use crate::typing::{
    abstract_type::{AbstractType, FullType},
    template::{GlobalReference, TemplateArgs, TemplateID},
};

pub struct FlatIDMarker;
impl UUIDMarker for FlatIDMarker {const DISPLAY_NAME : &'static str = "obj_";}
pub type FlatID = UUID<FlatIDMarker>;

pub type FlatIDRange = UUIDRange<FlatIDMarker>;

pub struct PortIDMarker;
impl UUIDMarker for PortIDMarker {const DISPLAY_NAME : &'static str = "port_";}
pub type PortID = UUID<PortIDMarker>;

pub type PortIDRange = UUIDRange<PortIDMarker>;

pub struct InterfaceIDMarker;
impl UUIDMarker for InterfaceIDMarker {const DISPLAY_NAME : &'static str = "interface_";}
pub type InterfaceID = UUID<InterfaceIDMarker>;

pub struct DomainIDMarker;
impl UUIDMarker for DomainIDMarker {const DISPLAY_NAME : &'static str = "domain_";}
/// Interfaces are also indexed using DomainIDs. But in general, these refer to (clock/latency counting) domains
pub type DomainID = UUID<DomainIDMarker>;

/// Modules are compiled in 4 stages. All modules must pass through each stage before advancing to the next stage. 
/// 
/// 1. Initialization: initial name resolution and port discovery. The Module objects themselves are constructed. 
/// 
/// 2. Flattening: 
/// 
///     2.1: Parsing: Parse source code to create instruction list. 
/// 
///     2.2: Typecheck: Add typ variables to everything. [Declaration::typ], [WireInstance::typ] and [SubModuleInstance::local_interface_domains] are set in this stage. 
/// 
/// 3. Instantiation: Actually run generative code and instantiate modules. 
/// 
///     3.1: Execution
///     
///     3.2: Concrete Typecheck, Latency Counting
#[derive(Debug)]
pub struct Module {
    /// Created in Stage 1: Initialization
    pub link_info : LinkInfo,

    /// Created in Stage 1: Initialization
    /// 
    /// [Port::declaration_instruction] are set in Stage 2: Flattening
    pub ports : FlatAlloc<Port, PortIDMarker>,

    /// Created in Stage 1: Initialization
    pub domain_names : FlatAlloc<String, DomainIDMarker>,

    /// Created in Stage 1: Initialization
    pub interfaces : FlatAlloc<Interface, InterfaceIDMarker>,

    /// Created in Stage 2: Flattening. type data is filled out during Typechecking
    pub instructions : FlatAlloc<Instruction, FlatIDMarker>,

    /// Created in Stage 2: Typechecking
    pub domains : FlatAlloc<DomainInfo, DomainIDMarker>,

    /// Created in Stage 3: Instantiation
    pub instantiations : InstantiationList
}

impl Module {
    pub fn get_main_interface(&self) -> Option<(InterfaceID, &Interface)> {
        self.interfaces.iter().find(|(_, interf)| interf.name == self.link_info.name)
    }

    pub fn get_port_decl(&self, port : PortID) -> &Declaration {
        let flat_port = self.ports[port].declaration_instruction;

        self.instructions[flat_port].unwrap_wire_declaration()
    }

    /// Get a port by the given name. Reports non existing ports errors
    /// 
    /// Prefer interfaces over ports in name conflicts
    pub fn get_port_or_interface_by_name(&self, name_span : Span, file_text : &FileText, errors : &ErrorCollector) -> Option<PortOrInterface> {
        let name_text = &file_text[name_span];
        for (id, data) in &self.interfaces {
            if data.name == name_text {
                return Some(PortOrInterface::Interface(id))
            }
        }
        for (id, data) in &self.ports {
            if data.name == name_text {
                return Some(PortOrInterface::Port(id))
            }
        }
        errors
            .error(name_span, format!("There is no port or interface of name '{name_text}' on module {}", self.link_info.name))
            .info_obj(self);
        return None
    }

    pub fn get_instruction_span(&self, instr_id : FlatID) -> Span {
        match &self.instructions[instr_id] {
            Instruction::SubModule(sm) => sm.module_ref.span,
            Instruction::FuncCall(fc) => fc.whole_func_span,
            Instruction::Declaration(decl) => decl.decl_span,
            Instruction::Wire(w) => w.span,
            Instruction::Write(conn) => conn.to_span,
            Instruction::IfStatement(if_stmt) => self.get_instruction_span(if_stmt.condition),
            Instruction::ForStatement(for_stmt) => self.get_instruction_span(for_stmt.loop_var_decl),
        }
    }

    pub fn is_multi_domain(&self) -> bool {
        self.domains.len() > 1
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PortOrInterface {
    Port(PortID),
    Interface(InterfaceID)
}

#[derive(Debug)]
pub struct DomainInfo {
    pub name : String,
}

#[derive(Clone, Copy)]
pub struct InterfaceToDomainMap<'linker> {
    pub local_domain_map : &'linker FlatAlloc<DomainID, DomainIDMarker>,
    pub domains : &'linker FlatAlloc<DomainInfo, DomainIDMarker>
}

impl<'linker> InterfaceToDomainMap<'linker> {
    pub fn local_domain_to_global_domain(&self, domain : DomainID) -> &'linker DomainInfo {
        let local_domain = self.local_domain_map[domain];
        &self.domains[local_domain]
    }
}


#[derive(Debug,Clone,Copy,PartialEq,Eq)]
pub enum IdentifierType {
    Local,
    State,
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


#[derive(Debug)]
pub struct Port {
    pub name : String,
    pub name_span : Span,
    pub decl_span : Span,
    pub is_input : bool,
    pub domain : DomainID,
    /// This is only set after flattening is done. Initially just [UUID::PLACEHOLDER]
    pub declaration_instruction : FlatID
}

#[derive(Debug)]
pub struct Interface {
    pub name_span : Span,
    pub name : String,
    /// All the interface's ports have this domain too
    pub domain : DomainID,
    pub func_call_inputs : PortIDRange,
    pub func_call_outputs : PortIDRange
}

impl Interface {
    pub fn all_ports(&self) -> PortIDRange {
        assert_eq!(self.func_call_inputs.1, self.func_call_outputs.0);
        UUIDRange(self.func_call_inputs.0, self.func_call_outputs.1)
    }
}

#[derive(Debug, Clone, Copy)]
pub enum WireReferencePathElement {
    ArrayAccess{idx : FlatID, bracket_span : BracketSpan},
}

impl WireReferencePathElement {
    fn for_each_dependency<F : FnMut(FlatID)>(path : &[WireReferencePathElement], mut f : F) {
        for p in path {
            match p {
                WireReferencePathElement::ArrayAccess { idx, bracket_span:_ } => f(*idx),
            }
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum WireReferenceRoot {
    LocalDecl(FlatID, Span),
    NamedConstant(ConstantUUID, Span),
    SubModulePort(PortInfo)
}

impl WireReferenceRoot {
    pub fn get_root_flat(&self) -> Option<FlatID> {
        match self {
            WireReferenceRoot::LocalDecl(f, _) => Some(*f),
            WireReferenceRoot::NamedConstant(_, _) => None,
            WireReferenceRoot::SubModulePort(port) => Some(port.submodule_decl),
        }
    }
    #[track_caller]
    pub fn unwrap_local_decl(&self) -> FlatID {
        let Self::LocalDecl(decl, _) = self else {unreachable!()};
        *decl
    }
}

/// References to wires
/// 
/// Example: myModule.port[a][b:c]
#[derive(Debug)]
pub struct WireReference {
    pub root : WireReferenceRoot,
    pub path : Vec<WireReferencePathElement>
}

impl WireReference {
    fn simple_port(port : PortInfo) -> WireReference {
        WireReference{
            root : WireReferenceRoot::SubModulePort(port),
            path : Vec::new()
        }
    }
}

#[derive(Debug)]
pub enum WriteModifiers {
    Connection{num_regs : i64, regs_span : Span},
    Initial{initial_kw_span : Span}
}

#[derive(Debug)]
pub struct Write {
    pub from : FlatID,
    pub to : WireReference,
    pub to_span : Span,
    pub write_modifiers : WriteModifiers,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UnaryOperator {
    And,
    Or,
    Xor,
    Not,
    Sum,
    Product,
    Negate,
}

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
    LesserEq
}

#[derive(Debug, Clone, Copy)]
pub struct PortInfo {
    pub submodule_decl : FlatID,
    pub port : PortID,
    pub is_input : bool,
    /// Only set if the port is named as an explicit field. If the port name is implicit, such as in the function call syntax, then it is not present. 
    pub port_name_span : Option<Span>,
    /// Even this can be implicit. In the inline function call instantiation syntax there's no named submodule. my_mod(a, b, c)
    /// 
    /// Finally, if [Self::port_name_span].is_none(), then for highlighting and renaming, this points to a duplicate of a Function Call
    pub submodule_name_span : Option<Span>
}

#[derive(Debug)]
pub struct WireInstance {
    pub typ : FullType,
    pub span : Span,
    pub source : WireSource
}

#[derive(Debug)]
pub enum WireSource {
    WireRef(WireReference), // Used to add a span to the reference of a wire. 
    UnaryOp{op : UnaryOperator, right : FlatID},
    BinaryOp{op : BinaryOperator, left : FlatID, right : FlatID},
    Constant(Value)
}

impl WireSource {
    pub const fn new_error() -> WireSource {
        WireSource::Constant(Value::Error)
    }
}

#[derive(Debug)]
pub enum WrittenType {
    Error(Span),
    Template(Span, TemplateID),
    Named(GlobalReference<TypeUUID>),
    Array(Span, Box<(WrittenType, FlatID, BracketSpan)>)
}

impl WrittenType {
    pub fn get_span(&self) -> Span {
        match self {
            WrittenType::Error(span) | WrittenType::Template(span, ..) | WrittenType::Named(GlobalReference { span, ..}) | WrittenType::Array(span, _) => *span
        }
    }

    pub fn to_type(&self) -> AbstractType {
        match self {
            WrittenType::Error(_) => AbstractType::Error,
            WrittenType::Template(_, template_id) => AbstractType::Template(*template_id),
            WrittenType::Named(named_type) => AbstractType::Named(named_type.id),
            WrittenType::Array(_, arr_box) => {
                let (elem_typ, _arr_idx, _br_span) = arr_box.deref();
                AbstractType::Array(Box::new(elem_typ.to_type()))
            }
        }
    }

    pub fn to_type_with_substitute(&self, template_args : &TemplateArgs) -> AbstractType {
        match self {
            WrittenType::Error(_) => AbstractType::Error,
            WrittenType::Template(_, template_id) => {
                let Some(type_arg) = &template_args[*template_id] else {return AbstractType::Error}; // Could not infer the type, though this is a TODO
                let target_typ = type_arg.kind.unwrap_type();

                target_typ.to_type()
            }
            WrittenType::Named(named_type) => AbstractType::Named(named_type.id),
            WrittenType::Array(_, arr_box) => {
                let (elem_typ, _arr_idx, _br_span) = arr_box.deref();
                AbstractType::Array(Box::new(elem_typ.to_type_with_substitute(template_args)))
            }
        }
    }
}


const DECL_DEPTH_LATER : usize = usize::MAX;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DeclarationPortInfo {
    NotPort,
    RegularPort{is_input : bool, port_id : PortID},
    GenerativeInput(TemplateID)
}

impl DeclarationPortInfo {
    pub fn as_regular_port(&self) -> Option<bool> {
        if let DeclarationPortInfo::RegularPort{is_input, port_id:_} = self {
            Some(*is_input)
        } else {
            None
        }
    }
    pub fn implies_read_only(&self) -> bool {
        match self {
            DeclarationPortInfo::NotPort => false,
            DeclarationPortInfo::RegularPort { is_input, port_id:_ } => *is_input,
            DeclarationPortInfo::GenerativeInput(_) => true,
        }
    }
}

#[derive(Debug)]
pub struct Declaration {
    pub typ_expr : WrittenType,
    pub typ : FullType,
    pub decl_span : Span,
    pub name_span : Span,
    pub name : String,
    pub declaration_runtime_depth : usize,
    /// Variables are read_only when they may not be controlled by the current block of code. 
    /// This is for example, the inputs of the current module, or the outputs of nested modules. 
    /// But could also be the iterator of a for loop. 
    pub read_only : bool,
    /// If the program text already covers the write, then lsp stuff on this declaration shouldn't use it. 
    pub declaration_itself_is_not_written_to : bool,
    pub is_port : DeclarationPortInfo,
    pub identifier_type : IdentifierType,
    pub latency_specifier : Option<FlatID>,
    pub documentation : Documentation
}

#[derive(Debug)]
pub struct SubModuleInstance {
    pub module_ref : GlobalReference<ModuleUUID>,
    /// Name is not always present in source code. Such as in inline function call syntax: my_mod(a, b, c)
    pub name : Option<(String, Span)>,
    pub declaration_runtime_depth : usize,
    /// Maps each of the module's local domains to the domain that it is used in. 
    pub local_interface_domains : FlatAlloc<DomainID, DomainIDMarker>,
    pub documentation : Documentation
}

impl SubModuleInstance {
    pub fn get_name<'o, 's : 'o, 'l : 'o>(&'s self, corresponding_module : &'l Module) -> &'o str {
        if let Some((n, _span)) = &self.name {
            n
        } else {
            &corresponding_module.link_info.name
        }
    }
}

#[derive(Debug)]
pub struct ModuleInterfaceReference {
    pub submodule_decl : FlatID,
    pub submodule_interface : InterfaceID,

    /// If this is None, that means the submodule was declared implicitly. Hence it could also be used at compiletime
    pub name_span : Option<Span>,

    /// Best-effort span for the interface that is called. [my_mod<abc>](), my_mod<abc> mm; [mm]() or mm.[my_interface]()
    /// 
    /// if interface_span == name_span then no specific interface is selected, so the main interface is used
    pub interface_span : Span,
}

#[derive(Debug)]
pub struct FuncCallInstruction {
    pub interface_reference : ModuleInterfaceReference,
    /// arguments.len() == func_call_inputs.len() ALWAYS
    pub arguments : Vec<FlatID>,
    /// arguments.len() == func_call_inputs.len() ALWAYS
    pub func_call_inputs : PortIDRange,
    pub func_call_outputs : PortIDRange,

    pub arguments_span : BracketSpan,
    pub whole_func_span : Span,
}

impl FuncCallInstruction {
    pub fn could_be_at_compile_time(&self) -> bool {
        todo!("self.name_span.is_none() but also other requirements, like if the module is a function")
    }
}


#[derive(Debug)]
pub struct IfStatement {
    pub condition : FlatID,
    pub then_start : FlatID,
    pub then_end_else_start : FlatID,
    pub else_end : FlatID
}

#[derive(Debug)]
// Always is_compiletime
pub struct ForStatement {
    pub loop_var_decl : FlatID,
    pub start : FlatID,
    pub end : FlatID,
    pub loop_body : FlatIDRange
}

#[derive(Debug)]
pub enum Instruction {
    SubModule(SubModuleInstance),
    FuncCall(FuncCallInstruction),
    Declaration(Declaration),
    Wire(WireInstance),
    Write(Write),
    IfStatement(IfStatement),
    ForStatement(ForStatement)
}

impl Instruction {
    #[track_caller]
    pub fn unwrap_wire(&self) -> &WireInstance {
        let Self::Wire(w) = self else {panic!("unwrap_wire on not a wire! Found {self:?}")};
        w
    }
    #[track_caller]
    pub fn unwrap_wire_declaration(&self) -> &Declaration {
        let Self::Declaration(w) = self else {panic!("unwrap_wire_declaration on not a WireDeclaration! Found {self:?}")};
        w
    }
    #[track_caller]
    pub fn unwrap_submodule(&self) -> &SubModuleInstance {
        let Self::SubModule(sm) = self else {panic!("unwrap_submodule on not a SubModule! Found {self:?}")};
        sm
    }
    #[track_caller]
    pub fn unwrap_func_call(&self) -> &FuncCallInstruction {
        let Self::FuncCall(fc) = self else {panic!("unwrap_func_call on not a FuncCallInstruction! Found {self:?}")};
        fc
    }
}


#[derive(Debug)]
pub enum ModuleOrWrittenType {
    WrittenType(WrittenType),
    Module(GlobalReference<ModuleUUID>)
}
