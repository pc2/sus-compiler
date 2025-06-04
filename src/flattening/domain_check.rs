use crate::alloc::{IndexExcept, UUIDAllocator};
use crate::errors::{ErrorInfo, ErrorStore};
use crate::flattening::typechecking::RemoteSubModule;
use crate::linker::AFTER_DOMAINCHECK_CP;
use crate::prelude::*;
use crate::typing::type_inference::{FailedUnification, TypeSubstitutor, TypeUnifier};

use super::*;

pub fn domain_check_all(linker: &mut Linker) {
    let module_uuids: Vec<ModuleUUID> = linker.modules.iter().map(|(id, _md)| id).collect();
    for id in module_uuids {
        let md = &mut linker.modules[id];
        let errors = ErrorCollector::from_storage(
            md.link_info.errors.take(),
            md.link_info.file,
            &linker.files,
        );
        let resolved_globals = md.link_info.resolved_globals.take();
        let instructions =
            &mut md.link_info.instructions as *mut FlatAlloc<Instruction, FlatIDMarker>;

        let md = &linker.modules[id];

        let errors = domain_check_link_info(
            linker,
            errors,
            unsafe { &mut *instructions },
            md.domains.get_next_alloc_id(),
        );

        let md = &mut linker.modules[id];
        md.link_info
            .reabsorb_errors_globals((errors, resolved_globals), AFTER_DOMAINCHECK_CP);
    }
}

fn domain_check_link_info<'l>(
    linker: &'l Linker,
    mut errors: ErrorCollector<'l>,
    instructions: &mut FlatAlloc<Instruction, FlatIDMarker>,
    domains_next_alloc_id: DomainID,
) -> ErrorStore {
    let mut domain_substitutor = TypeUnifier::default();

    for (instr, instructions) in instructions.iter_mut_convenient() {
        let mut ctx = DomainCheckingContext {
            linker,
            errors,
            domain_substitutor,
            instructions,
        };
        ctx.check_instr(instr);
        errors = ctx.errors;
        domain_substitutor = ctx.domain_substitutor;
    }

    // Set the remaining domain variables that aren't associated with a module port.
    // We just find domain IDs that haven't been
    let mut leftover_domain_alloc = UUIDAllocator::new_start_from(domains_next_alloc_id);
    for (_, d) in domain_substitutor.iter() {
        if d.get().is_none() {
            assert!(d
                .set(DomainType::Physical(leftover_domain_alloc.alloc()))
                .is_ok());
        }
    }

    for (_, instr) in instructions {
        match instr {
            Instruction::SubModule(sm) => {
                for (_, d) in &mut sm.local_interface_domains {
                    assert!(d.fully_substitute(&domain_substitutor));
                }
            }
            Instruction::Declaration(declaration) => {
                assert!(declaration.domain.fully_substitute(&domain_substitutor));
            }
            Instruction::Expression(expression) => {
                assert!(expression.domain.fully_substitute(&domain_substitutor));
            }
            Instruction::IfStatement(_) | Instruction::ForStatement(_) => {}
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

    errors.into_storage()
}

struct DomainCheckingContext<'l, 'instrs> {
    linker: &'l Linker,
    errors: ErrorCollector<'l>,
    domain_substitutor: TypeUnifier<TypeSubstitutor<DomainType>>,
    instructions: IndexExcept<'instrs, FlatID, Instruction, FlatAlloc<Instruction, FlatIDMarker>>,
}

impl<'l, 'instrs> DomainCheckingContext<'l, 'instrs> {
    fn check_instr(&mut self, instr: &mut Instruction) {
        match instr {
            Instruction::SubModule(sub_module_instance) => {
                sub_module_instance.local_interface_domains = self.linker.modules
                    [sub_module_instance.module_ref.id]
                    .domains
                    .map(|_| self.domain_substitutor.alloc_unknown());
            }
            Instruction::Declaration(declaration) => {
                self.written_type_must_be_generative(&declaration.typ_expr);
                declaration.domain = match declaration.identifier_type {
                    IdentifierType::Local | IdentifierType::State => match declaration.decl_kind {
                        DeclarationKind::RegularPort { domain, .. } => DomainType::Physical(domain),
                        _ => self.domain_substitutor.alloc_unknown(),
                    },
                    IdentifierType::Generative => DomainType::Generative,
                };
                if let Some(latency_spec) = declaration.latency_specifier {
                    self.must_be_generative(latency_spec, "Latency Specifier");
                }
            }
            Instruction::Expression(expression) => {
                let mut total_domain = match &mut expression.source {
                    ExpressionSource::WireRef(wire_ref) => self.get_wireref_root_domain(wire_ref),
                    ExpressionSource::FuncCall(func_call) => {
                        let sm = RemoteSubModule::make(
                            func_call.interface_reference.submodule_decl,
                            &self.instructions,
                            &self.linker.modules,
                        );
                        let interface = sm.get_interface_reference(
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

                expression.domain = total_domain.0;

                if let ExpressionOutput::MultiWrite(writes) = &mut expression.output {
                    for wr in writes {
                        let mut target_domain = self.get_wireref_root_domain(&mut wr.to);

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
                        let expr_domain = (expression.domain, expression.span);
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

                match (if_statement.is_generative, condition.domain.is_generative()) {
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
    fn get_wireref_root_domain(&mut self, wire_ref: &mut WireReference) -> (DomainType, Span) {
        let root_domain = match &mut wire_ref.root {
            WireReferenceRoot::LocalDecl(id) => self.instructions[*id].unwrap_declaration().domain,
            WireReferenceRoot::NamedConstant(global_ref) => {
                self.global_ref_must_be_generative(global_ref);
                DomainType::Generative
            }
            WireReferenceRoot::SubModulePort(port_ref) => {
                let sm = RemoteSubModule::make(
                    port_ref.submodule_decl,
                    &self.instructions,
                    &self.linker.modules,
                );
                sm.get_port(port_ref.port).get_local_domain()
            }
            WireReferenceRoot::Error => DomainType::Generative,
        };

        (root_domain, wire_ref.root_span)
    }

    /// Used to quickly combine domains with each other. Also performs unification
    pub fn unify_physicals(&mut self, a: (DomainType, Span), b: (DomainType, Span), context: &str) {
        assert!(a.0 != DomainType::Generative);
        assert!(b.0 != DomainType::Generative);
        self.domain_substitutor
            .unify_report_error(&b.0, &a.0, b.1, || {
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
        if !expr.domain.is_generative() {
            self.errors.error(
                expr.span,
                format!("{context} must be a compile-time expression"),
            );
        }
    }
}
