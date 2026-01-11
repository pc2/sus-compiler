use crate::prelude::*;

use crate::typing::domain_type::DomainTypeRef;

use super::*;

impl<'l> TypeCheckingContext<'l> {
    pub fn domain_check_instr(&self, instr: &'l Instruction) {
        match instr {
            Instruction::SubModule(sub_module_instance) => {
                sub_module_instance
                    .local_domain_map
                    .set(
                        self.globals
                            .get_module(sub_module_instance.module_ref.id)
                            .domains
                            .map(|_| DomainID::UNKNOWN),
                    )
                    .unwrap();
            }
            Instruction::Declaration(declaration) => {
                self.written_type_must_be_generative(&declaration.typ_expr);
                if let Some(latency_spec) = declaration.latency_specifier {
                    self.must_be_generative(latency_spec, "Latency Specifier");
                }
            }
            Instruction::Expression(expr) => {
                let mut total_physical_domain =
                    if let ExpressionSource::WireRef(wire_ref) = &expr.source {
                        match self.get_wireref_root_domain(wire_ref) {
                            Some(DomainTypeRef::Physical(phys)) => Some((phys, wire_ref.root_span)),
                            None | Some(DomainTypeRef::Generative) => None,
                        }
                    } else {
                        None
                    };

                expr.source.for_each_input_wire(&mut |id| {
                    let expr = self.instructions[id].unwrap_subexpression();
                    let expr_domain = expr.domain.unwrap();

                    match (total_physical_domain, expr_domain) {
                        (None, DomainType::Physical(phys)) => {
                            total_physical_domain = Some((phys, expr.span));
                        }
                        (Some(phys_total), DomainType::Physical(expr_phys)) => {
                            self.unify_physicals(phys_total, (expr_phys, expr.span), "expression");
                        }
                        (None, DomainType::Generative) | (Some(_), DomainType::Generative) => {} // No conflict
                    }
                });

                // Function call "writes" also require the condition domain
                if let ExpressionSource::FuncCall(fc) = &expr.source {
                    let call_expr = self.instructions[fc.func_wire_ref].unwrap_subexpression();
                    if !call_expr.domain.unwrap().is_generative() {
                        if let Some(condition_domain) =
                            self.get_condition_domain(expr.parent_condition)
                        {
                            if let Some(phys_total) = total_physical_domain {
                                self.unify_physicals(
                                    phys_total,
                                    condition_domain,
                                    "the runtime condition for function calls",
                                );
                            }
                        }
                    } else {
                        // TODO generative function calls
                    }
                }

                let resulting_domain = match total_physical_domain {
                    Some(phys) => DomainType::Physical(self.unifier.clone_unify(phys.0)),
                    None => DomainType::Generative,
                };
                expr.domain.set_initial(resulting_domain);

                // Regular "writes"
                if let ExpressionOutput::MultiWrite(writes) = &expr.output {
                    for wr in writes {
                        let mut target_domain =
                            self.get_wireref_root_domain(&wr.to).unwrap_or_else(|| {
                                DomainTypeRef::Physical(
                                    self.extra_allocator.alloc(DomainID::UNKNOWN),
                                )
                            });
                        let mut target_span = wr.to.root_span;

                        match wr.write_modifiers {
                            WriteModifiers::Connection { .. } => {
                                if let Some(condition_domain) =
                                    self.get_condition_domain(expr.parent_condition)
                                    && let DomainTypeRef::Physical(target_phys) = &target_domain
                                {
                                    let target_phys = self.extra_allocator.alloc(target_phys);
                                    self.unify_physicals(
                                        (target_phys, target_span),
                                        condition_domain,
                                        "the runtime condition",
                                    );
                                }
                            }
                            WriteModifiers::Initial { initial_kw_span } => {
                                target_domain = DomainTypeRef::Generative;
                                target_span = initial_kw_span;
                            }
                        }
                        let target_domain = match target_domain {
                            DomainTypeRef::Generative => DomainType::Generative,
                            DomainTypeRef::Physical(phys) => {
                                DomainType::Physical(self.unifier.clone_unify(phys))
                            }
                        };
                        wr.target_domain.set_initial(target_domain);
                        let target_domain = wr.target_domain.unwrap();

                        wr.to.for_each_input_wire_in_path(&mut |id| {
                            let expr = self.instructions[id].unwrap_subexpression();
                            self.write_to_domain(
                                target_domain,
                                target_span,
                                expr.domain.unwrap(),
                                expr.span,
                            );
                        });

                        self.write_to_domain(
                            target_domain,
                            target_span,
                            expr.domain.unwrap(),
                            expr.span,
                        );
                    }
                }
            }
            Instruction::IfStatement(if_statement) => {
                let condition = self.instructions[if_statement.condition].unwrap_subexpression();

                match (if_statement.is_generative, condition.domain.unwrap()) {
                    (true, DomainType::Physical(_)) => {
                        self.errors.error(
                            if_statement.if_keyword_span,
                            "Used 'if' in a non generative context, use 'when' instead",
                        );
                    }
                    (false, DomainType::Generative) => {
                        self.errors.error(
                            if_statement.if_keyword_span,
                            "Used 'when' in a generative context, use 'if' instead",
                        );
                    }
                    (_, _) => (),
                }

                // Ensure all bindings are in the condition's domain
                if let DomainType::Physical(phys_condition) = condition.domain.unwrap() {
                    for b in if_statement.iter_all_bindings() {
                        let binding_decl = self.link_info.instructions[b].unwrap_declaration();
                        // If the binding was generative, then that should have been its own error.
                        if let DomainType::Physical(binding_domain) = &binding_decl.domain {
                            self.unify_physicals(
                                (phys_condition, condition.span),
                                (binding_domain, binding_decl.decl_span),
                                "conditional binding",
                            );
                        }
                    }
                }
            }
            Instruction::ForStatement(for_statement) => {
                self.must_be_generative(for_statement.start, "For Loop start");
                self.must_be_generative(for_statement.end, "For Loop end");
            }
            Instruction::Interface(_) => {}
        }
    }

