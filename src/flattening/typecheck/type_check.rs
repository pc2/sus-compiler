use std::ops::Deref;

use crate::errors::ErrorInfo;
use crate::linker::passes::{LocalOrRemoteParentModule, RemoteDeclaration, RemoteFn};
use crate::prelude::*;
use crate::to_string::display_join;
use crate::typing::abstract_type::{
    AbstractInnerType, AbstractRankedType, BOOL_INNER, BOOL_SCALAR, FLOAT_SCALAR, INT_INNER,
    INT_SCALAR,
};
use crate::typing::template::TVec;
use crate::typing::type_inference::{AbstractTypeSubstitutor, TypeUnifier, UnifyErrorReport};

use crate::linker::GlobalUUID;

use crate::typing::template::TemplateKind;

use super::*;

impl<'l> TypeCheckingContext<'l> {
    // ===== Declaration and Global Reference Initialization =====
    fn initialize_global_ref<ID: Copy + Into<GlobalUUID>>(
        &mut self,
        global_ref: &GlobalReference<ID>,
    ) {
        let global_obj: GlobalUUID = global_ref.id.into();
        let target_link_info = &self.globals.get(global_obj).get_link_info();

        global_ref.resolve_template_args(self.errors, target_link_info);

        // This iteration has to split into two parts, because we first have to set all the type
        // parameters for use by creating the types to compare against the value parameters
        let template_arg_types = target_link_info
            .parameters
            .map(|(id, param)| match &param.kind {
                TemplateKind::Type(_) => TemplateKind::Type({
                    if let Some(wr_typ) = global_ref.get_type_arg_for(id) {
                        self.written_to_abstract_type(wr_typ)
                    } else {
                        self.type_checker.alloc_unknown()
                    }
                }),
                TemplateKind::Value(_) => TemplateKind::Value(()),
            });

        global_ref.template_arg_types.set(template_arg_types);
    }
    fn written_to_abstract_type(&mut self, wr_typ: &WrittenType) -> AbstractRankedType {
        match wr_typ {
            WrittenType::Error(_) => self.type_checker.alloc_unknown(),
            WrittenType::TemplateVariable(_, var) => AbstractInnerType::Template(*var).scalar(),
            WrittenType::Named(global_ref) => {
                self.initialize_global_ref(global_ref);

                AbstractInnerType::Named(global_ref.as_abstract_global_ref()).scalar()
            }
            WrittenType::Array(_, arr_box) => {
                let (content_typ, _idx, _bracket_span) = arr_box.deref();

                self.written_to_abstract_type(content_typ).rank_up()
            }
        }
    }
    fn init_wire_ref(&mut self, wr: &'l WireReference) {
        match &wr.root {
            WireReferenceRoot::LocalDecl(_)
            | WireReferenceRoot::LocalSubmodule(_)
            | WireReferenceRoot::LocalInterface(_)
            | WireReferenceRoot::Error => {}
            WireReferenceRoot::NamedConstant(global_ref) => {
                self.initialize_global_ref(global_ref);
            }
            WireReferenceRoot::NamedModule(global_ref) => {
                self.initialize_global_ref(global_ref);
            }
        }
    }
    pub fn init_all_declarations(&mut self) {
        for (_, instr) in self.instructions {
            match instr {
                Instruction::SubModule(submod_instr) => {
                    self.initialize_global_ref(&submod_instr.module_ref);
                }
                Instruction::Declaration(decl) => {
                    decl.typ.set(self.written_to_abstract_type(&decl.typ_expr));
                }
                Instruction::Expression(expr) => {
                    if let ExpressionSource::WireRef(wr) = &expr.source {
                        self.init_wire_ref(wr);
                    }
                    if let ExpressionOutput::MultiWrite(wrs) = &expr.output {
                        for wr in wrs {
                            self.init_wire_ref(&wr.to);
                        }
                    }
                }
                Instruction::Interface(_)
                | Instruction::IfStatement(_)
                | Instruction::ForStatement(_) => {}
            }
        }
    }

