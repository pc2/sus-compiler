
use sus_proc_macro::{field, kind};


use crate::{
    arena_alloc::{FlatAlloc, UUID}, errors::ErrorCollector, file_position::FileText, flattening::Module, instantiation::InstantiationList, linker::{checkpoint::CheckPoint, FileBuilder, LinkInfo, ResolvedGlobals}, parser::Cursor
};

use super::*;

struct ModuleInitializationContext<'linker> {
    ports : FlatAlloc<Port, PortIDMarker>,
    interfaces : FlatAlloc<Interface, DomainIDMarker>,
    file_text : &'linker FileText
}

impl<'linker> ModuleInitializationContext<'linker> {
    fn gather_initial_module(&mut self, cursor : &mut Cursor) {
        let name_span = cursor.field_span(field!("name"), kind!("identifier"));

        self.gather_func_call_ports(name_span, cursor);
        assert!(self.interfaces.len() == 1); // Therefore the first interface is [Module::MAIN_INTERFACE_ID]

        cursor.field(field!("block"));
        self.gather_all_interfaces_in_block(cursor);
    }

    fn gather_interfaces_in_if_stmt(&mut self, cursor : &mut Cursor) {
        cursor.go_down_no_check(|cursor| {
            cursor.field(field!("condition"));
            cursor.field(field!("then_block"));
            self.gather_all_interfaces_in_block(cursor);
            if cursor.optional_field(field!("else_block")) {
                match cursor.kind() {
                    kind!("if_statement") => {
                        self.gather_interfaces_in_if_stmt(cursor);
                    }
                    kind!("block") => {
                        self.gather_all_interfaces_in_block(cursor);
                    }
                    _other => unreachable!()
                }
            }
        })
    }

    fn gather_all_interfaces_in_block(&mut self, cursor : &mut Cursor) {
        cursor.list(kind!("block"), |cursor| {
            match cursor.kind() {
                kind!("interface_statement") => {
                    cursor.go_down_no_check(|cursor| {
                        let name_span = cursor.field_span(field!("name"), kind!("identifier"));
    
                        self.gather_func_call_ports(name_span, cursor);
                    });
                },
                kind!("block") => {
                    self.gather_all_interfaces_in_block(cursor);
                }
                kind!("if_statement") => {
                    self.gather_interfaces_in_if_stmt(cursor);
                }
                kind!("for_statement") => {
                    cursor.go_down_no_check(|cursor| {
                        cursor.field(field!("for_decl"));
                        cursor.field(field!("from"));
                        cursor.field(field!("to"));
                        cursor.field(field!("block"));
                        self.gather_all_interfaces_in_block(cursor);
                    })
                }
                _other => {} // Nothing
            }
        });
    }

    fn gather_func_call_ports(&mut self, interface_name_span : Span, cursor : &mut Cursor) -> DomainID {
        let mut func_call_inputs = PortIDRange::empty();
        let mut func_call_outputs = PortIDRange::empty();
        
        let interface = self.interfaces.get_next_alloc_id();

        if cursor.optional_field(field!("interface_ports")) {
            cursor.go_down(kind!("interface_ports"), |cursor| {
                if cursor.optional_field(field!("inputs")) {
                    func_call_inputs = self.gather_decl_names_in_list(IdentifierType::Input, interface, cursor);
                }
                if cursor.optional_field(field!("outputs")) {
                    func_call_outputs = self.gather_decl_names_in_list(IdentifierType::Output, interface, cursor);
                }
            })
        }

        self.interfaces.alloc(Interface{
            func_call_inputs,
            func_call_outputs,
            name_span: interface_name_span,
            name: self.file_text[interface_name_span].to_owned()
        });

        interface
    }

    fn gather_decl_names_in_list(&mut self, id_typ: IdentifierType, interface : DomainID, cursor: &mut Cursor) -> PortIDRange {
        let list_start_at = self.ports.get_next_alloc_id();
        cursor.list(kind!("declaration_list"), |cursor| {
            let decl_span = cursor.span();
            cursor.go_down(kind!("declaration"), |cursor| {
                cursor.field(field!("type"));
                let name_span = cursor.field_span(field!("name"), kind!("identifier"));
                let name = self.file_text[name_span].to_owned();
                self.ports.alloc(Port{name, name_span, decl_span, identifier_type: id_typ, interface, declaration_instruction : UUID::PLACEHOLDER})
            });
        });
        self.ports.range_since(list_start_at)
    }
}

pub fn gather_initial_file_data(mut builder : FileBuilder) {
    let mut cursor = Cursor::new_at_root(builder.tree, builder.file_text);
    cursor.list_and_report_errors(kind!("source_file"), &builder.other_parsing_errors, |cursor| {
        let (kind, span) = cursor.kind_span();
        match kind {
            kind!("module") => {
                let parsing_errors = ErrorCollector::new_empty(builder.file_id, builder.files);
                cursor.report_all_decendant_errors(&parsing_errors);
                cursor.go_down_no_check(|cursor| {
                    let mut ctx = ModuleInitializationContext {
                        ports: FlatAlloc::new(),
                        interfaces: FlatAlloc::new(),
                        file_text: builder.file_text,
                    };

                    ctx.gather_initial_module(cursor);

                    let resolved_globals = ResolvedGlobals::empty();
                    let errors = parsing_errors.into_storage();
                    let after_initial_parse_cp = CheckPoint::checkpoint(&errors, &resolved_globals);

                    let main_interface = &ctx.interfaces[Module::MAIN_INTERFACE_ID];
                    
                    let main_interface_used = ctx.ports.iter().any(|(_, p)| p.interface == Module::MAIN_INTERFACE_ID);

                    let md = Module{
                        link_info : LinkInfo {
                            documentation: cursor.extract_gathered_comments(),
                            file: builder.file_id,
                            name : main_interface.name.clone(),
                            name_span : main_interface.name_span,
                            span,
                            errors,
                            resolved_globals,
                            after_initial_parse_cp,
                            after_flatten_cp : None
                        },
                        main_interface_used,
                        instructions : FlatAlloc::new(),
                        ports : ctx.ports,
                        domains : FlatAlloc::new(),
                        interfaces : ctx.interfaces,
                        instantiations: InstantiationList::new()
                    };

                    builder.add_module(md);
                });
            },
            _other => cursor.could_not_match()
        }
    });
}
