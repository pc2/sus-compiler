use std::fmt::Display;

use crate::{
    dev_aid::lsp::tree_walk::{LocationKind, get_selected_object},
    flattening::{Direction, FieldDeclKind, InterfaceKind, Module, PathElemRefersTo},
    linker::FileData,
    prelude::*,
    to_string::display_join,
};

use lsp_types::{CompletionItem, CompletionItemKind, CompletionItemLabelDetails};

use crate::{flattening::Instruction, linker::Linker};

fn completions_fallback(linker: &Linker, position: usize) -> Vec<CompletionItem> {
    let mut result = Vec::new();

    for (_, m) in &linker.modules {
        result.push(CompletionItem {
            label: m.link_info.name.to_string(),
            kind: Some(CompletionItemKind::FUNCTION),
            ..Default::default()
        });

        if m.link_info.span.contains_pos(position) {
            for (_id, v) in &m.link_info.instructions {
                if let Instruction::Declaration(d) = v {
                    result.push(CompletionItem {
                        label: d.name.to_string(),
                        kind: Some(CompletionItemKind::VARIABLE),
                        ..Default::default()
                    });
                }
            }
        }
    }
    for (_, c) in &linker.constants {
        result.push(CompletionItem {
            label: c.link_info.name.to_string(),
            kind: Some(CompletionItemKind::CONSTANT),
            ..Default::default()
        });
    }
    for (_, t) in &linker.types {
        result.push(CompletionItem {
            label: t.link_info.name.to_string(),
            kind: Some(CompletionItemKind::STRUCT),
            ..Default::default()
        });
    }

    result
}

fn get_module_port_completions(md: &Module, file: &FileData) -> Vec<CompletionItem> {
    let mut completions = Vec::new();

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
                    let _action_outputs = suggest_interface_ports(md, file, &interf_decl.outputs);
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
                    let trigger_inputs = suggest_interface_ports(md, file, &interf_decl.inputs);
                    let trigger_outputs = suggest_interface_ports(md, file, &interf_decl.outputs);
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

fn suggest_interface_ports(md: &Module, file: &FileData, ports: &[FlatID]) -> impl Display {
    display_join(", ", ports, |f, port| {
        let port = md.link_info.instructions[*port].unwrap_declaration();
        let typ_expr = &file.file_text[port.typ_expr.get_span()];
        write!(f, "{typ_expr} {}", port.name)
    })
}

pub fn gather_completions(
    linker: &Linker,
    file_uuid: FileUUID,
    position: usize,
) -> Vec<CompletionItem> {
    let file = &linker.files[file_uuid];
    let Some(found_location) = get_selected_object(linker, file, position) else {
        return completions_fallback(linker, position);
    };

    match &found_location.kind {
        LocationKind::Field { refers_to, .. } => {
            let Some(refers_to) = refers_to else {
                return completions_fallback(linker, position);
            };
            match refers_to {
                PathElemRefersTo::Field(in_module, _interf_opt) => {
                    let md = &linker.modules[*in_module];

                    get_module_port_completions(md, file)
                }
            }
        }
        /*LocationKind::WireRefRoot(wire_reference_root) => todo!(),
        LocationKind::GlobalReference(global_obj) => todo!(),
        LocationKind::LocalDecl(uuid) => todo!(),
        LocationKind::LocalInterface(uuid) => todo!(),
        LocationKind::LocalSubmodule(uuid) => todo!(),
        LocationKind::Parameter(global_obj, uuid, parameter) => todo!(),
        LocationKind::Global(global_obj) => todo!(),*/
        _ => completions_fallback(linker, position),
    }
}
