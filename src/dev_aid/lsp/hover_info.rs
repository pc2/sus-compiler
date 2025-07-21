use crate::latency::CALCULATE_LATENCY_LATER;
use crate::prelude::*;
use crate::typing::domain_type::DomainType;
use crate::typing::template::TemplateKind;

use lsp_types::{LanguageString, MarkedString};

use crate::flattening::{DeclarationKind, InterfaceDeclKind, InterfaceToDomainMap};
use crate::instantiation::SubModuleOrWire;
use crate::linker::{Documentation, FileData, GlobalUUID, LinkInfo};

use crate::typing::template::{GenerativeParameterKind, TypeParameterKind};

use super::tree_walk::{InGlobal, LocationInfo};

struct HoverCollector<'l> {
    list: Vec<MarkedString>,
    linker: &'l Linker,
    file_data: &'l FileData,
}

impl HoverCollector<'_> {
    fn documentation(&mut self, d: &Documentation) {
        self.list
            .push(MarkedString::String(d.to_string(&self.file_data.file_text)))
    }
    fn documentation_link_info(&mut self, link_info: &LinkInfo) {
        self.list.push(MarkedString::String(
            link_info
                .documentation
                .to_string(&self.linker.files[link_info.file].file_text),
        ))
    }
    fn sus_code<Str: ToOwned<Owned = String>>(&mut self, text: Str) {
        self.list.push(MarkedString::LanguageString(LanguageString {
            language: "SUS".to_owned(),
            value: text.to_owned(),
        }));
    }
    fn monospace<Str: AsRef<str>>(&mut self, text: Str) {
        let mut result_str = text.as_ref().replace("\n", "\n    ");
        result_str.replace_range(0..0, "    ");
        self.list.push(MarkedString::String(result_str));
    }

    fn gather_hover_infos(&mut self, obj_id: GlobalUUID, id: FlatID, is_generative: bool) {
        if let GlobalUUID::Module(md_id) = obj_id {
            for (_template_args, inst) in self.linker.instantiator.borrow().iter_for_module(md_id) {
                if is_generative {
                    let value_str = match &inst.generation_state[id] {
                        SubModuleOrWire::SubModule(_) | SubModuleOrWire::Wire(_) => {
                            unreachable!()
                        }
                        SubModuleOrWire::CompileTimeValue(v) => format!(" = {}", v),
                        SubModuleOrWire::Unassigned => "never assigned to".to_string(),
                    };
                    self.monospace(value_str);
                } else {
                    for (_id, wire) in &inst.wires {
                        if wire.original_instruction != id {
                            continue;
                        }
                        let typ_str = wire.typ.display(self.linker, true);
                        let name_str = &wire.name;
                        let latency_str = if wire.absolute_latency != CALCULATE_LATENCY_LATER {
                            wire.absolute_latency.to_string()
                        } else {
                            "?".to_owned()
                        };
                        self.sus_code(format!("{typ_str} {name_str}'{latency_str}"));
                    }
                }
            }
        }
    }

    fn gather_submodule_hover_infos(&mut self, md_id: ModuleUUID, submodule_instr: FlatID) {
        for (_template_args, inst) in self.linker.instantiator.borrow().iter_for_module(md_id) {
            for (_id, sm) in &inst.submodules {
                if sm.original_instruction == submodule_instr {
                    self.sus_code(sm.refers_to.display(self.linker, true).to_string());
                }
            }
        }
    }
}

