

use std::{ops::{Deref, DerefMut}, str::FromStr};

use num::BigInt;
use sus_proc_macro::{field, kind, kw};
use crate::{
    arena_alloc::{UUIDRange, UUIDRangeIter, UUID}, debug::SpanDebugger, errors::ErrorCollector, file_position::{BracketSpan, Span}, linker::{with_module_editing_context, ConstantUUIDMarker, Linker, ModuleUUID, ModuleUUIDMarker, NameElem, NameResolver, NamedConstant, NamedType, Resolver, TypeUUIDMarker, WorkingOnResolver}, parser::Cursor, template::{TemplateArg, TemplateArgKind, TemplateArgs, TemplateIDMarker, TemplateInputs}, value::Value
};

use super::name_context::LocalVariableContext;
use super::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum NamedLocal {
    Declaration(FlatID),
    SubModule(FlatID),
    TemplateType(TemplateID),
}

enum LocalOrGlobal {
    Local(Span, NamedLocal),
    Global(NameElem, Span, TemplateArgs, Option<BracketSpan>),
    // Error is already handled
    NotFound(Span)
}

#[derive(Debug)]
enum PartialWireReference {
    /// Means the error has already been reported
    Error,
    /// A module that is implicitly declared in a function call. 
    GlobalModuleName(GlobalReference<ModuleUUID>),
    /// Partial result, waiting for a port to be grabbed
    ModuleButNoPort(FlatID, Span),
    /// A module with an interface specified
    ModuleWithInterface{submodule_decl : FlatID, submodule_name_span : Span, interface : DomainID, interface_name_span : Span},
    /// It's ready for use higher up
    WireReference(WireReference),
}

impl PartialWireReference {
    fn expect_wireref(self, ctx : &FlatteningContext) -> Option<WireReference> {
        match self {
            PartialWireReference::Error => None, // Error already reported
            PartialWireReference::ModuleButNoPort(submod_decl, span) => {
                let md_uuid = ctx.working_on.instructions[submod_decl].unwrap_submodule().module_ref.id;
                ctx.errors
                    .error(span, "cannot operate on modules directly. Should use ports instead")
                    .info_obj(&ctx.modules[md_uuid]);
                None
            },
            PartialWireReference::GlobalModuleName(md_ref) => {
                let md = &ctx.modules[md_ref.id];
                ctx.errors
                    .error(md_ref.span, format!("Expected a Wire Reference, but found module '{}' instead", md.link_info.name))
                    .info_obj(md);
                None
            }
            PartialWireReference::ModuleWithInterface { submodule_decl: submod_decl, submodule_name_span:_, interface, interface_name_span } => {
                let md_uuid = ctx.working_on.instructions[submod_decl].unwrap_submodule().module_ref.id;
                let md = &ctx.modules[md_uuid];
                let interf = &md.interfaces[interface];
                ctx.errors
                    .error(interface_name_span, format!("Expected a port, but found module interface '{}' instead", &interf.name))
                    .info((interf.name_span, md.link_info.file), "Declared here");
                None
            }
            PartialWireReference::WireReference(wr) => Some(wr),
        }
    }
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum DeclarationContext {
    Input,
    Output,
    ForLoopGenerative,
    PlainWire
}

use crate::template::TemplateID;

use super::FlatID;

struct FlatteningContext<'l, 'errs> {
    modules : WorkingOnResolver<'l, 'errs, ModuleUUIDMarker, Module>,
    #[allow(dead_code)]
    types : Resolver<'l, 'errs, TypeUUIDMarker, NamedType>,
    #[allow(dead_code)]
    constants : Resolver<'l, 'errs, ConstantUUIDMarker, NamedConstant>,
    name_resolver : NameResolver<'l, 'errs>,
    errors : &'errs ErrorCollector<'l>,

    ports_to_visit : UUIDRangeIter<PortIDMarker>,
    template_inputs_to_visit : UUIDRangeIter<TemplateIDMarker>,

    local_variable_context : LocalVariableContext<'l, NamedLocal>
}

impl<'l, 'errs> Deref for FlatteningContext<'l, 'errs> {
    type Target = WorkingOnResolver<'l, 'errs, ModuleUUIDMarker, Module>;

    fn deref(&self) -> &Self::Target {
        &self.modules
    }
}
impl<'l, 'errs> DerefMut for FlatteningContext<'l, 'errs> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.modules
    }
}

impl<'l, 'errs> FlatteningContext<'l, 'errs> {
    fn flatten_template_args(&mut self, cursor : &mut Cursor) -> Vec<TemplateArg> {
        cursor.collect_list(kind!("template_params"), |cursor| {
            let (outer_kind, whole_span) = cursor.kind_span();
            match outer_kind {
                kind!("template_type_param") => {
                    cursor.go_down_no_check(|cursor| {
                        let name_specification = cursor.optional_field_span(field!("name"), kind!("identifier"));
                        cursor.field(field!("arg"));
                        let typ = self.flatten_type(cursor);
                        TemplateArg{name_specification, whole_span, kind : TemplateArgKind::Type(typ)}
                    })
                }
                kind!("template_value_param") => {
                    cursor.go_down_no_check(|cursor| {
                        let name_specification = cursor.optional_field_span(field!("name"), kind!("identifier"));
                        cursor.field(field!("arg"));
                        let expr = self.flatten_expr(cursor);
                        TemplateArg{name_specification, whole_span, kind : TemplateArgKind::Value(expr)}
                    })
                }
                _ => cursor.could_not_match()
            }
        })
    }

