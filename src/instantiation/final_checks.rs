//! After instantiation, we preform a few final checks of each module.
//! - Check all subtype relations
//! - Check array bounds

use ibig::IBig;

use crate::{errors::ErrorReference, typing::concrete_type::ConcreteType};

use super::{ModuleTypingContext, RealWire, RealWireDataSource, RealWirePathElem};

use crate::prelude::*;

impl<'l> ModuleTypingContext<'l> {
    fn typecheck_error(
        &self,
        found: &ConcreteType,
        expected: &ConcreteType,
        span: Span,
    ) -> ErrorReference<'_> {
        self.errors.error(
            span,
            format!(
                "Typecheck error: Found {}, which is not a subtype of the expected type {}",
                found.display(self.linker, true),
                expected.display(self.linker, true)
            ),
        )
    }
    fn wire_must_be_subtype(
        &self,
        wire: &RealWire,
        expected: &ConcreteType,
    ) -> Option<ErrorReference<'_>> {
        (!wire.typ.is_subtype_of(expected))
            .then(|| self.typecheck_error(&wire.typ, expected, wire.get_span(self.link_info)))
    }
    fn check_wire_ref_bounds<'c>(
        &self,
        mut typ: &'c ConcreteType,
        path: &[RealWirePathElem],
    ) -> &'c ConcreteType {
        for path_elem in path {
            match path_elem {
                RealWirePathElem::ArrayAccess { span, idx_wire } => {
                    let (content, arr_sz) = typ.unwrap_array();
                    let arr_sz = arr_sz.unwrap_integer();
                    typ = content;
                    let idx_wire = &self.wires[*idx_wire];
                    let (min, max) = idx_wire.typ.unwrap_integer_bounds();
                    if min < &IBig::from(0) || max >= arr_sz {
                        self.errors.error(span.inner_span(), format!("Out of bounds! The array is of size {arr_sz}, but the index has bounds {min}..{max}"));
                    }
                }
            }
        }
        typ
    }
    fn check_all_subtypes_in_wires(&self) {
        for (_, w) in &self.wires {
            match &w.source {
                RealWireDataSource::ReadOnly => {}
                RealWireDataSource::Multiplexer { is_state, sources } => {
                    if let Some(is_state) = is_state {
                        if !is_state.is_of_type(&w.typ) {
                            self.errors.error(
                                w.get_span(self.link_info),
                                "Wire's initial value is not a subtype of the wire's type!",
                            );
                        }
                    }
                    for s in sources {
                        let target_typ = self.check_wire_ref_bounds(&w.typ, &s.to_path);
                        if let Some(e) = self.wire_must_be_subtype(&self.wires[s.from], target_typ)
                        {
                            e.info_same_file(
                                s.write_span,
                                format!(
                                    "Writing to this, which has type {}",
                                    target_typ.display(self.linker, true)
                                ),
                            );
                        }
                    }
                }
                RealWireDataSource::ConstructArray { array_wires } => {
                    let (arr_content, array_size) = w.typ.unwrap_array();

                    let array_wires_len = array_wires.len();
                    let expected_array_size: usize = array_size.unwrap_int();

                    if array_wires_len != expected_array_size {
                        self.errors.error(w.get_span(self.link_info), format!("This construct creates an array of size {array_wires_len}, but the expected size is {expected_array_size}"));
                    }

                    for arr_wire in array_wires {
                        self.wire_must_be_subtype(&self.wires[*arr_wire], arr_content);
                    }
                }
                RealWireDataSource::Select { root, path } => {
                    let root_wire = &self.wires[*root];
                    let found_typ = self.check_wire_ref_bounds(&root_wire.typ, path);
                    let expected_typ = &w.typ;

                    // Yes, we do a comparison here, rather than found_typ.is_subtype_of(expected_typ).
                    // That is because the value of subtype-able parameters should never be able to bubble up into a Select.
                    // Of course, non-subtypeable parameters *can* bubble up, and so we need to check against these.
                    if expected_typ != found_typ {
                        assert!(!expected_typ.is_subtype_of(found_typ), "Subtype-able parameters should never be able to bubble up into a Select!");
                        self.typecheck_error(found_typ, expected_typ, w.get_span(self.link_info))
                            .info_same_file(
                                root_wire.get_span(self.link_info),
                                format!(
                                    "{} declared here of type {}",
                                    &root_wire.name,
                                    root_wire.typ.display(&self.linker.globals, true)
                                ),
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
