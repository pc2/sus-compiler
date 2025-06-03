use sus_proc_macro::get_builtin_const;

use crate::linker::{IsExtern, LinkInfo, AFTER_LINTS_CP};
use crate::prelude::*;
use crate::typing::template::TemplateKind;

use super::{
    Expression, ExpressionOutput, ExpressionSource, Instruction, Module, WireReferenceRoot,
};

pub fn perform_lints(linker: &mut Linker) {
    for (_, md) in &mut linker.modules {
        let errors = ErrorCollector::from_storage(
            md.link_info.errors.take(),
            md.link_info.file,
            &linker.files,
        );
        let resolved_globals = md.link_info.resolved_globals.take();
        find_unused_variables(md, &errors);
        extern_objects_may_not_have_type_template_args(&md.link_info, &errors);
        md.link_info
            .reabsorb_errors_globals((errors, resolved_globals), AFTER_LINTS_CP);
    }
}

/*
    ==== Additional Errors ====
*/
fn extern_objects_may_not_have_type_template_args(link_info: &LinkInfo, errors: &ErrorCollector) {
    if link_info.is_extern == IsExtern::Extern {
        for (_id, arg) in &link_info.template_parameters {
            if let TemplateKind::Type(_) = &arg.kind {
                errors.error(
                    arg.name_span,
                    "'extern' modules may not have 'type' arguments. Convert to bool[] first",
                );
            }
        }
    }
}

/*
    ==== Additional Warnings ====
*/
fn find_unused_variables(md: &Module, errors: &ErrorCollector) {
    match md.link_info.is_extern {
        IsExtern::Normal => {}
        IsExtern::Extern | IsExtern::Builtin => return, // Don't report unused variables for extern modules.
    }

    let instruction_fanins = make_fanins(&md.link_info.instructions);

    let mut is_instance_used_map: FlatAlloc<bool, FlatIDMarker> =
        md.link_info.instructions.map(|_| false);

    let mut wire_to_explore_queue: Vec<FlatID> = Vec::new();

    // Output ports
    for (_id, port) in &md.ports {
        if !port.is_input {
            is_instance_used_map[port.declaration_instruction] = true;
            wire_to_explore_queue.push(port.declaration_instruction);
        }
    }

    // All asserts
    for (assert_instr_id, instr) in &md.link_info.instructions {
        if let Instruction::Expression(expr) = instr {
            if let ExpressionSource::WireRef(wr) = &expr.source {
                if let WireReferenceRoot::NamedConstant(cst) = &wr.root {
                    if cst.id == get_builtin_const!("assert") {
                        is_instance_used_map[assert_instr_id] = true;
                        wire_to_explore_queue.push(assert_instr_id);
                    }
                }
            }
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
    for (id, inst) in md.link_info.instructions.iter() {
        if !is_instance_used_map[id] {
            if let Instruction::Declaration(decl) = inst {
                errors.warn(decl.name_span, "Unused Variable: This variable does not affect the output ports of this module");
            }
        }
    }
}

fn make_fanins(
    instructions: &FlatAlloc<Instruction, FlatIDMarker>,
) -> FlatAlloc<Vec<FlatID>, FlatIDMarker> {
    // Setup Wire Fanouts List for faster processing
    let mut instruction_fanins: FlatAlloc<Vec<FlatID>, FlatIDMarker> =
        instructions.map(|_| Vec::new());

    for (inst_id, inst) in instructions.iter() {
        match inst {
            Instruction::SubModule(sm) => {
                sm.module_ref
                    .for_each_generative_input_in_template_args(&mut |id| {
                        instruction_fanins[inst_id].push(id);
                    });
            }
            Instruction::Declaration(decl) => {
                if let Some(lat_spec) = decl.latency_specifier {
                    instruction_fanins[inst_id].push(lat_spec);
                }
                decl.typ_expr.for_each_generative_input(&mut |id| {
                    instruction_fanins[inst_id].push(id);
                });
            }
            Instruction::Expression(expr) => {
                expr.source.for_each_dependency(&mut |id| {
                    instruction_fanins[inst_id].push(id);
                });
                match &expr.output {
                    ExpressionOutput::MultiWrite(writes) => {
                        for wr in writes {
                            if let Some(flat_root) = wr.to.root.get_root_flat() {
                                instruction_fanins[flat_root].push(inst_id);
                                wr.to.for_each_input_wire_in_path(&mut |idx_wire| {
                                    instruction_fanins[flat_root].push(idx_wire)
                                });
                            }
                        }
                    }
                    ExpressionOutput::SubExpression(_) => {}
                }
            }
            Instruction::IfStatement(stm) => {
                for id in FlatIDRange::new(stm.then_block.0, stm.else_block.1) {
                    if let Instruction::Expression(Expression {
                        output: ExpressionOutput::MultiWrite(writes),
                        ..
                    }) = &instructions[id]
                    {
                        for wr in writes {
                            if let Some(flat_root) = wr.to.root.get_root_flat() {
                                instruction_fanins[flat_root].push(stm.condition);
                            }
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
