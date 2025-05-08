//! After instantiation, we preform a few final checks of each module.

use crate::{typing::concrete_type::ConcreteType, value::Value};

use super::{InstantiationContext, RealWireDataSource, RealWirePathElem};
impl InstantiationContext<'_, '_> {
    fn check_array_accesses_in(&self, path: &[RealWirePathElem], mut arr_typ: &ConcreteType) {
        for elem in path {
            let ConcreteType::Array(arr) = arr_typ else {
                break;
            }; // May still contain unknowns
            let ConcreteType::Value(Value::Integer(arr_sz)) = &arr.1 else {
                break;
            }; // May still contain unknowns
            arr_typ = &arr.0;
            match elem {
                RealWirePathElem::ArrayAccess { span, idx_wire } => {
                    let idx_wire_wire = &self.wires[*idx_wire];
                    if let RealWireDataSource::Constant { value } = &idx_wire_wire.source {
                        // Constant access into array! We can check.
                        let integer_value = value.unwrap_integer();
                        if integer_value >= arr_sz || integer_value < &ibig::ibig!(0) {
                            self.errors
                                .error(span.inner_span(), format!("Index out of bounds. Array is of size {arr_sz}, but the index is {integer_value}."));
                        }
                    }
                }
                RealWirePathElem::ArraySlice {
                    span,
                    idx_a_wire,
                    idx_b_wire,
                } => {
                    let wire_a = &self.wires[*idx_a_wire];
                    let wire_b = &self.wires[*idx_b_wire];

                    for wire in [wire_a, wire_b] {
                        if let RealWireDataSource::Constant { value } = &wire.source {
                            // Constant access into array! We can check.
                            let integer_value = value.unwrap_integer();
                            if integer_value >= arr_sz || integer_value < &ibig::ibig!(0) {
                                self.errors
                                    .error(span.inner_span(), format!("Index out of bounds. Array is of size {arr_sz}, but the index is {integer_value}."));
                            }
                        }
                    }
                }
            }
        }
    }
    pub fn check_array_accesses(&self) {
        for (_id, w) in &self.wires {
            match &w.source {
                RealWireDataSource::Select { root, path } => {
                    let from = &self.wires[*root];
                    self.check_array_accesses_in(path, &from.typ);
                }
                RealWireDataSource::Multiplexer {
                    is_state: _,
                    sources,
                } => {
                    for s in sources {
                        self.check_array_accesses_in(&s.to_path, &w.typ);
                    }
                }
                _ => {}
            }
        }
    }
}
