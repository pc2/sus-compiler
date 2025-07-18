//! After instantiation, we preform a few final checks of each module.
//! - Check all subtype relations
//! - Check array bounds

use std::borrow::Cow;

use ibig::IBig;

use crate::{
    errors::ErrorReference,
    instantiation::PartSelectDirection,
    typing::{concrete_type::ConcreteType, value_unifier::UnifyableValue},
    value::Value,
};

use super::{ModuleTypingContext, RealWire, RealWireDataSource, RealWirePathElem};

use crate::prelude::*;

fn make_output_typ<'c>(typ: &'c ConcreteType, path: &[RealWirePathElem]) -> Cow<'c, ConcreteType> {
    let Some((fst, rest_of_path)) = path.split_first() else {
        return Cow::Borrowed(typ);
    };

    match fst {
        RealWirePathElem::Index { .. } => {
            let (content, _) = typ.unwrap_array();
            make_output_typ(content, rest_of_path)
        }
        RealWirePathElem::Slice { from, to, .. } => {
            let (content, _) = typ.unwrap_array();
            let content = make_output_typ(content, rest_of_path).into_owned();
            Cow::Owned(ConcreteType::Array(Box::new((
                content,
                UnifyableValue::from(Value::Integer(to.unwrap_integer() - from.unwrap_integer())),
            ))))
        }
        RealWirePathElem::PartSelect { width, .. } => {
            let (content, _) = typ.unwrap_array();
            let content = make_output_typ(content, rest_of_path).into_owned();
            Cow::Owned(ConcreteType::Array(Box::new((
                content,
                UnifyableValue::from(Value::Integer(width.unwrap_integer().clone())),
            ))))
        }
    }
}

impl<'l> ModuleTypingContext<'l> {
    fn wire_must_be_subtype(
        &self,
        wire: &RealWire,
        expected: &ConcreteType,
    ) -> Option<ErrorReference<'_>> {
        (!wire.typ.is_subtype_of(expected)).then(|| {
            self.errors.subtype_error(
                wire.get_span(self.link_info),
                wire.typ.display(&self.linker.globals, true),
                expected.display(&self.linker.globals, true),
            )
        })
    }
    fn check_array_bound_min_max(&self, min: &IBig, max: &IBig, sz: &IBig, span: Span, ctx: &str) {
        if min < &IBig::from(0) || max >= sz {
            self.errors.error(
                span,
                format!(
                    "Out of bounds! The array is of size {sz}, but the {ctx} has bounds {min}:{max}"
                ),
            );
        }
    }

    fn check_wire_ref_bounds(&self, mut typ: &ConcreteType, path: &[RealWirePathElem]) {
        for p in path {
            match p {
                RealWirePathElem::Index { span, idx_wire } => {
                    let (content, arr_sz) = typ.unwrap_array_known_size();
                    typ = content;

                    let span = span.inner_span();
                    let wire = &self.wires[*idx_wire];
                    let (min, max) = wire.typ.unwrap_integer_bounds();
                    self.check_array_bound_min_max(min, max, arr_sz, span, "index");
                }
                RealWirePathElem::Slice {
                    from_span,
                    to_span,
                    from,
                    to,
                } => {
                    let from = from.unwrap_integer();
                    let to = to.unwrap_integer();
                    let (content, arr_sz) = typ.unwrap_array_known_size();
                    typ = content;

                    let span = Span::new_overarching(*from_span, *to_span);
                    self.check_array_bound_min_max(from, to, arr_sz, span, "slice bound");
                }
                RealWirePathElem::PartSelect {
                    span,
                    from_wire,
                    width,
                    direction,
                } => {
                    let (content, arr_sz) = typ.unwrap_array_known_size();
                    typ = content;

                    let from_wire = &self.wires[*from_wire];

                    let (min_a, max_a) = from_wire.typ.unwrap_integer_bounds();

                    let width = width.unwrap_integer();

                    let (lower, upper) = match direction {
                        PartSelectDirection::Up => (min_a.clone(), max_a + width - 1),
                        PartSelectDirection::Down => (min_a - width + 1, max_a.clone()),
                    };

                    self.check_array_bound_min_max(
                        &lower,
                        &upper,
                        arr_sz,
                        span.inner_span(),
                        "indexed part-select",
                    );
                }
            }
        }
    }
    fn check_all_subtypes_in_wires(&self) {
        for (_, w) in &self.wires {
            match &w.source {
                RealWireDataSource::ReadOnly
                | RealWireDataSource::UnaryOp { .. }
                | RealWireDataSource::BinaryOp { .. }
                | RealWireDataSource::Constant { .. } => {}
                RealWireDataSource::Select { root, path } => {
                    self.check_wire_ref_bounds(&self.wires[*root].typ, path);
                }
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
                        self.check_wire_ref_bounds(&w.typ, &s.to_path);
                        let target_typ = make_output_typ(&w.typ, &s.to_path);
                        if let Some(e) = self.wire_must_be_subtype(&self.wires[s.from], &target_typ)
                        {
                            e.info_same_file(
                                s.write_span,
                                format!(
                                    "Writing to this, which has type {}",
                                    w.typ.display(self.linker, true)
                                ),
                            );
                        }
                    }
                }
                RealWireDataSource::ConstructArray { array_wires } => {
                    let (arr_content, _sz) = w.typ.unwrap_array_known_size();

                    for arr_wire in array_wires {
                        self.wire_must_be_subtype(&self.wires[*arr_wire], arr_content);
                    }
                }
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
