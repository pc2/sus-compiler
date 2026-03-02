use crate::prelude::*;

use crate::{
    dev_aid::lsp::tree_walk::RefersTo,
    flattening::FieldDeclKind,
    instantiation::SubModuleOrWire,
    linker::{Documentation, FileData, GlobalObj, GlobalUUID, LinkInfo},
    to_string::display_all_infer_params,
    typing::template::{GenerativeParameterKind, TemplateKind, TypeParameterKind},
};

use super::tree_walk::LocationInfo;

use lsp_types::{LanguageString, MarkedString};

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

    fn gather_hover_infos(&mut self, obj_id: GlobalUUID, id: FlatID) {
        if let GlobalUUID::Module(md_id) = obj_id {
            let decl = self.linker.modules[md_id].link_info.instructions[id].unwrap_declaration();
            let is_generative = decl.decl_kind.is_generative();

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

    match info.refers_to(linker) {
        Some(RefersTo::LocalDecl(in_global, decl, decl_id)) => {
            let link_info = &linker.globals[in_global];
            hover.documentation(&decl.documentation);

            let latency_domains = match in_global {
                GlobalObj::Module(md_id) => Some(&linker.modules[md_id].latency_domains),
                GlobalObj::Type(_) | GlobalObj::Constant(_) => None,
            };
            hover.sus_code(
                link_info
                    .display_decl(latency_domains, decl, &file_data.file_text)
                    .to_string(),
            );

            hover.gather_hover_infos(in_global, decl_id);
        }
        Some(RefersTo::LocalSubModule(in_global, submod, submod_id)) => {
            // Submodules can only exist within Modules
            let md_id = in_global.unwrap_module();
            let submodule = &linker.modules[submod.module_ref.id];

            hover.sus_code(format!(
                "{} {}",
                submodule.link_info.display_full_name(),
                submod.name
            ));

            hover.gather_submodule_hover_infos(md_id, submod_id);
        }
        Some(RefersTo::Global(global)) => {
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
        Some(RefersTo::Field(in_global, field)) => {
            let md = &linker.modules[in_global.unwrap_module()];
            match field.declaration_instruction.unwrap() {
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
                        md.link_info
                            .display_decl(
                                Some(&md.latency_domains),
                                port_decl,
                                &linker.files[md.link_info.span.file].file_text,
                            )
                            .to_string(),
                    );
                }
            }
        }
        Some(RefersTo::Parameter(in_global, parameter)) => {
            let link_info = &linker.globals[in_global];
            let arg_name = &parameter.name;
            match &parameter.kind {
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
                }
            }
        }
        None => {}
    }

    hover.list
}
