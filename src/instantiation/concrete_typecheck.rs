use ibig::IBig;

use crate::alloc::{zip_eq, zip_eq3, UUID};
use crate::typing::abstract_type::PeanoType;
use crate::typing::{
    concrete_type::{ConcreteType, BOOL_CONCRETE_TYPE},
    type_inference::{
        DelayedConstraint, DelayedConstraintStatus, DelayedConstraintsList, FailedUnification,
    },
};

use super::*;

use crate::typing::type_inference::{Substitutor, TypeSubstitutor};

impl InstantiationContext<'_, '_> {
    fn peano_to_nested_array_of(
        &mut self,
        p: &PeanoType,
        c: ConcreteType,
        dims: &mut Vec<ConcreteType>,
    ) -> ConcreteType {
        match p {
            PeanoType::Zero => c,
            PeanoType::Succ(p) => {
                let this_dim_var = self.type_substitutor.alloc_unknown();
                let arr = ConcreteType::Array(Box::new((c, this_dim_var.clone())));
                let typ = self.peano_to_nested_array_of(p, arr, dims);
                dims.push(this_dim_var.clone());
                typ
            }
            _ => unreachable!("Peano abstract ranks being used at concrete type-checking time should never be anything other than Zero, Succ or Named ({p:?})"),
        }
    }
    fn walk_type_along_path(
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
            }
        }

        current_type_in_progress
    }

    fn typecheck_all_wires(&mut self, delayed_constraints: &mut DelayedConstraintsList<Self>) {
        for this_wire_id in self.wires.id_range() {
            let this_wire = &self.wires[this_wire_id];
            let span = self.md.get_instruction_span(this_wire.original_instruction);
            span.debug();

            match &this_wire.source {
                RealWireDataSource::ReadOnly => {}
                RealWireDataSource::Multiplexer { is_state, sources } => {
                    if let Some(is_state) = is_state {
                        match is_state.get_type(&mut self.type_substitutor) {
                            Ok(value_typ) => self.type_substitutor.unify_report_error(
                                &value_typ,
                                &this_wire.typ,
                                span,
                                "initial value of state",
                            ),
                            Err(reason) => {
                                self.errors.error(span, reason);
                            }
                        }
                    }
                    for s in sources {
                        let source_typ = &self.wires[s.from].typ;
                        let destination_typ = Self::walk_type_along_path(
                            &mut self.type_substitutor,
                            self.wires[this_wire_id].typ.clone(),
                            &s.to_path,
                        );
                        self.type_substitutor.unify_report_error(
                            &destination_typ,
                            source_typ,
                            span,
                            "write wire access",
                        );
                    }
                }
                &RealWireDataSource::UnaryOp { op, rank: _, right } => {
                    // TODO overloading
                    let (input_typ, output_typ) = match op {
                        UnaryOperator::Not => (BOOL_CONCRETE_TYPE, BOOL_CONCRETE_TYPE),
                        UnaryOperator::And | UnaryOperator::Or | UnaryOperator::Xor => (
                            self.type_substitutor.make_array_of(BOOL_CONCRETE_TYPE),
                            BOOL_CONCRETE_TYPE,
                        ),
                        UnaryOperator::Negate | UnaryOperator::Sum | UnaryOperator::Product => {
                            delayed_constraints.push(UnaryOpTypecheckConstraint {
                                op,
                                out: this_wire_id,
                                input: right,
                                span,
                            });
                            continue;
                        }
                    };

                    self.type_substitutor.unify_report_error(
                        &self.wires[right].typ,
                        &input_typ,
                        span,
                        "unary input",
                    );
                    self.type_substitutor.unify_report_error(
                        &self.wires[this_wire_id].typ,
                        &output_typ,
                        span,
                        "unary output",
                    );
                }
                &RealWireDataSource::BinaryOp {
                    op,
                    rank: _,
                    left,
                    right,
                } => {
                    // TODO overloading
                    // Typecheck generic INT
                    let ((left_typ, right_typ), output_typ) = match op {
                        BinaryOperator::And => {
                            ((BOOL_CONCRETE_TYPE, BOOL_CONCRETE_TYPE), BOOL_CONCRETE_TYPE)
                        }
                        BinaryOperator::Or => {
                            ((BOOL_CONCRETE_TYPE, BOOL_CONCRETE_TYPE), BOOL_CONCRETE_TYPE)
                        }
                        BinaryOperator::Xor => {
                            ((BOOL_CONCRETE_TYPE, BOOL_CONCRETE_TYPE), BOOL_CONCRETE_TYPE)
                        }
                        BinaryOperator::Add
                        | BinaryOperator::Subtract
                        | BinaryOperator::Multiply
                        | BinaryOperator::Divide
                        | BinaryOperator::Modulo => {
                            delayed_constraints.push(BinaryOpTypecheckConstraint {
                                op,
                                left,
                                right,
                                out: this_wire_id,
                                span,
                            });
                            continue;
                        }
                        BinaryOperator::Equals
                        | BinaryOperator::NotEquals
                        | BinaryOperator::GreaterEq
                        | BinaryOperator::Greater
                        | BinaryOperator::LesserEq
                        | BinaryOperator::Lesser => (
                            (
                                self.type_substitutor.new_int_type(None, None),
                                self.type_substitutor.new_int_type(None, None),
                            ),
                            BOOL_CONCRETE_TYPE,
                        ),
                    };

                    // gets the corresponding abstract type to figure out how many layers of array to unify with:
                    let peano_type = &self.md.link_info.instructions
                        [this_wire.original_instruction]
                        .unwrap_expression()
                        .as_single_output_expr()
                        .unwrap()
                        .typ
                        .typ
                        .rank;
                    let mut out_dims = vec![];
                    let out_type =
                        self.peano_to_nested_array_of(peano_type, output_typ, &mut out_dims);

                    let mut in_left_dims = vec![];
                    let in_left_type =
                        self.peano_to_nested_array_of(peano_type, left_typ, &mut in_left_dims);

                    for (in_left, out) in out_dims.iter().zip(in_left_dims.iter()) {
                        self.type_substitutor.unify_report_error(
                            in_left,
                            out,
                            span,
                            "binary output dimension",
                        );
                    }

                    let mut in_right_dims = vec![];
                    let in_right_type =
                        self.peano_to_nested_array_of(peano_type, right_typ, &mut in_right_dims);

                    for (in_right, out) in out_dims.iter().zip(in_right_dims.iter()) {
                        self.type_substitutor.unify_report_error(
                            in_right,
                            out,
                            span,
                            "binary output dimension",
                        );
                    }

                    self.type_substitutor.unify_report_error(
                        &self.wires[this_wire_id].typ,
                        &out_type,
                        span,
                        "binary output",
                    );
                    self.type_substitutor.unify_report_error(
                        &self.wires[left].typ,
                        &in_left_type,
                        span,
                        "binary left",
                    );
                    self.type_substitutor.unify_report_error(
                        &self.wires[right].typ,
                        &in_right_type,
                        span,
                        "binary right",
                    );
                }
                RealWireDataSource::Select { root, path } => {
                    let found_typ = Self::walk_type_along_path(
                        &mut self.type_substitutor,
                        self.wires[*root].typ.clone(),
                        path,
                    );
                    self.type_substitutor.unify_report_error(
                        &found_typ,
                        &self.wires[this_wire_id].typ,
                        span,
                        "wire access",
                    );
                }
                RealWireDataSource::ConstructArray { array_wires } => {
                    let mut array_wires_iter = array_wires.iter();
                    let first_elem = array_wires_iter.next().unwrap();
                    let element_type = self.wires[*first_elem].typ.clone();
                    for w in array_wires_iter {
                        self.type_substitutor.unify_report_error(
                            &self.wires[*w].typ,
                            &element_type,
                            span,
                            "array construction",
                        );
                    }
                    let array_size_value =
                        ConcreteType::Value(Value::Integer(IBig::from(array_wires.len())));
                    self.type_substitutor.unify_report_error(
                        &self.wires[this_wire_id].typ,
                        &ConcreteType::Array(Box::new((element_type, array_size_value))),
                        span,
                        "array construction",
                    );
                }
                // type is already set when the wire was created
                RealWireDataSource::Constant { value: _ } => {}
            };
        }
    }

    fn finalize(&mut self) {
        for (_id, w) in &mut self.wires {
            if !self.type_substitutor.fully_substitute(&mut w.typ) {
                let typ_as_str = w.typ.display(&self.linker.types);

                let span = self.md.get_instruction_span(w.original_instruction);
                span.debug();
                self.errors.error(span, format!("Could not finalize this type, some parameters were still unknown: {typ_as_str}"));
            }
        }

        // Print all errors
        for FailedUnification {
            mut found,
            mut expected,
            span,
            context,
            infos,
        } in self.type_substitutor.extract_errors()
        {
            // Not being able to fully substitute is not an issue. We just display partial types
            let _ = self.type_substitutor.fully_substitute(&mut found);
            let _ = self.type_substitutor.fully_substitute(&mut expected);

            let expected_name = expected.display(&self.linker.types).to_string();
            let found_name = found.display(&self.linker.types).to_string();
            self.errors
                .error(span, format!("Typing Error: {context} expects a {expected_name} but was given a {found_name}"))
                .add_info_list(infos);

            assert!(
                expected_name != found_name,
                "{expected_name} != {found_name}"
            );
        }
    }

    pub fn typecheck(&mut self) {
        let mut delayed_constraints: DelayedConstraintsList<Self> = DelayedConstraintsList::new();
        for (sm_id, _sm) in &self.submodules {
            delayed_constraints.push(SubmoduleTypecheckConstraint { sm_id });
        }

        self.typecheck_all_wires(&mut delayed_constraints);

        delayed_constraints.push(LatencyInferenceDelayedConstraint {});

        delayed_constraints.resolve_delayed_constraints(self);

        self.finalize();
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
            context.md.link_info.instructions[sm.original_instruction].unwrap_submodule();

        let sub_module = &context.linker.modules[sm.refers_to.id];

        // Check if there's any argument that isn't known
        for (_id, arg) in &mut Rc::get_mut(&mut sm.refers_to).unwrap().template_args {
            if !context
                .type_substitutor
                .fully_substitute(arg.as_mut().unwrap_identical())
            {
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
            context.md.link_info.instructions[sm.original_instruction].unwrap_submodule();

        let submodule_template_args_string = sm.refers_to.display(context.linker, true);
        let message = format!("Could not fully instantiate {submodule_template_args_string}");

        context
            .errors
            .error(submod_instr.get_most_relevant_span(), message);
    }
}

#[derive(Debug)]
struct UnaryOpTypecheckConstraint {
    op: UnaryOperator,
    input: UUID<WireIDMarker>,
    out: UUID<WireIDMarker>,
    span: Span,
}

impl DelayedConstraint<InstantiationContext<'_, '_>> for UnaryOpTypecheckConstraint {
    fn try_apply(&mut self, context: &mut InstantiationContext<'_, '_>) -> DelayedConstraintStatus {
        if let Some(input_complete_type) = context.wires[self.input]
            .typ
            .try_fully_substitute(&context.type_substitutor)
        {
            if let UnaryOperator::Negate = self.op {
                let (min, max) = input_complete_type.unwrap_integer_bounds();
                let (out_size_min, out_size_max) = (min * -1, (max * -1) + 1);
                let expected_out = context
                    .type_substitutor
                    .new_int_type(Some(out_size_min), Some(out_size_max));
                context.type_substitutor.unify_report_error(
                    &context.wires[self.out].typ,
                    &expected_out,
                    self.span,
                    "unary output",
                );
                return DelayedConstraintStatus::Resolved;
            }
            if let UnaryOperator::Product | UnaryOperator::Sum = self.op {
                let (array_type, array_size) = input_complete_type.unwrap_array();
                let array_size = array_size.unwrap_value().unwrap_integer();
                let (min, max) = array_type.unwrap_integer_bounds();
                let (out_size_min, out_size_max) = match self.op {
                    UnaryOperator::Sum => (min * array_size, max * array_size + 1),
                    UnaryOperator::Product => {
                        // TODO: This is a potential ICE! Though I expect we'll get rid UnaryOperator::Product soon enough before it matters
                        let array_size_usize = usize::try_from(array_size).unwrap();
                        let potentials: [IBig; 4] = [
                            min.pow(array_size_usize),
                            max.pow(array_size_usize),
                            -min.pow(array_size_usize),
                            -max.pow(array_size_usize),
                        ];

                        (
                            potentials.iter().min().unwrap().clone(),
                            potentials.iter().max().unwrap() + 1,
                        )
                    }
                    _ => unreachable!(),
                };
                let expected_out = context
                    .type_substitutor
                    .new_int_type(Some(out_size_min), Some(out_size_max));
                context.type_substitutor.unify_report_error(
                    &context.wires[self.out].typ,
                    &expected_out,
                    self.span,
                    "unary output",
                );
                return DelayedConstraintStatus::Resolved;
            }
            unreachable!(
                "The BinaryOpTypecheckConstraint should only check Negate, Product and Sum operations but got {}",
                self.op
            );
        } else {
            DelayedConstraintStatus::NoProgress
        }
    }

    fn report_could_not_resolve_error(&self, context: &InstantiationContext<'_, '_>) {
        let mut in_full = context.wires[self.input].typ.clone();
        context.type_substitutor.fully_substitute(&mut in_full);
        let mut out_full = context.wires[self.out].typ.clone();
        context.type_substitutor.fully_substitute(&mut out_full);
        let message = format!(
            "Failed to Typecheck {:?} = {}{:?}",
            out_full, self.op, in_full
        );

        context.errors.error(self.span, message);
    }
}

#[derive(Debug)]
struct BinaryOpTypecheckConstraint {
    op: BinaryOperator,
    left: UUID<WireIDMarker>,
    right: UUID<WireIDMarker>,
    out: UUID<WireIDMarker>,
    span: Span,
}

impl DelayedConstraint<InstantiationContext<'_, '_>> for BinaryOpTypecheckConstraint {
    fn try_apply(&mut self, context: &mut InstantiationContext<'_, '_>) -> DelayedConstraintStatus {
        if let (Some(left_complete_type), Some(right_complete_type)) = (
            context.wires[self.left]
                .typ
                .try_fully_substitute(&context.type_substitutor),
            context.wires[self.right]
                .typ
                .try_fully_substitute(&context.type_substitutor),
        ) {
            let (left_size_min, left_size_max) = left_complete_type.unwrap_integer_bounds();
            let (right_size_min, right_size_max) = right_complete_type.unwrap_integer_bounds();
            let (out_size_min, out_size_max) = match self.op {
                BinaryOperator::Add => (
                    right_size_min + left_size_min,
                    right_size_max + left_size_max + 1,
                ),
                BinaryOperator::Subtract => (
                    left_size_min - right_size_max,
                    left_size_max - right_size_min + 1,
                ),
                BinaryOperator::Multiply => {
                    let potentials = [
                        &left_size_min * &right_size_min,
                        left_size_min * &right_size_max,
                        &left_size_max * right_size_min,
                        left_size_max * right_size_max,
                    ];
                    (
                        potentials.iter().min().unwrap().clone(),
                        potentials.iter().max().unwrap() + IBig::from(1),
                    )
                }
                BinaryOperator::Divide => {
                    if right_size_min == IBig::from(0) {
                        let potentials: [IBig; 2] = [
                            left_size_min / &right_size_max,
                            left_size_max / right_size_max,
                        ];
                        (
                            potentials.iter().min().unwrap().clone(),
                            potentials.iter().max().unwrap() + IBig::from(1),
                        )
                    } else {
                        let potentials = [
                            &left_size_min / &right_size_max,
                            left_size_min / &right_size_min,
                            &left_size_max / right_size_max,
                            left_size_max / right_size_min,
                        ];
                        (
                            potentials.iter().min().unwrap().clone(),
                            potentials.iter().max().unwrap() + IBig::from(1),
                        )
                    }
                }
                BinaryOperator::Modulo => {
                    if !right_size_min > IBig::from(0) {
                        context.errors.error(self.span, "Modulos must be > 0");
                        return DelayedConstraintStatus::NoProgress;
                    }
                    (IBig::from(0), right_size_max.clone())
                }
                _ => {
                    unreachable!("The BinaryOpTypecheckConstraint should only check arithmetic operations but got {}", self.op);
                }
            };
            let expected_out = context
                .type_substitutor
                .new_int_type(Some(out_size_min), Some(out_size_max));
            context.type_substitutor.unify_report_error(
                &context.wires[self.out].typ,
                &expected_out,
                self.span,
                "binary output",
            );
            DelayedConstraintStatus::Resolved
        } else {
            DelayedConstraintStatus::NoProgress
        }
    }

    fn report_could_not_resolve_error(&self, context: &InstantiationContext<'_, '_>) {
        let mut left_full = context.wires[self.left].typ.clone();
        context.type_substitutor.fully_substitute(&mut left_full);
        let mut right_full = context.wires[self.right].typ.clone();
        context.type_substitutor.fully_substitute(&mut right_full);
        let mut out_full = context.wires[self.out].typ.clone();
        context.type_substitutor.fully_substitute(&mut out_full);
        let message = format!(
            "Failed to Typecheck {:?} = {:?} {} {:?}",
            out_full, left_full, self.op, right_full,
        );

        context.errors.error(self.span, message);
    }
}

pub struct LatencyInferenceDelayedConstraint {}
impl DelayedConstraint<InstantiationContext<'_, '_>> for LatencyInferenceDelayedConstraint {
    fn try_apply(&mut self, context: &mut InstantiationContext<'_, '_>) -> DelayedConstraintStatus {
        context.infer_parameters_for_latencies()
    }

    fn report_could_not_resolve_error(&self, _context: &InstantiationContext<'_, '_>) {} // Handled by incomplete submodules themselves
}
