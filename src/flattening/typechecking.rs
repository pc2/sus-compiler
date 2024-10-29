use crate::alloc::ArenaAllocator;
use crate::prelude::*;
use crate::typing::type_inference::{FailedUnification, HindleyMilner};

use crate::debug::SpanDebugger;
use crate::linker::{
    GlobalResolver, Linkable, NameElem, AFTER_TYPECHECK_CP
};

use crate::typing::{
    abstract_type::{DomainType, TypeUnifier, BOOL_TYPE, INT_TYPE},
    template::TemplateArgKind,
};

use super::*;

pub fn typecheck_all_modules(linker: &mut Linker) {
    let module_uuids : Vec<ModuleUUID> = linker.modules.iter().map(|(id, _md)| id).collect();
    for module_uuid in module_uuids {
        let global_id = NameElem::Module(module_uuid);
        let errs_globals = GlobalResolver::take_errors_globals(linker, global_id);
        let globals = GlobalResolver::new(linker, global_id, errs_globals);

        let ctx_info_string = format!("Typechecking {}", &globals.obj_link_info.name);
        println!("{ctx_info_string}");
        let mut span_debugger = SpanDebugger::new(
            &ctx_info_string,
            &linker.files[globals.obj_link_info.file],
        );

        let working_on: &Module = &linker.modules[module_uuid];

        let mut context = TypeCheckingContext {
            globals : &globals,
            errors: &globals.errors,
            type_checker: TypeUnifier::new(
                &working_on.link_info.template_arguments,
                working_on.link_info.type_variable_alloc.clone()
            ),
            runtime_condition_stack: Vec::new(),
            working_on,
        };

        context.typecheck();

        let type_checker = context.type_checker;
        let errs_and_globals = globals.decommission(&linker.files);

        // Grab another mutable copy of md so it doesn't force a borrow conflict
        let working_on_mut = &mut linker.modules[module_uuid];
        apply_types(type_checker, working_on_mut, &errs_and_globals.0, &linker.types);

        working_on_mut.link_info.reabsorb_errors_globals(errs_and_globals, AFTER_TYPECHECK_CP);
        
        span_debugger.defuse();
    }
}

struct ConditionStackElem {
    ends_at: FlatID,
    span: Span,
    domain: DomainType,
}

struct TypeCheckingContext<'l, 'errs> {
    globals: &'l GlobalResolver<'l>,
    errors: &'errs ErrorCollector<'l>,
    working_on: &'l Module,
    type_checker: TypeUnifier,
    runtime_condition_stack: Vec<ConditionStackElem>,
}

impl<'l, 'errs> TypeCheckingContext<'l, 'errs> {
    fn get_link_info<ID: Into<NameElem>>(&self, id: ID) -> Option<&LinkInfo> {
        let ne: NameElem = id.into();
        match ne {
            NameElem::Module(md_id) => Some(&self.globals[md_id].link_info),
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
        let module = &self.globals[submodule_id];
        let decl = module.get_port_decl(port);
        (decl, module.link_info.file)
    }

    fn get_type_of_port(&self, port: PortID, submodule_instr: FlatID) -> FullType {
        let (decl, _file) = self.get_decl_of_module_port(port, submodule_instr);
        let submodule_inst = self.working_on.instructions[submodule_instr].unwrap_submodule();
        let submodule_module = &self.globals[submodule_inst.module_ref.id];
        let port_interface = submodule_module.ports[port].domain;
        let port_local_domain = submodule_inst.local_interface_domains.get().unwrap()[port_interface];
        FullType {
            typ: decl
                .typ_expr
                .to_type_with_substitute(&submodule_inst.module_ref.template_args),
            domain: port_local_domain.clone(),
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
                let linker_cst = &self.globals[*cst];
                linker_cst.get_span_file()
            }
            WireReferenceRoot::SubModulePort(port) => {
                let (decl, file) = self.get_decl_of_module_port(port.port, port.submodule_decl);
                Some((decl.decl_span, file))
            }
        }
    }

