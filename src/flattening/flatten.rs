use crate::alloc::{ArenaAllocator, UUIDAllocator, UUIDRange, UUID};
use crate::typing::abstract_type::DomainType;
use crate::typing::type_inference::{AbstractTypeSubstitutor, Substitutor, TypeSubstitutor};
use crate::{alloc::UUIDRangeIter, prelude::*};

use ibig::IBig;
use sus_proc_macro::{field, kind, kw};

use crate::linker::{FileData, GlobalResolver, GlobalUUID, AFTER_FLATTEN_CP};
use crate::{debug::SpanDebugger, value::Value};

use super::name_context::LocalVariableContext;
use super::parser::Cursor;
use super::*;

use crate::typing::template::{
    AbstractTemplateArg, GenerativeParameterKind, TVec, TemplateArg, TemplateKind,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum NamedLocal {
    Declaration(FlatID),
    SubModule(FlatID),
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

#[derive(Debug)]
enum PartialWireReference {
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
    WireReference {
        wire_ref: WireReference,
        domain: DomainType,
    },
}

impl PartialWireReference {
    const ERROR: Self = PartialWireReference::WireReference {
        wire_ref: WireReference::ERROR,
        domain: DomainType::Generative,
    };
}

impl PartialWireReference {
    /// Returns the domain of the WireReferenceRoot
    fn extract(self, ctx: &FlatteningContext) -> (WireReference, DomainType) {
        match self {
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
                    .info_obj(&ctx.globals[md_uuid]);
                (WireReference::ERROR, DomainType::Generative)
            }
            PartialWireReference::GlobalModuleName(md_ref) => {
                let md = &ctx.globals[md_ref.id];
                ctx.errors
                    .error(
                        md_ref.name_span,
                        format!(
                            "Expected a Wire Reference, but found module '{}' instead",
                            md.link_info.name
                        ),
                    )
                    .info_obj(md);
                (WireReference::ERROR, DomainType::Generative)
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
                let md = &ctx.globals[md_uuid];
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
                (WireReference::ERROR, DomainType::Generative)
            }
            PartialWireReference::WireReference { wire_ref, domain } => (wire_ref, domain),
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
enum GenerativeKind {
    PlainGenerative,
    ForLoopGenerative,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum DeclarationContext {
    IO { is_input: bool },
    Generative(GenerativeKind),
    TemplateGenerative(TemplateID),
    PlainWire,
    StructField,
}

#[derive(Debug)]
enum ModuleOrWrittenType {
    WrittenType(WrittenType),
    Module(GlobalReference<ModuleUUID>),
}

/// Small wrapper struct for allocating the Hindley-Milner variables
/// required for [crate::typing::abstract_type::AbstractType::Unknown] and [DomainType::DomainVariable]
///
/// See [crate::typing::type_inference::HindleyMilner]
#[derive(Debug, Default)]
struct TypingAllocator {
    pub type_alloc: AbstractTypeSubstitutor,
    pub domain_alloc: TypeSubstitutor<DomainType>,
}

impl TypingAllocator {
    fn alloc_unset_type(&mut self, domain: DomainType) -> FullType {
        FullType {
            typ: self.type_alloc.alloc_unknown(),
            domain,
        }
    }
}

#[derive(Clone, Copy)]
#[allow(unused)]
enum InterfacePortsInfo {
    InputsThenOutputs,
    OutputsThenInputs,
    ConditionalBindings,
}

struct WrittenTemplateArg<'a> {
    name: &'a str,
    name_span: Span,
    value_span: Span,
    kind: Option<TemplateKind<WrittenType, FlatID>>,
}

struct FlatteningContext<'l, 'errs> {
    globals: &'l GlobalResolver<'l>,
    errors: &'errs ErrorCollector<'l>,

    working_on_link_info: &'l LinkInfo,
    domains: &'l FlatAlloc<DomainInfo, DomainIDMarker>,
    instructions: FlatAlloc<Instruction, FlatIDMarker>,
    type_alloc: TypingAllocator,
    named_domain_alloc: UUIDAllocator<DomainIDMarker>,
    is_implicit_clk_domain: bool,

    fields_to_visit: UUIDRangeIter<FieldIDMarker>,
    ports_to_visit: UUIDRangeIter<PortIDMarker>,

    local_variable_context: LocalVariableContext<'l, NamedLocal>,

    default_declaration_context: DeclarationContext,
}

impl<'l, 'c: 'l> FlatteningContext<'l, '_> {
    fn flatten_parameters(&mut self, cursor: &mut Cursor<'c>) {
        let mut parameters_to_visit = self
            .working_on_link_info
            .template_parameters
            .id_range()
            .into_iter();
        if cursor.optional_field(field!("template_declaration_arguments")) {
            cursor.list(kind!("template_declaration_arguments"), |cursor| {
                let claimed_type_id = parameters_to_visit.next().unwrap();
                match cursor.kind() {
                    kind!("template_declaration_type") => cursor.go_down_no_check(|cursor| {
                        // Already covered in initialization
                        cursor.field(field!("name"));

                        let selected_arg =
                            &self.working_on_link_info.template_parameters[claimed_type_id];

                        let name_span = selected_arg.name_span;

                        self.alloc_local_name(
                            name_span,
                            &cursor.file_data.file_text[name_span],
                            NamedLocal::TemplateType(claimed_type_id),
                        );
                    }),
                    kind!("declaration") => {
                        let _ = self.flatten_declaration::<false>(
                            DeclarationContext::TemplateGenerative(claimed_type_id),
                            true,
                            true,
                            cursor,
                        );
                    }
                    _other => cursor.could_not_match(),
                }
            })
        }
        assert!(parameters_to_visit.is_empty());
    }

    fn must_be_generative(&self, domain: DomainType, context: &str, span: Span) {
        if !domain.is_generative() {
            self.errors
                .error(span, format!("{context} must be a compile-time expression"));
        }
    }

    fn flatten_latency_specifier(&mut self, cursor: &mut Cursor<'c>) -> Option<(FlatID, Span)> {
        cursor.optional_field(field!("latency_specifier")).then(|| {
            cursor.go_down_content(kind!("latency_specifier"), |cursor| {
                let (expr, domain) = self.flatten_subexpr(cursor);
                let span = cursor.span();
                self.must_be_generative(domain, "Latency Specifier", span);
                (expr, span)
            })
        })
    }

    fn flatten_template_args(&mut self, cursor: &mut Cursor<'c>) -> Vec<WrittenTemplateArg<'c>> {
        cursor.collect_list(kind!("template_args"), |cursor| {
            cursor.go_down(kind!("template_arg"), |cursor| {
                let (name_span, name) =
                    cursor.field_span(field!("name"), kind!("identifier"));

                let (kind, value_span) = if cursor.optional_field(field!("val_arg")) {
                    let value_span = cursor.span();
                    let (expr, domain) = self.flatten_subexpr(cursor);
                    if !domain.is_generative() {
                        self.errors.error(value_span, "Template arguments must be known at compile-time!");
                    }
                    (Some(TemplateKind::Value(expr)), value_span)
                } else if cursor.optional_field(field!("type_arg")) {
                    let value_span = cursor.span();
                    let typ = self.flatten_type(cursor);
                    (Some(TemplateKind::Type(typ)), value_span)
                } else {
                    (match self.local_variable_context.get_declaration_for(name) {
                        Some(NamedLocal::TemplateType(t)) => Some(TemplateKind::Type(WrittenType::TemplateVariable(name_span, t))),
                        Some(NamedLocal::Declaration(decl_id)) => {
                            // Insert extra Expression, to support named template arg syntax #(MY_VAR, OTHER_VAR: BEEP)
                            let wire_read_id = self.instructions.alloc(Instruction::Expression(Expression {
                                output: ExpressionOutput::SubExpression(self.type_alloc.alloc_unset_type(DomainType::Generative)),
                                span: name_span,
                                domain: DomainType::Generative,
                                source: ExpressionSource::WireRef(WireReference::simple_var_read(decl_id, name_span))
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
                        None => {
                            self.errors.error(name_span, format!("{name} does not name a Type or a Value."));
                            None
                        },
                    }, name_span)
                };

                WrittenTemplateArg{ name, name_span, value_span, kind }
            })
        })
    }

    fn apply_template_args_to_global(
        &mut self,
        found_global: GlobalUUID,
        mut template_args: Vec<WrittenTemplateArg<'c>>,
    ) -> TVec<AbstractTemplateArg> {
        let link_info = self.globals.get_link_info(found_global);
        let full_object_name = link_info.get_full_name();
        let result = link_info.template_parameters.map(|(_, param)| {
            let mut found_arg : Option<WrittenTemplateArg> = None;

            template_args.retain_mut(|arg| {
                if arg.name == param.name {
                    if let Some(prev) = &found_arg {
                        self.errors.error(arg.name_span, format!("'{}' has already been defined previously", arg.name))
                            .info_same_file(prev.name_span, "Defined here previously");
                    } else {
                        found_arg = Some(WrittenTemplateArg{ name: arg.name, name_span: arg.name_span, value_span: arg.value_span, kind: arg.kind.take() });
                    }
                    false
                } else {
                    true
                }
            });

            if let Some(WrittenTemplateArg { name, name_span, value_span, kind: Some(arg) }) = found_arg {
                match (&param.kind, arg) {
                        // Correct pairing
                        (TemplateKind::Type(_), TemplateKind::Type(arg)) => {
                            let abs_typ = self.type_alloc.type_alloc.written_to_abstract_type(&arg);
                            TemplateKind::Type(TemplateArg::Provided { name_span, value_span, arg, abs_typ })
                        }
                        // Correct pairing
                        (TemplateKind::Value(_), TemplateKind::Value(arg)) => {
                            let abs_typ = self.instructions[arg].unwrap_subexpression().typ.typ.clone();
                            TemplateKind::Value(TemplateArg::Provided { name_span, value_span, arg, abs_typ })
                        }
                        (TemplateKind::Type(_), TemplateKind::Value(_)) => {
                            self.errors.error(name_span, format!("'{name}' is not a value. `type` keyword cannot be used for values"))
                                .info((param.name_span, link_info.file), "Declared here");
                            TemplateKind::Type(TemplateArg::NotProvided {
                                abs_typ: self.type_alloc.type_alloc.alloc_unknown(),
                            })
                        }
                        (TemplateKind::Value(_), TemplateKind::Type(_)) => {
                            self.errors.error(name_span, format!("'{name}' is not a type. To use template type arguments use the `type` keyword like `T: type int[123]`"))
                                .info((param.name_span, link_info.file), "Declared here");
                            TemplateKind::Value(TemplateArg::NotProvided {
                                abs_typ: self.type_alloc.type_alloc.alloc_unknown(),
                            })
                        }
                    }
            } else {
                match &param.kind {
                    TemplateKind::Type(_) => TemplateKind::Type(TemplateArg::NotProvided {
                        abs_typ: self.type_alloc.type_alloc.alloc_unknown(),
                    }),
                    TemplateKind::Value(_) => TemplateKind::Value(TemplateArg::NotProvided {
                        abs_typ: self.type_alloc.type_alloc.alloc_unknown(),
                    }),
                }
            }
        });

        for remaining_arg in template_args {
            self.errors
                .error(
                    remaining_arg.name_span,
                    format!(
                        "{} is not a valid template argument of {full_object_name}",
                        remaining_arg.name
                    ),
                )
                .info_obj(link_info);
        }

        result
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
            if let Some(global_id) = self
                .globals
                .resolve_global(name_span, &cursor.file_data.file_text[name_span])
            {
                let template_args = self.apply_template_args_to_global(global_id, template_args);

                match global_id {
                    GlobalUUID::Module(id) => LocalOrGlobal::Module(GlobalReference {
                        id,
                        name_span,
                        template_args,
                        template_span,
                    }),
                    GlobalUUID::Type(id) => LocalOrGlobal::Type(GlobalReference {
                        id,
                        name_span,
                        template_args,
                        template_span,
                    }),
                    GlobalUUID::Constant(id) => LocalOrGlobal::Constant(GlobalReference {
                        id,
                        name_span,
                        template_args,
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
            let (array_size_wire_id, domain, bracket_span) = self.flatten_array_bracket(cursor);
            self.must_be_generative(domain, "Array Size", span);

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
                        self.globals
                            .not_expected_global_error(&module_ref, accepted_text);
                        ModuleOrWrittenType::WrittenType(WrittenType::Error(module_ref.name_span))
                    }
                    LocalOrGlobal::Constant(constant_ref) => {
                        self.globals
                            .not_expected_global_error(&constant_ref, accepted_text);
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
                NamedLocal::TemplateType(template_id) => {
                    err_ref.info_obj_same_file(
                        &self.working_on_link_info.template_parameters[template_id],
                    );
                }
                NamedLocal::DomainDecl(domain_id) => {
                    err_ref.info_obj_same_file(&self.domains[domain_id]);
                }
            }
        }
    }

    fn alloc_submodule_instruction(
        &mut self,
        module_ref: GlobalReference<ModuleUUID>,
        name: Option<(String, Span)>,
        documentation: Documentation,
    ) -> FlatID {
        let md = &self.globals[module_ref.id];
        let local_interface_domains = md
            .domains
            .map(|_| self.type_alloc.domain_alloc.alloc_unknown());

        self.instructions
            .alloc(Instruction::SubModule(SubModuleInstance {
                name,
                module_ref,
                local_interface_domains,
                documentation,
            }))
    }

    fn flatten_declaration<const ALLOW_MODULES: bool>(
        &mut self,
        declaration_context: DeclarationContext,
        mut read_only: bool,
        declaration_itself_is_not_written_to: bool,
        cursor: &mut Cursor<'c>,
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
            let mut decl_kind = match declaration_context {
                DeclarationContext::IO{is_input} => {
                    if let Some((_, io_span)) = io_kw {
                        self.errors.error(io_span, "Cannot redeclare 'input' or 'output' on functional syntax IO");
                    }
                    DeclarationKind::RegularPort { is_input, port_id: PortID::PLACEHOLDER }
                }
                DeclarationContext::Generative(_) => {
                    if let Some((_, io_span)) = io_kw {
                        self.errors.error(io_span, "Cannot declare 'input' or 'output' to declarations in a generative context");
                    }
                    DeclarationKind::NotPort
                }
                DeclarationContext::TemplateGenerative(template_id) => {
                    if let Some((_, io_span)) = io_kw {
                        self.errors.error(io_span, "Cannot declare 'input' or 'output' on template values");
                    }
                    DeclarationKind::GenerativeInput(template_id)
                }
                DeclarationContext::PlainWire => {
                    match io_kw {
                        Some((is_input, _)) => DeclarationKind::RegularPort { is_input, port_id: PortID::PLACEHOLDER },
                        None => DeclarationKind::NotPort,
                    }
                }
                DeclarationContext::StructField => {
                    if let Some((_, io_span)) = io_kw {
                        self.errors.error(io_span, "Cannot declare 'input' or 'output' in a struct");
                    }
                    DeclarationKind::StructField { field_id: UUID::PLACEHOLDER }
                }
            };

            let identifier_type = match declaration_context {
                DeclarationContext::IO{is_input:_} | DeclarationContext::PlainWire | DeclarationContext::StructField => {
                    match declaration_modifiers {
                        Some((kw!("state"), modifier_span)) => {
                            if decl_kind.is_io_port() == Some(true) {
                                self.errors.error(modifier_span, "Inputs cannot be decorated with 'state'");
                            }
                            IdentifierType::State
                        }
                        Some((kw!("gen"), modifier_span)) => {
                            match decl_kind {
                                DeclarationKind::NotPort => {}
                                DeclarationKind::RegularPort { is_input : _, port_id : _ } | DeclarationKind::StructField { field_id:_ } => {
                                    self.errors.error(modifier_span, "Cannot declare `gen` on inputs and outputs. To declare template inputs write it between the #()");
                                    decl_kind = DeclarationKind::NotPort; // Make it not a port anymore, because it errored
                                }
                                DeclarationKind::GenerativeInput(_) => unreachable!("Caught by DeclarationContext::ForLoopGenerative | DeclarationContext::TemplateGenerative(_)")
                            }
                            IdentifierType::Generative
                        }
                        Some(_) => cursor.could_not_match(),
                        None => {
                            IdentifierType::Local
                        }
                    }
                }
                DeclarationContext::Generative(_) | DeclarationContext::TemplateGenerative(_) => {
                    if let Some((_, modifier_span)) = declaration_modifiers {
                        self.errors.error(modifier_span, "Cannot add modifiers to implicitly generative declarations");
                    }
                    IdentifierType::Generative
                }
            };

            let domain = match &mut decl_kind {
                DeclarationKind::NotPort => if identifier_type.is_generative() {
                    DomainType::Generative
                } else {
                    self.type_alloc.domain_alloc.alloc_unknown()
                },
                DeclarationKind::GenerativeInput(_template_id) => {DomainType::Generative}
                DeclarationKind::StructField { field_id } => {*field_id = self.fields_to_visit.next().unwrap(); DomainType::Physical(UUID::PLACEHOLDER)}
                DeclarationKind::RegularPort { is_input:_, port_id } => {*port_id = self.ports_to_visit.next().unwrap(); DomainType::Physical(self.named_domain_alloc.peek())}
            };

            cursor.field(field!("type"));
            let decl_span = Span::new_overarching(cursor.span(), whole_declaration_span.empty_span_at_end());
            let typ_or_module_expr = self.flatten_module_or_type::<ALLOW_MODULES>(cursor);

            let (name_span, name) = cursor.field_span(field!("name"), kind!("identifier"));

            let span_latency_specifier = self.flatten_latency_specifier(cursor);
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
                    let submod_id = self.alloc_submodule_instruction(module_ref, Some((name.to_owned(), name_span)), documentation);

                    self.alloc_local_name(name_span, name, NamedLocal::SubModule(submod_id));

                    return submod_id
                }
            };

            let typ = self.type_alloc.type_alloc.written_to_abstract_type(&typ_expr);

            if decl_kind.implies_read_only() {
                read_only = true;
            }

            let decl_id = self.instructions.alloc(Instruction::Declaration(Declaration{
                typ_expr,
                typ : FullType {
                    typ,
                    domain,
                },
                read_only,
                declaration_itself_is_not_written_to,
                decl_kind,
                identifier_type,
                name : name.to_owned(),
                name_span,
                decl_span,
                declaration_runtime_depth : OnceCell::new(),
                latency_specifier : span_latency_specifier.map(|(ls, _)| ls),
                documentation
            }));

            self.alloc_local_name(name_span, name, NamedLocal::Declaration(decl_id));

            decl_id
        })
    }

    fn flatten_array_bracket(
        &mut self,
        cursor: &mut Cursor<'c>,
    ) -> (FlatID, DomainType, BracketSpan) {
        let bracket_span = BracketSpan::from_outer(cursor.span());
        cursor.go_down_content(kind!("array_bracket_expression"), |cursor| {
            let (expr, is_generative) = self.flatten_subexpr(cursor);
            (expr, is_generative, bracket_span)
        })
    }

    fn get_main_interface(
        &self,
        submodule_decl: FlatID,
        span: Span,
    ) -> Option<(InterfaceID, &Interface)> {
        let sm = self.instructions[submodule_decl].unwrap_submodule();

        let md = &self.globals[sm.module_ref.id];

        let result = md.get_main_interface();

        if result.is_none() {
            self.errors.error(span, format!("{} does not have a main interface. You should explicitly specify an interface to access", md.link_info.get_full_name()))
                .info_obj(md);
        }

        result
    }

    /// Produces a new [SubModuleInstance] if a global was passed, or a reference to the existing instance if it's referenced by name
    fn get_or_alloc_module(&mut self, cursor: &mut Cursor<'c>) -> Option<ModuleInterfaceReference> {
        let outer_span = cursor.span();

        match self.flatten_wire_reference(cursor) {
            PartialWireReference::GlobalModuleName(module_ref) => {
                let documentation = cursor.extract_gathered_comments();
                let interface_span = module_ref.get_total_span();
                let submodule_decl =
                    self.alloc_submodule_instruction(module_ref, None, documentation);
                Some(ModuleInterfaceReference {
                    submodule_decl,
                    submodule_interface: self.get_main_interface(submodule_decl, interface_span)?.0,
                    name_span: None,
                    interface_span,
                })
            }
            PartialWireReference::ModuleButNoPort(submodule_decl, name_span) => {
                Some(ModuleInterfaceReference {
                    submodule_decl,
                    submodule_interface: self.get_main_interface(submodule_decl, name_span)?.0,
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
            PartialWireReference::WireReference { wire_ref, .. } => {
                if !wire_ref.is_error() {
                    // Error already reported
                    self.errors.error(
                        outer_span,
                        "Function call syntax is only possible on modules or interfaces of modules",
                    );
                }
                None
            }
        }
    }

    fn flatten_subexpr(&mut self, cursor: &mut Cursor<'c>) -> (FlatID, DomainType) {
        let (source, span, domain) = self.flatten_expr_source(cursor);

        let typ = self.type_alloc.alloc_unset_type(domain);
        let wire_instance = Expression {
            domain,
            span,
            source,
            output: ExpressionOutput::SubExpression(typ),
        };
        (
            self.instructions
                .alloc(Instruction::Expression(wire_instance)),
            domain,
        )
    }

    fn flatten_assign_to_expr(&mut self, writes: Vec<WriteTo>, cursor: &mut Cursor<'c>) {
        let (source, span, domain) = self.flatten_expr_source(cursor);

        for to in &writes {
            if to.to_type.domain.is_generative() && !domain.is_generative() {
                self.errors
                    .error(span, "This value is non-generative, yet it is being assigned to a generative value")
                    .info_same_file(to.to_span, "This object is generative");
            }
        }

        let wire_instance = Expression {
            span,
            domain,
            source,
            output: ExpressionOutput::MultiWrite(writes),
        };
        self.instructions
            .alloc(Instruction::Expression(wire_instance));
    }

    fn flatten_expr_source(
        &mut self,
        cursor: &mut Cursor<'c>,
    ) -> (ExpressionSource, Span, DomainType) {
        let (kind, expr_span) = cursor.kind_span();
        let mut resulting_domain = DomainType::Generative;

        let source = match kind {
            kind!("number") => {
                let text = &cursor.file_data.file_text[expr_span];
                use std::str::FromStr;
                ExpressionSource::Constant(Value::Integer(IBig::from_str(text).unwrap()))
            }
            kind!("unary_op") => cursor.go_down_no_check(|cursor| {
                cursor.field(field!("operator"));
                let op = UnaryOperator::from_kind_id(cursor.kind());

                cursor.field(field!("right"));
                let (right, right_domain) = self.flatten_subexpr(cursor);
                resulting_domain.combine_with(right_domain);

                let rank = self.type_alloc.type_alloc.rank_substitutor.alloc_unknown();
                ExpressionSource::UnaryOp { op, rank, right }
            }),
            kind!("binary_op") => cursor.go_down_no_check(|cursor| {
                cursor.field(field!("left"));
                let (left, left_domain) = self.flatten_subexpr(cursor);
                resulting_domain.combine_with(left_domain);

                cursor.field(field!("operator"));
                let op = BinaryOperator::from_kind_id(cursor.kind());

                cursor.field(field!("right"));
                let (right, right_domain) = self.flatten_subexpr(cursor);
                resulting_domain.combine_with(right_domain);

                let rank = self.type_alloc.type_alloc.rank_substitutor.alloc_unknown();

                ExpressionSource::BinaryOp {
                    op,
                    rank,
                    left,
                    right,
                }
            }),
            kind!("func_call") => cursor.go_down_no_check(|cursor| {
                cursor.field(field!("name"));
                let interface_reference = self.get_or_alloc_module(cursor);

                cursor.field(field!("arguments"));
                let arguments_span = BracketSpan::from_outer(cursor.span());

                let arguments =
                    cursor.collect_list(kind!("parenthesis_expression_list"), |cursor| {
                        let (arg_expr, arg_domain) = self.flatten_subexpr(cursor);
                        resulting_domain.combine_with(arg_domain);
                        arg_expr
                    });

                // TODO add compile-time functions https://github.com/pc2/sus-compiler/issues/10
                if resulting_domain.is_generative() {
                    resulting_domain.combine_with(self.type_alloc.domain_alloc.alloc_unknown());
                }

                match interface_reference {
                    Some(interface_reference) => ExpressionSource::FuncCall(FuncCall {
                        interface_reference,
                        arguments,
                        arguments_span,
                    }),
                    None => ExpressionSource::WireRef(WireReference::ERROR),
                }
            }),
            kind!("parenthesis_expression") => {
                // Explicitly return so we don't alloc another WireInstance Instruction
                return cursor.go_down_content(kind!("parenthesis_expression"), |cursor| {
                    self.flatten_expr_source(cursor)
                });
            }
            kind!("array_list_expression") => {
                let list = cursor.collect_list(kind!("array_list_expression"), |cursor| {
                    let (expr_id, elem_domain) = self.flatten_subexpr(cursor);
                    resulting_domain.combine_with(elem_domain);
                    expr_id
                });
                ExpressionSource::ArrayConstruct(list)
            }
            _other => {
                let (wr, root_domain) = self.flatten_wire_reference(cursor).extract(self);

                resulting_domain.combine_with(root_domain);
                for elem in &wr.path {
                    match elem {
                        WireReferencePathElement::ArrayAccess {
                            idx,
                            bracket_span: _,
                        } => {
                            let idx_expr = self.instructions[*idx].unwrap_subexpression();
                            resulting_domain.combine_with(idx_expr.typ.domain);
                        }
                    }
                }
                ExpressionSource::WireRef(wr)
            }
        };
        (source, expr_span, resulting_domain)
    }

    fn flatten_wire_reference(&mut self, cursor: &mut Cursor<'c>) -> PartialWireReference {
        let (kind, expr_span) = cursor.kind_span();
        match kind {
        kind!("template_global") => {
            match self.flatten_local_or_template_global(cursor) {
                LocalOrGlobal::Local(span, named_obj) => match named_obj {
                    NamedLocal::Declaration(decl_id) => {
                        let root = WireReferenceRoot::LocalDecl(decl_id, expr_span);
                        PartialWireReference::WireReference{
                            wire_ref: WireReference {
                                root,
                                path: Vec::new(),
                            },
                            domain: self.instructions[decl_id]
                                .unwrap_declaration()
                                .typ.domain
                        }
                    }
                    NamedLocal::SubModule(submod_id) => {
                        PartialWireReference::ModuleButNoPort(submod_id, expr_span)
                    }
                    NamedLocal::TemplateType(template_id) => {
                        self.errors
                            .error(
                                span,
                                format!(
                                    "Expected a value, but instead found template type '{}'",
                                    self.working_on_link_info.template_parameters[template_id].name
                                ),
                            )
                            .info_obj_same_file(
                                &self.working_on_link_info.template_parameters[template_id],
                            );
                        PartialWireReference::ERROR
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
                            .info_same_file(span, format!("Domain {} declared here", domain.name));
                        PartialWireReference::ERROR
                    }
                },
                LocalOrGlobal::Constant(cst_ref) => {
                    let root = WireReferenceRoot::NamedConstant(cst_ref);
                    PartialWireReference::WireReference{
                        wire_ref: WireReference {
                        root,
                        path: Vec::new(),
                    },
                    domain: DomainType::Generative}
                }
                LocalOrGlobal::Module(md_ref) => PartialWireReference::GlobalModuleName(md_ref),
                LocalOrGlobal::Type(type_ref) => {
                    self.globals
                        .not_expected_global_error(&type_ref, "named wire: local or constant");
                    PartialWireReference::ERROR
                }
                LocalOrGlobal::NotFound(_) => PartialWireReference::ERROR, // Error handled by [flatten_local_or_template_global]
            }
        } kind!("array_op") => {
            cursor.go_down_no_check(|cursor| {
                cursor.field(field!("arr"));
                let mut flattened_arr_expr = self.flatten_wire_reference(cursor);

                cursor.field(field!("arr_idx"));
                let arr_idx_span = cursor.span();
                let (idx, _is_generative, bracket_span) = self.flatten_array_bracket(cursor);

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
                        self.errors.todo(arr_idx_span, "Module Arrays");
                    }
                    PartialWireReference::WireReference{wire_ref, domain: _} => {
                        wire_ref.path
                            .push(WireReferencePathElement::ArrayAccess { idx, bracket_span });
                    }
                }

                flattened_arr_expr
            })
        } kind!("field_access") => {
            cursor.go_down_no_check(|cursor| {
                cursor.field(field!("left"));
                let flattened_arr_expr = self.flatten_wire_reference(cursor);

                let (port_name_span, port_name) = cursor.field_span(field!("name"), kind!("identifier"));

                match flattened_arr_expr {
                    PartialWireReference::GlobalModuleName(md_ref) => {
                        self.errors.error(md_ref.get_total_span(), "Ports or interfaces can only be accessed on modules that have been explicitly declared. Declare this submodule on its own line");
                        PartialWireReference::ERROR
                    }
                    PartialWireReference::ModuleWithInterface { submodule_decl:_, submodule_name_span, interface:_, interface_name_span } => {
                        self.errors.error(port_name_span, "Omit the interface when accessing a port")
                            .suggest_remove(Span::new_overarching(submodule_name_span.empty_span_at_end(), interface_name_span));

                        PartialWireReference::ERROR
                    }
                    PartialWireReference::ModuleButNoPort(submodule_decl, submodule_name_span) => {
                        let submodule = self.instructions[submodule_decl].unwrap_submodule();

                        let submod = &self.globals[submodule.module_ref.id];

                        match submod.get_port_or_interface_by_name(port_name_span, port_name, self.errors) {
                            Some(PortOrInterface::Port(port)) => {
                                let port_info = PortReference{
                                    submodule_name_span : Some(submodule_name_span),
                                    submodule_decl,
                                    port,
                                    port_name_span : Some(port_name_span),
                                    is_input: submod.ports[port].is_input
                                };
                                PartialWireReference::WireReference{
                                    wire_ref: WireReference{
                                    root : WireReferenceRoot::SubModulePort(port_info),
                                    path : Vec::new()
                                },
                            domain: self.type_alloc.domain_alloc.alloc_unknown()}
                            }
                            Some(PortOrInterface::Interface(interface)) => {
                                PartialWireReference::ModuleWithInterface { submodule_decl, submodule_name_span, interface, interface_name_span: port_name_span }
                            }
                            None => PartialWireReference::ERROR
                        }
                    }
                    PartialWireReference::WireReference{..} => {
                        println!("TODO: Struct fields");
                        PartialWireReference::ERROR
                    }
                }
            })
        } kind!("number") => {
            self.errors
                .error(expr_span, "A constant is not a wire reference");
            PartialWireReference::ERROR
        } kind!("unary_op") | kind!("binary_op") => {
            self.errors.error(
                expr_span,
                "The result of an operator is not a wire reference",
            );
            PartialWireReference::ERROR
        } kind!("func_call") => {
            self.errors
                .error(expr_span, "A submodule call is not a wire reference");
            PartialWireReference::ERROR
        } kind!("parenthesis_expression") => {
            self.errors.error(
                expr_span,
                "Parentheses are not allowed within a wire reference",
            );
            PartialWireReference::ERROR
        } _other =>
            cursor.could_not_match()
        }
    }

    fn flatten_if_statement(&mut self, cursor: &mut Cursor<'c>) {
        cursor.go_down(kind!("if_statement"), |cursor| {
            cursor.field(field!("statement_type"));
            let expects_generative = match cursor.kind() {
                kw!("if") => true,
                kw!("when") => false,
                _ => unreachable!(),
            };
            let position_statement_keyword = cursor.span();
            cursor.field(field!("condition"));

            let (condition, condition_is_generative) = self.flatten_subexpr(cursor);
            match (expects_generative, condition_is_generative.is_generative()) {
                (true, false) => {
                    self.errors.error(
                        position_statement_keyword,
                        "Used 'if' in a non generative context, use 'when' instead",
                    );
                }
                (false, true) => {
                    self.errors.warn(
                        position_statement_keyword,
                        "Used 'when' in a generative context, use 'if' instead",
                    );
                }
                (_, _) => (),
            }

            let if_id = self
                .instructions
                .alloc(Instruction::IfStatement(IfStatement {
                    condition,
                    is_generative: expects_generative,
                    then_block: FlatIDRange::PLACEHOLDER,
                    else_block: FlatIDRange::PLACEHOLDER,
                }));

            cursor.field(field!("then_block"));
            let (then_block, else_block) = self.flatten_then_else_blocks(cursor);

            let Instruction::IfStatement(if_stmt) = &mut self.instructions[if_id] else {
                unreachable!()
            };
            if_stmt.then_block = then_block;
            if_stmt.else_block = else_block;
        })
    }

    fn flatten_then_else_blocks(&mut self, cursor: &mut Cursor<'c>) -> (FlatIDRange, FlatIDRange) {
        let then_block = self.flatten_code(cursor);

        let else_start = self.instructions.get_next_alloc_id();
        if cursor.optional_field(field!("else_block")) {
            cursor.go_down(kind!("else_block"), |cursor| {
                cursor.field(field!("content"));
                if cursor.kind() == kind!("if_statement") {
                    self.flatten_if_statement(cursor); // Chained if statements
                } else {
                    self.flatten_code(cursor);
                }
            })
        };
        let else_end = self.instructions.get_next_alloc_id();
        let else_block = FlatIDRange::new(else_start, else_end);
        (then_block, else_block)
    }

    fn flatten_code(&mut self, cursor: &mut Cursor<'c>) -> FlatIDRange {
        let old_frame = self.local_variable_context.new_frame();

        let instr_range = self.flatten_code_keep_context(cursor);

        self.local_variable_context.pop_frame(old_frame);

        instr_range
    }
    /// Returns the range in the [self.instructions] buffer corresponding to the flattened instructions
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
                            DeclarationContext::Generative(GenerativeKind::ForLoopGenerative),
                            true,
                            true,
                            cursor,
                        );

                        cursor.field(field!("from"));
                        let (start, start_domain) = self.flatten_subexpr(cursor);
                        self.must_be_generative(start_domain, "for loop start", cursor.span());

                        cursor.field(field!("to"));
                        let (end, end_domain) = self.flatten_subexpr(cursor);
                        self.must_be_generative(end_domain, "for loop end", cursor.span());

                        let for_id =
                            self.instructions
                                .alloc(Instruction::ForStatement(ForStatement {
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
                kind!("interface_statement") => {
                    cursor.go_down_no_check(|cursor| {
                        // Skip name
                        cursor.field(field!("name"));

                        if cursor.optional_field(field!("interface_ports")) {
                            self.flatten_interface_ports(
                                cursor,
                                InterfacePortsInfo::InputsThenOutputs,
                            );
                        }
                    });
                }
                kind!("domain_statement") => {
                    // Skip, because we already covered domains in initialization.
                    // TODO synchronous & async clocks

                    if self.is_implicit_clk_domain {
                        self.is_implicit_clk_domain = false;
                    } else {
                        self.named_domain_alloc.alloc();
                    }
                }
                _other => cursor.could_not_match(),
            }
            cursor.clear_gathered_comments(); // Clear comments after every statement, so comments don't bleed over
        });

        let end_of_code = self.instructions.get_next_alloc_id();
        FlatIDRange::new(start_of_code, end_of_code)
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
                if kind == kind!("declaration") {
                    let root = self.flatten_declaration::<false>(
                        self.default_declaration_context,
                        false,
                        true,
                        cursor,
                    );
                    let flat_root_decl = self.instructions[root].unwrap_declaration();
                    let is_generative = flat_root_decl.identifier_type.is_generative();
                    let domain = if is_generative {
                        DomainType::Generative
                    } else {
                        self.type_alloc.domain_alloc.alloc_unknown()
                    };
                    let to_type = self.type_alloc.alloc_unset_type(domain);
                    WriteTo {
                        to: WireReference {
                            root: WireReferenceRoot::LocalDecl(root, flat_root_decl.name_span),
                            path: Vec::new(),
                        },
                        to_span,
                        write_modifiers,
                        to_type,
                    }
                } else {
                    // It's _expression
                    let (to, domain) = self.flatten_wire_reference(cursor).extract(self);
                    let to_type = self.type_alloc.alloc_unset_type(domain);
                    WriteTo {
                        to,
                        to_span,
                        write_modifiers,
                        to_type,
                    }
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
                    let _ = self.flatten_declaration::<true>(self.default_declaration_context, false, true, cursor);
                } else { // It's _expression
                    self.flatten_assign_to_expr(Vec::new(), cursor);
                }
            });
        })
    }

    fn flatten_declaration_list(
        &mut self,
        declaration_context: DeclarationContext,
        read_only: bool,
        cursor: &mut Cursor<'c>,
    ) -> Vec<FlatID> {
        cursor.collect_list(kind!("declaration_list"), |cursor| {
            self.flatten_declaration::<false>(declaration_context, read_only, true, cursor)
        })
    }

    fn flatten_interface_ports(
        &mut self,
        cursor: &mut Cursor<'c>,
        ctx: InterfacePortsInfo,
    ) -> (Vec<FlatID>, Vec<FlatID>) {
        let (inputs_ctx, outputs_ctx) = match ctx {
            InterfacePortsInfo::InputsThenOutputs => (
                DeclarationContext::IO { is_input: true },
                DeclarationContext::IO { is_input: false },
            ),
            InterfacePortsInfo::OutputsThenInputs => (
                DeclarationContext::IO { is_input: false },
                DeclarationContext::IO { is_input: true },
            ),
            InterfacePortsInfo::ConditionalBindings => {
                (DeclarationContext::PlainWire, DeclarationContext::PlainWire)
            }
        };

        cursor.go_down(kind!("interface_ports"), |cursor| {
            (
                if cursor.optional_field(field!("inputs")) {
                    self.flatten_declaration_list(inputs_ctx, true, cursor)
                } else {
                    Vec::new()
                },
                if cursor.optional_field(field!("outputs")) {
                    self.flatten_declaration_list(outputs_ctx, false, cursor)
                } else {
                    Vec::new()
                },
            )
        })
    }

    fn flatten_global(&mut self, cursor: &mut Cursor<'c>) {
        // Skip because we covered it in initialization.
        let _ = cursor.optional_field(field!("extern_marker"));
        // Skip because we know this from initialization.
        cursor.field(field!("object_type"));

        // We parse this one a bit strangely. Just because visually it looks nicer to have the template arguments after
        // const int[SIZE] range #(int SIZE) {}
        let const_type_cursor = (cursor.kind() == kind!("const_and_type")).then(|| cursor.clone());

        let (name_span, module_name) = cursor.field_span(field!("name"), kind!("identifier"));
        self.flatten_parameters(cursor);
        println!("TREE SITTER module! {module_name}");

        if let Some(mut const_type_cursor) = const_type_cursor {
            let decl_span = const_type_cursor.span();
            const_type_cursor.go_down(kind!("const_and_type"), |const_type_cursor| {
                const_type_cursor.field(field!("const_type"));
                let typ_expr = self.flatten_type(const_type_cursor);
                let module_output_decl =
                    self.instructions
                        .alloc(Instruction::Declaration(Declaration {
                            typ_expr,
                            typ: self.type_alloc.alloc_unset_type(DomainType::Generative),
                            decl_span,
                            name_span,
                            name: module_name.to_string(),
                            declaration_runtime_depth: OnceCell::new(),
                            read_only: false,
                            declaration_itself_is_not_written_to: true,
                            decl_kind: DeclarationKind::NotPort,
                            identifier_type: IdentifierType::Generative,
                            latency_specifier: None,
                            documentation: const_type_cursor.extract_gathered_comments(),
                        }));

                self.alloc_local_name(
                    name_span,
                    module_name,
                    NamedLocal::Declaration(module_output_decl),
                );
            });
        }

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
        let Ok(mut cursor) = Cursor::new_at_root(&file.tree, file) else {
            assert!(file.associated_values.is_empty());
            continue; // Error already handled in initialization
        };

        let mut associated_value_iter = file.associated_values.iter();

        cursor.list(kind!("source_file"), |cursor| {
            cursor.go_down(kind!("global_object"), |cursor| {
                let global_obj = *associated_value_iter
                    .next()
                    .expect("Iterator cannot be exhausted");

                flatten_global(linker, global_obj, cursor);
            });
        });
    }
}

fn flatten_global(linker: &mut Linker, global_obj: GlobalUUID, cursor: &mut Cursor) {
    let obj_link_info_mut = Linker::get_link_info_mut(
        &mut linker.modules,
        &mut linker.types,
        &mut linker.constants,
        global_obj,
    );
    let errors_globals = obj_link_info_mut.take_errors_globals();
    let obj_link_info = linker.get_link_info(global_obj);
    let globals = GlobalResolver::new(linker, obj_link_info, errors_globals);

    let _panic_guard = SpanDebugger::new("flatten_global", &obj_link_info.name, cursor.file_data);

    let mut local_variable_context = LocalVariableContext::new_initial();

    let (
        ports_to_visit,
        fields_to_visit,
        default_declaration_context,
        domains,
        is_implicit_clk_domain,
    ) = match global_obj {
        GlobalUUID::Module(module_uuid) => {
            let md = &globals[module_uuid];

            for (id, domain) in &md.domains {
                if let Err(conflict) =
                    local_variable_context.add_declaration(&domain.name, NamedLocal::DomainDecl(id))
                {
                    let NamedLocal::DomainDecl(conflict) = conflict else {
                        unreachable!()
                    };

                    globals.errors.error(domain.name_span.unwrap(), format!("Conflicting domain declaration. Domain '{}' was already declared earlier", domain.name))
                    .info_obj_same_file(&md.domains[conflict]);
                }
            }

            (
                md.ports.id_range().into_iter(),
                UUIDRange::empty().into_iter(),
                DeclarationContext::PlainWire,
                &md.domains,
                !md.implicit_clk_domain,
            )
        }
        GlobalUUID::Type(type_uuid) => {
            let typ = &globals[type_uuid];
            (
                UUIDRange::empty().into_iter(),
                typ.fields.id_range().into_iter(),
                DeclarationContext::StructField,
                &FlatAlloc::EMPTY_FLAT_ALLOC,
                true,
            )
        }
        GlobalUUID::Constant(_const_uuid) => (
            UUIDRange::empty().into_iter(),
            UUIDRange::empty().into_iter(),
            DeclarationContext::Generative(GenerativeKind::PlainGenerative),
            &FlatAlloc::EMPTY_FLAT_ALLOC,
            true,
        ),
    };

    let mut context = FlatteningContext {
        globals: &globals,
        ports_to_visit,
        fields_to_visit,
        domains,
        is_implicit_clk_domain,
        default_declaration_context,
        errors: &globals.errors,
        working_on_link_info: linker.get_link_info(global_obj),
        instructions: FlatAlloc::new(),
        type_alloc: Default::default(),
        named_domain_alloc: UUIDAllocator::new(),
        local_variable_context,
    };

    context.flatten_global(cursor);

    // Make sure all ports have been visited
    assert!(context.ports_to_visit.is_empty());

    let mut instructions = context.instructions;
    let type_alloc = context.type_alloc;

    let errors_globals = globals.decommission(&linker.files);

    let link_info: &mut LinkInfo = match global_obj {
        GlobalUUID::Module(module_uuid) => {
            let md = &mut linker.modules[module_uuid];
            // Set all declaration_instruction values
            for (decl_id, instr) in &instructions {
                if let Instruction::Declaration(decl) = instr {
                    match decl.decl_kind {
                        DeclarationKind::NotPort => {}
                        DeclarationKind::RegularPort {
                            is_input: _,
                            port_id,
                        } => {
                            let port = &mut md.ports[port_id];
                            assert_eq!(port.name_span, decl.name_span);
                            port.declaration_instruction = decl_id;
                        }
                        DeclarationKind::GenerativeInput(_) => {}
                        DeclarationKind::StructField { field_id: _ } => {
                            unreachable!("No Struct fields in Modules")
                        }
                    }
                }
            }
            for (_id, port) in &md.ports {
                let Instruction::Declaration(decl) =
                    &mut instructions[port.declaration_instruction]
                else {
                    unreachable!()
                };
                decl.typ.domain = DomainType::Physical(port.domain);
            }

            if crate::debug::is_enabled("print-flattened-pre-typecheck") {
                md.print_flattened_module(&linker.files[md.link_info.file]);
            }

            &mut md.link_info
        }
        GlobalUUID::Type(type_uuid) => {
            let typ = &mut linker.types[type_uuid];

            // Set all declaration_instruction values
            for (decl_id, instr) in &instructions {
                if let Instruction::Declaration(decl) = instr {
                    match decl.decl_kind {
                        DeclarationKind::NotPort => {
                            assert!(
                                decl.identifier_type == IdentifierType::Generative,
                                "If a variable isn't generative, then it MUST be a struct field"
                            )
                        }
                        DeclarationKind::StructField { field_id } => {
                            let field = &mut typ.fields[field_id];
                            assert_eq!(field.name_span, decl.name_span);
                            field.declaration_instruction = decl_id;
                        }
                        DeclarationKind::RegularPort {
                            is_input: _,
                            port_id: _,
                        } => {
                            unreachable!("No ports in structs")
                        }
                        DeclarationKind::GenerativeInput(_) => {}
                    }
                }
            }
            &mut typ.link_info
        }
        GlobalUUID::Constant(const_uuid) => {
            let cst = &mut linker.constants[const_uuid];

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

    // Make the template parameters point to the proper declaration instructions
    for (decl_id, instr) in &mut instructions {
        if let Instruction::Declaration(decl) = instr {
            if let DeclarationKind::GenerativeInput(this_template_id) = decl.decl_kind {
                let TemplateKind::Value(GenerativeParameterKind {
                    decl_span: _,
                    declaration_instruction,
                }) = &mut link_info.template_parameters[this_template_id].kind
                else {
                    unreachable!()
                };

                *declaration_instruction = decl_id;
            }
        }
    }

    link_info.reabsorb_errors_globals(errors_globals, AFTER_FLATTEN_CP);
    link_info.type_variable_alloc =
        Some(Box::new((type_alloc.type_alloc, type_alloc.domain_alloc)));
    link_info.instructions = instructions;
}
