use std::ops::Deref;

use sus_proc_macro::get_builtin_type;

use crate::alloc::{zip_eq, zip_eq3, UUID};
use crate::errors::ErrorInfoObject;
use crate::flattening::{DeclarationKind, ExpressionSource, WireReferenceRoot, WrittenType};
use crate::linker::LinkInfo;
use crate::typing::concrete_type::ConcreteGlobalReference;
use crate::typing::template::TemplateArgKind;
use crate::typing::{
    concrete_type::{ConcreteType, BOOL_CONCRETE_TYPE},
    type_inference::{
        DelayedConstraint, DelayedConstraintStatus, DelayedConstraintsList, FailedUnification,
    },
};

use super::*;

use crate::typing::type_inference::HindleyMilner;

impl InstantiationContext<'_, '_> {
    fn walk_type_along_path(
        &self,
        mut current_type_in_progress: ConcreteType,
        path: &[RealWirePathElem],
    ) -> ConcreteType {
        for p in path {
            let typ_after_applying_array = ConcreteType::Unknown(self.type_substitutor.alloc());
            match p {
                RealWirePathElem::ArrayAccess {
                    span: _,
                    idx_wire: _,
                } => {
                    // TODO #28 integer size <-> array bound check
                    let arr_size = ConcreteType::Unknown(self.type_substitutor.alloc());
                    let arr_box = Box::new((typ_after_applying_array.clone(), arr_size));
                    self.type_substitutor.unify_must_succeed(
                        &current_type_in_progress,
                        &ConcreteType::Array(arr_box),
                    );
                    current_type_in_progress = typ_after_applying_array;
                }
            }
        }

        current_type_in_progress
    }

    fn make_array_of(&self, concrete_typ: ConcreteType) -> ConcreteType {
        ConcreteType::Array(Box::new((
            concrete_typ,
            ConcreteType::Unknown(self.type_substitutor.alloc()),
        )))
    }

