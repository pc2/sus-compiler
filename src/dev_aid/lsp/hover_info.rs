use std::borrow::Cow;

use crate::alloc::ArenaAllocator;
use crate::prelude::*;
use crate::to_string::pretty_print_concrete_instance;

use lsp_types::{LanguageString, MarkedString};

use crate::flattening::{DeclarationKind, IdentifierType, InterfaceToDomainMap, Module};
use crate::instantiation::{SubModuleOrWire, CALCULATE_LATENCY_LATER};
use crate::linker::{Documentation, FileData, GlobalUUID, LinkInfo};

use crate::typing::{
    abstract_type::DomainType,
    template::{GenerativeParameterKind, ParameterKind, TypeParameterKind},
};

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
            let md = &self.linker.modules[md_id];

            md.instantiations.for_each_instance(|_template_args, inst| {
                if is_generative {
                    let value_str = match &inst.generation_state[id] {
                        SubModuleOrWire::SubModule(_) | SubModuleOrWire::Wire(_) => unreachable!(),
                        SubModuleOrWire::CompileTimeValue(v) => format!(" = {}", v),
                        SubModuleOrWire::Unnasigned => "never assigned to".to_string(),
                    };
                    self.monospace(value_str);
                } else {
                    for (_id, wire) in &inst.wires {
                        if wire.original_instruction != id {
                            continue;
                        }
                        let typ_str = wire.typ.display(&self.linker.types);
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

    fn gather_submodule_hover_infos(&mut self, md: &Module, submodule: &Module, id: FlatID) {
        md.instantiations.for_each_instance(|_template_args, inst| {
            for (_id, sm) in &inst.submodules {
                if sm.original_instruction != id {
                    continue;
                }
                self.sus_code(pretty_print_concrete_instance(
                    &submodule.link_info,
                    &sm.template_args,
                    &self.linker.types,
                ));
            }
        });
    }
}

fn try_get_module(
    linker_modules: &ArenaAllocator<Module, ModuleUUIDMarker>,
    id: GlobalUUID,
) -> Option<&Module> {
    if let GlobalUUID::Module(md_id) = id {
        Some(&linker_modules[md_id])
    } else {
        None
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
            let mut details_vec: Vec<String> = Vec::with_capacity(5);

            if let Some(md) = try_get_module(&linker.modules, obj_id) {
                if md.implicit_clk_domain {
                    if let DomainType::Physical(ph) = decl.typ.domain {
                        details_vec.push(DomainType::physical_to_string(ph, &md.domains));
                    }
                }
            }

            match decl.decl_kind {
                DeclarationKind::RegularPort {
                    is_input,
                    port_id: _,
                } => details_vec.push(if is_input { "input" } else { "output" }.to_owned()),
                DeclarationKind::NotPort | DeclarationKind::StructField { field_id: _ } => {}
                DeclarationKind::GenerativeInput(_) => details_vec.push("param".to_owned()),
            }

            match decl.identifier_type {
                IdentifierType::Local => {}
                IdentifierType::State => details_vec.push("state".to_owned()),
                IdentifierType::Generative => details_vec.push("gen".to_owned()),
            }

            let typ_str = decl
                .typ
                .typ
                .display(&linker.types, &link_info.template_parameters)
                .to_string();
            details_vec.push(typ_str);

            details_vec.push(decl.name.clone());

            hover.documentation(&decl.documentation);
            hover.sus_code(details_vec.join(" "));

            hover.gather_hover_infos(obj_id, decl_id, decl.identifier_type.is_generative());
        }
        LocationInfo::InGlobal(obj_id, _link_info, id, InGlobal::NamedSubmodule(submod)) => {
            let md = &linker.modules[obj_id.unwrap_module()]; // Submodules can only exist within Modules
            let submodule = &linker.modules[submod.module_ref.id];

            // Declaration's documentation
            hover.documentation(&submod.documentation);

            hover.sus_code(format!(
                "{} {}",
                submodule.link_info.get_full_name(),
                submod
                    .name
                    .as_ref()
                    .expect("Impossible to select an unnamed submodule")
                    .0
            ));

            let show_interfaces = submodule
                .implicit_clk_domain
                .then_some(InterfaceToDomainMap {
                    local_domain_map: &submod.local_interface_domains,
                    domains: &md.domains,
                });
            hover.sus_code(submodule.make_all_ports_info_string(
                &linker.files[submodule.link_info.file].file_text,
                show_interfaces,
            ));

            // Module documentation
            hover.documentation_link_info(&submodule.link_info);
            hover.gather_submodule_hover_infos(md, submodule, id);
        }
        LocationInfo::InGlobal(obj_id, link_info, id, InGlobal::Temporary(wire)) => {
            let mut details_vec: Vec<Cow<str>> = Vec::with_capacity(2);
            match wire.typ.domain {
                DomainType::Generative => details_vec.push(Cow::Borrowed("gen")),
                DomainType::Physical(ph) => {
                    if let Some(md) = try_get_module(&linker.modules, obj_id) {
                        if md.implicit_clk_domain {
                            details_vec
                                .push(Cow::Owned(DomainType::physical_to_string(ph, &md.domains)))
                        }
                    }
                }
                DomainType::Unknown(_) => {
                    unreachable!("Variables should have been eliminated already")
                }
            };
            details_vec.push(Cow::Owned(
                wire.typ
                    .typ
                    .display(&linker.types, &link_info.template_parameters)
                    .to_string(),
            ));
            hover.sus_code(details_vec.join(" "));
            hover.gather_hover_infos(obj_id, id, wire.typ.domain.is_generative());
        }
        LocationInfo::Type(typ, link_info) => {
            hover.sus_code(
                typ.display(&linker.types, &link_info.template_parameters)
                    .to_string(),
            );
        }
        LocationInfo::Parameter(obj_id, link_info, _template_id, template_arg) => {
            match &template_arg.kind {
                ParameterKind::Type(TypeParameterKind {}) => {
                    hover.monospace(format!("type {}", template_arg.name));
                }
                ParameterKind::Generative(GenerativeParameterKind {
                    decl_span: _,
                    declaration_instruction,
                }) => {
                    let decl =
                        link_info.instructions[*declaration_instruction].unwrap_declaration();
                    hover.sus_code(format!(
                        "param {} {}",
                        decl.typ_expr
                            .display(&linker.types, &link_info.template_parameters),
                        template_arg.name
                    ));
                    hover.gather_hover_infos(obj_id, *declaration_instruction, true);
                }
            }
        }
        LocationInfo::Global(global) => {
            let link_info = linker.get_link_info(global);
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
        LocationInfo::Port(_sm, md, port_id) => {
            hover.sus_code(
                md.make_port_info_string(port_id, &linker.files[md.link_info.file].file_text),
            );
        }
        LocationInfo::Interface(_md_uuid, md, _, interface) => {
            hover.sus_code(
                md.make_interface_info_string(
                    interface,
                    &linker.files[md.link_info.file].file_text,
                ),
            );
        }
    };

    hover.list
}
