use crate::errors::ErrorInfo;
use crate::prelude::*;

use super::*;

impl FinalizationContext {
    pub fn apply_domains(&self, instructions: &mut FlatAlloc<Instruction, FlatIDMarker>) {
        for (_, instr) in instructions {
            match instr {
                Instruction::SubModule(sm) => {
                    let local_domain_map = sm.local_domain_map.get_mut().unwrap();
                    for (_, d) in local_domain_map {
                        assert!(d.fully_substitute(&self.domain_checker));
                    }
                }
                Instruction::Declaration(declaration) => {
                    assert!(
                        declaration
                            .domain
                            .get_mut()
                            .fully_substitute(&self.domain_checker)
                    );
                }
                Instruction::Expression(expr) => {
                    assert!(expr.domain.get_mut().fully_substitute(&self.domain_checker));

                    if let ExpressionOutput::MultiWrite(writes) = &mut expr.output {
                        for w in writes {
                            assert!(
                                w.target_domain
                                    .get_mut()
                                    .fully_substitute(&self.domain_checker)
                            );
                        }
                    }
                }
                Instruction::IfStatement(_)
                | Instruction::ForStatement(_)
                | Instruction::Interface(_) => {}
            }
        }
    }
}

impl<'l> TypeCheckingContext<'l> {
    pub fn domain_check_instr(&mut self, instr: &Instruction) {
        match instr {
            Instruction::SubModule(sub_module_instance) => {
                sub_module_instance
                    .local_domain_map
                    .set(
                        self.globals
                            .get_module(sub_module_instance.module_ref.id)
                            .domains
                            .map(|_| self.domain_checker.alloc_unknown()),
                    )
                    .unwrap();
            }
            Instruction::Declaration(declaration) => {
                self.written_type_must_be_generative(&declaration.typ_expr);
                match declaration.decl_kind {
                    DeclarationKind::Port { .. } => {} // Domain is already set by flatten
                    DeclarationKind::StructField(..)
                    | DeclarationKind::RegularWire { .. }
                    | DeclarationKind::ConditionalBinding { .. } => {
                        declaration.domain.set(self.domain_checker.alloc_unknown())
                    }
                    DeclarationKind::RegularGenerative { .. }
                    | DeclarationKind::TemplateParameter { .. } => {
                        declaration.domain.set(DomainType::Generative)
                    }
                }
                if let Some(latency_spec) = declaration.latency_specifier {
                    self.must_be_generative(latency_spec, "Latency Specifier");
                }
            }
            Instruction::Expression(expression) => {
                let mut total_domain =
                    if let ExpressionSource::WireRef(wire_ref) = &expression.source {
                        let domain = self
                            .get_wireref_root_domain(wire_ref)
                            .unwrap_or(DomainType::Generative);
                        (domain, wire_ref.root_span)
                    } else {
                        (DomainType::Generative, Span::MAX_POSSIBLE_SPAN)
                    };

                expression.source.for_each_input_wire(&mut |id| {
                    let expr = self.instructions[id].unwrap_subexpression();
                    let expr_domain = (expr.domain, expr.span);

                    if total_domain.0 == DomainType::Generative {
                        total_domain = expr_domain;
                    } else if expr_domain.0 != DomainType::Generative {
                        self.unify_physicals(total_domain, expr_domain, "expression");
                    }
                });

                // Function call "writes" also require the condition domain
                if let ExpressionSource::FuncCall(fc) = &expression.source {
                    let call_expr = self.instructions[fc.func_wire_ref].unwrap_subexpression();
                    if call_expr.domain != DomainType::Generative {
                        if let Some(condition_domain) =
                            self.get_condition_domain(expression.parent_condition)
                        {
                            self.unify_physicals(
                                total_domain,
                                condition_domain,
                                "the runtime condition for function calls",
                            );
                        }
                    } else {
                        // TODO generative function calls
                    }
                }

                expression.domain.set(total_domain.0);

                // Regular "writes"
                if let ExpressionOutput::MultiWrite(writes) = &expression.output {
                    for wr in writes {
                        let mut target_domain = (
                            self.get_wireref_root_domain(&wr.to)
                                .unwrap_or_else(|| self.domain_checker.alloc_unknown()),
                            wr.to.root_span,
                        );

                        match wr.write_modifiers {
                            WriteModifiers::Connection { .. } => {
                                if let Some(condition_domain) =
                                    self.get_condition_domain(expression.parent_condition)
                                    && target_domain.0 != DomainType::Generative
                                {
                                    self.unify_physicals(
                                        target_domain,
                                        condition_domain,
                                        "the runtime condition",
                                    );
                                }
                            }
                            WriteModifiers::Initial { initial_kw_span } => {
                                target_domain = (DomainType::Generative, initial_kw_span);
                            }
                        }

                        wr.target_domain.set(target_domain.0);

                        wr.to.for_each_input_wire_in_path(&mut |id| {
                            let expr = self.instructions[id].unwrap_subexpression();
                            let expr_domain = (expr.domain, expr.span);
                            if expr_domain.0 != DomainType::Generative {
                                if target_domain.0 == DomainType::Generative {
                                    self.errors.error(expr_domain.1, "Attempting to write from a non-generative value to a generative value").info_same_file(target_domain.1, "This is a generative value");
                                } else {
                                    self.unify_physicals(target_domain, expr_domain, "assignment");
                                }
                            }
                        });
                        let expr_domain = (expression.domain.get(), expression.span);
                        if expr_domain.0 != DomainType::Generative {
                            if target_domain.0 == DomainType::Generative {
                                self.errors.error(expr_domain.1, "Attempting to write from a non-generative value to a generative value").info_same_file(target_domain.1, "This is a generative value");
                            } else {
                                self.unify_physicals(target_domain, expr_domain, "assignment");
                            }
                        }
                    }
                }
            }
            Instruction::IfStatement(if_statement) => {
                let condition = self.instructions[if_statement.condition].unwrap_subexpression();

                match (
                    if_statement.is_generative,
                    condition.domain == DomainType::Generative,
                ) {
                    (true, false) => {
                        self.errors.error(
                            if_statement.if_keyword_span,
                            "Used 'if' in a non generative context, use 'when' instead",
                        );
                    }
                    (false, true) => {
                        self.errors.error(
                            if_statement.if_keyword_span,
                            "Used 'when' in a generative context, use 'if' instead",
                        );
                    }
                    (_, _) => (),
                }
            }
            Instruction::ForStatement(for_statement) => {
                self.must_be_generative(for_statement.start, "For Loop start");
                self.must_be_generative(for_statement.end, "For Loop end");
            }
            Instruction::Interface(_) => {}
        }
    }

