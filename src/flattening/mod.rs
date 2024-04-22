
pub mod name_context;
pub mod initialization;
pub mod typechecking;

use std::{iter::zip, str::FromStr};

use num::BigInt;
use sus_proc_macro::{field, kind, kw};
use crate::{
    arena_alloc::{ArenaAllocator, FlatAlloc, UUIDMarker, UUIDRange, UUID},
    errors::{error_info, ErrorCollector, ErrorInfo},
    file_position::{BracketSpan, Span},
    instantiation::InstantiationList,
    linker::{ConstantUUID, FileUUID, GlobalResolver, LinkInfo, Linker, ModuleUUID, NameElem, NamedConstant, NamedType, ResolvedGlobals, ResolvedNameElem, TypeUUIDMarker},
    parser::{Cursor, Documentation},
    typing::{get_binary_operator_types, typecheck, typecheck_is_array_indexer, typecheck_unary_operator, AbstractType, WrittenType, BOOL_TYPE, INT_TYPE},
    value::Value
};

use self::{initialization::{ModulePorts, PortIDMarker}, name_context::LocalVariableContext};



use core::ops::Range;

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
}

#[derive(Debug, Clone)]
pub struct InterfacePorts<ID : Clone + Copy> {
    pub outputs_start : usize,
    pub ports : Box<[ID]>
}

impl<ID : Clone + Copy> InterfacePorts<ID> {
    pub fn empty() -> Self {
        InterfacePorts{outputs_start : 0, ports : Box::new([])}
    }

    // Todo, just treat all inputs and outputs as function call interface
    pub fn func_call_syntax_inputs(&self) -> Range<usize> {
        0..self.outputs_start
    }
    pub fn func_call_syntax_outputs(&self) -> Range<usize> {
        self.outputs_start..self.ports.len()
    }
    pub fn inputs(&self) -> &[ID] {
        &self.ports[..self.outputs_start]
    }
    pub fn outputs(&self) -> &[ID] {
        &self.ports[self.outputs_start..]
    }

    pub fn map<OtherID : Clone + Copy, MapFn : FnMut(ID, /*is_input : */bool) -> OtherID>(&self, f : &mut MapFn) -> InterfacePorts<OtherID> {
        InterfacePorts{
            ports : self.ports.iter().enumerate().map(|(idx, v)| f(*v, idx < self.outputs_start)).collect(),
            outputs_start : self.outputs_start
        }
    }
    pub fn iter(&self) -> impl Iterator<Item = (ID, /*is_input : */bool)> + '_ {
        self.ports.iter().enumerate().map(|(idx, v)| (*v, idx < self.outputs_start))
    }
}

#[derive(Debug)]
pub struct Module {
    pub link_info : LinkInfo,

    pub parsing_errors : ErrorCollector,

    pub module_ports : ModulePorts,

    pub flattened : FlattenedModule,

    pub instantiations : InstantiationList
}

impl Module {
    #[allow(dead_code)]
    pub fn print_flattened_module(&self) {
        println!("[[{}]]:", self.link_info.name);
        println!("Interface:");
        for (port, is_input) in self.flattened.interface_ports.iter() {
            let port_direction = if is_input {"input"} else {"output"};
            let port_name = &self.flattened.instructions[port].extract_wire_declaration().name;
            println!("    {port_direction} {port_name} -> {:?}", port);
        }
        println!("Instantiations:");
        for (id, inst) in &self.flattened.instructions {
            println!("    {:?}: {:?}", id, inst);
        }
    }
}


#[derive(Debug,Clone,Copy,PartialEq,Eq,Hash)]
pub struct FlatIDMarker;
impl UUIDMarker for FlatIDMarker {const DISPLAY_NAME : &'static str = "obj_";}
pub type FlatID = UUID<FlatIDMarker>;

pub type FlatIDRange = UUIDRange<FlatIDMarker>;

#[derive(Debug)]
pub enum ConnectionWritePathElement {
    ArrayIdx{idx : FlatID, bracket_span : BracketSpan},
    //ModulePort{id : PortID, name_span : Span}
}
#[derive(Debug)]
pub enum ConnectionWritePathElementComputed {
    ArrayIdx(usize),
    //ModulePort(PortID)
}

// These are assignable connections
#[derive(Debug)]
pub struct ConnectionWrite {
    pub root : FlatID,
    pub root_span : Span,
    pub path : Vec<ConnectionWritePathElement>,
    pub span : Span,
    pub is_declared_in_this_module : bool,
    pub write_modifiers : WriteModifiers
}

#[derive(Debug)]
pub enum WriteModifiers {
    Connection{num_regs : i64, regs_span : Span},
    Initial{initial_kw_span : Span}
}

#[derive(Debug)]
pub struct Write {
    pub from : FlatID,
    pub to : ConnectionWrite
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
impl UnaryOperator {
    pub fn from_kind_id(kind_id : u16) -> Self {
        match kind_id {
            kw!("+") => UnaryOperator::Sum,
            kw!("*") => UnaryOperator::Product,
            kw!("-") => UnaryOperator::Negate,
            kw!("&") => UnaryOperator::And,
            kw!("|") => UnaryOperator::Or,
            kw!("^") => UnaryOperator::Xor,
            kw!("!") => UnaryOperator::Not,
            _ => unreachable!()
        }
    }
    pub fn op_text(&self) -> &'static str {
        match self {
            UnaryOperator::And => "&",
            UnaryOperator::Or => "|",
            UnaryOperator::Xor => "^",
            UnaryOperator::Not => "!",
            UnaryOperator::Sum => "+",
            UnaryOperator::Product => "*",
            UnaryOperator::Negate => "-",
        }
    }
}
impl core::fmt::Display for UnaryOperator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.op_text())
    }
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
impl BinaryOperator {
    pub fn from_kind_id(kind_id : u16) -> Self {
        match kind_id {
            kw!("&") => BinaryOperator::And,
            kw!("|") => BinaryOperator::Or,
            kw!("^") => BinaryOperator::Xor,
            //kw!("<<") => BinaryOperator::ShiftLeft,
            //kw!(">>") => BinaryOperator::ShiftRight,
            kw!("+") => BinaryOperator::Add,
            kw!("-") => BinaryOperator::Subtract,
            kw!("*") => BinaryOperator::Multiply,
            kw!("/") => BinaryOperator::Divide,
            kw!("%") => BinaryOperator::Modulo,
            kw!("==") => BinaryOperator::Equals,
            kw!("!=") => BinaryOperator::NotEquals,
            kw!(">") => BinaryOperator::Greater,
            kw!(">=") => BinaryOperator::GreaterEq,
            kw!("<") => BinaryOperator::Lesser,
            kw!("<=") => BinaryOperator::LesserEq,
            _ => unreachable!()
        }
    }
    pub fn op_text(&self) -> &'static str {
        match self {
            BinaryOperator::And => "&",
            BinaryOperator::Or => "|",
            BinaryOperator::Xor => "^",
            //BinaryOperator::ShiftLeft => "<<",
            //BinaryOperator::ShiftRight => ">>",
            BinaryOperator::Add => "+",
            BinaryOperator::Subtract => "-",
            BinaryOperator::Multiply => "*",
            BinaryOperator::Divide => "/",
            BinaryOperator::Modulo => "%",
            BinaryOperator::Equals => "==",
            BinaryOperator::NotEquals => "!=",
            BinaryOperator::Greater => ">",
            BinaryOperator::GreaterEq => ">=",
            BinaryOperator::Lesser => "<",
            BinaryOperator::LesserEq => "<=",
        }
    }
}
impl core::fmt::Display for BinaryOperator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.op_text())
    }
}

