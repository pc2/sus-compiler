use std::ops::Deref;

use crate::alloc::{ArenaAllocator, UUID};
use crate::errors::ErrorInfo;
use crate::prelude::*;
use crate::typing::abstract_type::{AbstractInnerType, AbstractRankedType};
use crate::typing::template::{Parameter, TVec};
use crate::typing::type_inference::{
    AbstractTypeSubstitutor, FailedUnification, TypeUnifier, UnifyErrorReport,
};

use crate::linker::GlobalUUID;

use crate::typing::{
    abstract_type::{BOOL_TYPE, INT_TYPE},
    template::TemplateKind,
};

use super::*;

pub fn typecheck_all_modules(linker: &mut Linker) {
    let module_uuids: Vec<ModuleUUID> = linker.modules.iter().map(|(id, _md)| id).collect();
    for module_uuid in module_uuids {
        let type_checker = linker.immutable_pass(
            "Typechecking",
            GlobalUUID::Module(module_uuid),
            |working_on, errors, globals| {
                let mut context = TypeCheckingContext {
                    globals,
                    errors,
                    type_checker: TypeUnifier::from(AbstractTypeSubstitutor::default()),
                    instructions: &working_on.instructions,
                };

                context.typecheck();

                context.type_checker
            },
        );
        // Grab another mutable copy of md so it doesn't force a borrow conflict
        let working_on_mut = &mut linker.modules[module_uuid];
        let errors = ErrorCollector::from_storage(
            working_on_mut.link_info.errors.take(),
            working_on_mut.link_info.file,
            &linker.files,
        );
        let finalize_ctx = FinalizationContext {
            linker_types: &linker.types,
            errors: &errors,
            type_checker,
            template_names: &working_on_mut.link_info.template_parameters,
        };
        finalize_ctx.apply_types(&mut working_on_mut.link_info.instructions);
        assert!(working_on_mut.link_info.errors.is_untouched());
        working_on_mut.link_info.errors = errors.into_storage();
        if crate::debug::is_enabled("print-flattened") {
            working_on_mut.print_flattened_module(&linker.files[working_on_mut.link_info.file]);
        }
    }
}

impl<'l> RemotePort<'l> {
    fn get_local_type(&self, type_checker: &mut AbstractTypeSubstitutor) -> AbstractRankedType {
        type_checker.written_to_abstract_type_substitute_templates(
            &self.remote_decl.typ_expr,
            &self.parent.submodule.module_ref.template_arg_types,
        )
    }
}

impl<'l> TypeCheckingContext<'l> {
    fn typecheck_wire_reference(&mut self, wire_ref: &WireReference) {
        let root_typ = match &wire_ref.root {
            WireReferenceRoot::LocalDecl(decl_id) => {
                let decl = self.instructions[*decl_id].unwrap_declaration();
                decl.typ.clone()
            }
            WireReferenceRoot::NamedConstant(cst) => {
                self.typecheck_template_global(cst);

                let linker_cst = &self.globals[cst.id];
                let decl =
                    linker_cst.link_info.instructions[linker_cst.output_decl].unwrap_declaration();
                self.type_checker
                    .written_to_abstract_type_substitute_templates(
                        &decl.typ_expr,
                        &cst.template_arg_types,
                    )
            }
            WireReferenceRoot::SubModulePort(port) => {
                let submod_port =
                    RemoteSubModule::make(port.submodule_decl, self.instructions, self.globals)
                        .get_port(port.port);
                if submod_port.remote_decl.domain.get() == DomainType::Generative {
                    self.errors
                        .error(
                            wire_ref.root_span,
                            "Invalid Submodule port: It is marked as generative!",
                        )
                        .info_obj(&submod_port);
                }

                submod_port.get_local_type(&mut self.type_checker)
            }
            WireReferenceRoot::Error => self.type_checker.alloc_unknown(),
        };
        wire_ref.root_typ.set(root_typ);

        let mut walking_typ: &AbstractRankedType = &wire_ref.root_typ;
        for p in &wire_ref.path {
            match p {
                WireReferencePathElement::ArrayAccess {
                    idx,
                    bracket_span,
                    output_typ,
                } => {
                    let idx_expr = self.instructions[*idx].unwrap_subexpression();

                    self.type_checker.unify_report_error(
                        idx_expr.typ,
                        &INT_TYPE.scalar(),
                        idx_expr.span,
                        "array index",
                    );

                    output_typ.set(self.type_checker.rank_down(
                        walking_typ,
                        bracket_span.outer_span(),
                        "array access",
                    ));

                    walking_typ = output_typ;
                }
            }
        }
    }