    // ===== Further Typechecking =====
    fn typecheck_wire_reference(&mut self, wire_ref: &WireReference) {
        let root_typ = match &wire_ref.root {
            WireReferenceRoot::LocalDecl(decl_id) => {
                let decl = self.instructions[*decl_id].unwrap_declaration();
                decl.typ.clone()
            }
            WireReferenceRoot::LocalSubmodule(submod_decl) => {
                let submod = self.instructions[*submod_decl].unwrap_submodule();
                submod.typ.clone()
            }
            WireReferenceRoot::LocalInterface(interface_decl) => {
                let _ = self.instructions[*interface_decl].unwrap_interface();
                AbstractRankedType {
                    inner: AbstractInnerType::LocalInterface(*interface_decl),
                    rank: PeanoType::Zero,
                }
            }
            WireReferenceRoot::NamedConstant(cst) => {
                self.typecheck_global_ref(cst);

                self.globals
                    .get_global_constant(cst)
                    .get_target_decl()
                    .get_local_type(&self.globals, &mut self.type_checker)
            }
            WireReferenceRoot::NamedModule(md) => {
                self.typecheck_global_ref(md);

                AbstractRankedType {
                    inner: AbstractInnerType::Interface(
                        md.as_abstract_global_ref(),
                        InterfaceID::MAIN_INTERFACE,
                    ),
                    rank: PeanoType::Zero,
                }
            }
            WireReferenceRoot::Error => self.type_checker.alloc_unknown(),
        };

        let mut walking_typ = root_typ;
        for p in &wire_ref.path {
            match p {
                WireReferencePathElement::FieldAccess {
                    name,
                    name_span,
                    refers_to,
                } => {
                    walking_typ = match &walking_typ.inner {
                        AbstractInnerType::Template(template_id) => {
                            let template_arg = &self.link_info.parameters[*template_id];
                            self.errors
                                .error(
                                    *name_span,
                                    format!(
                                        "The type of this object is the template parameter '{}'. You cannot use struct fields on template args",
                                        template_arg.name
                                    ),
                                )
                                .info_obj_same_file(template_arg);
                            self.type_checker.alloc_unknown()
                        }
                        AbstractInnerType::LocalInterface(interface_id) => {
                            let interface_decl =
                                self.link_info.instructions[*interface_id].unwrap_interface();
                            self.errors
                                .error(
                                    *name_span,
                                    format!(
                                        "The type of this object is a local interface '{}'. You cannot use struct fields on local interfaces",
                                        interface_decl.name
                                    ),
                                )
                                .info_obj_same_file(interface_decl);
                            self.type_checker.alloc_unknown()
                        }
                        AbstractInnerType::Named(_) => {
                            self.errors.todo(*name_span, "Structs");
                            self.type_checker.alloc_unknown() // todo!("Structs")
                        }
                        // TODO "subinterfaces"
                        AbstractInnerType::Interface(md_ref, _interface) => {
                            let md = self.globals.get_submodule(md_ref);

                            let interface = md
                                .md
                                .interfaces
                                .find(|_, interface| &interface.name == name);
                            refers_to
                                .set(PathElemRefersTo::Interface(md_ref.id, interface))
                                .unwrap();

                            if let Some(interface) = interface {
                                if let Some(InterfaceDeclKind::SinglePort(port_decl)) =
                                    md.md.interfaces[interface].declaration_instruction
                                {
                                    md.get_decl(port_decl)
                                        .get_local_type(&self.globals, &mut self.type_checker)
                                } else {
                                    AbstractRankedType {
                                        inner: AbstractInnerType::Interface(
                                            md_ref.clone(),
                                            interface,
                                        ),
                                        rank: PeanoType::Zero,
                                    }
                                }
                            } else {
                                let obj = self.globals.get_module(md_ref.id);
                                let obj_name = md_ref.display(self.globals.globals, self.link_info);
                                let field_names =
                                    display_join(", ", obj.interfaces.iter(), |f, (_, v)| {
                                        write!(f, "'{}'", v.name)
                                    });
                                self.errors
                                    .error(
                                        *name_span,
                                        format!("No such field '{name}' on {obj_name}. Available fields are {field_names}"),
                                    )
                                    .info_obj(obj);

                                self.type_checker.alloc_unknown()
                            }
                        }
                        AbstractInnerType::Unknown(_) => self.type_checker.alloc_unknown(), // todo!("Structs")
                    }
                }
                WireReferencePathElement::ArrayAccess { idx, bracket_span } => {
                    self.must_be_int(*idx);

                    walking_typ = self.type_checker.rank_down(
                        &walking_typ,
                        bracket_span.outer_span(),
                        "array access",
                    );
                }
                WireReferencePathElement::ArraySlice { from, to, .. } => {
                    if let Some(from) = from {
                        self.must_be_int(*from);
                    }
                    if let Some(to) = to {
                        self.must_be_int(*to);
                    }

                    // Identity
                    // TODO: This doesn't cover the case where there are more array accesses than arrays.
                    // walking_typ = walking_typ;
                }
                WireReferencePathElement::ArrayPartSelect { from, width, .. } => {
                    self.must_be_int(*from);
                    self.must_be_int(*width);

                    // Identity
                    // TODO: This doesn't cover the case where there are more array accesses than arrays.
                    // walking_typ = walking_typ;
                }
            }
        }
        wire_ref.output_typ.set(walking_typ);
    }

