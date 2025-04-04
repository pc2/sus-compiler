mod latency_algorithm;
mod list_of_lists;
pub mod port_latency_inference;

use std::{cmp::max, iter::zip};

use crate::alloc::zip_eq;
use crate::prelude::*;

use crate::flattening::{Instruction, WriteModifiers};
use crate::typing::concrete_type::ConcreteType;
use crate::typing::type_inference::{DelayedConstraintStatus, HindleyMilner};
use crate::value::Value;

use latency_algorithm::{
    infer_unknown_latency_edges, is_valid, solve_latencies, FanInOut, LatencyCountingError,
    LatencyCountingPorts, PartialSubmoduleInfo, SpecifiedLatency,
};

use self::list_of_lists::ListOfLists;

use crate::instantiation::*;

// Temporary value before proper latency is given
pub const CALCULATE_LATENCY_LATER: i64 = i64::MIN;

struct PathMuxSource<'s> {
    to_wire: &'s RealWire,
    to_latency: i64,
    mux_input: &'s MultiplexerSource,
}

fn write_path_elem_to_string(
    result: &mut String,
    decl_name: &str,
    to_absolute_latency: i64,
    prev_absolute_latency: i64,
) {
    use std::fmt::Write;

    let delta_latency = to_absolute_latency - prev_absolute_latency;

    let plus_sign = if delta_latency >= 0 { "+" } else { "" };

    writeln!(
        result,
        "-> {decl_name}'{to_absolute_latency} ({plus_sign}{delta_latency})"
    )
    .unwrap();
}

fn make_path_info_string(
    writes: &[PathMuxSource<'_>],
    from_latency: i64,
    from_name: &str,
) -> String {
    let mut prev_decl_absolute_latency = from_latency;
    let mut result = format!("{from_name}'{prev_decl_absolute_latency}\n");

    for wr in writes {
        let decl_name = &wr.to_wire.name;

        let to_absolute_latency = wr.to_latency;

        write_path_elem_to_string(
            &mut result,
            decl_name,
            to_absolute_latency,
            prev_decl_absolute_latency,
        );

        prev_decl_absolute_latency = to_absolute_latency;
    }

    result
}

fn filter_unique_write_flats<'w>(
    writes: &'w [PathMuxSource<'w>],
    instructions: &'w FlatAlloc<Instruction, FlatIDMarker>,
) -> Vec<&'w crate::flattening::Write> {
    let mut result: Vec<&'w crate::flattening::Write> = Vec::new();
    for w in writes {
        if let Instruction::Write(original_write) = &instructions[w.mux_input.original_connection] {
            if !result
                .iter()
                .any(|found_write| std::ptr::eq(*found_write, original_write))
            {
                result.push(original_write)
            }
        }
    }
    result
}

struct WireToLatencyMap {
    map_wire_to_latency_node: FlatAlloc<usize, WireIDMarker>,
    domain_infos: FlatAlloc<LatencyDomainInfo, DomainIDMarker>,
    /// Wires that are ports point to the next port in the chain, to form a complete cycle. This binds all the ports togehter
    next_port_chain: FlatAlloc<Option<(WireID, i64)>, WireIDMarker>,
}

struct LatencyDomainInfo {
    latency_node_meanings: Vec<WireID>,
    initial_values: Vec<SpecifiedLatency>,
    ports: LatencyCountingPorts,
}

impl RealWireDataSource {
    fn iter_sources_with_min_latency(&self, mut f: impl FnMut(WireID, i64)) {
        match self {
            RealWireDataSource::ReadOnly => {}
            RealWireDataSource::Multiplexer {
                is_state: _,
                sources,
            } => {
                for s in sources {
                    f(s.from, s.num_regs);
                    RealWirePathElem::for_each_wire_in_path(&s.to_path, |w| f(w, s.num_regs));
                    for elem in s.condition.iter() {
                        f(elem.condition_wire, s.num_regs);
                    }
                }
            }
            RealWireDataSource::UnaryOp { op: _, right } => {
                f(*right, 0);
            }
            RealWireDataSource::BinaryOp { op: _, left, right } => {
                f(*left, 0);
                f(*right, 0);
            }
            RealWireDataSource::Select { root, path } => {
                f(*root, 0);
                RealWirePathElem::for_each_wire_in_path(path, |w| f(w, 0));
            }
            RealWireDataSource::ConstructArray { array_wires } => {
                for w in array_wires {
                    f(*w, 0);
                }
            }
            RealWireDataSource::Constant { value: _ } => {}
        }
    }
}