    fn write_to_domain(
        &self,
        target_domain: &'l DomainType,
        target_span: Span,
        expr_domain: &'l DomainType,
        expr_span: Span,
    ) {
        match (target_domain, expr_domain) {
            (DomainType::Generative, DomainType::Generative) => {} // Okay
            (DomainType::Physical(_target_phys), DomainType::Generative) => {} // Okay
            (DomainType::Generative, DomainType::Physical(_expr_phys)) => {
                self.errors
                    .error(
                        expr_span,
                        "Attempting to write a runtime value to a generative target",
                    )
                    .info(target_span, "This is a generative target");
            }
            (DomainType::Physical(target_phys), DomainType::Physical(expr_phys)) => {
                self.unify_physicals(
                    (target_phys, target_span),
                    (expr_phys, expr_span),
                    "assignment",
                );
            }
        }
    }

    /// Returns the physical domain that would be forced by a parent `when` condition.
    fn get_condition_domain(
        &self,
        mut parent_condition: Option<ParentCondition>,
    ) -> Option<(&'l UniCell<DomainID>, Span)> {
        while let Some(p_cond) = parent_condition {
            match &self.instructions[p_cond.parent_when] {
                Instruction::Interface(decl) => return Some((&decl.domain, decl.name_span)),
                Instruction::IfStatement(when) => {
                    let when_cond_expr = self.instructions[when.condition].unwrap_subexpression();
                    if let DomainType::Physical(when_cond_physical) = when_cond_expr.domain.unwrap()
                    {
                        return Some((when_cond_physical, when_cond_expr.span));
                    }
                    parent_condition = when.parent_condition;
                }
                _ => unreachable!(),
            }
        }
        None
    }

