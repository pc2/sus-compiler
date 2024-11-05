use std::borrow::Cow;

use crate::prelude::*;

use lsp_types::{LanguageString, MarkedString};

use crate::flattening::{DeclarationPortInfo, IdentifierType, InterfaceToDomainMap, Module};
use crate::instantiation::{SubModuleOrWire, CALCULATE_LATENCY_LATER};
use crate::linker::{Documentation, FileData, LinkInfo, NameElem};

use crate::typing::{
    abstract_type::DomainType,
    template::{GenerativeTemplateInputKind, TemplateInputKind, TypeTemplateInputKind},
};

use super::tree_walk::{InModule, LocationInfo};

struct HoverCollector<'l> {
    list: Vec<MarkedString>,
    linker: &'l Linker,
    file_data: &'l FileData,
}

impl<'l> HoverCollector<'l> {
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

    fn gather_hover_infos(&mut self, md: &Module, id: FlatID, is_generative: bool) {
        md.instantiations.for_each_instance(|_template_args, inst| {
            if is_generative {
                let value_str = match &inst.generation_state[id] {
                    SubModuleOrWire::SubModule(_) | SubModuleOrWire::Wire(_) => unreachable!(),
                    SubModuleOrWire::CompileTimeValue(v) => format!(" = {}", v.value),
                    SubModuleOrWire::Unnasigned => format!("never assigned to"),
                };
                self.monospace(value_str);
            } else {
                for (_id, wire) in &inst.wires {
                    if wire.original_instruction != id {
                        continue;
                    }
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
        list: Vec::new(),
        linker,
        file_data,
    };

    match info {
        LocationInfo::InModule(_md_id, md, decl_id, InModule::NamedLocal(decl)) => {
            let mut details_vec: Vec<&str> = Vec::with_capacity(5);
            let domain_str = if md.is_multi_domain() {
                if let DomainType::Physical(ph) = decl.typ.domain {
                    Some(DomainType::physical_to_string(ph, &md.domains))
                } else {
                    None
                }
            } else {
                None
            };

            if let Some(ds) = &domain_str {
                details_vec.push(ds);
            }
            match decl.is_port {
                DeclarationPortInfo::RegularPort {
                    is_input,
                    port_id: _,
                } => details_vec.push(if is_input { "input" } else { "output" }),
                DeclarationPortInfo::NotPort | DeclarationPortInfo::StructField { field_id:_ } => {}
                DeclarationPortInfo::GenerativeInput(_) => details_vec.push("param"),
            }

            match decl.identifier_type {
                IdentifierType::Local => {}
                IdentifierType::State => details_vec.push("state"),
                IdentifierType::Generative => details_vec.push("gen"),
            }

            let typ_str = decl
                .typ
                .typ
                .to_string(&linker.types, &md.link_info.template_arguments);
            details_vec.push(&typ_str);

            details_vec.push(&decl.name);

            hover.documentation(&decl.documentation);
            hover.sus_code(details_vec.join(" "));

            hover.gather_hover_infos(md, decl_id, decl.identifier_type.is_generative());
        }
        LocationInfo::InModule(_, md, _, InModule::NamedSubmodule(submod)) => {
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

            let show_interfaces = submodule.is_multi_domain().then_some(InterfaceToDomainMap {
                local_domain_map: &submod.local_interface_domains,
                domains: &md.domains,
            });
            hover.sus_code(submodule.make_all_ports_info_string(
                &linker.files[submodule.link_info.file].file_text,
                show_interfaces,
            ));

            // Module documentation
            hover.documentation_link_info(&submodule.link_info);
        }
        LocationInfo::InModule(_md_id, md, id, InModule::Temporary(wire)) => {
            let mut details_vec: Vec<Cow<str>> = Vec::with_capacity(2);
            match wire.typ.domain {
                DomainType::Generative => details_vec.push(Cow::Borrowed("gen")),
                DomainType::Physical(ph) => {
                    if md.is_multi_domain() {
                        details_vec
                            .push(Cow::Owned(DomainType::physical_to_string(ph, &md.domains)))
                    }
                }
                DomainType::DomainVariable(_) => unreachable!("Variables should have been eliminated already")
            };
            details_vec.push(Cow::Owned(
                wire.typ
                    .typ
                    .to_string(&linker.types, &md.link_info.template_arguments),
            ));
            hover.sus_code(details_vec.join(" "));
            hover.gather_hover_infos(md, id, wire.typ.domain.is_generative());
        }
        LocationInfo::Type(typ, link_info) => {
            hover.sus_code(
                typ.to_string(&linker.types, &link_info.template_arguments),
            );
        }
        LocationInfo::TemplateInput(in_obj, link_info, _template_id, template_arg) => {
            match &template_arg.kind {
                TemplateInputKind::Type(TypeTemplateInputKind {  }) => {
                    hover.monospace(format!("type {}", template_arg.name));
                }
                TemplateInputKind::Generative(GenerativeTemplateInputKind {
                    decl_span: _,
                    declaration_instruction,
                }) => {
                    let NameElem::Module(md_id) = in_obj else {
                        todo!("Non-module template args")
                    };
                    let md = &linker.modules[md_id];
                    let decl = md.link_info.instructions[*declaration_instruction].unwrap_wire_declaration();
                    hover.sus_code(format!(
                        "param {} {}",
                        decl.typ_expr
                            .to_string(&linker.types, &link_info.template_arguments),
                        template_arg.name
                    ));
                    hover.gather_hover_infos(md, *declaration_instruction, true);
                }
            }
        }
        LocationInfo::Global(global) => {
            let link_info = linker.get_link_info(global);
            hover.documentation_link_info(link_info);
            let file = &linker.files[link_info.file];
            hover.sus_code(format!("{}", link_info.get_full_name_and_template_args(&file.file_text)));
            match global {
                NameElem::Module(md_uuid) => {
                    let md = &linker.modules[md_uuid];
                    hover.sus_code(md.make_all_ports_info_string(
                        &linker.files[md.link_info.file].file_text,
                        None,
                    ));
                }
                NameElem::Type(_) => {}
                NameElem::Constant(_) => {}
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