    fn collect_template_args(&self, name_elem : NameElem, template_argument_list : Vec<TemplateArg>) -> TemplateArgs {
        let empty_template = TemplateInputs::new();
    
        let template_info = match name_elem {
            NameElem::Module(md_id) => {
                &self.modules[md_id].link_info.template_arguments
            },
            NameElem::Constant(_cst) => &empty_template, //TODO
            NameElem::Type(_ty) => &empty_template, //TODO
        };
    
        let mut resulting_template_arguments : TemplateArgs = FlatAlloc::new_nones(template_info.len());
    
        let mut index_iter = template_info.id_range().iter();
    
        // Now that we have all arguments, we construct the map of TemplateID to TemplateArg
        for given_arg in template_argument_list {
            let template_idx = if let Some(name_span) = given_arg.name_specification {
                let target_name = &self.name_resolver.file_text[name_span];
    
                let Some(named_index) = template_info.find(|_, template_input| template_input.name == target_name) else {
                    let info = self.name_resolver.get_linking_error_location(name_elem);
                    let err_ref = self.errors.error(name_span, format!("No template argument of name '{target_name}' on '{}'", info.full_name));
                    if let Some(pos) = info.location {
                        err_ref.info(pos, format!("'{}' declared here", info.full_name));
                    }
                    continue;
                };
                named_index
            } else {
                let Some(index) = index_iter.next() else {
                    let info = self.name_resolver.get_linking_error_location(name_elem);
                    let err_ref = self.errors.error(given_arg.whole_span, format!("Too many template arguments! '{}' only requires {} template arguments", info.full_name, template_info.len()));
                    if let Some(pos) = info.location {
                        err_ref.info(pos, format!("'{}' declared here", info.full_name));
                    }
                    continue;
                };
                index
            };
    
            let arg = &mut resulting_template_arguments[template_idx];

            if let Some(existing_arg) = arg {
                self.errors.error(given_arg.name_specification.unwrap_or(given_arg.whole_span), "This template variable has already been set")
                    .info_same_file(existing_arg.whole_span, format!("'{} has already been defined here'", template_info[template_idx].name));
            } else {
                *arg = Some(given_arg);
            }
        }
        resulting_template_arguments
    }

    fn flatten_local_or_template_global(&mut self, cursor : &mut Cursor) -> LocalOrGlobal {
        let total_span = cursor.span();
        
        let mut must_be_global = false;
        cursor.go_down(kind!("template_global"), |cursor| {
            must_be_global |= cursor.optional_field(field!("is_global_path"));

            let mut name_path = vec![cursor.field_span(field!("item"), kind!("identifier"))];
            let mut template_args : Vec<TemplateArg> = Vec::new();
            let mut template_args_whole_span : Option<BracketSpan> = None;

            while cursor.optional_field(field!("item")) {
                let (kind, span) = cursor.kind_span();
                if let Some(t_span) = &template_args_whole_span {
                    self.errors.error(Span::new_overarching(t_span.outer_span().empty_span_at_end(), total_span.empty_span_at_end()), format!("Namespace must end after template arguments"));
                    break;
                }
                match kind {
                    kind!("identifier") => {
                        must_be_global = true;
                        name_path.push(span);
                    }
                    kind!("template_params") => {
                        must_be_global = true;
                        
                        template_args = self.flatten_template_args(cursor);
                        template_args_whole_span = Some(BracketSpan::from_outer(span));
                    }
                    _ => cursor.could_not_match()
                }
            }

            // Possibly local
            if !must_be_global {
                let [local_name] = name_path.as_slice() else {unreachable!()};
                let name_text = &self.name_resolver.file_text[*local_name];
                if let Some(decl_id) = self.local_variable_context.get_declaration_for(name_text) {
                    return LocalOrGlobal::Local(*local_name, decl_id);
                }
            }

            // Global identifier
            let [name_span] = name_path.as_slice() else {todo!("Namespaces")};
            if let Some((found_global, found_global_span)) = self.name_resolver.resolve_global(*name_span) {
                let template_arg_map = self.collect_template_args(found_global, template_args);

                LocalOrGlobal::Global(found_global, found_global_span, template_arg_map, template_args_whole_span)
            } else {
                LocalOrGlobal::NotFound(*name_span)
            }
        })
    }
    
    fn flatten_array_type(&mut self, span : Span, cursor : &mut Cursor) -> WrittenType {
        cursor.go_down(kind!("array_type"), |cursor| {
            cursor.field(field!("arr"));
            let array_element_type = self.flatten_type(cursor);

            cursor.field(field!("arr_idx"));
            let (array_size_wire_id, bracket_span) = self.flatten_array_bracket(cursor);
            
            WrittenType::Array(span, Box::new((array_element_type, array_size_wire_id, bracket_span)))
        })
    }

    fn flatten_type(&mut self, cursor : &mut Cursor) -> WrittenType {
        let ModuleOrWrittenType::WrittenType(wr_typ) = self.flatten_module_or_type::<false>(cursor) else {unreachable!("Can't not be type")};
        wr_typ
    }

