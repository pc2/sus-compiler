use crate::alloc::{ArenaAllocator, UUIDRange, UUID};
use crate::{alloc::UUIDRangeIter, prelude::*};

use std::
    str::FromStr
;

use num::BigInt;
use sus_proc_macro::{field, kind, kw};

use crate::linker::{
    make_resolvers, FileData, NameElem, NameResolver, NamedConstant, Resolver
};
use crate::{debug::SpanDebugger, value::Value};

use super::name_context::LocalVariableContext;
use super::parser::Cursor;
use super::*;

use crate::typing::template::{
    GenerativeTemplateInputKind, TemplateArg, TemplateArgKind, TemplateArgs, TemplateInputKind,
};

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
    NotFound(Span),
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
    ModuleWithInterface {
        submodule_decl: FlatID,
        submodule_name_span: Span,
        interface: InterfaceID,
        interface_name_span: Span,
    },
    /// It's ready for use higher up
    WireReference(WireReference),
}

impl PartialWireReference {
    fn expect_wireref(self, ctx: &FlatteningContext) -> Option<WireReference> {
        match self {
            PartialWireReference::Error => None, // Error already reported
            PartialWireReference::ModuleButNoPort(submod_decl, span) => {
                let md_uuid = ctx.instructions[submod_decl]
                    .unwrap_submodule()
                    .module_ref
                    .id;
                ctx.errors
                    .error(
                        span,
                        "cannot operate on modules directly. Should use ports instead",
                    )
                    .info_obj(&ctx.modules[md_uuid]);
                None
            }
            PartialWireReference::GlobalModuleName(md_ref) => {
                let md = &ctx.modules[md_ref.id];
                ctx.errors
                    .error(
                        md_ref.span,
                        format!(
                            "Expected a Wire Reference, but found module '{}' instead",
                            md.link_info.name
                        ),
                    )
                    .info_obj(md);
                None
            }
            PartialWireReference::ModuleWithInterface {
                submodule_decl: submod_decl,
                submodule_name_span: _,
                interface,
                interface_name_span,
            } => {
                let md_uuid = ctx.instructions[submod_decl]
                    .unwrap_submodule()
                    .module_ref
                    .id;
                let md = &ctx.modules[md_uuid];
                let interf = &md.interfaces[interface];
                ctx.errors
                    .error(
                        interface_name_span,
                        format!(
                            "Expected a port, but found module interface '{}' instead",
                            &interf.name
                        ),
                    )
                    .info((interf.name_span, md.link_info.file), "Declared here");
                None
            }
            PartialWireReference::WireReference(wr) => Some(wr),
        }
    }
}

