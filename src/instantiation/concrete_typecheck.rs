use ibig::IBig;
use sus_proc_macro::get_builtin_type;

use crate::alloc::{zip_eq, zip_eq3};
use crate::typing::set_unifier::{DelayedErrorCollector, FullySubstitutable, Unifyable};
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

/// Walks a path `b == a[x][y][z:w]`, and returns `a[x][y][z:w]` and `b[]` (take one array from b). Also unifies this one array with w - z
pub fn co_walk_path<'inst>(
    mut a: &'inst ConcreteType,
    mut b: &'inst ConcreteType,
    path: &'inst [RealWirePathElem],
    unifier: &mut ValueUnifier<'inst>,
) -> (&'inst ConcreteType, &'inst ConcreteType) {
    for p in path {
        match p {
            RealWirePathElem::Index { .. } => {
                a = &a.unwrap_array().0;
            }
            RealWirePathElem::PartSelect { width, .. } => {
                a = &a.unwrap_array().0;
                let (new_b, b_sz) = b.unwrap_array();
                b = new_b;

                // Is checked in final_checks
                let _ = unifier.set(b_sz, Value::Integer(width.clone()));
            }
            RealWirePathElem::Slice { bounds, .. } => {
                let (new_a, a_sz) = a.unwrap_array();
                a = new_a;
                let (new_b, b_sz) = b.unwrap_array();
                b = new_b;

                match bounds {
                    PartialBound::Known(from, to) => {
                        // Is checked in final_checks
                        let _ = unifier.set(b_sz, Value::Integer(to - from));
                    }
                    PartialBound::From(from) => {
                        unifier_constraint_ints!(unifier, [a_sz], {
                            // TODO #88, Slices of variable base offset
                            // Is checked in final_checks
                            let _ = unifier.set(b_sz, Value::Integer(a_sz - from));
                        })
                    }
                    PartialBound::To(to) => {
                        // TODO #88, Slices of variable base offset
                        // Is checked in final_checks
                        let _ = unifier.set(b_sz, Value::Integer(to.clone()));
                    }
                    PartialBound::WholeSlice => {
                        // Is checked in final_checks
                        let _ = unifier.unify(b_sz, a_sz);
                    }
                }
            }
        }
    }

    (a, b)
}

