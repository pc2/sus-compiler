use crate::typing::concrete_type::ConcreteType;

use super::*;

impl<'fl, 'l> InstantiationContext<'fl, 'l> {
    fn walk_type_along_path(&self, mut cur_typ : ConcreteType, path : &[RealWirePathElem]) -> ConcreteType {
        for p in path {
            match p {
                RealWirePathElem::ArrayAccess { span:_, idx_wire:_ } => {
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
                RealWireDataSource::Multiplexer { is_state:_, sources:_ } => {} // Do muxes later. 
                &RealWireDataSource::UnaryOp { op, right } => {
                    let right_typ = self.wires[right].typ.clone();
                    self.wires[this_wire_id].typ.typecheck_concrete_unary_operator(op, &right_typ, span, &self.linker.types, &self.errors);
                }
                &RealWireDataSource::BinaryOp { op, left, right } => {
                    let left_typ = self.wires[left].typ.clone();
                    let right_typ = self.wires[right].typ.clone();
                    self.wires[this_wire_id].typ.typecheck_concrete_binary_operator(op, &left_typ, &right_typ, span, &self.linker.types, &self.errors);
                }
                RealWireDataSource::Select { root, path } => {
                    let found_typ = self.walk_type_along_path(self.wires[*root].typ.clone(), path);
                    self.wires[this_wire_id].typ.check_or_update_type(&found_typ, span, &self.linker.types, &self.errors);
                }
                RealWireDataSource::Constant { value } => {
                    assert!(value.is_of_type(&this_wire.typ), "Assigned type to a constant should already be of the type");
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
                    let destination_typ = self.walk_type_along_path(self.wires[this_wire_id].typ.clone(), &s.to_path);
                    destination_typ.check_type(&source_typ, span, &self.linker.types, &self.errors);
                }
            };
        }
    }
}
