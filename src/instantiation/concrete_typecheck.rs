use crate::typing::{concrete_type::ConcreteType, type_inference::FailedUnification};

use super::*;

use crate::typing::type_inference::HindleyMilner;

impl<'fl, 'l> InstantiationContext<'fl, 'l> {
    fn walk_type_along_path(
        &self,
        mut cur_typ: ConcreteType,
        path: &[RealWirePathElem],
    ) -> ConcreteType {
        for p in path {
            match p {
                RealWirePathElem::ArrayAccess {
                    span: _,
                    idx_wire: _,
                } => {
                    cur_typ = cur_typ.down_array().clone();
                }
            }
        }

        cur_typ
    }

    pub fn typecheck(&mut self) {
        for this_wire_id in self.wires.id_range() {
            let this_wire = &self.wires[this_wire_id];
            let span = self.md.get_instruction_span(this_wire.original_instruction);
            span.debug();

            match &this_wire.source {
                RealWireDataSource::ReadOnly => {}
                RealWireDataSource::Multiplexer {
                    is_state: _,
                    sources: _,
                } => {} // Do muxes later.
                &RealWireDataSource::UnaryOp { op, right } => {
                    let right_typ = self.wires[right].typ.clone();
                    self.wires[this_wire_id]
                        .typ
                        .typecheck_concrete_unary_operator(
                            op,
                            &right_typ,
                            span,
                            &self.linker.types,
                            &self.errors,
                        );
                }
                &RealWireDataSource::BinaryOp { op, left, right } => {
                    let left_typ = self.wires[left].typ.clone();
                    let right_typ = self.wires[right].typ.clone();
                    self.wires[this_wire_id]
                        .typ
                        .typecheck_concrete_binary_operator(
                            op,
                            &left_typ,
                            &right_typ,
                            span,
                            &self.linker.types,
                            &self.errors,
                        );
                }
                RealWireDataSource::Select { root, path } => {
                    let found_typ = self.walk_type_along_path(self.wires[*root].typ.clone(), path);
                    self.wires[this_wire_id].typ.check_or_update_type(
                        &found_typ,
                        span,
                        &self.linker.types,
                        &self.errors,
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

        // Do typechecking of Multiplexers afterwards, because typechecker isn't so smart right now.
        for this_wire_id in self.wires.id_range() {
            let this_wire = &self.wires[this_wire_id];
            let span = self.md.get_instruction_span(this_wire.original_instruction);
            span.debug();

            if let RealWireDataSource::Multiplexer { is_state, sources } = &this_wire.source {
                if let Some(is_state) = is_state {
                    assert!(is_state.is_of_type(&this_wire.typ));
                }
                for s in sources {
                    let source_typ = &self.wires[s.from.from].typ;
                    let destination_typ =
                        self.walk_type_along_path(self.wires[this_wire_id].typ.clone(), &s.to_path);
                    destination_typ.check_type(&source_typ, span, &self.linker.types, &self.errors);
                }
            };
        }

        self.finalize();
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
}
