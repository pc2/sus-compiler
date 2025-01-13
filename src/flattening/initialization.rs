use arrayvec::ArrayVec;
use sus_proc_macro::{field, kind, kw};

use crate::errors::ErrorStore;
use crate::linker::{IsExtern, NamedConstant, AFTER_INITIAL_PARSE_CP};
use crate::prelude::*;

use crate::linker::{FileBuilder, LinkInfo, ResolvedGlobals};
use crate::{file_position::FileText, flattening::Module, instantiation::InstantiationList};

use crate::typing::template::{
    GenerativeTemplateInputKind, TemplateInput, TemplateInputKind, TemplateInputs,
    TypeTemplateInputKind,
};

use super::parser::Cursor;
use super::*;

struct InitializationContext<'linker> {
    template_inputs: TemplateInputs,

    // module-only stuff
    ports: FlatAlloc<Port, PortIDMarker>,
    interfaces: FlatAlloc<Interface, InterfaceIDMarker>,
    domains: FlatAlloc<String, DomainIDMarker>,
    
    // struct-only stuff
    fields: FlatAlloc<StructField, FieldIDMarker>,

    file_text: &'linker FileText,
}

impl<'linker> InitializationContext<'linker> {
    fn gather_initial_global_object(&mut self, cursor: &mut Cursor) -> (Span, String) {
        let name_span = cursor.field_span(field!("name"), kind!("identifier"));
        let name = self.file_text[name_span].to_owned();
        self.domains.alloc(name.clone());
        if cursor.optional_field(field!("template_declaration_arguments")) {
            cursor.list(kind!("template_declaration_arguments"), |cursor| {
                let (kind, decl_span) = cursor.kind_span();
                match kind {
                    kind!("template_declaration_type") => cursor.go_down_no_check(|cursor| {
                        let name_span = cursor.field_span(field!("name"), kind!("identifier"));
                        let name = self.file_text[name_span].to_owned();
                        self.template_inputs.alloc(TemplateInput {
                            name,
                            name_span,
                            kind: TemplateInputKind::Type(TypeTemplateInputKind {}),
                        });
                    }),
                    kind!("declaration") => cursor.go_down_no_check(|cursor| {
                        let _ = cursor.optional_field(field!("io_port_modifiers"));
                        let _ = cursor.optional_field(field!("declaration_modifiers"));
                        cursor.field(field!("type"));
                        let name_span = cursor.field_span(field!("name"), kind!("identifier"));
                        let name = self.file_text[name_span].to_owned();
                        
                        self.template_inputs.alloc(TemplateInput {
                            name,
                            name_span,
                            kind: TemplateInputKind::Generative(GenerativeTemplateInputKind {
                                decl_span,
                                declaration_instruction: FlatID::PLACEHOLDER,
                            }),
                        });
                    }),
                    _other => cursor.could_not_match()
                }
                
            });
        }

        cursor.field(field!("block"));
        self.gather_all_ports_in_block(cursor);

        (name_span, name)
    }

    fn gather_ports_in_if_stmt(&mut self, cursor: &mut Cursor) {
        cursor.go_down_no_check(|cursor| {
            cursor.field(field!("if_literal"));
            cursor.field(field!("condition"));
            cursor.field(field!("then_block"));
            self.gather_all_ports_in_block(cursor);
            if cursor.optional_field(field!("else_block")) {
                match cursor.kind() {
                    kind!("if_statement") => {
                        self.gather_ports_in_if_stmt(cursor);
                    }
                    kind!("block") => {
                        self.gather_all_ports_in_block(cursor);
                    }
                    _other => unreachable!(),
                }
            }
        })
    }

    fn gather_assign_left_side(&mut self, cursor: &mut Cursor) {
        cursor.list(kind!("assign_left_side"), |cursor| {
            cursor.go_down(kind!("assign_to"), |cursor| {
                let _ = cursor.optional_field(field!("write_modifiers"));
                cursor.field(field!("expr_or_decl"));

                if cursor.kind() == kind!("declaration") {
                    let whole_decl_span = cursor.span();
                    cursor.go_down_no_check(|cursor| {
                        let is_input = cursor.optional_field(field!("io_port_modifiers")).then(|| {
                            match cursor.kind() {
                                kw!("input") => true,
                                kw!("output") => false,
                                _ => cursor.could_not_match(),
                            }
                        });
                        self.finish_gather_decl(is_input, whole_decl_span, cursor);
                    });
                }
            });
        });
    }