    fn typecheck_template_global<ID: Copy + Into<GlobalUUID>>(
        &mut self,
        global_ref: &GlobalReference<ID>,
    ) {
        let global_obj: GlobalUUID = global_ref.id.into();
        let target_link_info = self.globals.get_link_info(global_obj);

        global_ref.resolve_template_args(self.errors, target_link_info);

        // This iteration has to split into two parts, because we first have to set all the type
        // parameters for use by creating the types to compare against the value parameters
        let mut abs_types =
            target_link_info
                .template_parameters
                .map(|(id, param)| match &param.kind {
                    TemplateKind::Type(_) => {
                        if let Some(wr_typ) = global_ref.get_type_arg_for(id) {
                            self.typecheck_written_type(wr_typ)
                        } else {
                            self.type_checker.alloc_unknown()
                        }
                    }
                    TemplateKind::Value(_) => {
                        AbstractRankedType {
                            // Will immediately get overwritten
                            inner: AbstractInnerType::Unknown(UUID::PLACEHOLDER),
                            rank: PeanoType::Unknown(UUID::PLACEHOLDER),
                        }
                    }
                });

        for (id, param) in &target_link_info.template_parameters {
            match &param.kind {
                TemplateKind::Type(_) => {}
                TemplateKind::Value(v) => {
                    let target_decl = target_link_info.instructions[v.declaration_instruction]
                        .unwrap_declaration();

                    let param_required_typ = self
                        .type_checker
                        .written_to_abstract_type_substitute_templates(
                            &target_decl.typ_expr,
                            &abs_types, // We substitute the templates for type variables here
                        );

                    if let Some(from_expr) = global_ref.get_value_arg_for(id) {
                        let from_expr = self.instructions[from_expr].unwrap_subexpression();

                        self.type_checker.unify_report_error(
                            from_expr.typ,
                            &param_required_typ,
                            from_expr.span,
                            "Template argument",
                        );
                    }
                    abs_types[id] = param_required_typ;
                }
            }
        }

        global_ref.template_arg_types.set(abs_types);
    }

    fn typecheck_written_type(&mut self, wr_typ: &WrittenType) -> AbstractRankedType {
        match wr_typ {
            WrittenType::Error(_) => self.type_checker.alloc_unknown(),
            WrittenType::TemplateVariable(_, var) => AbstractInnerType::Template(*var).scalar(),
            WrittenType::Named(global_ref) => {
                self.typecheck_template_global(global_ref);

                AbstractInnerType::Named(global_ref.id).scalar()
            }
            WrittenType::Array(_, arr_box) => {
                let (content_typ, arr_idx, _bracket_span) = arr_box.deref();

                let content_typ = self.typecheck_written_type(content_typ);

                let idx_expr = self.instructions[*arr_idx].unwrap_subexpression();
                self.type_checker.unify_report_error(
                    idx_expr.typ,
                    &INT_TYPE.scalar(),
                    idx_expr.span,
                    "array size",
                );

                content_typ.rank_up()
            }
        }
    }

    fn typecheck_visit_latency_specifier(&mut self, lat_spec: Option<FlatID>) {
        if let Some(latency_spec) = lat_spec {
            let latency_specifier_expr = self.instructions[latency_spec].unwrap_subexpression();
            self.type_checker.unify_report_error(
                latency_specifier_expr.typ,
                &INT_TYPE.scalar(),
                latency_specifier_expr.span,
                "latency specifier",
            );
        }
    }

