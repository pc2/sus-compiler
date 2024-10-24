use crate::prelude::*;

use super::walk::for_each_generative_input_in_template_args;

use super::{Instruction, Module, WireReferencePathElement};


pub fn perform_lints(linker: &mut Linker) {
    for (_, md) in &mut linker.modules {
        let errors = md.link_info.errors.take_for_editing(md.link_info.file, &linker.files);
        find_unused_variables(md, &errors);
        md.link_info.errors = errors.into_storage()
    }
}


/*
    ==== Additional Warnings ====
*/
fn find_unused_variables(md: &Module, errors: &ErrorCollector) {
    let instruction_fanins = make_fanins(&md.instructions);

    let mut is_instance_used_map: FlatAlloc<bool, FlatIDMarker> =
        md.instructions.map(|_| false);

    let mut wire_to_explore_queue: Vec<FlatID> = Vec::new();

    for (_id, port) in &md.ports {
        if !port.is_input {
            is_instance_used_map[port.declaration_instruction] = true;
            wire_to_explore_queue.push(port.declaration_instruction);
        }
    }

    while let Some(item) = wire_to_explore_queue.pop() {
        for from in &instruction_fanins[item] {
            if !is_instance_used_map[*from] {
                is_instance_used_map[*from] = true;
                wire_to_explore_queue.push(*from);
            }
        }
    }

    // Now produce warnings from the unused list
    for (id, inst) in md.instructions.iter() {
        if !is_instance_used_map[id] {
            if let Instruction::Declaration(decl) = inst {
                errors.warn(decl.name_span, "Unused Variable: This variable does not affect the output ports of this module");
            }
        }
    }
}

fn make_fanins(instructions: &FlatAlloc<Instruction, FlatIDMarker>) -> FlatAlloc<Vec<FlatID>, FlatIDMarker> {
    // Setup Wire Fanouts List for faster processing
    let mut instruction_fanins: FlatAlloc<Vec<FlatID>, FlatIDMarker> =
        instructions.map(|_| Vec::new());

    for (inst_id, inst) in instructions.iter() {
        let mut collector_func = |id| instruction_fanins[inst_id].push(id);
        match inst {
            Instruction::Write(conn) => {
                if let Some(flat_root) = conn.to.root.get_root_flat() {
                    instruction_fanins[flat_root].push(conn.from);
                    WireReferencePathElement::for_each_dependency(&conn.to.path, |idx_wire| {
                        instruction_fanins[flat_root].push(idx_wire)
                    });
                }
            }
            Instruction::SubModule(sm) => {
                for_each_generative_input_in_template_args(
                    &sm.module_ref.template_args,
                    &mut collector_func,
                );
            }
            Instruction::FuncCall(fc) => {
                for a in &fc.arguments {
                    instruction_fanins[fc.interface_reference.submodule_decl].push(*a);
                }
            }
            Instruction::Declaration(decl) => {
                decl.typ_expr.for_each_generative_input(&mut collector_func);
            }
            Instruction::Wire(wire) => {
                wire.source.for_each_dependency(collector_func);
            }
            Instruction::IfStatement(stm) => {
                for id in FlatIDRange::new(stm.then_start, stm.else_end) {
                    if let Instruction::Write(conn) = &instructions[id] {
                        if let Some(flat_root) = conn.to.root.get_root_flat() {
                            instruction_fanins[flat_root].push(stm.condition);
                        }
                    }
                }
            }
            Instruction::ForStatement(stm) => {
                instruction_fanins[stm.loop_var_decl].push(stm.start);
                instruction_fanins[stm.loop_var_decl].push(stm.end);
            }
        }
    }
    instruction_fanins
}
