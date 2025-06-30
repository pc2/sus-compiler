mod latency_algorithm;
mod list_of_lists;

pub mod port_latency_inference;

use std::{cmp::max, iter::zip};

use crate::dev_aid::dot_graphs::display_latency_count_graph;
use crate::errors::ErrorInfoObject;
use crate::prelude::*;
use crate::typing::value_unifier::ValueUnifierStore;
use crate::{alloc::zip_eq, typing::value_unifier::ValueUnifier};

use crate::value::Value;

use ibig::IBig;
use latency_algorithm::{
    add_cycle_to_extra_fanin, infer_unknown_latency_edges, is_valid, solve_latencies, FanInOut,
    LatencyCountingError, LatencyCountingPorts, LatencyInferenceCandidate, SpecifiedLatency,
    ValueToInfer,
};

use self::list_of_lists::ListOfLists;

use crate::instantiation::*;

// Temporary value before proper latency is given
pub const CALCULATE_LATENCY_LATER: i64 = i64::MIN;

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
#[derive(Default)]
pub struct LatencyCountingProblem {
    pub map_wire_to_latency_node: FlatAlloc<usize, WireIDMarker>,
    pub map_latency_node_to_wire: Vec<WireID>,

    pub ports: LatencyCountingPorts,
    pub specified_latencies: Vec<SpecifiedLatency>,
    pub inference_variables:
        FlatAlloc<ValueToInfer<(SubModuleID, TemplateID)>, InferenceVarIDMarker>,
    pub inference_edges: Vec<LatencyInferenceCandidate>,
    // "to" comes first
    pub edges: Vec<(usize, FanInOut)>,
}

