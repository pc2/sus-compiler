
use std::ops::Deref;

use crate::errors::ErrorInfoObject;
use crate::flattening::{DeclarationKind, WireReferenceRoot, ExpressionSource, WrittenType};
use crate::linker::LinkInfo;
use crate::typing::template::{ConcreteTemplateArg, HowDoWeKnowTheTemplateArg};
use crate::typing::{
    concrete_type::{ConcreteType, BOOL_CONCRETE_TYPE, INT_CONCRETE_TYPE},
    type_inference::{FailedUnification, DelayedConstraint, DelayedConstraintStatus, DelayedConstraintsList},
};

use super::*;

use crate::typing::type_inference::HindleyMilner;

impl<'fl, 'l> InstantiationContext<'fl, 'l> {
    fn walk_type_along_path(
        &self,
        mut current_type_in_progress: ConcreteType,
        path: &[RealWirePathElem]
    ) -> ConcreteType {
        for p in path {
            let typ_after_applying_array = ConcreteType::Unknown(self.type_substitutor.alloc());
            match p {
                RealWirePathElem::ArrayAccess {span: _, idx_wire: _} => { // TODO #28 integer size <-> array bound check
                    let arr_size = ConcreteType::Unknown(self.type_substitutor.alloc());
                    let arr_box = Box::new((typ_after_applying_array.clone(), arr_size));
                    self.type_substitutor.unify_must_succeed(&current_type_in_progress, &ConcreteType::Array(arr_box));
                    current_type_in_progress = typ_after_applying_array;
                }
            }
        }

        current_type_in_progress
    }

    fn make_array_of(&self, concrete_typ: ConcreteType) -> ConcreteType {
        ConcreteType::Array(Box::new((concrete_typ, ConcreteType::Unknown(self.type_substitutor.alloc()))))
    }

    fn typecheck_all_wires(&self) {
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
                        let destination_typ = self.walk_type_along_path(self.wires[this_wire_id].typ.clone(), &s.to_path);
                        self.type_substitutor.unify_report_error(&destination_typ, &source_typ, span, "write wire access");
                    }
                }
                &RealWireDataSource::UnaryOp { op, right } => {
                    // TODO overloading
                    let (input_typ, output_typ) = match op {
                        UnaryOperator::Not => (BOOL_CONCRETE_TYPE, BOOL_CONCRETE_TYPE),
                        UnaryOperator::Negate => (INT_CONCRETE_TYPE, INT_CONCRETE_TYPE),
                        UnaryOperator::And | UnaryOperator::Or | UnaryOperator::Xor => (self.make_array_of(BOOL_CONCRETE_TYPE), BOOL_CONCRETE_TYPE),
                        UnaryOperator::Sum | UnaryOperator::Product => (self.make_array_of(INT_CONCRETE_TYPE), INT_CONCRETE_TYPE),
                    };

                    self.type_substitutor.unify_report_error(&self.wires[right].typ, &input_typ, span, "unary input");
                    self.type_substitutor.unify_report_error(&self.wires[this_wire_id].typ, &output_typ, span, "unary output");
                }
                &RealWireDataSource::BinaryOp { op, left, right } => {
                    // TODO overloading
                    let ((in_left, in_right), out) = match op {
                        BinaryOperator::And => ((BOOL_CONCRETE_TYPE, BOOL_CONCRETE_TYPE), BOOL_CONCRETE_TYPE),
                        BinaryOperator::Or => ((BOOL_CONCRETE_TYPE, BOOL_CONCRETE_TYPE), BOOL_CONCRETE_TYPE),
                        BinaryOperator::Xor => ((BOOL_CONCRETE_TYPE, BOOL_CONCRETE_TYPE), BOOL_CONCRETE_TYPE),
                        BinaryOperator::Add => ((INT_CONCRETE_TYPE, INT_CONCRETE_TYPE), INT_CONCRETE_TYPE),
                        BinaryOperator::Subtract => ((INT_CONCRETE_TYPE, INT_CONCRETE_TYPE), INT_CONCRETE_TYPE),
                        BinaryOperator::Multiply => ((INT_CONCRETE_TYPE, INT_CONCRETE_TYPE), INT_CONCRETE_TYPE),
                        BinaryOperator::Divide => ((INT_CONCRETE_TYPE, INT_CONCRETE_TYPE), INT_CONCRETE_TYPE),
                        BinaryOperator::Modulo => ((INT_CONCRETE_TYPE, INT_CONCRETE_TYPE), INT_CONCRETE_TYPE),
                        BinaryOperator::Equals => ((INT_CONCRETE_TYPE, INT_CONCRETE_TYPE), BOOL_CONCRETE_TYPE),
                        BinaryOperator::NotEquals => ((INT_CONCRETE_TYPE, INT_CONCRETE_TYPE), BOOL_CONCRETE_TYPE),
                        BinaryOperator::GreaterEq => ((INT_CONCRETE_TYPE, INT_CONCRETE_TYPE), BOOL_CONCRETE_TYPE),
                        BinaryOperator::Greater => ((INT_CONCRETE_TYPE, INT_CONCRETE_TYPE), BOOL_CONCRETE_TYPE),
                        BinaryOperator::LesserEq => ((INT_CONCRETE_TYPE, INT_CONCRETE_TYPE), BOOL_CONCRETE_TYPE),
                        BinaryOperator::Lesser => ((INT_CONCRETE_TYPE, INT_CONCRETE_TYPE), BOOL_CONCRETE_TYPE),
                    };
                    self.type_substitutor.unify_report_error(&self.wires[this_wire_id].typ, &out, span, "binary output");
                    self.type_substitutor.unify_report_error(&self.wires[left].typ, &in_left, span, "binary left");
                    self.type_substitutor.unify_report_error(&self.wires[right].typ, &in_right, span, "binary right");
                }
                RealWireDataSource::Select { root, path } => {
                    let found_typ = self.walk_type_along_path(self.wires[*root].typ.clone(), path);
                    self.type_substitutor.unify_report_error(&found_typ, &self.wires[this_wire_id].typ, span, "wire access");
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
            if w.typ.fully_substitute(&self.type_substitutor) == false {
                let typ_as_str = w.typ.display(&self.linker.types);
                
                let span = self.md.get_instruction_span(w.original_instruction);
                self.errors.error(span, format!("Could not finalize this type, some parameters were still unknown: {typ_as_str}"));
            }
        }

        // Print all errors
        for FailedUnification{mut found, mut expected, span, context, infos} in self.type_substitutor.extract_errors() {
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
        let mut delayed_constraints : DelayedConstraintsList<Self> = DelayedConstraintsList::new();
        for (sm_id, _sm) in &self.submodules {
            delayed_constraints.push(SubmoduleTypecheckConstraint {sm_id});
        }

        self.typecheck_all_wires();

        delayed_constraints.resolve_delayed_constraints(self);

        self.finalize();
    }
}