    /// [WireReferenceRoot::Error] maps to None, such that in reading context it can be interpreted as [DomainType::Generative], and in writing it can be [DomainType::Physical]. Both
    ///
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
    fn get_wireref_root_domain(&self, wire_ref: &'l WireReference) -> Option<DomainTypeRef<'l>> {
        match &wire_ref.root {
            WireReferenceRoot::LocalDecl(id) => Some(DomainTypeRef::from(
                &self.instructions[*id].unwrap_declaration().domain,
            )),
            WireReferenceRoot::LocalInterface(id) => {
                let interface = self.instructions[*id].unwrap_interface();

                Some(DomainTypeRef::Physical(&interface.domain))
            }
            WireReferenceRoot::LocalSubmodule(local_submod) => {
                let submod = self.instructions[*local_submod].unwrap_submodule();
                let submod_ref = self.globals.get_declared_submodule(submod);
                let local_domain_map = submod.local_domain_map.get().unwrap();
                if local_domain_map.len() == 1 {
                    let [singular_domain] = local_domain_map.cast_to_array();
                    return Some(DomainTypeRef::Physical(singular_domain));
                }

                for p in &wire_ref.path {
                    if let WireReferencePathElement::FieldAccess { refers_to, .. } = p {
                        match refers_to.get() {
                            Some(PathElemRefersTo::Interface(_, Some(interface))) => {
                                if let Some(domain_in_submod) =
                                    submod_ref.md.interfaces[*interface].domain
                                {
                                    return Some(DomainTypeRef::Physical(
                                        &local_domain_map[domain_in_submod],
                                    ));
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
                Some(DomainTypeRef::Generative)
            }
            WireReferenceRoot::NamedModule(global_ref) => {
                self.global_ref_must_be_generative(global_ref);
                Some(DomainTypeRef::Physical(
                    self.extra_allocator.alloc(DomainID::UNKNOWN),
                ))
            }
            WireReferenceRoot::Error => None,
        }
    }

    /// Used to quickly combine domains with each other. Also performs unification
    pub fn unify_physicals(
        &self,
        a: (&'l UniCell<DomainID>, Span),
        b: (&'l UniCell<DomainID>, Span),
        context: &str,
    ) {
        if self.unifier.unify(a.0, b.0) != UnifyResult::Success {
            // Given that there *was* a unification failure, both a and b should be resolvable.
            let a_dom = self.unifier.resolve(a.0).unwrap();
            let b_dom = self.unifier.resolve(b.0).unwrap();

            let expected_name = a_dom.display(self.domains);
            let found_name = b_dom.display(self.domains);
            self.errors
                .error(b.1, format!("Domain error: Attempting to combine domains {found_name} and {expected_name} in {context}"))
                .info(a.1, "Conflicting with");
        }
    }

    fn global_ref_must_be_generative<ID>(&self, global_ref: &'l GlobalReference<ID>) {
        global_ref.for_each_generative_input(&mut |id| {
            self.must_be_generative(id, "Argument in global reference")
        });
    }

    fn written_type_must_be_generative(&self, wr_typ: &'l WrittenType) {
        wr_typ.for_each_generative_input(&mut |id| self.must_be_generative(id, "Argument in type"));
    }

    /// `expr_id` must point to a [SingleOutputExpression]
    fn must_be_generative(&self, expr_id: FlatID, context: &str) {
        let expr = self.instructions[expr_id].unwrap_subexpression();
        if !matches!(expr.domain.unwrap(), DomainType::Generative) {
            self.errors.error(
                expr.span,
                format!("{context} must be a compile-time expression"),
            );
        }
    }

    // ===== Finalization =====

    fn finalize_physical(
        &self,
        domain: &'l UniCell<DomainID>,
        id_alloc: &mut UUIDAllocator<DomainIDMarker>,
    ) {
        if self.unifier.resolve(domain).is_err() {
            self.unifier.set_hard(domain, id_alloc.alloc());
        }
        assert!(self.unifier.fully_substitute(domain));
    }
    fn finalize_domain(
        &self,
        domain: &'l UniCell<DomainType>,
        id_alloc: &mut UUIDAllocator<DomainIDMarker>,
    ) {
        match self.unifier.resolve(domain) {
            Ok(DomainType::Generative) => {}
            Ok(DomainType::Physical(phys)) => {
                self.finalize_physical(phys, id_alloc);
            }
            Err(_) => {
                self.unifier.set_hard(domain, DomainType::Generative);
            }
        }
        assert!(self.unifier.fully_substitute(domain));
    }
    pub fn finalize_domains(&self) {
        let mut unknown_domain_alloc =
            UUIDAllocator::new_start_from(self.domains.get_next_alloc_id());
        for (_, instr) in self.instructions {
            match instr {
                Instruction::SubModule(sm) => {
                    for (_, d) in sm.local_domain_map.get().unwrap() {
                        self.finalize_physical(d, &mut unknown_domain_alloc);
                    }
                }
                Instruction::Declaration(declaration) => match &declaration.domain {
                    DomainType::Generative => {}
                    DomainType::Physical(phys) => {
                        self.finalize_physical(phys, &mut unknown_domain_alloc)
                    }
                },
                Instruction::Expression(expr) => {
                    self.finalize_domain(&expr.domain, &mut unknown_domain_alloc);

                    if let ExpressionOutput::MultiWrite(writes) = &expr.output {
                        for w in writes {
                            self.finalize_domain(&w.target_domain, &mut unknown_domain_alloc);
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