impl<'inst, 'l: 'inst> ModuleTypingContext<'l> {
    pub fn typecheck(&mut self, type_substitutor_alloc: ValueUnifierAlloc) {
        let error_reporter = DelayedErrorCollector::new();

        let mut unifier = ValueUnifier::from_alloc(type_substitutor_alloc);

        for (_, wire) in &self.wires {
            self.typecheck_wire(wire, &mut unifier, &error_reporter);
        }
        for (_, sm) in &self.submodules {
            self.typecheck_submodule(sm, &mut unifier);
        }

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
    /*fn peano_to_nested_array_of(
        &mut self,
        p: &PeanoType,
        c: ConcreteType,
        dims: &mut Vec<ConcreteType>,
    ) -> ConcreteType {
        let substitutor: TypeSubstitutor<ConcreteType> = 0;
        match p {
            PeanoType::Zero => c,
            PeanoType::Succ(p) => {
                let this_dim_var = substitutor.alloc_unknown();
                let arr = ConcreteType::Array(Box::new((c, this_dim_var.clone())));
                let typ = self.peano_to_nested_array_of(p, arr, dims);
                dims.push(this_dim_var.clone());
                typ
            }
            _ => unreachable!("Peano abstract ranks being used at concrete type-checking time should never be anything other than Zero, Succ or Named ({p:?})"),
        }
    }*/
    /*fn walk_type_along_path(
        type_substitutor: &mut TypeUnifier<TypeSubstitutor<ConcreteType>>,
        mut current_type_in_progress: ConcreteType,
        path: &[RealWirePathElem],
    ) -> ConcreteType {
        for p in path {
            let typ_after_applying_array = type_substitutor.alloc_unknown();
            match p {
                RealWirePathElem::ArrayAccess {
                    span: _,
                    idx_wire: _,
                } => {
                    // TODO #28 integer size <-> array bound check
                    let arr_size = type_substitutor.alloc_unknown();
                    let arr_box = Box::new((typ_after_applying_array.clone(), arr_size));
                    type_substitutor.unify_must_succeed(
                        &current_type_in_progress,
                        &ConcreteType::Array(arr_box),
                    );
                    current_type_in_progress = typ_after_applying_array;
                }
                RealWirePathElem::ArraySlice { .. }
                | RealWirePathElem::ArrayPartSelectDown { .. }
                | RealWirePathElem::ArrayPartSelectUp { .. } => {
                    let inner_of_array_being_sliced = type_substitutor.alloc_unknown();

                    let array_being_sliced = Box::new((
                        inner_of_array_being_sliced.clone(),
                        type_substitutor.alloc_unknown(),
                    ));

                    let slice_size = type_substitutor.alloc_unknown();
                    type_substitutor.unify_must_succeed(
                        &current_type_in_progress,
                        &ConcreteType::Array(array_being_sliced),
                    );
                    type_substitutor.unify_must_succeed(
                        &typ_after_applying_array,
                        &ConcreteType::Array(Box::new((inner_of_array_being_sliced, slice_size))),
                    );

                    current_type_in_progress = typ_after_applying_array;
                }
            }
        }

        current_type_in_progress
    }*/

    fn typecheck_wire(
        &'inst self,
        out: &'inst RealWire,
        unifier: &mut ValueUnifier<'inst>,
        errors: &ValueErrorReporter<'inst>,
    ) {
        let original_instr = &self.link_info.instructions[out.original_instruction];
        let span = original_instr.get_span();
        span.debug();

        match &out.source {
            RealWireDataSource::ReadOnly => {}
            RealWireDataSource::Multiplexer {
                is_state: _,
                sources,
            } => {
                // Temporary vector because borrow checker doesn't like constructing the iter with unifier
                let pairs: Vec<_> = sources
                    .iter()
                    .map(|s| {
                        let source_wire = &self.wires[s.from];
                        co_walk_path(&out.typ, &source_wire.typ, &s.to_path, unifier)
                    })
                    .collect();

                unifier.create_subtype_constraint(pairs.iter().copied());
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
                        if !unifier.unify_concrete_only_exact(left_root, right_root) {
                            errors.error(move |substitutor| {
                                self.errors.type_error(
                                    "operator ==",
                                    span,
                                    right_root.display_substitute(self.linker, substitutor),
                                    left_root.display_substitute(self.linker, substitutor),
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
                let root_wire = &self.wires[*root];

                let (found, expected) = co_walk_path(&root_wire.typ, &out.typ, path, unifier);

                // Checked in final_check
                let _ = unifier.unify_concrete_all(found, expected);
            }
            RealWireDataSource::ConstructArray { array_wires } => {
                let (array_content_supertyp, array_size) = out.typ.unwrap_array();

                unifier.create_subtype_constraint(array_wires.iter().map(|w_id| {
                    let w = &self.wires[*w_id];
                    (array_content_supertyp, &w.typ)
                }));

                // The output's size cannot have already been unified, this is the first time we see it
                assert!(unifier.set(array_size, Value::Integer(IBig::from(array_wires.len()))));
            }
            // type is already set when the wire was created
            RealWireDataSource::Constant { value: _ } => {}
        };
    }

    fn typecheck_submodule(&'inst self, sm: &'inst SubModule, unifier: &mut ValueUnifier<'inst>) {
        assert!(sm.instance.get().is_none());

        // Check if there's any argument that isn't known
        let mut substitutables = Vec::new();
        sm.refers_to
            .template_args
            .gather_all_substitutables(&mut substitutables);

        unifier.add_constraint(substitutables, |unifier| {
            let submod_instr = &self.link_info.instructions[sm.original_instruction];

            let mut refers_to_clone = sm.refers_to.clone();
            refers_to_clone
                .template_args
                .fully_substitute(&unifier.store);

            let instance = self
                .linker
                .instantiator
                .instantiate(self.linker, refers_to_clone);

            if let Some(instance) = instance {
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
                                let port_name = &source_code_port.name;
                                let err = format!("Port '{port_name}' is used, but the instantiated module has this port disabled");
                                self.errors
                                    .error(*span, err)
                                    .info_obj_different_file(
                                        source_code_port,
                                        sub_module.link_info.file,
                                    )
                                    .info_obj_same_file(submod_instr);
                            }
                        }
                        (Some(_concrete_port), None) => {
                            // Port is enabled, but not used
                            self.errors
                                .warn(
                                    submod_instr.get_span(),
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
                            match source_code_port.direction {
                                // Subtype relations always flow FORWARD.
                                Direction::Input => {
                                    let _ = unifier
                                        .unify_concrete_only_exact(&wire.typ, &concrete_port.typ);
                                }
                                Direction::Output => {
                                    if !unifier.unify_concrete_all(&wire.typ, &concrete_port.typ) {}
                                }
                            }
                        }
                    }
                }
                sm.instance.set(instance).unwrap();
            } else {
                self.errors
                    .error(submod_instr.get_span(), "Error instantiating submodule");
            }
        });
    }

    /// Calls [FullySubstitutable::fully_substitute] on everything. From this point on the substitutor is unneccesary
    fn finalize(&mut self, substitutor: &ValueUnifierStore) {
        let mut selects_to_check = Vec::new();
        for (w_id, w) in &mut self.wires {
            if !w.typ.fully_substitute(substitutor) {
                let span = w.get_span(self.link_info);
                span.debug();
                self.errors.error(
                    span,
                    format!(
                        "Could not finalize this type, some parameters were still unknown: {}",
                        w.typ.display(self.linker, true)
                    ),
                );
            } else if !w.typ.is_valid() {
                self.errors.error(
                    w.get_span(self.link_info),
                    format!(
                        "The type of this wire is invalid! {}",
                        w.typ.display(self.linker, true)
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
                RealWireDataSource::Multiplexer { sources, .. } => {
                    for s in sources {
                        Self::finalize_partial_bounds(&mut s.to_path, &w.typ);
                    }
                }
                RealWireDataSource::Select { root, .. } => selects_to_check.push((w_id, *root)),
                _ => {}
            }
        }

        for (target, root) in selects_to_check {
            let (target, root) = self.wires.get2_mut(target, root).unwrap();
            let_unwrap!(
                RealWireDataSource::Select { root: _, path },
                &mut target.source
            );
            Self::finalize_partial_bounds(path, &root.typ);
        }

        for (_, sm) in &mut self.submodules {
            if !sm.refers_to.template_args.fully_substitute(substitutor) {
                self.errors.error(sm.get_span(self.link_info), format!("Could not infer the parameters of this submodule, some parameters were still unknown: {}", 
                    sm.refers_to.display(self.linker, true)
                ));
            } else if let Err(reason) = sm.refers_to.report_if_errors(
                self.linker,
                "Invalid arguments found in a submodule's template arguments.",
            ) {
                self.errors.error(sm.get_span(self.link_info), reason);
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

    fn finalize_partial_bounds(path: &mut [RealWirePathElem], mut typ: &ConcreteType) {
        for pe in path {
            match pe {
                RealWirePathElem::Index { .. } => {
                    typ = &typ.unwrap_array().0;
                }
                RealWirePathElem::PartSelect { .. } => {
                    typ = &typ.unwrap_array().0;
                }
                RealWirePathElem::Slice { bounds, .. } => {
                    // TODO: #88: Variable base arrays, that's why this is part here
                    let (new_typ, sz) = typ.unwrap_array();
                    typ = new_typ;

                    if let Unifyable::Set(sz) = sz {
                        let sz = sz.unwrap_integer();
                        *bounds = match std::mem::replace(bounds, PartialBound::WholeSlice) {
                            PartialBound::Known(from, to) => PartialBound::Known(from, to),
                            PartialBound::From(from) => PartialBound::Known(from, sz.clone()),
                            PartialBound::To(to) => PartialBound::Known(IBig::from(0), to),
                            PartialBound::WholeSlice => {
                                PartialBound::Known(IBig::from(0), sz.clone())
                            }
                        };
                    }
                }
            }
        }
    }
}