    /// Output is guaranteed not to be DomainType::Generative
    fn get_condition_domain(
        &self,
        mut parent_condition: Option<ParentCondition>,
    ) -> Option<(DomainType, Span)> {
        while let Some(p_cond) = parent_condition {
            match &self.instructions[p_cond.parent_when] {
                Instruction::Interface(decl) => return Some((decl.domain, decl.name_span)),
                Instruction::IfStatement(when) => {
                    let when_cond_expr = self.instructions[when.condition].unwrap_subexpression();
                    if when_cond_expr.domain != DomainType::Generative {
                        return Some((when_cond_expr.domain, when_cond_expr.span));
                    }
                    parent_condition = when.parent_condition;
                }
                _ => unreachable!(),
            }
        }
        None
    }

    /// Wire references are used in two contexts:
    /// - Reading from a wire
    /// - Writing to a wire
    ///
    /// The AbstractTypes just get unified
    ///
    /// But the domains behave differently.
    /// - Reading:
    ///   The domains combine to form the lowest common denominator.
    ///   If all are generative this becomes generative
    ///   At least one non-generative domain makes the whole thing non-generative
    ///   It should be supplied with a generative output_typ domain when generative, and an unknown domain variable otherwise
    /// - Writing:
    ///   The output_typ domain should be generative when wire_ref.root is generative, or a generative value is required such as with "initial"
    ///   When wire_ref.root is not generative, it should be an unknown domain variable
    fn get_wireref_root_domain(&mut self, wire_ref: &WireReference) -> Option<DomainType> {
        match &wire_ref.root {
            WireReferenceRoot::LocalDecl(id) => {
                Some(self.instructions[*id].unwrap_declaration().domain.get())
            }
            WireReferenceRoot::LocalInterface(id) => {
                let interface = self.instructions[*id].unwrap_interface();

                Some(interface.domain)
            }
            WireReferenceRoot::LocalSubmodule(local_submod) => {
                let submod = self.instructions[*local_submod].unwrap_submodule();
                let submod_ref = self.globals.get_declared_submodule(submod);
                let local_domain_map = submod.local_domain_map.get().unwrap();
                if local_domain_map.len() == 1 {
                    let [singular_domain] = local_domain_map.cast_to_array();
                    return Some(*singular_domain);
                }

                for p in &wire_ref.path {
                    if let WireReferencePathElement::FieldAccess { refers_to, .. } = p {
                        match refers_to.get() {
                            Some(PathElemRefersTo::Interface(_, Some(interface))) => {
                                if let Some(domain_in_submod) =
                                    submod_ref.md.interfaces[*interface].domain
                                {
                                    return Some(local_domain_map[domain_in_submod]);
                                }
                            }
                            Some(PathElemRefersTo::Interface(_, None)) | None => {}
                        };
                    }
                }
                None
            }
            WireReferenceRoot::NamedConstant(global_ref) => {
                self.global_ref_must_be_generative(global_ref);
                Some(DomainType::Generative)
            }
            WireReferenceRoot::NamedModule(global_ref) => {
                self.global_ref_must_be_generative(global_ref);
                Some(self.domain_checker.alloc_unknown())
            }
            WireReferenceRoot::Error => None,
        }
    }

    /// Used to quickly combine domains with each other. Also performs unification
    pub fn unify_physicals(&mut self, a: (DomainType, Span), b: (DomainType, Span), context: &str) {
        assert!(a.0 != DomainType::Generative);
        assert!(b.0 != DomainType::Generative);
        self.domain_checker.unify_report_error(&b.0, &a.0, b.1, || {
            (
                context.to_string(),
                vec![ErrorInfo {
                    position: a.1,
                    file: self.errors.file,
                    info: "Conflicting with".to_string(),
                }],
            )
        });
    }

    fn global_ref_must_be_generative<ID>(&mut self, global_ref: &GlobalReference<ID>) {
        global_ref.for_each_generative_input(&mut |id| {
            self.must_be_generative(id, "Argument in global reference")
        });
    }

    fn written_type_must_be_generative(&mut self, wr_typ: &WrittenType) {
        wr_typ.for_each_generative_input(&mut |id| self.must_be_generative(id, "Argument in type"));
    }

    /// `expr_id` must point to a [SingleOutputExpression]
    fn must_be_generative(&self, expr_id: FlatID, context: &str) {
        let expr = self.instructions[expr_id].unwrap_subexpression();
        if expr.domain != DomainType::Generative {
            self.errors.error(
                expr.span,
                format!("{context} must be a compile-time expression"),
            );
        }
    }
}
