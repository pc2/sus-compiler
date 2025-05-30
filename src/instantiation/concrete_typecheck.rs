use ibig::IBig;
use sus_proc_macro::get_builtin_type;

use crate::alloc::{zip_eq, zip_eq3};
use crate::typing::set_unifier::{DelayedErrorCollector, FullySubstitutable};
use crate::typing::value_unifier::{ValueErrorReporter, ValueUnifierStore};
use crate::typing::{concrete_type::ConcreteType, value_unifier::ValueUnifier};

use super::*;

macro_rules! unifier_constraint_ints {
    ($unifier:ident, [$($var:ident),+], $body:block) => {
        $unifier.add_constraint([$($var),+], move |$unifier| {
            $(let $var = $unifier.unwrap_known($var).unwrap_integer();)+
            $body
        })
    };
}

macro_rules! assert_due_to_variable_clones {
    ($cond:expr) => {
        assert!($cond, "This assertion cannot fail, because the variables that caused the unification failure should have been cloned in execute")
    };
}

fn unify_rank<'inst>(
    rank: &'inst [UnifyableValue],
    mut typ: &'inst ConcreteType,
    unifier: &mut ValueUnifier<'inst>,
) -> bool {
    rank.iter().all(|r| {
        let_unwrap!(ConcreteType::Array(arr), typ);
        typ = &arr.0;
        unifier.unify(r, &arr.1)
    })
}