    fn flatten_module_or_type<const ALLOW_MODULES : bool>(&mut self, cursor : &mut Cursor) -> ModuleOrWrittenType {
        let accepted_text = if ALLOW_MODULES {"Type or Module"} else {"Type"};
        let (kind, span) = cursor.kind_span();
        // Only difference is that 
        if kind == kind!("template_global") {
            match self.flatten_local_or_template_global(cursor) {
                LocalOrGlobal::Local(span, NamedLocal::Declaration(instr)) | LocalOrGlobal::Local(span, NamedLocal::SubModule(instr)) => {
                    self.errors.error(span, format!("This is not a {accepted_text}, it is a local variable instead!"))
                        .info_obj_same_file(&self.working_on.instructions[instr]);
                    
                    ModuleOrWrittenType::WrittenType(WrittenType::Error(span))
                }
                LocalOrGlobal::Local(span, NamedLocal::TemplateType(template_id)) => ModuleOrWrittenType::WrittenType(WrittenType::Template(span, template_id)),
                LocalOrGlobal::Global(resolved_global, resolved_global_span, template_args, template_span) => {
                    match &resolved_global {
                        NameElem::Type(typ_id) => ModuleOrWrittenType::WrittenType(WrittenType::Named(GlobalReference { span: resolved_global_span, id: *typ_id, template_args, template_span })),
                        NameElem::Module(md_id) if ALLOW_MODULES => ModuleOrWrittenType::Module(GlobalReference { span: resolved_global_span, id: *md_id, template_args, template_span }),
                        _ => {
                            self.name_resolver.not_expected_global_error(resolved_global, resolved_global_span, accepted_text);
                            ModuleOrWrittenType::WrittenType(WrittenType::Error(resolved_global_span))
                        }
                    }
                }
                LocalOrGlobal::NotFound(name_span) => ModuleOrWrittenType::WrittenType(WrittenType::Error(name_span)) // Already covered
            }
        } else if kind == kind!("array_type") {
            ModuleOrWrittenType::WrittenType(self.flatten_array_type(span, cursor))
        } else {cursor.could_not_match()}
    }

    fn alloc_local_name(&mut self, name_span : Span, named_local : NamedLocal) {
        if let Err(conflict) = self.local_variable_context.add_declaration(&self.name_resolver.file_text[name_span], named_local) {
            let err_ref = self.errors.error(name_span, "This declaration conflicts with a previous declaration in the same scope");
            
            match conflict {
                NamedLocal::Declaration(decl_id) => {err_ref.info_obj_same_file(self.working_on.instructions[decl_id].unwrap_wire_declaration());}
                NamedLocal::SubModule(submod_id) => {err_ref.info_obj_same_file(self.working_on.instructions[submod_id].unwrap_submodule());}
                NamedLocal::TemplateType(template_id) => {err_ref.info_obj_same_file(&self.working_on.link_info.template_arguments[template_id]);}
            }
        }
    }
    
    fn flatten_declaration<const ALLOW_MODULES : bool>(&mut self, declaration_context : DeclarationContext, mut read_only : bool, declaration_itself_is_not_written_to : bool, cursor : &mut Cursor) -> FlatID {
        let whole_declaration_span = cursor.span();
        cursor.go_down(kind!("declaration"), |cursor| {
            // Extra inputs and outputs declared in the body of the module
            let io_kw = cursor.optional_keyword(field!("io_port_modifiers")).map(|(k, span)| {
                match k {
                    kw!("input") => (true, span),
                    kw!("output") => (false, span),
                    _ => cursor.could_not_match(),
                }
            });

            // State or Generative
            let declaration_modifiers = cursor.optional_keyword(field!("declaration_modifiers"));

            // Still gets overwritten 
            let mut is_port = match declaration_context {
                DeclarationContext::Input | DeclarationContext::Output => {
                    if let Some((_, io_span)) = io_kw {
                        self.errors.error(io_span, "Cannot redeclare 'input' or 'output' on functional syntax IO");
                    }
                    DeclarationPortInfo::RegularPort { is_input: declaration_context == DeclarationContext::Input }
                }
                DeclarationContext::ForLoopGenerative => {
                    if let Some((_, io_span)) = io_kw {
                        self.errors.error(io_span, "Cannot declare 'input' or 'output' to the iterator of a for loop");
                    }
                    DeclarationPortInfo::NotPort
                }
                DeclarationContext::PlainWire => {
                    match io_kw {
                        Some((is_input, _)) => DeclarationPortInfo::RegularPort { is_input },
                        None => DeclarationPortInfo::NotPort,
                    }
                }
            };

            if is_port.implies_read_only() {
                read_only = true;
            }
            
            let identifier_type = match declaration_context {
                DeclarationContext::Input | DeclarationContext::Output | DeclarationContext::PlainWire => {
                    match declaration_modifiers {
                        Some((kw!("state"), modifier_span)) => {
                            if is_port.as_regular_port() == Some(true) {
                                self.errors.error(modifier_span, "Inputs cannot be decorated with 'state'");
                            }
                            IdentifierType::State
                        }
                        Some((kw!("gen"), modifier_span)) => {
                            match is_port {
                                DeclarationPortInfo::NotPort => {}
                                DeclarationPortInfo::RegularPort { is_input : true } => {
                                    let this_template_id = self.template_inputs_to_visit.next().unwrap();
                                    // AHA! Generative input
                                    is_port = DeclarationPortInfo::GenerativeInput(this_template_id)
                                }
                                DeclarationPortInfo::RegularPort { is_input : false } => {
                                    self.errors.error(modifier_span, "Cannot make generative outputs. This is because it could interfere with inference of generic types and generative inputs");
                                }
                                DeclarationPortInfo::GenerativeInput(_) => unreachable!("Can't have been GenerativeInput here already, because it only gets converted to that here"), 
                            }
                            IdentifierType::Generative
                        }
                        Some(_) => cursor.could_not_match(),
                        None => {
                            IdentifierType::Local
                        }
                    }
                }
                DeclarationContext::ForLoopGenerative => {
                    if let Some((_, modifier_span)) = declaration_modifiers {
                        self.errors.error(modifier_span, "Cannot add modifiers to the iterator of a for loop");
                    }
                    IdentifierType::Generative
                }
            };

            cursor.field(field!("type"));
            let decl_span = Span::new_overarching(cursor.span(), whole_declaration_span.empty_span_at_end());
            let typ_or_module_expr = self.flatten_module_or_type::<ALLOW_MODULES>(cursor);
            
            let name_span = cursor.field_span(field!("name"), kind!("identifier"));
    
            let span_latency_specifier = if cursor.optional_field(field!("latency_specifier")) {
                cursor.go_down_content(kind!("latency_specifier"), 
                    |cursor| Some((self.flatten_expr(cursor), cursor.span()))
            )} else {None};
            // Parsing components done

            let documentation = cursor.extract_gathered_comments();

            let typ_expr = match typ_or_module_expr {
                ModuleOrWrittenType::WrittenType(typ) => {
                    typ
                }
                ModuleOrWrittenType::Module(module_ref) => {
                    assert!(ALLOW_MODULES);
                    if let Some((_, span)) = span_latency_specifier {
                        self.errors.error(span, "Cannot add latency specifier to module instances");
                    }
                    let name = &self.name_resolver.file_text[name_span];

                    let submod_id = self.working_on.instructions.alloc(Instruction::SubModule(SubModuleInstance{
                        name : Some((name.to_owned(), name_span)),
                        module_ref,
                        declaration_runtime_depth : DECL_DEPTH_LATER,
                        local_interface_domains : FlatAlloc::new(),
                        documentation
                    }));

                    self.alloc_local_name(name_span, NamedLocal::SubModule(submod_id));

                    return submod_id
                }
            };

            let name = &self.name_resolver.file_text[name_span];

            let decl_id = self.working_on.instructions.alloc(Instruction::Declaration(Declaration{
                typ_expr,
                typ : FullType::new_unset(),
                read_only,
                declaration_itself_is_not_written_to,
                is_port,
                identifier_type,
                name : name.to_owned(),
                name_span,
                decl_span,
                declaration_runtime_depth : DECL_DEPTH_LATER,
                latency_specifier : span_latency_specifier.map(|(ls, _)| ls),
                documentation
            }));

            self.alloc_local_name(name_span, NamedLocal::Declaration(decl_id));

            match is_port {
                DeclarationPortInfo::NotPort => {},
                DeclarationPortInfo::RegularPort { is_input:_ } => {
                    let this_port_id = self.ports_to_visit.next().unwrap();
                    let port = &mut self.working_on.ports[this_port_id];
                    assert_eq!(port.name_span, name_span);
                    port.declaration_instruction = decl_id;
                }
                DeclarationPortInfo::GenerativeInput(this_template_id) => {
                    let TemplateInputKind::Generative { decl_span:_, declaration_instruction } = &mut self.working_on.link_info.template_arguments[this_template_id].kind else {unreachable!()};

                    *declaration_instruction = decl_id;
                }
            }

            decl_id
        })
    }

