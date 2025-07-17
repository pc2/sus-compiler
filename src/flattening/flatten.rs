use std::cell::OnceCell;
use std::num::NonZeroU16;

use crate::alloc::{ArenaAllocator, UUIDRange, UUID};

use crate::linker::passes::{GlobalResolver, LinkerPass};
use crate::prelude::*;

use ibig::IBig;
use sus_proc_macro::{field, kind, kw};

use crate::linker::{FileData, GlobalObj, GlobalUUID};
use crate::value::Value;

use super::name_context::LocalVariableContext;
use super::parser::Cursor;
use super::*;

use crate::typing::template::{
    GenerativeParameterKind, Parameter, TemplateKind, TypeParameterKind,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum NamedLocal {
    Declaration(FlatID),
    SubModule(FlatID),
    LocalInterface(FlatID),
    TemplateType(TemplateID),
    DomainDecl(DomainID),
}

enum LocalOrGlobal {
    Local(Span, NamedLocal),
    Module(GlobalReference<ModuleUUID>),
    Type(GlobalReference<TypeUUID>),
    Constant(GlobalReference<ConstantUUID>),
    // Error is already handled
    NotFound(Span),
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

impl core::fmt::Display for PartSelectDirection {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            PartSelectDirection::Up => "+:",
            PartSelectDirection::Down => "-:",
        })
    }
}

impl SliceType {
    pub fn from_kind_id(kind_id: u16) -> Self {
        match kind_id {
            kw!(":") => SliceType::Normal,
            kw!("+:") => SliceType::PartSelect(PartSelectDirection::Up),
            kw!("-:") => SliceType::PartSelect(PartSelectDirection::Down),
            _ => unreachable!(),
        }
    }
}
impl core::fmt::Display for SliceType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            SliceType::Normal => ":",
            SliceType::PartSelect(PartSelectDirection::Up) => "+:",
            SliceType::PartSelect(PartSelectDirection::Down) => "-:",
        })
    }
}

#[derive(Debug)]
enum ModuleOrWrittenType {
    WrittenType(WrittenType),
    Module(GlobalReference<ModuleUUID>),
}

struct FlatteningContext<'l, 'errs> {
    globals: GlobalResolver<'l, 'l>,
    errors: &'errs ErrorCollector<'l>,

    name: &'l str,
    parameters: TVec<Parameter>,
    instructions: FlatAlloc<Instruction, FlatIDMarker>,

    domains: FlatAlloc<DomainInfo, DomainIDMarker>,
    current_domain: DomainID,

    fields: FlatAlloc<StructField, FieldIDMarker>,
    ports: FlatAlloc<Port, PortIDMarker>,
    interfaces: FlatAlloc<Interface, InterfaceIDMarker>,

    local_variable_context: LocalVariableContext<'l, NamedLocal>,

    default_decl_kind: DeclarationKind,

    current_parent_condition: Option<ParentCondition>,
}

