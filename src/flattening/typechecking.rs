use crate::prelude::*;

use std::ops::{Deref, DerefMut};

use walk::for_each_generative_input_in_template_args;

use crate::debug::SpanDebugger;
use crate::linker::{
    with_module_editing_context, Linkable, NameElem, NamedConstant, Resolver, WorkingOnResolver,
};

use crate::typing::{
    abstract_type::{BestName, DomainType, TypeUnifier, BOOL_TYPE, INT_TYPE},
    template::TemplateArgKind,
};

use super::*;

pub fn typecheck_all_modules(linker: &mut Linker) {
    let linker_ptr: *mut Linker = linker;
    for (module_uuid, module) in &mut linker.modules {
        let ctx_info_string = format!("Typechecking {}", &module.link_info.name);
        println!("{ctx_info_string}");
        let mut span_debugger = SpanDebugger::new(
            &ctx_info_string,
            &linker.files[module.link_info.file].file_text,
        );

        with_module_editing_context(
            linker_ptr,
            module_uuid,
            |modules, types, constants, name_resolver| {
                let mut context = TypeCheckingContext {
                    errors: name_resolver.errors,
                    type_checker: TypeUnifier::new(
                        types,
                        &modules.working_on.link_info.template_arguments,
                        name_resolver.errors,
                        &modules.working_on.domain_names,
                    ),
                    constants,
                    runtime_condition_stack: Vec::new(),
                    modules,
                };

                context.typecheck();
                context.find_unused_variables();
            },
        );

        span_debugger.defuse();
    }
}

struct ConditionStackElem {
    ends_at: FlatID,
    span: Span,
    domain: DomainID,
}

struct TypeCheckingContext<'l, 'errs> {
    modules: WorkingOnResolver<'l, 'errs, ModuleUUIDMarker, Module>,
    type_checker: TypeUnifier<'l, 'errs>,
    constants: Resolver<'l, 'errs, ConstantUUIDMarker, NamedConstant>,
    errors: &'errs ErrorCollector<'l>,
    runtime_condition_stack: Vec<ConditionStackElem>,
}

impl<'l, 'errs> Deref for TypeCheckingContext<'l, 'errs> {
    type Target = WorkingOnResolver<'l, 'errs, ModuleUUIDMarker, Module>;

    fn deref(&self) -> &Self::Target {
        &self.modules
    }
}
impl<'l, 'errs> DerefMut for TypeCheckingContext<'l, 'errs> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.modules
    }
}

impl<'l, 'errs> TypeCheckingContext<'l, 'errs> {
    fn get_link_info<ID: Into<NameElem>>(&self, id: ID) -> Option<&LinkInfo> {
        let ne: NameElem = id.into();
        match ne {
            NameElem::Module(md_id) => Some(&self.modules[md_id].link_info),
            NameElem::Type(_) | NameElem::Constant(_) => None, // TODO all globals should have link_info
        }
    }

