
mod name_context;
mod initialization;
mod typechecking;
mod parse;

use std::ops::{Deref, Index};

pub use parse::flatten_all_modules;
pub use initialization::gather_initial_file_data;
pub use typechecking::typecheck_all_modules;

use crate::{
    abstract_type::{AbstractType, FullType}, arena_alloc::{FlatAlloc, UUIDMarker, UUIDRange, UUID}, errors::ErrorCollector, file_position::{BracketSpan, FileText, Span}, instantiation::InstantiationList, linker::{ConstantUUID, LinkInfo, Linkable, ModuleUUID, NamedType, TypeUUID}, parser::Documentation, pretty_print_many_spans, value::Value
};

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
#[derive(Debug)]
pub struct Module {
    /// Created by Stage 1: Initialization
    pub link_info : LinkInfo,

    /// Created by Stage 1: Initialization
    pub ports : FlatAlloc<Port, PortIDMarker>,
    
    /// Created by Stage 1: Initialization
    pub main_interface_used : bool,

    /// Created by Stage 1: Initialization
    /// 
    /// Every interface is a distinct Domain, but domains need not be attached to an interface
    pub interfaces : FlatAlloc<Interface, DomainIDMarker>,

    /// Created in Stage 2: Flattening and Typechecking
    pub instructions : FlatAlloc<Instruction, FlatIDMarker>,

    /// Created in Stage 2: Flattening and Typechecking
    /// 
    /// Every interface is a distinct Domain, but domains need not be attached to an interface
    pub domains : FlatAlloc<DomainInfo, DomainIDMarker>,

    /// Created in Stage 3: Instantiation
    pub instantiations : InstantiationList
}

impl Module {
    pub const MAIN_INTERFACE_ID : DomainID = DomainID::from_hidden_value(0);

    pub fn get_port_decl(&self, port : PortID) -> &Declaration {
        let flat_port = self.ports[port].declaration_instruction;

        self.instructions[flat_port].unwrap_wire_declaration()
    }

    pub fn make_port_info_fmt(&self, port_id : PortID, file_text : &FileText, result : &mut String) {
        use std::fmt::Write;
        let port = &self.ports[port_id];
        let port_direction = if port.identifier_type == IdentifierType::Input {"input"} else {"output"};
        writeln!(result, "{port_direction} {}", &file_text[port.decl_span]).unwrap()
    }
    pub fn make_port_info_string(&self, port_id : PortID, file_text : &FileText) -> String {
        let mut r = String::new(); self.make_port_info_fmt(port_id, file_text, &mut r); r
    }

    pub fn make_interface_info_fmt(&self, interface_id : DomainID, file_text : &FileText, result : &mut String) {
        for (port_id, port) in &self.ports {
            if port.interface == interface_id {
                self.make_port_info_fmt(port_id, file_text, result);
            }
        }
    }
    pub fn make_interface_info_string(&self, interface_id : DomainID, file_text : &FileText) -> String {
        let mut r = String::new(); self.make_interface_info_fmt(interface_id, file_text, &mut r); r
    }

    pub fn make_all_ports_info_fmt(&self, file_text : &FileText, local_domains : Option<InterfaceToDomainMap>, result : &mut String) {
        let mut interface_iter = self.interfaces.iter();
        if !self.main_interface_used {
            interface_iter.next();
        }

        for (interface_id, interface) in interface_iter {
            use std::fmt::Write;
            if let Some(domain_map) = &local_domains {
                writeln!(result, "{}: ({})", &interface.name, domain_map.get_submodule_interface_domain(interface_id).name).unwrap();
            } else {
                writeln!(result, "{}:", &interface.name).unwrap();
            }
            self.make_interface_info_fmt(interface_id, file_text, result);
        }
    }
    pub fn make_all_ports_info_string(&self, file_text : &FileText, local_domains : Option<InterfaceToDomainMap>) -> String {
        let mut r = String::new(); self.make_all_ports_info_fmt(file_text, local_domains, &mut r); r
    }

    pub fn print_flattened_module(&self, file_text : &FileText) {
        println!("[[{}]]:", self.link_info.name);
        println!("Interface:");
        for (port_id, port) in &self.ports {
            println!("    {} -> {:?}", self.make_port_info_string(port_id, file_text), port);
        }
        println!("Instructions:");
        let mut spans_print = Vec::new();
        for (id, inst) in &self.instructions {
            println!("    {id:?}: {inst:?}");
            let span = self.get_instruction_span(id);
            spans_print.push((format!("{id:?}"), span.into_range()));
        }
        pretty_print_many_spans(file_text.file_text.clone(), &spans_print);
    }

