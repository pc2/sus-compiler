use crate::alloc::{zip_eq3, ArenaAllocator};
use crate::errors::{ErrorInfo, ErrorInfoObject, FileKnowingErrorInfoObject};
use crate::prelude::*;
use crate::typing::template::ParameterKind;
use crate::typing::type_inference::{FailedUnification, Substitutor};

use crate::debug::SpanDebugger;
use crate::linker::{GlobalResolver, GlobalUUID, AFTER_TYPECHECK_CP};

use crate::typing::{
    abstract_type::{DomainType, FullTypeUnifier, BOOL_TYPE, INT_TYPE},
    template::TemplateArgKind,
};

use super::*;

pub fn typecheck_all_modules(linker: &mut Linker) {
    let module_uuids: Vec<ModuleUUID> = linker.modules.iter().map(|(id, _md)| id).collect();
    for module_uuid in module_uuids {
        let working_on_mut = &mut linker.modules[module_uuid];
        let errs_globals = working_on_mut.link_info.take_errors_globals();
        let type_alloc = *working_on_mut.link_info.type_variable_alloc.take().unwrap();

        let working_on: &Module = &linker.modules[module_uuid];
        let globals = GlobalResolver::new(linker, &working_on.link_info, errs_globals);

        println!("Typechecking {}", &working_on.link_info.name);
        let _panic_guard = SpanDebugger::new(
            "Typechecking",
            &working_on.link_info.name,
            &linker.files[working_on.link_info.file],
        );

        let mut context = TypeCheckingContext {
            globals: &globals,
            errors: &globals.errors,
            type_checker: FullTypeUnifier::new(
                &working_on.link_info.template_parameters,
                type_alloc,
            ),
            runtime_condition_stack: Vec::new(),
            working_on: &working_on.link_info,
        };

        context.typecheck();

        let type_checker = context.type_checker;
        let errs_and_globals = globals.decommission(&linker.files);

        // Grab another mutable copy of md so it doesn't force a borrow conflict
        let working_on_mut = &mut linker.modules[module_uuid];
        apply_types(
            type_checker,
            working_on_mut,
            &errs_and_globals.0,
            &linker.types,
        );

        working_on_mut
            .link_info
            .reabsorb_errors_globals(errs_and_globals, AFTER_TYPECHECK_CP);

        // Also create the inference info now.
        working_on_mut.latency_inference_info = PortLatencyInferenceInfo::make(
            &working_on_mut.ports,
            &working_on_mut.link_info.instructions,
            working_on_mut.link_info.template_parameters.len(),
        );

        if crate::debug::is_enabled("print-flattened") {
            working_on_mut.print_flattened_module(&linker.files[working_on_mut.link_info.file]);
        }
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
    working_on: &'l LinkInfo,
    type_checker: FullTypeUnifier,
    runtime_condition_stack: Vec<ConditionStackElem>,
}

impl<'l> TypeCheckingContext<'l, '_> {
    fn get_decl_of_module_port(
        &self,
        port: PortID,
        submodule_instr: FlatID,
    ) -> (&'l Declaration, FileUUID) {
        let submodule_id = self.working_on.instructions[submodule_instr]
            .unwrap_submodule()
            .module_ref
            .id;
        let module = &self.globals[submodule_id];
        let decl = module.get_port_decl(port);
        (decl, module.link_info.file)
    }

    fn get_type_of_port(&mut self, port: PortID, submodule_instr: FlatID) -> FullType {
        let submodule_inst = self.working_on.instructions[submodule_instr].unwrap_submodule();
        let submodule_module = &self.globals[submodule_inst.module_ref.id];
        let decl = submodule_module.get_port_decl(port);
        let port_interface = submodule_module.ports[port].domain;
        let port_local_domain = submodule_inst.local_interface_domains[port_interface];
        let typ = self.type_checker.abstract_type_substitutor.alloc_unknown();
        self.type_checker
            .unify_with_written_type_substitute_templates_must_succeed(
                &decl.typ_expr,
                &typ,
                &submodule_inst.module_ref.template_arg_types,
            );
        FullType {
            typ,
            domain: port_local_domain,
        }
    }

    fn get_wire_ref_info(&self, wire_ref_root: &WireReferenceRoot) -> ErrorInfo {
        match wire_ref_root {
            WireReferenceRoot::LocalDecl(id, _) => {
                let decl_root = self.working_on.instructions[*id].unwrap_declaration();
                decl_root.make_info(self.errors.file).unwrap()
            }
            WireReferenceRoot::NamedConstant(cst) => {
                let linker_cst = &self.globals[cst.id];
                linker_cst.link_info.make_global_info(self.errors.files)
            }
            WireReferenceRoot::SubModulePort(port) => {
                let (decl, file) = self.get_decl_of_module_port(port.port, port.submodule_decl);
                decl.make_info(file).unwrap()
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
    fn typecheck_wire_reference(
        &mut self,
        wire_ref: &WireReference,
        whole_span: Span,
        output_typ: &FullType,
    ) {
        let root_type = match &wire_ref.root {
            WireReferenceRoot::LocalDecl(id, _) => {
                let decl_root = self.working_on.instructions[*id].unwrap_declaration();
                decl_root.typ.clone()
            }
            WireReferenceRoot::NamedConstant(cst) => {
                self.typecheck_template_global(cst);

                let linker_cst = &self.globals[cst.id];
                let decl =
                    linker_cst.link_info.instructions[linker_cst.output_decl].unwrap_declaration();
                let typ = self.type_checker.abstract_type_substitutor.alloc_unknown();
                self.type_checker
                    .unify_with_written_type_substitute_templates_must_succeed(
                        &decl.typ_expr,
                        &typ,
                        &cst.template_arg_types,
                    );
                FullType {
                    typ,
                    domain: DomainType::Generative,
                }
            }
            WireReferenceRoot::SubModulePort(port) => {
                self.get_type_of_port(port.port, port.submodule_decl)
            }
        };
        self.type_checker.unify_domains(
            &root_type.domain,
            &output_typ.domain,
            whole_span,
            "wire reference root with root type",
        );

        let mut current_type_in_progress = root_type.typ;
        for p in &wire_ref.path {
            match p {
                &WireReferencePathElement::ArrayAccess { idx, bracket_span } => {
                    let idx_expr = self.working_on.instructions[idx].unwrap_expression();

                    let new_resulting_variable =
                        self.type_checker.abstract_type_substitutor.alloc_unknown();
                    let arr_span = bracket_span.outer_span();
                    {
                        self.type_checker
                            .abstract_type_substitutor
                            .unify_report_error(
                                &idx_expr.typ.typ,
                                &INT_TYPE.scalar(),
                                idx_expr.span,
                                "array index",
                            );
                        self.type_checker.unify_with_array_of(
                            &current_type_in_progress,
                            new_resulting_variable.clone(),
                            arr_span,
                        );
                    };

                    self.type_checker.unify_domains(
                        &idx_expr.typ.domain,
                        &output_typ.domain,
                        idx_expr.span,
                        "array access index",
                    );
                    current_type_in_progress = new_resulting_variable;
                }
            }
        }

        self.type_checker
            .abstract_type_substitutor
            .unify_report_error(
                &current_type_in_progress,
                &output_typ.typ,
                whole_span,
                "variable reference",
            );
    }

    fn control_flow_visit_instruction(&mut self, inst_id: FlatID) {
        while let Some(parent_block) = self.runtime_condition_stack.last() {
            if parent_block.ends_at != inst_id {
                break;
            }
            self.runtime_condition_stack.pop().unwrap();
        }
        match &self.working_on.instructions[inst_id] {
            Instruction::SubModule(sm) => {
                self.typecheck_template_global(&sm.module_ref);
            }
            Instruction::FuncCall(_) => {}
            Instruction::Declaration(decl) => {
                // For both runtime, and compiletime declarations.
                decl.declaration_runtime_depth
                    .set(self.runtime_condition_stack.len())
                    .unwrap();
            }
            Instruction::Expression(_) => {}
            Instruction::Write(conn) => {
                let (decl, file) = match &conn.to.root {
                    WireReferenceRoot::LocalDecl(decl_id, _) => {
                        let decl = self.working_on.instructions[*decl_id].unwrap_declaration();
                        if decl.read_only {
                            self.errors
                                .error(conn.to_span, format!("'{}' is read-only", decl.name))
                                .info_obj_same_file(decl);
                        }
                        (decl, self.errors.file)
                    }
                    WireReferenceRoot::NamedConstant(cst) => {
                        self.errors
                            .error(cst.name_span, "Cannot assign to a global");
                        return;
                    }
                    WireReferenceRoot::SubModulePort(port) => {
                        let module_port_decl =
                            self.get_decl_of_module_port(port.port, port.submodule_decl);

                        if !module_port_decl.0.decl_kind.is_io_port().unwrap() {
                            self.errors
                                .error(conn.to_span, "Cannot assign to a submodule output port")
                                .info_obj_different_file(module_port_decl.0, module_port_decl.1);
                        }

                        module_port_decl
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
                                let to_decl =
                                    self.working_on.instructions[root_flat].unwrap_declaration();

                                let found_decl_depth =
                                    *to_decl.declaration_runtime_depth.get().unwrap();
                                if self.runtime_condition_stack.len() > found_decl_depth {
                                    let err_ref = self.errors.error(conn.to_span, "Cannot write to generative variables in runtime conditional block");
                                    err_ref.info_obj_different_file(decl, file);
                                    for elem in &self.runtime_condition_stack[found_decl_depth..] {
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
                let condition_expr =
                    self.working_on.instructions[if_stmt.condition].unwrap_expression();
                if !condition_expr.typ.domain.is_generative() {
                    self.runtime_condition_stack.push(ConditionStackElem {
                        ends_at: if_stmt.else_end,
                        span: condition_expr.span,
                        domain: condition_expr.typ.domain,
                    });
                }
            }
            Instruction::ForStatement(_) => {}
        }
    }

    fn typecheck_template_global<ID: Copy + Into<GlobalUUID>>(
        &mut self,
        global_ref: &GlobalReference<ID>,
    ) {
        let global_obj: GlobalUUID = global_ref.id.into();
        let target_link_info = self.globals.get_link_info(global_obj);

        for (_parameter_id, argument_type, parameter, given_template_arg) in zip_eq3(
            &global_ref.template_arg_types,
            &target_link_info.template_parameters,
            &global_ref.template_args,
        ) {
            match &parameter.kind {
                ParameterKind::Type(_) => {} // Do nothing, nothing to unify with. Maybe in the future traits?
                ParameterKind::Generative(parameter) => {
                    let decl = target_link_info.instructions[parameter.declaration_instruction]
                        .unwrap_declaration();

                    self.type_checker
                        .unify_with_written_type_substitute_templates_must_succeed(
                            &decl.typ_expr,
                            argument_type,
                            &global_ref.template_arg_types, // Yes that's right. We already must substitute the templates for type variables here
                        );
                }
            }

            if let Some(given_arg) = given_template_arg {
                match &given_arg.kind {
                    TemplateArgKind::Type(wr_typ) => {
                        self.typecheck_written_type(wr_typ);
                        // This slot will not have been filled out yet
                        self.type_checker
                            .unify_with_written_type_must_succeed(wr_typ, argument_type);
                    }
                    TemplateArgKind::Value(val) => {
                        let argument_expr = self.working_on.instructions[*val].unwrap_expression();

                        self.type_checker.typecheck_write_to_abstract(
                            &argument_expr.typ.typ,
                            argument_type,
                            argument_expr.span,
                            "generative template argument",
                        );
                    }
                }
            }
        }
    }

    /// Critically, this is different from [TypeUnifier::unify_with_written_type].
    /// That one unifies a given typ with the written type, without checking the written type.
    ///
    /// This function checks the written type itself.
    fn typecheck_written_type(&mut self, wr_typ: &WrittenType) {
        match wr_typ {
            WrittenType::Error(_) => {}
            WrittenType::TemplateVariable(_, _) => {}
            WrittenType::Named(global_ref) => {
                self.typecheck_template_global(global_ref);
            }
            WrittenType::Array(_, arr_box) => {
                let (content_typ, arr_idx, _bracket_span) = arr_box.deref();

                self.typecheck_written_type(content_typ);

                let idx_expr = self.working_on.instructions[*arr_idx].unwrap_expression();
                self.type_checker.typecheck_write_to_abstract(
                    &idx_expr.typ.typ,
                    &INT_TYPE.scalar(),
                    idx_expr.span,
                    "array size",
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
    fn join_with_condition(&mut self, ref_domain: &DomainType, span: Span) {
        if let Some(condition_domain) = self.get_current_condition_domain() {
            self.type_checker.unify_domains(
                ref_domain,
                &condition_domain.0,
                span,
                "condition join",
            );
        }
    }

    fn typecheck_visit_instruction(&mut self, instr_id: FlatID) {
        match &self.working_on.instructions[instr_id] {
            Instruction::SubModule(sm) => {
                self.typecheck_template_global(&sm.module_ref);
            }
            Instruction::Declaration(decl) => {
                if let Some(latency_spec) = decl.latency_specifier {
                    let latency_specifier_expr =
                        self.working_on.instructions[latency_spec].unwrap_expression();
                    self.type_checker.typecheck_write_to_abstract(
                        &latency_specifier_expr.typ.typ,
                        &INT_TYPE.scalar(),
                        latency_specifier_expr.span,
                        "latency specifier",
                    );
                }

                self.typecheck_written_type(&decl.typ_expr);

                // Unify with the type written in the source code
                self.type_checker
                    .unify_with_written_type_must_succeed(&decl.typ_expr, &decl.typ.typ);
            }
            Instruction::IfStatement(stm) => {
                let condition_expr =
                    &self.working_on.instructions[stm.condition].unwrap_expression();
                self.type_checker.typecheck_write_to_abstract(
                    &condition_expr.typ.typ,
                    &BOOL_TYPE.scalar(),
                    condition_expr.span,
                    "if statement condition",
                );
            }
            Instruction::ForStatement(stm) => {
                let loop_var = self.working_on.instructions[stm.loop_var_decl].unwrap_declaration();
                let start = self.working_on.instructions[stm.start].unwrap_expression();
                let end = self.working_on.instructions[stm.end].unwrap_expression();

                self.type_checker.typecheck_write_to_abstract(
                    &start.typ.typ,
                    &loop_var.typ.typ,
                    start.span,
                    "for loop start",
                );
                self.type_checker.typecheck_write_to_abstract(
                    &end.typ.typ,
                    &loop_var.typ.typ,
                    end.span,
                    "for loop end",
                );
            }
            Instruction::Expression(expr) => {
                match &expr.source {
                    ExpressionSource::WireRef(from_wire) => {
                        self.typecheck_wire_reference(from_wire, expr.span, &expr.typ);
                    }
                    ExpressionSource::UnaryOp { op, rank, right } => {
                        let right_expr = self.working_on.instructions[*right].unwrap_expression();
                        self.type_checker.typecheck_unary_operator_abstr(
                            *op,
                            rank,
                            &right_expr.typ.typ,
                            right_expr.span,
                            &expr.typ.typ,
                        );
                        self.type_checker.unify_domains(
                            &right_expr.typ.domain,
                            &expr.typ.domain,
                            right_expr.span,
                            "unary op",
                        );
                    }
                    ExpressionSource::BinaryOp {
                        op,
                        rank,
                        left,
                        right,
                    } => {
                        let left_expr = self.working_on.instructions[*left].unwrap_expression();
                        let right_expr = self.working_on.instructions[*right].unwrap_expression();
                        {
                            self.type_checker.typecheck_binary_operator_abstr(
                                *op,
                                rank,
                                &left_expr.typ.typ,
                                &right_expr.typ.typ,
                                left_expr.span,
                                right_expr.span,
                                &expr.typ.typ,
                            );
                            self.type_checker.unify_domains(
                                &left_expr.typ.domain,
                                &expr.typ.domain,
                                left_expr.span,
                                "binop left",
                            );
                            self.type_checker.unify_domains(
                                &right_expr.typ.domain,
                                &expr.typ.domain,
                                right_expr.span,
                                "binop right",
                            );
                        }
                    }
                    ExpressionSource::Constant(value) => {
                        self.type_checker
                            .unify_with_constant(&expr.typ.typ, value, expr.span)
                    }
                    ExpressionSource::ArrayConstruct(arr) => {
                        for elem_id in arr {
                            let elem_expr =
                                self.working_on.instructions[*elem_id].unwrap_expression();

                            self.type_checker.unify_with_array_of(
                                &expr.typ.typ,
                                elem_expr.typ.typ.clone(),
                                elem_expr.span,
                            );
                            self.type_checker.unify_domains(
                                &elem_expr.typ.domain,
                                &expr.typ.domain,
                                elem_expr.span,
                                "Array construction",
                            );
                        }
                    }
                };
            }
            Instruction::FuncCall(fc) => {
                for (port, arg) in std::iter::zip(fc.func_call_inputs.into_iter(), &fc.arguments) {
                    let write_to_type =
                        self.get_type_of_port(port, fc.interface_reference.submodule_decl);

                    let (decl, file) =
                        self.get_decl_of_module_port(port, fc.interface_reference.submodule_decl);

                    // Typecheck the value with target type
                    let from_expr = self.working_on.instructions[*arg].unwrap_expression();

                    self.join_with_condition(&write_to_type.domain, from_expr.span.debug());
                    self.type_checker.typecheck_write_to(
                        &from_expr.typ,
                        &write_to_type,
                        from_expr.span,
                        || {
                            (
                                "function argument".to_string(),
                                vec![decl.make_info(file).unwrap()],
                            )
                        },
                    );
                }
            }
            Instruction::Write(conn) => {
                // Typecheck the value with target type
                let from_expr: &'l Expression =
                    self.working_on.instructions[conn.from].unwrap_expression();

                // Typecheck digging down into write side
                self.typecheck_wire_reference(&conn.to, conn.to_span, &conn.to_type);
                self.join_with_condition(&conn.to_type.domain, conn.to_span.debug());

                from_expr.span.debug();

                let write_context = match conn.write_modifiers {
                    WriteModifiers::Connection { .. } => "connection",
                    WriteModifiers::Initial { initial_kw_span: _ } => "initial value",
                };
                let declared_here = self.get_wire_ref_info(&conn.to.root);
                let pass_to_write_to = (write_context.to_string(), vec![declared_here]);

                self.type_checker.typecheck_write_to(
                    &from_expr.typ,
                    &conn.to_type,
                    from_expr.span,
                    || pass_to_write_to,
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
    mut type_checker: FullTypeUnifier,
    working_on: &mut Module,
    errors: &ErrorCollector,
    linker_types: &ArenaAllocator<StructType, TypeUUIDMarker>,
) {
    // Set the remaining domain variables that aren't associated with a module port.
    // We just find domain IDs that haven't been
    let mut leftover_domain_alloc =
        UUIDAllocator::new_start_from(working_on.domains.get_next_alloc_id());
    for (_, d) in type_checker.domain_substitutor.iter() {
        if d.get().is_none() {
            assert!(d
                .set(DomainType::Physical(leftover_domain_alloc.alloc()))
                .is_ok());
        }
    }

    // Assign names to all of the domains in this module
    working_on.domains = leftover_domain_alloc.as_range().map(|id| {
        if let Some(work_on_domain) = working_on.domains.get(id) {
            work_on_domain.clone()
        } else {
            DomainInfo {
                name: format!("domain_{}", id.get_hidden_value()),
                name_span: None,
            }
        }
    });

    // Post type application. Solidify types and flag any remaining AbstractType::Unknown
    for (_id, inst) in working_on.link_info.instructions.iter_mut() {
        match inst {
            Instruction::Expression(expr) => {
                type_checker.finalize_type(linker_types, &mut expr.typ, expr.span, errors);
                match &mut expr.source {
                    ExpressionSource::WireRef(wr) => {
                        if let WireReferenceRoot::NamedConstant(cst) = &mut wr.root {
                            type_checker.finalize_global_ref(linker_types, cst, errors);
                        }
                    }
                    ExpressionSource::UnaryOp { rank, .. }
                    | ExpressionSource::BinaryOp { rank, .. } => {
                        let _ = type_checker
                            .abstract_type_substitutor
                            .rank_substitutor
                            .fully_substitute(rank); // No need to report incomplete peano error, as one of the ports would have reported it
                    }
                    _ => {}
                }
            }
            Instruction::Declaration(decl) => {
                type_checker.finalize_type(linker_types, &mut decl.typ, decl.name_span, errors)
            }
            Instruction::Write(Write {
                to_type,
                to_span,
                to,
                ..
            }) => {
                type_checker.finalize_type(linker_types, to_type, *to_span, errors);
                if let WireReferenceRoot::NamedConstant(cst) = &mut to.root {
                    type_checker.finalize_global_ref(linker_types, cst, errors);
                }
            }
            // TODO Submodule domains may not be crossed either?
            Instruction::SubModule(sm) => {
                for (_domain_id_in_submodule, domain_assigned_to_it_here) in
                    &mut sm.local_interface_domains
                {
                    type_checker.finalize_domain_type(domain_assigned_to_it_here);
                }
                type_checker.finalize_global_ref(linker_types, &mut sm.module_ref, errors);
            }
            _other => {}
        }
    }

    // Print all errors
    for FailedUnification {
        mut found,
        mut expected,
        span,
        context,
        infos,
    } in type_checker.abstract_type_substitutor.extract_errors()
    {
        // Not being able to fully substitute is not an issue. We just display partial types
        let _ = type_checker
            .abstract_type_substitutor
            .fully_substitute(&mut found);
        let _ = type_checker
            .abstract_type_substitutor
            .fully_substitute(&mut expected);

        let expected_name = expected
            .display(linker_types, &type_checker.template_type_names)
            .to_string();
        let found_name = found
            .display(linker_types, &type_checker.template_type_names)
            .to_string();
        errors
            .error(span, format!("Typing Error: {context} expects a {expected_name} but was given a {found_name}"))
            .add_info_list(infos);

        assert!(
            expected_name != found_name,
            "{expected_name} != {found_name}"
        );
    }
    for FailedUnification {
        mut found,
        mut expected,
        span,
        context,
        infos,
    } in type_checker.domain_substitutor.extract_errors()
    {
        assert!(type_checker.domain_substitutor.fully_substitute(&mut found));
        assert!(type_checker
            .domain_substitutor
            .fully_substitute(&mut expected));

        let expected_name = format!("{expected:?}");
        let found_name = format!("{found:?}");
        errors
            .error(span, format!("Domain error: Attempting to combine domains {found_name} and {expected_name} in {context}"))
            .add_info_list(infos);

        assert!(
            expected_name != found_name,
            "{expected_name} != {found_name}"
        );
    }
}