impl InstantiatedModule {
    /// Is used to add implicit registers to wires that are used longer than one cycle.
    ///
    /// If needed only the same cycle it is generated, then this is equal to [RealWire::absolute_latency].
    pub fn compute_needed_untils(&self) -> FlatAlloc<i64, WireIDMarker> {
        let mut result = self.wires.map(|(_id, w)| w.absolute_latency);

        for (_id, w) in &self.wires {
            w.source.iter_sources_with_min_latency(|other, _| {
                let nu = &mut result[other];

                *nu = max(*nu, w.absolute_latency);
            });
        }

        result
    }
}

impl InstantiationContext<'_, '_> {
    fn make_wire_to_latency_map(&self) -> WireToLatencyMap {
        const PLACEHOLDER: usize = usize::MAX;

        let mut map_wire_to_latency_node: FlatAlloc<usize, WireIDMarker> =
            self.wires.map(|_| PLACEHOLDER);

        let mut domain_infos: FlatAlloc<LatencyDomainInfo, DomainIDMarker> =
            self.md.domains.id_range().map(|domain| {
                let mut latency_node_meanings = Vec::with_capacity(self.wires.len());
                let mut initial_values = Vec::new();

                for (w_id, w) in &self.wires {
                    if w.domain == domain {
                        let new_idx = latency_node_meanings.len();
                        latency_node_meanings.push(w_id);
                        debug_assert!(map_wire_to_latency_node[w_id] == PLACEHOLDER);
                        map_wire_to_latency_node[w_id] = new_idx;

                        if w.specified_latency != CALCULATE_LATENCY_LATER {
                            initial_values.push(SpecifiedLatency {
                                node: new_idx,
                                latency: w.specified_latency,
                            });
                        }
                    }
                }

                LatencyDomainInfo {
                    latency_node_meanings,
                    initial_values,
                    ports: LatencyCountingPorts::default(),
                }
            });

        for (_id, p) in self.interface_ports.iter_valids() {
            let node = map_wire_to_latency_node[p.wire];
            domain_infos[p.domain].ports.push(node, p.is_input);
        }

        let mut next_port_chain: FlatAlloc<Option<(WireID, i64)>, WireIDMarker> =
            self.wires.map(|_| None);

        for (_sm_id, sm) in &self.submodules {
            // Instances may not be valid (or may not exist yet due to inference)
            let Some(instance) = &sm.instance.get() else {
                continue;
            };

            for domain_id in self.linker.modules[sm.refers_to.id].domains.id_range() {
                struct Prev {
                    first: (WireID, i64),
                    prev: (WireID, i64),
                }

                let mut prev: Option<Prev> = None;
                for (port_id, port) in sm.port_map.iter_valids() {
                    let Some(instance_port) = &instance.interface_ports[port_id] else {
                        continue;
                    };
                    if instance_port.domain != domain_id {
                        continue;
                    }

                    let port_ref = &mut next_port_chain[port.maps_to_wire];
                    assert!(port_ref.is_none());

                    if let Some(prev) = &mut prev {
                        *port_ref =
                            Some((prev.prev.0, instance_port.absolute_latency - prev.prev.1));
                        prev.prev = (port.maps_to_wire, instance_port.absolute_latency);
                    } else {
                        prev = Some(Prev {
                            first: (port.maps_to_wire, instance_port.absolute_latency),
                            prev: (port.maps_to_wire, instance_port.absolute_latency),
                        });
                    }
                }
                if let Some(prev) = prev {
                    // If only one port, then we don't need to make any connection
                    if prev.first.0 != prev.prev.0 {
                        let port_ref = &mut next_port_chain[prev.first.0];
                        assert!(port_ref.is_none());
                        *port_ref = Some((prev.prev.0, prev.first.1 - prev.prev.1));
                    }
                }
            }
        }

        // Every wire has been covered
        debug_assert!(map_wire_to_latency_node
            .iter()
            .all(|(_id, v)| *v != PLACEHOLDER));

        WireToLatencyMap {
            map_wire_to_latency_node,
            domain_infos,
            next_port_chain,
        }
    }

