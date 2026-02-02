use super::*;
use crate::prelude::*;

use crate::typing::unifyable_cell::UnifyRecurse;

// Exceptional use of these, to make the code below a little terser
use ClockDomain::Generative;
use ClockDomain::Physical;

impl<'l> TypeCheckingContext<'l> {
    pub fn domain_check_instr(&self, instr: &'l Instruction) {
        match instr {
            Instruction::SubModule(sub_module_instance) => {
                sub_module_instance
                    .submodule_clock_map
                    .set(
                        self.globals
                            .get_module(sub_module_instance.module_ref.id)
                            .clocks
                            .map(|_| ClockID::UNKNOWN),
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
                // We can safely set it to PLACEHOLDER here. We would only access it *if* we're Physical anyway, and in that case it *must* have been set
                let mut resulting_domain_span = Span::PLACEHOLDER;
                let mut resulting_domain = if let ExpressionSource::WireRef(wire_ref) = &expr.source
                {
                    resulting_domain_span = wire_ref.root_span;
                    self.get_wireref_root_domain(wire_ref).unwrap_or(Generative)
                } else {
                    Generative // We use Generative for any errors, just because it won't cause errors. 
                };

                expr.source.for_each_input_wire(&mut |id| {
                    let expr = self.instructions[id].unwrap_subexpression();
                    let expr_domain = expr.domain.unwrap();

                    match (&mut resulting_domain, expr_domain) {
                        (Generative, Physical(phys)) => {
                            resulting_domain = Physical(self.unifier.clone_unify(phys));
                            resulting_domain_span = expr.span;
                        }
                        (Physical(phys_total), Physical(expr_phys)) => {
                            self.set_physicals(
                                phys_total,
                                resulting_domain_span,
                                expr_phys,
                                expr.span,
                                "expression",
                            );
                        }
                        (Generative, Generative) | (Physical(_), Generative) => {} // No conflict
                    }
                });

                // Function call "writes" also require the condition domain
                if let ExpressionSource::FuncCall(fc) = &expr.source {
                    let call_expr = self.instructions[fc.func_wire_ref].unwrap_subexpression();
                    if !call_expr.domain.unwrap().is_generative() {
                        if let Some(condition_domain) =
                            self.get_condition_domain(expr.parent_condition)
                        {
                            if let Physical(phys_total) = &mut resulting_domain {
                                self.set_physicals(
                                    phys_total,
                                    resulting_domain_span,
                                    condition_domain.0,
                                    condition_domain.1,
                                    "the runtime condition for function calls",
                                );
                            }
                        }
                    } else {
                        // TODO generative function calls
                    }
                }

                // Remove mutability
                let resulting_domain = resulting_domain;
                expr.clock_domain.set_initial(resulting_domain);

                // Regular "writes"
                if let ExpressionOutput::MultiWrite(writes) = &expr.output {
                    for wr in writes {
                        let mut target_domain: ClockDomain = self
                            .get_wireref_root_domain(&wr.to)
                            .unwrap_or(Physical(ClockID::UNKNOWN));
                        let mut target_span = wr.to.root_span;

                        match wr.write_modifiers {
                            WriteModifiers::Connection { .. } => {
                                if let Some(condition_domain) =
                                    self.get_condition_domain(expr.parent_condition)
                                    && let Physical(target_phys) = &mut target_domain
                                {
                                    self.set_physicals(
                                        target_phys,
                                        target_span,
                                        condition_domain.0,
                                        condition_domain.1,
                                        "the runtime condition",
                                    );
                                }
                            }
                            WriteModifiers::Initial { initial_kw_span } => {
                                target_domain = Generative;
                                target_span = initial_kw_span;
                            }
                        }
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
                            expr.clock_domain.unwrap(),
                            expr.span,
                        );
                    }
                }
            }
            Instruction::IfStatement(if_statement) => {
                let condition = self.instructions[if_statement.condition].unwrap_subexpression();

                match (if_statement.is_generative, condition.domain.unwrap()) {
                    (true, Physical(_)) => {
                        self.errors.error(
                            if_statement.if_keyword_span,
                            "Used 'if' in a non generative context, use 'when' instead",
                        );
                    }
                    (false, Generative) => {
                        self.errors.error(
                            if_statement.if_keyword_span,
                            "Used 'when' in a generative context, use 'if' instead",
                        );
                    }
                    (_, _) => (),
                }

                // Ensure all bindings are in the condition's domain
                if let Physical(phys_condition) = condition.domain.unwrap() {
                    for b in if_statement.iter_all_bindings() {
                        let binding_decl = self.link_info.instructions[b].unwrap_declaration();
                        // If the binding was generative, then that should have been its own error.
                        if let Physical(binding_domain) = &binding_decl.clock_domain {
                            self.unify_physicals(
                                phys_condition,
                                condition.span,
                                binding_domain,
                                binding_decl.decl_span,
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
        target_domain: &'l ClockDomain,
        target_span: Span,
        expr_domain: &'l ClockDomain,
        expr_span: Span,
    ) {
        match (target_domain, expr_domain) {
            (Generative, Generative) => {}             // Okay
            (Physical(_target_phys), Generative) => {} // Okay
            (Generative, Physical(_expr_phys)) => {
                self.errors
                    .error(
                        expr_span,
                        "Attempting to write a runtime value to a generative target",
                    )
                    .info(target_span, "This is a generative target");
            }
            (Physical(target_phys), Physical(expr_phys)) => {
                self.unify_physicals(target_phys, target_span, expr_phys, expr_span, "assignment");
            }
        }
    }

    /// Returns the physical domain that would be forced by a parent `when` condition.
    fn get_condition_domain(
        &self,
        mut parent_condition: Option<ParentCondition>,
    ) -> Option<(&'l UniCell<ClockID>, Span)> {
        while let Some(p_cond) = parent_condition {
            match &self.instructions[p_cond.parent_when] {
                Instruction::Interface(decl) => return Some((&decl.clock_domain, decl.name_span)),
                Instruction::IfStatement(when) => {
                    let when_cond_expr = self.instructions[when.condition].unwrap_subexpression();
                    if let Physical(when_cond_physical) = when_cond_expr.domain.unwrap() {
                        return Some((when_cond_physical, when_cond_expr.span));
                    }
                    parent_condition = when.parent_condition;
                }
                _ => unreachable!(),
            }
        }
        None
    }

    /// [WireReferenceRoot::Error] maps to None, such that in reading context it can be interpreted as [Generative], and in writing it can be [Physical]. Both
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
    fn get_wireref_root_domain(&self, wire_ref: &'l WireReference) -> Option<ClockDomain> {
        match &wire_ref.root {
            WireReferenceRoot::LocalDecl(id) => Some(
                self.unifier
                    .clone_known(&self.instructions[*id].unwrap_declaration().clock_domain),
            ),
            WireReferenceRoot::LocalInterface(id) => {
                let interface = self.instructions[*id].unwrap_interface();

                Some(Physical(self.unifier.clone_unify(&interface.clock_domain)))
            }
            WireReferenceRoot::LocalSubmodule(local_submod) => {
                let submod = self.instructions[*local_submod].unwrap_submodule();
                let submod_ref = self.globals.get_declared_submodule(submod);
                let submodule_clock_map = submod.submodule_clock_map.get().unwrap();
                if submodule_clock_map.len() == 1 {
                    let [singular_domain] = submodule_clock_map.cast_to_array();
                    return Some(Physical(self.unifier.clone_unify(singular_domain)));
                }

                for p in &wire_ref.path {
                    if let WireReferencePathElement::FieldAccess { refers_to, .. } = p {
                        match refers_to.get() {
                            Some(PathElemRefersTo::Interface(_, Some(interface))) => {
                                let interf = &submod_ref.md.interfaces[*interface];
                                if let Some(domain_in_submod) = interf.clock {
                                    return Some(Physical(
                                        self.unifier
                                            .clone_unify(&submodule_clock_map[domain_in_submod]),
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
                Some(Generative)
            }
            WireReferenceRoot::NamedModule(global_ref) => {
                self.global_ref_must_be_generative(global_ref);
                Some(Physical(ClockID::UNKNOWN))
            }
            WireReferenceRoot::Error => None,
        }
    }

    /// Used to quickly combine domains with each other. Also performs unification
    pub fn unify_physicals(
        &self,
        a_dom: &'l UniCell<ClockID>,
        a_span: Span,
        b_dom: &'l UniCell<ClockID>,
        b_span: Span,
        context: &str,
    ) {
        if self.unifier.unify(a_dom, b_dom) != UnifyResult::Success {
            self.report_domains_error(a_dom, a_span, b_dom, b_span, context);
        }
    }

    /// Used to quickly combine domains with each other. Also performs unification
    pub fn set_physicals(
        &self,
        a_dom: &mut UniCell<ClockID>,
        a_span: Span,
        b_dom: &'l UniCell<ClockID>,
        b_span: Span,
        context: &str,
    ) {
        // I swapped the arguments here, because I didn't want the error message to change. (It's more intuitive right now)
        if self.unifier.set(b_dom, a_dom) != UnifyResult::Success {
            self.report_domains_error(a_dom, a_span, b_dom, b_span, context);
        }
    }

    fn report_domains_error(
        &self,
        a_dom: &UniCell<ClockID>,
        a_span: Span,
        b_dom: &UniCell<ClockID>,
        b_span: Span,
        context: &str,
    ) {
        // Given that there *was* a unification failure, both a and b should be resolvable.
        assert!(self.unifier.fully_substitute(a_dom));
        assert!(self.unifier.fully_substitute(b_dom));

        let expected_name = a_dom.display(self.domains);
        let found_name = b_dom.display(self.domains);
        self.errors
            .error(b_span, format!("Domain error: Attempting to combine domains {found_name} and {expected_name} in {context}"))
            .info(a_span, "Conflicting with");
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
        if !matches!(expr.domain.unwrap(), Generative) {
            self.errors.error(
                expr.span,
                format!("{context} must be a compile-time expression"),
            );
        }
    }

    // ===== Finalization =====

    fn finalize_physical(
        &self,
        domain: &'l UniCell<ClockID>,
        id_alloc: &mut UUIDAllocator<ClockIDMarker>,
    ) {
        if self.unifier.resolve(domain).is_err() {
            self.unifier.set_hard(domain, id_alloc.alloc());
        }
        assert!(self.unifier.fully_substitute(domain));
    }
    fn finalize_domain(
        &self,
        domain: &'l UniCell<ClockDomain>,
        id_alloc: &mut UUIDAllocator<ClockIDMarker>,
    ) {
        match self.unifier.resolve(domain) {
            Ok(Generative) => {}
            Ok(Physical(phys)) => {
                self.finalize_physical(phys, id_alloc);
            }
            Err(_) => {
                self.unifier.set_hard(domain, Generative);
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
                    for (_, d) in sm.submodule_clock_map.get().unwrap() {
                        self.finalize_physical(d, &mut unknown_domain_alloc);
                    }
                }
                Instruction::Declaration(declaration) => match &declaration.clock_domain {
                    Generative => {}
                    Physical(phys) => self.finalize_physical(phys, &mut unknown_domain_alloc),
                },
                Instruction::Expression(expr) => {
                    self.finalize_domain(&expr.clock_domain, &mut unknown_domain_alloc);

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
