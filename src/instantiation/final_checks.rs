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
                    // [0:3] and [3:0] are the reverse of each other as slices: for this to work, the smaller index must be
                    //
                    let wire_a = &self.wires[*idx_a_wire];
                    let wire_b = &self.wires[*idx_b_wire];

                    if let RealWireDataSource::Constant { value } = &wire_a.source {
                        // Constant access into array! We can check.
                        let integer_value = value.unwrap_integer();
                        if integer_value > arr_sz || integer_value < &ibig::ibig!(0) {
                            self.errors
                                .error(span.inner_span(), format!("Slice start index out of bounds. Array is of size {arr_sz}, but the index is {integer_value}."));
                        }
                    } else {
                        unreachable!()
                    };
                    if let RealWireDataSource::Constant { value } = &wire_b.source {
                        let integer_value = value.unwrap_integer();
                        if integer_value > arr_sz || integer_value < &ibig::ibig!(0) {
                            self.errors
                                .error(span.inner_span(), format!("Slice end index out of bounds. Array is of size {arr_sz}, but the index is {integer_value}."));
                        }
                    } else {
                        unreachable!()
                    };
                }
                RealWirePathElem::ArrayPartSelectDown {
                    span,
                    idx_a_wire,
                    width_wire,
                }
                | RealWirePathElem::ArrayPartSelectUp {
                    span,
                    idx_a_wire,
                    width_wire,
                } => {
                    let wire_start = &self.wires[*idx_a_wire];
                    let wire_width = &self.wires[*width_wire];

                    if let RealWireDataSource::Constant { value: value_width } = &wire_width.source
                    {
                        // Width is constant, can check
                        let integer_value = value_width.unwrap_integer();
                        if integer_value > arr_sz || integer_value < &ibig::ibig!(0) {
                            self.errors
                                .error(span.inner_span(), format!("Width out of bounds. Array is of size {arr_sz}, but slice width is {integer_value}."));
                        }
                        if let RealWireDataSource::Constant { value } = &wire_start.source {
                            // Start index is constant, we can check.
                            let start_integer_value = value.unwrap_integer();
                            if start_integer_value >= arr_sz
                                || start_integer_value < &ibig::ibig!(0)
                            {
                                self.errors
                                    .error(span.inner_span(), format!("Index out of bounds. Array is of size {arr_sz}, but start index is {start_integer_value}."));
                            }
                            // Both width and start are constant, can fully check
                            let width_integer_value = value_width.unwrap_integer();

                            let end = &match elem {
                                RealWirePathElem::ArrayPartSelectDown { .. } => {
                                    start_integer_value - width_integer_value
                                }
                                RealWirePathElem::ArrayPartSelectUp { .. } => {
                                    start_integer_value + width_integer_value
                                }
                                _ => unreachable!(),
                            };
                            if end > arr_sz || start_integer_value < &ibig::ibig!(0) {
                                self.errors
                                    .error(span.inner_span(), format!("Endpoint of part-select out of bounds. Array is of size {arr_sz}, but start {start_integer_value} and width {width_integer_value} yields an endpoint of {end}."));
                            }
                        }
                    } else {
                        unreachable!()
                    };
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
