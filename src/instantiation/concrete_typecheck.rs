use std::fmt::Display;
use std::ops::{Deref, DerefMut};

use ibig::IBig;
use sus_proc_macro::get_builtin_type;

use crate::alloc::{zip_eq, zip_eq3, ArenaAllocator, UUID};
use crate::flattening::StructType;
use crate::typing::value_unifier::{ValueErrorReporter, ValueUnificationRelation};
use crate::{let_unwrap, unifier_constraint, unifier_constraint_ints};
use crate::linker::LinkInfo;
use crate::to_string::ConcreteTypeDisplay;
use crate::typing::concrete_type;
use crate::typing::set_unifier::{DelayedErrorCollector, FullySubstitutable, Unifyable};
use crate::typing::{
    concrete_type::ConcreteTemplateArg,
    concrete_type::{ConcreteType, BOOL_CONCRETE_TYPE},
    set_unifier::SetUnifierStore,
    template::TVec,
    type_inference::{
        DelayedConstraint, DelayedConstraintStatus, DelayedConstraintsList, FailedUnification,
    },
    value_unifier::ValueUnifier,
};

use super::*;

use crate::typing::type_inference::{ConcreteTypeVariableIDMarker, Substitutor};

macro_rules! assert_due_to_variable_clones {
    ($cond:expr) => {
        assert!($cond, "This assertion cannot fail, because the variables that caused the unification failure should have been cloned in execute")
    };
}

fn typecheck(
    exec: Executed,
    link_info: &LinkInfo,
    linker: &Linker,
    working_on_template_args: Rc<TVec<ConcreteTemplateArg>>,
) -> Typechecked {
    let mut ctx = TypingContext {
        wires: exec.wires,
        submodules: exec.submodules,
        working_on_template_args,
        link_info,
    };
    let error_reporter = DelayedErrorCollector::new();

    let mut unifier = ValueUnifier::from_alloc(exec.type_substitutor);

    ctx.typecheck_all_wires(&mut unifier, &error_reporter);

    // Reports all the delayed errors that have built up
    let substitutor = unifier.execute();

    let errors = error_reporter.report(&substitutor, link_info.file, linker);
    
    for (_id, w) in &mut ctx.wires {
        if !w.typ.fully_substitute(&substitutor) {
            let typ_as_str = w.typ.display(&linker.types);

            let span = w.get_span(link_info);
            span.debug();
            errors.error(span, format!("Could not finalize this type, some parameters were still unknown: {typ_as_str}"));
        }
    }

    Typechecked {
        wires: ctx.wires,
        submodules: ctx.submodules,
        generation_state: exec.generation_state,
        errors: errors.into_storage(),
    }
}

/// As with other contexts, this is the shared state we're lugging around while executing & typechecking a module.
struct TypingContext<'l> {
    wires: FlatAlloc<RealWire, WireIDMarker>,
    submodules: FlatAlloc<SubModule, SubModuleIDMarker>,
    working_on_template_args: Rc<TVec<ConcreteTemplateArg>>,
    link_info: &'l LinkInfo,
}