    fn flatten_array_bracket(&mut self, cursor : &mut Cursor) -> (FlatID, BracketSpan) {
        let bracket_span = BracketSpan::from_outer(cursor.span());
        cursor.go_down_content(kind!("array_bracket_expression"), 
            |cursor| (self.flatten_expr(cursor), bracket_span)
        )
    }

    fn alloc_error(&mut self, span : Span) -> FlatID {
        self.working_on.instructions.alloc(Instruction::Wire(WireInstance{typ : FullType::new_unset(), span, source : WireSource::new_error()}))
    }

    /// Returns the ID of the [FuncCallInstruction]
    fn flatten_func_call(&mut self, cursor : &mut Cursor) -> Option<FlatID> {
        let whole_func_span = cursor.span();
        cursor.go_down(kind!("func_call"), |cursor| {
            cursor.field(field!("name"));
            let interface_reference = self.get_or_alloc_module(cursor);

            cursor.field(field!("arguments"));
            let arguments_span = BracketSpan::from_outer(cursor.span());
            let mut arguments = cursor.collect_list(kind!("parenthesis_expression_list"), |cursor| {
                self.flatten_expr(cursor)
            });

            let interface_reference = interface_reference?;
            
            let (md, interface) = self.get_interface_reference(&interface_reference);

            let func_call_inputs = interface.func_call_inputs;
            let func_call_outputs = interface.func_call_outputs;
            
            let arg_count = arguments.len();
            let expected_arg_count = func_call_inputs.len();

            if arg_count != expected_arg_count {
                if arg_count > expected_arg_count {
                    // Too many args, complain about excess args at the end
                    let excess_args_span = Span::new_overarching(self.working_on.instructions[arguments[expected_arg_count]].unwrap_wire().span, self.working_on.instructions[*arguments.last().unwrap()].unwrap_wire().span);
                    
                    self.errors
                        .error(excess_args_span, format!("Excess argument. Function takes {expected_arg_count} args, but {arg_count} were passed."))
                        .info_obj(&(md, interface));
                    // Shorten args to still get proper type checking for smaller arg array
                    arguments.truncate(expected_arg_count);
                } else {
                    // Too few args, mention missing argument names
                    self.errors
                        .error(arguments_span.close_bracket(), format!("Too few arguments. Function takes {expected_arg_count} args, but {arg_count} were passed."))
                        .info_obj(&(md, interface));

                    while arguments.len() < expected_arg_count {
                        arguments.push(self.alloc_error(arguments_span.close_bracket()));
                    }
                }
            }

            Some(self.working_on.instructions.alloc(Instruction::FuncCall(FuncCallInstruction{
                interface_reference,
                arguments,
                func_call_inputs,
                func_call_outputs,
                arguments_span,
                whole_func_span
            })))
        })
    }

