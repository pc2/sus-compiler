use crate::alloc::UUIDAllocator;
use crate::errors::ErrorInfo;
use crate::linker::{GlobalUUID, AFTER_DOMAIN_CHECK_CP};
use crate::prelude::*;
use crate::typing::type_inference::{FailedUnification, TypeSubstitutor, TypeUnifier};

use super::*;

pub fn domain_check_all(linker: &mut Linker) {
    let module_uuids: Vec<ModuleUUID> = linker.modules.iter().map(|(id, _md)| id).collect();
    for module_uuid in module_uuids {
        let domain_substitutor = linker.immutable_pass(
            "Domain Check",
            GlobalUUID::Module(module_uuid),
            |link_info, errors, globals| {
                let mut ctx = DomainCheckingContext {
                    globals,
                    errors,
                    domain_checker: TypeUnifier::default(),
                    instructions: &link_info.instructions,
                };

                for (_, instr) in ctx.instructions {
                    ctx.check_instr(instr);
                }

                ctx.domain_checker
            },
        );
        let md = &mut linker.modules[module_uuid];
        // Set the remaining domain variables that aren't associated with a module port.
        // We just find domain IDs that haven't been
        let mut leftover_domain_alloc =
            UUIDAllocator::new_start_from(md.domains.get_next_alloc_id());
        for (_, d) in domain_substitutor.iter() {
            if d.get().is_none() {
                assert!(d
                    .set(DomainType::Physical(leftover_domain_alloc.alloc()))
                    .is_ok());
            }
        }

        let errors = md.link_info.take_errors(&linker.files);
        finalize_domains(&errors, &mut md.link_info.instructions, domain_substitutor);
        md.link_info.reabsorb_errors(errors.into_storage());
        md.link_info.checkpoint(AFTER_DOMAIN_CHECK_CP);
    }
}

fn finalize_domains(
    errors: &ErrorCollector,
    instructions: &mut FlatAlloc<Instruction, FlatIDMarker>,
    mut domain_substitutor: TypeUnifier<TypeSubstitutor<DomainType>>,
) {
    for (_, instr) in instructions {
        match instr {
            Instruction::SubModule(sm) => {
                for (_, d) in sm.local_interface_domains.get_mut() {
                    assert!(d.fully_substitute(&domain_substitutor));
                }
            }
            Instruction::Declaration(declaration) => {
                assert!(declaration
                    .domain
                    .get_mut()
                    .fully_substitute(&domain_substitutor));
            }
            Instruction::Expression(expression) => {
                assert!(expression
                    .domain
                    .get_mut()
                    .fully_substitute(&domain_substitutor));
            }
            Instruction::IfStatement(_)
            | Instruction::ForStatement(_)
            | Instruction::ActionTriggerDeclaration(_) => {}
        }
    }

    for FailedUnification {
        mut found,
        mut expected,
        span,
        context,
        infos,
    } in domain_substitutor.extract_errors()
    {
        let _ = found.fully_substitute(&domain_substitutor);
        let _ = expected.fully_substitute(&domain_substitutor);

        let expected_name = format!("{expected:?}");
        let found_name = format!("{found:?}");
        errors
            .error(span, format!("Domain error: Attempting to combine domains {found_name} and {expected_name} in {context}"))
            .add_info_list(infos);

        assert_ne!(found, expected);

        /*assert!(
            expected_name != found_name,
            "{expected_name} != {found_name}"
        );*/
    }
}