impl<'inst, 'errs> TypingContext<'_> {
    fn walk_type_along_path(
        &'inst self,
        root_wire: &'inst RealWire,
        path: &'inst [RealWirePathElem],
        unifier: &mut ValueUnifier<'inst>,
        errors: &'errs ValueErrorReporter<'inst>
    ) -> &'inst ConcreteType {
        let mut cur_type = &root_wire.typ;
        for p in path {
            match p {
                RealWirePathElem::ArrayAccess {
                    span,
                    idx_wire,
                } => {
                    let_unwrap!(ConcreteType::Array(arr_box), cur_type);
                    let (arr_content, arr_size) = arr_box.deref();
                    let idx_wire = &self.wires[*idx_wire];
                    let [min, max] = idx_wire.typ.get_value_args(get_builtin_type!("int"));
                    unifier_constraint_ints!(unifier, [min, max, arr_size], {
                        if min < &IBig::from(0) || max >= arr_size {
                            let min = min.clone();
                            let max = max.clone();
                            let arr_size = arr_size.clone();
                            let span = span.inner_span();
                            errors.error(move |substitutor, errors, linker| {
                                errors.error(span, format!("Array access out of bounds! The index bounds are {min}:{max}, but this array is of size {arr_size}."))
                                    .info_same_file(root_wire.get_span(self.link_info), format!("{} of type {} declared here", root_wire.name, root_wire.typ.display_substitute(linker, substitutor)));
                            });
                        }
                    });
                    cur_type = arr_content;
                }
            }
        }

        cur_type
    }

    fn walk_rank(
        rank: &[UnifyableValue],
        mut typ: &'inst ConcreteType,
        unifier: &mut ValueUnifier<'inst>,
    ) -> (&'inst ConcreteType, bool) {
        let rank_unify_success = rank.iter().all(|r| {
            let_unwrap!(ConcreteType::Array(arr), typ);
            typ = &arr.0;
            unifier.unify(r, &arr.1)
        });
        (typ, rank_unify_success)
    }

    fn typecheck_all_wires(&'inst self, unifier: &mut ValueUnifier<'inst>, errors: &'errs ValueErrorReporter<'inst>) {
        for this_wire_id in self.wires.id_range() {
            let out = &self.wires[this_wire_id];
            let original_instr = &self.link_info.instructions[out.original_instruction];
            let span = original_instr.get_span();
            span.debug();

            match &out.source {
                RealWireDataSource::ReadOnly => {}
                RealWireDataSource::Multiplexer { is_state, sources } => {
                    if let Some(is_state) = is_state {
                        match is_state.concretize_type(
                            self.linker,
                            &original_instr.unwrap_declaration().typ.typ,
                            &self.working_on_template_args,
                            &mut self.type_substitutor,
                        ) {
                            Ok(value_typ) => self.type_substitutor.unify_report_error(
                                &value_typ,
                                &out.typ,
                                span,
                                "initial value of state",
                            ),
                            Err(reason) => {
                                self.errors.error(span, reason);
                            }
                        }
                    }
                    unifier.create_subtype_constraint(errors, sources.iter().map(|s| {
                        let source_wire = &self.wires[s.from];
                        let destination_typ = self.walk_type_along_path(out, &s.to_path, unifier, errors);
                        (destination_typ, &source_wire.typ, source_wire.get_span(self.link_info))
                    }));
                }
                RealWireDataSource::UnaryOp { op, rank, right } => {
                    // TODO overloading
                    let right = &self.wires[*right];
                    let (out_root, out_success) = Self::walk_rank(rank, &out.typ, unifier);
                    assert_due_to_variable_clones!(out_success);
                    let (right_root, right_success) = Self::walk_rank(rank, &right.typ, unifier);
                    if !right_success {
                        errors.error(|substitutor, errors, linker| {
                            errors
                                .error(right.get_span(self.link_info), format!("Incompatible multi-rank for higher-rank operator: Found {} but output is {}", 
                                right.typ.display_substitute(linker, substitutor), 
                                out.typ.display_substitute(linker, substitutor))
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
                                assert!(right_root.unify_named_template_args(get_builtin_type!("int"), unifier, [-(max.clone()), -(min.clone())]));
                            });
                        }
                        UnaryOperator::Sum => {
                            let (content, sz) = right_root.unwrap_array();
                            let [min, max] = content.get_value_args(get_builtin_type!("int"));

                            unifier_constraint_ints!(unifier, [min, max, sz], {
                                assert!(out_root.unify_named_template_args(get_builtin_type!("int"), unifier, [min * sz, max * sz]));
                            });
                        }
                        UnaryOperator::Product => {
                            let (content, sz) = right_root.unwrap_array();
                            let [min, max] = content.get_value_args(get_builtin_type!("int"));

                            unifier_constraint_ints!(unifier, [min, max, sz], {
                                let sz = usize::try_from(sz).unwrap();
                                assert!(out_root.unify_named_template_args(get_builtin_type!("int"), unifier, [min.pow(sz), max.pow(sz)]));
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
                    let (out_root, out_success) = Self::walk_rank(rank, &out.typ, unifier);
                    assert_due_to_variable_clones!(out_success);
                    let (left_root, left_success) = Self::walk_rank(rank, &left.typ, unifier);
                    if !left_success {
                        errors.error(|substitutor, errors, linker| {
                            errors
                                .error(left.get_span(self.link_info), format!("Incompatible multi-rank for higher-rank operator: Found {} but output is {}", 
                                left.typ.display_substitute(linker, substitutor), 
                                out.typ.display_substitute(linker, substitutor))
                            ).info_same_file(right.get_span(self.link_info), format!("Right argument has type {}", right.typ.display_substitute(linker, substitutor)));
                        });
                    }
                    let (right_root, right_success) = Self::walk_rank(rank, &right.typ, unifier);
                    if !right_success {
                        errors.error(|substitutor, errors, linker| {
                            errors
                                .error(right.get_span(self.link_info), format!("Incompatible multi-rank for higher-rank operator: Found {} but output is {}", 
                                right.typ.display_substitute(linker, substitutor),
                                out.typ.display_substitute(linker, substitutor))
                            ).info_same_file(left.get_span(self.link_info), format!("Left argument has type {}", left.typ.display_substitute(linker, substitutor)));
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
                            let [left_min, left_max] = left_root.get_value_args(get_builtin_type!("int"));
                            let [right_min, right_max] = right_root.get_value_args(get_builtin_type!("int"));
                            unifier_constraint_ints!(unifier, [left_min, left_max, right_min, right_max], {
                                let (out_min, out_max) = match op {
                                    BinaryOperator::Add => (
                                        right_min + left_min,
                                        right_max + left_max,
                                    ),
                                    BinaryOperator::Subtract => (
                                        left_min - right_max,
                                        left_max - right_min,
                                    ),
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
                                        if right_min == &IBig::from(0) {
                                            let potentials = [
                                                left_min / right_max,
                                                left_max / right_max,
                                            ];
                                            (
                                                potentials.iter().min().unwrap().clone(),
                                                potentials.iter().max().unwrap().clone(),
                                            )
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
                                            errors.error(|substitutor, errors, linker| {
                                                errors.error(right.get_span(self.link_info), format!("Modulo divisor must be > 0, but its range is {}", 
                                                    right.typ.display_substitute(linker, substitutor)
                                                ));
                                            });
                                            return;
                                        }
                                        (IBig::from(0), right_max - IBig::from(1))
                                    }
                                    _ => {
                                        unreachable!("The BinaryOpTypecheckConstraint should only check arithmetic operations but got {}", op);
                                    }
                                };
                                assert!(out.typ.unify_named_template_args(get_builtin_type!("int"), unifier, [out_min, out_max]))
                            });
                        }
                        BinaryOperator::Equals
                        | BinaryOperator::NotEquals => {
                            if !unifier.unify_concrete::<false>(left_root, right_root) {
                                errors.error_bad_unify(&right.typ, &left.typ, right.get_span(self.link_info));
                            }
                            assert_eq!(out_root.unwrap_named().id, get_builtin_type!("bool"));
                        }
                        | BinaryOperator::GreaterEq
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
                    let found_typ = self.walk_type_along_path(&self.wires[*root], path, unifier, errors);
                    self.set_wire_typ(unifier, errors, out, found_typ);
                }
                RealWireDataSource::ConstructArray { array_wires } => {
                    let (array_content_supertyp, array_size) = out.typ.unwrap_array();

                    unifier.create_subtype_constraint(errors, array_wires.iter().map(|w_id| {
                        let w = &self.wires[*w_id];
                        (array_content_supertyp, &w.typ, w.get_span(self.link_info))
                    }));

                    let array_wires_len = array_wires.len();
                    if let Err(expected_array_size) = unifier.set(array_size, Value::Integer(IBig::from(array_wires_len))) {
                        errors.error(|substitutor, errors, linker| {
                            errors.error(span, format!("This construct creates an array of size {array_wires_len}, but the expected size is {expected_array_size}"));
                        });
                    }
                }
                // type is already set when the wire was created
                RealWireDataSource::Constant { value: _ } => {}
            };
        }
    }

    fn set_wire_typ(&'inst self, unifier: &mut ValueUnifier<'inst>, errors: &'errs ValueErrorReporter<'inst>, out: &'inst RealWire, found_typ: &'inst ConcreteType) {
        if !unifier.unify_concrete::<true>(&out.typ, found_typ) {
            errors.error_bad_unify(found_typ, &out.typ, out.get_span(self.link_info));
        }
    }
}

struct SubmoduleTypecheckConstraint {
    sm_id: SubModuleID,
}

impl DelayedConstraint<InstantiationContext<'_, '_>> for SubmoduleTypecheckConstraint {
    fn try_apply(&mut self, context: &mut InstantiationContext) -> DelayedConstraintStatus {
        let sm = &mut context.submodules[self.sm_id];
        assert!(sm.instance.get().is_none());

        let submod_instr =
            context.link_info.instructions[sm.original_instruction].unwrap_submodule();

        let sub_module = &context.linker.modules[sm.refers_to.id];

        // Check if there's any argument that isn't known
        for (_id, arg) in &mut Rc::get_mut(&mut sm.refers_to).unwrap().template_args {
            if !arg.fully_substitute(&context.type_substitutor) {
                // We don't actually *need* to already fully_substitute here, but it's convenient and saves some work
                return DelayedConstraintStatus::NoProgress;
            }
        }

        if let Some(instance) = context
            .linker
            .instantiator
            .instantiate(context.linker, sm.refers_to.clone())
        {
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
                            context.errors.error(*span, format!("Port '{}' is used, but the instantiated module has this port disabled", source_code_port.name))
                                .info_obj_different_file(source_code_port, sub_module.link_info.file)
                                .info_obj_same_file(submod_instr);
                        }
                    }
                    (Some(_concrete_port), None) => {
                        // Port is enabled, but not used
                        context
                            .errors
                            .warn(
                                submod_instr.module_ref.get_total_span(),
                                format!("Unused port '{}'", source_code_port.name),
                            )
                            .info_obj_different_file(source_code_port, sub_module.link_info.file)
                            .info_obj_same_file(submod_instr);
                    }
                    (Some(concrete_port), Some(connecting_wire)) => {
                        let wire = &context.wires[connecting_wire.maps_to_wire];
                        context.type_substitutor.unify_report_error(
                            &wire.typ,
                            &concrete_port.typ,
                            submod_instr.module_ref.get_total_span(),
                            || {
                                use crate::errors::ErrorInfoObject;
                                let port_declared_here = source_code_port
                                    .make_info(sub_module.link_info.file)
                                    .unwrap();

                                (
                                    format!("Port '{}'", source_code_port.name),
                                    vec![port_declared_here],
                                )
                            },
                        );
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
                                context.errors.error(*span, format!("The interface '{interface_name}' is disabled in this submodule instance"))
                                    .info_obj_same_file(submod_instr)
                                    .info((sm_interface.name_span, sub_module.link_info.file), format!("Interface '{interface_name}' declared here"));
                            }
                        }
                    } else {
                        for span in interface_references {
                            context.errors.todo(*span, format!("Using empty interface '{interface_name}' (This is a TODO with Actions etc)"))
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

            // Overwrite the refers_to with the identical instance.global_ref
            assert!(sm.refers_to == instance.global_ref);
            sm.refers_to = instance.global_ref.clone();

            sm.instance
                .set(instance)
                .expect("Can only set an InstantiatedModule once");

            DelayedConstraintStatus::Resolved
        } else {
            context.errors.error(
                submod_instr.module_ref.get_total_span(),
                "Error instantiating submodule",
            );
            DelayedConstraintStatus::Resolved
        }
    }

    fn report_could_not_resolve_error(&self, context: &InstantiationContext) {
        let sm = &context.submodules[self.sm_id];

        let submod_instr =
            context.link_info.instructions[sm.original_instruction].unwrap_submodule();

        let submodule_template_args_string = sm.refers_to.display(context.linker, true);
        let message = format!("Could not fully instantiate {submodule_template_args_string}");

        context
            .errors
            .error(submod_instr.get_most_relevant_span(), message);
    }
}

pub struct LatencyInferenceDelayedConstraint {}
impl DelayedConstraint<InstantiationContext<'_, '_>> for LatencyInferenceDelayedConstraint {
    fn try_apply(&mut self, context: &mut InstantiationContext<'_, '_>) -> DelayedConstraintStatus {
        context.infer_parameters_for_latencies()
    }

    fn report_could_not_resolve_error(&self, _context: &InstantiationContext<'_, '_>) {} // Handled by incomplete submodules themselves
}

/// As with other contexts, this is the shared state we're lugging around while executing & typechecking a module.
struct TypingFinalizationContext<'l> {
    pub wires: FlatAlloc<RealWire, WireIDMarker>,
    pub submodules: FlatAlloc<SubModule, SubModuleIDMarker>,
    pub type_substitutor: SetUnifierStore<Value, ConcreteTypeVariableIDMarker>,

    pub errors: ErrorCollector<'l>,

    pub link_info: &'l LinkInfo,
    pub linker: &'l Linker,
}