    fn gather_all_ports_in_block(&mut self, cursor: &mut Cursor) {
        cursor.list(kind!("block"), |cursor| {
            match cursor.kind() {
                kind!("domain_statement") => {
                    cursor.go_down_no_check(|cursor| {
                        let domain_name_span =
                            cursor.field_span(field!("name"), kind!("identifier"));
                        let name = &self.file_text[domain_name_span];
                        self.domains.alloc(name.to_owned())
                    });
                }
                kind!("interface_statement") => {
                    cursor.go_down_no_check(|cursor| {
                        let name_span = cursor.field_span(field!("name"), kind!("identifier"));

                        self.gather_func_call_ports(name_span, cursor);
                    });
                }
                kind!("block") => {
                    self.gather_all_ports_in_block(cursor);
                }
                kind!("if_statement") => {
                    self.gather_ports_in_if_stmt(cursor);
                }
                kind!("for_statement") => cursor.go_down_no_check(|cursor| {
                    cursor.field(field!("for_decl"));
                    cursor.field(field!("from"));
                    cursor.field(field!("to"));
                    cursor.field(field!("block"));
                    self.gather_all_ports_in_block(cursor);
                }),
                kind!("assign_left_side") => {
                    self.gather_assign_left_side(cursor);
                }
                kind!("decl_assign_statement") => {
                    cursor.go_down_no_check(|cursor| {
                        cursor.field(field!("assign_left"));
                        self.gather_assign_left_side(cursor);
                    });
                }
                _other => {} // Nothing
            }
        });
    }

    fn gather_func_call_ports(&mut self, interface_name_span: Span, cursor: &mut Cursor) {
        let ports = if cursor.optional_field(field!("interface_ports")) {
            cursor.go_down(kind!("interface_ports"), |cursor| {
                (
                    cursor
                        .optional_field(field!("inputs"))
                        .then(|| self.gather_decl_names_in_list(true, cursor)),
                    cursor
                        .optional_field(field!("outputs"))
                        .then(|| self.gather_decl_names_in_list(false, cursor)),
                )
            })
        } else {
            (None, None)
        };

        let (func_call_inputs, func_call_outputs) = match ports {
            (None, None) => (PortIDRange::empty(), PortIDRange::empty()),
            (None, Some(fouts)) => (PortIDRange::new(fouts.0, fouts.0), fouts),
            (Some(fins), None) => (fins, PortIDRange::new(fins.1, fins.1)),
            (Some(fins), Some(fouts)) => (fins, fouts),
        };
        // All ports are consecutive
        assert_eq!(func_call_inputs.1, func_call_outputs.0);

        self.interfaces.alloc(Interface {
            func_call_inputs,
            func_call_outputs,
            domain: self.domains.last_id(),
            name_span: interface_name_span,
            name: self.file_text[interface_name_span].to_owned(),
        });
    }

    fn gather_decl_names_in_list(&mut self, is_input: bool, cursor: &mut Cursor) -> PortIDRange {
        let list_start_at = self.ports.get_next_alloc_id();
        cursor.list(kind!("declaration_list"), |cursor| {
            let whole_decl_span = cursor.span();
            cursor.go_down(kind!("declaration"), |cursor| {
                // Skip fields if they exist
                let _ = cursor.optional_field(field!("io_port_modifiers"));
                self.finish_gather_decl(Some(is_input), whole_decl_span, cursor);
            });
        });
        self.ports.range_since(list_start_at)
    }

