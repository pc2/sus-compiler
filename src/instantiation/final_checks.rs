//! After instantiation, we preform a few final checks of each module.
//! - Check all subtype relations
//! - Check array bounds

use crate::{flattening::ClockVisibility, prelude::*};

use std::borrow::Cow;

use ibig::IBig;

use crate::{
    errors::DiagnosticBuilder,
    instantiation::{
        ModuleTypingContext, PartSelectDirection, RealWire, RealWireDataSource, RealWirePathElem,
    },
    typing::{
        concrete_type::{ConcreteType, IntBounds},
        unifyable_cell::UniCell,
    },
    value::Value,
};

/// Takes a type, and a bunch of slicing operations applied to it, and it returns the resulting type
fn make_output_typ<'c>(typ: &'c ConcreteType, path: &[RealWirePathElem]) -> Cow<'c, ConcreteType> {
    let Some((fst, rest_of_path)) = path.split_first() else {
        return Cow::Borrowed(typ);
    };

    match fst {
        RealWirePathElem::Index { .. } | RealWirePathElem::ConstIndex { .. } => {
            let (content, _) = typ.unwrap_array();
            make_output_typ(content, rest_of_path)
        }
        RealWirePathElem::PartSelect { width, .. } => {
            let (content, _) = typ.unwrap_array();
            let content = make_output_typ(content, rest_of_path).into_owned();
            Cow::Owned(ConcreteType::Array(Box::new((
                content,
                UniCell::from(Value::Integer(width.clone())),
            ))))
        }
        RealWirePathElem::Slice { bounds, .. } => {
            let (content, _) = typ.unwrap_array();
            let content = make_output_typ(content, rest_of_path).into_owned();
            Cow::Owned(ConcreteType::Array(Box::new((
                content,
                UniCell::from(Value::Integer(bounds.unwrap_width())),
            ))))
        }
    }
}