impl<'inst, 'l: 'inst> ModuleTypingContext<'l> {
    pub fn typecheck(&mut self, type_substitutor_alloc: ValueUnifierAlloc) {
        let error_reporter = DelayedErrorCollector::new();

        let mut unifier = ValueUnifier::from_alloc(type_substitutor_alloc);

        self.typecheck_all_wires(&mut unifier, &error_reporter);
        self.typecheck_all_submodules(&mut unifier);

        unifier.execute_ready_constraints();

        loop {
            self.infer_parameters_for_latencies(&mut unifier);
            if !unifier.execute_ready_constraints() {
                break;
            }
        }

        let substitutor = unifier.decomission();

        error_reporter.report(&substitutor);

        self.finalize(&substitutor);

        self.compute_latencies(&substitutor);
    }
    fn typecheck_all_wires(
        &'inst self,
        unifier: &mut ValueUnifier<'inst>,
        errors: &ValueErrorReporter<'inst>,
    ) {
        for (_, out) in &self.wires {
            let original_instr = &self.link_info.instructions[out.original_instruction];
            let span = original_instr.get_span();
            span.debug();

            match &out.source {
                RealWireDataSource::ReadOnly => {}
                RealWireDataSource::Multiplexer {
                    is_state: _,
                    sources,
                } => {
                    unifier.create_subtype_constraint(sources.iter().map(|s| {
                        let source_wire = &self.wires[s.from];
                        let destination_typ = out.typ.walk_path(&s.to_path);
                        (destination_typ, &source_wire.typ)
                    }));
                }
                RealWireDataSource::UnaryOp { op, rank, right } => {
                    // TODO overloading
                    let right = &self.wires[*right];
                    let out_root = out.typ.walk_rank(rank.len());
                    let right_root = right.typ.walk_rank(rank.len());
                    assert_due_to_variable_clones!(unify_rank(rank, &out.typ, unifier));
                    if !unify_rank(rank, &right.typ, unifier) {
                        errors.error(|substitutor| {
                            self.errors
                                .error(right.get_span(self.link_info), format!("Incompatible multi-rank for higher-rank operator: Found {} but output is {}",
                                right.typ.display_substitute(self.linker, substitutor),
                                out.typ.display_substitute(self.linker, substitutor))
                            );
                        });
                    }
                    match op {
                        UnaryOperator::Not => {
                            assert_eq!(right_root.unwrap_named().id, get_builtin_type!("bool"));
                            assert_eq!(out_root.unwrap_named().id, get_builtin_type!("bool"));
                        }
                        UnaryOperator::And | UnaryOperator::Or | UnaryOperator::Xor => {
                            assert_eq!(
                                right_root.unwrap_array().0.unwrap_named().id,
                                get_builtin_type!("bool")
                            );
                            assert_eq!(out_root.unwrap_named().id, get_builtin_type!("bool"));
                        }
                        UnaryOperator::Negate => {
                            let [min, max] = right_root.get_value_args(get_builtin_type!("int"));

                            unifier_constraint_ints!(unifier, [min, max], {
                                assert!(right_root.set_named_template_args(
                                    get_builtin_type!("int"),
                                    unifier,
                                    [-(max.clone()), -(min.clone())]
                                ));
                            });
                        }
                        UnaryOperator::Sum => {
                            let (content, sz) = right_root.unwrap_array();
                            let [min, max] = content.get_value_args(get_builtin_type!("int"));

                            unifier_constraint_ints!(unifier, [min, max, sz], {
                                assert!(out_root.set_named_template_args(
                                    get_builtin_type!("int"),
                                    unifier,
                                    [min * sz, max * sz]
                                ));
                            });
                        }
                        UnaryOperator::Product => {
                            let (content, sz) = right_root.unwrap_array();
                            let [min, max] = content.get_value_args(get_builtin_type!("int"));

                            unifier_constraint_ints!(unifier, [min, max, sz], {
                                let sz = usize::try_from(sz).unwrap();
                                assert!(out_root.set_named_template_args(
                                    get_builtin_type!("int"),
                                    unifier,
                                    [min.pow(sz), max.pow(sz)]
                                ));
                            });
                        }
                    }
                }
                RealWireDataSource::BinaryOp {
                    op,
                    rank,
                    left,
                    right,
                } => {
                    let left = &self.wires[*left];
                    let right = &self.wires[*right];
                    let out_root = out.typ.walk_rank(rank.len());
                    let left_root = left.typ.walk_rank(rank.len());
                    let right_root = right.typ.walk_rank(rank.len());
                    assert_due_to_variable_clones!(unify_rank(rank, &out.typ, unifier));
                    if !unify_rank(rank, &left.typ, unifier) {
                        errors.error(|substitutor| {
                            self.errors
                                .error(left.get_span(self.link_info), format!("Incompatible multi-rank for higher-rank operator: Found {} but output is {}",
                                left.typ.display_substitute(self.linker, substitutor),
                                out.typ.display_substitute(self.linker, substitutor))
                            ).info_same_file(right.get_span(self.link_info), format!("Right argument has type {}", right.typ.display_substitute(self.linker, substitutor)));
                        });
                    }
                    if !unify_rank(rank, &right.typ, unifier) {
                        errors.error(|substitutor| {
                            self.errors
                                .error(right.get_span(self.link_info), format!("Incompatible multi-rank for higher-rank operator: Found {} but output is {}",
                                right.typ.display_substitute(self.linker, substitutor),
                                out.typ.display_substitute(self.linker, substitutor))
                            ).info_same_file(left.get_span(self.link_info), format!("Left argument has type {}", left.typ.display_substitute(self.linker, substitutor)));
                        });
                    }
                    // TODO overloading
                    // Typecheck generic INT
                    match op {
                        BinaryOperator::And | BinaryOperator::Or | BinaryOperator::Xor => {
                            assert_eq!(left_root.unwrap_named().id, get_builtin_type!("bool"));
                            assert_eq!(right_root.unwrap_named().id, get_builtin_type!("bool"));
                            assert_eq!(out_root.unwrap_named().id, get_builtin_type!("bool"));
                        }
                        BinaryOperator::Add
                        | BinaryOperator::Subtract
                        | BinaryOperator::Multiply
                        | BinaryOperator::Divide
                        | BinaryOperator::Modulo => {
                            let [left_min, left_max] =
                                left_root.get_value_args(get_builtin_type!("int"));
                            let [right_min, right_max] =
                                right_root.get_value_args(get_builtin_type!("int"));
                            unifier_constraint_ints!(
                                unifier,
                                [left_min, left_max, right_min, right_max],
                                {
                                    let (out_min, out_max) = match op {
                                        BinaryOperator::Add => {
                                            (right_min + left_min, right_max + left_max)
                                        }
                                        BinaryOperator::Subtract => {
                                            (left_min - right_max, left_max - right_min)
                                        }
                                        BinaryOperator::Multiply => {
                                            let potentials = [
                                                left_min * right_min,
                                                left_min * right_max,
                                                left_max * right_min,
                                                left_max * right_max,
                                            ];
                                            (
                                                potentials.iter().min().unwrap().clone(),
                                                potentials.iter().max().unwrap().clone(),
                                            )
                                        }
                                        BinaryOperator::Divide => {
                                            if right_min <= &IBig::from(0)
                                                && right_max >= &IBig::from(0)
                                            {
                                                self.errors.error(right.get_span(self.link_info), format!("Divisor may not possibly be == 0, but its range is {right_min}..{right_max}"));
                                                (IBig::from(0), IBig::from(0))
                                            } else {
                                                let potentials = [
                                                    left_min / right_max,
                                                    left_min / right_min,
                                                    left_max / right_max,
                                                    left_max / right_min,
                                                ];
                                                (
                                                    potentials.iter().min().unwrap().clone(),
                                                    potentials.iter().max().unwrap().clone(),
                                                )
                                            }
                                        }
                                        BinaryOperator::Modulo => {
                                            if !right_min > IBig::from(0) {
                                                self.errors.error(right.get_span(self.link_info), format!("Modulo divisor must be > 0, but its range is {right_min}..{right_max}"));
                                                (IBig::from(0), IBig::from(0))
                                            } else {
                                                (IBig::from(0), right_max - IBig::from(1))
                                            }
                                        }
                                        _ => {
                                            unreachable!("The BinaryOpTypecheckConstraint should only check arithmetic operations but got {}", op);
                                        }
                                    };
                                    assert!(out.typ.set_named_template_args(
                                        get_builtin_type!("int"),
                                        unifier,
                                        [out_min, out_max]
                                    ))
                                }
                            );
                        }
                        BinaryOperator::Equals | BinaryOperator::NotEquals => {
                            if !unifier.unify_concrete::<false>(left_root, right_root) {
                                errors.error(move |substitutor| {
                                    self.errors.error(
                                        span,
                                        format!(
                                            "Typecheck error: Found {}, but expected {}",
                                            right_root.display_substitute(self.linker, substitutor),
                                            left_root.display_substitute(self.linker, substitutor)
                                        ),
                                    );
                                });
                            }
                            assert_eq!(out_root.unwrap_named().id, get_builtin_type!("bool"));
                        }
                        BinaryOperator::GreaterEq
                        | BinaryOperator::Greater
                        | BinaryOperator::LesserEq
                        | BinaryOperator::Lesser => {
                            assert_eq!(left_root.unwrap_named().id, get_builtin_type!("int"));
                            assert_eq!(right_root.unwrap_named().id, get_builtin_type!("int"));
                            assert_eq!(out_root.unwrap_named().id, get_builtin_type!("bool"));
                        }
                    }
                }
                RealWireDataSource::Select { root, path } => {
                    let found_typ = self.wires[*root].typ.walk_path(path);
                    let _ = unifier.unify_concrete::<true>(&out.typ, found_typ);
                }
                RealWireDataSource::ConstructArray { array_wires } => {
                    let (array_content_supertyp, array_size) = out.typ.unwrap_array();

                    unifier.create_subtype_constraint(array_wires.iter().map(|w_id| {
                        let w = &self.wires[*w_id];
                        (array_content_supertyp, &w.typ)
                    }));

                    // Error is reported in final_checks
                    let _ = unifier.set(array_size, Value::Integer(IBig::from(array_wires.len())));
                }
                // type is already set when the wire was created
                RealWireDataSource::Constant { value: _ } => {}
            };
        }
    }

