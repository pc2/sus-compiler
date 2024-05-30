
use lsp_types::{LanguageString, MarkedString};

use crate::{
    flattening::{FlatID, InterfaceToDomainMap, Module}, instantiation::{SubModuleOrWire, CALCULATE_LATENCY_LATER}, linker::{FileData, LinkInfo, Linker, NameElem}, parser::Documentation
};

use super::tree_walk::{InModule, LocationInfo};


struct HoverCollector<'l> {
    list : Vec<MarkedString>,
    linker : &'l Linker,
    file_data : &'l FileData,
}

impl<'l> HoverCollector<'l> {
    fn documentation(&mut self, d : &Documentation) {
        self.list.push(MarkedString::String(d.to_string(&self.file_data.file_text)))
    }
    fn documentation_link_info(&mut self, link_info : &LinkInfo) {
        self.list.push(MarkedString::String(link_info.documentation.to_string(&self.linker.files[link_info.file].file_text)))
    }
    fn sus_code<Str : ToOwned<Owned = String>>(&mut self, text : Str) {
        self.list.push(MarkedString::LanguageString(LanguageString{language: "SUS".to_owned(), value : text.to_owned()}));
    }
    fn monospace<Str : AsRef<str>>(&mut self, text : Str) {
        let mut result_str = text.as_ref().replace("\n", "\n    ");
        result_str.replace_range(0..0, "    ");
        self.list.push(MarkedString::String(result_str));
    }

    fn gather_hover_infos(&mut self, md: &Module, id: FlatID, is_generative : bool) {
        md.instantiations.for_each_instance(|inst| {
            if is_generative {
                let value_str = match &inst.generation_state[id] {
                    SubModuleOrWire::SubModule(_) | SubModuleOrWire::Wire(_) => unreachable!(),
                    SubModuleOrWire::CompileTimeValue(v) => format!(" = {}", v.value.to_string()),
                    SubModuleOrWire::Unnasigned => format!("never assigned to"),
                };
                self.monospace(value_str);
            } else {
                for (_id, wire) in &inst.wires {
                    if wire.original_instruction != id {continue}
                    let typ_str = wire.typ.to_string(&self.linker.types);
                    let name_str = &wire.name;
                    let latency_str = if wire.absolute_latency != CALCULATE_LATENCY_LATER {
                        format!("{}", wire.absolute_latency)
                    } else {
                        "?".to_owned()
                    };
                    self.sus_code(format!("{typ_str} {name_str}'{latency_str}"));
                }
            }
        });
    }
}

pub fn hover(info: LocationInfo, linker: &Linker, file_data: &FileData) -> Vec<MarkedString> {
    let mut hover = HoverCollector {
        list : Vec::new(),
        linker,
        file_data,
    };

    match info {
        LocationInfo::InModule(_md_id, md, decl_id, InModule::NamedLocal(decl)) => {
            let typ_str = decl.typ.to_string(&linker.types, &md.interfaces);
            let name_str = &decl.name;

            let identifier_type_keyword = decl.identifier_type.get_keyword();
            hover.documentation(&decl.documentation);
            hover.sus_code(format!("{identifier_type_keyword} {typ_str} {name_str}"));

            hover.gather_hover_infos(md, decl_id, decl.identifier_type.is_generative());
        }
        LocationInfo::InModule(_, md, _, InModule::NamedSubmodule(submod)) => {
            let submodule = &linker.modules[submod.module_uuid];
        
            // Declaration's documentation
            hover.documentation(&submod.documentation);

            hover.sus_code(format!("{} {}", submodule.link_info.get_full_name(), submod.name.as_ref().expect("Impossible to select an unnamed submodule").0));
            
            let show_interfaces = submodule.is_multi_interface().then_some(InterfaceToDomainMap{
                local_domain_map: &submod.local_interface_domains,
                domains: &md.domains,
            });
            hover.sus_code(submodule.make_all_ports_info_string(&linker.files[submodule.link_info.file].file_text, show_interfaces));
        
            // Module documentation
            hover.documentation_link_info(&submodule.link_info);
        }
        LocationInfo::InModule(_md_id, md, id, InModule::Temporary(wire)) => {
            let typ_str = wire.typ.to_string(&linker.types, &md.interfaces);

            let gen_kw = if wire.typ.is_generative() {"gen "} else {""};
            hover.sus_code(format!("{gen_kw}{typ_str}"));
            hover.gather_hover_infos(md, id, wire.typ.is_generative());
        }
        LocationInfo::Type(typ) => {
            hover.sus_code(typ.to_type().to_string(&linker.types));
        }
        LocationInfo::Global(global) => {
            if let Some(link_info) = linker.get_link_info(global) {
                hover.documentation_link_info(link_info);
            }
            hover.sus_code(format!("{}", linker.get_full_name(global)));
            match global {
                NameElem::Module(md_uuid) => {
                    let md = &linker.modules[md_uuid];
                    hover.sus_code(md.make_all_ports_info_string(&linker.files[md.link_info.file].file_text, None));
                }
                NameElem::Type(_) => {}
                NameElem::Constant(_) => {}
            }
        }
        LocationInfo::Port(_sm, md, port_id) => {
            hover.sus_code(md.make_port_info_string(port_id, &linker.files[md.link_info.file].file_text));
        }
        LocationInfo::Interface(_md_uuid, md, interface_id, _) => {
            hover.sus_code(md.make_interface_info_string(interface_id, &linker.files[md.link_info.file].file_text));
        }
    };

    hover.list
}