#[derive(Debug)]
pub enum WireSource {
    WireRead(FlatID), // Used to add a span to the reference of a wire. 
    UnaryOp{op : UnaryOperator, right : FlatID},
    BinaryOp{op : BinaryOperator, left : FlatID, right : FlatID},
    ArrayAccess{arr : FlatID, arr_idx : FlatID, bracket_span : BracketSpan},
    Constant(Value),
    NamedConstant(ConstantUUID),
}

impl WireSource {
    pub fn for_each_input_wire<F : FnMut(FlatID)>(&self, func : &mut F) {
        match self {
            &WireSource::WireRead(from_wire) => {func(from_wire)}
            &WireSource::UnaryOp { op:_, right } => {func(right)}
            &WireSource::BinaryOp { op:_, left, right } => {func(left); func(right)}
            &WireSource::ArrayAccess { arr, arr_idx, bracket_span:_ } => {func(arr); func(arr_idx)}
            WireSource::Constant(_) => {}
            WireSource::NamedConstant(_) => {}
        }
    }
}

const IS_GEN_UNINIT : bool = false;

#[derive(Debug)]
pub struct WireInstance {
    pub typ : AbstractType,
    pub is_compiletime : bool,
    pub span : Span,
    pub is_declared_in_this_module : bool,
    pub source : WireSource
}

#[derive(Debug)]
pub struct Declaration {
    pub typ_expr : WrittenType,
    pub typ : AbstractType,
    pub is_declared_in_this_module : bool,
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
    pub fn make_declared_here(&self, file : FileUUID) -> ErrorInfo {
        error_info(Span::new_overarching(self.typ_expr.get_span(), self.name_span), file, "Declared here")
    }
}

#[derive(Debug)]
pub struct SubModuleInstance {
    pub module_uuid : ModuleUUID,
    pub name : String,
    pub module_name_span : Span,
    pub is_declared_in_this_module : bool,
    pub interface_ports : InterfacePorts<FlatID>
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
    Declaration(Declaration),
    Wire(WireInstance),
    Write(Write),
    IfStatement(IfStatement),
    ForStatement(ForStatement)
}

impl Instruction {
    #[track_caller]
    pub fn extract_wire(&self) -> &WireInstance {
        let Self::Wire(w) = self else {panic!("extract_wire on not a wire! Found {self:?}")};
        w
    }
    #[track_caller]
    pub fn extract_wire_declaration(&self) -> &Declaration {
        let Self::Declaration(w) = self else {panic!("extract_wire on not a WireDeclaration! Found {self:?}")};
        w
    }
    #[track_caller]
    pub fn extract_submodule(&self) -> &SubModuleInstance {
        let Self::SubModule(sm) = self else {panic!("extract_wire on not a SubModule! Found {self:?}")};
        sm
    }
    #[track_caller]
    pub fn extract_write(&self) -> &Write {
        let Self::Write(sm) = self else {panic!("extract_write on not a Write! Found {self:?}")};
        sm
    }

    pub fn for_each_embedded_type<F : FnMut(&AbstractType, Span)>(&self, f : &mut F) {
        match self {
            Instruction::SubModule(_) | Instruction::Write(_) | Instruction::IfStatement(_) | Instruction::ForStatement(_) => {}
            Instruction::Declaration(decl) => {
                f(&decl.typ, decl.typ_expr.get_span());
            }
            Instruction::Wire(w) => {
                f(&w.typ, w.span);
            }
        }
    }