    fn make_fanins(
        &self,
        latency_node_mapper: &WireToLatencyMap,
        latency_node_to_wire_map: &[WireID],
        domain_id: DomainID,
    ) -> ListOfLists<FanInOut> {
        let mut fanins: ListOfLists<FanInOut> =
            ListOfLists::new_with_groups_capacity(latency_node_to_wire_map.len());

        // Wire to wire Fanin
        for wire_id in latency_node_to_wire_map.iter() {
            fanins.new_group();

            self.wires[*wire_id]
                .source
                .iter_sources_with_min_latency(|from, delta_latency| {
                    assert_eq!(self.wires[from].domain, domain_id);
                    fanins.push_to_last_group(FanInOut {
                        to_node: latency_node_mapper.map_wire_to_latency_node[from],
                        delta_latency: Some(delta_latency),
                    });
                });

            if let Some((from, delta_latency)) = latency_node_mapper.next_port_chain[*wire_id] {
                fanins.push_to_last_group(FanInOut {
                    to_node: latency_node_mapper.map_wire_to_latency_node[from],
                    delta_latency: Some(delta_latency),
                })
            }
        }

        fanins
    }

    // Returns a proper interface if all ports involved did not produce an error. If a port did produce an error then returns None.
    // Computes all latencies involved
    pub fn compute_latencies(&mut self) {
        let mut any_invalid_port = false;
        for (port_id, p) in self.interface_ports.iter_valids() {
            if !p.is_input {
                let port_wire = &self.wires[p.wire];
                let RealWireDataSource::Multiplexer {
                    is_state: _,
                    sources,
                } = &port_wire.source
                else {
                    unreachable!()
                };
                if sources.is_empty() && port_wire.specified_latency == CALCULATE_LATENCY_LATER {
                    any_invalid_port = true;
                    let port = &self.md.ports[port_id];
                    self.errors.error(port.name_span, format!("Pre-emptive error because latency-unspecified '{}' is never written to. \n(This is because work-in-progress code would get a lot of latency counting errors while unfinished)", port.name));
                }
            }
        }
        if any_invalid_port {
            return; // Early exit so we don't flood WIP modules with "Node not reached by Latency Counting" errors
        }

        let latency_node_mapper = self.make_wire_to_latency_map();

        for (domain_id, domain_info) in &latency_node_mapper.domain_infos {
            let fanins = self.make_fanins(
                &latency_node_mapper,
                &domain_info.latency_node_meanings,
                domain_id,
            );

            // TODO: Should include the extra edges from partial submodules too.
            match solve_latencies(
                fanins,
                Vec::new(),
                &domain_info.ports,
                &domain_info.initial_values,
            ) {
                Ok(latencies) => {
                    for (node, lat) in
                        zip(domain_info.latency_node_meanings.iter(), latencies.iter())
                    {
                        let wire = &mut self.wires[*node];
                        if is_valid(*lat) {
                            wire.absolute_latency = *lat;
                        } else {
                            let source_location =
                                self.md.get_instruction_span(wire.original_instruction);
                            self.errors.error(
                                source_location,
                                "Latency Counting couldn't reach this node".to_string(),
                            );
                            wire.absolute_latency = CALCULATE_LATENCY_LATER;
                        }
                    }
                }
                Err(err) => {
                    self.report_error(&domain_info.latency_node_meanings, err);
                }
            };
        }

        // Finally update interface absolute latencies
        for (_id, port) in self.interface_ports.iter_valids_mut() {
            port.absolute_latency = self.wires[port.wire].absolute_latency;
        }
    }