    fn report_errors_for_bad_function_call(
        &self,
        func_call: &FuncCall,
        interface: &RemoteInterface<'l>,
        whole_func_span: Span,
        mut to_spans_iter: impl ExactSizeIterator<Item = Span>,
    ) {
        let arg_count = func_call.arguments.len();
        let expected_arg_count = interface.interface.func_call_inputs.len();

        if arg_count != expected_arg_count {
            if arg_count > expected_arg_count {
                // Too many args, complain about excess args at the end
                let excess_args_span = Span::new_overarching(
                    self.instructions[func_call.arguments[expected_arg_count]]
                        .unwrap_expression()
                        .span,
                    self.instructions[*func_call.arguments.last().unwrap()]
                        .unwrap_expression()
                        .span,
                );

                self.errors
                    .error(excess_args_span, format!("Excess argument. Function takes {expected_arg_count} args, but {arg_count} were passed."))
                    .info_obj(interface);
            } else {
                // Too few args, mention missing argument names
                self.errors
                    .error(func_call.arguments_span.close_bracket(), format!("Too few arguments. Function takes {expected_arg_count} args, but {arg_count} were passed."))
                    .info_obj(interface);
            }
        }

        let num_func_outputs = interface.interface.func_call_outputs.len();
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
                    .info_obj(interface);
            } else {
                self.errors
                    .error(whole_func_span, format!("Too few output targets. Function returns {num_func_outputs} results, but {num_targets} targets were given."))
                    .info_obj(interface);
            }
        }
    }

    fn typecheck_func_call(&mut self, func_call: &FuncCall) -> RemoteInterface<'l> {
        let interface = RemoteSubModule::make(
            func_call.interface_reference.submodule_decl,
            self.instructions,
            self.globals,
        )
        .get_interface_reference(func_call.interface_reference.submodule_interface);

        for (port, arg) in
            std::iter::zip(interface.interface.func_call_inputs, &func_call.arguments)
        {
            let port_decl = interface.get_port(port);
            let port_type = port_decl.get_local_type(&mut self.type_checker);

            // Typecheck the value with target type
            let from = self.instructions[*arg].unwrap_subexpression();

            self.type_checker
                .unify_report_error(from.typ, &port_type, from.span, || {
                    ("function argument".to_string(), vec![port_decl.make_info()])
                });
        }

        interface
    }

    fn typecheck_single_output_expr(&mut self, expr: &Expression) -> AbstractRankedType {
        match &expr.source {
            ExpressionSource::WireRef(wire_ref) => {
                self.typecheck_wire_reference(wire_ref);
                wire_ref.get_output_typ().clone()
            }
            ExpressionSource::UnaryOp { op, rank, right } => {
                let right_expr = self.instructions[*right].unwrap_subexpression();
                let out_typ = self.type_checker.typecheck_unary_operator_abstr(
                    *op,
                    right_expr.typ,
                    right_expr.span,
                );
                rank.set(out_typ.rank.clone());
                out_typ
            }
            ExpressionSource::BinaryOp {
                op,
                rank,
                left,
                right,
            } => {
                let left_expr = self.instructions[*left].unwrap_subexpression();
                let right_expr = self.instructions[*right].unwrap_subexpression();
                let out_typ = self.type_checker.typecheck_binary_operator_abstr(
                    *op,
                    left_expr.typ,
                    right_expr.typ,
                    left_expr.span,
                    right_expr.span,
                );
                rank.set(out_typ.rank.clone());
                out_typ
            }
            ExpressionSource::FuncCall(func_call) => {
                let interface = self.typecheck_func_call(func_call);

                self.report_errors_for_bad_function_call(
                    func_call,
                    &interface,
                    expr.span,
                    std::iter::once(expr.span),
                );

                if let Some(first_output) = interface.interface.func_call_outputs.first() {
                    let port_decl = interface.get_port(first_output);

                    port_decl.get_local_type(&mut self.type_checker)
                } else {
                    self.type_checker.alloc_unknown()
                }
            }
            ExpressionSource::Literal(value) => AbstractRankedType {
                inner: AbstractInnerType::Named(value.get_type_id()),
                rank: PeanoType::Zero,
            },
            ExpressionSource::ArrayConstruct(arr) => {
                let mut arr_iter = arr.iter();
                let arr_elem_typ = if let Some(first_elem) = arr_iter.next() {
                    let first_elem_expr = self.instructions[*first_elem].unwrap_subexpression();
                    let elem_typ = first_elem_expr.typ.clone();

                    for elem_id in arr_iter {
                        let elem_expr = self.instructions[*elem_id].unwrap_subexpression();

                        self.type_checker.unify_report_error(
                            elem_expr.typ,
                            &elem_typ,
                            elem_expr.span,
                            || {
                                let first_elem_info = ErrorInfo {
                                    position: first_elem_expr.span,
                                    file: self.errors.file,
                                    info: "First array element defined here".to_owned(),
                                };
                                ("array construction types".to_owned(), vec![first_elem_info])
                            },
                        );
                    }

                    elem_typ
                } else {
                    self.type_checker.alloc_unknown()
                };
                arr_elem_typ.rank_up()
            }
        }
    }
    fn typecheck_multi_output_expr(&mut self, expr: &Expression, multi_write: &[WriteTo]) {
        for wr in multi_write {
            self.typecheck_wire_reference(&wr.to);
        }
        match &expr.source {
            ExpressionSource::FuncCall(func_call) => {
                let interface = self.typecheck_func_call(func_call);

                self.report_errors_for_bad_function_call(
                    func_call,
                    &interface,
                    expr.span,
                    multi_write.iter().map(|v| v.to_span),
                );

                for (port, to) in std::iter::zip(interface.interface.func_call_outputs, multi_write)
                {
                    let port_decl = interface.get_port(port);
                    let port_type = port_decl.get_local_type(&mut self.type_checker);

                    self.type_checker.unify_report_error(
                        to.to.get_output_typ(),
                        &port_type,
                        to.to_span,
                        || ("function output".to_string(), vec![port_decl.make_info()]),
                    );
                }
            }
            ExpressionSource::WireRef(..)
            | ExpressionSource::UnaryOp { .. }
            | ExpressionSource::BinaryOp { .. }
            | ExpressionSource::ArrayConstruct(..)
            | ExpressionSource::Literal(..) => {
                let expr_out_typ = self.typecheck_single_output_expr(expr);
                if let Some(first_write) = multi_write.first() {
                    self.type_checker.unify_report_error(
                        &expr_out_typ,
                        first_write.to.get_output_typ(),
                        first_write.to_span,
                        "writing the output of this expression",
                    );
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
    }

    fn typecheck_visit_instruction(&mut self, instr_id: FlatID) {
        match &self.instructions[instr_id] {
            Instruction::SubModule(sm) => {
                self.typecheck_template_global(&sm.module_ref);
            }
            Instruction::Declaration(decl) => {
                self.typecheck_visit_latency_specifier(decl.latency_specifier);

                decl.typ.set(self.typecheck_written_type(&decl.typ_expr));
            }
            Instruction::IfStatement(stm) => {
                let condition_expr = &self.instructions[stm.condition].unwrap_subexpression();
                self.type_checker.unify_report_error(
                    condition_expr.typ,
                    &BOOL_TYPE.scalar(),
                    condition_expr.span,
                    "if statement condition",
                );
            }
            Instruction::ForStatement(stm) => {
                let loop_var = self.instructions[stm.loop_var_decl].unwrap_declaration();
                let start = self.instructions[stm.start].unwrap_subexpression();
                let end = self.instructions[stm.end].unwrap_subexpression();

                self.type_checker.unify_report_error(
                    start.typ,
                    &loop_var.typ,
                    start.span,
                    "for loop start",
                );
                self.type_checker.unify_report_error(
                    end.typ,
                    &loop_var.typ,
                    end.span,
                    "for loop end",
                );
            }
            Instruction::Expression(expr) => match &expr.output {
                ExpressionOutput::SubExpression(typ) => {
                    typ.set(self.typecheck_single_output_expr(expr));
                }
                ExpressionOutput::MultiWrite(write_tos) => {
                    self.typecheck_multi_output_expr(expr, write_tos);
                }
            },
        }
    }

    /// Should be followed up by a [apply_types] call to actually apply all the checked types.
    fn typecheck(&mut self) {
        for elem_id in self.instructions.id_range() {
            self.typecheck_visit_instruction(elem_id);
        }
    }
}

impl AbstractTypeSubstitutor {
    /// This should always be what happens first to a given variable.
    ///
    /// Therefore it should be impossible that one of the internal unifications ever fails
    ///
    /// template_type_args applies to both Template Type args and Template Value args.
    ///
    /// For Types this is the Type, for Values this is unified with the parameter declaration type
    fn written_to_abstract_type_substitute_templates(
        &mut self,
        wr_typ: &WrittenType,
        template_args: &TVec<AbstractRankedType>,
    ) -> AbstractRankedType {
        match wr_typ {
            WrittenType::Error(_span) => self.alloc_unknown(),
            WrittenType::TemplateVariable(_span, argument_id) => {
                template_args[*argument_id].clone()
            }
            WrittenType::Named(global_reference) => {
                AbstractInnerType::Named(global_reference.id).scalar()
            }
            WrittenType::Array(_span, array_content_and_size) => {
                let (arr_content_type, _size_flat, _array_bracket_span) =
                    array_content_and_size.deref();

                let content_typ = self
                    .written_to_abstract_type_substitute_templates(arr_content_type, template_args);

                content_typ.rank_up()
            }
        }
    }
}
impl TypeUnifier<AbstractTypeSubstitutor> {
    /// Returns the type of the content of the array
    fn rank_down<Report: UnifyErrorReport>(
        &mut self,
        arr_typ: &AbstractRankedType,
        span: Span,
        context: Report,
    ) -> AbstractRankedType {
        if let PeanoType::Succ(content_rank) = &arr_typ.rank {
            AbstractRankedType {
                inner: arr_typ.inner.clone(),
                rank: content_rank.deref().clone(),
            }
        } else {
            let content_rank = self.rank_substitutor.alloc_unknown();
            let mut content_typ = AbstractRankedType {
                inner: arr_typ.inner.clone(),
                rank: PeanoType::Succ(Box::new(content_rank.clone())),
            };
            self.unify_report_error(arr_typ, &content_typ, span, context);
            content_typ.rank = content_rank;
            content_typ
        }
    }

    /// Returns the output type. It happens that the operator rank is the output type's rank
    fn typecheck_unary_operator_abstr(
        &mut self,
        op: UnaryOperator,
        input_typ: &AbstractRankedType,
        span: Span,
    ) -> AbstractRankedType {
        let input_rank = input_typ.rank.clone();
        if op == UnaryOperator::Not {
            self.unify_report_error(
                input_typ,
                &BOOL_TYPE.with_rank(input_rank.clone()),
                span,
                "! input",
            );

            BOOL_TYPE.with_rank(input_rank)
        } else if op == UnaryOperator::Negate {
            self.unify_report_error(
                input_typ,
                &INT_TYPE.with_rank(input_rank.clone()),
                span,
                "unary - input",
            );
            INT_TYPE.with_rank(input_rank)
        } else {
            let reduction_type = match op {
                UnaryOperator::And => BOOL_TYPE,
                UnaryOperator::Or => BOOL_TYPE,
                UnaryOperator::Xor => BOOL_TYPE,
                UnaryOperator::Sum => INT_TYPE,
                UnaryOperator::Product => INT_TYPE,
                _ => unreachable!(),
            };
            let reduction_type = reduction_type.with_rank(input_rank.clone());
            self.unify_report_error(input_typ, &reduction_type, span, "array reduction");
            self.rank_down(&reduction_type, span, "array reduction")
        }
    }

    fn typecheck_binary_operator_abstr(
        &mut self,
        op: BinaryOperator,
        left_typ: &AbstractRankedType,
        right_typ: &AbstractRankedType,
        left_span: Span,
        right_span: Span,
    ) -> AbstractRankedType {
        let (exp_left, exp_right, out_typ) = match op {
            BinaryOperator::And => (BOOL_TYPE, BOOL_TYPE, BOOL_TYPE),
            BinaryOperator::Or => (BOOL_TYPE, BOOL_TYPE, BOOL_TYPE),
            BinaryOperator::Xor => (BOOL_TYPE, BOOL_TYPE, BOOL_TYPE),
            BinaryOperator::Add => (INT_TYPE, INT_TYPE, INT_TYPE),
            BinaryOperator::Subtract => (INT_TYPE, INT_TYPE, INT_TYPE),
            BinaryOperator::Multiply => (INT_TYPE, INT_TYPE, INT_TYPE),
            BinaryOperator::Divide => (INT_TYPE, INT_TYPE, INT_TYPE),
            BinaryOperator::Modulo => (INT_TYPE, INT_TYPE, INT_TYPE),
            BinaryOperator::Equals => (INT_TYPE, INT_TYPE, BOOL_TYPE),
            BinaryOperator::NotEquals => (INT_TYPE, INT_TYPE, BOOL_TYPE),
            BinaryOperator::GreaterEq => (INT_TYPE, INT_TYPE, BOOL_TYPE),
            BinaryOperator::Greater => (INT_TYPE, INT_TYPE, BOOL_TYPE),
            BinaryOperator::LesserEq => (INT_TYPE, INT_TYPE, BOOL_TYPE),
            BinaryOperator::Lesser => (INT_TYPE, INT_TYPE, BOOL_TYPE),
        };
        let input_rank = left_typ.rank.clone();
        let exp_left = exp_left.with_rank(input_rank.clone());
        let exp_right = exp_right.with_rank(input_rank.clone());
        let out_typ = out_typ.with_rank(input_rank.clone());

        self.unify_report_error(left_typ, &exp_left, left_span, "binop left side");
        self.unify_report_error(right_typ, &exp_right, right_span, "binop right side");
        out_typ
    }
}

struct FinalizationContext<'l, 'errs> {
    linker_types: &'l ArenaAllocator<StructType, TypeUUIDMarker>,
    errors: &'errs ErrorCollector<'l>,
    type_checker: TypeUnifier<AbstractTypeSubstitutor>,
    template_names: &'l TVec<Parameter>,
}

impl FinalizationContext<'_, '_> {
    pub fn apply_types(mut self, instructions: &mut FlatAlloc<Instruction, FlatIDMarker>) {
        // Post type application. Solidify types and flag any remaining AbstractType::Unknown
        for (_id, inst) in instructions.iter_mut() {
            match inst {
                Instruction::Expression(expr) => {
                    match &mut expr.output {
                        ExpressionOutput::SubExpression(expr_typ) => {
                            self.finalize_abstract_type(expr_typ.get_mut(), expr.span);
                        }
                        ExpressionOutput::MultiWrite(write_tos) => {
                            for wr in write_tos {
                                self.finalize_wire_ref(&mut wr.to);
                            }
                        }
                    }
                    match &mut expr.source {
                        ExpressionSource::WireRef(wr) => {
                            self.finalize_wire_ref(wr);
                        }
                        ExpressionSource::UnaryOp { rank, .. }
                        | ExpressionSource::BinaryOp { rank, .. } => {
                            let _ = rank
                                .get_mut()
                                .fully_substitute(&self.type_checker.rank_substitutor);
                            // No need to report incomplete peano error, as one of the ports would have reported it
                        }
                        _ => {}
                    }
                }
                Instruction::Declaration(decl) => {
                    self.finalize_abstract_type(decl.typ.get_mut(), decl.name_span)
                }
                // TODO Submodule domains may not be crossed either?
                Instruction::SubModule(sm) => {
                    self.finalize_global_ref(&mut sm.module_ref);
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
        } in self.type_checker.extract_errors()
        {
            // Not being able to fully substitute is not an issue. We just display partial types
            let _ = found.fully_substitute(&self.type_checker);
            let _ = expected.fully_substitute(&self.type_checker);

            let expected_name = expected
                .display(self.linker_types, self.template_names)
                .to_string();
            let found_name = found
                .display(self.linker_types, self.template_names)
                .to_string();
            self.errors
            .error(span, format!("Typing Error: {context} expects a {expected_name} but was given a {found_name}"))
            .add_info_list(infos);

            assert_ne!(found, expected);

            /*assert!(
                expected_name != found_name,
                "{expected_name} != {found_name}"
            );*/
        }
    }

    pub fn finalize_abstract_type(&self, typ: &mut AbstractRankedType, span: Span) {
        if !typ.fully_substitute(&self.type_checker) {
            self.errors.error(
                span,
                format!(
                    "Could not fully figure out the type of this object. {}",
                    typ.display(self.linker_types, self.template_names)
                ),
            );
        }
    }

    pub fn finalize_global_ref<ID>(&self, global_ref: &mut GlobalReference<ID>) {
        let global_ref_span = global_ref.get_total_span();
        for (_template_id, arg) in global_ref.template_arg_types.get_mut() {
            self.finalize_abstract_type(arg, global_ref_span);
        }
    }

    pub fn finalize_wire_ref(&self, wire_ref: &mut WireReference) {
        if let WireReferenceRoot::NamedConstant(cst) = &mut wire_ref.root {
            self.finalize_global_ref(cst);
        }
        self.finalize_abstract_type(wire_ref.root_typ.get_mut(), wire_ref.root_span);
        for path_elem in &mut wire_ref.path {
            match path_elem {
                WireReferencePathElement::ArrayAccess {
                    output_typ,
                    bracket_span,
                    ..
                } => {
                    self.finalize_abstract_type(output_typ.get_mut(), bracket_span.outer_span());
                }
            }
        }
    }
}