impl<'l> DomainCheckingContext<'l> {
    fn check_instr(&mut self, instr: &Instruction) {
        match instr {
            Instruction::SubModule(sub_module_instance) => {
                sub_module_instance.local_interface_domains.set(
                    self.globals.globals[sub_module_instance.module_ref.id]
                        .domains
                        .map(|_| self.domain_checker.alloc_unknown()),
                );
            }
            Instruction::Declaration(declaration) => {
                self.written_type_must_be_generative(&declaration.typ_expr);
                declaration.domain.set(match declaration.decl_kind {
                    DeclarationKind::Port { domain, .. } => DomainType::Physical(domain),
                    DeclarationKind::StructField(..)
                    | DeclarationKind::RegularWire { .. }
                    | DeclarationKind::ConditionalBinding { .. } => {
                        self.domain_checker.alloc_unknown()
                    }
                    DeclarationKind::RegularGenerative { .. }
                    | DeclarationKind::TemplateParameter { .. } => DomainType::Generative,
                });
                if let Some(latency_spec) = declaration.latency_specifier {
                    self.must_be_generative(latency_spec, "Latency Specifier");
                }
            }
            Instruction::Expression(expression) => {
                let mut total_domain = match &expression.source {
                    ExpressionSource::WireRef(wire_ref) => self.get_wireref_root_domain(wire_ref),
                    ExpressionSource::FuncCall(func_call) => {
                        let interface = self
                            .globals
                            .get_submodule(func_call.interface_reference.submodule_decl)
                            .get_interface_reference(
                                func_call.interface_reference.submodule_interface,
                            );
                        (
                            interface.get_local_domain(),
                            func_call.interface_reference.interface_span,
                        )
                    }
                    _ => (DomainType::Generative, Span::MAX_POSSIBLE_SPAN),
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

                expression.domain.set(total_domain.0);

                if let ExpressionOutput::MultiWrite(writes) = &expression.output {
                    for wr in writes {
                        let mut target_domain = self.get_wireref_root_domain(&wr.to);

                        match wr.write_modifiers {
                            WriteModifiers::Connection { .. } => {
                                if let Some(condition_domain) =
                                    self.get_condition_domain(expression.parent_condition)
                                {
                                    if target_domain.0 != DomainType::Generative {
                                        self.unify_physicals(
                                            target_domain,
                                            condition_domain,
                                            "the runtime condition",
                                        );
                                    }
                                }
                            }
                            WriteModifiers::Initial { initial_kw_span } => {
                                target_domain = (DomainType::Generative, initial_kw_span);
                            }
                        }

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
                        self.errors.warn(
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
            Instruction::ActionTriggerDeclaration(_) => {}
        }
    }

    /// Output is guaranteed not to be DomainType::Generative
    fn get_condition_domain(
        &self,
        mut parent_condition: Option<ParentCondition>,
    ) -> Option<(DomainType, Span)> {
        while let Some(p_cond) = parent_condition {
            let when = self.instructions[p_cond.parent_when].unwrap_if();
            let when_cond_expr = self.instructions[when.condition].unwrap_subexpression();
            if when_cond_expr.domain != DomainType::Generative {
                return Some((when_cond_expr.domain, when_cond_expr.span));
            }
            parent_condition = when.parent_condition;
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
    ///     The domains combine to form the lowest common denominator.
    ///     If all are generative this becomes generative
    ///     At least one non-generative domain makes the whole thing non-generative
    ///     It should be supplied with a generative output_typ domain when generative, and an unknown domain variable otherwise
    /// - Writing:
    ///     The output_typ domain should be generative when wire_ref.root is generative, or a generative value is required such as with "initial"
    ///     When wire_ref.root is not generative, it should be an unknown domain variable
    fn get_wireref_root_domain(&mut self, wire_ref: &WireReference) -> (DomainType, Span) {
        let root_domain = match &wire_ref.root {
            WireReferenceRoot::LocalDecl(id) => {
                self.instructions[*id].unwrap_declaration().domain.get()
            }
            WireReferenceRoot::NamedConstant(global_ref) => {
                self.global_ref_must_be_generative(global_ref);
                DomainType::Generative
            }
            WireReferenceRoot::SubModulePort(port_ref) => self
                .globals
                .get_submodule(port_ref.submodule_decl)
                .get_port(port_ref.port)
                .get_local_domain(),
            WireReferenceRoot::Error => DomainType::Generative,
        };

        (root_domain, wire_ref.root_span)
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