impl<'l> ModuleTypingContext<'l> {
    fn wire_must_be_subtype(
        &self,
        context: &'static str,
        wire: &RealWire,
        expected: &ConcreteType,
    ) -> Option<DiagnosticBuilder<'_>> {
        (!wire.typ.is_subtype_of(expected)).then(|| {
            self.errors.subtype_error(
                context,
                wire.get_span(self.link_info),
                wire.typ.display(self.globals),
                expected.display(self.globals),
            )
        })
    }
    fn boundscheck_array(&self, idx_bounds: IntBounds<&IBig>, sz: &IBig, span: Span, ctx: &str) {
        let array_bound = IntBounds {
            from: &IBig::from(0),
            to: sz,
        };
        if !array_bound.contains_bounds(idx_bounds) {
            self.errors.error(
                span,
                format!(
                    "Out of bounds! The array is of size {sz}, but the {ctx} has bounds {idx_bounds}"
                ),
            );
        }
    }
    fn boundscheck_idx(&self, idx: &IBig, sz: &IBig, span: Span) {
        let array_bound = IntBounds {
            from: &IBig::from(0),
            to: sz,
        };
        if !array_bound.contains(idx) {
            self.errors.error(
                span,
                format!("Out of bounds! The array is of size {sz}, but the index is {idx}"),
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
                    let idx_bounds = wire.typ.unwrap_int_bounds();
                    self.boundscheck_array(idx_bounds, arr_sz, span, "index");
                }
                RealWirePathElem::ConstIndex { span, idx } => {
                    let (content, arr_sz) = typ.unwrap_array_known_size();
                    typ = content;

                    let span = span.inner_span();
                    self.boundscheck_idx(idx, arr_sz, span);
                }
                RealWirePathElem::Slice { span, bounds, .. } => {
                    let idx_bounds = bounds.unwrap_valid();
                    let (content, arr_sz) = typ.unwrap_array_known_size();
                    typ = content;

                    self.boundscheck_array(idx_bounds, arr_sz, span.inner_span(), "slice bound");
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

                    let from_bounds = from_wire.typ.unwrap_int_bounds();

                    let tmp: IBig; // For fixing the lifetime for access_bounds
                    let access_bounds = match direction {
                        PartSelectDirection::Up => {
                            tmp = from_bounds.to + width - 1;
                            IntBounds {
                                from: from_bounds.from,
                                to: &tmp,
                            }
                        }
                        PartSelectDirection::Down => {
                            tmp = from_bounds.from - width + 1;
                            IntBounds {
                                from: &tmp,
                                to: from_bounds.to,
                            }
                        }
                    };
                    let span = span.inner_span();
                    self.boundscheck_array(access_bounds, arr_sz, span, "indexed part-select");
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
                    let root_wire = &self.wires[*root];
                    self.check_wire_ref_bounds(&root_wire.typ, path);
                    let found_output_typ = make_output_typ(&root_wire.typ, path);

                    if !found_output_typ.is_identical_to(&w.typ) {
                        self.errors
                            .type_error(
                                "select",
                                w.get_span(self.link_info),
                                found_output_typ.display(self.globals),
                                w.typ.display(self.globals),
                            )
                            .info(
                                root_wire.get_span(self.link_info),
                                format!(
                                    "{} declared here of type {}",
                                    &root_wire.name,
                                    root_wire.typ.display(self.globals),
                                ),
                            );
                    }
                }
                RealWireDataSource::Multiplexer { is_state, sources } => {
                    if let Some(is_state) = is_state
                        && !is_state.is_of_type(&w.typ)
                    {
                        self.errors.error(
                            w.get_span(self.link_info),
                            "Wire's initial value is not a subtype of the wire's type!",
                        );
                    }
                    for s in sources {
                        self.check_wire_ref_bounds(&w.typ, &s.to_path);
                        let target_typ = make_output_typ(&w.typ, &s.to_path);
                        if let Some(mut e) = self.wire_must_be_subtype(
                            "multiplexer",
                            &self.wires[s.from],
                            &target_typ,
                        ) {
                            e.info(
                                s.write_span,
                                format!(
                                    "Writing to this, which has type {}",
                                    w.typ.display(self.globals)
                                ),
                            );
                        }
                    }
                }
                RealWireDataSource::ConstructArray { array_wires } => {
                    let (arr_content, _sz) = w.typ.unwrap_array_known_size();

                    for arr_wire in array_wires {
                        self.wire_must_be_subtype(
                            "array construct",
                            &self.wires[*arr_wire],
                            arr_content,
                        );
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

    pub fn check_clocks_have_one_driver(&mut self) {
        for (submod_id, sm) in &self.submodules {
            let sm_name = &sm.name;
            for (submod_clk_id, parent_clk_id) in &sm.clock_map {
                let clock_in_self = &mut self.clocks[*parent_clk_id];
                clock_in_self.used = true;
                if let Some(instance) = sm.instance.get() {
                    if instance.clocks[submod_clk_id].visibility == ClockVisibility::Output {
                        let clock_in_submod = &instance.clocks[submod_clk_id].name;

                        assert!(clock_in_self.visibility != ClockVisibility::Input);
                        if let Some((existing_submodule, existing_clock)) = clock_in_self.driver {
                            let clock_name = &clock_in_self.name;
                            let existing_submodule = &self.submodules[existing_submodule];

                            let existing_module_name = &existing_submodule.name;
                            let existing_module_clock = &existing_submodule.instance.get().expect("Since this submodule was registered as a valid driver to this clock, it must have a avalid instance").clocks[existing_clock].name;
                            let error_text = format!(
                                "output clock '{sm_name}.{clock_in_submod}' tries to drive '{clock_name}', but this clock is already being driven by '{existing_module_name}.{existing_module_clock}'"
                            );
                            self.errors
                                .error(sm.get_span(self.link_info), error_text)
                                .info_obj((existing_submodule, self.link_info));
                        } else {
                            clock_in_self.driver = Some((submod_id, submod_clk_id));
                        }
                    }
                }
            }
        }

        let (default_clock_id, default_clock) = self.clocks.iter().next().unwrap();
        let mut clock_map: FlatAlloc<ClockID, ClockIDMarker> = self.clocks.map(|(id, _)| id);
        let default_clock_name = &default_clock.name;
        for (clk_id, clk) in &self.clocks {
            if clk.visibility != ClockVisibility::Input && clk.driver.is_none() {
                let clk_name = &clk.name;
                match clk.visibility {
                    ClockVisibility::Input => {}
                    ClockVisibility::Output => {
                        self.errors.error(clk.best_span, format!("Output clock '{clk_name}' has no driver. output clocks require exactly one driving submodule."));
                    }
                    ClockVisibility::Local => {
                        self.errors.warn(clk.best_span, format!("Clock '{clk_name}' has no driver. Non-input clocks require exactly one driving submodule. Defaulting to '{default_clock_name}'"));
                        clock_map[clk_id] = default_clock_id;
                    }
                }
            }
        }

        // Replace all undriven clocks with the default clock.
        for (_, wire) in &mut self.wires {
            wire.clock = clock_map[wire.clock];
        }
        for (_, submod) in &mut self.submodules {
            for (_, clk) in &mut submod.clock_map {
                *clk = clock_map[*clk];
            }
        }
    }
}