impl UnaryOperator {
    pub fn from_kind_id(kind_id: u16) -> Self {
        match kind_id {
            kw!("+") => UnaryOperator::Sum,
            kw!("*") => UnaryOperator::Product,
            kw!("-") => UnaryOperator::Negate,
            kw!("&") => UnaryOperator::And,
            kw!("|") => UnaryOperator::Or,
            kw!("^") => UnaryOperator::Xor,
            kw!("!") => UnaryOperator::Not,
            _ => unreachable!(),
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
    pub fn from_kind_id(kind_id: u16) -> Self {
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
            _ => unreachable!(),
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
    IO{is_input : bool},
    ForLoopGenerative,
    PlainWire,
    StructField
}

struct FlatteningContext<'l, 'errs> {
    modules: Resolver<'l, 'errs, ModuleUUIDMarker, Module>,
    #[allow(dead_code)]
    types: Resolver<'l, 'errs, TypeUUIDMarker, StructType>,
    #[allow(dead_code)]
    constants: Resolver<'l, 'errs, ConstantUUIDMarker, NamedConstant>,
    name_resolver: NameResolver<'l, 'errs>,
    errors: &'errs ErrorCollector<'l>,

    working_on_link_info: &'l LinkInfo,
    instructions: FlatAlloc<Instruction, FlatIDMarker>,

    fields_to_visit: UUIDRangeIter<FieldIDMarker>,
    ports_to_visit: UUIDRangeIter<PortIDMarker>,
    template_inputs_to_visit: UUIDRangeIter<TemplateIDMarker>,

    local_variable_context: LocalVariableContext<'l, NamedLocal>,

    default_declaration_context: DeclarationContext
}

impl<'l, 'errs : 'l> FlatteningContext<'l, 'errs> {
    fn flatten_template_inputs(&mut self, cursor: &mut Cursor) {
        if cursor.optional_field(field!("template_declaration_arguments")) {
            cursor.list(kind!("template_declaration_arguments"), |cursor| {
                cursor.go_down(kind!("template_declaration_type"), |cursor| {
                    // Already covered in initialization
                    cursor.field(field!("name"));

                    let claimed_type_id = self.template_inputs_to_visit.next().unwrap();

                    let selected_arg = &self.working_on_link_info.template_arguments[claimed_type_id];

                    let name_span = selected_arg.name_span;

                    self.alloc_local_name(name_span, NamedLocal::TemplateType(claimed_type_id));
                });
            })
        }
    }

    fn get_link_info_for(&self, found_global: NameElem) -> &'l LinkInfo {
        let info : *const LinkInfo = match found_global {
            NameElem::Module(md_id) => &self.modules[md_id].link_info,
            NameElem::Type(typ_id) => &self.types[typ_id].link_info,
            NameElem::Constant(_) => todo!("Constants don't have LinkInfo")
        };
        // SAFETY Can safely cast this away, because we can't touch anything in the Linker
        unsafe{&*info}
    }

    fn flatten_template_args(&mut self, found_global: NameElem, has_template_args: bool, cursor: &mut Cursor) -> TemplateArgs {
        if !has_template_args {return FlatAlloc::new();}
        
        let link_info = self.get_link_info_for(found_global);
        let full_object_name = link_info.get_full_name();

        let mut template_arg_map : FlatAlloc<Option<TemplateArg>, TemplateIDMarker> = link_info.template_arguments.map(|_| None);
        

        cursor.list(kind!("template_args"), |cursor| {
            cursor.go_down(kind!("template_arg"), |cursor| {
                let name_span =
                    cursor.field_span(field!("name"), kind!("identifier"));

                let name = &self.name_resolver.file_text[name_span];

                let name_found = link_info.template_arguments.iter().find(|(_id, arg)| arg.name == name);
                if name_found.is_none() {
                    self.errors.error(name_span, format!("{name} is not a valid template argument of {full_object_name}"))
                        .info_obj(link_info);
                }

                let template_arg = if cursor.optional_field(field!("val_arg")) {
                    let expr = self.flatten_expr(cursor);
                    TemplateArgKind::Value(expr)
                } else if cursor.optional_field(field!("type_arg")) {
                    let typ = self.flatten_type(cursor);
                    TemplateArgKind::Type(typ)
                } else {
                    match self.local_variable_context.get_declaration_for(name) {
                        Some(NamedLocal::TemplateType(t)) => TemplateArgKind::Type(WrittenType::Template(name_span, t)),
                        Some(NamedLocal::Declaration(decl_id)) => {
                            let wire_read_id = self.instructions.alloc(Instruction::Wire(WireInstance { 
                                typ: FullType::new_unset(),
                                span: name_span,
                                source: WireSource::WireRef(WireReference::simple_var_read(decl_id, name_span)) 
                            }));
                            TemplateArgKind::Value(wire_read_id)
                        }
                        Some(NamedLocal::SubModule(_sm)) => {
                            self.errors.error(name_span, format!("{name} does not name a Type or a Value. Local submodules are not allowed!"));
                            return;
                        }
                        None => {
                            self.errors.error(name_span, format!("{name} does not name a Type or a Value."));
                            return;
                        },
                    }
                };

                if let Some((id, template_input)) = name_found {
                    match (&template_input.kind, &template_arg) {
                        (TemplateInputKind::Type(_), TemplateArgKind::Type(_)) 
                        | (TemplateInputKind::Generative(_), TemplateArgKind::Value(_)) => {
                            // Correct pairing
                            if let Some(prev) = &template_arg_map[id] {
                                self.errors.error(name_span, format!("'{name}' has already been defined previously"))
                                    .info_same_file(prev.name_span, "Defined here previously");
                            } else {
                                template_arg_map[id] = Some(TemplateArg { name_span, kind: template_arg });
                            }
                        }
                        (TemplateInputKind::Type(_), TemplateArgKind::Value(_)) => {
                            self.errors.error(name_span, format!("'{name}' is not a value. `type` keyword cannot be used for values"))
                                .info((template_input.name_span, link_info.file), "Declared here");
                        }
                        (TemplateInputKind::Generative(_), TemplateArgKind::Type(_)) => {
                            self.errors.error(name_span, format!("'{name}' is not a type. To use template type arguments use the `type` keyword like `T: type int[123]`"))
                                .info((template_input.name_span, link_info.file), "Declared here");
                        }
                    }
                }
            });
        });

        template_arg_map
    }

    fn flatten_local_or_template_global(&mut self, cursor: &mut Cursor) -> LocalOrGlobal {
        cursor.go_down(kind!("template_global"), |cursor| {
            let mut must_be_global = cursor.optional_field(field!("is_global_path"));

            let name_path = cursor.collect_list(kind!("namespace_list"), |cursor| {
                let (kind, span) = cursor.kind_span();
                assert!(kind == kind!("identifier"));
                span
            });

            if name_path.len() > 1 {
                must_be_global = true;
            }

            let template_args_used = cursor.optional_field(field!("template_args"));

            must_be_global |= template_args_used;

            // Possibly local
            if !must_be_global {
                let [local_name] = name_path.as_slice() else {
                    unreachable!()
                };
                let name_text = &self.name_resolver.file_text[*local_name];
                if let Some(decl_id) = self.local_variable_context.get_declaration_for(name_text) {
                    return LocalOrGlobal::Local(*local_name, decl_id);
                }
            }

            // Global identifier
            let [name_span] = name_path.as_slice() else {
                self.errors.todo(name_path[1], "Namespaces");
                return LocalOrGlobal::NotFound(name_path[0]);
            };
            if let Some((found_global, found_global_span)) = self.name_resolver.resolve_global(*name_span) {
                // MUST Still be at field!("template_args")
                let template_args_whole_span = template_args_used.then(|| BracketSpan::from_outer(cursor.span()));

                let template_arg_map = self.flatten_template_args(found_global, template_args_used, cursor);

                LocalOrGlobal::Global(
                    found_global,
                    found_global_span,
                    template_arg_map,
                    template_args_whole_span,
                )
            } else {
                LocalOrGlobal::NotFound(*name_span)
            }
        })
    }

    fn flatten_array_type(&mut self, span: Span, cursor: &mut Cursor) -> WrittenType {
        cursor.go_down(kind!("array_type"), |cursor| {
            cursor.field(field!("arr"));
            let array_element_type = self.flatten_type(cursor);

            cursor.field(field!("arr_idx"));
            let (array_size_wire_id, bracket_span) = self.flatten_array_bracket(cursor);

            WrittenType::Array(
                span,
                Box::new((array_element_type, array_size_wire_id, bracket_span)),
            )
        })
    }

    fn flatten_type(&mut self, cursor: &mut Cursor) -> WrittenType {
        let ModuleOrWrittenType::WrittenType(wr_typ) = self.flatten_module_or_type::<false>(cursor)
        else {
            unreachable!("Can't not be type")
        };
        wr_typ
    }

    fn flatten_module_or_type<const ALLOW_MODULES: bool>(
        &mut self,
        cursor: &mut Cursor,
    ) -> ModuleOrWrittenType {
        let accepted_text = if ALLOW_MODULES {
            "Type or Module"
        } else {
            "Type"
        };
        let (kind, span) = cursor.kind_span();
        // Only difference is that
        if kind == kind!("template_global") {
            match self.flatten_local_or_template_global(cursor) {
                LocalOrGlobal::Local(span, NamedLocal::Declaration(instr))
                | LocalOrGlobal::Local(span, NamedLocal::SubModule(instr)) => {
                    self.errors
                        .error(
                            span,
                            format!(
                                "This is not a {accepted_text}, it is a local variable instead!"
                            ),
                        )
                        .info_obj_same_file(&self.instructions[instr]);

                    ModuleOrWrittenType::WrittenType(WrittenType::Error(span))
                }
                LocalOrGlobal::Local(span, NamedLocal::TemplateType(template_id)) => {
                    ModuleOrWrittenType::WrittenType(WrittenType::Template(span, template_id))
                }
                LocalOrGlobal::Global(
                    resolved_global,
                    resolved_global_span,
                    template_args,
                    template_span,
                ) => match &resolved_global {
                    NameElem::Type(typ_id) => {
                        ModuleOrWrittenType::WrittenType(WrittenType::Named(GlobalReference {
                            span: resolved_global_span,
                            id: *typ_id,
                            template_args,
                            template_span,
                        }))
                    }
                    NameElem::Module(md_id) if ALLOW_MODULES => {
                        ModuleOrWrittenType::Module(GlobalReference {
                            span: resolved_global_span,
                            id: *md_id,
                            template_args,
                            template_span,
                        })
                    }
                    _ => {
                        self.name_resolver.not_expected_global_error(
                            resolved_global,
                            resolved_global_span,
                            accepted_text,
                        );
                        ModuleOrWrittenType::WrittenType(WrittenType::Error(resolved_global_span))
                    }
                },
                LocalOrGlobal::NotFound(name_span) => {
                    ModuleOrWrittenType::WrittenType(WrittenType::Error(name_span))
                } // Already covered
            }
        } else if kind == kind!("array_type") {
            ModuleOrWrittenType::WrittenType(self.flatten_array_type(span, cursor))
        } else {
            cursor.could_not_match()
        }
    }

    fn alloc_local_name(&mut self, name_span: Span, named_local: NamedLocal) {
        if let Err(conflict) = self
            .local_variable_context
            .add_declaration(&self.name_resolver.file_text[name_span], named_local)
        {
            let err_ref = self.errors.error(
                name_span,
                "This declaration conflicts with a previous declaration in the same scope",
            );

            match conflict {
                NamedLocal::Declaration(decl_id) => {
                    err_ref.info_obj_same_file(
                        self.instructions[decl_id].unwrap_wire_declaration(),
                    );
                }
                NamedLocal::SubModule(submod_id) => {
                    err_ref.info_obj_same_file(
                        self.instructions[submod_id].unwrap_submodule(),
                    );
                }
                NamedLocal::TemplateType(template_id) => {
                    err_ref.info_obj_same_file(
                        &self.working_on_link_info.template_arguments[template_id],
                    );
                }
            }
        }
    }

    fn flatten_declaration<const ALLOW_MODULES: bool>(
        &mut self,
        declaration_context: DeclarationContext,
        mut read_only: bool,
        declaration_itself_is_not_written_to: bool,
        cursor: &mut Cursor,
    ) -> FlatID {
        let whole_declaration_span = cursor.span();
        cursor.go_down(kind!("declaration"), |cursor| {
            // Extra inputs and outputs declared in the body of the module
            let io_kw = cursor.optional_field(field!("io_port_modifiers")).then(|| {
                let (k, span) = cursor.kind_span();
                match k {
                    kw!("input") => (true, span),
                    kw!("output") => (false, span),
                    _ => cursor.could_not_match(),
                }
            });

            // State or Generative
            let declaration_modifiers = cursor.optional_field(field!("declaration_modifiers")).then(|| cursor.kind_span());

            // Still gets overwritten 
            let mut is_port = match declaration_context {
                DeclarationContext::IO{is_input} => {
                    if let Some((_, io_span)) = io_kw {
                        self.errors.error(io_span, "Cannot redeclare 'input' or 'output' on functional syntax IO");
                    }
                    DeclarationPortInfo::RegularPort { is_input, port_id: PortID::PLACEHOLDER }
                }
                DeclarationContext::ForLoopGenerative => {
                    if let Some((_, io_span)) = io_kw {
                        self.errors.error(io_span, "Cannot declare 'input' or 'output' to the iterator of a for loop");
                    }
                    DeclarationPortInfo::NotPort
                }
                DeclarationContext::PlainWire => {
                    match io_kw {
                        Some((is_input, _)) => DeclarationPortInfo::RegularPort { is_input, port_id: PortID::PLACEHOLDER },
                        None => DeclarationPortInfo::NotPort,
                    }
                }
                DeclarationContext::StructField => {
                    if let Some((_, io_span)) = io_kw {
                        self.errors.error(io_span, "Cannot declare 'input' or 'output' in a struct");
                    }
                    DeclarationPortInfo::StructField { field_id: UUID::PLACEHOLDER }
                }
            };

            let identifier_type = match declaration_context {
                DeclarationContext::IO{is_input:_} | DeclarationContext::PlainWire | DeclarationContext::StructField => {
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
                                DeclarationPortInfo::RegularPort { is_input : true, port_id : _ } | DeclarationPortInfo::StructField { field_id:_ }=> {
                                    // AHA! Generative input
                                    is_port = DeclarationPortInfo::GenerativeInput(TemplateID::PLACEHOLDER)
                                }
                                DeclarationPortInfo::RegularPort { is_input : false, port_id : _ } => {
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

            match &mut is_port {
                DeclarationPortInfo::NotPort => {}
                DeclarationPortInfo::StructField { field_id } => {*field_id = self.fields_to_visit.next().unwrap();}
                DeclarationPortInfo::RegularPort { is_input:_, port_id } => {*port_id = self.ports_to_visit.next().unwrap();}
                DeclarationPortInfo::GenerativeInput(template_id) => {*template_id = self.template_inputs_to_visit.next().unwrap();}
            }

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

                    let submod_id = self.instructions.alloc(Instruction::SubModule(SubModuleInstance{
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

            if is_port.implies_read_only() {
                read_only = true;
            }

            let decl_id = self.instructions.alloc(Instruction::Declaration(Declaration{
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

            decl_id
        })
    }

    fn flatten_array_bracket(&mut self, cursor: &mut Cursor) -> (FlatID, BracketSpan) {
        let bracket_span = BracketSpan::from_outer(cursor.span());
        cursor.go_down_content(kind!("array_bracket_expression"), |cursor| {
            (self.flatten_expr(cursor), bracket_span)
        })
    }

    fn alloc_error(&mut self, span: Span) -> FlatID {
        self.instructions.alloc(Instruction::Wire(WireInstance {
            typ: FullType::new_unset(),
            span,
            source: WireSource::new_error(),
        }))
    }

    /// Returns the ID of the [FuncCallInstruction]
    fn flatten_func_call(&mut self, cursor: &mut Cursor) -> Option<FlatID> {
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
                    let excess_args_span = Span::new_overarching(self.instructions[arguments[expected_arg_count]].unwrap_wire().span, self.instructions[*arguments.last().unwrap()].unwrap_wire().span);

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

            Some(self.instructions.alloc(Instruction::FuncCall(FuncCallInstruction{
                interface_reference,
                arguments,
                func_call_inputs,
                func_call_outputs,
                arguments_span,
                whole_func_span
            })))
        })
    }

    fn get_main_interface(&self, submodule_decl: FlatID) -> Option<(InterfaceID, &Interface)> {
        let sm = self.instructions[submodule_decl].unwrap_submodule();

        let md = &self.modules[sm.module_ref.id];

        md.get_main_interface()
    }

    /// Produces a new [SubModuleInstance] if a global was passed, or a reference to the existing instance if it's referenced by name
    fn get_or_alloc_module(&mut self, cursor: &mut Cursor) -> Option<ModuleInterfaceReference> {
        let outer_span = cursor.span();

        match self.flatten_wire_reference(cursor) {
            PartialWireReference::Error => None,
            PartialWireReference::GlobalModuleName(module_ref) => {
                let documentation = cursor.extract_gathered_comments();
                let interface_span = module_ref.span;
                let submodule_decl =
                    self.instructions.alloc(Instruction::SubModule(SubModuleInstance {
                        name: None,
                        module_ref,
                        declaration_runtime_depth: DECL_DEPTH_LATER,
                        local_interface_domains: FlatAlloc::new(),
                        documentation,
                    }));
                Some(ModuleInterfaceReference {
                    submodule_decl,
                    submodule_interface: self.get_main_interface(submodule_decl)?.0,
                    name_span: None,
                    interface_span,
                })
            }
            PartialWireReference::ModuleButNoPort(submodule_decl, name_span) => {
                Some(ModuleInterfaceReference {
                    submodule_decl,
                    submodule_interface: self.get_main_interface(submodule_decl)?.0,
                    name_span: Some(name_span),
                    interface_span: name_span,
                })
            }
            PartialWireReference::ModuleWithInterface {
                submodule_decl,
                submodule_name_span,
                interface,
                interface_name_span,
            } => Some(ModuleInterfaceReference {
                submodule_decl,
                submodule_interface: interface,
                name_span: Some(submodule_name_span),
                interface_span: interface_name_span,
            }),
            PartialWireReference::WireReference(_wire_ref) => {
                self.errors.error(
                    outer_span,
                    "Function call syntax is only possible on modules or interfaces of modules",
                );
                None
            }
        }
    }

    fn get_interface_reference(
        &self,
        interface_reference: &ModuleInterfaceReference,
    ) -> (&Module, &Interface) {
        let submodule =
            self.instructions[interface_reference.submodule_decl].unwrap_submodule();
        let md = &self.modules[submodule.module_ref.id];
        let interface = &md.interfaces[interface_reference.submodule_interface];
        (md, interface)
    }

    fn flatten_expr(&mut self, cursor: &mut Cursor) -> FlatID {
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

                WireSource::UnaryOp { op, right }
            })
        } else if kind == kind!("binary_op") {
            cursor.go_down_no_check(|cursor| {
                cursor.field(field!("left"));
                let left = self.flatten_expr(cursor);

                cursor.field(field!("operator"));
                let op = BinaryOperator::from_kind_id(cursor.kind());

                cursor.field(field!("right"));
                let right = self.flatten_expr(cursor);

                WireSource::BinaryOp { op, left, right }
            })
        } else if kind == kind!("func_call") {
            if let Some(fc_id) = self.flatten_func_call(cursor) {
                let fc = self.instructions[fc_id].unwrap_func_call();
                let (md, interface) = self.get_interface_reference(&fc.interface_reference);
                if interface.func_call_outputs.len() != 1 {
                    self.errors
                        .error(expr_span, "A function called in this context may only return one result. Split this function call into a separate line instead.")
                        .info_obj(&(md, interface));
                }

                if interface.func_call_outputs.len() >= 1 {
                    WireSource::WireRef(WireReference::simple_port(PortInfo {
                        submodule_name_span: fc.interface_reference.name_span,
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
            return cursor.go_down_content(kind!("parenthesis_expression"), |cursor| {
                self.flatten_expr(cursor)
            });
        } else {
            if let Some(wr) = self.flatten_wire_reference(cursor).expect_wireref(self) {
                WireSource::WireRef(wr)
            } else {
                WireSource::new_error()
            }
        };

        let wire_instance = WireInstance {
            typ: FullType::new_unset(),
            span: expr_span,
            source,
        };
        self.instructions
            .alloc(Instruction::Wire(wire_instance))
    }

    fn flatten_wire_reference(&mut self, cursor: &mut Cursor) -> PartialWireReference {
        let (kind, expr_span) = cursor.kind_span();
        if kind == kind!("template_global") {
            match self.flatten_local_or_template_global(cursor) {
                LocalOrGlobal::Local(span, named_obj) => match named_obj {
                    NamedLocal::Declaration(decl_id) => {
                        let root = WireReferenceRoot::LocalDecl(decl_id, expr_span);
                        PartialWireReference::WireReference(WireReference {
                            root,
                            path: Vec::new(),
                        })
                    }
                    NamedLocal::SubModule(submod_id) => {
                        PartialWireReference::ModuleButNoPort(submod_id, expr_span)
                    }
                    NamedLocal::TemplateType(template_id) => {
                        self.errors.error(
                            span,
                            format!(
                                "Expected a value, but instead found template type '{}'",
                                self.working_on_link_info.template_arguments[template_id].name
                            ),
                        );
                        PartialWireReference::Error
                    }
                },
                LocalOrGlobal::Global(name_elem, span, template_args, template_span) => {
                    match name_elem {
                        NameElem::Constant(cst) => {
                            let root = WireReferenceRoot::NamedConstant(cst, expr_span);
                            PartialWireReference::WireReference(WireReference {
                                root,
                                path: Vec::new(),
                            })
                        }
                        NameElem::Module(md_id) => {
                            PartialWireReference::GlobalModuleName(GlobalReference {
                                span,
                                id: md_id,
                                template_args,
                                template_span,
                            })
                        }
                        NameElem::Type(_) => {
                            self.name_resolver.not_expected_global_error(
                                name_elem,
                                span,
                                "named wire: local or constant",
                            );
                            PartialWireReference::Error
                        }
                    }
                }
                LocalOrGlobal::NotFound(_) => PartialWireReference::Error,
            }
        } else if kind == kind!("array_op") {
            cursor.go_down_no_check(|cursor| {
                cursor.field(field!("arr"));
                let mut flattened_arr_expr = self.flatten_wire_reference(cursor);

                cursor.field(field!("arr_idx"));
                let (idx, bracket_span) = self.flatten_array_bracket(cursor);

                // only unpack the subexpr after flattening the idx, so we catch all errors
                match &mut flattened_arr_expr {
                    PartialWireReference::ModuleButNoPort(_, _)
                    | PartialWireReference::GlobalModuleName(_)
                    | PartialWireReference::ModuleWithInterface {
                        submodule_decl: _,
                        submodule_name_span: _,
                        interface: _,
                        interface_name_span: _,
                    } => {
                        todo!("Module Arrays")
                    }
                    PartialWireReference::Error => {}
                    PartialWireReference::WireReference(wr) => {
                        wr.path
                            .push(WireReferencePathElement::ArrayAccess { idx, bracket_span });
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
                        let submodule = self.instructions[submodule_decl].unwrap_submodule();

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
        } else if kind == kind!("number") {
            self.errors
                .error(expr_span, "A constant is not a wire reference");
            PartialWireReference::Error
        } else if kind == kind!("unary_op") {
            self.errors.error(
                expr_span,
                "The result of an operator is not a wire reference",
            );
            PartialWireReference::Error
        } else if kind == kind!("binary_op") {
            self.errors.error(
                expr_span,
                "The result of an operator is not a wire reference",
            );
            PartialWireReference::Error
        } else if kind == kind!("func_call") {
            self.errors
                .error(expr_span, "A submodule call is not a wire reference");
            PartialWireReference::Error
        } else if kind == kind!("parenthesis_expression") {
            self.errors.error(
                expr_span,
                "Parentheses are not allowed within a wire reference",
            );
            PartialWireReference::Error
        } else {
            cursor.could_not_match()
        }
    }

    fn flatten_if_statement(&mut self, cursor: &mut Cursor) {
        cursor.go_down(kind!("if_statement"), |cursor| {
            cursor.field(field!("condition"));
            let condition = self.flatten_expr(cursor);

            let if_id = self.instructions.alloc(Instruction::IfStatement(IfStatement {
                condition,
                then_start: FlatID::PLACEHOLDER,
                then_end_else_start: FlatID::PLACEHOLDER,
                else_end: FlatID::PLACEHOLDER,
            }));
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

            let Instruction::IfStatement(if_stmt) = &mut self.instructions[if_id] else {
                unreachable!()
            };
            if_stmt.then_start = then_start;
            if_stmt.then_end_else_start = then_end_else_start;
            if_stmt.else_end = else_end;
        })
    }

    fn flatten_assign_function_call(
        &mut self,
        to: Vec<(Option<(WireReference, WriteModifiers)>, Span)>,
        cursor: &mut Cursor,
    ) {
        let func_call_span = cursor.span();
        let to_iter = if let Some(fc_id) = self.flatten_func_call(cursor) {
            let fc = self.instructions[fc_id].unwrap_func_call();

            let (md, interface) = self.get_interface_reference(&fc.interface_reference);
            let outputs = interface.func_call_outputs;
            // Already extract some fields here, so we don't keep a ref to fc
            let submodule_name_span = fc.interface_reference.name_span;
            let submodule_decl = fc.interface_reference.submodule_decl;

            let num_func_outputs = outputs.len();
            let num_targets = to.len();
            if num_targets != num_func_outputs {
                if num_targets > num_func_outputs {
                    let excess_results_span =
                        Span::new_overarching(to[num_func_outputs].1, to.last().unwrap().1);
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
                    let from =
                        self.instructions.alloc(Instruction::Wire(WireInstance {
                            typ: FullType::new_unset(),
                            span: func_call_span,
                            source: WireSource::WireRef(WireReference::simple_port(PortInfo {
                                port,
                                port_name_span: None,
                                is_input: false,
                                submodule_name_span,
                                submodule_decl,
                            })),
                        }));
                    self.instructions.alloc(Instruction::Write(Write {
                        from,
                        to,
                        to_span,
                        write_modifiers,
                    }));
                }
            }
            to_iter
        } else {
            to.into_iter()
        };
        for leftover_to in to_iter {
            if let (Some((to, write_modifiers)), to_span) = leftover_to {
                let err_id = self.instructions.alloc(Instruction::Wire(WireInstance {
                    typ: FullType::new_unset(),
                    span: func_call_span,
                    source: WireSource::new_error(),
                }));
                self.instructions.alloc(Instruction::Write(Write {
                    from: err_id,
                    to,
                    to_span,
                    write_modifiers,
                }));
            }
        }
    }

    fn flatten_code(&mut self, cursor: &mut Cursor) {
        let old_frame = self.local_variable_context.new_frame();

        self.flatten_code_keep_context(cursor);

        self.local_variable_context.pop_frame(old_frame);
    }
    fn flatten_code_keep_context(&mut self, cursor: &mut Cursor) {
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
                            self.instructions.alloc(Instruction::Write(Write{from: read_side, to, to_span, write_modifiers}));
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

                    let for_id = self.instructions.alloc(Instruction::ForStatement(ForStatement{loop_var_decl, start, end, loop_body: FlatIDRange::PLACEHOLDER}));

                    let code_start = self.instructions.get_next_alloc_id();

                    cursor.field(field!("block"));
                    // We already started a new local_variable_context to include the loop var
                    self.flatten_code_keep_context(cursor);

                    let code_end = self.instructions.get_next_alloc_id();

                    let Instruction::ForStatement(for_stmt) = &mut self.instructions[for_id] else {unreachable!()};

                    for_stmt.loop_body = FlatIDRange::new(code_start, code_end);

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
            } else if kind == kind!("domain_statement") {
                // Skip, because we already covered domains in initialization. 
                // TODO synchronous & async clocks
            } else {
                cursor.could_not_match()
            }
            cursor.clear_gathered_comments(); // Clear comments after every statement, so comments don't bleed over
        });
    }

    fn flatten_write_modifiers(&self, cursor: &mut Cursor) -> WriteModifiers {
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
                (0, num_regs) => WriteModifiers::Connection {
                    num_regs,
                    regs_span: modifiers_span,
                },
                (1, 0) => WriteModifiers::Initial {
                    initial_kw_span: modifiers_span,
                },
                _other => unreachable!(),
            }
        } else {
            WriteModifiers::Connection {
                num_regs: 0,
                regs_span: cursor.span().empty_span_at_front(),
            }
        }
    }

    /// See [Self::flatten_standalone_decls]
    /// Two cases:
    /// - Left side of assignment:
    ///     No modules, Yes write modifiers, Only assignable expressions
    fn flatten_assignment_left_side(
        &mut self,
        cursor: &mut Cursor,
    ) -> Vec<(Option<(WireReference, WriteModifiers)>, Span)> {
        cursor.collect_list(kind!("assign_left_side"), |cursor| {
            cursor.go_down(kind!("assign_to"), |cursor| {
                let write_modifiers = self.flatten_write_modifiers(cursor);

                cursor.field(field!("expr_or_decl"));
                let (kind, span) = cursor.kind_span();

                (
                    if kind == kind!("declaration") {
                        let root = self.flatten_declaration::<false>(
                            self.default_declaration_context,
                            false,
                            true,
                            cursor,
                        );
                        let flat_root_decl =
                            self.instructions[root].unwrap_wire_declaration();
                        Some((
                            WireReference {
                                root: WireReferenceRoot::LocalDecl(root, flat_root_decl.name_span),
                                path: Vec::new(),
                            },
                            write_modifiers,
                        ))
                    } else {
                        // It's _expression
                        if let Some(wire_ref) =
                            self.flatten_wire_reference(cursor).expect_wireref(self)
                        {
                            Some((wire_ref, write_modifiers))
                        } else {
                            None
                        }
                    },
                    span,
                )
            })
        })
    }

    /// See [Self::flatten_assignment_left_side]
    /// - Standalone declarations:
    ///     Yes modules, No write modifiers, Yes expressions (-> single expressions)
    fn flatten_standalone_decls(&mut self, cursor: &mut Cursor) {
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
                    let _ = self.flatten_declaration::<true>(self.default_declaration_context, false, true, cursor);
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

    fn flatten_declaration_list(
        &mut self,
        declaration_context: DeclarationContext,
        read_only: bool,
        cursor: &mut Cursor,
    ) {
        cursor.list(kind!("declaration_list"), |cursor| {
            let _ = self.flatten_declaration::<false>(declaration_context, read_only, true, cursor);
        });
    }

    fn flatten_interface_ports(&mut self, cursor: &mut Cursor) {
        cursor.go_down(kind!("interface_ports"), |cursor| {
            if cursor.optional_field(field!("inputs")) {
                self.flatten_declaration_list(DeclarationContext::IO{is_input:true}, true, cursor)
            }
            if cursor.optional_field(field!("outputs")) {
                self.flatten_declaration_list(DeclarationContext::IO{is_input:false}, false, cursor)
            }
        })
    }

    fn flatten_global(&mut self, cursor: &mut Cursor) {
        // Skip because we covered it in initialization. 
        let _ = cursor.optional_field(field!("extern_marker"));
        // Skip because we know this from initialization. 
        cursor.field(field!("object_type"));
        
        let name_span = cursor.field_span(field!("name"), kind!("identifier"));
        self.flatten_template_inputs(cursor);
        let module_name = &self.name_resolver.file_text[name_span];
        println!("TREE SITTER module! {module_name}");
        // Interface is allocated in self
        if cursor.optional_field(field!("interface_ports")) {
            self.flatten_interface_ports(cursor);
        }

        cursor.field(field!("block"));
        self.flatten_code(cursor);
    }
}

/// Flattens all modules in the project.
///
/// Requires that first, all modules have been initialized.
pub fn flatten_all_modules(linker: &mut Linker) {
    let linker_files : *const ArenaAllocator<FileData, FileUUIDMarker> = &linker.files;
    // SAFETY we won't be touching the files anywere. This is just to get the compiler to stop complaining about linker going into the closure. 
    for (_file_id, file) in unsafe{&*linker_files} {
        let mut span_debugger = SpanDebugger::new("flatten_all_modules", file);
        let mut associated_value_iter = file.associated_values.iter();

        let mut cursor = Cursor::new_at_root(&file.tree, &file.file_text);

        cursor.list(kind!("source_file"), |cursor| {
            cursor.go_down(kind!("global_object"), |cursor| {
                let file_obj = *associated_value_iter.next().expect("Iterator cannot be exhausted");
                let (obj_link_info_mut, ports_to_visit, fields_to_visit, default_declaration_context) = match file_obj {
                    NameElem::Module(module_uuid) => {
                        let md = &mut linker.modules[module_uuid];
                        (&mut md.link_info, md.ports.id_range().into_iter(), UUIDRange::empty().into_iter(), DeclarationContext::PlainWire)
                    }
                    NameElem::Type(type_uuid) => {
                        let typ = &mut linker.types[type_uuid];
                        (&mut typ.link_info, UUIDRange::empty().into_iter(), typ.fields.id_range().into_iter(), DeclarationContext::StructField)
                    }
                    NameElem::Constant(const_uuid) => {
                        todo!("TODO Constant flattening")
                    }
                };

                let errors_globals = obj_link_info_mut.take_errors_globals_for_editing(&linker.files);
                let template_inputs_to_visit = obj_link_info_mut.template_arguments.id_range().into_iter();

                let (modules, types, constants, name_resolver) = make_resolvers(linker, &file.file_text, &errors_globals);

                let mut context = FlatteningContext {
                    ports_to_visit,
                    fields_to_visit,
                    default_declaration_context,
                    template_inputs_to_visit,
                    errors: name_resolver.errors,
                    working_on_link_info: linker.get_link_info(file_obj).unwrap(),
                    instructions: FlatAlloc::new(),
                    modules,
                    types,
                    constants,
                    name_resolver,
                    local_variable_context: LocalVariableContext::new_initial(),
                };

                context.flatten_global(cursor);
    
                // Make sure all ports have been visited
                assert!(context.ports_to_visit.is_empty());
                assert!(context.template_inputs_to_visit.is_empty());

                let instructions = context.instructions;
                
                match file_obj {
                    NameElem::Module(module_uuid) => {
                        let md = &mut linker.modules[module_uuid];
                        // Set all declaration_instruction values
                        for (decl_id, instr) in &instructions {
                            if let Instruction::Declaration(decl) = instr {
                                match decl.is_port {
                                    DeclarationPortInfo::NotPort => {},
                                    DeclarationPortInfo::RegularPort { is_input:_, port_id } => {
                                        let port = &mut md.ports[port_id];
                                        assert_eq!(port.name_span, decl.name_span);
                                        port.declaration_instruction = decl_id;
                                    }
                                    DeclarationPortInfo::GenerativeInput(this_template_id) => {
                                        let TemplateInputKind::Generative(GenerativeTemplateInputKind { decl_span:_, declaration_instruction }) = 
                                            &mut md.link_info.template_arguments[this_template_id].kind else {unreachable!()};
                    
                                        *declaration_instruction = decl_id;
                                    }
                                    DeclarationPortInfo::StructField { field_id:_ } => unreachable!("No Struct fields in Modules")
                                }
                            }
                        }
                        md.instructions = instructions;
                    }
                    NameElem::Type(type_uuid) => {
                        let typ = &mut linker.types[type_uuid];
                        
                        // Set all declaration_instruction values
                        for (decl_id, instr) in &instructions {
                            if let Instruction::Declaration(decl) = instr {
                                match decl.is_port {
                                    DeclarationPortInfo::NotPort => {assert!(decl.identifier_type == IdentifierType::Generative, "If a variable isn't generative, then it MUST be a struct field")}
                                    DeclarationPortInfo::StructField { field_id } => {
                                        let field = &mut typ.fields[field_id];
                                        assert_eq!(field.name_span, decl.name_span);
                                        field.declaration_instruction = decl_id;
                                    }
                                    DeclarationPortInfo::RegularPort { is_input:_, port_id:_ } => {unreachable!("No ports in structs")}
                                    DeclarationPortInfo::GenerativeInput(this_template_id) => {
                                        let TemplateInputKind::Generative(GenerativeTemplateInputKind { decl_span:_, declaration_instruction }) = 
                                            &mut typ.link_info.template_arguments[this_template_id].kind else {unreachable!()};
                    
                                        *declaration_instruction = decl_id;
                                    }
                                }
                            }
                        }
                        typ.instructions = instructions;
                    }
                    NameElem::Constant(const_uuid) => {
                        todo!("TODO Constant flattening")
                    }
                }
            });
        });
        span_debugger.defuse();
    }
}