    pub fn get_location_of_module_part(&self) -> Option<Span> {
        match self {
            Instruction::SubModule(sm) => sm.is_declared_in_this_module.then_some(sm.module_name_span),
            Instruction::Declaration(decl) => decl.is_declared_in_this_module.then_some(decl.name_span),
            Instruction::Wire(w) => w.is_declared_in_this_module.then_some(w.span),
            Instruction::Write(conn) => conn.to.is_declared_in_this_module.then_some(conn.to.span),
            Instruction::IfStatement(_) | Instruction::ForStatement(_) => None
        }
    }
}


#[derive(Debug, Clone)]
pub enum ModuleOrWrittenType {
    WrittenType(WrittenType),
    Module(Span, ModuleUUID)
}

enum LocalOrGlobal<'l, 'e> {
    Local(FlatID),
    Global(ResolvedNameElem<'l, 'e>)
}

impl<'l, 'e> LocalOrGlobal<'l, 'e> {
    fn expect_local(self, context : &str) -> Option<FlatID> {
        match self {
            LocalOrGlobal::Local(local) => Some(local),
            LocalOrGlobal::Global(global) => {
                global.errors.error_basic(global.span, format!("Can only use local variables in {context}!"));
                None
            }
        }
    }
}

struct FlatteningContext<'l> {
    instructions : FlatAlloc<Instruction, FlatIDMarker>,
    port_map : FlatAlloc<FlatID, PortIDMarker>,
    errors : ErrorCollector,
    is_declared_in_this_module : bool,

    local_variable_context : LocalVariableContext<'l, FlatID>,
    linker : GlobalResolver<'l>
}

impl<'l> FlatteningContext<'l> {
    fn resolve_identifier(&self, identifier_span : Span) -> LocalOrGlobal {
        // Possibly local
        let name_text = &self.linker.file.file_text[identifier_span];
        if let Some(decl_id) = self.local_variable_context.get_declaration_for(name_text) {
            return LocalOrGlobal::Local(decl_id);
        }
        // Global identifier
        LocalOrGlobal::Global(self.linker.resolve_global(identifier_span, &self.errors))
    }

    fn flatten_array_type(&mut self, span : Span, cursor : &mut Cursor<'l>) -> WrittenType {
        cursor.go_down(kind!("array_type"), |cursor| {
            cursor.field(field!("arr"));
            let array_element_type = self.flatten_type(cursor);

            cursor.field(field!("arr_idx"));
            let (array_size_wire_id, bracket_span) = self.flatten_array_bracket(cursor);
            
            WrittenType::Array(span, Box::new((array_element_type, array_size_wire_id, bracket_span)))
        })
    }
    
    fn flatten_type(&mut self, cursor : &mut Cursor<'l>) -> WrittenType {
        let (kind, span) = cursor.kind_span();
        if kind == kind!("global_identifier") {
            if let Some(typ_id) = &self.linker.resolve_global(span, &self.errors).expect_type() {
                WrittenType::Named(span, *typ_id)
            } else {
                WrittenType::Error(span)
            }
        } else if kind == kind!("array_type") {
            self.flatten_array_type(span, cursor)
        } else {cursor.could_not_match()}
    }

    fn flatten_module_or_type<const ALLOW_MODULES : bool>(&mut self, cursor : &mut Cursor<'l>) -> ModuleOrWrittenType {
        let (kind, span) = cursor.kind_span();
        // Only difference is that 
        if kind == kind!("global_identifier") {
            let found_global = self.linker.resolve_global(span, &self.errors);
            match &found_global.name_elem {
                Some(NameElem::Type(typ_id)) => ModuleOrWrittenType::WrittenType(WrittenType::Named(span, *typ_id)),
                Some(NameElem::Module(md)) if ALLOW_MODULES => ModuleOrWrittenType::Module(span, *md),
                Some(_) => {
                    let accepted_text = if ALLOW_MODULES {"Type or Module"} else {"Type"};
                    found_global.not_expected_global_error(accepted_text);
                    ModuleOrWrittenType::WrittenType(WrittenType::Error(span))
                }
                None => ModuleOrWrittenType::WrittenType(WrittenType::Error(span)) // Non existent global already covered by Linker
            }
        } else if kind == kind!("array_type") {
            ModuleOrWrittenType::WrittenType(self.flatten_array_type(span, cursor))
        } else {cursor.could_not_match()}
    }