    fn typecheck_all_submodules(&'inst self, unifier: &mut ValueUnifier<'inst>) {
        for (_, sm) in &self.submodules {
            assert!(sm.instance.get().is_none());

            // Check if there's any argument that isn't known
            let mut substitutables = Vec::new();
            sm.refers_to
                .template_args
                .gather_all_substitutables(&mut substitutables);

            unifier.add_constraint(substitutables, |unifier| {
                let submod_instr =
                    self.link_info.instructions[sm.original_instruction].unwrap_submodule();

                let mut refers_to_clone = sm.refers_to.clone();
                refers_to_clone.template_args.fully_substitute(&unifier.store);

                if let Some(instance) = self
                    .linker
                    .instantiator
                    .instantiate(self.linker, refers_to_clone)
                {
                    let sub_module = &self.linker.modules[sm.refers_to.id];

                    for (_port_id, concrete_port, source_code_port, connecting_wire) in
                        zip_eq3(&instance.interface_ports, &sub_module.ports, &sm.port_map)
                    {
                        match (concrete_port, connecting_wire) {
                            (None, None) => {} // Invalid port not connected, good!
                            (None, Some(connecting_wire)) => {
                                // Port is not enabled, but attempted to be used
                                // A question may be "What if no port was in the source code? There would be no error reported"
                                // But this is okay, because nonvisible ports are only possible for function calls
                                // We have a second routine that reports invalid interfaces.
                                for span in &connecting_wire.name_refs {
                                    self.errors.error(*span, format!("Port '{}' is used, but the instantiated module has this port disabled", source_code_port.name))
                                    .info_obj_different_file(source_code_port, sub_module.link_info.file)
                                    .info_obj_same_file(submod_instr);
                                }
                            }
                            (Some(_concrete_port), None) => {
                                // Port is enabled, but not used
                                self.errors
                                    .warn(
                                        submod_instr.module_ref.get_total_span(),
                                        format!("Unused port '{}'", source_code_port.name),
                                    )
                                    .info_obj_different_file(
                                        source_code_port,
                                        sub_module.link_info.file,
                                    )
                                    .info_obj_same_file(submod_instr);
                            }
                            (Some(concrete_port), Some(connecting_wire)) => {
                                let wire = &self.wires[connecting_wire.maps_to_wire];
                                if source_code_port.is_input {
                                    // Subtype relations always flow FORWARD.
                                    unifier.unify_concrete::<false>(&wire.typ, &concrete_port.typ);
                                } else {
                                    unifier.unify_concrete::<true>(&wire.typ, &concrete_port.typ);
                                }
                            }
                        }
                    }
                    for (_interface_id, interface_references, sm_interface) in
                        zip_eq(&sm.interface_call_sites, &sub_module.interfaces)
                    {
                        if !interface_references.is_empty() {
                            let interface_name = &sm_interface.name;
                            if let Some(representative_port) = sm_interface
                                .func_call_inputs
                                .first()
                                .or(sm_interface.func_call_outputs.first())
                            {
                                if instance.interface_ports[representative_port].is_none() {
                                    for span in interface_references {
                                        self.errors.error(*span, format!("The interface '{interface_name}' is disabled in this submodule instance"))
                                        .info_obj_same_file(submod_instr)
                                        .info((sm_interface.name_span, sub_module.link_info.file), format!("Interface '{interface_name}' declared here"));
                                    }
                                }
                            } else {
                                for span in interface_references {
                                    self.errors.todo(*span, format!("Using empty interface '{interface_name}' (This is a TODO with Actions etc)"))
                                    .info_obj_same_file(submod_instr)
                                    .info((sm_interface.name_span, sub_module.link_info.file), format!("Interface '{interface_name}' declared here"));
                                }
                            }
                            if sm_interface
                                .all_ports()
                                .iter()
                                .any(|port_id| instance.interface_ports[port_id].is_none())
                            {
                                // We say an interface is invalid if it has an invalid port.
                                todo!("Invalid Interfaces");
                            }
                        }
                    }

                    sm.instance
                        .set(instance)
                        .expect("Can only set an InstantiatedModule once");
                } else {
                    self.errors.error(
                        submod_instr.module_ref.get_total_span(),
                        "Error instantiating submodule",
                    );
                }
            });
        }
    }

