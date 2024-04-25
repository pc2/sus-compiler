
use sus_proc_macro::{field, kind};


use crate::{
    arena_alloc::{FlatAlloc, UUIDMarker, UUIDRange, UUID}, file_position::{FileText, Span}, flattening::{FlattenedModule, Module}, instantiation::InstantiationList, linker::{FileBuilder, LinkInfo}, parser::Cursor
};

use super::IdentifierType;



#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PortIDMarker;
impl UUIDMarker for PortIDMarker {const DISPLAY_NAME : &'static str = "port_";}
pub type PortID = UUID<PortIDMarker>;

pub type PortIDRange = UUIDRange<PortIDMarker>;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct InterfaceIDMarker;
impl UUIDMarker for InterfaceIDMarker {const DISPLAY_NAME : &'static str = "port_";}
pub type InterfaceID = UUID<InterfaceIDMarker>;


#[derive(Debug)]
pub struct Port {
    pub name : String,
    pub name_span : Span,
    pub decl_span : Span,
    pub id_typ : IdentifierType,
    pub interface : InterfaceID
}

#[derive(Debug)]
pub struct Interface {
    pub ports_for_this_interface : PortIDRange,
    pub func_call_inputs : PortIDRange,
    pub func_call_outputs : PortIDRange
}

#[derive(Debug)]
pub struct ModulePorts {
    pub ports : FlatAlloc<Port, PortIDMarker>,
    pub interfaces : FlatAlloc<Interface, InterfaceIDMarker>
}

impl ModulePorts {
    pub const MAIN_INTERFACE_ID : InterfaceID = InterfaceID::from_hidden_value(0);

    /// Get a port by the given name. Returns None if it does not exist
    pub fn get_port_by_name(&self, name : &str) -> Option<PortID> {
        for (id, data) in &self.ports {
            if data.name == name {
                return Some(id)
            }
        }
        return None
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
}

pub fn gather_initial_file_data(builder : &mut FileBuilder) {
    let mut cursor = Cursor::new_at_root(builder.tree, builder.file_text);
    cursor.list_and_report_errors(kind!("source_file"), &builder.other_parsing_errors, |cursor| {
        let (kind, span) = cursor.kind_span();
        match kind {
            kind!("module") => {
                let parsing_errors = builder.other_parsing_errors.new_for_same_file_inherit_did_error();
                cursor.report_all_decendant_errors(&parsing_errors);
                cursor.go_down_no_check(|cursor| {
                    let name_span = cursor.field_span(field!("name"), kind!("identifier"));

                    let mut ports = FlatAlloc::new();
                    let mut interfaces = FlatAlloc::new();

                    let mut func_call_inputs = PortIDRange::empty();
                    let mut func_call_outputs = PortIDRange::empty();

                    let ports_start_at = ports.get_next_alloc_id();

                    if cursor.optional_field(field!("interface_ports")) {
                        if cursor.optional_field(field!("inputs")) {
                            func_call_inputs = gather_decl_names_in_list(IdentifierType::Input, ModulePorts::MAIN_INTERFACE_ID, &mut ports, cursor, builder.file_text);
                        }
                        if cursor.optional_field(field!("outputs")) {
                            func_call_outputs = gather_decl_names_in_list(IdentifierType::Output, ModulePorts::MAIN_INTERFACE_ID, &mut ports, cursor, builder.file_text);
                        }
                    }

                    interfaces.alloc(Interface{func_call_inputs, func_call_outputs, ports_for_this_interface : ports.range_since(ports_start_at)});

                    let md = Module{
                        link_info: LinkInfo {
                            documentation: cursor.extract_gathered_comments(),
                            file: builder.file_id,
                            name: builder.file_text[name_span].to_owned(),
                            name_span,
                            span
                        },
                        flattened: FlattenedModule::empty(parsing_errors.new_for_same_file_inherit_did_error()),
                        module_ports : ModulePorts{
                            ports,
                            interfaces
                        },
                        parsing_errors,
                        instantiations: InstantiationList::new()
                    };
    
                    builder.add_module(md);
                });
            },
            _other => cursor.could_not_match()
        }
    });
}

fn gather_decl_names_in_list(id_typ: IdentifierType, interface : InterfaceID, ports: &mut FlatAlloc<Port, PortIDMarker>, cursor: &mut Cursor, file_text : &FileText) -> PortIDRange {
    let list_start_at = ports.get_next_alloc_id();
    cursor.list(kind!("declaration_list"), |cursor| {
        let decl_span = cursor.span();
        cursor.go_down(kind!("declaration"), |cursor| {
            cursor.field(field!("type"));
            let name_span = cursor.field_span(field!("name"), kind!("identifier"));
            let name = file_text[name_span].to_owned();
            ports.alloc(Port{name, name_span, decl_span, id_typ, interface})
        });
    });
    ports.range_since(list_start_at)
}