    fn flatten_declaration<const ALLOW_MODULES : bool, const ALLOW_MODIFIERS : bool>(&mut self, fallback_identifier_type : IdentifierType, read_only : bool, declaration_itself_is_not_written_to : bool, cursor : &mut Cursor<'l>) -> FlatID {
        cursor.go_down(kind!("declaration"), |cursor| {
            let identifier_type = if cursor.optional_field(field!("declaration_modifiers")) {
                let (modifier_kind, modifier_span) = cursor.kind_span();

                if !ALLOW_MODIFIERS {
                    self.errors.error_basic(modifier_span, "Inputs and outputs of a module cannot be decorated with 'state' or 'gen'");
                    fallback_identifier_type
                } else {
                    if modifier_kind == kw!("state") {
                        IdentifierType::State
                    } else if modifier_kind == kw!("gen") {
                        IdentifierType::Generative
                    } else {
                        cursor.could_not_match()
                    }
                }
            } else {fallback_identifier_type};
            
            cursor.field(field!("type"));
            let typ_or_module_expr = self.flatten_module_or_type::<ALLOW_MODULES>(cursor);
            
            let name_span = cursor.field_span(field!("name"), kind!("identifier"));
    
            let span_latency_specifier = if cursor.optional_field(field!("latency_specifier")) {
                cursor.go_down_content(kind!("latency_specifier"), 
                    |cursor| Some((self.flatten_expr(cursor), cursor.span()))
            )} else {None};
            // Parsing components done

            let typ_expr = match typ_or_module_expr {
                ModuleOrWrittenType::WrittenType(typ) => {
                    typ
                }
                ModuleOrWrittenType::Module(span, md_id) => {
                    assert!(ALLOW_MODULES);
                    if let Some((_, span)) = span_latency_specifier {
                        self.errors.error_basic(span, "Cannot add latency specifier to module instances");
                    }
                    let md = self.linker.get_module(md_id);
                    return self.alloc_module_interface(self.linker.file.file_text[name_span].to_owned(), md, md_id, span)
                }
            };

            let typ_expr_span = typ_expr.get_span();
            let name = &self.linker.file.file_text[name_span];
            let documentation = cursor.extract_gathered_comments();

            let inst_id = self.instructions.alloc(Instruction::Declaration(Declaration{
                typ : typ_expr.to_type(),
                typ_expr,
                is_declared_in_this_module : self.is_declared_in_this_module,
                read_only,
                declaration_itself_is_not_written_to,
                identifier_type,
                name : name.to_owned(),
                name_span,
                latency_specifier : span_latency_specifier.map(|(ls, _)| ls),
                documentation
            }));

            if let Err(conflict) = self.local_variable_context.add_declaration(name, inst_id) {
                self.errors.error_with_info(Span::new_overarching(typ_expr_span, name_span), "This declaration conflicts with a previous declaration in the same scope", vec![self.instructions[conflict].extract_wire_declaration().make_declared_here(self.errors.file)])
            }

            inst_id
        })
    }

    fn flatten_array_bracket(&mut self, cursor : &mut Cursor<'l>) -> (FlatID, BracketSpan) {
        cursor.go_down_content(kind!("array_bracket_expression"), 
            |cursor| (self.flatten_expr(cursor), BracketSpan::from_outer(cursor.span()))
        )
    }

    fn desugar_func_call(&mut self, cursor : &mut Cursor<'l>) -> Option<(&Module, InterfacePorts<FlatID>)> {
        let whole_function_span = cursor.span();
        cursor.go_down(kind!("func_call"), |cursor| {
            cursor.field(field!("name"));
            let instantiation_flat_id = self.get_module_by_global_identifier(cursor);

            cursor.field(field!("arguments"));
            let arguments_span = BracketSpan::from_outer(cursor.span());
            let arguments = cursor.collect_list(kind!("parenthesis_expression_list"), |cursor| {
                self.flatten_expr(cursor)
            });

            let func_instantiation = self.instructions[instantiation_flat_id?].extract_submodule();

            let md = self.linker.get_module(func_instantiation.module_uuid);

            let submodule_local_wires = func_instantiation.interface_ports.clone();
            
            let inputs = submodule_local_wires.func_call_syntax_inputs();

            let arg_count = arguments.len();
            let expected_arg_count = inputs.len();

            let mut args = arguments.as_slice();
            
            if arg_count != expected_arg_count {
                let module_info = vec![error_info(md.link_info.span, md.link_info.file, "Interface defined here")];
                if arg_count > expected_arg_count {
                    // Too many args, complain about excess args at the end
                    let excess_args_span = Span::new_overarching(self.instructions[args[expected_arg_count]].extract_wire().span, self.instructions[*args.last().unwrap()].extract_wire().span);
                    
                    self.errors.error_with_info(excess_args_span, format!("Excess argument. Function takes {expected_arg_count} args, but {arg_count} were passed."), module_info);
                    // Shorten args to still get proper type checking for smaller arg array
                    args = &args[..expected_arg_count];
                } else {
                    // Too few args, mention missing argument names
                    self.errors.error_with_info(arguments_span.close_bracket(), format!("Too few arguments. Function takes {expected_arg_count} args, but {arg_count} were passed."), module_info);
                }
            }

            for (field, arg_read_side) in zip(inputs, args) {
                let arg_wire = self.instructions[*arg_read_side].extract_wire();
                let func_input_port = &submodule_local_wires.ports[field];
                self.instructions.alloc(Instruction::Write(Write{from: *arg_read_side, to: ConnectionWrite{root : *func_input_port, root_span : whole_function_span, path : Vec::new(), span : arg_wire.span, is_declared_in_this_module : self.is_declared_in_this_module, write_modifiers : WriteModifiers::Connection{num_regs : 0, regs_span : arg_wire.span.empty_span_at_front()}}}));
            }

            Some((md, submodule_local_wires))
        })
    }

    /// Produces a new [SubModuleInstance] if a global was passed, or a reference to the existing instance if it's referenced by name
    fn get_module_by_global_identifier(&mut self, cursor : &mut Cursor<'l>) -> Option<FlatID> {
        let (kind, span) = cursor.kind_span();
        if kind == kind!("global_identifier") {
            cursor.go_down(kind!("global_identifier"), |_cursor| {
                match self.resolve_identifier(span) {
                    LocalOrGlobal::Local(id) => {
                        if let Instruction::SubModule(_) = &self.instructions[id] {
                            Some(id)
                        } else {
                            let decl = self.instructions[id].extract_wire_declaration();
                            self.errors.error_with_info(span, "Function call syntax is only possible on modules", vec![decl.make_declared_here(self.errors.file)]);
                            None
                        }
                    }
                    LocalOrGlobal::Global(global) => {
                        if let Some(module_id) = global.expect_module() {
                            let md = &self.linker.get_module(module_id);
                            Some(self.alloc_module_interface(md.link_info.name.clone(), md, module_id, span))
                        } else {
                            None
                        }
                    }
                }
            })
        } else {
            self.errors.error_basic(span, "Module name may not be an expression");
            None
        }
    }