impl<'l, 'c: 'l> FlatteningContext<'l, '_> {
    fn flatten_latency_specifier(&mut self, cursor: &mut Cursor<'c>) -> Option<(FlatID, Span)> {
        cursor.optional_field(field!("latency_specifier")).then(|| {
            cursor.go_down_content(kind!("latency_specifier"), |cursor| {
                let expr = self.flatten_subexpr(cursor);
                let span = cursor.span();
                (expr, span)
            })
        })
    }

    fn flatten_template_args(&mut self, cursor: &mut Cursor<'c>) -> Vec<WrittenTemplateArg> {
        cursor.collect_list(kind!("template_args"), |cursor| {
            cursor.go_down(kind!("template_arg"), |cursor| {
                let (name_span, name) =
                    cursor.field_span(field!("name"), kind!("identifier"));

                let (kind, value_span) = if cursor.optional_field(field!("val_arg")) {
                    let value_span = cursor.span();
                    let expr = self.flatten_subexpr(cursor);
                    (Some(TemplateKind::Value(expr)), value_span)
                } else if cursor.optional_field(field!("type_arg")) {
                    let value_span = cursor.span();
                    let typ = self.flatten_type(cursor);
                    (Some(TemplateKind::Type(typ)), value_span)
                } else {
                    (match self.local_variable_context.get_declaration_for(name) {
                        Some(NamedLocal::TemplateType(t)) => Some(TemplateKind::Type(WrittenType::TemplateVariable(name_span, t))),
                        Some(NamedLocal::Declaration(decl_id)) => {
                            let wire_read_id = self.instructions.alloc(Instruction::Expression(Expression {
                                parent_condition: self.current_parent_condition,
                                output: ExpressionOutput::SubExpression(TyCell::new()),
                                span: name_span,
                                domain: Cell::new(DomainType::PLACEHOLDER),
                                source: ExpressionSource::WireRef(WireReference {
                                    root: WireReferenceRoot::LocalDecl(decl_id),
                                    root_span: name_span,
                                    output_typ: TyCell::new(),
                                    path: Vec::new(),
                                })
                            }));
                            Some(TemplateKind::Value(wire_read_id))
                        }
                        Some(NamedLocal::SubModule(sm)) => {
                            self.errors.error(name_span, format!("{name} does not name a Type or a Value. Local submodules are not allowed!"))
                                .info_obj_same_file(self.instructions[sm].unwrap_submodule());
                            None
                        }
                        Some(NamedLocal::DomainDecl(dom)) => {
                            self.errors.error(name_span, format!("{name} does not name a Type or a Value. Domains are not allowed!"))
                                .info_obj_same_file(&self.domains[dom]);
                            None
                        }
                        Some(NamedLocal::LocalInterface(interf)) => {
                            self.errors.error(name_span, format!("{name} does not name a Type or a Value. Local Interfaces are not allowed!"))
                                .info_obj_same_file(self.instructions[interf].unwrap_interface());
                            None
                        }
                        None => {
                            self.errors.error(name_span, format!("{name} does not name a Type or a Value."));
                            None
                        },
                    }, name_span)
                };

                WrittenTemplateArg{ name: name.to_owned(), name_span, value_span, kind, refers_to: OnceCell::new() }
            })
        })
    }

    fn flatten_local_or_template_global(&mut self, cursor: &mut Cursor<'c>) -> LocalOrGlobal {
        cursor.go_down(kind!("template_global"), |cursor| {
            let mut must_be_global = cursor.optional_field(field!("is_global_path"));

            cursor.field(field!("namespace_list"));
            let name_path = cursor.collect_list(kind!("namespace_list"), |cursor| {
                let (kind, span) = cursor.kind_span();
                assert!(kind == kind!("identifier"));
                span
            });

            if name_path.len() > 1 {
                must_be_global = true;
            }

            let (template_args, template_span) = if cursor.optional_field(field!("template_args")) {
                must_be_global = true;
                let bracket_span = BracketSpan::from_outer(cursor.span());

                let args = self.flatten_template_args(cursor);

                (args, Some(bracket_span))
            } else {
                (Vec::new(), None)
            };

            // Possibly local
            if !must_be_global {
                let [local_name] = name_path.as_slice() else {
                    unreachable!()
                };
                let name_text = &cursor.file_data.file_text[*local_name];
                if let Some(decl_id) = self.local_variable_context.get_declaration_for(name_text) {
                    return LocalOrGlobal::Local(*local_name, decl_id);
                }
            }

            // Global identifier
            let [name_span] = *name_path.as_slice() else {
                self.errors.todo(name_path[1], "Namespaces");
                return LocalOrGlobal::NotFound(name_path[0]);
            };
            if let Some(global_id) = self.globals.resolve_global(
                name_span,
                &cursor.file_data.file_text[name_span],
                self.errors,
            ) {
                match global_id {
                    GlobalUUID::Module(id) => LocalOrGlobal::Module(GlobalReference {
                        id,
                        name_span,
                        template_args,
                        template_arg_types: TyCell::new(),
                        template_span,
                    }),
                    GlobalUUID::Type(id) => LocalOrGlobal::Type(GlobalReference {
                        id,
                        name_span,
                        template_args,
                        template_arg_types: TyCell::new(),
                        template_span,
                    }),
                    GlobalUUID::Constant(id) => LocalOrGlobal::Constant(GlobalReference {
                        id,
                        name_span,
                        template_args,
                        template_arg_types: TyCell::new(),
                        template_span,
                    }),
                }
            } else {
                LocalOrGlobal::NotFound(name_span)
            }
        })
    }

    fn flatten_array_type(&mut self, span: Span, cursor: &mut Cursor<'c>) -> WrittenType {
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

    fn flatten_type(&mut self, cursor: &mut Cursor<'c>) -> WrittenType {
        let ModuleOrWrittenType::WrittenType(wr_typ) = self.flatten_module_or_type::<false>(cursor)
        else {
            unreachable!("Can't not be type")
        };
        wr_typ
    }

    fn flatten_module_or_type<const ALLOW_MODULES: bool>(
        &mut self,
        cursor: &mut Cursor<'c>,
    ) -> ModuleOrWrittenType {
        let accepted_text = if ALLOW_MODULES {
            "Type or Module"
        } else {
            "Type"
        };
        let (kind, span) = cursor.kind_span();
        // Only difference is that
        match kind {
            kind!("template_global") => {
                match self.flatten_local_or_template_global(cursor) {
                    LocalOrGlobal::Local(span, NamedLocal::Declaration(instr))
                    | LocalOrGlobal::Local(span, NamedLocal::SubModule(instr))
                    | LocalOrGlobal::Local(span, NamedLocal::LocalInterface(instr)) => {
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
                    LocalOrGlobal::Local(span, NamedLocal::DomainDecl(domain_id)) => {
                        self.errors
                            .error(
                                span,
                                format!("This is not a {accepted_text}, it is a domain instead!"),
                            )
                            .info_obj_same_file(&self.domains[domain_id]);

                        ModuleOrWrittenType::WrittenType(WrittenType::Error(span))
                    }
                    LocalOrGlobal::Local(span, NamedLocal::TemplateType(template_id)) => {
                        ModuleOrWrittenType::WrittenType(WrittenType::TemplateVariable(
                            span,
                            template_id,
                        ))
                    }
                    LocalOrGlobal::Type(type_ref) => {
                        ModuleOrWrittenType::WrittenType(WrittenType::Named(type_ref))
                    }
                    LocalOrGlobal::Module(module_ref) if ALLOW_MODULES => {
                        ModuleOrWrittenType::Module(module_ref)
                    }
                    LocalOrGlobal::Module(module_ref) => {
                        self.globals.not_expected_global_error(
                            &module_ref,
                            accepted_text,
                            self.errors,
                        );
                        ModuleOrWrittenType::WrittenType(WrittenType::Error(module_ref.name_span))
                    }
                    LocalOrGlobal::Constant(constant_ref) => {
                        self.globals.not_expected_global_error(
                            &constant_ref,
                            accepted_text,
                            self.errors,
                        );
                        ModuleOrWrittenType::WrittenType(WrittenType::Error(constant_ref.name_span))
                    }
                    LocalOrGlobal::NotFound(name_span) => {
                        ModuleOrWrittenType::WrittenType(WrittenType::Error(name_span))
                    } // Already covered
                }
            }
            kind!("array_type") => {
                ModuleOrWrittenType::WrittenType(self.flatten_array_type(span, cursor))
            }
            _other => cursor.could_not_match(),
        }
    }

    fn alloc_local_name(&mut self, name_span: Span, name: &'c str, named_local: NamedLocal) {
        if let Err(conflict) = self
            .local_variable_context
            .add_declaration(name, named_local)
        {
            let err_ref = self.errors.error(
                name_span,
                "This declaration conflicts with a previous declaration in the same scope",
            );

            match conflict {
                NamedLocal::Declaration(decl_id) => {
                    err_ref.info_obj_same_file(self.instructions[decl_id].unwrap_declaration());
                }
                NamedLocal::SubModule(submod_id) => {
                    err_ref.info_obj_same_file(self.instructions[submod_id].unwrap_submodule());
                }
                NamedLocal::LocalInterface(interf_id) => {
                    err_ref.info_obj_same_file(self.instructions[interf_id].unwrap_interface());
                }
                NamedLocal::TemplateType(template_id) => {
                    err_ref.info_obj_same_file(&self.parameters[template_id]);
                }
                NamedLocal::DomainDecl(domain_id) => {
                    err_ref.info_obj_same_file(&self.domains[domain_id]);
                }
            }
        }
    }

    fn forbid_keyword(&self, kw_span: Option<Span>, context: &str) {
        if let Some(kw_span) = kw_span {
            self.errors
                .error(kw_span, format!("This can't be used {context}"));
        }
    }

    fn flatten_declaration<const ALLOW_MODULES: bool>(
        &mut self,
        decl_context: DeclarationKind,
        declaration_itself_is_not_written_to: bool,
        cursor: &mut Cursor<'c>,
    ) -> FlatID {
        let decl_span = cursor.span();
        cursor.go_down(kind!("declaration"), |cursor| {
            // Extra inputs and outputs declared in the body of the module

            let mut input_kw = None;
            let mut output_kw = None;
            let mut gen_kw = None;
            let mut state_kw = None;

            if cursor.optional_field(field!("declaration_modifiers")) {
                cursor.list(kind!("declaration_modifiers"), |cursor| {
                    let (kind, span) = cursor.kind_span();
                    let selected_kw = match kind {
                        kw!("input") => &mut input_kw,
                        kw!("output") => &mut output_kw,
                        kw!("gen") => &mut gen_kw,
                        kw!("state") => &mut state_kw,
                        _ => cursor.could_not_match(),
                    };
                    if let Some(prev_span) = *selected_kw {
                        self.errors
                            .error(span, "Duplicate keyword!")
                            .info_same_file(prev_span, "Previously used here");
                    }
                    *selected_kw = Some(span);
                })
            };

            cursor.field(field!("type"));
            let typ_or_module_expr = self.flatten_module_or_type::<ALLOW_MODULES>(cursor);

            let (name_span, name) = cursor.field_span(field!("name"), kind!("identifier"));

            let span_latency_specifier = self.flatten_latency_specifier(cursor);
            let latency_specifier = span_latency_specifier.map(|(ls, _)| ls);
            // Parsing components done

            let documentation = cursor.extract_gathered_comments();

            let declaration_instruction = self.instructions.get_next_alloc_id();

            let typ_expr = match typ_or_module_expr {
                ModuleOrWrittenType::WrittenType(typ) => typ,
                ModuleOrWrittenType::Module(module_ref) => {
                    assert!(ALLOW_MODULES);
                    if let Some((_, span)) = span_latency_specifier {
                        self.errors
                            .error(span, "Cannot add latency specifier to module instances");
                    }

                    let new_submod = SubModuleInstance {
                        parent_condition: self.current_parent_condition,
                        name: name.to_owned(),
                        name_span,
                        module_ref,
                        local_domain_map: TyCell::new(),
                        typ: TyCell::new(),
                        documentation,
                    };
                    let submod_id = self.instructions.alloc(Instruction::SubModule(new_submod));

                    self.alloc_local_name(name_span, name, NamedLocal::SubModule(submod_id));

                    return submod_id;
                }
            };

            let decl_kind = match decl_context {
                DeclarationKind::RegularWire { .. } => {
                    if gen_kw.is_some() {
                        self.forbid_keyword(input_kw, "on a generative declaration.");
                        self.forbid_keyword(output_kw, "on a generative declaration.");
                        self.forbid_keyword(state_kw, "on a generative declaration.");
                        DeclarationKind::RegularGenerative { read_only: false }
                    } else if input_kw.is_some() | output_kw.is_some() {
                        let (direction, is_state) = if input_kw.is_some() {
                            self.forbid_keyword(
                                output_kw,
                                "on a port which has already been declared an input",
                            );
                            self.forbid_keyword(
                                state_kw,
                                "on an input port, because it is read-only",
                            );
                            (Direction::Input, false)
                        } else {
                            (Direction::Output, state_kw.is_some())
                        };
                        let port_id = self.ports.alloc(Port {
                            name: name.to_owned(),
                            name_span,
                            decl_span,
                            direction,
                            domain: self.current_domain,
                            declaration_instruction,
                            latency_specifier,
                        });
                        let parent_interface = self.interfaces.alloc(Interface {
                            name_span,
                            name: name.to_owned(),
                            domain: Some(self.current_domain),
                            declaration_instruction: Some(InterfaceDeclKind::SinglePort(
                                declaration_instruction,
                            )),
                        });
                        DeclarationKind::Port {
                            direction,
                            is_state,
                            port_id,
                            parent_interface,
                            domain: self.current_domain,
                        }
                    } else {
                        let is_state = state_kw.is_some();
                        DeclarationKind::RegularWire {
                            is_state,
                            read_only: false,
                        }
                    }
                }
                DeclarationKind::StructField(_uuid) => {
                    self.forbid_keyword(input_kw, "in struct fields");
                    self.forbid_keyword(output_kw, "in struct fields");
                    self.forbid_keyword(state_kw, "in struct fields");
                    if gen_kw.is_some() {
                        DeclarationKind::RegularGenerative { read_only: false }
                    } else {
                        DeclarationKind::StructField(self.fields.alloc(StructField {
                            name: name.to_owned(),
                            name_span,
                            decl_span,
                            declaration_instruction,
                        }))
                    }
                }
                DeclarationKind::ConditionalBinding {
                    when_id, direction, ..
                } => {
                    self.forbid_keyword(input_kw, "on a conditional binding");
                    self.forbid_keyword(output_kw, "on a conditional binding");
                    self.forbid_keyword(gen_kw, "on a conditional binding");
                    let is_state = match direction {
                        Direction::Input => {
                            self.forbid_keyword(
                                state_kw,
                                "on input conditional bindings, because they are read-only",
                            );
                            false
                        }
                        Direction::Output => state_kw.is_some(),
                    };
                    DeclarationKind::ConditionalBinding {
                        direction,
                        is_state,
                        when_id,
                    }
                }
                DeclarationKind::Port {
                    direction,
                    domain,
                    parent_interface,
                    ..
                } => {
                    let port_ctx = match direction {
                        Direction::Input => {
                            "here, it's already implicitly declared as an input port"
                        }
                        Direction::Output => {
                            "here, it's already implicitly declared as an output port"
                        }
                    };
                    self.forbid_keyword(input_kw, port_ctx);
                    self.forbid_keyword(output_kw, port_ctx);
                    self.forbid_keyword(gen_kw, "on ports");
                    let is_state = match direction {
                        Direction::Input => {
                            self.forbid_keyword(
                                state_kw,
                                "on input ports, because they are read-only",
                            );
                            false
                        }
                        Direction::Output => state_kw.is_some(),
                    };
                    let port_id = self.ports.alloc(Port {
                        name: name.to_owned(),
                        name_span,
                        decl_span,
                        direction,
                        domain,
                        declaration_instruction,
                        latency_specifier,
                    });
                    DeclarationKind::Port {
                        direction,
                        is_state,
                        port_id,
                        parent_interface,
                        domain,
                    }
                }
                d @ DeclarationKind::RegularGenerative { .. } => {
                    self.forbid_keyword(input_kw, "in a generative context");
                    self.forbid_keyword(output_kw, "in a generative context");
                    self.forbid_keyword(
                        gen_kw,
                        "in a generative context, it is already generative!",
                    );
                    self.forbid_keyword(state_kw, "in a generative context");
                    d
                }
                d @ DeclarationKind::TemplateParameter(_) => {
                    self.forbid_keyword(input_kw, "in template parameters");
                    self.forbid_keyword(output_kw, "in template parameters");
                    self.forbid_keyword(
                        gen_kw,
                        "in a template parameter, it is already generative!",
                    );
                    self.forbid_keyword(state_kw, "in template parameters");
                    d
                }
            };

            assert_eq!(
                declaration_instruction,
                self.instructions
                    .alloc(Instruction::Declaration(Declaration {
                        parent_condition: self.current_parent_condition,
                        typ_expr,
                        typ: TyCell::new(),
                        domain: Cell::new(DomainType::PLACEHOLDER),
                        declaration_itself_is_not_written_to,
                        decl_kind,
                        name: name.to_owned(),
                        name_span,
                        decl_span,
                        latency_specifier,
                        documentation,
                    }))
            );

            self.alloc_local_name(
                name_span,
                name,
                NamedLocal::Declaration(declaration_instruction),
            );

            declaration_instruction
        })
    }

    // function to flatten a straightforward xxx[size] array type expression (no slicing)
    fn flatten_array_bracket(&mut self, cursor: &mut Cursor<'c>) -> (FlatID, BracketSpan) {
        let bracket_span = BracketSpan::from_outer(cursor.span());
        cursor.go_down_content(kind!("array_type_bracket"), |cursor| {
            let expr = self.flatten_subexpr(cursor);
            (expr, bracket_span)
        })
    }

    fn flatten_array_access_bracket(
        &mut self,
        cursor: &mut Cursor<'c>,
    ) -> (WireReferencePathElement, BracketSpan) {
        let bracket_span = BracketSpan::from_outer(cursor.span());
        let path_elem = cursor.go_down(kind!("array_access_bracket_expression"), |cursor| {
            if cursor.optional_field(field!("index")) {
                let expr = self.flatten_subexpr(cursor);

                WireReferencePathElement::ArrayAccess {
                    idx: expr,
                    bracket_span,
                    input_typ: TyCell::new(),
                }
            } else {
                cursor.field(field!("slice"));
                cursor.go_down(kind!("slice"), |cursor| {
                    let from = if cursor.optional_field(field!("index_a")) {
                        let idx_a = self.flatten_subexpr(cursor);
                        Some(idx_a)
                    } else {
                        None
                    };
                    cursor.field(field!("type"));
                    let (slice_op_kind, slice_op_span) = cursor.kind_span();
                    let slice_kind = SliceType::from_kind_id(slice_op_kind);

                    let to = if cursor.optional_field(field!("index_b")) {
                        let idx_b = self.flatten_subexpr(cursor);
                        Some(idx_b)
                    } else {
                        None
                    };

                    match slice_kind {
                        SliceType::PartSelect(direction) => {
                            let from = from.unwrap_or_else(|| {
                                self.errors.error(
                                    bracket_span.inner_span().empty_span_at_front(),
                                    "Missing indexed part-select slices start index",
                                );

                                self.new_error_subexpr(slice_op_span)
                            });
                            let width = to.unwrap_or_else(|| {
                                self.errors.error(
                                    bracket_span.inner_span().empty_span_at_front(),
                                    "Missing indexed part-select slices width",
                                );

                                self.new_error_subexpr(slice_op_span)
                            });
                            WireReferencePathElement::ArrayPartSelect {
                                from,
                                width,
                                bracket_span,
                                input_typ: TyCell::new(),
                                direction,
                            }
                        }
                        SliceType::Normal => WireReferencePathElement::ArraySlice {
                            from,
                            to,
                            bracket_span,
                            input_typ: TyCell::new(),
                        },
                    }
                })
            }
        });
        (path_elem, bracket_span)
    }

    fn flatten_subexpr(&mut self, cursor: &mut Cursor<'c>) -> FlatID {
        let (source, span) = self.flatten_expr_source(cursor);
        let wire_instance = Expression {
            parent_condition: self.current_parent_condition,
            domain: Cell::new(DomainType::PLACEHOLDER),
            span,
            source,
            output: ExpressionOutput::SubExpression(TyCell::new()),
        };

        self.instructions
            .alloc(Instruction::Expression(wire_instance))
    }

    fn flatten_assign_to_expr(&mut self, writes: Vec<WriteTo>, cursor: &mut Cursor<'c>) {
        let (source, span) = self.flatten_expr_source(cursor);

        let output = match (&source, writes.is_empty()) {
            (ExpressionSource::FuncCall(_), _) | (_, false) => ExpressionOutput::MultiWrite(writes),
            (_, true) => {
                self.errors.warn(span, "The result of this expression is not used. Only function calls can return nothing. ");
                ExpressionOutput::SubExpression(TyCell::new())
            }
        };

        let wire_instance = Expression {
            parent_condition: self.current_parent_condition,
            span,
            domain: Cell::new(DomainType::PLACEHOLDER),
            source,
            output,
        };
        self.instructions
            .alloc(Instruction::Expression(wire_instance));
    }

    fn new_error(&mut self, root_span: Span) -> WireReference {
        WireReference {
            root: WireReferenceRoot::Error,
            path: Vec::new(),
            root_span,
            output_typ: TyCell::new(),
        }
    }

    fn new_error_subexpr(&mut self, root_span: Span) -> FlatID {
        let wire_ref = self.new_error(root_span);

        self.instructions.alloc(Instruction::Expression(Expression {
            span: root_span,
            parent_condition: self.current_parent_condition,
            source: ExpressionSource::WireRef(wire_ref),
            domain: Cell::new(DomainType::Generative),
            output: ExpressionOutput::SubExpression(TyCell::new()),
        }))
    }

    fn flatten_expr_source(&mut self, cursor: &mut Cursor<'c>) -> (ExpressionSource, Span) {
        let (kind, expr_span) = cursor.kind_span();

        let source = match kind {
            kind!("number") => {
                let text = &cursor.file_data.file_text[expr_span];
                use std::str::FromStr;
                ExpressionSource::Literal(Value::Integer(IBig::from_str(text).unwrap()))
            }
            kind!("unary_op") => cursor.go_down_no_check(|cursor| {
                cursor.field(field!("operator"));
                let op = UnaryOperator::from_kind_id(cursor.kind());

                cursor.field(field!("right"));
                let right = self.flatten_subexpr(cursor);

                ExpressionSource::UnaryOp {
                    op,
                    rank: TyCell::new(),
                    right,
                }
            }),
            kind!("binary_op") => cursor.go_down_no_check(|cursor| {
                cursor.field(field!("left"));
                let left = self.flatten_subexpr(cursor);

                cursor.field(field!("operator"));
                let op = BinaryOperator::from_kind_id(cursor.kind());

                cursor.field(field!("right"));
                let right = self.flatten_subexpr(cursor);

                ExpressionSource::BinaryOp {
                    op,
                    rank: TyCell::new(),
                    left,
                    right,
                }
            }),
            kind!("func_call") => cursor.go_down_no_check(|cursor| {
                cursor.field(field!("name"));
                let func_wire_ref = self.flatten_subexpr(cursor);

                cursor.field(field!("arguments"));
                let arguments_span = BracketSpan::from_outer(cursor.span());

                let arguments = cursor
                    .collect_list(kind!("parenthesis_expression_list"), |cursor| {
                        self.flatten_subexpr(cursor)
                    });

                ExpressionSource::FuncCall(FuncCall {
                    func_wire_ref,
                    arguments,
                    arguments_span,
                })
            }),
            kind!("parenthesis_expression") => {
                // Explicitly return so we don't alloc another WireInstance Instruction
                return cursor.go_down_content(kind!("parenthesis_expression"), |cursor| {
                    self.flatten_expr_source(cursor)
                });
            }
            kind!("array_list_expression") => {
                let list = cursor.collect_list(kind!("array_list_expression"), |cursor| {
                    self.flatten_subexpr(cursor)
                });
                ExpressionSource::ArrayConstruct(list)
            }
            _other => ExpressionSource::WireRef(self.flatten_wire_reference(cursor)),
        };
        (source, expr_span)
    }

    fn flatten_wire_reference(&mut self, cursor: &mut Cursor<'c>) -> WireReference {
        let (kind, expr_span) = cursor.kind_span();
        match kind {
            kind!("template_global") => {
                match self.flatten_local_or_template_global(cursor) {
                    LocalOrGlobal::Local(span, named_obj) => match named_obj {
                        NamedLocal::Declaration(instr) => {
                            let root = WireReferenceRoot::LocalDecl(instr);
                            WireReference {
                                root,
                                output_typ: TyCell::new(),
                                root_span: expr_span,
                                path: Vec::new(),
                            }
                        }
                        NamedLocal::SubModule(instr) => {
                            let root = WireReferenceRoot::LocalSubmodule(instr);
                            WireReference {
                                root,
                                output_typ: TyCell::new(),
                                root_span: expr_span,
                                path: Vec::new(),
                            }
                        }
                        NamedLocal::LocalInterface(instr) => {
                            let root = WireReferenceRoot::LocalInterface(instr);
                            WireReference {
                                root,
                                output_typ: TyCell::new(),
                                root_span: expr_span,
                                path: Vec::new(),
                            }
                        }
                        NamedLocal::TemplateType(template_id) => {
                            self.errors
                                .error(
                                    span,
                                    format!(
                                        "Expected a value, but instead found template type '{}'",
                                        self.parameters[template_id].name
                                    ),
                                )
                                .info_obj_same_file(&self.parameters[template_id]);
                            self.new_error(expr_span)
                        }
                        NamedLocal::DomainDecl(domain_id) => {
                            let domain = &self.domains[domain_id];
                            self.errors
                                .error(
                                    span,
                                    format!(
                                        "Expected a value, but instead found domain '{}'",
                                        domain.name
                                    ),
                                )
                                .info_same_file(
                                    span,
                                    format!("Domain {} declared here", domain.name),
                                );
                            self.new_error(expr_span)
                        }
                    },
                    LocalOrGlobal::Constant(cst_ref) => {
                        let root = WireReferenceRoot::NamedConstant(cst_ref);
                        WireReference {
                            root,
                            output_typ: TyCell::new(),
                            root_span: expr_span,
                            path: Vec::new(),
                        }
                    }
                    LocalOrGlobal::Module(md_ref) => {
                        let root = WireReferenceRoot::NamedModule(md_ref);
                        WireReference {
                            root,
                            output_typ: TyCell::new(),
                            root_span: expr_span,
                            path: Vec::new(),
                        }
                    }
                    LocalOrGlobal::Type(type_ref) => {
                        self.globals.not_expected_global_error(
                            &type_ref,
                            "named wire: local or constant",
                            self.errors,
                        );
                        self.new_error(expr_span)
                    }
                    LocalOrGlobal::NotFound(_) => self.new_error(expr_span), // Error handled by [flatten_local_or_template_global]
                }
            }
            kind!("array_op") => cursor.go_down_no_check(|cursor| {
                cursor.field(field!("arr"));
                let mut wire_ref = self.flatten_wire_reference(cursor);

                // only unpack the subexpr after flattening the idx, so we catch all errors
                cursor.field(field!("arr_idx"));
                let (access, _) = self.flatten_array_access_bracket(cursor);

                wire_ref.path.push(access);

                wire_ref
            }),
            kind!("field_access") => cursor.go_down_no_check(|cursor| {
                cursor.field(field!("left"));
                let mut wire_ref = self.flatten_wire_reference(cursor);

                let (name_span, name) = cursor.field_to_string(field!("name"), kind!("identifier"));

                wire_ref.path.push(WireReferencePathElement::FieldAccess {
                    name,
                    name_span,
                    refers_to: OnceCell::new(),
                    input_typ: TyCell::new(),
                });

                wire_ref
            }),
            kind!("number") => {
                self.errors
                    .error(expr_span, "A constant is not a wire reference");
                self.new_error(expr_span)
            }
            kind!("unary_op") | kind!("binary_op") => {
                self.errors.error(
                    expr_span,
                    "The result of an operator is not a wire reference",
                );
                self.new_error(expr_span)
            }
            kind!("func_call") => {
                self.errors
                    .error(expr_span, "A submodule call is not a wire reference");
                self.new_error(expr_span)
            }
            kind!("parenthesis_expression") => {
                self.errors.error(
                    expr_span,
                    "Parentheses are not allowed within a wire reference",
                );
                self.new_error(expr_span)
            }
            _other => cursor.could_not_match(),
        }
    }

    fn flatten_then_else_blocks(
        &mut self,
        cursor: &mut Cursor<'c>,
        parent_when: Option<FlatID>,
    ) -> (FlatIDRange, FlatIDRange, Option<Span>, Option<Span>) {
        let prev_parent_condition = self.current_parent_condition;
        if let Some(parent_when) = parent_when {
            self.current_parent_condition = Some(ParentCondition {
                parent_when,
                is_else_branch: false,
            });
        }

        let start_at = self.instructions.get_next_alloc_id();
        if !cursor.optional_field(field!("then_block")) {
            let empty = UUIDRange(start_at, start_at);
            return (empty, empty, None, None);
        }
        let then_block_span = cursor.span();
        let then_block = self.flatten_code(cursor);

        if let Some(parent_when) = parent_when {
            self.current_parent_condition = Some(ParentCondition {
                parent_when,
                is_else_branch: true,
            });
        }

        let else_start = self.instructions.get_next_alloc_id();
        let else_span = if cursor.optional_field(field!("else_block")) {
            cursor.go_down(kind!("else_block"), |cursor| {
                cursor.field(field!("content"));
                if cursor.kind() == kind!("if_statement") {
                    self.flatten_if_statement(cursor); // Chained if statements
                } else {
                    self.flatten_code(cursor);
                }
            });
            Some(cursor.span())
        } else {
            None
        };
        let else_end = self.instructions.get_next_alloc_id();
        let else_block = FlatIDRange::new(else_start, else_end);

        self.current_parent_condition = prev_parent_condition;

        (then_block, else_block, Some(then_block_span), else_span)
    }

    fn flatten_if_statement(&mut self, cursor: &mut Cursor<'c>) {
        cursor.go_down(kind!("if_statement"), |cursor| {
            cursor.field(field!("statement_type"));
            let (if_typ, if_keyword_span) = cursor.kind_span();
            let expects_generative = match if_typ {
                kw!("if") => true,
                kw!("when") => false,
                _ => unreachable!(),
            };
            cursor.field(field!("condition"));

            let condition = self.flatten_subexpr(cursor);

            let if_id = self
                .instructions
                .alloc(Instruction::IfStatement(IfStatement {
                    if_keyword_span,
                    parent_condition: self.current_parent_condition,
                    condition,
                    is_generative: expects_generative,
                    then_block: FlatIDRange::PLACEHOLDER,
                    else_block: FlatIDRange::PLACEHOLDER,
                    bindings_read_only: Vec::new(),
                    bindings_writable: Vec::new(),
                    conditional_bindings_span: None,
                }));

            let bindings_start_at = self.instructions.get_next_alloc_id();

            let ((bindings_inputs, bindings_outputs), conditional_binding_span) =
                if cursor.optional_field(field!("conditional_bindings")) {
                    let conditional_bindings_span = cursor.span();
                    cursor.go_down(kind!("interface_ports"), |cursor| {
                        (
                            self.flatten_conditional_bindings(if_id, cursor),
                            Some(conditional_bindings_span),
                        )
                    })
                } else {
                    ((Vec::new(), Vec::new()), None)
                };

            let (then_block, else_block, then_block_span, _else_span) =
                self.flatten_then_else_blocks(cursor, (!expects_generative).then_some(if_id));

            let then_block = UUIDRange(bindings_start_at, then_block.1);

            assert!(then_block_span.is_some());

            let_unwrap!(
                Instruction::IfStatement(if_stmt),
                &mut self.instructions[if_id]
            );
            if_stmt.then_block = then_block;
            if_stmt.else_block = else_block;
            if_stmt.bindings_read_only = bindings_inputs;
            if_stmt.bindings_writable = bindings_outputs;
            if_stmt.conditional_bindings_span = conditional_binding_span;
        })
    }

    fn flatten_code(&mut self, cursor: &mut Cursor<'c>) -> FlatIDRange {
        let old_frame = self.local_variable_context.new_frame();

        let instr_range = self.flatten_code_keep_context(cursor);

        self.local_variable_context.pop_frame(old_frame);

        instr_range
    }
    /// Returns the range in the [Self::instructions] buffer corresponding to the flattened instructions
    fn flatten_code_keep_context(&mut self, cursor: &mut Cursor<'c>) -> FlatIDRange {
        let start_of_code = self.instructions.get_next_alloc_id();

        cursor.clear_gathered_comments(); // Clear comments at the start of a block
        cursor.list(kind!("block"), |cursor| {
            match cursor.kind() {
                kind!("assign_left_side") => {
                    self.flatten_standalone_decls(cursor);
                }
                kind!("decl_assign_statement") => {
                    cursor.go_down_no_check(|cursor| {
                        cursor.field(field!("assign_left"));
                        let write_outputs = self.flatten_assignment_left_side(cursor);

                        cursor.field(field!("assign_value"));
                        self.flatten_assign_to_expr(write_outputs, cursor);
                    });
                }
                kind!("block") => {
                    self.flatten_code(cursor);
                }
                kind!("if_statement") => {
                    self.flatten_if_statement(cursor);
                }
                kind!("for_statement") => {
                    cursor.go_down_no_check(|cursor| {
                        let loop_var_decl_frame = self.local_variable_context.new_frame();
                        cursor.field(field!("for_decl"));
                        let loop_var_decl = self.flatten_declaration::<false>(
                            DeclarationKind::RegularGenerative { read_only: true },
                            true,
                            cursor,
                        );

                        cursor.field(field!("from"));
                        let start = self.flatten_subexpr(cursor);

                        cursor.field(field!("to"));
                        let end = self.flatten_subexpr(cursor);

                        let for_id =
                            self.instructions
                                .alloc(Instruction::ForStatement(ForStatement {
                                    parent_condition: self.current_parent_condition,
                                    loop_var_decl,
                                    start,
                                    end,
                                    loop_body: FlatIDRange::PLACEHOLDER,
                                }));

                        cursor.field(field!("block"));
                        // We already started a new local_variable_context to include the loop var
                        let loop_body = self.flatten_code_keep_context(cursor);

                        let Instruction::ForStatement(for_stmt) = &mut self.instructions[for_id]
                        else {
                            unreachable!()
                        };

                        for_stmt.loop_body = loop_body;

                        self.local_variable_context.pop_frame(loop_var_decl_frame);
                    })
                }
                /*kind!("interface_statement") => {
                    cursor.go_down_no_check(|cursor| {
                        // Skip name
                        let (name_span, name) =
                            cursor.field_to_string(field!("name"), kind!("identifier"));

                        let (inputs, outputs) = self.flatten_interface_ports(true, cursor);

                        self.alloc_interface(
                            name.clone(),
                            name_span,
                            InterfaceKind::RegularInterface,
                            inputs,
                            outputs,
                        );
                    });
                }*/
                kind!("interface_statement") => cursor.go_down_no_check(|cursor| {
                    self.parse_interface(cursor);
                }),
                kind!("domain_statement") => cursor.go_down_no_check(|cursor| {
                    self.parse_domain(cursor);
                }),
                _other => cursor.could_not_match(),
            }
            cursor.clear_gathered_comments(); // Clear comments after every statement, so comments don't bleed over
        });

        let end_of_code = self.instructions.get_next_alloc_id();
        FlatIDRange::new(start_of_code, end_of_code)
    }

    fn parse_interface(&mut self, cursor: &mut Cursor<'c>) {
        // Skip interface kind
        let is_local = cursor.optional_field(field!("local"));
        cursor.field(field!("interface_kind"));
        let (interface_kw, interface_kw_span) = cursor.kind_span();
        let (left_direction, interface_kind) = match interface_kw {
            kw!("interface") => (Direction::Input, InterfaceKind::RegularInterface),
            kw!("action") => (Direction::Input, InterfaceKind::Action(UUID::PLACEHOLDER)),
            kw!("trigger") => (Direction::Output, InterfaceKind::Trigger(UUID::PLACEHOLDER)),
            _ => unreachable!(),
        };

        let (name_span, name) = cursor.field_span(field!("name"), kind!("identifier"));
        let parsed_latency_specifier = self.flatten_latency_specifier(cursor);
        let latency_specifier = parsed_latency_specifier.map(|(l, _)| l);

        let interface_decl_span = if let Some((_, span)) = parsed_latency_specifier {
            Span::new_overarching(interface_kw_span, span)
        } else {
            Span::new_overarching(interface_kw_span, name_span)
        };

        let documentation = cursor.extract_gathered_comments();

        let decl_instr = self
            .instructions
            .alloc(Instruction::Interface(InterfaceDeclaration {
                parent_condition: self.current_parent_condition,
                name: name.to_owned(),
                name_span,
                decl_span: interface_decl_span,
                interface_kw_span,
                documentation,
                interface_id: UUID::PLACEHOLDER,
                interface_kind,
                latency_specifier,
                is_local,
                inputs: Vec::new(),
                outputs: Vec::new(),
                domain: DomainType::Physical(self.current_domain),
                then_block: FlatIDRange::PLACEHOLDER,
                else_block: FlatIDRange::PLACEHOLDER,
            }));

        let then_block_starts_at = self.instructions.get_next_alloc_id();

        let new_interface = Interface {
            name_span,
            name: name.to_owned(),
            domain: Some(self.current_domain),
            declaration_instruction: Some(InterfaceDeclKind::Interface(decl_instr)),
        };
        let interface_id = if name == self.name {
            self.interfaces[InterfaceID::MAIN_INTERFACE] = new_interface;
            InterfaceID::MAIN_INTERFACE
        } else {
            let interface_id = self.interfaces.alloc(new_interface);
            self.alloc_local_name(name_span, name, NamedLocal::LocalInterface(decl_instr));
            interface_id
        };

        let variable_ctx_frame = match interface_kind {
            InterfaceKind::RegularInterface => None,
            InterfaceKind::Action(_) | InterfaceKind::Trigger(_) => {
                Some(self.local_variable_context.new_frame())
            }
        };

        let (inputs, outputs) = self.flatten_interface_ports(left_direction, interface_id, cursor);

        let (then_block, else_block, then_block_span, else_span) = self.flatten_then_else_blocks(
            cursor,
            interface_kind.is_conditional().then_some(decl_instr),
        );
        let then_block = UUIDRange(then_block_starts_at, then_block.1);
        let_unwrap!(
            Instruction::Interface(interface),
            &mut self.instructions[decl_instr]
        );

        match &mut interface.interface_kind {
            InterfaceKind::RegularInterface => {}
            InterfaceKind::Action(port_id) => {
                *port_id = self.ports.alloc(Port {
                    name: name.to_owned(),
                    name_span,
                    decl_span: interface_decl_span,
                    direction: Direction::Input,
                    domain: self.current_domain,
                    declaration_instruction: decl_instr,
                    latency_specifier,
                });
            }
            InterfaceKind::Trigger(port_id) => {
                *port_id = self.ports.alloc(Port {
                    name: name.to_owned(),
                    name_span,
                    decl_span: interface_decl_span,
                    direction: Direction::Output,
                    domain: self.current_domain,
                    declaration_instruction: decl_instr,
                    latency_specifier,
                });
            }
        }
        interface.interface_id = interface_id;
        interface.inputs = inputs;
        interface.outputs = outputs;
        interface.then_block = then_block;
        interface.else_block = else_block;

        match interface_kind {
            InterfaceKind::RegularInterface => {
                if let Some((_, lat_spec_span)) = parsed_latency_specifier {
                    self.errors.error(
                        lat_spec_span,
                        "Can only add latency specifiers to actions or triggers",
                    );
                }
                if let Some(else_span) = else_span {
                    self.errors
                        .error(else_span, "Regular interfaces cannot take else blocks");
                }
            }
            InterfaceKind::Action(_) => {
                if then_block_span.is_none() {
                    self.errors
                        .error(interface_kw_span, "An action requires a block");
                }
            }
            InterfaceKind::Trigger(_) => {}
        }

        if let Some(variable_ctx_frame) = variable_ctx_frame {
            self.local_variable_context.pop_frame(variable_ctx_frame);
        }
    }

    fn parse_domain(&mut self, cursor: &mut Cursor<'c>) {
        let (domain_name_span, domain_name) =
            cursor.field_span(field!("name"), kind!("identifier"));
        if self.domains.is_empty() {
            if let Some(existing_port) = self.ports.iter().next() {
                // Sad Path: Having ports on the implicit clk domain is not allowed.
                self.errors.error(domain_name_span, "When using explicit domains, no port is allowed to be declared on the implicit 'clk' domain.")
                        .info_same_file(existing_port.1.decl_span, "A domain should be explicitly defined before this port");
            }
        }
        let domain_id = self.domains.alloc(DomainInfo {
            name: domain_name.to_owned(),
            name_span: Some(domain_name_span),
        });

        self.alloc_local_name(
            domain_name_span,
            domain_name,
            NamedLocal::DomainDecl(domain_id),
        );
    }

    fn flatten_write_modifiers(&self, cursor: &mut Cursor<'c>) -> WriteModifiers {
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
    fn flatten_assignment_left_side(&mut self, cursor: &mut Cursor<'c>) -> Vec<WriteTo> {
        cursor.collect_list(kind!("assign_left_side"), |cursor| {
            cursor.go_down(kind!("assign_to"), |cursor| {
                let write_modifiers = self.flatten_write_modifiers(cursor);

                cursor.field(field!("expr_or_decl"));
                let (kind, to_span) = cursor.kind_span();
                let to = if kind == kind!("declaration") {
                    let root =
                        self.flatten_declaration::<false>(self.default_decl_kind, true, cursor);
                    let flat_root_decl = self.instructions[root].unwrap_declaration();
                    WireReference {
                        root: WireReferenceRoot::LocalDecl(root),
                        output_typ: TyCell::new(),
                        root_span: flat_root_decl.name_span,
                        path: Vec::new(),
                    }
                } else {
                    // It's _expression
                    self.flatten_wire_reference(cursor)
                };
                WriteTo {
                    to,
                    to_span,
                    write_modifiers,
                    target_domain: Cell::new(DomainType::PLACEHOLDER),
                }
            })
        })
    }

    /// See [Self::flatten_assignment_left_side]
    /// - Standalone declarations:
    ///     Yes modules, No write modifiers, Yes expressions (-> single expressions)
    fn flatten_standalone_decls(&mut self, cursor: &mut Cursor<'c>) {
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
                let kind = cursor.kind();

                if kind == kind!("declaration") {
                    let _ = self.flatten_declaration::<true>(self.default_decl_kind, true, cursor);
                } else { // It's _expression
                    self.flatten_assign_to_expr(Vec::new(), cursor);
                }
            });
        })
    }

    fn flatten_declaration_list(
        &mut self,
        field: NonZeroU16,
        default_decl_kind: DeclarationKind,
        cursor: &mut Cursor<'c>,
    ) -> Vec<FlatID> {
        if cursor.optional_field(field) {
            cursor.collect_list(kind!("declaration_list"), |cursor| {
                self.flatten_declaration::<false>(default_decl_kind, true, cursor)
            })
        } else {
            Vec::new()
        }
    }

    fn flatten_interface_ports(
        &mut self,
        left_direction: Direction,
        parent_interface: InterfaceID,
        cursor: &mut Cursor<'c>,
    ) -> (Vec<FlatID>, Vec<FlatID>) {
        if !cursor.optional_field(field!("interface_ports")) {
            return (Vec::new(), Vec::new());
        }
        cursor.go_down(kind!("interface_ports"), |cursor| {
            let inputs = self.flatten_declaration_list(
                field!("inputs"),
                DeclarationKind::Port {
                    direction: left_direction,
                    is_state: false,
                    parent_interface,
                    port_id: UUID::PLACEHOLDER,
                    domain: self.current_domain,
                },
                cursor,
            );

            let outputs = self.flatten_declaration_list(
                field!("outputs"),
                DeclarationKind::Port {
                    direction: left_direction.invert(),
                    is_state: false,
                    parent_interface,
                    port_id: UUID::PLACEHOLDER,
                    domain: self.current_domain,
                },
                cursor,
            );

            (inputs, outputs)
        })
    }

    fn flatten_conditional_bindings(
        &mut self,
        when_id: FlatID,
        cursor: &mut Cursor<'c>,
    ) -> (Vec<FlatID>, Vec<FlatID>) {
        let left_bindings = self.flatten_declaration_list(
            field!("inputs"),
            DeclarationKind::ConditionalBinding {
                when_id,
                direction: Direction::Input,
                is_state: false,
            },
            cursor,
        );
        let right_bindings = self.flatten_declaration_list(
            field!("outputs"),
            DeclarationKind::ConditionalBinding {
                when_id,
                direction: Direction::Output,
                is_state: false,
            },
            cursor,
        );
        (left_bindings, right_bindings)
    }

    /// Expects to begin at `field!("template_declaration_arguments")`
    fn flatten_global(
        &mut self,
        cursor: &mut Cursor<'c>,
        const_type_cursor: Option<Cursor<'c>>,
        name: &'c str,
        name_span: Span,
    ) {
        if cursor.optional_field(field!("template_declaration_arguments")) {
            cursor.list(
                kind!("template_declaration_arguments"),
                |cursor| match cursor.kind() {
                    kind!("template_declaration_type") => cursor.go_down_no_check(|cursor| {
                        let (name_span, name) =
                            cursor.field_to_string(field!("name"), kind!("identifier"));
                        let type_param_id = self.parameters.alloc(Parameter {
                            name,
                            name_span,
                            kind: TemplateKind::Type(TypeParameterKind {}),
                        });
                        self.alloc_local_name(
                            name_span,
                            &cursor.file_data.file_text[name_span],
                            NamedLocal::TemplateType(type_param_id),
                        );
                    }),
                    kind!("declaration") => {
                        let next_param_id = self.parameters.get_next_alloc_id();
                        let decl_id = self.flatten_declaration::<false>(
                            DeclarationKind::TemplateParameter(next_param_id),
                            true,
                            cursor,
                        );
                        let decl = self.instructions[decl_id].unwrap_declaration();
                        self.parameters.alloc(Parameter {
                            name: decl.name.clone(),
                            name_span: decl.name_span,
                            kind: TemplateKind::Value(GenerativeParameterKind {
                                decl_span: decl.decl_span,
                                declaration_instruction: decl_id,
                            }),
                        });
                    }
                    _other => cursor.could_not_match(),
                },
            );
        }

        if let Some(mut const_type_cursor) = const_type_cursor {
            let decl_span = const_type_cursor.span();
            const_type_cursor.go_down(kind!("const_and_type"), |const_type_cursor| {
                const_type_cursor.field(field!("const_type"));
                let typ_expr = self.flatten_type(const_type_cursor);
                let module_output_decl =
                    self.instructions
                        .alloc(Instruction::Declaration(Declaration {
                            parent_condition: self.current_parent_condition,
                            typ_expr,
                            typ: TyCell::new(),
                            domain: Cell::new(DomainType::PLACEHOLDER),
                            decl_span,
                            name_span,
                            name: name.to_owned(),
                            declaration_itself_is_not_written_to: true,
                            decl_kind: DeclarationKind::RegularGenerative { read_only: false },
                            latency_specifier: None,
                            documentation: const_type_cursor.extract_gathered_comments(),
                        }));

                self.alloc_local_name(name_span, name, NamedLocal::Declaration(module_output_decl));
            });
        }

        self.interfaces.alloc(Interface {
            name_span,
            name: name.to_owned(),
            domain: None,
            declaration_instruction: None,
        });

        cursor.field(field!("block"));
        self.flatten_code(cursor);
    }
}

