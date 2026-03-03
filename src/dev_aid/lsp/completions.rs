use std::fmt::Display;

use crate::{
    dev_aid::lsp::tree_walk::{LocationKind, MultiGlobalRef, get_selected_object},
    flattening::{
        Declaration, Direction, FieldDeclKind, GlobalReference, InterfaceKind, Module,
        PathElemRefersTo, SubModuleInstance,
    },
    linker::{FileData, GlobalObj, GlobalUUID, LinkInfo, LinkerGlobals},
    prelude::*,
    to_string::display_join,
    typing::template::{Parameter, TemplateKind, TypeParameterKind},
};

use lsp_types::{CompletionItem, CompletionItemKind, CompletionItemLabelDetails};

use crate::{flattening::Instruction, linker::Linker};

fn completions_fallback(linker: &Linker, file: &FileData, position: usize) -> Vec<CompletionItem> {
    let mut completions = Vec::new();

    // Local completions
    if let Some(global) = file.associated_values.iter().find_map(|g_id| {
        let global = &linker.globals[*g_id];
        global.span.contains_pos(position).then_some(global)
    }) {
        for (_id, v) in &global.instructions {
            match v {
                Instruction::Declaration(d) => {
                    let detail = Some(make_decl_detail(d, &linker.globals, &global.parameters));
                    completions.push(CompletionItem {
                        label: d.name.to_string(),
                        detail,
                        kind: Some(CompletionItemKind::VARIABLE),
                        ..Default::default()
                    });
                }
                Instruction::Interface(d) => match &d.interface_kind {
                    InterfaceKind::Action(_) | InterfaceKind::RegularInterface => {}
                    InterfaceKind::Trigger(_) => {
                        let trigger_params = suggest_interface_ports(global, file, &d.inputs);

                        let label = d.name.to_string();
                        let insert_text = Some(format!("{label}({trigger_params})"));
                        completions.push(CompletionItem {
                            label,
                            insert_text,
                            detail: Some("trigger".to_string()),
                            kind: Some(CompletionItemKind::METHOD),
                            ..Default::default()
                        });
                    }
                },
                Instruction::SubModule(submod) => {
                    let detail = Some(make_submod_detail(
                        submod,
                        &linker.globals,
                        &global.parameters,
                    ));
                    completions.push(CompletionItem {
                        label: submod.name.to_string(),
                        detail,
                        kind: Some(CompletionItemKind::INTERFACE),
                        ..Default::default()
                    });
                }
                _ => {}
            }
        }
        for (_, p) in &global.parameters {
            match &p.kind {
                TemplateKind::Type(TypeParameterKind {}) => {
                    completions.push(CompletionItem {
                        label: p.name.to_string(),
                        kind: Some(CompletionItemKind::TYPE_PARAMETER),
                        ..Default::default()
                    });
                }
                TemplateKind::Value(_) => {}
            }
        }
    }

    // Suggest other modules, constants, etc
    for (_, m) in &linker.modules {
        completions.push(CompletionItem {
            label: m.link_info.name.to_string(),
            kind: Some(CompletionItemKind::FUNCTION),
            ..Default::default()
        });
    }
    for (_, c) in &linker.constants {
        completions.push(CompletionItem {
            label: c.link_info.name.to_string(),
            kind: Some(CompletionItemKind::CONSTANT),
            ..Default::default()
        });
    }
    for (_, t) in &linker.types {
        completions.push(CompletionItem {
            label: t.link_info.name.to_string(),
            kind: Some(CompletionItemKind::STRUCT),
            ..Default::default()
        });
    }

    completions
}

fn get_module_port_completions(linker: &Linker, md: &Module) -> Vec<CompletionItem> {
    let mut completions = Vec::new();

    let file = &linker.files[md.link_info.span.file];

    for (_, field) in &md.fields {
        let instr = match field.declaration_instruction {
            Some(FieldDeclKind::Interface(instr)) | Some(FieldDeclKind::SinglePort(instr)) => instr,
            None => continue,
        };
        match &md.link_info.instructions[instr] {
            Instruction::Declaration(decl) => {
                let typ = &file.file_text[decl.typ_expr.get_span()];
                let direction = decl.decl_kind.is_io_port().unwrap();
                let kind = match direction {
                    Direction::Input => Some(CompletionItemKind::FIELD),
                    Direction::Output => Some(CompletionItemKind::VALUE),
                };
                completions.push(CompletionItem {
                    label: decl.name.clone(),
                    label_details: Some(CompletionItemLabelDetails {
                        detail: Some(format!(" {direction} {typ}")),
                        description: None,
                    }),
                    kind,
                    ..Default::default()
                });
            }
            Instruction::Interface(interf_decl) => match interf_decl.interface_kind {
                InterfaceKind::RegularInterface | InterfaceKind::Action(_) => {
                    let label = interf_decl.name.clone();
                    let action_func_params = display_join(", ", &interf_decl.inputs, |f, input| {
                        let input = md.link_info.instructions[*input].unwrap_declaration();
                        write!(f, "{}", input.name)
                    });
                    // TODO: Also add "int a, int b = myAct(...)"
                    let _action_outputs =
                        suggest_interface_ports(&md.link_info, file, &interf_decl.outputs);
                    let insert_text = Some(format!("{label}({action_func_params})"));
                    completions.push(CompletionItem {
                        label,
                        insert_text,
                        detail: Some("action".to_string()),
                        kind: Some(CompletionItemKind::METHOD),
                        ..Default::default()
                    });
                }
                InterfaceKind::Trigger(_) => {
                    let label = interf_decl.name.clone();
                    let trigger_inputs =
                        suggest_interface_ports(&md.link_info, file, &interf_decl.inputs);
                    let trigger_outputs =
                        suggest_interface_ports(&md.link_info, file, &interf_decl.outputs);
                    let insert_text = Some(
                        match (
                            interf_decl.inputs.is_empty(),
                            interf_decl.outputs.is_empty(),
                        ) {
                            (true, true) => label.to_string(),
                            (true, false) => {
                                format!("{label} : -> {trigger_outputs}")
                            }
                            (false, true) => format!("{label} : {trigger_inputs}"),
                            (false, false) => {
                                format!("{label} : {trigger_inputs} -> {trigger_outputs}")
                            }
                        },
                    );

                    completions.push(CompletionItem {
                        label,
                        insert_text,
                        detail: Some("trigger".to_string()),
                        kind: Some(CompletionItemKind::EVENT),
                        ..Default::default()
                    });
                }
            },
            Instruction::SubModule(_)
            | Instruction::Expression(_)
            | Instruction::IfStatement(_)
            | Instruction::ForStatement(_) => unreachable!(),
        }
    }
    completions
}