    fn flatten_expr(&mut self, cursor : &mut Cursor<'l>) -> FlatID {
        let (kind, expr_span) = cursor.kind_span();
        
        let source = if kind == kind!("global_identifier") {
            // TODO add namespacing
            match self.resolve_identifier(expr_span) {
                LocalOrGlobal::Local(id) => {
                    WireSource::WireRead(id)
                }
                LocalOrGlobal::Global(global) => {
                    if let Some(cst) = global.expect_constant() {
                        WireSource::NamedConstant(cst)
                    } else {
                        WireSource::Constant(Value::Error)
                    }
                }
            }
        } else if kind == kind!("number") {
            let text = &self.linker.file.file_text[expr_span];
            WireSource::Constant(Value::Integer(BigInt::from_str(text).unwrap()))
        } else if kind == kind!("unary_op") {
            cursor.go_down_no_check(|cursor| {
                cursor.field(field!("operator"));
                let op = UnaryOperator::from_kind_id(cursor.kind());
                
                cursor.field(field!("right"));
                let right = self.flatten_expr(cursor);

                WireSource::UnaryOp{op, right}
            })
        } else if kind == kind!("binary_op") {
            cursor.go_down_no_check(|cursor| {
                cursor.field(field!("left"));
                let left = self.flatten_expr(cursor);

                cursor.field(field!("operator"));
                let op = BinaryOperator::from_kind_id(cursor.kind());

                cursor.field(field!("right"));
                let right = self.flatten_expr(cursor);

                WireSource::BinaryOp{op, left, right}
            })
        } else if kind == kind!("array_op") {
            cursor.go_down_no_check(|cursor| {
                cursor.field(field!("arr"));
                let arr = self.flatten_expr(cursor);
                
                cursor.field(field!("arr_idx"));
                let (arr_idx, bracket_span) = self.flatten_array_bracket(cursor);
                
                WireSource::ArrayAccess{arr, arr_idx, bracket_span}
            })
        } else if kind == kind!("func_call") {
            if let Some((md, interface_wires)) = self.desugar_func_call(cursor) {
                let output_range = interface_wires.func_call_syntax_outputs();

                if output_range.len() != 1 {
                    let info = error_info(md.link_info.span, md.link_info.file, "Module Defined here");
                    self.errors.error_with_info(expr_span, "A function called in this context may only return one result. Split this function call into a separate line instead.", vec![info]);
                }

                if output_range.len() >= 1 {
                    return interface_wires.ports[output_range.start];
                }
            }
            // Function desugaring or using threw an error
            WireSource::Constant(Value::Error)
        } else if kind == kind!("parenthesis_expression") {
            return cursor.go_down_content(kind!("parenthesis_expression"), |cursor| self.flatten_expr(cursor));
        } else if kind == kind!("field_access") {
            cursor.go_down_no_check(|cursor| {
                cursor.field(field!("left"));
                if let Some(instr_id) = self.get_module_by_global_identifier(cursor) {
                    let submodule = self.instructions[instr_id].extract_submodule();

                    //submodule.module_uuid
                }
            });
            println!("TODO: Field access");
            WireSource::Constant(Value::Error)
        } else {
            cursor.could_not_match();
        };

        let wire_instance = WireInstance{
            typ : AbstractType::Unknown,
            is_compiletime : IS_GEN_UNINIT,
            span: expr_span,
            source,
            is_declared_in_this_module : self.is_declared_in_this_module
        };
        self.instructions.alloc(Instruction::Wire(wire_instance))
    }
    fn flatten_assignable_expr(&mut self, write_modifiers : WriteModifiers, cursor : &mut Cursor<'l>) -> Option<ConnectionWrite> {
        let (kind, span) = cursor.kind_span();
        if kind == kind!("global_identifier") {
            let root = self.resolve_identifier(span).expect_local("assignments")?;

            Some(ConnectionWrite{root, root_span : span, path : Vec::new(), span, is_declared_in_this_module : self.is_declared_in_this_module, write_modifiers})
        } else if kind == kind!("array_op") {
            cursor.go_down_no_check(|cursor| {
                cursor.field(field!("arr"));
                let flattened_arr_expr_opt = self.flatten_assignable_expr(write_modifiers, cursor);
                
                cursor.field(field!("arr_idx"));
                let (idx, bracket_span) = self.flatten_array_bracket(cursor);
                
                let mut flattened_arr_expr = flattened_arr_expr_opt?; // only unpack the subexpr after flattening the idx, so we catch all errors
                
                flattened_arr_expr.path.push(ConnectionWritePathElement::ArrayIdx{idx, bracket_span});
                flattened_arr_expr.span = Span::new_overarching(flattened_arr_expr.span, bracket_span.outer_span());
                
                Some(flattened_arr_expr)
            })
        } else if kind == kind!("field_access") {
            cursor.go_down_no_check(|cursor| {
                cursor.field(field!("left"));
                let flattened_arr_expr_opt = self.flatten_assignable_expr(write_modifiers, cursor);
                
                cursor.field(field!("name"));
                //let (idx, bracket_span) = self.flatten_array_bracket(cursor);
                
                //let mut flattened_arr_expr = flattened_arr_expr_opt?; // only unpack the subexpr after flattening the idx, so we catch all errors
                
                println!("TODO: Field access in assign");

                return None
            })
        } else if kind == kind!("number") {self.errors.error_basic(span, "Cannot assign to constant"); None
        } else if kind == kind!("unary_op") {self.errors.error_basic(span, "Cannot assign to the result of an operator"); None
        } else if kind == kind!("binary_op") {self.errors.error_basic(span, "Cannot assign to the result of an operator"); None
        } else if kind == kind!("func_call") {self.errors.error_basic(span, "Cannot assign to submodule call"); None
        } else if kind == kind!("parenthesis_expression") {self.errors.error_basic(span, "Remove these parentheses"); None
        } else {cursor.could_not_match()}
    }