    fn finish_gather_decl(&mut self, is_input: Option<bool>, whole_decl_span: Span, cursor: &mut Cursor) {
        // If generative input it's a template arg
        let is_generative = if cursor.optional_field(field!("declaration_modifiers")) {
            cursor.kind() == kw!("gen")
        } else {
            false
        };

        cursor.field(field!("type"));
        let type_span = cursor.span();
        let decl_span = Span::new_overarching(type_span, whole_decl_span.empty_span_at_end());
        let name_span = cursor.field_span(field!("name"), kind!("identifier"));
        let name = self.file_text[name_span].to_owned();

        match (is_generative, is_input) {
            (false, Some(is_input)) => {
                self.ports.alloc(Port {
                    name,
                    name_span,
                    decl_span,
                    is_input,
                    domain: self.domains.last_id(),
                    declaration_instruction: FlatID::PLACEHOLDER,
                });
            }
            (false, None) => {
                self.fields.alloc(StructField{
                    name: name.clone(),
                    name_span,
                    decl_span,
                    declaration_instruction: FlatID::PLACEHOLDER
                });
            }
            _other => {}
        }
    }
}

pub fn gather_initial_file_data(mut builder: FileBuilder) {
    let mut cursor = Cursor::new_at_root(builder.tree, &builder.file_data.file_text);
    cursor.list_and_report_errors(
        kind!("source_file"),
        &builder.other_parsing_errors,
        |cursor| {
            let parsing_errors = ErrorCollector::new_empty(builder.file_id, builder.files);
            cursor.report_all_decendant_errors(&parsing_errors);

            let span = cursor.span();
            cursor.go_down(kind!("global_object"), |cursor| {
                initialize_global_object(&mut builder, parsing_errors, span, cursor);
            });
        },
    );
}

fn initialize_global_object(builder: &mut FileBuilder, parsing_errors: ErrorCollector, span: Span, cursor: &mut Cursor) {
    let is_extern = match cursor.optional_field(field!("extern_marker")).then(|| cursor.kind()) {
        None => IsExtern::Normal,
        Some(kw!("extern")) => IsExtern::Extern,
        Some(kw!("__builtin__")) => IsExtern::Builtin,
        Some(_) => cursor.could_not_match()
    };
    
    cursor.field(field!("object_type"));
    let global_obj_kind = match cursor.kind() {
        kw!("module") => {
            GlobalObjectKind::Module
        }
        kind!("const_and_type") => {
            GlobalObjectKind::Const
        }
        kw!("struct") => {
            GlobalObjectKind::Struct
        }
        _other => cursor.could_not_match()
    };
    
    let mut ctx = InitializationContext {
        ports: FlatAlloc::new(),
        interfaces: FlatAlloc::new(),
        domains: FlatAlloc::new(),
        template_inputs: FlatAlloc::new(),
        fields: FlatAlloc::new(),
        file_text: &builder.file_data.file_text,
    };

    let (name_span, name) = ctx.gather_initial_global_object(cursor);

    let mut link_info = LinkInfo {
        type_variable_alloc: TypingAllocator{domain_variable_alloc: UUIDAllocator::new(), type_variable_alloc: UUIDAllocator::new()},
        template_arguments: ctx.template_inputs,
        instructions: FlatAlloc::new(),
        documentation: cursor.extract_gathered_comments(),
        file: builder.file_id,
        name,
        name_span,
        span,
        errors: ErrorStore::new(),
        is_extern,
        resolved_globals: ResolvedGlobals::empty(),
        checkpoints: ArrayVec::new()
    };

    link_info.reabsorb_errors_globals((parsing_errors, ResolvedGlobals::empty()), AFTER_INITIAL_PARSE_CP);

    match global_obj_kind {
        GlobalObjectKind::Module => {
            builder.add_module(Module {
                link_info,
                ports: ctx.ports,
                domain_names: ctx.domains,
                domains: FlatAlloc::new(),
                interfaces: ctx.interfaces,
                instantiations: InstantiationList::new(),
            });
        }
        GlobalObjectKind::Struct => {
            builder.add_type(StructType {
                link_info,
                fields: ctx.fields
            });
        }
        GlobalObjectKind::Const => {
            builder.add_const(NamedConstant {
                link_info,
                output_decl: FlatID::PLACEHOLDER
            });
        }
    }
}
