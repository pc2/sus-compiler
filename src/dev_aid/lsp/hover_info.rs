use crate::instantiation::InstantiatedModule;
use crate::prelude::*;

use crate::typing::template::TemplateKind;
use crate::{
    dev_aid::lsp::tree_walk::LocationKind,
    flattening::FieldDeclKind,
    flattening::{GlobalReference, PathElemRefersTo, WireReferenceRoot},
    instantiation::SubModuleOrWire,
    linker::LinkerGlobals,
    linker::{Documentation, FileData, GlobalObj, GlobalUUID},
    to_string::display_all_infer_params,
};

use super::tree_walk::LocationInfo;

use lsp_types::{LanguageString, MarkedString};

struct HoverCollector<'l> {
    list: Vec<MarkedString>,
    linker: &'l Linker,
}

impl HoverCollector<'_> {
    fn documentation(&mut self, d: &Documentation, file_data: &FileData) {
        self.list
            .push(MarkedString::String(d.to_string(&file_data.file_text)))
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

    fn gather_decl_hover_infos(
        &mut self,
        obj_id: GlobalUUID,
        id: FlatID,
        instantiated_modules: &[&InstantiatedModule],
    ) {
        let GlobalUUID::Module(md_id) = obj_id else {
            return;
        };

        let decl = self.linker.modules[md_id].link_info.instructions[id].unwrap_declaration();
        let is_generative = decl.decl_kind.is_generative();

        for inst in instantiated_modules {
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
                    self.sus_code(wire.display_decl(&self.linker.globals).to_string());
                }
            }
        }
    }

    fn gather_interface_hover_infos(
        &mut self,
        obj_id: GlobalUUID,
        interf_id: FlatID,
        instantiated_modules: &[&InstantiatedModule],
    ) {
        let GlobalUUID::Module(md_id) = obj_id else {
            return;
        };

        let md = &self.linker.modules[md_id];

        for inst in instantiated_modules {
            self.sus_code(
                inst.display_action(md, interf_id, &self.linker.globals)
                    .to_string(),
            );
        }
    }

    fn gather_submodule_hover_infos(
        &mut self,
        global_id: GlobalUUID,
        submodule_instr: FlatID,
        instantiated_modules: &[&InstantiatedModule],
    ) {
        let GlobalObj::Module(_md_id) = global_id else {
            return;
        };
        for inst in instantiated_modules {
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

    fn gather_type_param_hover_infos(
        &mut self,
        globals: &LinkerGlobals,
        global_id: GlobalUUID,
        type_param: TemplateID,
        instantiated_modules: &[&InstantiatedModule],
    ) {
        let GlobalObj::Module(_md_id) = global_id else {
            return;
        };
        for inst in instantiated_modules {
            let typ = inst.global_ref.template_args[type_param]
                .unwrap_type()
                .display(globals);
            self.sus_code(format!(" = {typ}"));
        }
    }

    fn hover_decl(
        &mut self,
        linker: &Linker,
        in_global: GlobalUUID,
        decl_id: FlatID,
        instantiated_modules: &[&InstantiatedModule],
    ) {
        let link_info = &linker.globals[in_global];
        let link_info_file_data = &linker.files[link_info.span.file];
        let decl = link_info.instructions[decl_id].unwrap_declaration();
        self.documentation(&decl.documentation, link_info_file_data);

        let latency_domains = match in_global {
            GlobalObj::Module(md_id) => Some(&linker.modules[md_id].latency_domains),
            GlobalObj::Type(_) | GlobalObj::Constant(_) => None,
        };
        self.sus_code(
            link_info
                .display_decl(latency_domains, decl, &link_info_file_data.file_text)
                .to_string(),
        );

        self.gather_decl_hover_infos(in_global, decl_id, instantiated_modules);
    }

    fn hover_interface(
        &mut self,
        linker: &Linker,
        in_global: GlobalUUID,
        interface_id: FlatID,
        instantiated_modules: &[&InstantiatedModule],
    ) {
        let md = &linker.modules[in_global.unwrap_module()];
        let interface_decl = md.link_info.instructions[interface_id].unwrap_interface();

        self.sus_code(
            md.display_interface_info(
                interface_decl,
                &linker.files[md.link_info.span.file].file_text,
                true,
            )
            .to_string(),
        );

        self.gather_interface_hover_infos(in_global, interface_id, instantiated_modules);
    }

    fn hover_submodule(
        &mut self,
        linker: &Linker,
        in_global: GlobalUUID,
        submodule_id: FlatID,
        instantiated_modules: &[&InstantiatedModule],
    ) {
        // Submodules can only exist within Modules
        let md_id = in_global.unwrap_module();
        let submod_inst =
            linker.modules[md_id].link_info.instructions[submodule_id].unwrap_submodule();
        let submodule = &linker.modules[submod_inst.module_ref.id];

        self.sus_code(format!(
            "{} {}",
            submodule.link_info.display_full_name(),
            submod_inst.name
        ));

        self.gather_submodule_hover_infos(in_global, submodule_id, instantiated_modules);
    }

    fn hover_global(&mut self, linker: &Linker, global_id: GlobalUUID) {
        let link_info = &linker.globals[global_id];
        let file = &linker.files[link_info.span.file];
        self.documentation(&link_info.documentation, file);
        self.sus_code(
            link_info
                .display_full_name_and_args::<true>(&file.file_text)
                .to_string(),
        );
        match global_id {
            GlobalUUID::Module(md_uuid) => {
                let md = &linker.modules[md_uuid];
                self.sus_code(
                    md.display_all_ports_info(&linker.files[md.link_info.span.file].file_text)
                        .to_string(),
                );
            }
            GlobalUUID::Type(_) => {}
            GlobalUUID::Constant(_) => {}
        }
    }

    fn hover_global_ref<ID: Copy>(&mut self, linker: &Linker, global_ref: &GlobalReference<ID>)
    where
        GlobalUUID: From<ID>,
    {
        self.hover_global(linker, global_ref.id.into());
    }

    fn hover_type_arg(
        &mut self,
        linker: &Linker,
        global_id: GlobalUUID,
        param_id: TemplateID,
        instantiated_modules: &[&InstantiatedModule],
    ) {
        let link_info = &linker.globals[global_id];
        let parameter = &link_info.parameters[param_id];
        let arg_name = &parameter.name;
        self.monospace(format!("type {arg_name}"));
        self.gather_type_param_hover_infos(
            &linker.globals,
            global_id,
            param_id,
            instantiated_modules,
        );
    }
}

impl Linker {
    /// TODO in the future, I would like to only use the instances of types, modules and consts actually used in this module. But that would require being able to backlink instances to arbitrary [GlobalReference]s, which we can't _really_ do yet.
    fn get_all_instances_for_module(&self, global_obj: GlobalUUID) -> Vec<&InstantiatedModule> {
        let GlobalObj::Module(md_id) = global_obj else {
            return Vec::new();
        };
        self.instantiator
            .iter_for_module(md_id)
            .map(|(_template_args, inst)| inst.as_ref())
            .collect()
    }
}

pub fn hover(info: LocationInfo, linker: &Linker) -> Vec<MarkedString> {
    let mut hover = HoverCollector {
        list: Vec::new(),
        linker,
    };

    let refers_to = info.refers_to(linker);
    eprintln!("{info:?} refers to {refers_to:?}");

    match &info.kind {
        LocationKind::WireRefRoot(WireReferenceRoot::Error) => {}
        LocationKind::WireRefRoot(WireReferenceRoot::LocalDecl(decl_id)) => {
            let in_global = info.in_global.unwrap();
            let all_instances = linker.get_all_instances_for_module(in_global);
            hover.hover_decl(linker, in_global, *decl_id, &all_instances);
        }
        LocationKind::WireRefRoot(WireReferenceRoot::LocalInterface(interface_id)) => {
            let in_global = info.in_global.unwrap();
            let all_instances = linker.get_all_instances_for_module(in_global);
            hover.hover_interface(linker, in_global, *interface_id, &all_instances);
        }
        LocationKind::WireRefRoot(WireReferenceRoot::LocalSubmodule(submodule_id)) => {
            let in_global = info.in_global.unwrap();
            let all_instances = linker.get_all_instances_for_module(in_global);
            hover.hover_submodule(linker, in_global, *submodule_id, &all_instances);
        }
        LocationKind::WireRefRoot(WireReferenceRoot::NamedConstant(global_ref)) => {
            hover.hover_global_ref(linker, global_ref);
        }
        LocationKind::WireRefRoot(WireReferenceRoot::NamedModule(global_ref)) => {
            hover.hover_global_ref(linker, global_ref);
        }
        LocationKind::GlobalReference(GlobalObj::Type(global_ref)) => {
            hover.hover_global_ref(linker, global_ref);
        }
        LocationKind::GlobalReference(GlobalObj::Module(global_ref)) => {
            hover.hover_global_ref(linker, global_ref);
        }
        LocationKind::GlobalReference(GlobalObj::Constant(global_ref)) => {
            hover.hover_global_ref(linker, global_ref);
        }
        LocationKind::LocalDecl(decl_id) => {
            let in_global = info.in_global.unwrap();
            let all_instances = linker.get_all_instances_for_module(in_global);
            hover.hover_decl(linker, in_global, *decl_id, &all_instances);
        }
        LocationKind::LocalInterface(decl_id) => {
            let in_global = info.in_global.unwrap();
            let all_instances = linker.get_all_instances_for_module(in_global);
            hover.hover_interface(linker, in_global, *decl_id, &all_instances);
        }
        LocationKind::LocalSubmodule(submodule_id) => {
            let in_global = info.in_global.unwrap();
            let all_instances = linker.get_all_instances_for_module(in_global);
            hover.hover_submodule(linker, in_global, *submodule_id, &all_instances);
        }
        LocationKind::Field {
            name: _,
            name_span: _,
            refers_to,
            in_wire_ref: _,
        } => match refers_to {
            Some(PathElemRefersTo::Field(in_module_id, Some(field_id))) => {
                let md = &linker.modules[*in_module_id];
                match md.fields[*field_id].declaration_instruction {
                    Some(FieldDeclKind::Interface(decl_id)) => {
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
                    Some(FieldDeclKind::SinglePort(decl_id)) => {
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
                    None => {}
                }
            }
            Some(PathElemRefersTo::Field(_in_module_id, None)) => {}
            None => {}
        },
        LocationKind::UsedTemplateArg {
            written_arg,
            in_global_ref,
        } => {
            if let Some(param_id) = written_arg.refers_to.get() {
                let global_id = in_global_ref.get_global().0;
                let all_instances = linker.get_all_instances_for_module(global_id);
                let link_info = &linker.globals[global_id];
                match &link_info.parameters[*param_id].kind {
                    TemplateKind::Type(_type_param) => {
                        hover.hover_type_arg(linker, global_id, *param_id, &all_instances);
                    }
                    TemplateKind::Value(value_param) => {
                        hover.hover_decl(
                            linker,
                            global_id,
                            value_param.declaration_instruction,
                            &all_instances,
                        );
                    }
                }
            }
        }
        LocationKind::TypeTemplateParam(global_id, param_id) => {
            let in_global = info.in_global.unwrap();
            let all_instances = linker.get_all_instances_for_module(in_global);
            hover.hover_type_arg(linker, *global_id, *param_id, &all_instances);
        }
        LocationKind::Global(global_id) => {
            hover.hover_global(linker, *global_id);
        }
    }

    hover.list
}