    fn flatten_if_statement(&mut self, cursor : &mut Cursor<'l>) {
        cursor.go_down(kind!("if_statement"), |cursor| {
            cursor.field(field!("condition"));
            let condition = self.flatten_expr(cursor);
            
            let if_id = self.instructions.alloc(Instruction::IfStatement(IfStatement{condition, then_start : UUID::PLACEHOLDER, then_end_else_start : UUID::PLACEHOLDER, else_end : UUID::PLACEHOLDER}));
            let then_start = self.instructions.get_next_alloc_id();
            
            cursor.field(field!("then_block"));
            self.flatten_code(cursor);

            let then_end_else_start = self.instructions.get_next_alloc_id();
            if cursor.optional_field(field!("else_block")) {
                if cursor.kind() == kind!("if_statement") {
                    self.flatten_if_statement(cursor); // Chained if statements
                } else {
                    self.flatten_code(cursor)
                }
            };
            let else_end = self.instructions.get_next_alloc_id();
            
            let Instruction::IfStatement(if_stmt) = &mut self.instructions[if_id] else {unreachable!()};
            if_stmt.then_start = then_start;
            if_stmt.then_end_else_start = then_end_else_start;
            if_stmt.else_end = else_end;
        })
    }

    fn flatten_assign_function_call(&mut self, to : Vec<Result<ConnectionWrite, Span>>, cursor : &mut Cursor<'l>) {
        let func_call_span = cursor.span();
        let to_iter = if let Some((md, interface)) = self.desugar_func_call(cursor) {
            let output_range = interface.func_call_syntax_outputs();
            let outputs = &interface.ports[output_range];

            fn get_span(v : &Result<ConnectionWrite, Span>) -> Span {
                match v {
                    Ok(wr) => wr.span,
                    Err(span) => *span,
                }
            }

            let num_func_outputs = outputs.len();
            let num_targets = to.len();
            if num_targets != num_func_outputs {
                let info = vec![error_info(md.link_info.span, md.link_info.file, "Module Defined here")];
                if num_targets > num_func_outputs {
                    let excess_results_span = Span::new_overarching(get_span(&to[num_func_outputs]), get_span(to.last().unwrap()));
                    self.errors.error_with_info(excess_results_span, format!("Excess output targets. Function returns {num_func_outputs} results, but {num_targets} targets were given."), info);
                } else {
                    self.errors.error_with_info(func_call_span, format!("Too few output targets. Function returns {num_func_outputs} results, but {num_targets} targets were given."), info);
                }
            }

            let mut to_iter = to.into_iter();
            for field in outputs {
                let module_port_wire_decl = self.instructions[*field].extract_wire_declaration();
                let module_port_proxy = self.instructions.alloc(Instruction::Wire(WireInstance{typ : module_port_wire_decl.typ.clone(), is_compiletime : IS_GEN_UNINIT, span : func_call_span, is_declared_in_this_module : self.is_declared_in_this_module, source : WireSource::WireRead(*field)}));
                
                if let Some(Ok(to)) = to_iter.next() {
                    self.instructions.alloc(Instruction::Write(Write{from: module_port_proxy, to}));
                }
            }
            to_iter
        } else {
            to.into_iter()
        };
        for leftover_to in to_iter {
            if let Ok(to) = leftover_to {
                let err_id = self.instructions.alloc(Instruction::Wire(WireInstance{typ : AbstractType::Error, is_compiletime : true, span : func_call_span, is_declared_in_this_module : self.is_declared_in_this_module, source : WireSource::Constant(Value::Error)}));
                self.instructions.alloc(Instruction::Write(Write{from: err_id, to}));
            }
        }
    }

