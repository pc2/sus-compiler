
use lsp_types::MarkedString;

use crate::{
    flattening::{FlatID, Module}, instantiation::{SubModuleOrWire, CALCULATE_LATENCY_LATER}, linker::{FileData, Linker, NameElem}
};

use super::tree_walk::{InModule, LocationInfo};


fn gather_hover_infos(md: &Module, id: FlatID, is_generative : bool, linker: &Linker, hover_list: &mut Vec<MarkedString>) {
    md.instantiations.for_each_instance(|inst| {
        if is_generative {
            let value_str = match &inst.generation_state[id] {
                SubModuleOrWire::SubModule(_) | SubModuleOrWire::Wire(_) => unreachable!(),
                SubModuleOrWire::CompileTimeValue(v) => format!(" = {}", v.value.to_string()),
                SubModuleOrWire::Unnasigned => format!("never assigned to"),
            };
            hover_list.push(MarkedString::String(value_str));
        } else {
            for (_id, wire) in &inst.wires {
                if wire.original_instruction != id {continue}
                let typ_str = wire.typ.to_string(&linker.types);
                let name_str = &wire.name;
                let latency_str = if wire.absolute_latency != CALCULATE_LATENCY_LATER {
                    format!("{}", wire.absolute_latency)
                } else {
                    "?".to_owned()
                };
                hover_list.push(MarkedString::String(format!("{typ_str} {name_str}'{latency_str}")));
            }
        }
    });
}

pub fn hover(info: LocationInfo, linker: &Linker, file_data: &FileData) -> Vec<MarkedString> {
    let mut hover_list = Vec::new();

    match info {
        LocationInfo::InModule(_md_id, md, decl_id, InModule::NamedLocal(decl)) => {
            let typ_str = decl.typ.to_string(&linker.types, &md.interfaces);
            let name_str = &decl.name;

            let identifier_type_keyword = decl.identifier_type.get_keyword();
            hover_list.push(MarkedString::String(decl.documentation.to_string(&file_data.file_text)));
            hover_list.push(MarkedString::String(format!("{identifier_type_keyword} {typ_str} {name_str}")));

            gather_hover_infos(md, decl_id, decl.identifier_type.is_generative(), &linker, &mut hover_list);
        }
        LocationInfo::InModule(_, _, _, InModule::NamedSubmodule(submod)) => {
            let submodule = &linker.modules[submod.module_uuid];
        
            // Declaration's documentation
            hover_list.push(MarkedString::String(submod.documentation.to_string(&file_data.file_text)));

            hover_list.push(MarkedString::String(format!("    {} {}", submodule.link_info.get_full_name(), submod.name.as_ref().expect("Impossible to select an unnamed submodule").0)));
            hover_list.push(MarkedString::String(submodule.make_all_ports_info_string(&linker.files[submodule.link_info.file].file_text)));
        
            // Module documentation
            hover_list.push(MarkedString::String(submodule.link_info.documentation.to_string(&linker.files[submodule.link_info.file].file_text)));
        }
        LocationInfo::InModule(_md_id, md, id, InModule::Temporary(wire)) => {
            let typ_str = wire.typ.to_string(&linker.types, &md.interfaces);

            let gen_kw = if wire.typ.is_generative() {"gen "} else {""};
            hover_list.push(MarkedString::String(format!("{gen_kw}{typ_str}")));
            gather_hover_infos(md, id, wire.typ.is_generative(), &linker, &mut hover_list);
        }
        LocationInfo::Type(typ) => {
            hover_list.push(MarkedString::String(typ.to_type().to_string(&linker.types)));
        }
        LocationInfo::Global(global) => {
            if let Some(link_info) = linker.get_link_info(global) {
                hover_list.push(MarkedString::String(link_info.documentation.to_string(&file_data.file_text)));
            }
            hover_list.push(MarkedString::String(format!("    {}", linker.get_full_name(global))));
            match global {
                NameElem::Module(md_uuid) => {
                    let md = &linker.modules[md_uuid];
                    hover_list.push(MarkedString::String(md.make_all_ports_info_string(&linker.files[md.link_info.file].file_text)));
                }
                NameElem::Type(_) => {}
                NameElem::Constant(_) => {}
            }
        }
        LocationInfo::Port(_md_uuid, md, port_id, _) => {
            hover_list.push(MarkedString::String(md.make_port_info_string(port_id, &linker.files[md.link_info.file].file_text)));
        }
        LocationInfo::Interface(_md_uuid, md, interface_id, _) => {
            hover_list.push(MarkedString::String(md.make_interface_info_string(interface_id, &linker.files[md.link_info.file].file_text)));
        }
    };

    hover_list
}