    /// Produces a new [SubModuleInstance] if a global was passed, or a reference to the existing instance if it's referenced by name
    fn get_or_alloc_module(&mut self, cursor : &mut Cursor) -> Option<ModuleInterfaceReference> {
        let outer_span = cursor.span();
        match self.flatten_wire_reference(cursor) {
            PartialWireReference::Error => None,
            PartialWireReference::GlobalModuleName(module_ref) => {
                let documentation = cursor.extract_gathered_comments();
                let interface_span = module_ref.span;
                let submodule_decl = self.working_on.instructions.alloc(Instruction::SubModule(SubModuleInstance{
                    name : None,
                    module_ref,
                    declaration_runtime_depth : DECL_DEPTH_LATER,
                    local_interface_domains : FlatAlloc::new(),
                    documentation
                }));
                Some(ModuleInterfaceReference{
                    submodule_decl,
                    submodule_interface : Module::MAIN_INTERFACE_ID,
                    name_span : None,
                    interface_span
                })
            }
            PartialWireReference::ModuleButNoPort(submodule_decl, name_span) => {
                Some(ModuleInterfaceReference{
                    submodule_decl,
                    submodule_interface : Module::MAIN_INTERFACE_ID,
                    name_span : Some(name_span),
                    interface_span : name_span
                })
            }
            PartialWireReference::ModuleWithInterface { submodule_decl, submodule_name_span, interface, interface_name_span } => {
                Some(ModuleInterfaceReference{
                    submodule_decl,
                    submodule_interface : interface,
                    name_span : Some(submodule_name_span),
                    interface_span : interface_name_span
                })
            }
            PartialWireReference::WireReference(_wire_ref) => {
                self.errors
                    .error(outer_span, "Function call syntax is only possible on modules or interfaces of modules");
                None
            }
        }
    }

    fn get_interface_reference(&self, interface_reference : &ModuleInterfaceReference) -> (&Module, &Interface) {
        let submodule = self.working_on.instructions[interface_reference.submodule_decl].unwrap_submodule();
        let md = &self.modules[submodule.module_ref.id];
        let interface = &md.interfaces[interface_reference.submodule_interface];
        (md, interface)
    }

    fn flatten_expr(&mut self, cursor : &mut Cursor) -> FlatID {
        let (kind, expr_span) = cursor.kind_span();
        
        let source = if kind == kind!("number") {
            let text = &self.name_resolver.file_text[expr_span];
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
        } else if kind == kind!("func_call") {
            if let Some(fc_id) = self.flatten_func_call(cursor) {
                let fc = self.working_on.instructions[fc_id].unwrap_func_call();
                let (md, interface) = self.get_interface_reference(&fc.interface_reference);
                if interface.func_call_outputs.len() != 1 {
                    self.errors
                        .error(expr_span, "A function called in this context may only return one result. Split this function call into a separate line instead.")
                        .info_obj(&(md, interface));
                }

                if interface.func_call_outputs.len() >= 1 {
                    WireSource::WireRef(WireReference::simple_port(PortInfo{
                        submodule_name_span : fc.interface_reference.name_span,
                        submodule_decl: fc.interface_reference.submodule_decl,
                        port: interface.func_call_outputs.0,
                        port_name_span: None,
                        is_input: false,
                    }))
                } else {
                    // Function desugaring or using threw an error
                    WireSource::new_error()
                }
            } else {
                // Function desugaring or using threw an error
                WireSource::new_error()
            }
        } else if kind == kind!("parenthesis_expression") {
            return cursor.go_down_content(kind!("parenthesis_expression"), |cursor| self.flatten_expr(cursor));
        } else {
            if let Some(wr) = self.flatten_wire_reference(cursor).expect_wireref(self) {
                WireSource::WireRef(wr)
            } else {
                WireSource::new_error()
            }
        };

        let wire_instance = WireInstance{
            typ : FullType::new_unset(),
            span: expr_span,
            source
        };
        self.working_on.instructions.alloc(Instruction::Wire(wire_instance))
    }