struct SubmoduleTypecheckConstraint {
    sm_id: SubModuleID
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
    let ExpressionSource::WireRef(wr) = &expr.source else {return None};
    if !wr.path.is_empty() {return None} // Must be a plain, no fuss reference to a de
    let WireReferenceRoot::LocalDecl(wire_declaration, _span) = &wr.root else {return None};
    let template_arg_decl = link_info.instructions[*wire_declaration].unwrap_declaration();
    let DeclarationKind::GenerativeInput(template_id) = &template_arg_decl.decl_kind else {return None};
    Some(*template_id)
}

fn try_to_attach_value_to_template_arg(template_wire_referernce: FlatID, found_value: &ConcreteType, template_args: &mut ConcreteTemplateArgs, submodule_link_info: &LinkInfo) {
    let ConcreteType::Value(v) = found_value else {return}; // We don't have a value to assign
    if let Some(template_id) = can_expression_be_value_inferred(submodule_link_info, template_wire_referernce) {
        if let ConcreteTemplateArg::NotProvided = &template_args[template_id] {
            template_args[template_id] = ConcreteTemplateArg::Provided(ConcreteType::Value(v.clone()), HowDoWeKnowTheTemplateArg::Inferred)
        }
    }
}

fn infer_parameters_by_walking_type(port_wr_typ: &WrittenType, connected_typ: &ConcreteType, template_args: &mut ConcreteTemplateArgs, submodule_link_info: &LinkInfo) {
    match port_wr_typ {
        WrittenType::Error(_) => {} // Can't continue, bad written type
        WrittenType::Named(_) => {} // Seems we've run out of type to check
        WrittenType::Array(_span, written_arr_box) => {
            let ConcreteType::Array(concrete_arr_box) = connected_typ else {return}; // Can't continue, type not worked out. TODO should we seed concrete types with derivates from AbstractTypes?
            let (written_arr, written_size_var, _) = written_arr_box.deref();
            let (concrete_arr, concrete_size) = concrete_arr_box.deref();

            infer_parameters_by_walking_type(written_arr, concrete_arr, template_args, submodule_link_info); // Recurse down

            try_to_attach_value_to_template_arg(*written_size_var, concrete_size, template_args, submodule_link_info); // Potential place for template inference!
        }
        WrittenType::TemplateVariable(_span, template_id) => {
            if !connected_typ.contains_unknown() {
                if let ConcreteTemplateArg::NotProvided = &template_args[*template_id] {
                    template_args[*template_id] = ConcreteTemplateArg::Provided(connected_typ.clone(), HowDoWeKnowTheTemplateArg::Inferred)
                }
            }
        }
    }
}

