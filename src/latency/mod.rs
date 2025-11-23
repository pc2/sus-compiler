mod latency_algorithm;
mod list_of_lists;

pub mod port_latency_inference;

use std::fmt::{Debug, Display, Write};
use std::{cmp::max, iter::zip};

use crate::alloc::zip_eq;
use crate::dev_aid::dot_graphs::display_latency_count_graph;
use crate::errors::ErrorInfoObject;
use crate::prelude::*;
use crate::to_string::display_join;

use latency_algorithm::{
    FanInOut, LatencyCountingError, LatencyCountingPorts, SpecifiedLatency,
    add_cycle_to_extra_fanin, is_valid, solve_latencies,
};

use self::list_of_lists::ListOfLists;

use crate::instantiation::*;

// Temporary value before proper latency is given
const CALCULATE_LATENCY_LATER: i64 = i64::MIN;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct AbsLat(i64);

impl AbsLat {
    pub const UNKNOWN: Self = Self(CALCULATE_LATENCY_LATER);
    pub fn new(v: i64) -> Self {
        assert!(
            v < i64::MAX - 100000000 && v > i64::MIN + 100000000,
            "Trying to create an AbsLat with a v too close to CALCULATE_LATENCY_LATER ({CALCULATE_LATENCY_LATER}): {v}"
        );
        Self(v)
    }
    pub fn get(self) -> Option<i64> {
        if self.0 == CALCULATE_LATENCY_LATER {
            None
        } else {
            Some(self.0)
        }
    }
    #[track_caller]
    pub fn unwrap(self) -> i64 {
        assert!(self.0 != CALCULATE_LATENCY_LATER);
        self.0
    }
}
impl Display for AbsLat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.get() {
            Some(lat) => write!(f, "{lat}"),
            None => f.write_char('?'),
        }
    }
}
impl Debug for AbsLat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.get() {
            Some(lat) => write!(f, "AbsLat({lat})"),
            None => f.write_str("AbsLat(?)"),
        }
    }
}
impl Default for AbsLat {
    fn default() -> Self {
        Self::UNKNOWN
    }
}

struct PathMuxSource<'s> {
    to_wire: &'s RealWire,
    to_latency: i64,
    #[allow(unused)]
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

/// We do all Domains together, as this simplifies the code.
pub struct LatencyCountingProblem {
    pub map_wire_to_latency_node: FlatAlloc<usize, WireIDMarker>,
    pub map_latency_node_to_wire: Vec<WireID>,

    pub ports: LatencyCountingPorts,
    pub specified_latencies: Vec<SpecifiedLatency>,

    /// "to" comes first
    pub edges: Vec<(usize, FanInOut)>,
}