    fn flatten_wire_reference(&mut self, cursor : &mut Cursor) -> PartialWireReference {
        let (kind, expr_span) = cursor.kind_span();
        if kind == kind!("template_global") {
            match self.flatten_local_or_template_global(cursor) {
                LocalOrGlobal::Local(span, named_obj) => {
                    match named_obj {
                        NamedLocal::Declaration(decl_id) => {
                            let root = WireReferenceRoot::LocalDecl(decl_id, expr_span);
                            PartialWireReference::WireReference(WireReference{root, path : Vec::new()})
                        }
                        NamedLocal::SubModule(submod_id) => PartialWireReference::ModuleButNoPort(submod_id, expr_span),
                        NamedLocal::TemplateType(template_id) => {
                            self.errors.error(span, format!("Expected a value, but instead found template type '{}'", self.working_on.link_info.template_arguments[template_id].name));
                            PartialWireReference::Error
                        }
                    }
                }
                LocalOrGlobal::Global(name_elem, span, template_args, template_span) => {
                    match name_elem {
                        NameElem::Constant(cst) => {
                            let root = WireReferenceRoot::NamedConstant(cst, expr_span);
                            PartialWireReference::WireReference(WireReference{root, path : Vec::new()})
                        }
                        NameElem::Module(md_id) => {
                            PartialWireReference::GlobalModuleName(GlobalReference { span, id: md_id, template_args, template_span })
                        }
                        NameElem::Type(_) => {
                            self.name_resolver.not_expected_global_error(name_elem, span, "named wire: local or constant");
                            PartialWireReference::Error
                        },
                    }
                }
                LocalOrGlobal::NotFound(_) => {
                    PartialWireReference::Error
                }
            }
        } else if kind == kind!("array_op") {
            cursor.go_down_no_check(|cursor| {
                cursor.field(field!("arr"));
                let mut flattened_arr_expr = self.flatten_wire_reference(cursor);
                
                cursor.field(field!("arr_idx"));
                let (idx, bracket_span) = self.flatten_array_bracket(cursor);
                
                // only unpack the subexpr after flattening the idx, so we catch all errors
                match &mut flattened_arr_expr {
                    PartialWireReference::ModuleButNoPort(_, _) | PartialWireReference::GlobalModuleName(_) | PartialWireReference::ModuleWithInterface { submodule_decl:_, submodule_name_span:_, interface:_, interface_name_span:_ }=> {
                        todo!("Module Arrays")
                    }
                    PartialWireReference::Error => {}
                    PartialWireReference::WireReference(wr) => {
                        wr.path.push(WireReferencePathElement::ArrayAccess{idx, bracket_span});
                    }
                }
                
                flattened_arr_expr
            })
        } else if kind == kind!("field_access") {
            cursor.go_down_no_check(|cursor| {
                cursor.field(field!("left"));
                let flattened_arr_expr = self.flatten_wire_reference(cursor);
                
                let port_name_span = cursor.field_span(field!("name"), kind!("identifier"));

                match flattened_arr_expr {
                    PartialWireReference::Error => PartialWireReference::Error,
                    PartialWireReference::GlobalModuleName(md_ref) => {
                        self.errors.error(md_ref.span, "Ports or interfaces can only be accessed on modules that have been explicitly declared. Declare this submodule on its own line");
                        PartialWireReference::Error
                    }
                    PartialWireReference::ModuleWithInterface { submodule_decl:_, submodule_name_span, interface:_, interface_name_span } => {
                        self.errors.error(port_name_span, "Omit the interface when accessing a port")
                            .suggest_remove(Span::new_overarching(submodule_name_span.empty_span_at_end(), interface_name_span));

                        PartialWireReference::Error
                    }
                    PartialWireReference::ModuleButNoPort(submodule_decl, submodule_name_span) => {
                        let submodule = self.working_on.instructions[submodule_decl].unwrap_submodule();

                        let submod = &self.modules[submodule.module_ref.id];

                        match submod.get_port_or_interface_by_name(port_name_span, &self.name_resolver.file_text, self.errors) {
                            Some(PortOrInterface::Port(port)) => {
                                let port_info = PortInfo{
                                    submodule_name_span : Some(submodule_name_span),
                                    submodule_decl,
                                    port,
                                    port_name_span : Some(port_name_span),
                                    is_input: submod.ports[port].is_input
                                };
                                PartialWireReference::WireReference(WireReference{
                                    root : WireReferenceRoot::SubModulePort(port_info),
                                    path : Vec::new()
                                })
                            }
                            Some(PortOrInterface::Interface(interface)) => {
                                PartialWireReference::ModuleWithInterface { submodule_decl, submodule_name_span, interface, interface_name_span: port_name_span }
                            }
                            None => PartialWireReference::Error
                        }
                    }
                    PartialWireReference::WireReference(_) => {
                        println!("TODO: Struct fields");
                        PartialWireReference::Error
                    }
                }
            })
        } else if kind == kind!("number") {self.errors.error(expr_span, "A constant is not a wire reference"); PartialWireReference::Error
        } else if kind == kind!("unary_op") {self.errors.error(expr_span, "The result of an operator is not a wire reference"); PartialWireReference::Error
        } else if kind == kind!("binary_op") {self.errors.error(expr_span, "The result of an operator is not a wire reference"); PartialWireReference::Error
        } else if kind == kind!("func_call") {self.errors.error(expr_span, "A submodule call is not a wire reference"); PartialWireReference::Error
        } else if kind == kind!("parenthesis_expression") {self.errors.error(expr_span, "Parentheses are not allowed within a wire reference"); PartialWireReference::Error
        } else {cursor.could_not_match()}
    }

    fn flatten_if_statement(&mut self, cursor : &mut Cursor) {
        cursor.go_down(kind!("if_statement"), |cursor| {
            cursor.field(field!("condition"));
            let condition = self.flatten_expr(cursor);
            
            let if_id = self.working_on.instructions.alloc(Instruction::IfStatement(IfStatement{condition, then_start : UUID::PLACEHOLDER, then_end_else_start : UUID::PLACEHOLDER, else_end : UUID::PLACEHOLDER}));
            let then_start = self.working_on.instructions.get_next_alloc_id();
            
            cursor.field(field!("then_block"));
            self.flatten_code(cursor);

            let then_end_else_start = self.working_on.instructions.get_next_alloc_id();
            if cursor.optional_field(field!("else_block")) {
                if cursor.kind() == kind!("if_statement") {
                    self.flatten_if_statement(cursor); // Chained if statements
                } else {
                    self.flatten_code(cursor)
                }
            };
            let else_end = self.working_on.instructions.get_next_alloc_id();
            
            let Instruction::IfStatement(if_stmt) = &mut self.working_on.instructions[if_id] else {unreachable!()};
            if_stmt.then_start = then_start;
            if_stmt.then_end_else_start = then_end_else_start;
            if_stmt.else_end = else_end;
        })
    }

