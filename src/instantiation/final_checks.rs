//! After instantiation, we preform a few final checks of each module.

use ibig::IBig;

use crate::prelude::*;

use crate::{errors::ErrorReference, typing::concrete_type::ConcreteType};

use super::{ModuleTypingContext, RealWireDataSource, RealWirePathElem};

impl<'l> ModuleTypingContext<'l> {
    fn must_be_subtype_of(
        &self,
        found: &ConcreteType,
        expected: &ConcreteType,
        span: Span,
    ) -> Option<ErrorReference<'_>> {
        if !found.is_subtype_of(expected) {
            Some(self.errors.error(
                span,
                format!(
                    "Typecheck error: Found {}, which is not a subtype of the expected type {}",
                    found.display(&self.linker.types, true),
                    expected.display(&self.linker.types, true)
                ),
            ))
        } else {
            None
        }
    }
    fn check_all_subtypes_in_wires(&self) {
        for (_, w) in &self.wires {
            let span = w.get_span(self.link_info);
            match &w.source {
                RealWireDataSource::ReadOnly => {}
                RealWireDataSource::Multiplexer { is_state, sources } => {
                    if let Some(is_state) = is_state {
                        if !is_state.is_of_type(&w.typ) {
                            self.errors.error(
                                span,
                                "Wire's initial value is not a subtype of the wire's type!",
                            );
                        }
                    }
                    for s in sources {
                        let mut target_typ = &w.typ;
                        for path_elem in &s.to_path {
                            match path_elem {
                                RealWirePathElem::ArrayAccess { span, idx_wire } => {
                                    let (content, arr_sz) = target_typ.unwrap_array();
                                    let arr_sz = arr_sz.unwrap_integer();
                                    target_typ = content;
                                    let idx_wire = &self.wires[*idx_wire];
                                    let (min, max) = idx_wire.typ.unwrap_integer_bounds();
                                    if min < &IBig::from(0) || max >= arr_sz {
                                        self.errors.error(span.inner_span(), format!("Out of bounds! The array is of size {arr_sz}, but the index has bounds {min}..{max}"));
                                    }
                                }
                            }
                        }
                        let from_wire = &self.wires[s.from];
                        self.must_be_subtype_of(
                            &from_wire.typ,
                            target_typ,
                            from_wire.get_span(self.link_info),
                        );
                    }
                }
                RealWireDataSource::ConstructArray { array_wires } => {
                    let (array_content_supertyp, array_size) = w.typ.unwrap_array();

                    let array_wires_len = array_wires.len();
                    let expected_array_size: usize = array_size.unwrap_int();

                    if array_wires_len != expected_array_size {
                        self.errors.error(span, format!("This construct creates an array of size {array_wires_len}, but the expected size is {expected_array_size}"));
                    }

                    for arr_wire in array_wires {
                        let arr_wire = &self.wires[*arr_wire];
                        self.must_be_subtype_of(
                            &arr_wire.typ,
                            array_content_supertyp,
                            arr_wire.get_span(self.link_info),
                        );
                    }
                }
                _ => {}
            }
        }
    }
    // This is unneeded for now. We already handle reporting subtypes by overwriting Multiplexer types
    /*fn check_all_subtypes_in_submodules(&self) {
        for (_, sm) in &self.submodules {
            let Some(instance) = sm.instance.get() else {
                continue;
            };
            let sub_module = &self.linker.modules[sm.refers_to.id];

            for (_port_id, concrete_port, source_code_port, connecting_wire) in
                zip_eq3(&instance.interface_ports, &sub_module.ports, &sm.port_map)
            {
                let (Some(concrete_port), Some(connecting_wire)) = (concrete_port, connecting_wire)
                else {
                    continue;
                };

                let connecting_wire = &self.wires[connecting_wire.maps_to_wire];

                let err = if concrete_port.is_input {
                    self.must_be_subtype_of(&connecting_wire.typ, &concrete_port.typ, span)
                } else {
                    self.must_be_subtype_of(&concrete_port.typ, &connecting_wire.typ, span)
                };
                if let Some(err) = err {
                    err.info_obj_different_file(source_code_port, sub_module.link_info.file);
                }
            }
        }
    }*/
    pub fn check_subtypes(&self) {
        self.check_all_subtypes_in_wires();
    }
}