fn suggest_interface_ports(
    link_info: &LinkInfo,
    file: &FileData,
    ports: &[FlatID],
) -> impl Display {
    display_join(", ", ports, |f, port| {
        let port = link_info.instructions[*port].unwrap_declaration();
        let typ_expr = &file.file_text[port.typ_expr.get_span()];
        write!(f, "{typ_expr} {}", port.name)
    })
}

fn make_decl_detail(
    decl: &Declaration,
    linker_globals: &LinkerGlobals,
    params: &FlatAlloc<Parameter, TemplateIDMarker>,
) -> String {
    decl.typ_expr.display(linker_globals, params).to_string()
}
fn make_submod_detail(
    submod: &SubModuleInstance,
    linker_globals: &LinkerGlobals,
    params: &FlatAlloc<Parameter, TemplateIDMarker>,
) -> String {
    submod
        .module_ref
        .display(linker_globals, params)
        .to_string()
}

fn complete_global_ref<ID: Copy>(
    linker: &Linker,
    in_global: Option<GlobalUUID>,
    global_ref: &GlobalReference<ID>,
    position: usize,
) -> Option<Vec<CompletionItem>>
where
    GlobalUUID: From<ID>,
{
    let inside_brackets = global_ref.get_inner_bracket_span()?;
    if inside_brackets.contains_pos(position) {
        return None;
    }

    let arg_of_global = &linker.globals[GlobalUUID::from(global_ref.id)];

    let mut completions = Vec::new();
    for (param_id, param_decl) in &arg_of_global.parameters {
        if global_ref.get_arg_for(param_id).is_none() {
            let detail = Some(match &param_decl.kind {
                TemplateKind::Type(TypeParameterKind {}) => "type".to_string(),
                TemplateKind::Value(v) => {
                    let v_decl =
                        arg_of_global.instructions[v.declaration_instruction].unwrap_declaration();

                    make_decl_detail(v_decl, &linker.globals, &arg_of_global.parameters)
                }
            });

            let insert_text = Some(
                if let Some(in_global) = in_global
                    && linker.globals[in_global]
                        .instructions
                        .iter()
                        .any(|(_, instr)| {
                            if let Instruction::Declaration(decl) = instr
                                && decl.name == param_decl.name
                            {
                                true
                            } else {
                                false
                            }
                        })
                {
                    // If there's a declaration by that name, we don't need the colon, we can simply insert the name and it'll map immediately.
                    param_decl.name.clone()
                } else {
                    format!("{}: ", param_decl.name)
                },
            );
            completions.push(CompletionItem {
                label: param_decl.name.clone(),
                insert_text,
                detail,
                kind: Some(CompletionItemKind::PROPERTY),
                ..Default::default()
            });
        }
    }
    Some(completions)
}
fn complete_multi_global_ref(
    linker: &Linker,
    in_global: Option<GlobalUUID>,
    global_ref: &MultiGlobalRef,
    position: usize,
) -> Option<Vec<CompletionItem>> {
    match global_ref {
        GlobalObj::Module(r) => complete_global_ref(linker, in_global, r, position),
        GlobalObj::Type(r) => complete_global_ref(linker, in_global, r, position),
        GlobalObj::Constant(r) => complete_global_ref(linker, in_global, r, position),
    }
}

pub fn gather_completions(
    linker: &Linker,
    file_uuid: FileUUID,
    position: usize,
) -> Vec<CompletionItem> {
    let file = &linker.files[file_uuid];
    let Some(found_location) = get_selected_object(linker, file, position) else {
        return completions_fallback(linker, file, position);
    };

    let special_completion = match &found_location.kind {
        LocationKind::Field {
            refers_to: Some(refers_to),
            ..
        } => match refers_to {
            PathElemRefersTo::Field(in_module, _interf_opt) => {
                let md = &linker.modules[*in_module];

                Some(get_module_port_completions(linker, md))
            }
        },
        LocationKind::GlobalReference(global_ref) => {
            complete_multi_global_ref(linker, found_location.in_global, global_ref, position)
        }
        LocationKind::UsedTemplateArg(_global_obj, _param, global_ref) => {
            complete_multi_global_ref(linker, found_location.in_global, global_ref, position)
        }
        /*LocationKind::WireRefRoot(wire_reference_root) => todo!(),
        LocationKind::LocalDecl(uuid) => todo!(),
        LocationKind::LocalInterface(uuid) => todo!(),
        LocationKind::LocalSubmodule(uuid) => todo!(),
        LocationKind::Global(global_obj) => todo!(),*/
        _ => None,
    };

    if let Some(special_completion) = special_completion {
        special_completion
    } else {
        completions_fallback(linker, file, position)
    }
}