    fn flatten_assign_function_call(&mut self, to : Vec<(Option<(WireReference, WriteModifiers)>, Span)>, cursor : &mut Cursor) {
        let func_call_span = cursor.span();
        let to_iter = if let Some(fc_id) = self.flatten_func_call(cursor) {
            let fc = self.working_on.instructions[fc_id].unwrap_func_call();

            let (md, interface) = self.get_interface_reference(&fc.interface_reference);
            let outputs = interface.func_call_outputs;
            // Already extract some fields here, so we don't keep a ref to fc
            let submodule_name_span = fc.interface_reference.name_span;
            let submodule_decl = fc.interface_reference.submodule_decl;

            let num_func_outputs = outputs.len();
            let num_targets = to.len();
            if num_targets != num_func_outputs {
                if num_targets > num_func_outputs {
                    let excess_results_span = Span::new_overarching(to[num_func_outputs].1, to.last().unwrap().1);
                    self.errors
                        .error(excess_results_span, format!("Excess output targets. Function returns {num_func_outputs} results, but {num_targets} targets were given."))
                        .info_obj(&(md, interface));
                } else {
                    self.errors
                        .error(func_call_span, format!("Too few output targets. Function returns {num_func_outputs} results, but {num_targets} targets were given."))
                        .info_obj(&(md, interface));
                }
            }

            let mut to_iter = to.into_iter();
            for port in outputs {
                if let Some((Some((to, write_modifiers)), to_span)) = to_iter.next() {
                    let from = self.working_on.instructions.alloc(Instruction::Wire(WireInstance{
                        typ: FullType::new_unset(),
                        span: func_call_span,
                        source: WireSource::WireRef(WireReference::simple_port(PortInfo{
                            port,
                            port_name_span: None,
                            is_input: false,
                            submodule_name_span,
                            submodule_decl
                        }))
                    }));
                    self.working_on.instructions.alloc(Instruction::Write(Write{from, to, to_span, write_modifiers}));
                }
            }
            to_iter
        } else {
            to.into_iter()
        };
        for leftover_to in to_iter {
            if let (Some((to, write_modifiers)), to_span) = leftover_to {
                let err_id = self.working_on.instructions.alloc(Instruction::Wire(WireInstance{typ : FullType::new_unset(), span : func_call_span, source : WireSource::new_error()}));
                self.working_on.instructions.alloc(Instruction::Write(Write{from: err_id, to, to_span, write_modifiers}));
            }
        }
    }

    fn flatten_code(&mut self, cursor : &mut Cursor) {
        let old_frame = self.local_variable_context.new_frame();
        
        self.flatten_code_keep_context(cursor);

        self.local_variable_context.pop_frame(old_frame);
    }
    fn flatten_code_keep_context(&mut self, cursor : &mut Cursor) {
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
                            self.errors.error(span, format!("Non-function assignments must output exactly 1 output instead of {}", to.len()));
                        }
                        if let Some((Some((to, write_modifiers)), to_span)) = to.into_iter().next() {
                            self.working_on.instructions.alloc(Instruction::Write(Write{from: read_side, to, to_span, write_modifiers}));
                        }
                    }
                });
            } else if kind == kind!("block") {
                self.flatten_code(cursor);
            } else if kind == kind!("if_statement") {
                self.flatten_if_statement(cursor);
            } else if kind == kind!("for_statement") {
                cursor.go_down_no_check(|cursor| {
                    let loop_var_decl_frame = self.local_variable_context.new_frame();
                    cursor.field(field!("for_decl"));
                    let loop_var_decl = self.flatten_declaration::<false>(DeclarationContext::ForLoopGenerative, true, true, cursor);

                    cursor.field(field!("from"));
                    let start = self.flatten_expr(cursor);

                    cursor.field(field!("to"));
                    let end = self.flatten_expr(cursor);
                    
                    let for_id = self.working_on.instructions.alloc(Instruction::ForStatement(ForStatement{loop_var_decl, start, end, loop_body: UUIDRange(UUID::PLACEHOLDER, UUID::PLACEHOLDER)}));

                    let code_start = self.working_on.instructions.get_next_alloc_id();

                    cursor.field(field!("block"));
                    // We already started a new local_variable_context to include the loop var
                    self.flatten_code_keep_context(cursor);
                    
                    let code_end = self.working_on.instructions.get_next_alloc_id();

                    let Instruction::ForStatement(for_stmt) = &mut self.working_on.instructions[for_id] else {unreachable!()};

                    for_stmt.loop_body = UUIDRange(code_start, code_end);

                    self.local_variable_context.pop_frame(loop_var_decl_frame);
                })
            } else if kind == kind!("interface_statement") {
                cursor.go_down_no_check(|cursor| {
                    // Skip name
                    cursor.field(field!("name"));

                    if cursor.optional_field(field!("interface_ports")) {
                        self.flatten_interface_ports(cursor);
                    }
                });
            } else {
                cursor.could_not_match()
            }
            cursor.clear_gathered_comments(); // Clear comments after every statement, so comments don't bleed over
        });
    }

    fn flatten_write_modifiers(&self, cursor : &mut Cursor) -> WriteModifiers {
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

    /// See [Self::flatten_standalone_decls]
    /// Two cases:
    /// - Left side of assignment:
    ///     No modules, Yes write modifiers, Only assignable expressions
    fn flatten_assignment_left_side(&mut self, cursor : &mut Cursor) -> Vec<(Option<(WireReference, WriteModifiers)>, Span)> {
        cursor.collect_list(kind!("assign_left_side"), |cursor| {
            cursor.go_down(kind!("assign_to"), |cursor| {
                let write_modifiers = self.flatten_write_modifiers(cursor);
                
                cursor.field(field!("expr_or_decl"));
                let (kind, span) = cursor.kind_span();

                (if kind == kind!("declaration") {
                    let root = self.flatten_declaration::<false>(DeclarationContext::PlainWire, false, true, cursor);
                    let flat_root_decl = self.working_on.instructions[root].unwrap_wire_declaration();
                    Some((WireReference{root : WireReferenceRoot::LocalDecl(root, flat_root_decl.name_span), path: Vec::new()}, write_modifiers))
                } else { // It's _expression
                    if let Some(wire_ref) = self.flatten_wire_reference(cursor).expect_wireref(self) {
                        Some((wire_ref, write_modifiers))
                    } else {
                        None
                    }
                }, span)
            })
        })
    }

    /// See [Self::flatten_assignment_left_side]
    /// - Standalone declarations:
    ///     Yes modules, No write modifiers, Yes expressions (-> single expressions)
    fn flatten_standalone_decls(&mut self, cursor : &mut Cursor) {
        let mut is_first_item = true;
        cursor.list(kind!("assign_left_side"), |cursor| {
            cursor.go_down(kind!("assign_to"), |cursor| {
                if !is_first_item {
                    self.errors.warn(cursor.span(), "Standalone declarations and expressions should be on their own line.");
                }
                is_first_item = false;

                if let Some(span) = cursor.optional_field_span(field!("write_modifiers"), kind!("write_modifiers")) {
                    self.errors.error(span, "No write modifiers are allowed on non-assigned to declarations or expressions");
                }
                
                cursor.field(field!("expr_or_decl"));
                let (kind, span) = cursor.kind_span();

                if kind == kind!("declaration") {
                    let _ = self.flatten_declaration::<true>(DeclarationContext::PlainWire, false, true, cursor);
                } else { // It's _expression
                    if kind == kind!("func_call") {
                        self.flatten_assign_function_call(Vec::new(), cursor);
                    } else {
                        self.errors.warn(span, "The result of this operation is not used");
                        let _ = self.flatten_expr(cursor);
                    }
                }
            });
        })
    }

    fn flatten_declaration_list(&mut self, declaration_context : DeclarationContext, read_only : bool, cursor : &mut Cursor) {
        cursor.list(kind!("declaration_list"), |cursor| {
            let _ = self.flatten_declaration::<false>(declaration_context, read_only, true, cursor);
        });
    }

    fn flatten_interface_ports(&mut self, cursor : &mut Cursor) {
        cursor.go_down(kind!("interface_ports"), |cursor| {
            if cursor.optional_field(field!("inputs")) {
                self.flatten_declaration_list(DeclarationContext::Input, true, cursor)
            }
            if cursor.optional_field(field!("outputs")) {
                self.flatten_declaration_list(DeclarationContext::Output, false, cursor)
            }
        })
    }

    fn flatten_module(&mut self, cursor : &mut Cursor) {
        cursor.go_down(kind!("module"), |cursor| {
            let name_span = cursor.field_span(field!("name"), kind!("identifier"));
            if cursor.optional_field(field!("template_declaration_arguments")) {
                cursor.list(kind!("template_declaration_arguments"), |cursor| {
                    cursor.go_down(kind!("template_declaration_type"), |cursor| {
                        // Already covered in initialization
                        cursor.field(field!("name"));
                        let default_type = cursor.optional_field(field!("default_value")).then(|| {
                            self.flatten_type(cursor)
                        });

                        let claimed_type_id = self.template_inputs_to_visit.next().unwrap();

                        let selected_arg = &mut self.working_on.link_info.template_arguments[claimed_type_id];
                        let TemplateInputKind::Type{default_value} = &mut selected_arg.kind else {unreachable!()};

                        *default_value = default_type;

                        let name_span = selected_arg.name_span;

                        self.alloc_local_name(name_span, NamedLocal::TemplateType(claimed_type_id));
                    });
                })
            }
            let module_name = &self.name_resolver.file_text[name_span];
            println!("TREE SITTER module! {module_name}");
            // Interface is allocated in self
            if cursor.optional_field(field!("interface_ports")) {
                self.flatten_interface_ports(cursor);
            }
            
            cursor.field(field!("block"));
            self.flatten_code(cursor);
        })
    }
}

