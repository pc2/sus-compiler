use crate::{
    alloc::FlatAlloc,
    errors::ErrorCollector,
    flattening::{ClockVisibility, Module},
    instantiation::{InstantiatedClock, RealWire, SubModule, unique_names::UniqueNames},
    prelude::{ClockID, ClockIDMarker, SubModuleIDMarker, WireIDMarker},
};

pub fn process_clocks(
    wires: &mut FlatAlloc<RealWire, WireIDMarker>,
    submodules: &mut FlatAlloc<SubModule, SubModuleIDMarker>,
    unique_name_producer: &mut UniqueNames,
    md: &Module,
    errors: &ErrorCollector,
) -> FlatAlloc<InstantiatedClock, ClockIDMarker> {
    let mut clocks = md.interior_clocks.map(|(_, cl)| {
        let name = unique_name_producer.get_unique_name(&cl.name);
        let best_span = if let Some(span) = cl.name_span {
            span
        } else {
            md.link_info.name_span
        };
        InstantiatedClock {
            name,
            best_span,
            visibility: cl.visibility,
            used: false,
            driver: None,
        }
    });
    for (_, w) in &mut *wires {
        let clock = &mut clocks[w.clock];
        if clock.best_span == md.link_info.name_span {
            // We can improve it!
            clock.best_span = w.get_span(&md.link_info);
        }
    }
    for (_, sm) in &mut *submodules {
        for (_, clock) in &sm.clock_map {
            let clock = &mut clocks[*clock];
            if clock.best_span == md.link_info.name_span {
                // We can improve it!
                clock.best_span = sm.get_span(&md.link_info);
            }
        }
    }

    check_clocks_have_one_driver(wires, submodules, &mut clocks, md, errors);

    clocks
}

/// Reports errors, and just to give more opportunities for codegen on unconnected wires,
/// turn unconnected clocks error for local clocks into a warning and default to the first clock.
fn check_clocks_have_one_driver(
    wires: &mut FlatAlloc<RealWire, WireIDMarker>,
    submodules: &mut FlatAlloc<SubModule, SubModuleIDMarker>,
    clocks: &mut FlatAlloc<InstantiatedClock, ClockIDMarker>,
    md: &Module,
    errors: &ErrorCollector,
) {
    for (submod_id, sm) in &*submodules {
        let sm_name = &sm.name;
        for (submod_clk_id, parent_clk_id) in &sm.clock_map {
            let clock_in_self = &mut clocks[*parent_clk_id];
            clock_in_self.used = true;
            if let Some(instance) = sm.instance.get() {
                if instance.clocks[submod_clk_id].visibility == ClockVisibility::Output {
                    let clock_in_submod = &instance.clocks[submod_clk_id].name;

                    assert!(clock_in_self.visibility != ClockVisibility::Input);
                    if let Some((existing_submodule, existing_clock)) = clock_in_self.driver {
                        let clock_name = &clock_in_self.name;
                        let existing_submodule = &submodules[existing_submodule];

                        let existing_module_name = &existing_submodule.name;
                        let existing_module_clock = &existing_submodule.instance.get().expect("Since this submodule was registered as a valid driver to this clock, it must have a avalid instance").clocks[existing_clock].name;
                        let error_text = format!(
                            "output clock '{sm_name}.{clock_in_submod}' tries to drive '{clock_name}', but this clock is already being driven by '{existing_module_name}.{existing_module_clock}'"
                        );
                        errors
                            .error(sm.get_span(&md.link_info), error_text)
                            .info_obj((existing_submodule, &md.link_info));
                    } else {
                        clock_in_self.driver = Some((submod_id, submod_clk_id));
                    }
                }
            }
        }
    }

    let (default_clock_id, default_clock) = clocks.iter().next().unwrap();
    let mut clock_map: FlatAlloc<ClockID, ClockIDMarker> = clocks.map(|(id, _)| id);
    let default_clock_name = &default_clock.name;
    for (clk_id, clk) in &*clocks {
        if clk.visibility != ClockVisibility::Input && clk.driver.is_none() {
            let clk_name = &clk.name;
            match clk.visibility {
                ClockVisibility::Input => {}
                ClockVisibility::Output => {
                    errors.error(clk.best_span, format!("Output clock '{clk_name}' has no driver. output clocks require exactly one driving submodule."));
                }
                ClockVisibility::Local => {
                    errors.warn(clk.best_span, format!("Clock '{clk_name}' has no driver. Non-input clocks require exactly one driving submodule. Defaulting to '{default_clock_name}'"));
                    clock_map[clk_id] = default_clock_id;
                }
            }
        }
    }

    // Replace all undriven clocks with the default clock.
    for (_, wire) in &mut *wires {
        wire.clock = clock_map[wire.clock];
    }
    for (_, submod) in &mut *submodules {
        for (_, clk) in &mut submod.clock_map {
            *clk = clock_map[*clk];
        }
    }
}