    fn flatten_code(&mut self, cursor : &mut Cursor<'l>) {
        let old_frame = self.local_variable_context.new_frame();
        
        self.flatten_code_keep_context(cursor);

        self.local_variable_context.pop_frame(old_frame);
    }
    fn flatten_code_keep_context(&mut self, cursor : &mut Cursor<'l>) {
        cursor.clear_gathered_comments(); // Clear comments at the start of a block
        cursor.list(kind!("block"), |cursor| {
            let kind = cursor.kind();
            if kind == kind!("assign_left_side") {
                self.flatten_standalone_decls(cursor);
            } else if kind == kind!("decl_assign_statement") {
                cursor.go_down_no_check(|cursor| {
                    cursor.field(field!("assign_left"));
                    let to = self.flatten_assignment_left_side(cursor);
                    
                    cursor.field(field!("assign_value"));

                    let (node_kind, span) = cursor.kind_span();
                    
                    if node_kind == kind!("func_call") {
                        self.flatten_assign_function_call(to, cursor);
                    } else {
                        let read_side = self.flatten_expr(cursor);
                        
                        if to.len() != 1 {
                            self.errors.error_basic(span, format!("Non-function assignments must output exactly 1 output instead of {}", to.len()));
                        }
                        if let Some(Ok(to)) = to.into_iter().next() {
                            self.instructions.alloc(Instruction::Write(Write{from: read_side, to}));
                        }
                    }
                });
            } else if kind == kind!("block") {
                self.flatten_code(cursor);
            } else if kind == kind!("if_statement") {
                self.flatten_if_statement(cursor);
            } else if kind == kind!("for_statement") {
                cursor.go_down_no_check(|cursor| {
                    cursor.field(field!("for_decl"));
                    let loop_var_decl = self.flatten_declaration::<false, false>(IdentifierType::Generative, true, true, cursor);

                    cursor.field(field!("from"));
                    let start = self.flatten_expr(cursor);

                    cursor.field(field!("to"));
                    let end = self.flatten_expr(cursor);
                    
                    let for_id = self.instructions.alloc(Instruction::ForStatement(ForStatement{loop_var_decl, start, end, loop_body: UUIDRange(UUID::PLACEHOLDER, UUID::PLACEHOLDER)}));

                    let code_start = self.instructions.get_next_alloc_id();

                    cursor.field(field!("block"));
                    self.flatten_code(cursor);
                    
                    let code_end = self.instructions.get_next_alloc_id();

                    let Instruction::ForStatement(for_stmt) = &mut self.instructions[for_id] else {unreachable!()};

                    for_stmt.loop_body = UUIDRange(code_start, code_end);
                })
            } else if kind == kind!("interface_statement") {
                println!("TODO: Interface Statement");
            } else if kind == kind!("cross_statement") {
                println!("TODO: Cross Statement");
            } else {
                cursor.could_not_match()
            }
            cursor.clear_gathered_comments(); // Clear comments after every statement, so comments don't bleed over
        });
    }

    fn flatten_write_modifiers(&self, cursor : &mut Cursor<'l>) -> WriteModifiers {
        if cursor.optional_field(field!("write_modifiers")) {
            let modifiers_span = cursor.span();
            let mut initial_count = 0;
            let mut reg_count = 0;
            cursor.list(kind!("write_modifiers"), |cursor| {
                let kw_kind = cursor.kind();
                if kw_kind == kw!("reg") {
                    reg_count += 1;
                } else if kw_kind == kw!("initial") {
                    initial_count += 1;
                } else {
                    unreachable!()
                }
            });
            match (initial_count, reg_count) {
                (0, num_regs) => WriteModifiers::Connection{num_regs, regs_span : modifiers_span},
                (1, 0) => WriteModifiers::Initial{initial_kw_span : modifiers_span},
                _other => unreachable!()
            }
        } else {
            WriteModifiers::Connection { num_regs: 0, regs_span: cursor.span().empty_span_at_front()}
        }
    }

    /// See [Self::flatten_standalone_decls][]
    /// Two cases:
    /// - Left side of assignment:
    ///     No modules, Yes write modifiers, Only assignable expressions
    fn flatten_assignment_left_side(&mut self, cursor : &mut Cursor<'l>) -> Vec<Result<ConnectionWrite, Span>> {
        cursor.collect_list(kind!("assign_left_side"), |cursor| {
            cursor.go_down(kind!("assign_to"), |cursor| {
                let write_modifiers = self.flatten_write_modifiers(cursor);
                
                cursor.field(field!("expr_or_decl"));
                let (kind, span) = cursor.kind_span();

                if kind == kind!("declaration") {
                    let root = self.flatten_declaration::<false, true>(IdentifierType::Local, false, true, cursor);
                    let flat_root_decl = self.instructions[root].extract_wire_declaration();
                    Ok(ConnectionWrite{root, root_span : flat_root_decl.name_span, path: Vec::new(), span, is_declared_in_this_module: true, write_modifiers})
                } else { // It's _expression
                    self.flatten_assignable_expr(write_modifiers, cursor).ok_or(span)
                }
            })
        })
    }

    /// See [Self::flatten_assignment_left_side][]
    /// - Standalone declarations:
    ///     Yes modules, No write modifiers, Yes expressions (-> single expressions)
    fn flatten_standalone_decls(&mut self, cursor : &mut Cursor<'l>) {
        let mut is_first_item = true;
        cursor.list(kind!("assign_left_side"), |cursor| {
            cursor.go_down(kind!("assign_to"), |cursor| {
                if !is_first_item {
                    self.errors.warn_basic(cursor.span(), "Standalone declarations and expressions should be on their own line.");
                }
                is_first_item = false;

                if let Some(span) = cursor.optional_field_span(field!("write_modifiers"), kind!("write_modifiers")) {
                    self.errors.error_basic(span, "No write modifiers are allowed on non-assigned to declarations or expressions");
                }
                
                cursor.field(field!("expr_or_decl"));
                let (kind, span) = cursor.kind_span();

                if kind == kind!("declaration") {
                    let _ = self.flatten_declaration::<true, true>(IdentifierType::Local, false, true, cursor);
                } else { // It's _expression
                    if kind == kind!("func_call") {
                        self.flatten_assign_function_call(Vec::new(), cursor);
                    } else {
                        self.errors.warn_basic(span, "The result of this operation is not used");
                        let _ = self.flatten_expr(cursor);
                    }
                }
            });
        })
    }

    fn flatten_declaration_list(&mut self, identifier_type : IdentifierType, read_only : bool, ports : &mut Vec<FlatID>, cursor : &mut Cursor<'l>) {
        cursor.list(kind!("declaration_list"), |cursor| {
            let id = self.flatten_declaration::<false, false>(identifier_type, read_only, true, cursor);
            ports.push(id);
            self.port_map.alloc(id);
        });
    }

