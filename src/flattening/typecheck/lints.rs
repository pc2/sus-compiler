use std::collections::HashMap;
use std::collections::hash_map::Entry;

use sus_proc_macro::{get_builtin_const, get_builtin_type};

use crate::dev_aid::ariadne_interface::pretty_print_many_spans;
use crate::flattening::WriteModifiers;
use crate::linker::{GlobalRef, IsExtern, LinkerFiles};
use crate::prelude::*;
use crate::to_string::FmtWrapper;
use crate::typing::abstract_type::AbstractInnerType;
use crate::typing::template::TemplateKind;

use super::*;

use super::{Expression, ExpressionOutput, ExpressionSource, Instruction, WireReferenceRoot};

pub fn perform_lints(pass: &mut LinkerPass, errors: &ErrorCollector, linker_files: &LinkerFiles) {
    let (working_on, globals) = pass.get_with_context();
    let ctx = LintContext {
        working_on,
        errors,
        globals,
        linker_files,
    };
    ctx.extern_may_not_have_type_template_args();
    ctx.extern_must_declare_abs_lats();
    ctx.lint_instructions();
    ctx.find_unused_variables();
    ctx.no_duplicate_ports();
    ctx.check_unsynthesizeable_types();
    ctx.no_calling_local_actions();
}