    /// Get a port by the given name. Reports non existing ports errors
    pub fn get_port_by_name(&self, name_span : Span, file_text : &FileText, errors : &ErrorCollector) -> Option<PortID> {
        let name_text = &file_text[name_span];
        for (id, data) in &self.ports {
            if data.name == name_text {
                return Some(id)
            }
        }
        errors
            .error(name_span, format!("There is no port '{name_text}' on module {}", self.link_info.name))
            .info_obj(self);
        return None
    }

    pub fn get_instruction_span(&self, instr_id : FlatID) -> Span {
        match &self.instructions[instr_id] {
            Instruction::SubModule(sm) => sm.module_name_span,
            Instruction::FuncCall(fc) => fc.whole_func_span,
            Instruction::Declaration(decl) => decl.get_span(),
            Instruction::Wire(w) => w.span,
            Instruction::Write(conn) => conn.to_span,
            Instruction::IfStatement(if_stmt) => self.get_instruction_span(if_stmt.condition),
            Instruction::ForStatement(for_stmt) => self.get_instruction_span(for_stmt.loop_var_decl),
        }
    }

    /// This function is intended to retrieve a known port while walking the syntax tree. panics if the port doesn't exist
    pub fn get_port_by_decl_span(&self, span : Span) -> PortID {
        for (id, data) in &self.ports {
            if data.decl_span == span {
                return id
            }
        }
        unreachable!()
    }

    pub fn is_multi_domain(&self) -> bool {
        self.domains.len() > 1
    }
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
    pub fn get_submodule_interface_domain(&self, domain : DomainID) -> &'linker DomainInfo {
        let local_domain = self.local_domain_map[domain];
        &self.domains[local_domain]
    }
}


#[derive(Debug,Clone,Copy,PartialEq,Eq)]
pub enum IdentifierType {
    Input,
    Output,
    Local,
    State,
    Generative
}

impl IdentifierType {
    pub fn get_keyword(&self) -> &'static str {
        match self {
            IdentifierType::Input => "input",
            IdentifierType::Output => "output",
            IdentifierType::Local => "",
            IdentifierType::State => "state",
            IdentifierType::Generative => "gen",
        }
    }
    pub fn is_generative(&self) -> bool {
        *self == IdentifierType::Generative
    }
    pub fn is_port(&self) -> bool {
        *self == IdentifierType::Input || *self == IdentifierType::Output
    }
    #[track_caller]
    pub fn unwrap_is_input(&self) -> bool {
        match self {
            IdentifierType::Input => true,
            IdentifierType::Output => false,
            _ => unreachable!()
        }
    }
}


#[derive(Debug)]
pub struct Port {
    pub name : String,
    pub name_span : Span,
    pub decl_span : Span,
    pub identifier_type : IdentifierType,
    pub interface : DomainID,
    /// This is only set after flattening is done. Initially just [UUID::PLACEHOLDER]
    pub declaration_instruction : FlatID
}

#[derive(Debug)]
pub struct Interface {
    pub name_span : Span,
    pub name : String,
    pub func_call_inputs : PortIDRange,
    pub func_call_outputs : PortIDRange
}

