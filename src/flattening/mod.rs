
pub mod name_context;

use std::{iter::zip, str::FromStr};

use num::BigInt;
use crate::{
    arena_alloc::{ArenaAllocator, FlatAlloc, UUIDMarker, UUIDRange, UUID}, errors::{error_info, ErrorCollector, ErrorInfo}, file_position::{BracketSpan, Span}, instantiation::InstantiationList, linker::{ConstantUUID, FileUUID, GlobalResolver, LinkInfo, Linker, ModuleUUID, NameElem, NamedConstant, NamedType, ResolvedGlobals, ResolvedNameElem, TypeUUIDMarker}, parser::{Cursor, Documentation, SUS}, typing::{get_binary_operator_types, typecheck, typecheck_is_array_indexer, typecheck_unary_operator, Type, WrittenType, BOOL_TYPE, INT_TYPE}, value::Value
};

use self::name_context::LocalVariableContext;



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
    //StructField(FieldID)
}
#[derive(Debug)]
pub enum ConnectionWritePathElementComputed {
    ArrayIdx(usize)
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
    pub fn from_text(op_text : &str) -> Self {
        match op_text {
            "+" => UnaryOperator::Sum,
            "*" => UnaryOperator::Product,
            "-" => UnaryOperator::Negate,
            "&" => UnaryOperator::And,
            "|" => UnaryOperator::Or,
            "^" => UnaryOperator::Xor,
            "!" => UnaryOperator::Not,
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
    ShiftLeft,
    ShiftRight,
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
    pub fn from_text(op_text : &str) -> Self {
        match op_text {
            "&" => BinaryOperator::And,
            "|" => BinaryOperator::Or,
            "^" => BinaryOperator::Xor,
            "<<" => BinaryOperator::ShiftLeft,
            ">>" => BinaryOperator::ShiftRight,
            "+" => BinaryOperator::Add,
            "-" => BinaryOperator::Subtract,
            "*" => BinaryOperator::Multiply,
            "/" => BinaryOperator::Divide,
            "%" => BinaryOperator::Modulo,
            "==" => BinaryOperator::Equals,
            "!=" => BinaryOperator::NotEquals,
            ">" => BinaryOperator::Greater,
            ">=" => BinaryOperator::GreaterEq,
            "<" => BinaryOperator::Lesser,
            "<=" => BinaryOperator::LesserEq,
            _ => unreachable!()
        }
    }
    pub fn op_text(&self) -> &'static str {
        match self {
            BinaryOperator::And => "&",
            BinaryOperator::Or => "|",
            BinaryOperator::Xor => "^",
            BinaryOperator::ShiftLeft => "<<",
            BinaryOperator::ShiftRight => ">>",
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
    pub typ : Type,
    pub is_compiletime : bool,
    pub span : Span,
    pub is_declared_in_this_module : bool,
    pub source : WireSource
}

#[derive(Debug)]
pub struct Declaration {
    pub typ_expr : WrittenType,
    pub typ : Type,
    pub is_declared_in_this_module : bool,
    pub name_span : Span,
    pub name : Box<str>,
    pub read_only : bool,
    // If the program text already covers the write, then lsp stuff on this declaration shouldn't use it. 
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
    pub name : Box<str>,
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

    pub fn for_each_embedded_type<F : FnMut(&Type, Span)>(&self, f : &mut F) {
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
    errors : ErrorCollector,
    is_declared_in_this_module : bool,

    local_variable_context : LocalVariableContext<'l, FlatID>,
    linker : GlobalResolver<'l>,
    type_list_for_naming : &'l ArenaAllocator<NamedType, TypeUUIDMarker>
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

    fn flatten_array_type_tree(&mut self, span : Span, cursor : &mut Cursor<'l>) -> WrittenType {
        cursor.go_down(SUS.array_type_kind, |cursor| {
            let array_element_type = cursor.field(SUS.arr_field, |cursor| self.flatten_type_tree(cursor));
            
            let (array_size_wire_id, bracket_span) = cursor.field(SUS.arr_idx_field, |cursor| self.flatten_array_bracket_tree(cursor));
            
            WrittenType::Array(span, Box::new((array_element_type, array_size_wire_id, bracket_span)))
        })
    }
    
    fn flatten_type_tree(&mut self, cursor : &mut Cursor<'l>) -> WrittenType {
        let (kind, span) = cursor.kind_span();
        if kind == SUS.global_identifier_kind {
            if let Some(typ_id) = &self.linker.resolve_global(span, &self.errors).expect_type() {
                WrittenType::Named(span, *typ_id)
            } else {
                WrittenType::Error(span)
            }
        } else if kind == SUS.array_type_kind {
            self.flatten_array_type_tree(span, cursor)
        } else {cursor.could_not_match()}
    }

    fn flatten_module_or_type_tree<const ALLOW_MODULES : bool>(&mut self, cursor : &mut Cursor<'l>) -> ModuleOrWrittenType {
        let (kind, span) = cursor.kind_span();
        // Only difference is that 
        if kind == SUS.global_identifier_kind {
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
        } else if kind == SUS.array_type_kind {
            ModuleOrWrittenType::WrittenType(self.flatten_array_type_tree(span, cursor))
        } else {cursor.could_not_match()}
    }

    fn flatten_declaration_tree<const ALLOW_MODULES : bool, const ALLOW_MODIFIERS : bool>(&mut self, fallback_identifier_type : IdentifierType, declaration_itself_is_not_written_to : bool, cursor : &mut Cursor<'l>) -> FlatID {
        cursor.go_down(SUS.declaration_kind, |cursor| {
            let identifier_type = cursor.optional_field(SUS.declaration_modifiers_field, |cursor| {
                let (modifier_kind, modifier_span) = cursor.kind_span();

                if !ALLOW_MODIFIERS {
                    self.errors.error_basic(modifier_span, "Inputs and outputs of a module cannot be decorated with 'state' or 'gen'");
                    return fallback_identifier_type;
                }

                if modifier_kind == SUS.state_kw {
                    IdentifierType::State
                } else if modifier_kind == SUS.gen_kw {
                    IdentifierType::Generative
                } else {
                    cursor.could_not_match()
                }
            }).unwrap_or(fallback_identifier_type);
    
            let typ_or_module_expr = cursor.field(SUS.type_field, |cursor| self.flatten_module_or_type_tree::<ALLOW_MODULES>(cursor));
            
            let name_span = cursor.field_span(SUS.name_field, SUS.identifier_kind);
    
            let span_latency_specifier = cursor.optional_field(SUS.latency_specifier_field, |cursor| {
                cursor.go_down_content(SUS.latency_specifier_kind, 
                    |cursor| (self.flatten_expr_tree(cursor), cursor.span())
            )});
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
                    return self.alloc_module_interface_tree(self.linker.file.file_text[name_span].to_owned().into_boxed_str(), md, md_id, span)
                }
            };

            let typ_expr_span = typ_expr.get_span();
            let name = &self.linker.file.file_text[name_span];
            let documentation = cursor.extract_gathered_comments();

            let inst_id = self.instructions.alloc(Instruction::Declaration(Declaration{
                typ : typ_expr.to_type(),
                typ_expr,
                is_declared_in_this_module : self.is_declared_in_this_module,
                read_only : fallback_identifier_type == IdentifierType::Input,
                declaration_itself_is_not_written_to,
                identifier_type,
                name : name.to_owned().into_boxed_str(),
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

    fn flatten_array_bracket_tree(&mut self, cursor : &mut Cursor<'l>) -> (FlatID, BracketSpan) {
        cursor.go_down_content(SUS.array_bracket_expression_kind, 
            |cursor| (self.flatten_expr_tree(cursor), BracketSpan::from_outer(cursor.span()))
        )
    }

    fn desugar_func_call_tree(&mut self, cursor : &mut Cursor<'l>) -> Option<(&Module, InterfacePorts<FlatID>)> {
        let whole_function_span = cursor.span();
        cursor.go_down(SUS.func_call_kind, |cursor| {
            let instantiation_flat_id = cursor.field(SUS.name_field, |cursor| self.get_module_by_global_identifier_tree(cursor));

            let (arguments_span, arguments) = cursor.field(SUS.arguments_field, |cursor| {
                (BracketSpan::from_outer(cursor.span()),
                cursor.collect_list(SUS.parenthesis_expression_list_kind, |cursor| {
                    self.flatten_expr_tree(cursor)
                }))
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
    fn get_module_by_global_identifier_tree(&mut self, cursor : &mut Cursor<'l>) -> Option<FlatID> {
        let (kind, span) = cursor.kind_span();
        if kind == SUS.global_identifier_kind {
            cursor.go_down(SUS.global_identifier_kind, |_cursor| {
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
                            Some(self.alloc_module_interface_tree(md.link_info.name.clone(), md, module_id, span))
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

    fn flatten_expr_tree(&mut self, cursor : &mut Cursor<'l>) -> FlatID {
        let (kind, expr_span) = cursor.kind_span();
        
        let source = if kind == SUS.global_identifier_kind {
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
        } else if kind == SUS.number_kind {
            let text = &self.linker.file.file_text[expr_span];
            WireSource::Constant(Value::Integer(BigInt::from_str(text).unwrap()))
        } else if kind == SUS.unary_op_kind {
            cursor.go_down_no_check(|cursor| {
                let op_text = &self.linker.file.file_text[cursor.field_span_no_check(SUS.operator_field)];
                let op = UnaryOperator::from_text(op_text);
                
                let right = cursor.field(SUS.right_field, |cursor| self.flatten_expr_tree(cursor));

                WireSource::UnaryOp{op, right}
            })
        } else if kind == SUS.binary_op_kind {
            cursor.go_down_no_check(|cursor| {
                let left = cursor.field(SUS.left_field, |cursor| self.flatten_expr_tree(cursor));
                let op_text = &self.linker.file.file_text[cursor.field_span_no_check(SUS.operator_field)];
                let op = BinaryOperator::from_text(op_text);
                let right = cursor.field(SUS.right_field, |cursor| self.flatten_expr_tree(cursor));

                WireSource::BinaryOp{op, left, right}
            })
        } else if kind == SUS.array_op_kind {
            cursor.go_down_no_check(|cursor| {
                let arr = cursor.field(SUS.arr_field, |cursor| self.flatten_expr_tree(cursor));
                
                let (arr_idx, bracket_span) = cursor.field(SUS.arr_idx_field, |cursor| self.flatten_array_bracket_tree(cursor));
                
                WireSource::ArrayAccess{arr, arr_idx, bracket_span}
            })
        } else if kind == SUS.func_call_kind {
            if let Some((md, interface_wires)) = self.desugar_func_call_tree(cursor) {
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
        } else if kind == SUS.parenthesis_expression_kind {
            return cursor.go_down_content(SUS.parenthesis_expression_kind, |cursor| self.flatten_expr_tree(cursor));
        } else {
            cursor.could_not_match()
        };

        let wire_instance = WireInstance{
            typ : Type::Unknown,
            is_compiletime : IS_GEN_UNINIT,
            span: expr_span,
            source,
            is_declared_in_this_module : self.is_declared_in_this_module
        };
        self.instructions.alloc(Instruction::Wire(wire_instance))
    }
    fn flatten_assignable_expr_tree(&mut self, write_modifiers : WriteModifiers, cursor : &mut Cursor<'l>) -> Option<ConnectionWrite> {
        let (kind, span) = cursor.kind_span();
        if kind == SUS.global_identifier_kind {
            let root = self.resolve_identifier(span).expect_local("assignments")?;

            Some(ConnectionWrite{root, root_span : span, path : Vec::new(), span, is_declared_in_this_module : self.is_declared_in_this_module, write_modifiers})
        } else if kind == SUS.array_op_kind {
            cursor.go_down_no_check(|cursor| {
                let flattened_arr_expr_opt = cursor.field(SUS.arr_field, |cursor| self.flatten_assignable_expr_tree(write_modifiers, cursor));
                
                let (idx, bracket_span) = cursor.field(SUS.arr_idx_field, |cursor| self.flatten_array_bracket_tree(cursor));
                
                let mut flattened_arr_expr = flattened_arr_expr_opt?; // only unpack the subexpr after flattening the idx, so we catch all errors
                
                flattened_arr_expr.path.push(ConnectionWritePathElement::ArrayIdx{idx, bracket_span});
                flattened_arr_expr.span = Span::new_overarching(flattened_arr_expr.span, bracket_span.outer_span());
                
                Some(flattened_arr_expr)
            })
        } else if kind == SUS.number_kind {self.errors.error_basic(span, "Cannot assign to constant"); None
        } else if kind == SUS.unary_op_kind {self.errors.error_basic(span, "Cannot assign to the result of an operator"); None
        } else if kind == SUS.binary_op_kind {self.errors.error_basic(span, "Cannot assign to the result of an operator"); None
        } else if kind == SUS.func_call_kind {self.errors.error_basic(span, "Cannot assign to submodule call"); None
        } else if kind == SUS.parenthesis_expression_kind {self.errors.error_basic(span, "Remove these parentheses"); None
        } else {cursor.could_not_match()}
    }

    fn flatten_if_statement_tree(&mut self, cursor : &mut Cursor<'l>) {
        cursor.go_down(SUS.if_statement_kind, |cursor| {
            let condition = cursor.field(SUS.condition_field, |cursor| self.flatten_expr_tree(cursor));
            
            let if_id = self.instructions.alloc(Instruction::IfStatement(IfStatement{condition, then_start : UUID::PLACEHOLDER, then_end_else_start : UUID::PLACEHOLDER, else_end : UUID::PLACEHOLDER}));
            let then_start = self.instructions.get_next_alloc_id();
            
            cursor.field(SUS.then_block_field, |cursor| self.flatten_code_tree(cursor));
            let then_end_else_start = self.instructions.get_next_alloc_id();
            cursor.optional_field(SUS.else_block_field, |cursor| {
                if cursor.kind() == SUS.if_statement_kind {
                    self.flatten_if_statement_tree(cursor); // Chained if statements
                } else {
                    self.flatten_code_tree(cursor)
                }
            });
            let else_end = self.instructions.get_next_alloc_id();
            
            let Instruction::IfStatement(if_stmt) = &mut self.instructions[if_id] else {unreachable!()};
            if_stmt.then_start = then_start;
            if_stmt.then_end_else_start = then_end_else_start;
            if_stmt.else_end = else_end;
        })
    }

    fn flatten_assign_function_call(&mut self, to : Vec<Result<ConnectionWrite, Span>>, cursor : &mut Cursor<'l>) {
        let func_call_span = cursor.span();
        let to_iter = if let Some((md, interface)) = self.desugar_func_call_tree(cursor) {
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
                let err_id = self.instructions.alloc(Instruction::Wire(WireInstance{typ : Type::Error, is_compiletime : true, span : func_call_span, is_declared_in_this_module : self.is_declared_in_this_module, source : WireSource::Constant(Value::Error)}));
                self.instructions.alloc(Instruction::Write(Write{from: err_id, to}));
            }
        }
    }

    fn flatten_code_tree(&mut self, cursor : &mut Cursor<'l>) {
        let old_frame = self.local_variable_context.new_frame();
        
        self.flatten_code_keep_context_tree(cursor);

        self.local_variable_context.pop_frame(old_frame);
    }
    fn flatten_code_keep_context_tree(&mut self, cursor : &mut Cursor<'l>) {
        cursor.clear_gathered_comments(); // Clear comments at the start of a block
        cursor.list(SUS.block_kind, |cursor| {
            let kind = cursor.kind();
            if kind == SUS.assign_left_side_kind {
                self.flatten_standalone_decls_tree(cursor);
            } else if kind == SUS.decl_assign_statement_kind {
                cursor.go_down_no_check(|cursor| {
                    let to = cursor.field(SUS.assign_left_field, |cursor| self.flatten_assignment_left_side_tree(cursor));
                    
                    cursor.field(SUS.assign_value_field, |cursor| {
                        let (node_kind, span) = cursor.kind_span();
                        
                        if node_kind == SUS.func_call_kind {
                            self.flatten_assign_function_call(to, cursor);
                        } else {
                            let read_side = self.flatten_expr_tree(cursor);
                            
                            if to.len() != 1 {
                                self.errors.error_basic(span, format!("Non-function assignments must output exactly 1 output instead of {}", to.len()));
                            }
                            if let Some(Ok(to)) = to.into_iter().next() {
                                self.instructions.alloc(Instruction::Write(Write{from: read_side, to}));
                            }
                        }
                    });
                });
            } else if kind == SUS.block_kind {
                self.flatten_code_tree(cursor);
            } else if kind == SUS.if_statement_kind {
                self.flatten_if_statement_tree(cursor);
            } else if kind == SUS.for_statement_kind {
                cursor.go_down_no_check(|cursor| {
                    let loop_var_decl = cursor.field(SUS.for_decl_field, |cursor| self.flatten_declaration_tree::<false, false>(IdentifierType::Generative, true, cursor));

                    let start = cursor.field(SUS.from_field, |cursor| self.flatten_expr_tree(cursor));
                    let end = cursor.field(SUS.to_field, |cursor| self.flatten_expr_tree(cursor));
                    
                    let for_id = self.instructions.alloc(Instruction::ForStatement(ForStatement{loop_var_decl, start, end, loop_body: UUIDRange(UUID::PLACEHOLDER, UUID::PLACEHOLDER)}));

                    let code_start = self.instructions.get_next_alloc_id();

                    cursor.field(SUS.block_field, |cursor| self.flatten_code_tree(cursor));
                    
                    let code_end = self.instructions.get_next_alloc_id();

                    let Instruction::ForStatement(for_stmt) = &mut self.instructions[for_id] else {unreachable!()};

                    for_stmt.loop_body = UUIDRange(code_start, code_end);
                })
            }
            cursor.clear_gathered_comments(); // Clear comments after every statement, so comments don't bleed over
        });
    }

    fn flatten_write_modifiers_tree(&self, cursor : &mut Cursor<'l>) -> WriteModifiers {
        cursor.optional_field(SUS.write_modifiers_field, |cursor| {
            let modifiers_span = cursor.span();
            let mut initial_count = 0;
            let mut reg_count = 0;
            cursor.list(SUS.write_modifiers_kind, |cursor| {
                let kw_kind = cursor.kind();
                if kw_kind == SUS.reg_kw {
                    reg_count += 1;
                } else if kw_kind == SUS.initial_kw {
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
        }).unwrap_or(WriteModifiers::Connection { num_regs: 0, regs_span: cursor.span().empty_span_at_front() })
    }

    /// See [Self::flatten_standalone_decls_tree][]
    /// Two cases:
    /// - Left side of assignment:
    ///     No modules, Yes write modifiers, Only assignable expressions
    fn flatten_assignment_left_side_tree(&mut self, cursor : &mut Cursor<'l>) -> Vec<Result<ConnectionWrite, Span>> {
        cursor.collect_list(SUS.assign_left_side_kind, |cursor| {
            cursor.go_down(SUS.assign_to_kind, |cursor| {
                let write_modifiers = self.flatten_write_modifiers_tree(cursor);
                
                cursor.field(SUS.expr_or_decl_field, |cursor| {
                    let (kind, span) = cursor.kind_span();
    
                    if kind == SUS.declaration_kind {
                        let root = self.flatten_declaration_tree::<false, true>(IdentifierType::Local, true, cursor);
                        let flat_root_decl = self.instructions[root].extract_wire_declaration();
                        Ok(ConnectionWrite{root, root_span : flat_root_decl.name_span, path: Vec::new(), span, is_declared_in_this_module: true, write_modifiers})
                    } else { // It's _expression
                        self.flatten_assignable_expr_tree(write_modifiers, cursor).ok_or(span)
                    }
                })
            })
        })
    }

    /// See [Self::flatten_assignment_left_side_tree][]
    /// - Standalone declarations:
    ///     Yes modules, No write modifiers, Yes expressions (-> single expressions)
    fn flatten_standalone_decls_tree(&mut self, cursor : &mut Cursor<'l>) {
        let mut is_first_item = true;
        cursor.list(SUS.assign_left_side_kind, |cursor| {
            cursor.go_down(SUS.assign_to_kind, |cursor| {
                if !is_first_item {
                    self.errors.warn_basic(cursor.span(), "Standalone declarations and expressions should be on their own line. ");
                }
                is_first_item = false;

                if let Some(span) = cursor.optional_field_span(SUS.write_modifiers_field, SUS.write_modifiers_kind) {
                    self.errors.error_basic(span, "No write modifiers are allowed on non-assigned to declarations or expressions");
                }
                
                cursor.field(SUS.expr_or_decl_field, |cursor| {
                    let (kind, span) = cursor.kind_span();
    
                    if kind == SUS.declaration_kind {
                        let _ = self.flatten_declaration_tree::<true, true>(IdentifierType::Local, true, cursor);
                    } else { // It's _expression
                        if kind == SUS.func_call_kind {
                            self.flatten_assign_function_call(Vec::new(), cursor);
                        } else {
                            self.errors.warn_basic(span, "The result of this operation is not used");
                            let _ = self.flatten_expr_tree(cursor);
                        }
                    }
                })
            });
        })
    }

    fn flatten_declaration_list_tree(&mut self, identifier_type : IdentifierType, ports : &mut Vec<FlatID>, cursor : &mut Cursor<'l>) {
        cursor.list(SUS.declaration_list_kind, |cursor| {
            ports.push(self.flatten_declaration_tree::<false, false>(identifier_type, true, cursor));
        });
    }

    fn flatten_interface_ports_tree<const IS_SUBMODULE : bool>(&mut self, cursor : &mut Cursor<'l>) -> InterfacePorts<FlatID> {
        cursor.optional_field(SUS.interface_ports_field, |cursor| {
            cursor.go_down(SUS.interface_ports_kind, |cursor| {
                let mut ports = Vec::new();
                cursor.optional_field(SUS.inputs_field, |cursor| {
                    let identifier_type = if IS_SUBMODULE {IdentifierType::Local} else {IdentifierType::Input};
                    self.flatten_declaration_list_tree(identifier_type, &mut ports, cursor)
                });
                let outputs_start = ports.len();
                cursor.optional_field(SUS.outputs_field, |cursor| {
                    let identifier_type = if IS_SUBMODULE {IdentifierType::Local} else {IdentifierType::Output};
                    self.flatten_declaration_list_tree(identifier_type, &mut ports, cursor)
                });
                InterfacePorts{ outputs_start, ports: ports.into_boxed_slice() }
            })
        }).unwrap_or(InterfacePorts::empty())
    }

    fn alloc_module_interface_tree(&mut self, name : Box<str>, module : &Module, module_uuid : ModuleUUID, typ_span : Span) -> FlatID {
        let local_linker = self.linker.new_sublinker(module.link_info.file);

        let mut nested_context = FlatteningContext {
            instructions: std::mem::replace(&mut self.instructions, FlatAlloc::new()),
            errors: ErrorCollector::new(module.link_info.file, local_linker.file.file_text.len()), // Temporary ErrorCollector, unused
            is_declared_in_this_module: false,
            linker: local_linker,
            type_list_for_naming: self.type_list_for_naming,
            local_variable_context : LocalVariableContext::new_initial()
        };
        
        let mut nested_cursor = Cursor::new_for_node(&nested_context.linker.file.tree, &nested_context.linker.file.file_text, module.link_info.span, SUS.module_kind);

        let interface_ports = nested_cursor.go_down(SUS.module_kind, |nested_cursor| {
            nested_cursor.field(SUS.name_field, |_| {}); // Get past name field
            nested_context.flatten_interface_ports_tree::<true>(nested_cursor)
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

    fn flatten_module_tree(&mut self, cursor : &mut Cursor<'l>) -> InterfacePorts<FlatID> {
        cursor.go_down(SUS.module_kind, |cursor| {
            let name_span = cursor.field_span(SUS.name_field, SUS.identifier_kind);
            let module_name = &self.linker.file.file_text[name_span];
            println!("TREE SITTER module! {module_name}");
            // Interface is allocated in self
            let interface_found = self.flatten_interface_ports_tree::<false>(cursor);
            cursor.field(SUS.block_field, |cursor| self.flatten_code_tree(cursor));
            interface_found
        })
    }

    /*
        ==== Typechecking ====
    */
    fn typecheck_wire_is_of_type(&self, wire : &WireInstance, expected : &Type, context : &str) {
        typecheck(&wire.typ, wire.span, expected, context, self.type_list_for_naming, &self.errors);
    }

    fn typecheck_connection(&self, to : &ConnectionWrite, from : FlatID) {
        // Typecheck digging down into write side
        let conn_root = self.instructions[to.root].extract_wire_declaration();
        let mut write_to_type = Some(&conn_root.typ);
        for p in &to.path {
            match p {
                &ConnectionWritePathElement::ArrayIdx{idx, bracket_span} => {
                    let idx_wire = self.instructions[idx].extract_wire();
                    self.typecheck_wire_is_of_type(idx_wire, &INT_TYPE, "array index");
                    if let Some(wr) = write_to_type {
                        write_to_type = typecheck_is_array_indexer(wr, bracket_span.outer_span(), self.type_list_for_naming, &self.errors);
                    }
                }
            }
        }

        // Typecheck the value with target type
        let from_wire = self.instructions[from].extract_wire();
        if let Some(target_type) = write_to_type {
            self.typecheck_wire_is_of_type(from_wire, &target_type, "connection");
        }
    }

    fn typecheck(&mut self) {
        let look_at_queue : Vec<FlatID> = self.instructions.iter().map(|(id,_)| id).collect();

        for elem_id in look_at_queue {
            match &self.instructions[elem_id] {
                Instruction::SubModule(_) => {}
                Instruction::Declaration(decl) => {
                    if let Some(latency_spec) = decl.latency_specifier {
                        let latency_spec_wire = &self.instructions[latency_spec].extract_wire();
                        self.typecheck_wire_is_of_type(latency_spec_wire, &INT_TYPE, "latency specifier");
                    }

                    decl.typ.for_each_generative_input(&mut |param_id| {
                        self.typecheck_wire_is_of_type(self.instructions[param_id].extract_wire(), &INT_TYPE, "Array size");
                    });
                }
                Instruction::IfStatement(stm) => {
                    let wire = &self.instructions[stm.condition].extract_wire();
                    self.typecheck_wire_is_of_type(wire, &BOOL_TYPE, "if statement condition")
                }
                Instruction::ForStatement(stm) => {
                    let loop_var = &self.instructions[stm.loop_var_decl].extract_wire_declaration();
                    let start = &self.instructions[stm.start].extract_wire();
                    let end = &self.instructions[stm.end].extract_wire();
                    self.typecheck_wire_is_of_type(start, &loop_var.typ, "for loop");
                    self.typecheck_wire_is_of_type(end, &loop_var.typ, "for loop");
                }
                Instruction::Wire(w) => {
                    let result_typ = match &w.source {
                        &WireSource::WireRead(from_wire) => {
                            self.instructions[from_wire].extract_wire_declaration().typ.clone()
                        }
                        &WireSource::UnaryOp{op, right} => {
                            let right_wire = self.instructions[right].extract_wire();
                            typecheck_unary_operator(op, &right_wire.typ, right_wire.span, self.type_list_for_naming, &self.errors)
                        }
                        &WireSource::BinaryOp{op, left, right} => {
                            let left_wire = self.instructions[left].extract_wire();
                            let right_wire = self.instructions[right].extract_wire();
                            let ((input_left_type, input_right_type), output_type) = get_binary_operator_types(op);
                            self.typecheck_wire_is_of_type(left_wire, &input_left_type, &format!("{op} left"));
                            self.typecheck_wire_is_of_type(right_wire, &input_right_type, &format!("{op} right"));
                            output_type
                        }
                        &WireSource::ArrayAccess{arr, arr_idx, bracket_span:_} => {
                            let arr_wire = self.instructions[arr].extract_wire();
                            let arr_idx_wire = self.instructions[arr_idx].extract_wire();
                
                            self.typecheck_wire_is_of_type(arr_idx_wire, &INT_TYPE, "array index");
                            if let Some(typ) = typecheck_is_array_indexer(&arr_wire.typ, arr_wire.span, self.type_list_for_naming, &self.errors) {
                                typ.clone()
                            } else {
                                Type::Error
                            }
                        }
                        WireSource::Constant(value) => {
                            value.get_type_of_constant()
                        }
                        &WireSource::NamedConstant(id) => {
                            let NamedConstant::Builtin{name:_, typ, val:_} = &self.linker.get_constant(id);
                            typ.clone()
                        }
                    };
                    let Instruction::Wire(w) = &mut self.instructions[elem_id] else {unreachable!()};
                    w.typ = result_typ;
                }
                Instruction::Write(conn) => {
                    self.typecheck_connection(&conn.to, conn.from);
                }
            }
        }

        // Post type application. Flag any remaining Type::Unknown
        for (_id, inst) in self.instructions.iter() {
            inst.for_each_embedded_type(&mut |typ, span| {
                if typ.contains_error_or_unknown::<false, true>() {
                    self.errors.error_basic(span, format!("Unresolved Type: {}", typ.to_string(self.type_list_for_naming)))
                }
            });
        }
    }

    /*
        ==== Generative Code Checking ====
    */
    fn must_be_compiletime_with_info<CtxFunc : FnOnce() -> Vec<ErrorInfo>>(&self, wire : &WireInstance, context : &str, ctx_func : CtxFunc) {
        if !wire.is_compiletime {
            self.errors.error_with_info(wire.span, format!("{context} must be compile time"), ctx_func());
        }
    }
    fn must_be_compiletime(&self, wire : &WireInstance, context : &str) {
        self.must_be_compiletime_with_info(wire, context, || Vec::new());
    }

    fn generative_check(&mut self) {
        let mut runtime_if_stack : Vec<(FlatID, Span)> = Vec::new();

        let mut declaration_depths : FlatAlloc<Option<usize>, FlatIDMarker> = self.instructions.iter().map(|_| None).collect();

        for inst_id in self.instructions.id_range() {
            while let Some((end_id, span)) = runtime_if_stack.pop() {
                if end_id != inst_id {
                    runtime_if_stack.push((end_id, span));
                    break;
                }
            }
            match &self.instructions[inst_id] {
                Instruction::SubModule(_) => {}
                Instruction::Declaration(decl) => {
                    if decl.identifier_type.is_generative() {
                        assert!(declaration_depths[inst_id].is_none());
                        declaration_depths[inst_id] = Some(runtime_if_stack.len())
                    }

                    if let Some(latency_specifier) = decl.latency_specifier {
                        self.must_be_compiletime(self.instructions[latency_specifier].extract_wire(), "Latency specifier");
                    }

                    decl.typ.for_each_generative_input(&mut |param_id| {
                        self.must_be_compiletime(self.instructions[param_id].extract_wire(), "Array size");
                    });
                }
                Instruction::Wire(wire) => {
                    let mut is_generative = true;
                    if let WireSource::WireRead(from) = &wire.source {
                        let decl = self.instructions[*from].extract_wire_declaration();
                        if !decl.identifier_type.is_generative() {
                            is_generative = false;
                        }
                    } else {
                        wire.source.for_each_input_wire(&mut |source_id| {
                            let source_wire = self.instructions[source_id].extract_wire();
                            if !source_wire.is_compiletime {
                                is_generative = false;
                            }
                        });
                    }
                    let Instruction::Wire(wire) = &mut self.instructions[inst_id] else {unreachable!()};
                    wire.is_compiletime = is_generative;
                }
                Instruction::Write(conn) => {
                    let decl = self.instructions[conn.to.root].extract_wire_declaration();

                    if decl.read_only {
                        self.errors.error_with_info(conn.to.span, "Cannot Assign to Read-Only value", vec![decl.make_declared_here(self.errors.file)]);
                    }

                    let from_wire = self.instructions[conn.from].extract_wire();
                    match conn.to.write_modifiers {
                        WriteModifiers::Connection{num_regs : _, regs_span : _} => {
                            if decl.identifier_type.is_generative() {
                                // Check that whatever's written to this declaration is also generative
                                self.must_be_compiletime_with_info(from_wire, "Assignments to generative variables", || vec![decl.make_declared_here(self.errors.file)]);

                                // Check that this declaration isn't used in a non-compiletime if
                                let declared_at_depth = declaration_depths[conn.to.root].unwrap();
            
                                if runtime_if_stack.len() > declared_at_depth {
                                    let mut infos = Vec::new();
                                    infos.push(decl.make_declared_here(self.errors.file));
                                    for (_, if_cond_span) in &runtime_if_stack[declared_at_depth..] {
                                        infos.push(error_info(*if_cond_span, self.errors.file, "Runtime Condition here"));
                                    }
                                    self.errors.error_with_info(conn.to.span, "Cannot write to generative variables in runtime conditional block", infos);
                                }
                            }
                        }
                        WriteModifiers::Initial{initial_kw_span} => {
                            if decl.identifier_type != IdentifierType::State {
                                self.errors.error_with_info(initial_kw_span, "Initial values can only be given to state registers!", vec![decl.make_declared_here(self.errors.file)])
                            }
                            self.must_be_compiletime(from_wire, "initial value assignment")
                        }
                    }
                }
                Instruction::IfStatement(if_stmt) => {
                    let condition_wire = self.instructions[if_stmt.condition].extract_wire();
                    if !condition_wire.is_compiletime {
                        runtime_if_stack.push((if_stmt.else_end, condition_wire.span));
                    }
                }
                Instruction::ForStatement(_) => {}
            }
        }
    }

    /* 
        ==== Additional Warnings ====
    */
    fn find_unused_variables(&self, interface : &InterfacePorts<FlatID>) {
        // Setup Wire Fanouts List for faster processing
        let mut instance_fanins : FlatAlloc<Vec<FlatID>, FlatIDMarker> = self.instructions.iter().map(|_| Vec::new()).collect();

        for (inst_id, inst) in self.instructions.iter() {
            match inst {
                Instruction::Write(conn) => {
                    instance_fanins[conn.to.root].push(conn.from);
                }
                Instruction::SubModule(sm) => {
                    for w in sm.interface_ports.outputs() {
                        instance_fanins[*w].push(inst_id);
                    }
                    for port in sm.interface_ports.inputs() {
                        instance_fanins[inst_id].push(*port);
                    }
                }
                Instruction::Declaration(decl) => {
                    decl.typ.for_each_generative_input(&mut |id| instance_fanins[inst_id].push(id));
                }
                Instruction::Wire(wire) => {
                    wire.source.for_each_input_wire(&mut |id| instance_fanins[inst_id].push(id));
                }
                Instruction::IfStatement(stm) => {
                    for id in UUIDRange(stm.then_start, stm.else_end) {
                        if let Instruction::Write(conn) = &self.instructions[id] {
                            instance_fanins[conn.to.root].push(stm.condition);
                        }
                    }
                }
                Instruction::ForStatement(stm) => {
                    instance_fanins[stm.loop_var_decl].push(stm.start);
                    instance_fanins[stm.loop_var_decl].push(stm.end);
                }
            }
        }

        let mut is_instance_used_map : FlatAlloc<bool, FlatIDMarker> = self.instructions.iter().map(|_| false).collect();

        let mut wire_to_explore_queue : Vec<FlatID> = Vec::new();

        for port in interface.outputs() {
            is_instance_used_map[*port] = true;
            wire_to_explore_queue.push(*port);
        }

        while let Some(item) = wire_to_explore_queue.pop() {
            for from in &instance_fanins[item] {
                if !is_instance_used_map[*from] {
                    is_instance_used_map[*from] = true;
                    wire_to_explore_queue.push(*from);
                }
            }
        }

        // Now produce warnings from the unused list
        for (id, inst) in self.instructions.iter() {
            if !is_instance_used_map[id] {
                if let Instruction::Declaration(decl) = inst {
                    if decl.is_declared_in_this_module {
                        self.errors.warn_basic(decl.name_span, "Unused Variable: This variable does not affect the output ports of this module");
                    }
                }
            }
        }
    }
}

#[derive(Debug)]
pub struct FlattenedInterfacePort {
    pub wire_id : FlatID,
    pub port_name : Box<str>,
    pub span : Span
}

#[derive(Debug)]
pub struct FlattenedModule {
    pub instructions : FlatAlloc<Instruction, FlatIDMarker>,
    pub errors : ErrorCollector,
    pub interface_ports : InterfacePorts<FlatID>,
    pub resolved_globals : ResolvedGlobals
}

impl FlattenedModule {
    pub fn empty(errors : ErrorCollector) -> FlattenedModule {
        FlattenedModule {
            instructions : FlatAlloc::new(),
            errors,
            interface_ports : InterfacePorts::empty(),
            resolved_globals : ResolvedGlobals::new()
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
        let mut cursor = Cursor::new_for_node(&global_resolver.file.tree, &global_resolver.file.file_text, module.link_info.span, SUS.module_kind);

        let mut context = FlatteningContext{
            instructions : FlatAlloc::new(),
            errors : ErrorCollector::new(module.link_info.file, global_resolver.file.file_text.len()),
            is_declared_in_this_module : true,
            linker : global_resolver,
            local_variable_context : LocalVariableContext::new_initial(),
            type_list_for_naming : &linker.types
        };

        // Temporary, switch to iterating over nodes in file itself when needed. 
        let interface_ports = context.flatten_module_tree(&mut cursor);

        //let interface_ports = context.initialize_interface::<false>();


        //context.flatten_code(&module.code);
        context.typecheck();
        context.generative_check();
        context.find_unused_variables(&interface_ports);

        FlattenedModule {
            resolved_globals : context.linker.extract_resolved_globals(),
            errors : context.errors,
            instructions : context.instructions,
            interface_ports,
        }
    }
}
