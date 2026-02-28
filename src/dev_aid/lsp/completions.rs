use std::fmt::Display;

use crate::{
    flattening::{
        Direction, FieldDeclKind, InterfaceKind, Module, PathElemRefersTo, WireReferencePathElement,
    },
    linker::{FileData, LinkInfo},
    prelude::*,
    to_string::display_join,
};

use lsp_types::{CompletionItem, CompletionItemKind, CompletionItemLabelDetails};
use sus_proc_macro::kind;
use tree_sitter::Node;

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

fn try_find_known_completion_parent<'tree>(
    mut cur_node: Node<'tree>,
    root_node: Node<'tree>,
) -> Option<u16> {
    let mut total_tree = String::new();
    while cur_node != root_node {
        use std::fmt::Write;
        write!(total_tree, " - {}", cur_node.kind()).unwrap();
        let Some(parent) = cur_node.parent() else {
            break;
        };
        cur_node = parent;
    }
    info!("Total tree: {total_tree}");
    None
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

fn complete_field_access(
    linker: &Linker,
    file: &FileData,
    position: usize,
    link_info: &LinkInfo,
    template_args_node: Node,
) -> Vec<CompletionItem> {
    assert_eq!(template_args_node.kind(), "field_access");
    if let Some((wr, _)) = link_info
        .iter_wire_refs()
        .find(|(wr, _)| wr.get_total_span().contains_pos(position))
    {
        for pe in &wr.path {
            match pe {
                WireReferencePathElement::FieldAccess {
                    name: _,
                    name_span,
                    refers_to,
                } => {
                    if !name_span.contains_pos(position) {
                        continue;
                    }
                    let Some(refers_to) = refers_to.get() else {
                        continue;
                    };
                    match refers_to {
                        PathElemRefersTo::Field(in_module, _interf_opt) => {
                            let md = &linker.modules[*in_module];

                            return get_module_port_completions(md, file);
                        }
                    }
                }
                WireReferencePathElement::ArrayAccess { .. }
                | WireReferencePathElement::ArraySlice { .. }
                | WireReferencePathElement::ArrayPartSelect { .. } => continue,
            }
        }
    }
    //if let Some((span, info)) = get_selected_object(linker, file, template_args_node.start_byte());
    completions_fallback(linker, position)
}

fn complete_template_args(
    linker: &Linker,
    file: &FileData,
    position: usize,
    link_info: &LinkInfo,
    template_args_node: Node,
) -> Vec<CompletionItem> {
    assert_eq!(template_args_node.kind(), "template_args");

    completions_fallback(linker, position)
}

impl FileData {
    fn find_global_for_position<'l>(
        &self,
        linker: &'l Linker,
        position: usize,
    ) -> Option<&'l LinkInfo> {
        self.associated_values.iter().find_map(|assoc_id| {
            let global = &linker.globals[*assoc_id];
            global.span.contains_pos(position).then_some(global)
        })
    }
}

pub fn gather_completions(
    linker: &Linker,
    file_uuid: FileUUID,
    position: usize,
) -> Vec<CompletionItem> {
    let file = &linker.files[file_uuid];
    let context_node = file
        .tree
        .root_node()
        .named_descendant_for_byte_range(position - 1, position - 1); // Seemingly tree sitter won't target a node if we're right at the end of it - a common case for completions. That's why we go back by one. 

    let in_global = file.find_global_for_position(linker, position);

    let Some(mut context_node) = context_node else {
        warn!(
            "Normally it should be impossible to request a completion outside the tree, but granted LSP weirdness we just do nothing to defend against crashes."
        );
        return completions_fallback(linker, position);
    };

    if context_node.kind_id() == kind!("number") {
        return Vec::new(); // No completions for numbers - DUH
    }
    if context_node.kind_id() == kind!("identifier") {
        context_node = context_node
            .parent()
            .expect("Identifier can't not have a parent");
    }
    let context_node_kind = context_node.kind();
    let context_node_span = context_node.byte_range();
    let completion_prefix = &file.file_text.file_text[context_node_span.start..position];

    info!("In node {context_node_kind} try completing '{completion_prefix}'");

    try_find_known_completion_parent(context_node, file.tree.root_node());

    match (context_node.kind_id(), in_global) {
        (kind!("field_access"), Some(in_global)) => {
            complete_field_access(linker, file, position, in_global, context_node)
        }
        (kind!("template_args"), Some(in_global)) => {
            // TODO: If we're outside of the brackets, then don't.
            complete_template_args(linker, file, position, in_global, context_node)
        }
        (kind!("template_arg"), Some(in_global)) => {
            context_node = context_node
                .parent()
                .expect("template_arg is only in template_args");

            complete_template_args(linker, file, position, in_global, context_node)
        }
        //kind!("namespace_list") |
        _ => completions_fallback(linker, position), // An expression, or a global
    }
    //completions_fallback(linker, position)
}