    fn typecheck_all_wires(&self, delayed_constraints: &mut DelayedConstraintsList<Self>) {
        for this_wire_id in self.wires.id_range() {
            let this_wire = &self.wires[this_wire_id];
            let span = self.md.get_instruction_span(this_wire.original_instruction);
            span.debug();

            match &this_wire.source {
                RealWireDataSource::ReadOnly => {}
                RealWireDataSource::Multiplexer { is_state, sources } => {
                    if let Some(is_state) = is_state {
                        assert!(is_state.is_of_type(&this_wire.typ));
                    }
                    for s in sources {
                        let source_typ = &self.wires[s.from].typ;
                        let destination_typ = self
                            .walk_type_along_path(self.wires[this_wire_id].typ.clone(), &s.to_path);
                        self.type_substitutor.unify_report_error(
                            &destination_typ,
                            source_typ,
                            span,
                            "write wire access",
                        );
                    }
                }
                &RealWireDataSource::UnaryOp { op, right } => {
                    // TODO overloading
                    let (input_typ, output_typ) = match op {
                        UnaryOperator::Not => (BOOL_CONCRETE_TYPE, BOOL_CONCRETE_TYPE),
                        UnaryOperator::Negate => (
                            self.type_substitutor.new_int_type(),
                            self.type_substitutor.new_int_type(),
                        ),
                        UnaryOperator::And | UnaryOperator::Or | UnaryOperator::Xor => {
                            (self.make_array_of(BOOL_CONCRETE_TYPE), BOOL_CONCRETE_TYPE)
                        }
                        UnaryOperator::Sum | UnaryOperator::Product => (
                            self.make_array_of(self.type_substitutor.new_int_type()),
                            self.type_substitutor.new_int_type(),
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
                &RealWireDataSource::BinaryOp { op, left, right } => {
                    // TODO overloading
                    // Typecheck generic INT
                    let ((in_left, in_right), out) = match op {
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
                        BinaryOperator::Equals => (
                            (
                                self.type_substitutor.new_int_type(),
                                self.type_substitutor.new_int_type(),
                            ),
                            BOOL_CONCRETE_TYPE,
                        ),
                        BinaryOperator::NotEquals => (
                            (
                                self.type_substitutor.new_int_type(),
                                self.type_substitutor.new_int_type(),
                            ),
                            BOOL_CONCRETE_TYPE,
                        ),
                        BinaryOperator::GreaterEq => (
                            (
                                self.type_substitutor.new_int_type(),
                                self.type_substitutor.new_int_type(),
                            ),
                            BOOL_CONCRETE_TYPE,
                        ),
                        BinaryOperator::Greater => (
                            (
                                self.type_substitutor.new_int_type(),
                                self.type_substitutor.new_int_type(),
                            ),
                            BOOL_CONCRETE_TYPE,
                        ),
                        BinaryOperator::LesserEq => (
                            (
                                self.type_substitutor.new_int_type(),
                                self.type_substitutor.new_int_type(),
                            ),
                            BOOL_CONCRETE_TYPE,
                        ),
                        BinaryOperator::Lesser => (
                            (
                                self.type_substitutor.new_int_type(),
                                self.type_substitutor.new_int_type(),
                            ),
                            BOOL_CONCRETE_TYPE,
                        ),
                    };
                    self.type_substitutor.unify_report_error(
                        &self.wires[this_wire_id].typ,
                        &out,
                        span,
                        "binary output",
                    );
                    self.type_substitutor.unify_report_error(
                        &self.wires[left].typ,
                        &in_left,
                        span,
                        "binary left",
                    );
                    self.type_substitutor.unify_report_error(
                        &self.wires[right].typ,
                        &in_right,
                        span,
                        "binary right",
                    );
                }
                RealWireDataSource::Select { root, path } => {
                    let found_typ = self.walk_type_along_path(self.wires[*root].typ.clone(), path);
                    self.type_substitutor.unify_report_error(
                        &found_typ,
                        &self.wires[this_wire_id].typ,
                        span,
                        "wire access",
                    );
                }
                RealWireDataSource::Constant { value } => {
                    assert!(
                        value.is_of_type(&this_wire.typ),
                        "Assigned type to a constant should already be of the type"
                    );
                }
            };
        }
    }

    fn finalize(&mut self) {
        for (_id, w) in &mut self.wires {
            if !w.typ.fully_substitute(&self.type_substitutor) {
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
            let _ = found.fully_substitute(&self.type_substitutor);
            let _ = expected.fully_substitute(&self.type_substitutor);

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
            let sub_module = &self.linker.modules[sm.module_uuid];

            for (port_id, p) in sm.port_map.iter_valids() {
                let wire = &self.wires[p.maps_to_wire];

                let port_decl_instr = sub_module.ports[port_id].declaration_instruction;
                let port_decl =
                    sub_module.link_info.instructions[port_decl_instr].unwrap_declaration();

                let typ_for_inference = concretize_written_type_with_possible_template_args(
                    &port_decl.typ_expr,
                    &sm.template_args,
                    &sub_module.link_info,
                    &self.type_substitutor,
                );

                self.type_substitutor
                    .unify_must_succeed(&wire.typ, &typ_for_inference);
            }

            delayed_constraints.push(SubmoduleTypecheckConstraint { sm_id });
        }

        self.typecheck_all_wires(&mut delayed_constraints);

        delayed_constraints.resolve_delayed_constraints(self);

        self.finalize();
    }
}

struct SubmoduleTypecheckConstraint {
    sm_id: SubModuleID,
}

/// Part of Template Value Inference.
///
/// Specifically, for code like this:
///
/// ```sus
/// module add_all #(int Size) {
///     input int[Size] arr // We're targeting the 'Size' within the array size
///     output int total
/// }
/// ```
fn can_expression_be_value_inferred(link_info: &LinkInfo, expr_id: FlatID) -> Option<TemplateID> {
    let expr = link_info.instructions[expr_id].unwrap_expression();
    let ExpressionSource::WireRef(wr) = &expr.source else {
        return None;
    };
    if !wr.path.is_empty() {
        return None;
    } // Must be a plain, no fuss reference to a de
    let WireReferenceRoot::LocalDecl(wire_declaration, _span) = &wr.root else {
        return None;
    };
    let template_arg_decl = link_info.instructions[*wire_declaration].unwrap_declaration();
    let DeclarationKind::GenerativeInput(template_id) = &template_arg_decl.decl_kind else {
        return None;
    };
    Some(*template_id)
}

fn concretize_written_type_with_possible_template_args(
    written_typ: &WrittenType,
    template_args: &TVec<ConcreteType>,
    link_info: &LinkInfo,
    type_substitutor: &TypeSubstitutor<ConcreteType, ConcreteTypeVariableIDMarker>,
) -> ConcreteType {
    match written_typ {
        WrittenType::Error(_span) => ConcreteType::Unknown(type_substitutor.alloc()),
        WrittenType::TemplateVariable(_span, uuid) => template_args[*uuid].clone(),
        WrittenType::Named(global_reference) => {
            let object_template_args: TVec<ConcreteType> =
                global_reference
                    .template_args
                    .map(|(_arg_id, arg)| -> ConcreteType {
                        if let Some(arg) = arg {
                            match &arg.kind {
                                TemplateArgKind::Type(arg_wr_typ) => {
                                    concretize_written_type_with_possible_template_args(
                                        arg_wr_typ,
                                        template_args,
                                        link_info,
                                        type_substitutor,
                                    )
                                }
                                TemplateArgKind::Value(uuid) => {
                                    if let Some(found_template_arg) =
                                        can_expression_be_value_inferred(link_info, *uuid)
                                    {
                                        template_args[found_template_arg].clone()
                                    } else {
                                        ConcreteType::Unknown(type_substitutor.alloc())
                                    }
                                }
                            }
                        } else {
                            ConcreteType::Unknown(type_substitutor.alloc())
                        }
                    });

            ConcreteType::Named(ConcreteGlobalReference {
                id: global_reference.id,
                template_args: object_template_args,
            })
        }
        WrittenType::Array(_span, arr_box) => {
            let (arr_content_wr, arr_idx_id, _arr_brackets) = arr_box.deref();

            let arr_content_concrete = concretize_written_type_with_possible_template_args(
                arr_content_wr,
                template_args,
                link_info,
                type_substitutor,
            );
            let arr_idx_concrete = if let Some(found_template_arg) =
                can_expression_be_value_inferred(link_info, *arr_idx_id)
            {
                template_args[found_template_arg].clone()
            } else {
                ConcreteType::Unknown(type_substitutor.alloc())
            };

            ConcreteType::Array(Box::new((arr_content_concrete, arr_idx_concrete)))
        }
    }
}

impl SubmoduleTypecheckConstraint {
    /// Directly named type and value parameters are immediately unified, but latency count deltas can only be computed from the latency counting graph
    fn try_infer_latency_counts(&mut self, _context: &mut InstantiationContext) {
        // TODO
    }
}

impl DelayedConstraint<InstantiationContext<'_, '_>> for SubmoduleTypecheckConstraint {
    fn try_apply(&mut self, context: &mut InstantiationContext) -> DelayedConstraintStatus {
        // Try to infer template arguments based on the connections to the ports of the module.
        self.try_infer_latency_counts(context);

        let sm = &mut context.submodules[self.sm_id];

        let submod_instr =
            context.md.link_info.instructions[sm.original_instruction].unwrap_submodule();
        let sub_module = &context.linker.modules[sm.module_uuid];

        // Check if there's any argument that isn't known
        for (_id, arg) in &mut sm.template_args {
            if !arg.fully_substitute(&context.type_substitutor) {
                // We don't actually *need* to already fully_substitute here, but it's convenient and saves some work
                return DelayedConstraintStatus::NoProgress;
            }
        }

        if let Some(instance) = sub_module.instantiations.instantiate(
            sub_module,
            context.linker,
            sm.template_args.clone(),
        ) {
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

            sm.instance
                .set(instance)
                .expect("Can only set the instance of a submodule once");
            DelayedConstraintStatus::Resolved
        } else {
            context.errors.error(
                submod_instr.module_ref.get_total_span(),
                "Error instantiating submodule",
            );
            DelayedConstraintStatus::NoProgress
        }
    }

    fn report_could_not_resolve_error(&self, context: &InstantiationContext) {
        let sm = &context.submodules[self.sm_id];

        let submod_instr =
            context.md.link_info.instructions[sm.original_instruction].unwrap_submodule();
        let sub_module = &context.linker.modules[sm.module_uuid];

        let submodule_template_args_string = pretty_print_concrete_instance(
            &sub_module.link_info,
            &sm.template_args,
            &context.linker.types,
        );
        let message = format!("Could not fully instantiate {submodule_template_args_string}");

        context
            .errors
            .error(submod_instr.get_most_relevant_span(), message);
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
            #[rustfmt::skip]
            let left_size = left_complete_type.unwrap_named().template_args
                [UUID::from_hidden_value(0)]
                .unwrap_value()
                .unwrap_integer();
            #[rustfmt::skip]
            let right_size = right_complete_type.unwrap_named().template_args
                [UUID::from_hidden_value(0)]
                .unwrap_value()
                .unwrap_integer();
            let out_size = match self.op {
                BinaryOperator::Add => left_size + right_size,
                BinaryOperator::Subtract => left_size.clone(),
                BinaryOperator::Multiply => left_size * right_size,
                BinaryOperator::Divide => left_size.clone(),
                BinaryOperator::Modulo => right_size.clone(),
                _ => {
                    unreachable!("The BinaryOpTypecheckConstraint should only check arithmetic operation but got {}", self.op);
                }
            };
            let mut template_args: FlatAlloc<ConcreteType, TemplateIDMarker> = FlatAlloc::new();
            template_args.alloc(ConcreteType::new_int(out_size));
            let expected_out = ConcreteType::Named(ConcreteGlobalReference {
                id: get_builtin_type!("int"),
                template_args,
            });
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
        let message = format!(
            "Failed to Typecheck {:?} = {:?} {} {:?}",
            context.wires[self.out]
                .typ
                .clone()
                .fully_substitute(&context.type_substitutor),
            context.wires[self.right]
                .typ
                .clone()
                .fully_substitute(&context.type_substitutor),
            self.op,
            context.wires[self.left]
                .typ
                .clone()
                .fully_substitute(&context.type_substitutor)
        );

        context.errors.error(self.span, message);
    }
}