impl LatencyCountingProblem {
    fn new(ctx: &ModuleTypingContext) -> Self {
        let mut map_latency_node_to_wire = Vec::new();
        let mut specified_latencies = Vec::new();

        let map_wire_to_latency_node = ctx.wires.map(|(w_id, w)| {
            // Create mappings
            let node = map_latency_node_to_wire.len();
            map_latency_node_to_wire.push(w_id);

            // Add specifieds
            if let Some(latency) = w.specified_latency.get() {
                specified_latencies.push(SpecifiedLatency { node, latency });
            }

            node
        });

        // Ports
        let mut ports = LatencyCountingPorts::default();
        for (wire_id, w) in &ctx.wires {
            if let IsPort::Port(_, direction) = w.is_port {
                let node = map_wire_to_latency_node[wire_id];
                ports.push(node, direction);
            }
        }

        // Basic wire-based edges
        let mut edges: Vec<(usize, FanInOut)> = Vec::new();
        for (_id, w, wire_lat_node) in zip_eq(ctx.wires.iter(), map_wire_to_latency_node.iter()) {
            // Wire to wire Fanin
            w.source
                .iter_sources_with_min_latency(|from, delta_latency| {
                    edges.push((
                        *wire_lat_node,
                        FanInOut {
                            to_node: map_wire_to_latency_node[from],
                            delta_latency: Some(delta_latency),
                        },
                    ));
                });
        }

        // For reuse of memory
        let mut cur_cycle = Vec::new();
        // Submodules
        for (_, sm) in &ctx.submodules {
            let sm_md = &ctx.globals.modules[sm.refers_to.id];

            if let Some(instance) = sm.instance.get() {
                // The module has already been instantiated, so we know all local absolute latencies
                // No inference edges, No poison edges

                for d in sm_md.domains.id_range() {
                    for (_, port, wire) in
                        crate::alloc::zip_eq(&instance.interface_ports, &sm.port_map)
                    {
                        if let (Some(port), Some(wire)) = (port, wire)
                            && port.domain == d
                        {
                            let latency = port.absolute_latency.unwrap();
                            let node = map_wire_to_latency_node[wire.maps_to_wire];
                            cur_cycle.push(SpecifiedLatency { latency, node });
                        }
                    }
                    add_cycle_to_extra_fanin(&cur_cycle, &mut edges);
                    cur_cycle.clear();
                }
            } else {
                for pg in &sm_md.inference_info.port_groups {
                    for &(port_id, latency) in pg {
                        if let Some(port) = &sm.port_map[port_id] {
                            let node = map_wire_to_latency_node[port.maps_to_wire];
                            cur_cycle.push(SpecifiedLatency { node, latency });
                        }
                    }
                    add_cycle_to_extra_fanin(&cur_cycle, &mut edges);
                    cur_cycle.clear();
                }
            }
        }

        // Finish up by adding a Specified Latencies cycle, because solve_latencies and inferernce expects that
        add_cycle_to_extra_fanin(&specified_latencies, &mut edges);

        Self {
            map_wire_to_latency_node,
            map_latency_node_to_wire,
            ports,
            specified_latencies,
            edges,
        }
    }

    fn make_ports_per_domain(&self, ctx: &ModuleTypingContext) -> Vec<Vec<usize>> {
        let mut ports_per_domain_flat = ctx.md.domains.map(|_| Vec::new());
        for (_id, port) in &ctx.md.ports {
            if let SubModuleOrWire::Wire(port_w) =
                ctx.generation_state[port.declaration_instruction]
            {
                ports_per_domain_flat[port.domain].push(self.map_wire_to_latency_node[port_w]);
            }
        }
        let mut ports_per_domain = ports_per_domain_flat.into_vec();
        ports_per_domain.retain(|v| v.len() > 1);
        ports_per_domain
    }

    fn make_fanins(&self) -> ListOfLists<FanInOut> {
        ListOfLists::from_random_access_iterator(
            self.map_latency_node_to_wire.len(),
            self.edges.iter().copied(),
        )
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InferenceFailure {
    NotReached,
    /// Points to the first poison edge that poisoned the target node.
    /// Because poison edges only exist in unresolved submodules,
    /// this is guaranteed to go from a submodule input to a submodule output
    Poison {
        edge_from: usize,
        edge_to: usize,
    },
    BadProblem,
}

pub struct LatencyInferenceProblem {
    pub latency_count_problem: LatencyCountingProblem,
    pub algo_inference_problem: Option<latency_algorithm::LatencyInferenceProblem>,
}
impl LatencyInferenceProblem {
    pub fn new(ctx: &ModuleTypingContext) -> Self {
        let mut lc = LatencyCountingProblem::new(ctx);

        // Add poison edges
        for (_, sm) in &ctx.submodules {
            let sm_md = &ctx.globals.modules[sm.refers_to.id];
            if sm.instance.get().is_none() {
                for &(from, to) in &sm_md.inference_info.extra_poison {
                    if let (Some(from), Some(to)) = (&sm.port_map[from], &sm.port_map[to]) {
                        let to = lc.map_wire_to_latency_node[to.maps_to_wire];
                        let from = lc.map_wire_to_latency_node[from.maps_to_wire];
                        lc.edges.push((to, FanInOut::mk_poison(from)));
                    }
                }
            }
        }

        let algo_inference_problem = latency_algorithm::LatencyInferenceProblem::new(
            lc.make_fanins(),
            &lc.ports,
            &lc.specified_latencies,
        );
        LatencyInferenceProblem {
            algo_inference_problem,
            latency_count_problem: lc,
        }
    }
    pub fn infer(&mut self, from: WireID, to: WireID) -> Result<i64, InferenceFailure> {
        let from = self.latency_count_problem.map_wire_to_latency_node[from];
        let to = self.latency_count_problem.map_wire_to_latency_node[to];
        if let Some(inf_prob) = &mut self.algo_inference_problem {
            inf_prob.infer_max_edge_latency(from, to)
        } else {
            Err(InferenceFailure::BadProblem)
        }
    }
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
                    s.for_each_wire(&mut |from| f(from, s.num_regs));
                }
            }
            RealWireDataSource::UnaryOp { right, .. } => {
                f(*right, 0);
            }
            RealWireDataSource::BinaryOp { left, right, .. } => {
                f(*left, 0);
                f(*right, 0);
            }
            RealWireDataSource::Select { root, path } => {
                f(*root, 0);
                path.for_each_wire(&mut |w| f(w, 0));
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
        let mut result = self.wires.map(|(_id, w)| w.absolute_latency.unwrap());

        for (_id, w) in &self.wires {
            w.source.iter_sources_with_min_latency(|other, _| {
                let other = match &self.wires[other].source {
                    // For inlining path-less Selects
                    RealWireDataSource::Select { root, path } if path.is_empty() => *root,
                    _ => other,
                };
                let nu = &mut result[other];

                *nu = max(*nu, w.absolute_latency.unwrap());
            });
        }

        result
    }
}

