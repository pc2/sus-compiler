use crate::alloc::{zip_eq, ArenaAllocator};
use crate::errors::{ErrorInfo, ErrorInfoObject, FileKnowingErrorInfoObject};
use crate::prelude::*;
use crate::typing::template::{ParameterKind, TemplateArg};
use crate::typing::type_inference::{FailedUnification, Substitutor};

use crate::debug::SpanDebugger;
use crate::linker::{GlobalResolver, GlobalUUID, AFTER_TYPECHECK_CP};

use crate::typing::written_type::WrittenType;
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
        let rank = self
            .type_checker
            .abstract_type_substitutor
            .rank_substitutor
            .alloc_unknown();

        self.type_checker
            .abstract_type_substitutor
            .rank_substitutor
            .unify_must_succeed(&rank, &submodule_inst.rank);

        let typ = self
            .type_checker
            .abstract_type_substitutor
            .written_to_abstract_type_around_rank_substitute_templates(
                &decl.typ_expr,
                rank,
                &submodule_inst.module_ref.template_arg_types,
            );

        FullType {
            typ,
            domain: port_local_domain,
        }
    }

    fn get_wire_ref_info(&self, wire_ref_root: &WireReferenceRoot) -> Option<ErrorInfo> {
        Some(match wire_ref_root {
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
            WireReferenceRoot::Error => {
                return None;
            }
        })
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
        self.join_with_condition(&output_typ.domain, whole_span);
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
                let typ = self
                    .type_checker
                    .abstract_type_substitutor
                    .written_to_abstract_type_substitute_templates(
                        &decl.typ_expr,
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
            WireReferenceRoot::Error => return,
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
                    let idx_expr = self.working_on.instructions[idx].unwrap_subexpression();

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
                &output_typ.typ,
                &current_type_in_progress,
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
            Instruction::Declaration(decl) => {
                // For both runtime, and compiletime declarations.
                decl.declaration_runtime_depth
                    .set(self.runtime_condition_stack.len())
                    .unwrap();
            }
            Instruction::Expression(Expression {
                output: ExpressionOutput::SubExpression(_),
                ..
            }) => {}
            Instruction::Expression(Expression {
                output: ExpressionOutput::MultiWrite(writes),
                ..
            }) => {
                for wr in writes {
                    let (decl, file) = match &wr.to.root {
                        WireReferenceRoot::LocalDecl(decl_id, _) => {
                            let decl = self.working_on.instructions[*decl_id].unwrap_declaration();
                            if decl.read_only {
                                self.errors
                                    .error(wr.to_span, format!("'{}' is read-only", decl.name))
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
                                    .error(wr.to_span, "Cannot assign to a submodule output port")
                                    .info_obj_different_file(
                                        module_port_decl.0,
                                        module_port_decl.1,
                                    );
                            }

                            module_port_decl
                        }
                        WireReferenceRoot::Error => {
                            return;
                        }
                    };

                    match wr.write_modifiers {
                        WriteModifiers::Connection { .. } => {
                            if decl.identifier_type.is_generative() {
                                // Check that this generative declaration isn't used in a non-compiletime if
                                if let Some(root_flat) = wr.to.root.get_root_flat() {
                                    let to_decl = self.working_on.instructions[root_flat]
                                        .unwrap_declaration();

                                    let found_decl_depth =
                                        *to_decl.declaration_runtime_depth.get().unwrap();
                                    if self.runtime_condition_stack.len() > found_decl_depth {
                                        let err_ref = self.errors.error(wr.to_span, "Cannot write to generative variables in runtime conditional block");
                                        err_ref.info_obj_different_file(decl, file);
                                        for elem in
                                            &self.runtime_condition_stack[found_decl_depth..]
                                        {
                                            err_ref
                                                .info((elem.span, file), "Runtime condition here");
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
            }
            Instruction::IfStatement(if_stmt) => {
                let condition_expr =
                    self.working_on.instructions[if_stmt.condition].unwrap_subexpression();
                if !if_stmt.is_generative {
                    self.runtime_condition_stack.push(ConditionStackElem {
                        ends_at: if_stmt.else_block.1,
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

        for (_, argument_type, given_arg) in
            zip_eq(&global_ref.template_arg_types, &global_ref.template_args)
        {
            if let Some(TemplateArg {
                kind: TemplateArgKind::Type(wr_typ),
                ..
            }) = given_arg
            {
                self.typecheck_written_type(wr_typ);
                // This slot will not have been filled out yet
                let specified_arg_type = self
                    .type_checker
                    .abstract_type_substitutor
                    .written_to_abstract_type(wr_typ);
                self.type_checker
                    .abstract_type_substitutor
                    .unify_must_succeed(argument_type, &specified_arg_type);
            }
        }

        for (_parameter_id, argument_type, parameter) in zip_eq(
            &global_ref.template_arg_types,
            &target_link_info.template_parameters,
        ) {
            match &parameter.kind {
                ParameterKind::Type(_) => {} // Do nothing, nothing to unify with. Maybe in the future traits?
                ParameterKind::Generative(parameter) => {
                    let decl = target_link_info.instructions[parameter.declaration_instruction]
                        .unwrap_declaration();

                    let param_required_typ = self
                        .type_checker
                        .abstract_type_substitutor
                        .written_to_abstract_type_substitute_templates(
                            &decl.typ_expr,
                            &global_ref.template_arg_types, // Yes that's right. We already must substitute the templates for type variables here
                        );

                    self.type_checker
                        .abstract_type_substitutor
                        .unify_must_succeed(argument_type, &param_required_typ);
                }
            }
        }

        for (_, argument_type, given_arg) in
            zip_eq(&global_ref.template_arg_types, &global_ref.template_args)
        {
            if let Some(TemplateArg {
                kind: TemplateArgKind::Value(val),
                ..
            }) = given_arg
            {
                let argument_expr = self.working_on.instructions[*val].unwrap_subexpression();

                self.type_checker.typecheck_write_to_abstract(
                    &argument_expr.typ.typ,
                    argument_type,
                    argument_expr.span,
                    "generative template argument",
                );
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

                let idx_expr = self.working_on.instructions[*arr_idx].unwrap_subexpression();
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

    fn typecheck_visit_latency_specifier(&mut self, lat_spec: Option<FlatID>) {
        if let Some(latency_spec) = lat_spec {
            let latency_specifier_expr =
                self.working_on.instructions[latency_spec].unwrap_subexpression();
            self.type_checker.typecheck_write_to_abstract(
                &latency_specifier_expr.typ.typ,
                &INT_TYPE.scalar(),
                latency_specifier_expr.span,
                "latency specifier",
            );
        }
    }

    fn typecheck_visit_write_to(
        &mut self,
        write_to: &WriteTo,
        from_typ: &FullType,
        from_span: Span,
    ) {
        let write_context = match write_to.write_modifiers {
            WriteModifiers::Connection { .. } => "connection",
            WriteModifiers::Initial { initial_kw_span: _ } => "initial value",
        };
        let declared_here = self.get_wire_ref_info(&write_to.to.root);
        self.type_checker
            .typecheck_write_to(from_typ, &write_to.to_type, from_span, || {
                (
                    write_context.to_string(),
                    declared_here.into_iter().collect(),
                )
            });
    }

    fn get_interface_reference(
        &self,
        interface_reference: &ModuleInterfaceReference,
    ) -> (&'l Module, &'l Interface) {
        let submodule =
            self.working_on.instructions[interface_reference.submodule_decl].unwrap_submodule();
        let md = &self.globals[submodule.module_ref.id];
        let interface = &md.interfaces[interface_reference.submodule_interface];
        (md, interface)
    }

    fn report_errors_for_bad_function_call(
        &self,
        func_call: &FuncCall,
        whole_func_span: Span,
        mut to_spans_iter: impl ExactSizeIterator<Item = Span>,
    ) {
        let (md, interface) = self.get_interface_reference(&func_call.interface_reference);

        let arg_count = func_call.arguments.len();
        let expected_arg_count = interface.func_call_inputs.len();

        if arg_count != expected_arg_count {
            if arg_count > expected_arg_count {
                // Too many args, complain about excess args at the end
                let excess_args_span = Span::new_overarching(
                    self.working_on.instructions[func_call.arguments[expected_arg_count]]
                        .unwrap_expression()
                        .span,
                    self.working_on.instructions[*func_call.arguments.last().unwrap()]
                        .unwrap_expression()
                        .span,
                );

                self.errors
                    .error(excess_args_span, format!("Excess argument. Function takes {expected_arg_count} args, but {arg_count} were passed."))
                    .info_obj(&(md, interface));
            } else {
                // Too few args, mention missing argument names
                self.errors
                    .error(func_call.arguments_span.close_bracket(), format!("Too few arguments. Function takes {expected_arg_count} args, but {arg_count} were passed."))
                    .info_obj(&(md, interface));
            }
        }

        let num_func_outputs = interface.func_call_outputs.len();
        let num_targets = to_spans_iter.size_hint().0;
        if num_targets != num_func_outputs {
            if num_targets > num_func_outputs {
                let start_span: Span = to_spans_iter.nth(num_func_outputs).unwrap();
                let mut end_span = start_span;
                if let Some(end) = to_spans_iter.last() {
                    end_span = end;
                }

                let excess_results_span = Span::new_overarching(start_span, end_span);
                self.errors
                    .error(excess_results_span, format!("Excess output targets. Function returns {num_func_outputs} results, but {num_targets} targets were given."))
                    .info_obj(&(md, interface));
            } else {
                self.errors
                    .error(whole_func_span, format!("Too few output targets. Function returns {num_func_outputs} results, but {num_targets} targets were given."))
                    .info_obj(&(md, interface));
            }
        }
    }

    fn typecheck_func_call(&mut self, func_call: &FuncCall) -> PortIDRange {
        let (md, interface) = self.get_interface_reference(&func_call.interface_reference);

        for (port, arg) in std::iter::zip(interface.func_call_inputs, &func_call.arguments) {
            let port_type =
                self.get_type_of_port(port, func_call.interface_reference.submodule_decl);

            let decl = md.get_port_decl(port);

            // Typecheck the value with target type
            let from = self.working_on.instructions[*arg].unwrap_subexpression();

            self.join_with_condition(&port_type.domain, from.span);
            self.type_checker
                .typecheck_write_to(from.typ, &port_type, from.span, || {
                    (
                        "function argument".to_string(),
                        vec![decl.make_info(md.link_info.file).unwrap()],
                    )
                });
        }

        interface.func_call_outputs
    }

    fn typecheck_single_output_expr(&mut self, expr: SingleOutputExpression) {
        match expr.source {
            ExpressionSource::WireRef(from_wire) => {
                self.typecheck_wire_reference(from_wire, expr.span, expr.typ);
            }
            ExpressionSource::UnaryOp { op, rank, right } => {
                let right_expr = self.working_on.instructions[*right].unwrap_subexpression();
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
                let left_expr = self.working_on.instructions[*left].unwrap_subexpression();
                let right_expr = self.working_on.instructions[*right].unwrap_subexpression();
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
            ExpressionSource::FuncCall(func_call) => {
                let func_call_outputs = self.typecheck_func_call(func_call);

                self.report_errors_for_bad_function_call(
                    func_call,
                    expr.span,
                    std::iter::once(expr.span),
                );

                if let Some(first_output) = func_call_outputs.first() {
                    let port_type = self.get_type_of_port(
                        first_output,
                        func_call.interface_reference.submodule_decl,
                    );

                    self.type_checker.typecheck_write_to(
                        &port_type,
                        expr.typ,
                        expr.span,
                        "function call as expression",
                    );
                }
            }
            ExpressionSource::Constant(value) => {
                self.type_checker
                    .unify_with_constant(&expr.typ.typ, value, expr.span)
            }
            ExpressionSource::ArrayConstruct(arr) => {
                for elem_id in arr {
                    let elem_expr = self.working_on.instructions[*elem_id].unwrap_subexpression();

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
    fn typecheck_multi_output_expr(&mut self, expr: &Expression, multi_write: &[WriteTo]) {
        match &expr.source {
            ExpressionSource::FuncCall(func_call) => {
                let func_call_outputs = self.typecheck_func_call(func_call);

                self.report_errors_for_bad_function_call(
                    func_call,
                    expr.span,
                    multi_write.iter().map(|v| v.to_span),
                );

                for (port, to) in std::iter::zip(func_call_outputs, multi_write) {
                    let port_type =
                        self.get_type_of_port(port, func_call.interface_reference.submodule_decl);

                    self.typecheck_visit_write_to(to, &port_type, expr.span);
                }
            }
            ExpressionSource::WireRef(..)
            | ExpressionSource::UnaryOp { .. }
            | ExpressionSource::BinaryOp { .. }
            | ExpressionSource::ArrayConstruct(..)
            | ExpressionSource::Constant(..) => {
                if let Some(single_write) = multi_write.first() {
                    self.typecheck_single_output_expr(SingleOutputExpression {
                        typ: &single_write.to_type,
                        span: expr.span,
                        source: &expr.source,
                    });
                }

                // Don't output errors for 0 outputs. See no errors on zero outputs (#79)
                if multi_write.len() > 1 {
                    self.errors.error(
                        expr.span,
                        format!(
                            "Non-function assignments must output exactly 1 output instead of {}",
                            multi_write.len()
                        ),
                    );
                }
            }
        }
        if let ExpressionSource::WireRef(wire_ref) = &expr.source {
            if let Some(first_write) = multi_write.first() {
                self.typecheck_wire_reference(wire_ref, expr.span, &first_write.to_type);
            } else {
                let sentinel = FullType {
                    typ: self.type_checker.abstract_type_substitutor.alloc_unknown(),
                    domain: expr.domain,
                };
                self.typecheck_wire_reference(wire_ref, expr.span, &sentinel);
            }
        }
        for wr in multi_write {
            self.typecheck_wire_reference(&wr.to, wr.to_span, &wr.to_type);
        }
    }

    fn typecheck_visit_instruction(&mut self, instr_id: FlatID) {
        match &self.working_on.instructions[instr_id] {
            Instruction::SubModule(sm) => {
                self.typecheck_template_global(&sm.module_ref);
            }
            Instruction::Declaration(decl) => {
                self.typecheck_visit_latency_specifier(decl.latency_specifier);

                self.typecheck_written_type(&decl.typ_expr);
            }
            Instruction::IfStatement(stm) => {
                let condition_expr =
                    &self.working_on.instructions[stm.condition].unwrap_subexpression();
                self.type_checker.typecheck_write_to_abstract(
                    &condition_expr.typ.typ,
                    &BOOL_TYPE.scalar(),
                    condition_expr.span,
                    "if statement condition",
                );
            }
            Instruction::ForStatement(stm) => {
                let loop_var = self.working_on.instructions[stm.loop_var_decl].unwrap_declaration();
                let start = self.working_on.instructions[stm.start].unwrap_subexpression();
                let end = self.working_on.instructions[stm.end].unwrap_subexpression();

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
            Instruction::Expression(expr) => match &expr.output {
                ExpressionOutput::SubExpression(typ) => {
                    self.typecheck_single_output_expr(SingleOutputExpression {
                        typ,
                        span: expr.span,
                        source: &expr.source,
                    });
                }
                ExpressionOutput::MultiWrite(write_tos) => {
                    self.typecheck_multi_output_expr(expr, write_tos);
                }
            },
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
                type_checker.finalize_domain_type(&mut expr.domain);
                match &mut expr.output {
                    ExpressionOutput::SubExpression(expr_typ) => {
                        type_checker.finalize_type(linker_types, expr_typ, expr.span, errors);
                    }
                    ExpressionOutput::MultiWrite(write_tos) => {
                        for wr in write_tos {
                            type_checker.finalize_type(
                                linker_types,
                                &mut wr.to_type,
                                wr.to_span,
                                errors,
                            );
                            type_checker.finalize_wire_ref(linker_types, &mut wr.to, errors);
                        }
                    }
                }
                match &mut expr.source {
                    ExpressionSource::WireRef(wr) => {
                        type_checker.finalize_wire_ref(linker_types, wr, errors);
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
            // TODO Submodule domains may not be crossed either?
            Instruction::SubModule(sm) => {
                for (_domain_id_in_submodule, domain_assigned_to_it_here) in
                    &mut sm.local_interface_domains
                {
                    type_checker.finalize_domain_type(domain_assigned_to_it_here);
                }
                type_checker.finalize_global_ref(linker_types, &mut sm.module_ref, errors);
                let span = sm.get_most_relevant_span();
                type_checker.finalize_peano_type(&mut sm.rank, span, errors);
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