    fn flatten_interface_ports<const IS_SUBMODULE : bool>(&mut self, cursor : &mut Cursor<'l>) -> InterfacePorts<FlatID> {
        if cursor.optional_field(field!("interface_ports")) {
            cursor.go_down(kind!("interface_ports"), |cursor| {
                let mut ports = Vec::new();
                if cursor.optional_field(field!("inputs")) {
                    let identifier_type = if IS_SUBMODULE {IdentifierType::Local} else {IdentifierType::Input};
                    // Read only on inputs and outputs is obviously flipped for submodules, since we're looking at the other side of the in/outputs. 
                    self.flatten_declaration_list(identifier_type, !IS_SUBMODULE, &mut ports, cursor)
                }
                let outputs_start = ports.len();
                if cursor.optional_field(field!("outputs")) {
                    let identifier_type = if IS_SUBMODULE {IdentifierType::Local} else {IdentifierType::Output};
                    // Read only on inputs and outputs is obviously flipped for submodules, since we're looking at the other side of the in/outputs. 
                    self.flatten_declaration_list(identifier_type, IS_SUBMODULE, &mut ports, cursor)
                }
                InterfacePorts{ outputs_start, ports: ports.into_boxed_slice() }
            })
        } else {InterfacePorts::empty()}
    }

    fn alloc_module_interface(&mut self, name : String, module : &Module, module_uuid : ModuleUUID, typ_span : Span) -> FlatID {
        let local_linker = self.linker.new_sublinker(module.link_info.file);

        let mut nested_context = FlatteningContext {
            instructions: std::mem::replace(&mut self.instructions, FlatAlloc::new()),
            port_map: FlatAlloc::new(),
            errors: ErrorCollector::new(module.link_info.file, local_linker.file.file_text.len()), // Temporary ErrorCollector, unused
            is_declared_in_this_module: false,
            linker: local_linker,
            local_variable_context : LocalVariableContext::new_initial()
        };
        
        let mut nested_cursor = Cursor::new_for_node(&nested_context.linker.file.tree, &nested_context.linker.file.file_text, module.link_info.span, kind!("module"));

        let interface_ports = nested_cursor.go_down(kind!("module"), |nested_cursor| {
            nested_cursor.field(field!("name")); // Get past name field
            nested_context.flatten_interface_ports::<true>(nested_cursor)
        });
        
        self.linker.reabsorb_sublinker(nested_context.linker);

        self.instructions = nested_context.instructions;
        self.instructions.alloc(Instruction::SubModule(SubModuleInstance{
            name,
            module_uuid,
            is_declared_in_this_module : self.is_declared_in_this_module,
            module_name_span: typ_span,
            interface_ports
        }))
    }

    fn flatten_module(&mut self, cursor : &mut Cursor<'l>) -> InterfacePorts<FlatID> {
        cursor.go_down(kind!("module"), |cursor| {
            let name_span = cursor.field_span(field!("name"), kind!("identifier"));
            let module_name = &self.linker.file.file_text[name_span];
            println!("TREE SITTER module! {module_name}");
            // Interface is allocated in self
            let interface_found = self.flatten_interface_ports::<false>(cursor);
            cursor.field(field!("block"));
            self.flatten_code(cursor);
            interface_found
        })
    }
}

#[derive(Debug)]
pub struct FlattenedInterfacePort {
    pub wire_id : FlatID,
    pub port_name : String,
    pub span : Span
}

#[derive(Debug)]
pub struct FlattenedModule {
    pub instructions : FlatAlloc<Instruction, FlatIDMarker>,
    pub errors : ErrorCollector,
    pub interface_ports : InterfacePorts<FlatID>,
    pub resolved_globals : ResolvedGlobals,
    pub port_map : FlatAlloc<FlatID, PortIDMarker>
}

impl FlattenedModule {
    pub fn empty(errors : ErrorCollector) -> FlattenedModule {
        FlattenedModule {
            instructions : FlatAlloc::new(),
            errors,
            interface_ports : InterfacePorts::empty(),
            resolved_globals : ResolvedGlobals::new(),
            port_map : FlatAlloc::new()
        }
    }
    
    /*
    This method flattens all given code into a simple set of assignments, operators and submodules. 
    It already does basic type checking and assigns a type to every wire. 
    The Generating Structure of the code is not yet executed. 
    It is template-preserving
    */
    pub fn flatten(linker : &Linker, module : &Module) -> FlattenedModule {
        let global_resolver = GlobalResolver::new(linker, module.link_info.file);
        
        // The given span should correspond perfectly to this, so impossible we don't find the node. 
        let mut cursor = Cursor::new_for_node(&global_resolver.file.tree, &global_resolver.file.file_text, module.link_info.span, kind!("module"));

        let mut context = FlatteningContext{
            instructions : FlatAlloc::new(),
            port_map : FlatAlloc::with_capacity(module.module_ports.ports.len()),
            errors : ErrorCollector::new(module.link_info.file, global_resolver.file.file_text.len()),
            is_declared_in_this_module : true,
            linker : global_resolver,
            local_variable_context : LocalVariableContext::new_initial()
        };

        // Make sure that the gathered ports 
        assert_eq!(module.module_ports.ports.len(), context.port_map.len());
        for ((_, port), (_, id)) in zip(&module.module_ports.ports, &context.port_map) {
            let name_span = context.instructions[*id].extract_wire_declaration().name_span;
            assert_eq!(port.name_span, name_span);
        }

        // Temporary, switch to iterating over nodes in file itself when needed. 
        let interface_ports = context.flatten_module(&mut cursor);

        FlattenedModule {
            resolved_globals : context.linker.extract_resolved_globals(),
            errors : context.errors,
            instructions : context.instructions,
            interface_ports,
            port_map : context.port_map
        }
    }
}