impl ModuleTypingContext<'_> {
    /// Computes and sets all latencies involved ([RealWire::absolute_latency])
    pub fn compute_latencies(&mut self) {
        let problem = LatencyCountingProblem::new(self);

        if crate::debug::is_enabled("dot-latency-problem") {
            display_latency_count_graph(
                &problem,
                &self.wires,
                &self.submodules,
                self.globals,
                None,
                &self.name,
                "latencies_problem",
            );
        }

        let fanins = problem.make_fanins();

        let ports_per_domain = problem.make_ports_per_domain(self);

        match solve_latencies(
            fanins,
            &problem.ports,
            &problem.specified_latencies,
            &ports_per_domain,
        ) {
            Ok(latencies) => {
                if crate::debug::is_enabled("dot-latency-solution") {
                    display_latency_count_graph(
                        &problem,
                        &self.wires,
                        &self.submodules,
                        self.globals,
                        Some(&latencies),
                        &self.name,
                        "latencies_solution",
                    );
                }
                for (node, lat) in zip(problem.map_latency_node_to_wire.iter(), latencies.iter()) {
                    let wire = &mut self.wires[*node];
                    if is_valid(*lat) {
                        wire.absolute_latency = AbsLat::new(*lat);
                    } else {
                        let source_location = self
                            .md
                            .link_info
                            .get_instruction_span(wire.original_instruction);
                        self.errors.error(
                            source_location,
                            "Latency Counting couldn't reach this node".to_string(),
                        );
                        wire.absolute_latency = AbsLat::UNKNOWN;
                    }
                }
            }
            Err(err) => {
                self.report_error(&problem.map_latency_node_to_wire, err);
            }
        };
    }

    fn gather_all_mux_inputs(
        &self,
        latency_node_meanings: &[WireID],
        conflicts: &[SpecifiedLatency],
    ) -> Vec<PathMuxSource<'_>> {
        let mut connection_list = Vec::new();
        for (idx, from) in conflicts.iter().enumerate() {
            let to = &conflicts[(idx + 1) % conflicts.len()];
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
                s.for_each_wire(&mut |source| {
                    if source == from_wire_id {
                        predecessor_found = true;
                    }
                });
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
        let mut error_placed_successfully = false;
        let mut error = |span, msg: String| {
            error_placed_successfully = true;
            self.errors.error(span, msg)
        };

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
                let rest_of_message = format!(
                    " part of a net-positive latency cycle of +{net_roundtrip_latency}\n\n{path_message}\nWhich conflicts with the starting latency"
                );

                /*let unique_write_instructions =
                    filter_unique_write_flats(&writes_involved, &self.md.link_info.instructions);
                let mut did_place_error = false;
                for wr in &unique_write_instructions {
                    match wr.to.write_modifiers {
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
                                error(
                                    regs_span,
                                    format!("{this_register_plural}{rest_of_message}"),
                                );
                            }
                        }
                        WriteModifiers::Initial { initial_kw_span: _ } => {
                            unreachable!("Initial assignment can only be from compile-time constant. Cannot be part of latency loop. ")
                        }
                    }
                }*/
                // Fallback if no register annotations used
                //if !did_place_error {
                for wr in writes_involved {
                    let to_instr = &self.md.link_info.instructions[wr.to_wire.original_instruction];
                    error(
                        to_instr.get_span(),
                        format!("This instruction is{rest_of_message}"),
                    );
                }
                //}
            }
            LatencyCountingError::IndeterminablePortLatency { bad_ports } => {
                for (port, a, b) in bad_ports {
                    let port_instr = self.wires[latency_node_meanings[port]].original_instruction;
                    let port_name_span = self.md.link_info.instructions[port_instr].get_span();
                    error(
                        port_name_span,
                        format!(
                            "Cannot determine port latency. Options are {a} and {b}\nTry specifying an explicit latency or rework the module to remove this ambiguity"
                        ),
                    );
                }
            }
            LatencyCountingError::PortsNotStronglyConnected { port_partitions } => {
                for (num_hit, all_nodes) in port_partitions {
                    let all_port_instrs: Vec<_> = all_nodes
                        .iter()
                        .map(|node| self.wires[latency_node_meanings[*node]].original_instruction)
                        .collect();

                    let (connected_ports, ports_not_in_group) = all_port_instrs.split_at(num_hit);

                    let strongly_connected_port_list =
                        display_join(", ", connected_ports, |f, instr| {
                            let name = self.md.link_info.instructions[*instr].get_name();
                            write!(f, "'{name}'")
                        });
                    let hit_names_error_infos: Vec<_> = connected_ports
                        .iter()
                        .map(|instr| {
                            self.md.link_info.instructions[*instr]
                                .make_info(self.md.link_info.file)
                                .unwrap()
                        })
                        .collect();

                    for non_hit in ports_not_in_group {
                        let node_instr_span = self.md.link_info.instructions[*non_hit].get_span();

                        error(node_instr_span, format!("This port is not strongly connected to the strongly connected port cluster {strongly_connected_port_list}.\nAn input and output port are strongly connected if there is a direct dependency path from the input port to the output port.\nStrongly connected ports are also transitive.\nIf you do not wish to change your design, then 'virtually' connect this port to the strongly connected cluster by explicitly annotating its absolute latency."))
                            .add_info_list(hit_names_error_infos.clone());
                    }
                }
            }
            LatencyCountingError::ConflictingSpecifiedLatencies { conflict_path } => {
                let start_wire =
                    &self.wires[latency_node_meanings[conflict_path.first().unwrap().node]];
                let end_wire =
                    &self.wires[latency_node_meanings[conflict_path.last().unwrap().node]];
                let start_decl = &self.md.link_info.instructions[start_wire.original_instruction];
                let end_decl = &self.md.link_info.instructions[end_wire.original_instruction];
                let end_latency_decl = self.md.link_info.instructions
                    [end_decl.get_latency_specifier().unwrap()]
                .unwrap_expression();

                let writes_involved =
                    self.gather_all_mux_inputs(latency_node_meanings, &conflict_path);
                let path_message = make_path_info_string(
                    &writes_involved,
                    start_wire.specified_latency.unwrap(),
                    &start_wire.name,
                );
                //assert!(!writes_involved.is_empty());

                let end_name = &end_wire.name;
                let specified_end_latency = end_wire.specified_latency.unwrap();
                error(end_latency_decl.span, format!("Conflicting specified latency\n\n{path_message}\nBut this was specified as {end_name}'{specified_end_latency}"))
                    .info_obj_same_file(start_decl);
            }
        }
        assert!(error_placed_successfully);
    }
}