pub fn hover(info: LocationInfo, linker: &Linker, file_data: &FileData) -> Vec<MarkedString> {
    let mut hover = HoverCollector {
        list: Vec::new(),
        linker,
        file_data,
    };

    match info {
        LocationInfo::InGlobal(obj_id, link_info, decl_id, InGlobal::NamedLocal(decl)) => {
            let mut details_vec: Vec<String> = Vec::new();

            let domains = if let GlobalUUID::Module(md_id) = obj_id {
                &linker.modules[md_id].domains
            } else {
                &FlatAlloc::EMPTY_FLAT_ALLOC
            };
            details_vec.push(decl.domain.get().display(domains).to_string());

            match decl.decl_kind {
                DeclarationKind::Port { direction, .. } => details_vec.push(direction.to_string()),
                DeclarationKind::TemplateParameter(_) => details_vec.push("param".to_owned()),
                DeclarationKind::RegularWire { .. }
                | DeclarationKind::RegularGenerative { .. }
                | DeclarationKind::StructField(_)
                | DeclarationKind::ConditionalBinding { .. } => {}
            }

            if decl.decl_kind.is_state() {
                details_vec.push("state".to_owned());
            }

            let typ_str = decl.typ.display(linker, link_info).to_string();
            details_vec.push(typ_str);

            details_vec.push(decl.name.clone());

            hover.documentation(&decl.documentation);
            hover.sus_code(details_vec.join(" "));

            hover.gather_hover_infos(obj_id, decl_id, decl.decl_kind.is_generative());
        }
        LocationInfo::InGlobal(obj_id, _, decl_id, InGlobal::LocalInterface(interface)) => {
            hover.documentation(&interface.documentation);

            if let GlobalUUID::Module(md_id) = obj_id {
                let md = &linker.modules[md_id];

                let mut result = String::new();
                md.make_interface_info_fmt(interface, &file_data.file_text, true, &mut result);

                hover.sus_code(result);
            }

            hover.gather_hover_infos(obj_id, decl_id, false);
        }
        LocationInfo::InGlobal(obj_id, _link_info, id, InGlobal::NamedSubmodule(submod)) => {
            let md_id = obj_id.unwrap_module();
            let md = &linker.modules[md_id]; // Submodules can only exist within Modules
            let submodule = &linker.modules[submod.module_ref.id];

            // Declaration's documentation
            hover.documentation(&submod.documentation);

            hover.sus_code(format!(
                "{} {}",
                submodule.link_info.get_full_name(),
                submod.name
            ));

            hover.sus_code(submodule.make_all_ports_info_string(
                &linker.files[submodule.link_info.file].file_text,
                Some(InterfaceToDomainMap {
                    local_domain_map: &submod.local_domain_map,
                    domains: &md.domains,
                }),
            ));

            // Module documentation
            hover.documentation_link_info(&submodule.link_info);
            hover.gather_submodule_hover_infos(md_id, id);
        }
        LocationInfo::InGlobal(obj_id, link_info, id, InGlobal::Temporary(expr)) => {
            let mut details_vec: Vec<String> = Vec::new();
            let domains = if let GlobalUUID::Module(md_id) = obj_id {
                &linker.modules[md_id].domains
            } else {
                &FlatAlloc::EMPTY_FLAT_ALLOC
            };
            details_vec.push(expr.domain.display(domains).to_string());
            details_vec.push(expr.typ.display(linker, link_info).to_string());
            hover.sus_code(details_vec.join(" "));
            hover.gather_hover_infos(obj_id, id, expr.domain == DomainType::Generative);
        }
        LocationInfo::Type(typ, link_info) => {
            hover.sus_code(
                typ.display(linker, &link_info.template_parameters)
                    .to_string(),
            );
        }
        LocationInfo::Parameter(obj_id, link_info, _template_id, template_arg) => {
            match &template_arg.kind {
                TemplateKind::Type(TypeParameterKind {}) => {
                    hover.monospace(format!("type {}", template_arg.name));
                }
                TemplateKind::Value(GenerativeParameterKind {
                    decl_span: _,
                    declaration_instruction,
                }) => {
                    let decl =
                        link_info.instructions[*declaration_instruction].unwrap_declaration();
                    hover.sus_code(format!(
                        "param {} {}",
                        decl.typ_expr
                            .display(linker, &link_info.template_parameters),
                        template_arg.name
                    ));
                    hover.gather_hover_infos(obj_id, *declaration_instruction, true);
                }
            }
        }
        LocationInfo::Global(global) => {
            let link_info = &linker.globals[global];
            hover.documentation_link_info(link_info);
            let file = &linker.files[link_info.file];
            hover.sus_code(
                link_info
                    .get_full_name_and_template_args(&file.file_text)
                    .to_string(),
            );
            match global {
                GlobalUUID::Module(md_uuid) => {
                    let md = &linker.modules[md_uuid];
                    hover.sus_code(md.make_all_ports_info_string(
                        &linker.files[md.link_info.file].file_text,
                        None,
                    ));
                }
                GlobalUUID::Type(_) => {}
                GlobalUUID::Constant(_) => {}
            }
        }
        LocationInfo::Interface(_md_uuid, md, _, interface) => {
            match interface.declaration_instruction.unwrap() {
                InterfaceDeclKind::Interface(decl_id) => {
                    let interface_decl = md.link_info.instructions[decl_id].unwrap_interface();

                    let mut result = String::new();
                    md.make_interface_info_fmt(
                        interface_decl,
                        &linker.files[md.link_info.file].file_text,
                        true,
                        &mut result,
                    );
                    hover.sus_code(result);
                }
                InterfaceDeclKind::SinglePort(decl_id) => {
                    let port_decl = md.link_info.instructions[decl_id].unwrap_declaration();

                    let mut result = String::new();
                    md.make_port_info_fmt(
                        port_decl,
                        &linker.files[md.link_info.file].file_text,
                        &mut result,
                    );
                    hover.sus_code(result);
                }
            }
        }
    };

    hover.list
}