/// Flattens all globals in the project.
///
/// Requires that first, all globals have been initialized.
pub fn flatten_all_globals(linker: &mut Linker) {
    let linker_files: *const ArenaAllocator<FileData, FileUUIDMarker> = &linker.files;
    // SAFETY we won't be touching the files anywere. This is just to get the compiler to stop complaining about linker going into the closure.
    for (_file_id, file) in unsafe { &*linker_files } {
        let Ok(mut cursor) = Cursor::new_at_root(file) else {
            assert!(file.associated_values.is_empty());
            continue; // Error already handled in initialization
        };

        let mut associated_value_iter = file.associated_values.iter();

        cursor.list(kind!("source_file"), |cursor| {
            cursor.go_down(kind!("global_object"), |cursor| {
                let global_obj = *associated_value_iter
                    .next()
                    .expect("Iterator cannot be exhausted");

                linker.pass("Flattening", global_obj, |pass, errors, _files| {
                    flatten_global(pass, errors, cursor);
                });
            });
        });
    }
}

fn flatten_global(pass: &mut LinkerPass, errors: &ErrorCollector, cursor: &mut Cursor) {
    let (working_on, globals) = pass.get_with_context();

    // Skip because we covered it in initialization.
    let _ = cursor.optional_field(field!("extern_marker"));
    // Skip because we know this from initialization.
    cursor.field(field!("object_type"));

    let default_decl_kind = match cursor.kind() {
        kw!("module") => DeclarationKind::RegularWire {
            is_state: false,
            read_only: false,
        },
        kind!("const_and_type") => DeclarationKind::RegularGenerative { read_only: false },
        kw!("struct") => DeclarationKind::StructField(UUID::PLACEHOLDER),
        _other => cursor.could_not_match(),
    };

    // We parse this one a bit strangely. Just because visually it looks nicer to have the template arguments after
    // const int[SIZE] range #(int SIZE) {}
    let const_type_cursor = (cursor.kind() == kind!("const_and_type")).then(|| cursor.clone());

    let (name_span, name) = cursor.field_span(field!("name"), kind!("identifier"));

    assert_eq!(working_on.get_link_info().name, name);

    let mut context = FlatteningContext {
        name,
        current_parent_condition: None,
        globals,
        fields: FlatAlloc::new(),
        ports: FlatAlloc::new(),
        interfaces: FlatAlloc::new(),
        domains: FlatAlloc::new(),
        default_decl_kind,
        errors,
        instructions: FlatAlloc::new(),
        parameters: FlatAlloc::new(),
        current_domain: UUID::from_hidden_value(0),
        local_variable_context: LocalVariableContext::new_initial(),
    };

    context.flatten_global(cursor, const_type_cursor, name, name_span);

    let instructions = context.instructions;
    let parameters = context.parameters;
    let mut domains = context.domains;
    let interfaces = context.interfaces;
    let ports = context.ports;
    let fields = context.fields;

    let mut working_on_mut = pass.get_mut();
    match &mut working_on_mut {
        GlobalObj::Module(md) => {
            if domains.is_empty() {
                domains.alloc(DomainInfo {
                    name: "clk".to_string(),
                    name_span: None,
                });
            }
            md.domains = domains;
            md.interfaces = interfaces;
            md.ports = ports;

            &mut md.link_info
        }
        GlobalObj::Type(typ) => {
            typ.fields = fields;

            &mut typ.link_info
        }
        GlobalObj::Constant(cst) => {
            cst.output_decl = instructions
                .iter()
                .find(|(_decl_id, instr)| {
                    if let Instruction::Declaration(decl) = instr {
                        decl.name_span == cst.link_info.name_span
                    } else {
                        false
                    }
                })
                .unwrap()
                .0;

            &mut cst.link_info
        }
    };

    let link_info = working_on_mut.get_link_info();
    link_info.instructions = instructions;
    link_info.template_parameters = parameters;

    // If an instruction has itself as parent when, it would create a deadlock
    for (id, instr) in &link_info.instructions {
        if let Some(parent_when) = instr.get_parent_condition() {
            assert_ne!(parent_when.parent_when, id);
        }
    }
}
