use sus_proc_macro::get_builtin_const;

use crate::flattening::WriteModifiers;
use crate::linker::{GlobalRef, IsExtern};
use crate::prelude::*;
use crate::typing::abstract_type::AbstractInnerType;
use crate::typing::template::TemplateKind;

use super::*;

use super::{Expression, ExpressionOutput, ExpressionSource, Instruction, WireReferenceRoot};

pub fn perform_lints(pass: &mut LinkerPass, errors: &ErrorCollector) {
    let (working_on, globals) = pass.get_with_context();
    let ctx = LintContext {
        working_on,
        errors,
        globals,
    };
    ctx.extern_objects_may_not_have_type_template_args();
    ctx.lint_instructions();
    ctx.find_unused_variables();
}

struct LintContext<'l> {
    working_on: GlobalRef<'l>,
    errors: &'l ErrorCollector<'l>,
    globals: GlobalResolver<'l, 'l>,
}

impl LintContext<'_> {
    fn lint_wire_ref(&self, wire_ref: &WireReference, is_writing_to: bool) {
        match &wire_ref.root {
            WireReferenceRoot::LocalDecl(decl_id) => {
                let decl = self.working_on.instructions[*decl_id].unwrap_declaration();
                if is_writing_to && decl.decl_kind.is_read_only() {
                    self.errors
                        .error(wire_ref.root_span, format!("'{}' is read-only", decl.name))
                        .info_obj_same_file(decl);
                }
            }
            WireReferenceRoot::LocalSubmodule(submod_decl_id) => {
                let submod = self.globals.get_declared_submodule(
                    self.working_on.instructions[*submod_decl_id].unwrap_submodule(),
                );
                for p in &wire_ref.path {
                    if let WireReferencePathElement::FieldAccess {
                        name_span,
                        refers_to,
                        ..
                    } = p
                    {
                        if let Some(PathElemRefersTo::Interface(port)) = refers_to.get() {
                            if let Some(InterfaceDeclKind::SinglePort(port_decl)) =
                                submod.md.interfaces[*port].declaration_instruction
                            {
                                let module_port_decl = submod.get_decl(port_decl);
                                let_unwrap!(
                                    DeclarationKind::Port { is_input, .. },
                                    module_port_decl.remote_decl.decl_kind
                                );
                                match (is_writing_to, is_input) {
                                    (true, true) | (false, false) => {}
                                    (true, false) => {
                                        self.errors
                                            .error(*name_span, "Cannot write to an output port")
                                            .info_obj(&module_port_decl);
                                    }
                                    (false, true) => {
                                        self.errors
                                            .error(*name_span, "Cannot read from an input port")
                                            .info_obj(&module_port_decl);
                                    }
                                }
                            }
                        }
                    }
                }
            }
            WireReferenceRoot::NamedConstant(cst) => {
                if is_writing_to {
                    self.errors
                        .error(cst.name_span, "Cannot write to a global constant!")
                        .info_obj(&self.globals.get_constant(cst.id).link_info);
                }
            }
            WireReferenceRoot::NamedModule(_global_md) => {
                if let Some(first_path_elem) = wire_ref.path.first() {
                    self.errors.error(first_path_elem.get_span(), "Cannot perform any accesses on an inline declared module. Declare it on a separate line!");
                }
            }
            WireReferenceRoot::Error => {}
        }
    }
    fn cant_be_interface(&self, operation: &'static str, wire_ref: &WireReference) {
        if let AbstractInnerType::Interface(_, _) = &wire_ref.output_typ.inner {
            self.errors.error(wire_ref.get_total_span(), format!("Can't {operation} an interface. Use a function call or interface connector instead"));
        }
    }
    fn lint_instructions(&self) {
        for (_, instr) in &self.working_on.instructions {
            match instr {
                Instruction::SubModule(_) => {}
                Instruction::Declaration(_) => {}
                Instruction::Expression(expr) => {
                    match &expr.source {
                        ExpressionSource::WireRef(wire_ref) => {
                            self.lint_wire_ref(wire_ref, false);
                            self.cant_be_interface("read from", wire_ref);
                        }
                        ExpressionSource::FuncCall(func_call) => {
                            self.lint_wire_ref(&func_call.func, false);
                        }
                        _ => {}
                    }
                    match &expr.output {
                        ExpressionOutput::MultiWrite(writes) => {
                            for wr in writes {
                                self.lint_wire_ref(&wr.to, true);
                                self.cant_be_interface("write to", &wr.to);
                                if let WireReferenceRoot::LocalDecl(decl_id) = &wr.to.root {
                                    let decl =
                                        self.working_on.instructions[*decl_id].unwrap_declaration();
                                    self.lint_write(&expr.parent_condition, wr, decl);
                                }
                            }
                        }
                        ExpressionOutput::SubExpression(_) => {}
                    }
                }
                Instruction::IfStatement(_)
                | Instruction::ForStatement(_)
                | Instruction::Interface(_) => {}
            }
        }
    }

    fn lint_write(
        &self,
        parent_condition: &Option<ParentCondition>,
        wr: &WriteTo,
        decl: &Declaration,
    ) {
        match wr.write_modifiers {
            WriteModifiers::Connection { .. } => {
                if decl.decl_kind.is_generative() {
                    // Check that this generative declaration isn't used in a non-compiletime if
                    if let Some(root_flat) = wr.to.root.get_root_flat() {
                        let to_decl = self.working_on.instructions[root_flat].unwrap_declaration();

                        if *parent_condition != to_decl.parent_condition {
                            let mut err_ref = self.errors.error(wr.to_span, "Cannot write to compiletime variable through runtime 'when' blocks");
                            err_ref = err_ref.info_obj_same_file(decl);

                            let mut cur = *parent_condition;

                            while cur != decl.parent_condition {
                                match &self.working_on.instructions[cur.unwrap().parent_when] {
                                    Instruction::IfStatement(parent_when) => {
                                        err_ref = err_ref.info_same_file(
                                            parent_when.if_keyword_span,
                                            "Assignment passes through this 'when'",
                                        );

                                        cur = parent_when.parent_condition;
                                    }
                                    Instruction::Interface(interface_declaration) => {
                                        let msg = match interface_declaration.interface_kind {
                                            InterfaceKind::RegularInterface => unreachable!(),
                                            InterfaceKind::Action(_) => {
                                                "Assignment passes through this 'action'"
                                            }
                                            InterfaceKind::Trigger(_) => {
                                                "Assignment passes through this 'trigger'"
                                            }
                                        };
                                        err_ref = err_ref.info_same_file(
                                            interface_declaration.interface_kw_span,
                                            msg,
                                        );

                                        cur = interface_declaration.parent_condition;
                                    }
                                    _ => unreachable!(),
                                }
                            }
                        }
                    }
                }
            }
            WriteModifiers::Initial { initial_kw_span } => {
                if decl.decl_kind.is_generative() {
                    self.errors
                        .error(
                            initial_kw_span,
                            "'initial' cannot be used with generative variables! Just assign a generative value as normal",
                        )
                        .info_obj_same_file(decl);
                }

                if !decl.decl_kind.is_state() {
                    self.errors
                        .error(
                            initial_kw_span,
                            "Initial values can only be given to state registers",
                        )
                        .info_obj_same_file(decl);
                }
            }
        }
    }

    fn extern_objects_may_not_have_type_template_args(&self) {
        if self.working_on.is_extern == IsExtern::Extern {
            for (_id, arg) in &self.working_on.template_parameters {
                if let TemplateKind::Type(_) = &arg.kind {
                    self.errors.error(
                        arg.name_span,
                        "'extern' modules may not have 'type' arguments. Convert to bool[] first",
                    );
                }
            }
        }
    }

    fn find_unused_variables(&self) {
        match self.working_on.is_extern {
            IsExtern::Normal => {}
            IsExtern::Extern | IsExtern::Builtin => return, // Don't report unused variables for extern modules.
        }

        let instruction_fanins = self.make_fanins();

        let mut is_instance_used_map: FlatAlloc<bool, FlatIDMarker> =
            self.working_on.instructions.map(|_| false);

        let mut wire_to_explore_queue: Vec<FlatID> = Vec::new();

        match self.working_on {
            GlobalObj::Module(md) => {
                // Output ports
                for (_id, port) in &md.ports {
                    if !port.is_input {
                        is_instance_used_map[port.declaration_instruction] = true;
                        wire_to_explore_queue.push(port.declaration_instruction);
                    }
                }
            }
            GlobalObj::Type(typ) => {
                for (_, field) in &typ.fields {
                    is_instance_used_map[field.declaration_instruction] = true;
                    wire_to_explore_queue.push(field.declaration_instruction);
                }
            }
            GlobalObj::Constant(cst) => {
                is_instance_used_map[cst.output_decl] = true;
                wire_to_explore_queue.push(cst.output_decl);
            }
        }

        // All asserts are also terminals
        for (assert_instr_id, instr) in &self.working_on.instructions {
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
        for (id, inst) in self.working_on.instructions.iter() {
            if !is_instance_used_map[id] {
                if let Instruction::Declaration(decl) = inst {
                    self.errors.warn(decl.name_span, "Unused Variable: This variable does not affect the output ports of this module");
                }
            }
        }
    }
    fn make_fanins(&self) -> FlatAlloc<Vec<FlatID>, FlatIDMarker> {
        // Setup Wire Fanouts List for faster processing
        let mut instruction_fanins: FlatAlloc<Vec<FlatID>, FlatIDMarker> =
            self.working_on.instructions.map(|_| Vec::new());

        for (instr_id, inst) in self.working_on.instructions.iter() {
            match inst {
                Instruction::SubModule(sm) => {
                    sm.module_ref.for_each_generative_input(&mut |id| {
                        instruction_fanins[instr_id].push(id);
                    });
                }
                Instruction::Declaration(decl) => {
                    if let Some(lat_spec) = decl.latency_specifier {
                        instruction_fanins[instr_id].push(lat_spec);
                    }
                    decl.typ_expr.for_each_generative_input(&mut |id| {
                        instruction_fanins[instr_id].push(id);
                    });
                }
                Instruction::Expression(expr) => {
                    expr.source.for_each_dependency(&mut |id| {
                        instruction_fanins[instr_id].push(id);
                    });
                    match &expr.output {
                        ExpressionOutput::MultiWrite(writes) => {
                            for wr in writes {
                                if let Some(flat_root) = wr.to.root.get_root_flat() {
                                    instruction_fanins[flat_root].push(instr_id);
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
                        }) = &self.working_on.instructions[id]
                        {
                            for wr in writes {
                                if let Some(flat_root) = wr.to.root.get_root_flat() {
                                    instruction_fanins[flat_root].push(stm.condition);
                                }
                            }
                        }
                    }
                }
                Instruction::Interface(stm) => {
                    for id in FlatIDRange::new(stm.then_block.0, stm.else_block.1) {
                        if let Instruction::Expression(Expression {
                            output: ExpressionOutput::MultiWrite(writes),
                            ..
                        }) = &self.working_on.instructions[id]
                        {
                            for wr in writes {
                                if let Some(flat_root) = wr.to.root.get_root_flat() {
                                    instruction_fanins[flat_root].push(instr_id);
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
}