    pub fn infer_parameters_for_latencies(&mut self) -> DelayedConstraintStatus {
        let latency_node_mapper = self.make_wire_to_latency_map();

        let mut latency_inference_variables = FlatAlloc::new();

        let mut domain_inference_infos = latency_node_mapper
            .domain_infos
            .map(|_| PartialSubmoduleInfo::default());

        for (sm_id, sm) in &self.submodules {
            if sm.instance.get().is_some() {
                continue; // Submodule already instantiated
            }
            let sm_md = &self.linker.modules[sm.refers_to.id];

            let sm_instruction =
                self.md.link_info.instructions[sm.original_instruction].unwrap_submodule();

            let known_template_args = sm.refers_to.template_args.map(|(_, t)| {
                let mut t_copy = t.clone();
                t_copy.fully_substitute(&self.type_substitutor);
                if let ConcreteType::Value(Value::Integer(num)) = &t_copy {
                    i64::try_from(num).ok()
                } else {
                    None
                }
            });

            let inference_edges = sm_md.latency_inference_info.get_inference_edges(
                &known_template_args,
                sm_md.named_domains,
                sm_id,
                &mut latency_inference_variables,
            );

            for (_local_domain, local_domain_info, edges) in zip_eq(
                sm_instruction.local_interface_domains.iter(),
                inference_edges.iter(),
            ) {
                let global_domain = local_domain_info.unwrap_physical();
                edges.apply_to_global_domain(
                    &sm.port_map,
                    &latency_node_mapper.map_wire_to_latency_node,
                    &mut domain_inference_infos[global_domain],
                );
            }
        }

        for (domain_id, domain_info, domain_inference_info) in zip_eq(
            latency_node_mapper.domain_infos.iter(),
            domain_inference_infos.into_iter(),
        ) {
            let fanins = self.make_fanins(
                &latency_node_mapper,
                &domain_info.latency_node_meanings,
                domain_id,
            );

            // We don't need to report the error, they'll bubble up later anyway during [solve_latencies]
            let _result = infer_unknown_latency_edges(
                fanins,
                &domain_info.ports,
                &domain_info.initial_values,
                domain_inference_info,
                &mut latency_inference_variables,
            );
        }

        let mut any_new_values = false;
        let mut all_new_values = true;
        for (_, var) in latency_inference_variables.into_iter() {
            if let Some(inferred_value) = var.get() {
                let (submod_id, arg_id) = var.back_reference;

                self.type_substitutor.unify_must_succeed(
                    &self.submodules[submod_id].refers_to.template_args[arg_id],
                    &ConcreteType::Value(Value::Integer(inferred_value.into())),
                );

                any_new_values = true;
            } else {
                all_new_values = false;
            }
        }

        match (any_new_values, all_new_values) {
            (_, true) => DelayedConstraintStatus::Resolved,
            (true, false) => DelayedConstraintStatus::Progress,
            (false, false) => DelayedConstraintStatus::NoProgress,
        }
    }