    fn must_be_int(&mut self, expr_id: FlatID) {
        let idx_expr = self.instructions[expr_id].unwrap_subexpression();

        self.type_checker.unify_report_error(
            idx_expr.typ,
            &INT_SCALAR,
            idx_expr.span,
            "array index",
        );
    }

    fn typecheck_global_ref<ID: Copy + Into<GlobalUUID>>(
        &mut self,
        global_ref: &GlobalReference<ID>,
    ) {
        let global_obj: GlobalUUID = global_ref.id.into();
        let target_link_info = &self.globals.get(global_obj).get_link_info();

        for arg in &global_ref.template_args {
            match &arg.kind {
                Some(TemplateKind::Type(t)) => {
                    // even if we're wrongly operating on a value, we might as well check the user's written type is correctly typed.
                    self.typecheck_written_type(t);
                }
                Some(TemplateKind::Value(from_expr)) => {
                    if let Some(template_id) = arg.refers_to.get() {
                        let TemplateKind::Value(remote_parameter) =
                            &target_link_info.parameters[*template_id].kind
                        else {
                            // Error handled by [GlobalReference::resolve_template_args]
                            continue;
                        };

                        let template_types: &FlatAlloc<_, _> = &global_ref.template_arg_types;

                        let target_decl = RemoteDeclaration::new(
                            target_link_info,
                            remote_parameter.declaration_instruction,
                            Some(template_types),
                        );

                        let param_required_typ =
                            target_decl.get_local_type(&self.globals, &mut self.type_checker);

                        let from_expr = self.instructions[*from_expr].unwrap_subexpression();

                        self.type_checker.unify_report_error(
                            from_expr.typ,
                            &param_required_typ,
                            from_expr.span,
                            "template argument",
                        );
                    }
                }
                None => {}
            }
        }
    }

    fn typecheck_written_type(&mut self, wr_typ: &WrittenType) {
        match wr_typ {
            WrittenType::Error(_) => {}
            WrittenType::TemplateVariable(_, _) => {}
            WrittenType::Named(global_ref) => {
                self.typecheck_global_ref(global_ref);
            }
            WrittenType::Array(_, arr_box) => {
                let (_content_typ, arr_idx, _bracket_span) = arr_box.deref();

                let idx_expr = self.instructions[*arr_idx].unwrap_subexpression();
                self.type_checker.unify_report_error(
                    idx_expr.typ,
                    &INT_SCALAR,
                    idx_expr.span,
                    "array size",
                );
            }
        }
    }

    fn typecheck_visit_latency_specifier(&mut self, lat_spec: Option<FlatID>) {
        if let Some(latency_spec) = lat_spec {
            let latency_specifier_expr = self.instructions[latency_spec].unwrap_subexpression();
            self.type_checker.unify_report_error(
                latency_specifier_expr.typ,
                &INT_SCALAR,
                latency_specifier_expr.span,
                "latency specifier",
            );
        }
    }

