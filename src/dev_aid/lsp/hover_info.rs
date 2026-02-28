use crate::prelude::*;
use crate::to_string::{display_all_infer_params, display_infer_param_info};
use crate::typing::template::TemplateKind;

use lsp_types::{LanguageString, MarkedString};

use crate::flattening::{DeclarationKind, FieldDeclKind};
use crate::instantiation::SubModuleOrWire;
use crate::linker::{Documentation, FileData, GlobalObj, GlobalUUID, LinkInfo};

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
                .to_string(&self.linker.files[link_info.span.file].file_text),
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
            for (_template_args, inst) in self.linker.instantiator.iter_for_module(md_id) {
                if is_generative {
                    let value_str = match &inst.generation_state[id] {
                        SubModuleOrWire::SubModule(_)
                        | SubModuleOrWire::Wire(_)
                        | SubModuleOrWire::SplitWire(_) => {
                            unreachable!()
                        }
                        SubModuleOrWire::CompileTimeValue(v) => format!(" = {v}"),
                        SubModuleOrWire::Unassigned => "never assigned to".to_string(),
                    };
                    self.monospace(value_str);
                } else {
                    for (_id, wire) in &inst.wires {
                        if wire.original_instruction != id {
                            continue;
                        }
                        let typ_str = wire.typ.display(self.linker);
                        let name = &wire.name;
                        let absolute_latency = &wire.absolute_latency;
                        self.sus_code(format!("{typ_str} {name}'{absolute_latency}"));
                    }
                }
            }
        }
    }

    fn gather_submodule_hover_infos(&mut self, md_id: ModuleUUID, submodule_instr: FlatID) {
        for (_template_args, inst) in self.linker.instantiator.iter_for_module(md_id) {
            for (_id, sm) in &inst.submodules {
                if sm.original_instruction == submodule_instr {
                    self.sus_code(sm.display_interface(self.linker).to_string());
                    self.monospace(
                        display_all_infer_params(self.linker, &inst.submodules, sm).to_string(),
                    );
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
        LocationInfo::InGlobal(obj_id, _link_info, decl_id, InGlobal::NamedLocal(decl)) => {
            hover.documentation(&decl.documentation);

            hover.sus_code(match obj_id {
                GlobalObj::Module(md_id) => {
                    let md = &linker.modules[md_id];
                    md.display_decl(decl, &file_data.file_text, true)
                        .to_string()
                }
                GlobalObj::Type(_) => todo!("Structs #8"),
                GlobalObj::Constant(_) => todo!("Custom Compiletime Functions #56"),
            });

            if let DeclarationKind::TemplateParameter(param_id) = &decl.decl_kind
                && let GlobalObj::Module(md) = &linker.get(obj_id)
            {
                hover.monospace(display_infer_param_info(linker, md, *param_id, None).to_string());
            }

            hover.gather_hover_infos(obj_id, decl_id, decl.decl_kind.is_generative());
        }
        LocationInfo::InGlobal(obj_id, _, decl_id, InGlobal::LocalInterface(interface)) => {
            hover.documentation(&interface.documentation);

            if let GlobalUUID::Module(md_id) = obj_id {
                let md = &linker.modules[md_id];

                hover.sus_code(
                    md.display_interface_info(interface, &file_data.file_text, true)
                        .to_string(),
                );
            }

            hover.gather_hover_infos(obj_id, decl_id, false);
        }
        LocationInfo::InGlobal(obj_id, _link_info, id, InGlobal::NamedSubmodule(submod)) => {
            let md_id = obj_id.unwrap_module();
            //let md = &linker.modules[md_id]; // Submodules can only exist within Modules
            let submodule = &linker.modules[submod.module_ref.id];

            hover.sus_code(format!(
                "{} {}",
                submodule.link_info.display_full_name(),
                submod.name
            ));

            /*hover.sus_code(submodule.make_all_ports_info_string(
                &linker.files[submodule.link_info.file].file_text,
                Some(InterfaceToDomainMap {
                    local_domain_map: submod.local_domain_map.get().unwrap(),
                    domains: &md.domains,
                }),
            ));*/

            // Module documentation
            //hover.documentation_link_info(&submodule.link_info);
            hover.gather_submodule_hover_infos(md_id, id);
        }
        LocationInfo::InGlobal(obj_id, link_info, id, InGlobal::Temporary(expr)) => {
            let domains = if let GlobalUUID::Module(md_id) = obj_id {
                &linker.modules[md_id].clocks
            } else {
                &FlatAlloc::EMPTY_FLAT_ALLOC
            };
            let domain = expr.domain.display(domains);
            let typ = expr.typ.display(linker, link_info);
            hover.sus_code(format!("{domain} {typ}"));
            hover.gather_hover_infos(obj_id, id, expr.domain.unwrap().is_generative());
        }
        LocationInfo::Type(typ, link_info) => {
            hover.sus_code(typ.display(linker, &link_info.parameters).to_string());
        }
        LocationInfo::Parameter(obj_id, link_info, _template_id, template_arg) => {
            let arg_name = &template_arg.name;
            match &template_arg.kind {
                TemplateKind::Type(TypeParameterKind {}) => {
                    hover.monospace(format!("type {arg_name}"));
                }
                TemplateKind::Value(GenerativeParameterKind {
                    decl_span: _,
                    declaration_instruction,
                }) => {
                    let decl =
                        link_info.instructions[*declaration_instruction].unwrap_declaration();
                    let typ_displ = decl.typ_expr.display(linker, &link_info.parameters);
                    hover.sus_code(format!("param {typ_displ} {arg_name}",));
                    hover.gather_hover_infos(obj_id, *declaration_instruction, true);
                }
            }
        }
        LocationInfo::Global(global) => {
            let link_info = &linker.globals[global];
            hover.documentation_link_info(link_info);
            let file = &linker.files[link_info.span.file];
            hover.sus_code(
                link_info
                    .display_full_name_and_args(&file.file_text)
                    .to_string(),
            );
            match global {
                GlobalUUID::Module(md_uuid) => {
                    let md = &linker.modules[md_uuid];
                    hover.sus_code(
                        md.display_all_ports_info(&linker.files[md.link_info.span.file].file_text)
                            .to_string(),
                    );
                }
                GlobalUUID::Type(_) => {}
                GlobalUUID::Constant(_) => {}
            }
        }
        LocationInfo::Field(_md_uuid, md, _, interface) => {
            match interface.declaration_instruction.unwrap() {
                FieldDeclKind::Interface(decl_id) => {
                    let interface_decl = md.link_info.instructions[decl_id].unwrap_interface();

                    hover.sus_code(
                        md.display_interface_info(
                            interface_decl,
                            &linker.files[md.link_info.span.file].file_text,
                            true,
                        )
                        .to_string(),
                    );
                }
                FieldDeclKind::SinglePort(decl_id) => {
                    let port_decl = md.link_info.instructions[decl_id].unwrap_declaration();

                    hover.sus_code(
                        md.display_decl(
                            port_decl,
                            &linker.files[md.link_info.span.file].file_text,
                            true,
                        )
                        .to_string(),
                    );
                }
            }
        }
    };

    hover.list
}