/// This method flattens all given code into a simple set of assignments, operators and submodules. 
/// It already does basic type checking and assigns a type to every wire. 
/// The Generating Structure of the code is not yet executed. 
/// It is template-preserving
/// 
/// Separate 'md lifetime for the module. 
/// For some reason if it has the same lifetime as the linker ('l), 
/// then the compiler thinks we could store cursor elements in the module, which would be bad? 
/// Don't fully understand this, but separating the lifetimes makes it work. 
pub fn flatten<'cursor_linker, 'errs>(linker : *mut Linker, module_uuid : ModuleUUID, cursor : &mut Cursor) {
    with_module_editing_context(linker, module_uuid, |modules, types, constants, name_resolver| {
        println!("Flattening {}", modules.working_on.link_info.name);

        let mut context = FlatteningContext {
            ports_to_visit : modules.working_on.ports.id_range().into_iter(),
            template_inputs_to_visit : modules.working_on.link_info.template_arguments.id_range().into_iter(),
            errors : name_resolver.errors,
            modules, 
            types, 
            constants,
            name_resolver,
            local_variable_context : LocalVariableContext::new_initial()
        };
        
        context.flatten_module(cursor);
        
        // Make sure all ports have been visited
        assert!(context.ports_to_visit.is_empty());
        assert!(context.template_inputs_to_visit.is_empty());
    });
}

/// Flattens all modules in the project. 
/// 
/// Requires that first, all modules have been initialized. 
pub fn flatten_all_modules(linker : &mut Linker) {
    let linker_ptr : *mut Linker = linker;
    for (_file_id, file) in &linker.files {
        let mut span_debugger = SpanDebugger::new("flatten_all_modules", &file.file_text);
        let mut associated_value_iter = file.associated_values.iter();

        let mut cursor = Cursor::new_at_root(&file.tree, &file.file_text);

        cursor.list(kind!("source_file"), |cursor| {
            match cursor.kind() {
                kind!("module") => {
                    let Some(NameElem::Module(module_uuid)) = associated_value_iter.next() else {unreachable!()};

                    flatten(linker_ptr, *module_uuid, cursor);
                }
                other => todo!("{}", tree_sitter_sus::language().node_kind_for_id(other).unwrap())
            }
        });
        span_debugger.defuse();
    }
}