struct LintContext<'l> {
    working_on: GlobalRef<'l>,
    linker_files: &'l LinkerFiles,
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
                        .info_obj(decl);
                }
            }
            WireReferenceRoot::LocalInterface(interface_decl_id) => {
                let _ = self.working_on.instructions[*interface_decl_id].unwrap_interface();
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
                        match refers_to.get() {
                            Some(PathElemRefersTo::Interface(_, Some(port))) => {
                                if let Some(InterfaceDeclKind::SinglePort(port_decl)) =
                                    submod.md.interfaces[*port].declaration_instruction
                                {
                                    let module_port_decl = submod.get_decl(port_decl);
                                    let_unwrap!(
                                        DeclarationKind::Port { direction, .. },
                                        module_port_decl.remote_decl.decl_kind
                                    );
                                    match (is_writing_to, direction) {
                                        (true, Direction::Input) | (false, Direction::Output) => {}
                                        (true, Direction::Output) => {
                                            self.errors
                                                .error(*name_span, "Cannot write to an output port")
                                                .info_obj(&module_port_decl);
                                        }
                                        (false, Direction::Input) => {
                                            self.errors
                                                .error(*name_span, "Cannot read from an input port")
                                                .info_obj(&module_port_decl);
                                        }
                                    }
                                }
                            }
                            Some(PathElemRefersTo::Interface(_, None)) | None => {}
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
        for pe in &wire_ref.path {
            match pe {
                WireReferencePathElement::FieldAccess { .. } => {}
                WireReferencePathElement::ArrayAccess { .. } => {}
                WireReferencePathElement::ArrayPartSelect {
                    from: _,
                    width,
                    bracket_span: _,
                    direction: _,
                } => {
                    let width = self.working_on.instructions[*width].unwrap_subexpression();
                    if width.domain != DomainType::Generative {
                        self.errors.error(
                            width.span,
                            "The width of a part-select cannot be non-generative",
                        );
                    }
                }
                WireReferencePathElement::ArraySlice {
                    from,
                    to,
                    bracket_span: _,
                } => {
                    for bound in from.iter().chain(to.iter()) {
                        let bound = self.working_on.instructions[*bound].unwrap_subexpression();
                        if bound.domain != DomainType::Generative {
                            self.errors
                                .error(bound.span, "A slice bound cannot be non-generative");
                        }
                    }
                }
            }
        }
    }
    fn cant_be_interface(&self, operation: &'static str, wire_ref: &WireReference) {
        match &wire_ref.output_typ.inner {
            AbstractInnerType::Interface(_, _) | AbstractInnerType::LocalInterface(_) => {
                self.errors.error(wire_ref.get_total_span(), format!("Can't {operation} an interface. Use a function call or interface connector instead"));
            }
            AbstractInnerType::Template(_)
            | AbstractInnerType::Named(_)
            | AbstractInnerType::Unknown(_) => {}
        }
    }
    fn lint_instructions(&self) {
        for (_, instr) in &self.working_on.instructions {
            match instr {
                Instruction::SubModule(_) => {}
                Instruction::Declaration(_) => {}
                Instruction::Expression(expr) => {
                    if let ExpressionSource::WireRef(wire_ref) = &expr.source {
                        self.lint_wire_ref(wire_ref, false);
                        // TODO: Now that function's func is also a plain Expression, we'd have to check if it's used in a func.
                        // self.cant_be_interface("read from", wire_ref);
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
                            err_ref.info_obj(decl);

                            let mut cur = *parent_condition;

                            while cur != decl.parent_condition {
                                match &self.working_on.instructions[cur.unwrap().parent_when] {
                                    Instruction::IfStatement(parent_when) => {
                                        err_ref.info(
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
                                        err_ref.info(interface_declaration.interface_kw_span, msg);

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
                        .info_obj(decl);
                }

                if !decl.decl_kind.is_state() {
                    self.errors
                        .error(
                            initial_kw_span,
                            "Initial values can only be given to state registers",
                        )
                        .info_obj(decl);
                }
            }
        }
    }

    fn extern_may_not_have_type_template_args(&self) {
        if self.working_on.is_extern != IsExtern::Extern {
            return;
        };

        for (_id, arg) in &self.working_on.parameters {
            if let TemplateKind::Type(_) = &arg.kind {
                self.errors.error(
                    arg.name_span,
                    "'extern' modules may not have 'type' arguments. Convert to bool[] first",
                );
            }
        }
    }

    fn extern_must_declare_abs_lats(&self) {
        let GlobalObj::Module(md) = self.working_on else {
            return;
        };
        match self.working_on.is_extern {
            IsExtern::Normal => return,
            IsExtern::Extern | IsExtern::Builtin => {}
        }

        for (_, p) in &md.ports {
            let instr = &md.link_info.instructions[p.declaration_instruction];
            if instr.get_latency_specifier().is_none() {
                self.errors.error(
                    instr.get_span(),
                    format!(
                        "In {} modules all ports must have a specified latency!",
                        self.working_on.is_extern
                    ),
                );
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
                    if port.direction == Direction::Output {
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

        if crate::debug::is_enabled("print-unused-vars-map") {
            eprintln!(
                "Find Unused Variables Fanins:\n{}",
                FmtWrapper(|f| {
                    for (to, fanins) in &instruction_fanins {
                        let is_target = if is_instance_used_map[to] {
                            " target"
                        } else {
                            ""
                        };
                        writeln!(f, "{to:?}{is_target} <- {:?}", fanins.as_slice())?;
                    }
                    Ok(())
                })
            );
            let spans = self
                .working_on
                .instructions
                .iter()
                .map(|(id, instr)| (instr.get_span(), format!("{id:?}")));
            pretty_print_many_spans(self.linker_files, spans);
        }

        // All asserts and declarations starting with '_' are also terminals
        for (instr_id, instr) in &self.working_on.instructions {
            match instr {
                Instruction::Expression(expr) => {
                    if let ExpressionSource::WireRef(wr) = &expr.source
                        && let WireReferenceRoot::NamedConstant(cst) = &wr.root
                        && cst.id == get_builtin_const!("assert")
                    {
                        is_instance_used_map[instr_id] = true;
                        wire_to_explore_queue.push(instr_id);
                    }
                }
                Instruction::Declaration(decl) => {
                    if decl.name.starts_with('_') {
                        is_instance_used_map[instr_id] = true;
                        wire_to_explore_queue.push(instr_id);
                    }
                }
                _ => {}
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
            if !is_instance_used_map[id]
                && let Instruction::Declaration(decl) = inst
            {
                self.errors.warn(decl.name_span, "Unused Variable: This variable does not affect the output ports of this module");
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
                Instruction::Interface(stm) => {
                    if let Some(lat_spec) = stm.latency_specifier {
                        instruction_fanins[instr_id].push(lat_spec);
                    }
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
                Instruction::Expression(expr) => {
                    expr.source.for_each_dependency(&mut |id| {
                        instruction_fanins[instr_id].push(id);
                    });
                    // Function arguments feed into the submodule of the function
                    if let ExpressionSource::FuncCall(fc) = &expr.source {
                        let wr_expr =
                            self.working_on.instructions[fc.func_wire_ref].unwrap_expression();
                        if let ExpressionSource::WireRef(fc_wr) = &wr_expr.source {
                            match &fc_wr.root {
                                WireReferenceRoot::LocalSubmodule(fc_target)
                                | WireReferenceRoot::LocalInterface(fc_target) => {
                                    instruction_fanins[*fc_target].push(instr_id);
                                }
                                WireReferenceRoot::LocalDecl(_)
                                | WireReferenceRoot::NamedConstant(_)
                                | WireReferenceRoot::NamedModule(_)
                                | WireReferenceRoot::Error => {}
                            }
                        }
                    }
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
                        instruction_fanins[id].push(stm.condition);
                    }
                }
                Instruction::ForStatement(stm) => {
                    instruction_fanins[stm.loop_var_decl].push(stm.start);
                    instruction_fanins[stm.loop_var_decl].push(stm.end);
                    for id in stm.loop_body {
                        instruction_fanins[id].push(stm.loop_var_decl);
                    }
                }
            }
        }
        instruction_fanins
    }

    /// #94 - Ports and Interfaces must not have identical names
    fn no_duplicate_ports(&self) {
        let GlobalObj::Module(md) = &self.working_on else {
            return;
        };

        let mut seen_names = HashMap::new();

        for (name, span, kind) in md
            .ports
            .iter()
            .map(|(_, port)| (&port.name, port.name_span, "port"))
            .chain(md.interfaces.iter().filter_map(|(_, interf)| {
                match interf.declaration_instruction? {
                    InterfaceDeclKind::Interface(interf_id) => {
                        let_unwrap!(
                            Instruction::Interface(interface_declaration),
                            &md.link_info.instructions[interf_id]
                        );
                        match interface_declaration.interface_kind {
                            InterfaceKind::RegularInterface => {
                                Some((&interf.name, interf.name_span, "interface"))
                            }
                            InterfaceKind::Action(_) | InterfaceKind::Trigger(_) => None, // Covered by ports
                        }
                    }
                    InterfaceDeclKind::SinglePort(_) => None, // Covered by ports
                }
            }))
        {
            match seen_names.entry(name) {
                Entry::Occupied(occupied_entry) => {
                    let (existing_span, existing_kind) = occupied_entry.get();
                    self.errors
                        .error(span, format!("Duplicate {kind} '{name}' declaration"))
                        .info(
                            *existing_span,
                            format!("{existing_kind} '{name}' declared here"),
                        );
                }
                Entry::Vacant(vacant_entry) => {
                    vacant_entry.insert((span, kind));
                }
            }
        }
    }

    fn check_unsynthesizeable_types(&self) {
        for (_, instr) in &self.working_on.get_link_info().instructions {
            match instr {
                Instruction::Declaration(declaration) => {
                    if declaration.decl_kind.is_generative() {
                        continue;
                    }
                    let non_synthesizeable_typ_name =
                        if let AbstractInnerType::Named(global_ref) = &declaration.typ.inner {
                            match global_ref.id {
                                get_builtin_type!("string") => "string",
                                _ => continue,
                            }
                        } else {
                            continue;
                        };
                    self.errors.error(
                        declaration.decl_span,
                        format!("'{non_synthesizeable_typ_name}' cannot be non-generative."),
                    );
                }
                Instruction::Expression(_)
                | Instruction::SubModule(_)
                | Instruction::Interface(_)
                | Instruction::IfStatement(_)
                | Instruction::ForStatement(_) => {}
            }
        }
    }

    fn no_calling_local_actions(&self) {
        let GlobalObj::Module(md) = &self.working_on else {
            return;
        };

        for (_, instr) in &md.link_info.instructions {
            let Instruction::Expression(expr_instr) = instr else {
                continue;
            };
            expr_instr.span.debug();
            let ExpressionSource::FuncCall(fc) = &expr_instr.source else {
                continue;
            };
            let ExpressionSource::WireRef(fc_func_wireref) = &md.link_info.instructions
                [fc.func_wire_ref]
                .unwrap_expression()
                .source
            else {
                continue;
            };

            let WireReferenceRoot::LocalInterface(interf) = &fc_func_wireref.root else {
                continue;
            };

            let interface_instr = md.link_info.instructions[*interf].unwrap_interface();
            match interface_instr.interface_kind {
                InterfaceKind::RegularInterface | InterfaceKind::Action(_) => {
                    let kind_capitalized = match interface_instr.interface_kind {
                        InterfaceKind::RegularInterface => "Interface",
                        InterfaceKind::Action(_) => "Action",
                        InterfaceKind::Trigger(_) => "Trigger",
                    };
                    self.errors
                        .error(
                            fc_func_wireref.root_span,
                            format!("Cannot call local {}s", interface_instr.interface_kind),
                        )
                        .info(
                            interface_instr.name_span,
                            format!("{kind_capitalized} {} declared here", interface_instr.name),
                        );
                }
                InterfaceKind::Trigger(_) => {}
            }
        }
    }
}