    fn get_decl_of_module_port<'s>(
        &'s self,
        port: PortID,
        submodule_instr: FlatID,
    ) -> (&'s Declaration, FileUUID) {
        let submodule_id = self.working_on.instructions[submodule_instr]
            .unwrap_submodule()
            .module_ref
            .id;
        let module = &self.modules[submodule_id];
        let decl = module.get_port_decl(port);
        (decl, module.link_info.file)
    }

    fn get_type_of_port(&self, port: PortID, submodule_instr: FlatID) -> FullType {
        let (decl, _file) = self.get_decl_of_module_port(port, submodule_instr);
        let submodule_inst = self.working_on.instructions[submodule_instr].unwrap_submodule();
        let submodule_module = &self.modules[submodule_inst.module_ref.id];
        let port_interface = submodule_module.ports[port].domain;
        let port_local_domain = submodule_inst.local_interface_domains[port_interface];
        FullType {
            typ: decl
                .typ_expr
                .to_type_with_substitute(&submodule_inst.module_ref.template_args),
            domain: DomainType::Physical(port_local_domain),
        }
    }

    fn get_wire_ref_declaration_point(
        &self,
        wire_ref_root: &WireReferenceRoot,
    ) -> Option<SpanFile> {
        match wire_ref_root {
            WireReferenceRoot::LocalDecl(id, _) => {
                let decl_root = self.working_on.instructions[*id].unwrap_wire_declaration();
                Some((decl_root.decl_span, self.errors.file))
            }
            WireReferenceRoot::NamedConstant(cst, _) => {
                let linker_cst = &self.constants[*cst];
                linker_cst.get_span_file()
            }
            WireReferenceRoot::SubModulePort(port) => {
                let (decl, file) = self.get_decl_of_module_port(port.port, port.submodule_decl);
                Some((decl.decl_span, file))
            }
        }
    }

    fn get_type_of_wire_reference(&self, wire_ref: &WireReference) -> FullType {
        let mut write_to_type = match &wire_ref.root {
            WireReferenceRoot::LocalDecl(id, _) => {
                let decl_root = self.working_on.instructions[*id].unwrap_wire_declaration();
                decl_root.typ.clone()
            }
            WireReferenceRoot::NamedConstant(cst, _) => {
                let linker_cst = &self.constants[*cst];
                linker_cst.get_full_type()
            }
            WireReferenceRoot::SubModulePort(port) => {
                self.get_type_of_port(port.port, port.submodule_decl)
            }
        };

        for p in &wire_ref.path {
            match p {
                &WireReferencePathElement::ArrayAccess { idx, bracket_span } => {
                    let idx_wire = self.working_on.instructions[idx].unwrap_wire();

                    write_to_type = self.type_checker.typecheck_array_access(
                        &write_to_type,
                        bracket_span.outer_span(),
                        &idx_wire.typ,
                        idx_wire.span,
                    );
                }
            }
        }

        write_to_type
    }

    fn control_flow_visit_instruction(&mut self, inst_id: FlatID) {
        while let Some(parent_block) = self.runtime_condition_stack.last() {
            if parent_block.ends_at != inst_id {
                break;
            }
            self.runtime_condition_stack.pop().unwrap();
        }
        match &self.working_on.instructions[inst_id] {
            Instruction::SubModule(_) => {}
            Instruction::FuncCall(_) => {}
            Instruction::Declaration(decl) => {
                if decl.identifier_type.is_generative() {
                    assert!(decl.declaration_runtime_depth == DECL_DEPTH_LATER);
                    let Instruction::Declaration(decl) =
                        &mut self.modules.working_on.instructions[inst_id]
                    else {
                        unreachable!()
                    };
                    decl.declaration_runtime_depth = self.runtime_condition_stack.len()
                }
            }
            Instruction::Wire(_) => {}
            Instruction::Write(conn) => {
                let (decl, file) = match conn.to.root {
                    WireReferenceRoot::LocalDecl(decl_id, _) => {
                        let decl = self.working_on.instructions[decl_id].unwrap_wire_declaration();
                        if decl.read_only {
                            self.errors
                                .error(conn.to_span, format!("'{}' is read-only", decl.name))
                                .info_obj_same_file(decl);
                        }
                        (decl, self.errors.file)
                    }
                    WireReferenceRoot::NamedConstant(_, span) => {
                        self.errors.error(span, "Cannot assign to a global");
                        return;
                    }
                    WireReferenceRoot::SubModulePort(port) => {
                        let r = self.get_decl_of_module_port(port.port, port.submodule_decl);

                        if !r.0.is_port.as_regular_port().unwrap() {
                            self.errors
                                .error(conn.to_span, "Cannot assign to a submodule output port")
                                .info_obj_different_file(r.0, r.1);
                        }

                        r
                    }
                };

                match conn.write_modifiers {
                    WriteModifiers::Connection {
                        num_regs: _,
                        regs_span: _,
                    } => {
                        if decl.identifier_type.is_generative() {
                            // Check that this generative declaration isn't used in a non-compiletime if
                            if let Some(root_flat) = conn.to.root.get_root_flat() {
                                let to_decl = self.working_on.instructions[root_flat]
                                    .unwrap_wire_declaration();

                                if self.runtime_condition_stack.len()
                                    > to_decl.declaration_runtime_depth
                                {
                                    let err_ref = self.errors.error(conn.to_span, "Cannot write to generative variables in runtime conditional block");
                                    err_ref.info_obj_different_file(decl, file);
                                    for elem in &self.runtime_condition_stack
                                        [to_decl.declaration_runtime_depth..]
                                    {
                                        err_ref.info((elem.span, file), "Runtime condition here");
                                    }
                                }
                            }
                        }
                    }
                    WriteModifiers::Initial { initial_kw_span } => {
                        if decl.identifier_type != IdentifierType::State {
                            self.errors
                                .error(
                                    initial_kw_span,
                                    "Initial values can only be given to state registers",
                                )
                                .info_obj_different_file(decl, file);
                        }
                    }
                }
            }
            Instruction::IfStatement(if_stmt) => {
                let condition_wire = self.working_on.instructions[if_stmt.condition].unwrap_wire();
                if let DomainType::Physical(domain) = condition_wire.typ.domain {
                    self.runtime_condition_stack.push(ConditionStackElem {
                        ends_at: if_stmt.else_end,
                        span: condition_wire.span,
                        domain,
                    });
                }
            }
            Instruction::ForStatement(_) => {}
        }
    }

    fn typecheck_template_global<ID: Copy + Into<NameElem>>(
        &self,
        global_ref: &GlobalReference<ID>,
    ) {
        let Some(link_info) = self.get_link_info(global_ref.id) else {
            return;
        }; // TODO all objects should have link_info
        let ne: NameElem = global_ref.id.into();
        let NameElem::Module(md_id) = ne else {
            todo!("TODO Move Instructions to link_info too")
        };
        let target_instructions = &self.modules[md_id].instructions;

        for (template_id, value) in global_ref.template_args.iter_valids() {
            match &value.kind {
                TemplateArgKind::Type(typ) => {
                    self.typecheck_written_type(typ);
                }
                TemplateArgKind::Value(val) => {
                    let template_input = link_info.template_arguments[template_id]
                        .kind
                        .unwrap_value();
                    let template_input_decl = target_instructions
                        [template_input.declaration_instruction]
                        .unwrap_wire_declaration();
                    let declared_here = Some((template_input_decl.name_span, self.errors.file));
                    let val_wire = self.working_on.instructions[*val].unwrap_wire();
                    let target_abstract_type = template_input_decl
                        .typ_expr
                        .to_type_with_substitute(&global_ref.template_args);
                    self.type_checker.typecheck_and_generative::<true>(
                        &val_wire.typ,
                        val_wire.span,
                        &target_abstract_type,
                        "generative template argument",
                        declared_here,
                    );
                }
            }
        }
    }

    fn typecheck_written_type(&self, wr_typ: &WrittenType) {
        match wr_typ {
            WrittenType::Error(_) => {}
            WrittenType::Template(_, _) => {}
            WrittenType::Named(global_ref) => {
                self.typecheck_template_global(global_ref);
            }
            WrittenType::Array(_, arr_box) => {
                let (content_typ, arr_idx, _bracket_span) = arr_box.deref();

                self.typecheck_written_type(content_typ);

                let idx_wire = self.working_on.instructions[*arr_idx].unwrap_wire();
                self.type_checker.typecheck_and_generative::<true>(
                    &idx_wire.typ,
                    idx_wire.span,
                    &INT_TYPE,
                    "array size",
                    None,
                );
            }
        }
    }

    /// TODO: writes to declarations that are in same scope need not be checked as such.
    ///
    /// This allows to work with temporaries of a different domain within an if statement
    ///
    /// Which could allow for a little more encapsulation in certain circumstances
    ///
    /// Also, this meshes with the thing where we only add condition wires to writes that go
    /// outside of a condition block
    fn join_with_condition(&self, ref_domain: &DomainType, span: Span) {
        if let Some(condition_domain) = self.get_current_condition_domain() {
            // Just check that
            self.type_checker.combine_domains::<false, _>(ref_domain, &DomainType::Physical(condition_domain.0), |wire_ref_domain_name, condition_domain_name| {
                let wire_ref_domain_name = wire_ref_domain_name.unwrap();
                self.errors.error(span, format!("Attempting to write to a wire from domain '{wire_ref_domain_name}' within a condition in domain '{condition_domain_name}'"))
                    .info_same_file(condition_domain.1, format!("This condition has domain '{condition_domain_name}'"));
            });
        }
    }

    fn typecheck_visit_instruction(&mut self, instr_id: FlatID) {
        match &self.working_on.instructions[instr_id] {
            Instruction::SubModule(sm) => {
                self.typecheck_template_global(&sm.module_ref);
                let md = &self.modules[sm.module_ref.id];
                let local_interface_domains = md
                    .domain_names
                    .map(|_| self.type_checker.new_unknown_domain_id());

                let Instruction::SubModule(sm) = &mut self.working_on.instructions[instr_id] else {
                    unreachable!()
                };
                sm.local_interface_domains = local_interface_domains;
            }
            Instruction::Declaration(decl) => {
                if let Some(latency_spec) = decl.latency_specifier {
                    let latency_spec_wire =
                        self.working_on.instructions[latency_spec].unwrap_wire();
                    self.type_checker.typecheck_and_generative::<true>(
                        &latency_spec_wire.typ,
                        latency_spec_wire.span,
                        &INT_TYPE,
                        "latency specifier",
                        None,
                    );
                }

                self.typecheck_written_type(&decl.typ_expr);

                let typ = decl.typ_expr.to_type();
                let Instruction::Declaration(decl) =
                    &mut self.modules.working_on.instructions[instr_id]
                else {
                    unreachable!()
                };
                decl.typ.typ = typ;
                if decl.typ.domain == DomainType::Physical(DomainID::PLACEHOLDER) {
                    decl.typ.domain = self
                        .type_checker
                        .new_unknown_domain(decl.identifier_type.is_generative());
                }
            }
            Instruction::IfStatement(stm) => {
                let wire = &self.working_on.instructions[stm.condition].unwrap_wire();
                self.type_checker.typecheck_and_generative::<false>(
                    &wire.typ,
                    wire.span,
                    &BOOL_TYPE,
                    "if statement condition",
                    None,
                );
            }
            Instruction::ForStatement(stm) => {
                let loop_var =
                    &self.working_on.instructions[stm.loop_var_decl].unwrap_wire_declaration();
                let start = &self.working_on.instructions[stm.start].unwrap_wire();
                let end = &self.working_on.instructions[stm.end].unwrap_wire();

                self.type_checker.typecheck_and_generative::<true>(
                    &start.typ,
                    start.span,
                    &loop_var.typ.typ,
                    "for loop start",
                    None,
                );
                self.type_checker.typecheck_and_generative::<true>(
                    &end.typ,
                    end.span,
                    &loop_var.typ.typ,
                    "for loop end",
                    None,
                );
            }
            Instruction::Wire(w) => {
                let result_typ = match &w.source {
                    WireSource::WireRef(from_wire) => self.get_type_of_wire_reference(from_wire),
                    &WireSource::UnaryOp { op, right } => {
                        let right_wire = self.working_on.instructions[right].unwrap_wire();
                        self.type_checker.typecheck_unary_operator(
                            op,
                            &right_wire.typ,
                            right_wire.span,
                        )
                    }
                    &WireSource::BinaryOp { op, left, right } => {
                        let left_wire = self.working_on.instructions[left].unwrap_wire();
                        let right_wire = self.working_on.instructions[right].unwrap_wire();
                        self.type_checker.typecheck_binary_operator(
                            op,
                            &left_wire.typ,
                            &right_wire.typ,
                            left_wire.span,
                            right_wire.span,
                        )
                    }
                    WireSource::Constant(value) => value.get_type_of_constant(),
                };
                let Instruction::Wire(w) = &mut self.working_on.instructions[instr_id] else {
                    unreachable!()
                };
                w.typ = result_typ;
            }
            Instruction::FuncCall(fc) => {
                for (port, arg) in std::iter::zip(fc.func_call_inputs.into_iter(), &fc.arguments) {
                    let write_to_type =
                        self.get_type_of_port(port, fc.interface_reference.submodule_decl);

                    let (decl, file) =
                        self.get_decl_of_module_port(port, fc.interface_reference.submodule_decl);
                    let declared_here = (decl.decl_span, file);

                    // Typecheck the value with target type
                    let from_wire = self.working_on.instructions[*arg].unwrap_wire();

                    self.join_with_condition(&write_to_type.domain, from_wire.span.debug());
                    self.type_checker.typecheck_write_to(
                        &from_wire.typ,
                        from_wire.span,
                        &write_to_type,
                        "function argument",
                        Some(declared_here),
                    );
                }
            }
            Instruction::Write(conn) => {
                // Typecheck digging down into write side
                let mut write_to_type = self.get_type_of_wire_reference(&conn.to);

                self.join_with_condition(&write_to_type.domain, conn.to_span.debug());

                let declared_here = self.get_wire_ref_declaration_point(&conn.to.root);

                let write_context = match conn.write_modifiers {
                    WriteModifiers::Connection {
                        num_regs: _,
                        regs_span: _,
                    } => "connection",
                    WriteModifiers::Initial { initial_kw_span: _ } => {
                        write_to_type.domain = DomainType::Generative;
                        "initial value"
                    }
                };

                // Typecheck the value with target type
                let from_wire = self.working_on.instructions[conn.from].unwrap_wire();

                from_wire.span.debug();
                self.type_checker.typecheck_write_to(
                    &from_wire.typ,
                    from_wire.span,
                    &write_to_type,
                    write_context,
                    declared_here,
                );
            }
        }
    }

    fn get_current_condition_domain(&self) -> Option<(DomainID, Span)> {
        let last = self.runtime_condition_stack.last()?;
        Some((last.domain, last.span))
    }

    fn typecheck(&mut self) {
        for (_id, port) in &self.modules.working_on.ports {
            let Instruction::Declaration(decl) =
                &mut self.modules.working_on.instructions[port.declaration_instruction]
            else {
                unreachable!()
            };
            decl.typ.domain = DomainType::Physical(port.domain);
        }

        for elem_id in self.working_on.instructions.id_range() {
            self.control_flow_visit_instruction(elem_id);
            self.typecheck_visit_instruction(elem_id);
        }

        // Post type application. Solidify types and flag any remaining AbstractType::Unknown
        for (id, inst) in self.modules.working_on.instructions.iter_mut() {
            match inst {
                Instruction::Wire(w) => {
                    self.type_checker
                        .finalize_type(&mut w.typ, w.span, BestName::UnnamedWire);
                }
                Instruction::Declaration(decl) => self.type_checker.finalize_type(
                    &mut decl.typ,
                    decl.decl_span,
                    BestName::NamedWire(id),
                ),
                Instruction::SubModule(sm) => {
                    for (interface_id, i) in &mut sm.local_interface_domains {
                        *i = self
                            .type_checker
                            .finalize_domain(*i, BestName::SubModule(id, interface_id));
                    }
                }
                _other => {}
            }
        }

        self.modules.working_on.domains =
            self.type_checker
                .final_domains
                .map(|(id, best_name)| DomainInfo {
                    name: match *best_name {
                        BestName::NamedDomain => self.modules.working_on.domain_names[id].clone(),
                        BestName::SubModule(sm_instr, sm_domain) => {
                            let sm = self.working_on.instructions[sm_instr].unwrap_submodule();
                            sm.module_ref.span.debug();
                            let sm_md = &self.modules[sm.module_ref.id];
                            format!("{}_{}", sm.get_name(&sm_md), sm_md.domain_names[sm_domain])
                        }
                        BestName::NamedWire(decl_id) => self.working_on.instructions[decl_id]
                            .unwrap_wire_declaration()
                            .name
                            .clone(),
                        BestName::UnnamedWire => format!("domain_{}", id.get_hidden_value()),
                    },
                });
    }

    /*
        ==== Additional Warnings ====
    */
    fn find_unused_variables(&self) {
        let instruction_fanins = self.make_fanins();

        let mut is_instance_used_map: FlatAlloc<bool, FlatIDMarker> =
            self.working_on.instructions.map(|_| false);

        let mut wire_to_explore_queue: Vec<FlatID> = Vec::new();

        for (_id, port) in &self.working_on.ports {
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

        for (inst_id, inst) in self.working_on.instructions.iter() {
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
                        if let Instruction::Write(conn) = &self.working_on.instructions[id] {
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
}
