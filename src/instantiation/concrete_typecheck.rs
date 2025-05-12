use ibig::IBig;

use crate::alloc::{zip_eq, zip_eq3};
use crate::typing::abstract_type::PeanoType;
use crate::typing::{
    concrete_type::{ConcreteType, BOOL_CONCRETE_TYPE, INT_CONCRETE_TYPE},
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
                // TODO #28 integer size <-> array bound check
                RealWirePathElem::ArrayAccess {
                    span: _,
                    idx_wire: _,
                } => {
                    let arr_size = type_substitutor.alloc_unknown();
                    let arr_box = Box::new((typ_after_applying_array.clone(), arr_size));
                    type_substitutor.unify_must_succeed(
                        &current_type_in_progress,
                        &ConcreteType::Array(arr_box),
                    );
                    current_type_in_progress = typ_after_applying_array;
                }
                RealWirePathElem::ArraySlice {
                    span: _,
                    idx_a_wire: _,
                    idx_b_wire: _,
                } => {
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
    }

    fn typecheck_all_wires(&mut self) {
        for this_wire_id in self.wires.id_range() {
            let this_wire = &self.wires[this_wire_id];
            let span = self.md.get_instruction_span(this_wire.original_instruction);
            span.debug();

            match &this_wire.source {
                RealWireDataSource::ReadOnly => {}
                RealWireDataSource::Multiplexer { is_state, sources } => {
                    if let Some(is_state) = is_state {
                        let value_typ = is_state.get_type(&mut self.type_substitutor);
                        self.type_substitutor.unify_report_error(
                            &value_typ,
                            &this_wire.typ,
                            span,
                            "initial value of state",
                        );
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
                        UnaryOperator::Negate => (INT_CONCRETE_TYPE, INT_CONCRETE_TYPE),
                        UnaryOperator::And | UnaryOperator::Or | UnaryOperator::Xor => (
                            self.type_substitutor.make_array_of(BOOL_CONCRETE_TYPE),
                            BOOL_CONCRETE_TYPE,
                        ),
                        UnaryOperator::Sum | UnaryOperator::Product => (
                            self.type_substitutor.make_array_of(INT_CONCRETE_TYPE),
                            INT_CONCRETE_TYPE,
                        ),
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
                        BinaryOperator::Add => {
                            ((INT_CONCRETE_TYPE, INT_CONCRETE_TYPE), INT_CONCRETE_TYPE)
                        }
                        BinaryOperator::Subtract => {
                            ((INT_CONCRETE_TYPE, INT_CONCRETE_TYPE), INT_CONCRETE_TYPE)
                        }
                        BinaryOperator::Multiply => {
                            ((INT_CONCRETE_TYPE, INT_CONCRETE_TYPE), INT_CONCRETE_TYPE)
                        }
                        BinaryOperator::Divide => {
                            ((INT_CONCRETE_TYPE, INT_CONCRETE_TYPE), INT_CONCRETE_TYPE)
                        }
                        BinaryOperator::Modulo => {
                            ((INT_CONCRETE_TYPE, INT_CONCRETE_TYPE), INT_CONCRETE_TYPE)
                        }
                        BinaryOperator::Equals => {
                            ((INT_CONCRETE_TYPE, INT_CONCRETE_TYPE), BOOL_CONCRETE_TYPE)
                        }
                        BinaryOperator::NotEquals => {
                            ((INT_CONCRETE_TYPE, INT_CONCRETE_TYPE), BOOL_CONCRETE_TYPE)
                        }
                        BinaryOperator::GreaterEq => {
                            ((INT_CONCRETE_TYPE, INT_CONCRETE_TYPE), BOOL_CONCRETE_TYPE)
                        }
                        BinaryOperator::Greater => {
                            ((INT_CONCRETE_TYPE, INT_CONCRETE_TYPE), BOOL_CONCRETE_TYPE)
                        }
                        BinaryOperator::LesserEq => {
                            ((INT_CONCRETE_TYPE, INT_CONCRETE_TYPE), BOOL_CONCRETE_TYPE)
                        }
                        BinaryOperator::Lesser => {
                            ((INT_CONCRETE_TYPE, INT_CONCRETE_TYPE), BOOL_CONCRETE_TYPE)
                        }
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
        for (sm_id, sm) in &self.submodules {
            let sub_module = &self.linker.modules[sm.refers_to.id];

            for (port_id, p) in sm.port_map.iter_valids() {
                let wire = &self.wires[p.maps_to_wire];

                let port_decl_instr = sub_module.ports[port_id].declaration_instruction;
                let port_decl =
                    sub_module.link_info.instructions[port_decl_instr].unwrap_declaration();

                let typ_for_inference = self
                    .type_substitutor
                    .concretize_written_type_with_possible_template_args(
                        &port_decl.typ_expr,
                        &sm.refers_to.template_args,
                        &sub_module.link_info,
                    );

                self.type_substitutor
                    .unify_must_succeed(&wire.typ, &typ_for_inference);
            }

            delayed_constraints.push(SubmoduleTypecheckConstraint { sm_id });
        }

        self.typecheck_all_wires();

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
            if !context.type_substitutor.fully_substitute(arg) {
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

        let submodule_template_args_string =
            sm.refers_to.pretty_print_concrete_instance(context.linker);
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