    fn gather_all_mux_inputs(
        &self,
        latency_node_meanings: &[WireID],
        conflict_iter: &[SpecifiedLatency],
    ) -> Vec<PathMuxSource<'_>> {
        let mut connection_list = Vec::new();
        for window in conflict_iter.windows(2) {
            let [from, to] = window else { unreachable!() };
            let from_wire_id = latency_node_meanings[from.node];
            //let from_wire = &self.wires[from_wire_id];
            let to_wire_id = latency_node_meanings[to.node];
            let to_wire = &self.wires[to_wire_id];
            let RealWireDataSource::Multiplexer {
                is_state: _,
                sources,
            } = &to_wire.source
            else {
                continue;
            }; // We can only name multiplexers

            for s in sources {
                let mut predecessor_found = false;
                let mut predecessor_adder = |source| {
                    if source == from_wire_id {
                        predecessor_found = true;
                    }
                };
                predecessor_adder(s.from);
                RealWirePathElem::for_each_wire_in_path(&s.to_path, predecessor_adder);
                if predecessor_found {
                    connection_list.push(PathMuxSource {
                        to_wire,
                        mux_input: s,
                        to_latency: to.latency,
                    });
                }
            }
        }
        connection_list
    }

    fn report_error(&self, latency_node_meanings: &[WireID], err: LatencyCountingError) {
        match err {
            LatencyCountingError::NetPositiveLatencyCycle {
                conflict_path,
                net_roundtrip_latency,
            } => {
                let writes_involved =
                    self.gather_all_mux_inputs(latency_node_meanings, &conflict_path);
                assert!(!writes_involved.is_empty());
                let (first_write, later_writes) = writes_involved.split_first().unwrap();
                let first_write_desired_latency = first_write.to_latency + net_roundtrip_latency;
                let mut path_message = make_path_info_string(
                    later_writes,
                    first_write.to_latency,
                    &first_write.to_wire.name,
                );
                write_path_elem_to_string(
                    &mut path_message,
                    &first_write.to_wire.name,
                    first_write_desired_latency,
                    writes_involved.last().unwrap().to_latency,
                );
                let unique_write_instructions =
                    filter_unique_write_flats(&writes_involved, &self.md.link_info.instructions);
                let rest_of_message = format!(" part of a net-positive latency cycle of +{net_roundtrip_latency}\n\n{path_message}\nWhich conflicts with the starting latency");

                let mut did_place_error = false;
                for wr in &unique_write_instructions {
                    match wr.write_modifiers {
                        WriteModifiers::Connection {
                            num_regs,
                            regs_span,
                        } => {
                            if num_regs >= 1 {
                                did_place_error = true;
                                let this_register_plural = if num_regs == 1 {
                                    "This register is"
                                } else {
                                    "These registers are"
                                };
                                self.errors.error(
                                    regs_span,
                                    format!("{this_register_plural}{rest_of_message}"),
                                );
                            }
                        }
                        WriteModifiers::Initial { initial_kw_span: _ } => {
                            unreachable!("Initial assignment can only be from compile-time constant. Cannot be part of latency loop. ")
                        }
                    }
                }
                // Fallback if no register annotations used
                if !did_place_error {
                    for wr in unique_write_instructions {
                        self.errors
                            .error(wr.to_span, format!("This write is{rest_of_message}"));
                    }
                }
            }
            LatencyCountingError::IndeterminablePortLatency { bad_ports } => {
                for (port, a, b) in bad_ports {
                    let port_decl = self.md.link_info.instructions
                        [self.wires[latency_node_meanings[port]].original_instruction]
                        .unwrap_declaration();
                    self.errors.error(port_decl.name_span, format!("Cannot determine port latency. Options are {a} and {b}\nTry specifying an explicit latency or rework the module to remove this ambiguity"));
                }
            }
            LatencyCountingError::PartialSolutionMergeConflict { bad_nodes } => {
                for (node, a, b) in bad_nodes {
                    let node_instr_span = self.md.get_instruction_span(
                        self.wires[latency_node_meanings[node]].original_instruction,
                    );
                    self.errors
                        .error(node_instr_span, format!("There were conflicting options when merging partial latency counting solutions for this node. Options were {a} and {b}\nTry specifying an explicit latency or rework the module to remove this ambiguity"));
                }
            }
            LatencyCountingError::ConflictingSpecifiedLatencies { conflict_path } => {
                let start_wire =
                    &self.wires[latency_node_meanings[conflict_path.first().unwrap().node]];
                let end_wire =
                    &self.wires[latency_node_meanings[conflict_path.last().unwrap().node]];
                let start_decl = self.md.link_info.instructions[start_wire.original_instruction]
                    .unwrap_declaration();
                let end_decl = self.md.link_info.instructions[end_wire.original_instruction]
                    .unwrap_declaration();
                let end_latency_decl = self.md.link_info.instructions
                    [end_decl.latency_specifier.unwrap()]
                .unwrap_expression();

                let writes_involved =
                    self.gather_all_mux_inputs(latency_node_meanings, &conflict_path);
                let path_message = make_path_info_string(
                    &writes_involved,
                    start_wire.specified_latency,
                    &start_wire.name,
                );
                //assert!(!writes_involved.is_empty());

                let end_name = &end_wire.name;
                let specified_end_latency = end_wire.specified_latency;
                self.errors
                    .error(end_latency_decl.span, format!("Conflicting specified latency\n\n{path_message}\nBut this was specified as {end_name}'{specified_end_latency}"))
                    .info_obj_same_file(start_decl);
            }
        }
    }
}