    /// Wire references are used in two contexts: 
    /// - Reading from a wire
    /// - Writing to a wire
    /// 
    /// The AbstractTypes just get unified
    /// 
    /// But the domains behave differently. 
    /// - Reading: 
    ///     The domains combine to form the lowest common denominator. 
    ///     If all are generative this becomes generative
    ///     At least one non-generative domain makes the whole thing non-generative
    ///     It should be supplied with a generative output_typ domain when generative, and an unknown domain variable otherwise
    /// - Writing: 
    ///     The output_typ domain should be generative when wire_ref.root is generative, or a generative value is required such as with "initial"
    ///     When wire_ref.root is not generative, it should be an unknown domain variable
    fn typecheck_wire_reference(&self, wire_ref: &WireReference, whole_span: Span, output_typ: &FullType) {
        let root_type = match &wire_ref.root {
            WireReferenceRoot::LocalDecl(id, _) => {
                let decl_root = self.working_on.instructions[*id].unwrap_wire_declaration();
                decl_root.typ.clone()
            }
            WireReferenceRoot::NamedConstant(cst, _) => {
                let linker_cst = &self.globals[*cst];
                linker_cst.get_full_type()
            }
            WireReferenceRoot::SubModulePort(port) => {
                self.get_type_of_port(port.port, port.submodule_decl)
            }
        };
        self.type_checker.unify_domains(&root_type.domain, &output_typ.domain, whole_span, "wire reference root with root type");
        
        let mut current_type_in_progress = root_type.typ;
        for p in &wire_ref.path {
            match p {
                &WireReferencePathElement::ArrayAccess { idx, bracket_span } => {
                    let idx_wire = self.working_on.instructions[idx].unwrap_wire();

                    let new_resulting_variable = AbstractType::Unknown(self.type_checker.alloc_typ_variable());
                    let arr_span = bracket_span.outer_span();
                    self.type_checker.typecheck_array_access(
                        &current_type_in_progress,
                        &idx_wire.typ.typ,
                        arr_span,
                        idx_wire.span,
                        &new_resulting_variable
                    );

                    self.type_checker.unify_domains(&idx_wire.typ.domain, &output_typ.domain, idx_wire.span, "array access index");
                    current_type_in_progress = new_resulting_variable;
                }
            }
        }

        self.type_checker.type_substitutor.unify_report_error(&current_type_in_progress, &output_typ.typ, whole_span, "variable reference");
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
                // TODO this is wrong? 
                if decl.identifier_type.is_generative() {
                    decl.declaration_runtime_depth.set(self.runtime_condition_stack.len()).unwrap();
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

                                let found_decl_depth = *to_decl.declaration_runtime_depth.get().unwrap();
                                if self.runtime_condition_stack.len() > found_decl_depth
                                {
                                    let err_ref = self.errors.error(conn.to_span, "Cannot write to generative variables in runtime conditional block");
                                    err_ref.info_obj_different_file(decl, file);
                                    for elem in &self.runtime_condition_stack
                                        [found_decl_depth..]
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
                if !condition_wire.typ.domain.is_generative() {
                    self.runtime_condition_stack.push(ConditionStackElem {
                        ends_at: if_stmt.else_end,
                        span: condition_wire.span,
                        domain: condition_wire.typ.domain.clone(),
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
        let target_instructions = &self.globals[md_id].instructions;

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
                    let val_wire = self.working_on.instructions[*val].unwrap_wire();
                    let target_abstract_type = template_input_decl
                        .typ_expr
                        .to_type_with_substitute(&global_ref.template_args);
                    self.type_checker.typecheck_and_generative::<true>(
                        &val_wire.typ,
                        &target_abstract_type,
                        val_wire.span,
                        "generative template argument",
                        &self.errors
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
                    &INT_TYPE,
                    idx_wire.span,
                    "array size",
                    &self.errors
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
            self.type_checker.unify_domains(ref_domain, &condition_domain.0, span, "condition join");
        }
    }

    fn typecheck_visit_instruction(&mut self, instr_id: FlatID) {
        match &self.working_on.instructions[instr_id] {
            Instruction::SubModule(sm) => {
                self.typecheck_template_global(&sm.module_ref);
                let md = &self.globals[sm.module_ref.id];
                let local_interface_domains = md
                    .domain_names
                    .map(|_| DomainType::DomainVariable(self.type_checker.alloc_domain_variable()));

                sm.local_interface_domains.set(local_interface_domains).unwrap();
            }
            Instruction::Declaration(decl) => {
                if let Some(latency_spec) = decl.latency_specifier {
                    let latency_spec_wire =
                        self.working_on.instructions[latency_spec].unwrap_wire();
                    self.type_checker.typecheck_and_generative::<true>(
                        &latency_spec_wire.typ,
                        &INT_TYPE,
                        latency_spec_wire.span,
                        "latency specifier",
                        &self.errors
                    );
                }
                
                // Unify with the type written in the source code
                self.type_checker.unify_with_written_type(&self.working_on.instructions, &decl.typ_expr, &decl.typ.typ, &self.errors);
            }
            Instruction::IfStatement(stm) => {
                let wire = &self.working_on.instructions[stm.condition].unwrap_wire();
                self.type_checker.typecheck_and_generative::<false>(
                    &wire.typ,
                    &BOOL_TYPE,
                    wire.span,
                    "if statement condition",
                    &self.errors
                );
            }
            Instruction::ForStatement(stm) => {
                let loop_var = self.working_on.instructions[stm.loop_var_decl].unwrap_wire_declaration();
                let start = self.working_on.instructions[stm.start].unwrap_wire();
                let end = self.working_on.instructions[stm.end].unwrap_wire();

                self.type_checker.typecheck_and_generative::<true>(
                    &start.typ,
                    &loop_var.typ.typ,
                    start.span,
                    "for loop start",
                    &self.errors
                );
                self.type_checker.typecheck_and_generative::<true>(
                    &end.typ,
                    &loop_var.typ.typ,
                    end.span,
                    "for loop end",
                    &self.errors
                );
            }
            Instruction::Wire(w) => {
                match &w.source {
                    WireSource::WireRef(from_wire) => {
                        self.typecheck_wire_reference(from_wire, w.span, &w.typ);
                    }
                    &WireSource::UnaryOp { op, right } => {
                        let right_wire = self.working_on.instructions[right].unwrap_wire();
                        self.type_checker.typecheck_unary_operator(
                            op,
                            &right_wire.typ,
                            &w.typ,
                            right_wire.span
                        );
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
                            &w.typ
                        )
                    }
                    WireSource::Constant(value) => self.type_checker.unify_with_constant(&w.typ.typ, value, w.span),
                };
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
                        &write_to_type,
                        from_wire.span,
                        "function argument"
                    );
                }
            }
            Instruction::Write(conn) => {
                // Typecheck the value with target type
                let from_wire = self.working_on.instructions[conn.from].unwrap_wire();

                // Typecheck digging down into write side
                self.typecheck_wire_reference(&conn.to, conn.to_span, &conn.to_type);
                self.join_with_condition(&conn.to_type.domain, conn.to_span.debug());

                let declared_here = self.get_wire_ref_declaration_point(&conn.to.root);

                let write_context = match conn.write_modifiers {
                    WriteModifiers::Connection {..} => "connection",
                    WriteModifiers::Initial { initial_kw_span: _ } => "initial value"
                };


                from_wire.span.debug();
                self.type_checker.typecheck_write_to(
                    &from_wire.typ,
                    &conn.to_type,
                    from_wire.span,
                    write_context,
                );
            }
        }
    }

    fn get_current_condition_domain(&self) -> Option<(DomainType, Span)> {
        let last = self.runtime_condition_stack.last()?;
        Some((last.domain, last.span))
    }

    /// Should be followed up by a [apply_types] call to actually apply all the checked types. 
    fn typecheck(&mut self) {
        for elem_id in self.working_on.instructions.id_range() {
            self.control_flow_visit_instruction(elem_id);
            self.typecheck_visit_instruction(elem_id);
        }
    }
}


// ====== Free functions for actually applying the result of type checking ======

pub fn apply_types(
    mut type_checker: TypeUnifier,
    working_on: &mut Module,
    errors: &ErrorCollector,
    types: &ArenaAllocator<StructType, TypeUUIDMarker>
) {
    // Set the remaining domain variables that aren't associated with a module port. 
    // We just find domain IDs that haven't been 
    let mut leftover_domain_alloc = UUIDAllocator::new_start_from(working_on.domain_names.get_next_alloc_id());
    for d in type_checker.domain_substitutor.iter() {
        if d.get().is_none() {
            assert!(d.set(DomainType::Physical(leftover_domain_alloc.alloc())).is_ok());
        }
    }

    // Assign names to all of the domains in this module
    working_on.domains = leftover_domain_alloc.into_range().map(|id| DomainInfo {
        name: {
            if let Some(name) = working_on.domain_names.get(id) {
                name.clone()
            } else {
                format!("domain_{}", id.get_hidden_value())
            }
        }
    });

    // Post type application. Solidify types and flag any remaining AbstractType::Unknown
    for (_id, inst) in working_on.instructions.iter_mut() {
        match inst {
            Instruction::Wire(w) => type_checker.finalize_type(types, &mut w.typ, w.span, errors),
            Instruction::Declaration(decl) => type_checker.finalize_type(types, &mut decl.typ, decl.name_span, errors),
            Instruction::Write(Write { to_type, to_span, .. }) => type_checker.finalize_type(types, to_type, *to_span, errors),
            // TODO Submodule domains may not be crossed either? 
            Instruction::SubModule(sm) => {
                for (_domain_id_in_submodule, domain_assigned_to_it_here) in sm.local_interface_domains.get_mut().unwrap() {
                    use self::HindleyMilner;
                    domain_assigned_to_it_here.fully_substitute(&type_checker.domain_substitutor).unwrap();
                }
            }
            _other => {}
        }
    }

    for FailedUnification{mut found, mut expected, span, context} in type_checker.type_substitutor.extract_errors() {
        // Not being able to fully substitute is not an issue. We just display partial types
        let _ = found.fully_substitute(&type_checker.type_substitutor);
        let _ = expected.fully_substitute(&type_checker.type_substitutor);

        let expected_name = expected.to_string(types, &type_checker.template_type_names);
        let found_name = found.to_string(types, &type_checker.template_type_names);
        errors.error(span, format!("Typing Error: {context} expects a {expected_name} but was given a {found_name}"));

        assert!(
            expected_name != found_name,
            "{expected_name} != {found_name}"
        );
    }
    for FailedUnification{mut found, mut expected, span, context} in type_checker.domain_substitutor.extract_errors() {
        found.fully_substitute(&type_checker.domain_substitutor).unwrap();
        expected.fully_substitute(&type_checker.domain_substitutor).unwrap();

        let expected_name = format!("{expected:?}");
        let found_name = format!("{found:?}");
        errors.error(span, format!("Domain error: Attempting to combine domains {found_name} and {expected_name} in {context}"));

        assert!(
            expected_name != found_name,
            "{expected_name} != {found_name}"
        );
    }
}