impl LatencyCountingProblem {
    fn new(ctx: &ModuleTypingContext, unifier: &ValueUnifierStore) -> Self {
        let mut map_latency_node_to_wire = Vec::new();
        let mut specified_latencies = Vec::new();

        let map_wire_to_latency_node = ctx.wires.map(|(w_id, w)| {
            // Create mappings
            let node = map_latency_node_to_wire.len();
            map_latency_node_to_wire.push(w_id);

            // Add specifieds
            if w.specified_latency != CALCULATE_LATENCY_LATER {
                specified_latencies.push(SpecifiedLatency {
                    node,
                    latency: w.specified_latency,
                });
            }

            node
        });

        // Ports
        let mut ports = LatencyCountingPorts::default();
        for (wire_id, w) in &ctx.wires {
            if let Some(direction) = w.is_port {
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

        // Inference
        let mut inference_variables = FlatAlloc::new();
        let mut inference_edges = Vec::new();

        // Submodules
        for (sm_id, sm) in &ctx.submodules {
            let local_inference_edges = sm.get_interface_relative_latencies(
                ctx.linker,
                sm_id,
                unifier,
                &mut inference_variables,
            );

            local_inference_edges.apply_to_global_domain(
                &sm.port_map,
                &map_wire_to_latency_node,
                &mut edges,
                &mut inference_edges,
            );
        }

        // Finish up by adding a Specified Latencies cycle, because solve_latencies and inferernce expects that
        add_cycle_to_extra_fanin(&specified_latencies, &mut edges);

        Self {
            map_wire_to_latency_node,
            map_latency_node_to_wire,
            ports,
            specified_latencies,
            inference_variables,
            inference_edges,
            edges,
        }
    }

    fn make_ports_per_domain(&self, ctx: &ModuleTypingContext) -> Vec<Vec<usize>> {
        let mut ports_per_domain_flat = ctx.md.domains.map(|_| Vec::new());
        for (_id, port) in ctx.interface_ports.iter_valids() {
            ports_per_domain_flat[port.domain].push(self.map_wire_to_latency_node[port.wire]);
        }
        let mut ports_per_domain = ports_per_domain_flat.into_vec();
        ports_per_domain.retain(|v| v.len() > 1);
        ports_per_domain
    }

    fn remove_poison_edges(&mut self) {
        self.edges.retain(|v| v.1.delta_latency.is_some());
    }

    fn make_fanins(&self) -> ListOfLists<FanInOut> {
        ListOfLists::from_random_access_iterator(
            self.map_latency_node_to_wire.len(),
            self.edges.iter().copied(),
        )
    }

    fn debug(
        &self,
        ctx: &ModuleTypingContext,
        solution: Option<&[i64]>,
        debug_flag: &'static str,
        file_name: &str,
    ) {
        if crate::debug::is_enabled(debug_flag) {
            display_latency_count_graph(
                self,
                &ctx.wires,
                &ctx.submodules,
                ctx.linker,
                solution,
                file_name,
            );
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
        let mut result = self.wires.map(|(_id, w)| w.absolute_latency);

        for (_id, w) in &self.wires {
            w.source.iter_sources_with_min_latency(|other, _| {
                let other = match &self.wires[other].source {
                    // For inlining path-less Selects
                    RealWireDataSource::Select { root, path } if path.is_empty() => *root,
                    _ => other,
                };
                let nu = &mut result[other];

                *nu = max(*nu, w.absolute_latency);
            });
        }

        result
    }
}

impl ModuleTypingContext<'_> {
    // Returns a proper interface if all ports involved did not produce an error. If a port did produce an error then returns None.
    // Computes all latencies involved
    pub fn compute_latencies(&mut self, unifier: &ValueUnifierStore) {
        let mut problem = LatencyCountingProblem::new(self, unifier);
        // Remove all poisoned edges as solve_latencies doesn't deal with them
        problem.remove_poison_edges();

        problem.debug(
            self,
            None,
            "dot-latency-problem",
            "solve_latencies_problem.dot",
        );

        let fanins = problem.make_fanins();

        match solve_latencies(
            fanins,
            &problem.ports,
            &problem.specified_latencies,
            problem.make_ports_per_domain(self),
        ) {
            Ok(latencies) => {
                problem.debug(
                    self,
                    Some(&latencies),
                    "dot-latency-solution",
                    "solve_latencies_solution.dot",
                );
                for (node, lat) in zip(problem.map_latency_node_to_wire.iter(), latencies.iter()) {
                    let wire = &mut self.wires[*node];
                    if is_valid(*lat) {
                        wire.absolute_latency = *lat;
                    } else {
                        let source_location = self
                            .md
                            .link_info
                            .get_instruction_span(wire.original_instruction);
                        self.errors.error(
                            source_location,
                            "Latency Counting couldn't reach this node".to_string(),
                        );
                        wire.absolute_latency = CALCULATE_LATENCY_LATER;
                    }
                }
            }
            Err(err) => {
                self.report_error(&problem.map_latency_node_to_wire, err);
            }
        };

        // Finally update interface absolute latencies
        for (_id, port) in self.interface_ports.iter_valids_mut() {
            port.absolute_latency = self.wires[port.wire].absolute_latency;
        }
    }

    pub fn infer_parameters_for_latencies<'inst>(&'inst self, unifier: &mut ValueUnifier<'inst>) {
        let mut problem = LatencyCountingProblem::new(self, &unifier.store);
        let fanins = problem.make_fanins();

        problem.debug(
            self,
            None,
            "dot-latency-infer",
            "latency_inference_problem.dot",
        );

        // We don't need to report the error, they'll bubble up later anyway during [solve_latencies]
        let _result = infer_unknown_latency_edges(
            fanins,
            &problem.ports,
            &problem.specified_latencies,
            &problem.inference_edges,
            &mut problem.inference_variables,
        );

        for (_, var) in problem.inference_variables.into_iter() {
            if let Some(inferred_value) = var.get() {
                let (submod_id, arg_id) = var.back_reference;

                assert!(unifier
                    .set(
                        self.submodules[submod_id].refers_to.template_args[arg_id].unwrap_value(),
                        Value::Integer(IBig::from(inferred_value)),
                    )
                    .is_ok());
            }
        }
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
                let rest_of_message = format!(" part of a net-positive latency cycle of +{net_roundtrip_latency}\n\n{path_message}\nWhich conflicts with the starting latency");

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
                    error(port_name_span, format!("Cannot determine port latency. Options are {a} and {b}\nTry specifying an explicit latency or rework the module to remove this ambiguity"));
                }
            }
            LatencyCountingError::UnreachablePortInThisDomain { hit_and_not_hit } => {
                for (num_hit, all_nodes) in hit_and_not_hit {
                    let all_port_instrs: Vec<_> = all_nodes
                        .iter()
                        .map(|node| self.wires[latency_node_meanings[*node]].original_instruction)
                        .collect();

                    let hit_instrs = &all_port_instrs[..num_hit];
                    let non_hit_instrs = &all_port_instrs[num_hit..];

                    let hit_names: Vec<_> = hit_instrs
                        .iter()
                        .map(|instr| {
                            let name = self.md.link_info.instructions[*instr].get_name();
                            format!("'{name}'")
                        })
                        .collect();
                    let hit_names_error_infos: Vec<_> = hit_instrs
                        .iter()
                        .map(|instr| {
                            self.md.link_info.instructions[*instr]
                                .make_info(self.md.link_info.file)
                                .unwrap()
                        })
                        .collect();
                    let strongly_connected_port_list = hit_names.join(", ");

                    for non_hit in non_hit_instrs {
                        let node_instr_span = self.md.link_info.instructions[*non_hit].get_span();

                        error(node_instr_span, format!("This port is not strongly connected to the strongly connected port cluster {strongly_connected_port_list}.\nAn input and output port are strongly connected if there is a direct dependency path from the input port to the output port.\nStrongly connected ports are also transitive.\nIf you do not wish to change your design, then 'virtually' connect this port to the strongly connected cluster by explicitly annotating its absolute latency.")).add_info_list(hit_names_error_infos.clone());
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
                    start_wire.specified_latency,
                    &start_wire.name,
                );
                //assert!(!writes_involved.is_empty());

                let end_name = &end_wire.name;
                let specified_end_latency = end_wire.specified_latency;
                error(end_latency_decl.span, format!("Conflicting specified latency\n\n{path_message}\nBut this was specified as {end_name}'{specified_end_latency}"))
                    .info_obj_same_file(start_decl);
            }
        }
        assert!(error_placed_successfully);
    }
}