    /// Calls [FullySubstitutable::fully_substitute] on everything. From this point on the substitutor is unneccesary
    fn finalize(&mut self, substitutor: &ValueUnifierStore) {
        for (_id, w) in &mut self.wires {
            if !w.typ.fully_substitute(substitutor) {
                let span = w.get_span(self.link_info);
                span.debug();
                self.errors.error(
                    span,
                    format!(
                        "Could not finalize this type, some parameters were still unknown: {}",
                        w.typ.display(&self.linker.types, true)
                    ),
                );
            }
            match &mut w.source {
                RealWireDataSource::UnaryOp { rank, .. }
                | RealWireDataSource::BinaryOp { rank, .. } => {
                    for r in rank {
                        // Rank not fully substituting is caught by the fully_substitute calls on w
                        let _ = r.fully_substitute(substitutor);
                    }
                }
                _ => {}
            }
        }

        for (_, sm) in &mut self.submodules {
            if !sm.refers_to.template_args.fully_substitute(substitutor) {
                self.errors.error(sm.get_span(self.link_info), format!("Could not infer the parameters of this submodule, some parameters were still unknown: {}", 
                sm.refers_to.display(self.linker, true)
            ));
            }
            if let Some(instance) = sm.instance.get() {
                for (_port_id, concrete_port, connecting_wire) in
                    zip_eq(&instance.interface_ports, &sm.port_map)
                {
                    let (Some(concrete_port), Some(connecting_wire)) =
                        (concrete_port, connecting_wire)
                    else {
                        continue;
                    };

                    // We overwrite the ports, to have the Multiplexer inputs correctly call type errors on submodule inputs
                    let connecting_wire = &mut self.wires[connecting_wire.maps_to_wire];
                    connecting_wire.typ.clone_from(&concrete_port.typ);
                }
            }
        }
    }
}