    fn report_errors_for_bad_function_call(
        &self,
        func_call: &FuncCall,
        interface: &RemoteFn<'l, &'l TVec<TemplateKind<AbstractRankedType, ()>>>,
        whole_func_span: Span,
        mut to_spans_iter: impl ExactSizeIterator<Item = Span>,
    ) {
        let arg_count = func_call.arguments.len();
        let expected_arg_count = interface.fn_decl.inputs.len();

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

        let num_func_outputs = interface.fn_decl.outputs.len();
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

    /// ```sus
    /// when myMod.f : int a, int b -> bool c {
    ///     c = a == b
    /// }
    /// ```
    /// `a` and `b` are read_only_decls
    /// `c` is the only writable_decls
    fn report_errors_for_bad_binding(
        &self,
        read_only_decls: &[FlatID],
        writable_decls: &[FlatID],
        interface: &RemoteFn<'l, &'l TVec<TemplateKind<AbstractRankedType, ()>>>,
        interface_name_span: Span,
    ) {
        let fn_decl = &interface.fn_decl;
        for (bindings, interface_args, name) in [
            (read_only_decls, &fn_decl.inputs, "read-only bindings"),
            (writable_decls, &fn_decl.outputs, "writable bindings"),
        ] {
            let arg_count = bindings.len();
            let expected_arg_count = interface_args.len();

            if arg_count != expected_arg_count {
                if arg_count > expected_arg_count {
                    // Too many args, complain about excess args at the end
                    let excess_args_span = Span::new_overarching(
                        self.instructions[bindings[expected_arg_count]]
                            .unwrap_declaration()
                            .decl_span,
                        self.instructions[*bindings.last().unwrap()]
                            .unwrap_declaration()
                            .decl_span,
                    );

                    self.errors
                        .error(excess_args_span, format!("Excess bindings. This interface provides {expected_arg_count} {name}, but {arg_count} were provided."))
                        .info_obj(interface);
                } else {
                    // Too few args, mention missing argument names
                    let too_few_args_span = if let Some(last) = bindings.last() {
                        self.instructions[*last]
                            .unwrap_declaration()
                            .decl_span
                            .empty_span_at_end()
                    } else {
                        interface_name_span
                    };

                    self.errors
                        .error(too_few_args_span, format!("Too few bindings. This interface provides {expected_arg_count} {name}, but {arg_count} were provided."))
                        .info_obj(interface);
                }
            }
        }
    }

    /// If the wire_ref refers to a callable (so not just a hierarchical) interface, then this returns a RemoteFn. Handles the needed error reporting
    fn get_callable_func(
        &mut self,
        wire_ref_id: FlatID,
        context: &'static str,
    ) -> Option<RemoteFn<'l, &'l TVec<TemplateKind<AbstractRankedType, ()>>>> {
        let wire_ref_expr = self.instructions[wire_ref_id].unwrap_expression();
        let ExpressionSource::WireRef(wire_ref) = &wire_ref_expr.source else {
            self.errors.error(
                wire_ref_expr.span,
                "Cannot function-call on any expression. It must be a wire reference",
            );
            return None;
        };
        match &wire_ref.output_typ.inner {
            AbstractInnerType::Interface(sm_ref, interface) => {
                let submod = self.globals.get_submodule(sm_ref);
                let interface = &submod.md.interfaces[*interface];
                let Some(interface) = interface.declaration_instruction else {
                    let name = &interface.name;
                    let err_text = format!(
                        "{context} expects this to be a callable interface, the interface `{name}` is not callable"
                    );
                    self.errors
                        .error(wire_ref.get_total_span(), err_text)
                        .info_obj_different_file(interface, submod.md.link_info.file);
                    return None;
                };
                let_unwrap!(InterfaceDeclKind::Interface(interface), interface);
                Some(submod.get_fn(interface))
            }
            AbstractInnerType::LocalInterface(interface_decl) => {
                let fn_decl = self.link_info.instructions[*interface_decl].unwrap_interface();
                Some(RemoteFn {
                    parent: LocalOrRemoteParentModule::Local(self.link_info),
                    fn_decl,
                })
            }
            AbstractInnerType::Template(_)
            | AbstractInnerType::Named(_)
            | AbstractInnerType::Unknown(_) => {
                self.errors.error(
                    wire_ref.get_total_span(),
                    format!("{context} expects this to be an interface, but found a regular wire"),
                );
                None
            }
        }
    }

    fn typecheck_func_call_args(
        &mut self,
        func_call: &FuncCall,
        interface: RemoteFn<'l, &'l TVec<TemplateKind<AbstractRankedType, ()>>>,
    ) {
        for (decl_id, arg) in std::iter::zip(&interface.fn_decl.inputs, &func_call.arguments) {
            let port_decl = interface.parent.get_decl(*decl_id);
            let port_type = port_decl.get_local_type(&self.globals, &mut self.type_checker);

            // Typecheck the value with target type
            let from = self.instructions[*arg].unwrap_subexpression();

            self.type_checker
                .unify_report_error(from.typ, &port_type, from.span, || {
                    ("function argument".to_string(), vec![port_decl.make_info()])
                });
        }
    }

    fn typecheck_single_output_expr(&mut self, expr: &'l Expression) -> AbstractRankedType {
        match &expr.source {
            ExpressionSource::WireRef(wire_ref) => {
                self.typecheck_wire_reference(wire_ref);
                wire_ref.output_typ.clone()
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
                if let Some(interface) =
                    self.get_callable_func(func_call.func_wire_ref, "A function call")
                {
                    self.report_errors_for_bad_function_call(
                        func_call,
                        &interface,
                        expr.span,
                        std::iter::once(expr.span),
                    );

                    self.typecheck_func_call_args(func_call, interface);

                    if let Some(first_output) = interface.fn_decl.outputs.first() {
                        let port_decl = interface.parent.get_decl(*first_output);

                        port_decl.get_local_type(&self.globals, &mut self.type_checker)
                    } else {
                        self.type_checker.alloc_unknown()
                    }
                } else {
                    self.type_checker.alloc_unknown()
                }
            }
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
            ExpressionSource::Literal(value) => match value {
                Value::Bool(_) => BOOL_SCALAR,
                Value::Float(_) => FLOAT_SCALAR,
                Value::Integer(_) => INT_SCALAR.clone(),
                Value::Array(elements) => {
                    if let Some(fst) = elements.first() {
                        assert!(
                            matches!(fst, Value::Bool(_)),
                            "The only type of array literal we have is boolean arrays!"
                        ); // Future proof? Idk
                    }

                    AbstractRankedType {
                        inner: BOOL_INNER,
                        rank: PeanoType::from_natural(1),
                    }
                }
                Value::Unset => unreachable!(),
            },
        }
    }
    fn typecheck_multi_output_expr(&mut self, expr: &'l Expression, multi_write: &'l [WriteTo]) {
        for wr in multi_write {
            self.typecheck_wire_reference(&wr.to);
        }
        match &expr.source {
            ExpressionSource::FuncCall(func_call) => {
                if let Some(interface) =
                    self.get_callable_func(func_call.func_wire_ref, "A function call")
                {
                    self.report_errors_for_bad_function_call(
                        func_call,
                        &interface,
                        expr.span,
                        multi_write.iter().map(|v| v.to_span),
                    );

                    self.typecheck_func_call_args(func_call, interface);

                    for (decl_id, to) in std::iter::zip(&interface.fn_decl.outputs, multi_write) {
                        let port_decl = interface.parent.get_decl(*decl_id);
                        let port_type =
                            port_decl.get_local_type(&self.globals, &mut self.type_checker);

                        self.type_checker.unify_report_error(
                            &to.to.output_typ,
                            &port_type,
                            to.to_span,
                            || ("function output".to_string(), vec![port_decl.make_info()]),
                        );
                    }
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
                        &first_write.to.output_typ,
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

    pub fn type_check_instr(&mut self, instr: &'l Instruction) {
        match instr {
            Instruction::SubModule(sm) => {
                self.typecheck_global_ref(&sm.module_ref);
                sm.typ.set(AbstractRankedType {
                    inner: AbstractInnerType::Interface(
                        sm.module_ref.as_abstract_global_ref(),
                        InterfaceID::MAIN_INTERFACE,
                    ),
                    rank: PeanoType::Zero,
                });
            }
            Instruction::Declaration(decl) => {
                self.typecheck_visit_latency_specifier(decl.latency_specifier);

                self.typecheck_written_type(&decl.typ_expr);
            }
            Instruction::IfStatement(if_stm) => {
                let condition_expr = &self.instructions[if_stm.condition].unwrap_subexpression();
                if condition_expr.typ.inner.is_interface() {
                    if let Some(trig) =
                        self.get_callable_func(if_stm.condition, "A conditional binding")
                    {
                        self.type_check_conditional_bindings(if_stm, condition_expr, trig);
                    }
                } else {
                    if let Some(bindings_span) = if_stm.conditional_bindings_span {
                        self.errors.error(bindings_span, "Cannot use conditional bingings because the condition isn't an action or a trigger");
                    }

                    self.type_checker.unify_report_error(
                        condition_expr.typ,
                        &BOOL_SCALAR,
                        condition_expr.span,
                        "if statement condition",
                    );
                }
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
            Instruction::Interface(_act_trig) => {}
        }
    }

    fn type_check_conditional_bindings(
        &mut self,
        if_stm: &IfStatement,
        condition_expr: &SingleOutputExpression<'_>,
        trig: RemoteFn<'l, &'l TVec<TemplateKind<AbstractRankedType, ()>>>,
    ) {
        let f = &trig.fn_decl;
        if !matches!(&f.interface_kind, InterfaceKind::Trigger(_)) {
            let interface_name = &f.name;
            let kind_str = f.interface_kind;
            let err = format!(
                "Can only use conditional bindings on triggers. '{interface_name}' is an {kind_str}"
            );
            self.errors.error(condition_expr.span, err).info_obj(&trig);
        }

        self.report_errors_for_bad_binding(
            &if_stm.bindings_read_only,
            &if_stm.bindings_writable,
            &trig,
            condition_expr.span,
        );

        for (ports, bindings, binding_name) in [
            (&f.inputs, &if_stm.bindings_read_only, "read-only binding"),
            (&f.outputs, &if_stm.bindings_writable, "writeable binding"),
        ] {
            for (port_decl_id, binding) in std::iter::zip(ports, bindings) {
                let port_decl = trig.parent.get_decl(*port_decl_id);
                let port_type = port_decl.get_local_type(&self.globals, &mut self.type_checker);

                let binding_decl = self.instructions[*binding].unwrap_declaration();

                self.type_checker.unify_report_error(
                    &binding_decl.typ,
                    &port_type,
                    binding_decl.decl_span,
                    || (binding_name.to_string(), vec![port_decl.make_info()]),
                );
            }
        }
    }
}

impl<'l> RemoteDeclaration<'l, &'l TVec<TemplateKind<AbstractRankedType, ()>>> {
    fn get_local_type(
        &self,
        globals: &GlobalResolver<'_, '_>,
        type_checker: &mut AbstractTypeSubstitutor,
    ) -> AbstractRankedType {
        if let Some(template_args) = self.template_args {
            type_checker.written_to_abstract_type_substitute_templates(
                &self.remote_decl.typ_expr,
                globals,
                template_args,
            )
        } else {
            self.remote_decl.typ.clone()
        }
    }
}

impl AbstractTypeSubstitutor {
    fn written_to_abstract_global_ref_substitute_templates<ID: Into<GlobalUUID> + Copy>(
        &mut self,
        global_ref: &GlobalReference<ID>,
        globals: &GlobalResolver<'_, '_>,
        template_args: &TVec<TemplateKind<AbstractRankedType, ()>>,
    ) -> AbstractGlobalReference<ID> {
        let global_obj: GlobalUUID = global_ref.id.into();
        let target_link_info = &globals.get(global_obj).get_link_info();

        let template_arg_types = target_link_info
            .parameters
            .map(|(_, param)| match &param.kind {
                TemplateKind::Type(_) => TemplateKind::Type(
                    global_ref
                        .template_args
                        .iter()
                        .find_map(|arg| {
                            if let (Some(TemplateKind::Type(typ)), true) =
                                (&arg.kind, arg.name == param.name)
                            {
                                Some(self.written_to_abstract_type_substitute_templates(
                                    typ,
                                    globals,
                                    template_args,
                                ))
                            } else {
                                None
                            }
                        })
                        .unwrap_or_else(|| self.alloc_unknown()),
                ),
                TemplateKind::Value(_) => TemplateKind::Value(()),
            });

        AbstractGlobalReference {
            id: global_ref.id,
            template_arg_types,
        }
    }

    fn written_to_abstract_type_substitute_templates(
        &mut self,
        wr_typ: &WrittenType,
        globals: &GlobalResolver<'_, '_>,
        template_args: &TVec<TemplateKind<AbstractRankedType, ()>>,
    ) -> AbstractRankedType {
        match wr_typ {
            WrittenType::Error(_span) => self.alloc_unknown(),
            WrittenType::TemplateVariable(_span, argument_id) => {
                template_args[*argument_id].unwrap_type().clone()
            }
            WrittenType::Named(global_reference) => {
                let abs_ref = self.written_to_abstract_global_ref_substitute_templates(
                    global_reference,
                    globals,
                    template_args,
                );
                AbstractInnerType::Named(abs_ref).scalar()
            }
            WrittenType::Array(_span, array_content_and_size) => {
                let (arr_content_type, _size_flat, _array_bracket_span) =
                    array_content_and_size.deref();

                let content_typ = self.written_to_abstract_type_substitute_templates(
                    arr_content_type,
                    globals,
                    template_args,
                );

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
            let result_typ = BOOL_INNER.with_rank(input_rank);
            self.unify_report_error(input_typ, &result_typ, span, "! input");

            result_typ
        } else if op == UnaryOperator::Negate {
            let result_typ = INT_INNER.clone().with_rank(input_rank);
            self.unify_report_error(input_typ, &result_typ, span, "unary - input");
            result_typ
        } else {
            let reduction_type = match op {
                UnaryOperator::And => BOOL_INNER,
                UnaryOperator::Or => BOOL_INNER,
                UnaryOperator::Xor => BOOL_INNER,
                UnaryOperator::Sum => INT_INNER.clone(),
                UnaryOperator::Product => INT_INNER.clone(),
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
        let (exp_left, exp_right, out_typ): (
            &AbstractInnerType,
            &AbstractInnerType,
            &AbstractInnerType,
        ) = match op {
            BinaryOperator::And => (&BOOL_INNER, &BOOL_INNER, &BOOL_INNER),
            BinaryOperator::Or => (&BOOL_INNER, &BOOL_INNER, &BOOL_INNER),
            BinaryOperator::Xor => (&BOOL_INNER, &BOOL_INNER, &BOOL_INNER),
            BinaryOperator::Add => (&INT_INNER, &INT_INNER, &INT_INNER),
            BinaryOperator::Subtract => (&INT_INNER, &INT_INNER, &INT_INNER),
            BinaryOperator::Multiply => (&INT_INNER, &INT_INNER, &INT_INNER),
            BinaryOperator::Divide => (&INT_INNER, &INT_INNER, &INT_INNER),
            BinaryOperator::Modulo => (&INT_INNER, &INT_INNER, &INT_INNER),
            BinaryOperator::Equals => (&INT_INNER, &INT_INNER, &BOOL_INNER),
            BinaryOperator::NotEquals => (&INT_INNER, &INT_INNER, &BOOL_INNER),
            BinaryOperator::GreaterEq => (&INT_INNER, &INT_INNER, &BOOL_INNER),
            BinaryOperator::Greater => (&INT_INNER, &INT_INNER, &BOOL_INNER),
            BinaryOperator::LesserEq => (&INT_INNER, &INT_INNER, &BOOL_INNER),
            BinaryOperator::Lesser => (&INT_INNER, &INT_INNER, &BOOL_INNER),
        };
        let input_rank = left_typ.rank.clone();
        let exp_left = exp_left.clone().with_rank(input_rank.clone());
        let exp_right = exp_right.clone().with_rank(input_rank.clone());
        let out_typ = out_typ.clone().with_rank(input_rank.clone());

        self.unify_report_error(left_typ, &exp_left, left_span, "binop left side");
        self.unify_report_error(right_typ, &exp_right, right_span, "binop right side");
        out_typ
    }
}

impl FinalizationContext {
    pub fn apply_types(&mut self, instructions: &mut FlatAlloc<Instruction, FlatIDMarker>) {
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
    }

    fn finalize_abstract_type(&mut self, typ: &mut AbstractRankedType, span: Span) {
        if !typ.fully_substitute(&self.type_checker) {
            self.substitution_failures.push((typ.clone(), span));
        }
    }

    fn finalize_global_ref<ID: Copy>(&mut self, global_ref: &mut GlobalReference<ID>) {
        let global_ref_span = global_ref.get_total_span();
        for (_template_id, arg) in global_ref.template_arg_types.get_mut() {
            match arg {
                TemplateKind::Type(arg) => {
                    self.finalize_abstract_type(arg, global_ref_span);
                }
                TemplateKind::Value(()) => {}
            }
        }
    }

    fn finalize_wire_ref(&mut self, wire_ref: &mut WireReference) {
        match &mut wire_ref.root {
            WireReferenceRoot::NamedConstant(cst) => {
                self.finalize_global_ref(cst);
            }
            WireReferenceRoot::NamedModule(md) => {
                self.finalize_global_ref(md);
            }
            _ => {}
        }
        let total_span = wire_ref.get_total_span();
        self.finalize_abstract_type(wire_ref.output_typ.get_mut(), total_span);
    }
}