impl SubmoduleTypecheckConstraint {
    fn try_infer_parameters(&mut self, context: &mut InstantiationContext) {
        let sm = &mut context.submodules[self.sm_id];

        let sub_module = &context.linker.modules[sm.module_uuid];

        for (id, p) in sm.port_map.iter_valids() {
            let wire = &context.wires[p.maps_to_wire];

            let mut wire_typ_clone = wire.typ.clone();
            wire_typ_clone.fully_substitute(&context.type_substitutor);

            let port_decl_instr = sub_module.ports[id].declaration_instruction;
            let port_decl = sub_module.link_info.instructions[port_decl_instr].unwrap_declaration();

            infer_parameters_by_walking_type(&port_decl.typ_expr, &wire_typ_clone, &mut sm.template_args, &sub_module.link_info);
        }
    }

}

impl DelayedConstraint<InstantiationContext<'_, '_>> for SubmoduleTypecheckConstraint {
    fn try_apply(&mut self, context : &mut InstantiationContext) -> DelayedConstraintStatus {
        // Try to infer template arguments based on the connections to the ports of the module
        self.try_infer_parameters(context);

        let sm = &context.submodules[self.sm_id];

        let submod_instr = context.md.link_info.instructions[sm.original_instruction].unwrap_submodule();
        let sub_module = &context.linker.modules[sm.module_uuid];

        // Check if there's any argument that isn't known
        for (_id, arg) in &sm.template_args {
            match arg {
                ConcreteTemplateArg::NotProvided => {
                    return DelayedConstraintStatus::NoProgress;
                }
                ConcreteTemplateArg::Provided(..) => {}
            }
        }

        if let Some(instance) = sub_module.instantiations.instantiate(
            sub_module,
            context.linker,
            sm.template_args.clone(),
        ) {
            for (port_id, concrete_port) in &instance.interface_ports {
                let connecting_wire = &sm.port_map[port_id];

                match (concrete_port, connecting_wire) {
                    (None, None) => {} // Invalid port not connected, good!
                    (None, Some(connecting_wire)) => {
                        // Port is not enabled, but attempted to be used
                        // A question may be "What if no port was in the source code? There would be no error reported"
                        // But this is okay, because nonvisible ports are only possible for function calls
                        // We have a second routine that reports invalid interfaces.
                        let source_code_port = &sub_module.ports[port_id];
                        for span in &connecting_wire.name_refs {
                            context.errors.error(*span, format!("Port '{}' is used, but the instantiated module has this port disabled", source_code_port.name))
                                .info_obj_different_file(source_code_port, sub_module.link_info.file)
                                .info_obj_same_file(submod_instr);
                        }
                    }
                    (Some(_concrete_port), None) => {
                        // Port is enabled, but not used
                        let source_code_port = &sub_module.ports[port_id];
                        context.errors
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
                        let wire = &context.wires[connecting_wire.maps_to_wire];
                        context.type_substitutor.unify_report_error(&wire.typ, &concrete_port.typ, submod_instr.module_ref.get_total_span(), || {
                            let abstract_port = &sub_module.ports[port_id];
                            let port_declared_here = abstract_port.make_info(sub_module.link_info.file);

                            (format!("Port '{}'", abstract_port.name), vec![port_declared_here])
                        });
                    }
                }
            }
            for (interface_id, interface_references) in &sm.interface_call_sites {
                if !interface_references.is_empty() {
                    let sm_interface = &sub_module.interfaces[interface_id];
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

            sm.instance.set(instance).expect("Can only set the instance of a submodule once");
            DelayedConstraintStatus::Resolved
        } else {
            context.errors.error(
                submod_instr.module_ref.get_total_span(),
                "Error instantiating submodule",
            );
            DelayedConstraintStatus::NoProgress
        }
    }

    fn report_could_not_resolve_error(&self, context : &InstantiationContext) {
        let sm = &context.submodules[self.sm_id];

        let submod_instr = context.md.link_info.instructions[sm.original_instruction].unwrap_submodule();
        let sub_module = &context.linker.modules[sm.module_uuid];

        let submodule_template_args_string = pretty_print_concrete_instance(&sub_module.link_info, &sm.template_args, &context.linker.types);
        let message = format!("Could not fully instantiate {submodule_template_args_string}");

        context.errors.error(submod_instr.get_most_relevant_span(), message);
    }
}