pub struct FlatIDMarker;
impl UUIDMarker for FlatIDMarker {const DISPLAY_NAME : &'static str = "obj_";}
pub type FlatID = UUID<FlatIDMarker>;

pub type FlatIDRange = UUIDRange<FlatIDMarker>;

pub struct PortIDMarker;
impl UUIDMarker for PortIDMarker {const DISPLAY_NAME : &'static str = "port_";}
pub type PortID = UUID<PortIDMarker>;

pub type PortIDRange = UUIDRange<PortIDMarker>;

pub struct DomainIDMarker;
impl UUIDMarker for DomainIDMarker {const DISPLAY_NAME : &'static str = "port_";}
/// Interfaces are also indexed using DomainIDs. But in general, these refer to (clock/latency counting) domains
pub type DomainID = UUID<DomainIDMarker>;


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
            WireReferenceRoot::SubModulePort(port) => Some(port.submodule_flat),
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
    /// Even this can be implicit. In the inline function call instantiation syntax there's no named submodule. my_mod(a, b, c)
    pub submodule_name_span : Option<Span>,
    pub submodule_flat : FlatID,
    pub port : PortID,
    /// Only set if the port is named as an explicit field. If the port name is implicit, such as in the function call syntax, then it is not present. 
    pub port_name_span : Option<Span>,
    pub port_identifier_typ : IdentifierType
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
    /// Enumerates all instructions that this instruction depends on. This includes (maybe compiletime) wires, and submodules. 
    pub fn for_each_dependency<F : FnMut(FlatID)>(&self, mut func : F) {
        match self {
            WireSource::WireRef(wire_ref) => {
                match &wire_ref.root {
                    WireReferenceRoot::LocalDecl(decl_id, _) => func(*decl_id),
                    WireReferenceRoot::NamedConstant(_, _) => {}
                    WireReferenceRoot::SubModulePort(submod_port) => func(submod_port.submodule_flat),
                }
                for p in &wire_ref.path {
                    match p {
                        WireReferencePathElement::ArrayAccess { idx, bracket_span:_ } => func(*idx),
                    }
                }
            }
            &WireSource::UnaryOp { op:_, right } => {func(right)}
            &WireSource::BinaryOp { op:_, left, right } => {func(left); func(right)},
            WireSource::Constant(_) => {}
        }
    }
    pub const fn new_error() -> WireSource {
        WireSource::Constant(Value::Error)
    }
}

#[derive(Debug, Clone)]
pub enum WrittenType {
    Error(Span),
    Named(Span, TypeUUID),
    Array(Span, Box<(WrittenType, FlatID, BracketSpan)>)
}

impl WrittenType {
    pub fn get_span(&self) -> Span {
        match self {
            WrittenType::Error(span) | WrittenType::Named(span, _) | WrittenType::Array(span, _) => *span
        }
    }

    pub fn to_type(&self) -> AbstractType {
        match self {
            WrittenType::Error(_) => AbstractType::Error,
            WrittenType::Named(_, id) => AbstractType::Named(*id),
            WrittenType::Array(_, arr_box) => {
                let (elem_typ, _arr_idx, _br_span) = arr_box.deref();
                AbstractType::Array(Box::new(elem_typ.to_type()))
            }
        }
    }

    pub fn for_each_generative_input<F : FnMut(FlatID)>(&self, mut f : F) {
        match self {
            WrittenType::Error(_) | WrittenType::Named(_, _) => {}
            WrittenType::Array(_span, arr_box) => {
                f(arr_box.deref().1)
            }
        }
    }

    pub fn to_string<TypVec : Index<TypeUUID, Output = NamedType>>(&self, linker_types : &TypVec) -> String {
        match self {
            WrittenType::Error(_) => {
                "{error}".to_owned()
            }
            WrittenType::Named(_, id) => {
                linker_types[*id].get_full_name()
            }
            WrittenType::Array(_, sub) => sub.deref().0.to_string(linker_types) + "[]",
        }
    }
}


#[derive(Debug)]
pub struct Declaration {
    pub typ_expr : WrittenType,
    pub typ : FullType,
    pub name_span : Span,
    pub name : String,
    /// Variables are read_only when they may not be controlled by the current block of code. 
    /// This is for example, the inputs of the current module, or the outputs of nested modules. 
    /// But could also be the iterator of a for loop. 
    pub read_only : bool,
    /// If the program text already covers the write, then lsp stuff on this declaration shouldn't use it. 
    pub declaration_itself_is_not_written_to : bool,
    pub identifier_type : IdentifierType,
    pub latency_specifier : Option<FlatID>,
    pub documentation : Documentation
}

impl Declaration {
    pub fn get_span(&self) -> Span {
        Span::new_overarching(self.typ_expr.get_span(), self.name_span).debug()
    }
}

#[derive(Debug)]
pub struct SubModuleInstance {
    pub module_uuid : ModuleUUID,
    pub module_name_span : Span,
    /// Name is not always present in source code. Such as in inline function call syntax: my_mod(a, b, c)
    pub name : Option<(String, Span)>,
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
pub struct FuncCallInstruction {
    pub submodule_instruction : FlatID,
    pub module_uuid : ModuleUUID,
    /// arguments.len() == func_call_inputs.len() ALWAYS
    pub arguments : Vec<FlatID>,
    /// arguments.len() == func_call_inputs.len() ALWAYS
    pub func_call_inputs : PortIDRange,
    pub func_call_outputs : PortIDRange,
    /// If this is None, that means the submodule was declared implicitly. Hence it could also be used at compiletime
    pub name_span : Option<Span>,
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


#[derive(Debug, Clone)]
pub enum ModuleOrWrittenType {
    WrittenType(WrittenType),
    Module(Span, ModuleUUID)
}
