use crate::typing::{concrete_type::{ConcreteType, BOOL_CONCRETE_TYPE, INT_CONCRETE_TYPE}, type_inference::FailedUnification};

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
                        let source_typ = &self.wires[s.from.from].typ;
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
            if let Err(()) = w.typ.fully_substitute(&self.type_substitutor) {
                let typ_as_str = w.typ.to_string(&self.linker.types);
                
                let span = self.md.get_instruction_span(w.original_instruction);
                self.errors.error(span, format!("Could not finalize this type, some parameters were still unknown: {typ_as_str}"));
            }
        }

        // Print all errors
        for FailedUnification{mut found, mut expected, span, context} in self.type_substitutor.extract_errors() {
            // Not being able to fully substitute is not an issue. We just display partial types
            let _ = found.fully_substitute(&self.type_substitutor);
            let _ = expected.fully_substitute(&self.type_substitutor);
    
            let expected_name = expected.to_string(&self.linker.types);
            let found_name = found.to_string(&self.linker.types);
            self.errors.error(span, format!("Typing Error: {context} expects a {expected_name} but was given a {found_name}"));
    
            assert!(
                expected_name != found_name,
                "{expected_name} != {found_name}"
            );
        }
    }

    pub fn typecheck(&mut self) {
        self.typecheck_all_wires();

        self.finalize();
    }
}
