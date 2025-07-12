//! Latency Counting concerns three types of nodes:
//! - Late nodes: These usually correspond to inputs. LC tries to make these as late as possible
//! - Early nodes: Usually correspond to outputs. LC tries to make these as early as possible
//!     (therby squeezing the inputs and outputs together as closely as possible)
//! - Neutral nodes: These just need to get some absolute latency assigned.
//!     LC will make these as early as possible, without affecting late nodes.
//!     Neutral nodes not constrained by Late nodes get added in last, by a single backwards pass
//!
//! Latency counting works in two stages:
//! - First we start from the ports (the early and late nodes).
//!     From here we try to discover other ports, by walking the dependency graph
//!     Any ports we discover must be unambiguously reachable at the exact same absolute latency from other ports
//! - Once we have found all ports, and no port reports a conflicting latency, we can fill in the internal latencies
//!     This starts from the late ports, and seeds them with the found latencies.
//!     From here it keeps all the found latencies, such that the resulting latencies are all as early as possible.

use std::collections::VecDeque;

use crate::{
    alloc::FlatAlloc,
    flattening::Direction,
    prelude::{InferenceVarIDMarker, LatencyCountInferenceVarID},
};

use super::list_of_lists::ListOfLists;

const UNSET: i64 = i64::MIN;
const POISON: i64 = i64::MAX;
/// Arbitrary recognisable offset. Partial solutions are offset by this value, such that the user can distinguish them better
const SEPARATE_SEED_OFFSET: i64 = 1000;

/// A wire for which a latency has been specified.
///
/// Provided as a list to [solve_latencies].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SpecifiedLatency {
    pub node: usize,
    pub latency: i64,
}

impl SpecifiedLatency {
    fn get_from_specify_list(list: &[SpecifiedLatency], node: usize) -> Option<i64> {
        list.iter()
            .find_map(|spec_lat| (spec_lat.node == node).then_some(spec_lat.latency))
    }
}

/// All the ways [solve_latencies] can go wrong
#[derive(Debug)]
pub enum LatencyCountingError {
    ConflictingSpecifiedLatencies {
        conflict_path: Vec<SpecifiedLatency>,
    },
    NetPositiveLatencyCycle {
        conflict_path: Vec<SpecifiedLatency>,
        net_roundtrip_latency: i64,
    },
    IndeterminablePortLatency {
        bad_ports: Vec<(usize, i64, i64)>,
    },
    UnreachablePortInThisDomain {
        /// Result is partitioned. The first tuple elem represents the number of hit ports, which come first in the list
        hit_and_not_hit: Vec<(usize, Vec<usize>)>,
    },
}

/// A graph connection from (respectively to) another wire, which specifies the minimal (respectively maximal) difference in latency between them.
#[derive(Debug, Clone, Copy)]
pub struct FanInOut {
    pub to_node: usize,
    /// If None, then this is a poisoned edge
    pub delta_latency: Option<i64>,
}

impl ListOfLists<FanInOut> {
    pub fn faninout_complement(&self) -> ListOfLists<FanInOut> {
        ListOfLists::from_random_access_iterator(
            self.len(),
            self.iter_flattened_by_bucket().map(
                |(
                    bucket,
                    &FanInOut {
                        to_node,
                        delta_latency,
                    },
                )| {
                    (
                        to_node,
                        FanInOut {
                            to_node: bucket,
                            delta_latency,
                        },
                    )
                },
            ),
        )
    }
    pub fn add_extra_fanin_and_specified_latencies(
        self,
        mut extra_fanin: Vec<(usize, FanInOut)>,
        specified_latencies: &[SpecifiedLatency],
    ) -> Self {
        add_cycle_to_extra_fanin(specified_latencies, &mut extra_fanin);
        self.extend_lists_with_new_elements(extra_fanin)
    }
}

/// ONLY RUN ON FANINS
pub fn add_cycle_to_extra_fanin(
    cycle: &[SpecifiedLatency],
    edges_to_add: &mut Vec<(usize, FanInOut)>,
) {
    // We only add constraints to cycles >= 2, because a 1-length cycle adds no constraints
    if cycle.len() < 2 {
        return;
    }
    let mut previous_node = *cycle.last().unwrap();

    for node in cycle {
        edges_to_add.push((
            node.node,
            FanInOut {
                to_node: previous_node.node,
                delta_latency: Some(node.latency - previous_node.latency),
            },
        ));
        previous_node = *node;
    }
}

pub fn is_valid(latency: i64) -> bool {
    latency != UNSET && latency != POISON
}

struct SolutionMemory {
    solution: Vec<i64>,
    to_explore_queue: VecDeque<usize>,
}
impl SolutionMemory {
    fn new(size: usize) -> Self {
        Self {
            solution: vec![UNSET; size],
            to_explore_queue: VecDeque::with_capacity(size),
        }
    }
    fn make_solution_with_initial_values<'s>(
        &'s mut self,
        specified_latencies: &[SpecifiedLatency],
    ) -> Solution<'s> {
        self.to_explore_queue.clear();
        self.solution.fill(UNSET);
        for spec in specified_latencies {
            let target_node = &mut self.solution[spec.node];
            assert!(*target_node == UNSET, "Duplicate Specified Latency");
            assert!(is_valid(spec.latency));
            *target_node = spec.latency;
            self.to_explore_queue.push_back(spec.node)
        }
        Solution {
            solution: &mut self.solution,
            to_explore_queue: &mut self.to_explore_queue,
        }
    }
}

pub struct Solution<'mem> {
    solution: &'mem mut [i64],
    to_explore_queue: &'mem mut VecDeque<usize>,
}

impl<'mem> Solution<'mem> {
    // Deletes any Poison values.
    fn invert_and_generate_queue(&mut self) {
        assert!(self.to_explore_queue.is_empty());
        for (node, latency) in self.solution.iter_mut().enumerate() {
            *latency = if *latency != UNSET {
                self.to_explore_queue.push_back(node);
                -*latency
            } else {
                UNSET
            };
        }
    }

    fn dfs_fill_poison(&mut self, fanouts: &ListOfLists<FanInOut>, node: usize) {
        for edge in &fanouts[node] {
            let target_node = &mut self.solution[edge.to_node];
            if *target_node != POISON {
                *target_node = POISON;
                self.dfs_fill_poison(fanouts, edge.to_node);
            }
        }
    }

    /// The graph given to this function must be solveable. (IE pass [check_if_unsolveable]), otherwise this will loop forever
    /// Worst-case Complexity O(V*E), but about O(E) for average case
    fn latency_count_bellman_ford(&mut self, fanouts: &ListOfLists<FanInOut>) {
        while let Some(from_idx) = self.to_explore_queue.pop_front() {
            let from_latency = self.solution[from_idx];
            if from_latency == POISON {
                // If this node is poisoned, then poison everything in its fanout immediately. That simplifies downstream logic
                self.dfs_fill_poison(fanouts, from_idx);
            } else {
                for edge in &fanouts[from_idx] {
                    let target_node = &mut self.solution[edge.to_node];
                    let new_value = if let Some(delta) = edge.delta_latency {
                        from_latency + delta
                    } else {
                        POISON
                    };

                    if new_value > *target_node {
                        *target_node = new_value;
                        self.to_explore_queue.push_back(edge.to_node);
                    }
                }
            }
        }
    }

    fn explore_all_connected_nodes(
        &mut self,
        fanins: &ListOfLists<FanInOut>,
        fanouts: &ListOfLists<FanInOut>,
    ) {
        loop {
            self.latency_count_bellman_ford(fanouts);
            self.invert_and_generate_queue();
            let original_num_valid = self.to_explore_queue.len();
            self.latency_count_bellman_ford(fanins);
            self.invert_and_generate_queue();
            let final_num_valid = self.to_explore_queue.len();
            if final_num_valid == original_num_valid {
                break; // No change, we're done
            }
        }
    }

    fn offset_to_pin_node_to(&mut self, spec_lat: SpecifiedLatency) {
        let existing_latency_of_node = self.solution[spec_lat.node];
        assert!(is_valid(existing_latency_of_node));
        let offset = spec_lat.latency - existing_latency_of_node;
        if offset == 0 {
            return; // Early exit, no change needed
        }
        for n in self.to_explore_queue.iter() {
            let lat = &mut self.solution[*n];
            assert!(*lat != POISON);
            if *lat != UNSET {
                *lat += offset;
            }
        }
    }

    fn merge_port_groups(
        &mut self,
        disjoint_groups: &mut Vec<Vec<SpecifiedLatency>>,
        ports: &LatencyCountingPorts,
        bad_ports: &mut Vec<(usize, i64, i64)>,
    ) {
        disjoint_groups.retain_mut(|existing_set| {
            if let Some(offset) = existing_set.iter().find_map(|v| {
                is_valid(self.solution[v.node]).then(|| self.solution[v.node] - v.latency)
            }) {
                for v in existing_set {
                    let latency = v.latency + offset;
                    if self.solution[v.node] == UNSET {
                        self.solution[v.node] = latency;
                    } else if self.solution[v.node] != latency {
                        bad_ports.push((v.node, self.solution[v.node], latency));
                    } else {
                        // The node's already in the set, and the latencies + offset are identical
                    }
                }
                false
            } else {
                true
            }
        });

        let mut new_group = Vec::new();
        for n in &ports.port_nodes {
            let latency = self.solution[*n];
            if is_valid(latency) {
                new_group.push(SpecifiedLatency { node: *n, latency });
            }
        }
        if !new_group.is_empty() {
            disjoint_groups.push(new_group);
        }
    }

    fn copy_to(self, final_solution: &mut [i64]) {
        for n in self.to_explore_queue.drain(..) {
            assert!(final_solution[n] == UNSET);
            final_solution[n] = self.solution[n];
        }
    }
}

#[derive(Clone, Copy, Debug)]
enum NodeProperty {
    Unreached,
    /// Specified nodes NEVER have parent set
    Specified,
    /// Parent node
    HasParent(usize),
}

#[derive(Clone, Copy, Debug)]
struct BellmanFordNode {
    value: i64,
    parent: NodeProperty,
}

#[derive(Debug)]
struct BellmanFordError {
    nodes: Vec<BellmanFordNode>,
    start_from: usize,
    end_at: usize,
}

impl BellmanFordError {
    fn to_lc_error(&self, fanouts: &ListOfLists<FanInOut>) -> LatencyCountingError {
        let mut conflict_path = Vec::new();
        let mut net_latency = -self.nodes[self.end_at].value;
        let mut cur_node = self.end_at;

        loop {
            conflict_path.push(SpecifiedLatency {
                node: cur_node,
                latency: net_latency,
            });
            let NodeProperty::HasParent(parent) = self.nodes[cur_node].parent else {
                unreachable!()
            };

            let mut cur_delta = i64::MIN;
            for fout in &fanouts[parent] {
                if let (true, Some(delta)) = (fout.to_node == cur_node, fout.delta_latency) {
                    cur_delta = i64::max(cur_delta, delta)
                }
            }
            net_latency += cur_delta;

            cur_node = parent;
            if cur_node == self.start_from {
                break;
            }
        }

        if self.start_from == self.end_at {
            LatencyCountingError::NetPositiveLatencyCycle {
                conflict_path,
                net_roundtrip_latency: net_latency,
            }
        } else {
            conflict_path.push(SpecifiedLatency {
                node: cur_node,
                latency: net_latency,
            });
            LatencyCountingError::ConflictingSpecifiedLatencies { conflict_path }
        }
    }
}

/// Check if the graph contains any cycles or incompatible specified latencies.
///
/// This ignores poison edges, since we don't know their value yet
fn find_positive_latency_cycle(
    fanouts: &ListOfLists<FanInOut>,
    specified_latencies: &[SpecifiedLatency],
) -> Result<(), BellmanFordError> {
    use NodeProperty::*;

    let mut nodes = vec![
        BellmanFordNode {
            value: UNSET,
            parent: Unreached
        };
        fanouts.len()
    ];

    let mut queue: VecDeque<usize> = VecDeque::new();
    let mut nodes_ever_seen = Vec::new();
    let mut cur_step = 0;

    for spec in specified_latencies {
        let spec_node = &mut nodes[spec.node];
        spec_node.parent = Specified;
        spec_node.value = -spec.latency; // Negate because we work with fanins not fanouts
        queue.push_back(spec.node);
        nodes_ever_seen.push(spec.node);
        //println!("Init node {} to: {spec_node:?}", spec.node);
    }

    /// Returns either a node with no parent, or a node in an infinite cycle
    fn find_root(nodes: &[BellmanFordNode], mut parent_idx: usize, max_steps: usize) -> usize {
        for _ in 0..max_steps {
            if let HasParent(parent) = nodes[parent_idx].parent {
                parent_idx = parent;
            } else {
                break;
            }
        }
        parent_idx
    }

    let mut next_start = 0;
    loop {
        while let Some(start_from_idx) = queue.pop_front() {
            let start_from = nodes[start_from_idx];
            for f in &fanouts[start_from_idx] {
                // Skip poison edges, since we don't know their value
                let Some(delta_lat) = f.delta_latency else {
                    continue;
                };
                let target_latency = start_from.value + delta_lat;

                let to_node = &mut nodes[f.to_node];
                if target_latency > to_node.value {
                    // We have to already replace the parent, because this way we complete the loop if there is one
                    let old_parent = to_node.parent;
                    to_node.parent = HasParent(start_from_idx);
                    match old_parent {
                        Unreached => {
                            nodes_ever_seen.push(f.to_node);
                        }
                        Specified => {
                            let root = find_root(&nodes, start_from_idx, nodes_ever_seen.len());
                            if let HasParent(_) = nodes[root].parent {
                                // Bad cycle error!
                                nodes[root].value = 0;
                                return Err(BellmanFordError {
                                    nodes,
                                    start_from: root,
                                    end_at: root,
                                });
                            } else {
                                // Incompatible Specified Latencies!
                                return Err(BellmanFordError {
                                    nodes,
                                    start_from: root,
                                    end_at: f.to_node,
                                });
                            }
                        }
                        HasParent(_) => {}
                    }

                    // Update the target node
                    // We only do that now, because otherwise we might've overwritten a Specified Latency
                    to_node.value = target_latency;
                    queue.push_back(f.to_node);

                    //println!("Set node {} to: {to_node:?}", f.to_node);

                    // Occasionally try to walk backwards, to see if we find an infinite cycle.
                    //
                    // Amortize the cost of O(N) backwards walking by only doing it every 1/N times
                    cur_step += 1;
                    if cur_step > nodes_ever_seen.len() {
                        cur_step = 0;
                        let root = find_root(&nodes, start_from_idx, nodes_ever_seen.len());
                        if let HasParent(_) = nodes[root].parent {
                            // Bad cycle error!
                            nodes[root].value = 0;
                            return Err(BellmanFordError {
                                nodes,
                                start_from: root,
                                end_at: root,
                            });
                        }
                    }
                }
            }
        }

        for node in nodes_ever_seen.drain(..) {
            nodes[node].value = i64::MAX; // Set to max because we don't want to visit these anymore.
        }
        cur_step = 0;

        while nodes[next_start].value == i64::MAX {
            next_start += 1;
            if next_start >= nodes.len() {
                return Ok(()); // We're done! ^-^
            }
        }

        nodes_ever_seen.push(next_start);
        nodes[next_start].value = 0;
        queue.push_back(next_start);
        //println!("Init node {next_start} to: {:?}", nodes[next_start]);
    }
}

#[derive(Default, Clone, Debug)]
pub struct LatencyCountingPorts {
    /// All inputs come first, then all outputs
    port_nodes: Vec<usize>,
    outputs_start_at: usize,
}

impl LatencyCountingPorts {
    pub fn push(&mut self, node: usize, direction: Direction) {
        match direction {
            Direction::Input => {
                self.port_nodes.insert(self.outputs_start_at, node);
                self.outputs_start_at += 1;
            }
            Direction::Output => {
                self.port_nodes.push(node);
            }
        }
    }
    pub fn inputs(&self) -> &[usize] {
        &self.port_nodes[..self.outputs_start_at]
    }
    pub fn outputs(&self) -> &[usize] {
        &self.port_nodes[self.outputs_start_at..]
    }

    fn new_from_inference_edges(
        inference_edges: &[LatencyInferenceCandidate],
        num_nodes: usize,
    ) -> Self {
        let mut was_port_seen = vec![None; num_nodes];
        let mut result = Self::default();

        for edge in inference_edges {
            match std::mem::replace(&mut was_port_seen[edge.to_node], Some(true)) {
                None => result.push(edge.to_node, Direction::Input),
                Some(false) => {
                    unreachable!("Inference port cannot be both input and output")
                }
                Some(true) => {}
            }
        }
        for edge in inference_edges {
            match std::mem::replace(&mut was_port_seen[edge.from_node], Some(false)) {
                None => result.push(edge.from_node, Direction::Output),
                Some(true) => {
                    unreachable!("Inference port cannot be both input and output")
                }
                Some(false) => {}
            }
        }
        result
    }
}

fn has_poison_edge(fanouts: &ListOfLists<FanInOut>) -> bool {
    !fanouts
        .iter()
        .all(|fanout_list| fanout_list.iter().all(|f| f.delta_latency.is_some()))
}

#[allow(unused)]
fn print_latency_test_case(
    fanins: &ListOfLists<FanInOut>,
    ports: &LatencyCountingPorts,
    specified_latencies: &[SpecifiedLatency],
) {
    println!("==== BEGIN LATENCY TEST CASE ====");
    println!("#[test]");
    println!("fn new_test_case() {{");
    println!("    let fanins : [&[FanInOut]; {}] = [", fanins.len());
    for (idx, fin) in fanins.iter().enumerate() {
        print!("        /*{idx}*/&[");
        for FanInOut {
            to_node,
            delta_latency,
        } in fin
        {
            if let Some(delta_lat) = delta_latency {
                print!("mk_fan({to_node}, {delta_lat}),")
            } else {
                print!("mk_poisoned({to_node}),")
            }
        }
        println!("],");
    }
    println!("    ];");
    println!("    let fanins = ListOfLists::from_slice_slice(&fanins);");
    println!("    let inputs = {:?};", ports.inputs());
    println!("    let outputs = {:?};", ports.outputs());
    println!("    let specified_latencies = {specified_latencies:?};");
    println!("    let _found_latencies = solve_latencies_test_case(&fanins, &inputs, &outputs, &specified_latencies).unwrap();");
    println!("}}");
    println!("==== END LATENCY TEST CASE ====");
}

#[allow(unused)]
fn print_inference_test_case<ID>(
    fanins: &ListOfLists<FanInOut>,
    ports: &LatencyCountingPorts,
    specified_latencies: &[SpecifiedLatency],
    inference_edges: &[LatencyInferenceCandidate],
    values_to_infer: &FlatAlloc<ValueToInfer<ID>, InferenceVarIDMarker>,
) {
    println!("==== BEGIN INFERENCE TEST CASE ====");
    println!("#[test]");
    println!("fn new_test_case() {{");
    println!("    let fanins : [&[FanInOut]; {}] = [", fanins.len());
    for (idx, fin) in fanins.iter().enumerate() {
        print!("        /*{idx}*/&[");
        for FanInOut {
            to_node,
            delta_latency,
        } in fin
        {
            if let Some(delta_lat) = delta_latency {
                print!("mk_fan({to_node}, {delta_lat}),")
            } else {
                print!("mk_poisoned({to_node}),")
            }
        }
        println!("],");
    }
    println!("    ];");
    println!("    let fanins = ListOfLists::from_slice_slice(&fanins);");
    println!(
        "    let ports = LatencyCountingPorts::from_inputs_outputs(&{:?}, &{:?});",
        ports.inputs(),
        ports.outputs()
    );
    println!("    let specified_latencies = {specified_latencies:?};");

    println!("    let mut values_to_infer = FlatAlloc::new();");
    for (id, _) in values_to_infer {
        println!("    let {id:?} = values_to_infer.alloc(ValueToInfer::new(()));");
    }
    println!("    let inference_edges = vec!{inference_edges:?};");
    println!("    let partial_submodule_info = PartialSubmoduleInfo {{inference_edges, extra_fanin: Vec::new()}};");
    println!("    infer_unknown_latency_edges(fanins, &ports, &specified_latencies, partial_submodule_info, &mut values_to_infer).unwrap();");
    println!("}}");
    println!("==== END INFERENCE TEST CASE ====");
}

/// Guarantees that if `specified_latencies` is non-empty, it'll be the first element in the result vector,
fn solve_port_latencies(
    fanouts: &ListOfLists<FanInOut>,
    ports: &LatencyCountingPorts,
    solution_memory: &mut SolutionMemory,
) -> Result<Vec<Vec<SpecifiedLatency>>, LatencyCountingError> {
    let mut bad_ports: Vec<(usize, i64, i64)> = Vec::new();

    let mut port_groups = ports
        .outputs()
        .iter()
        .copied()
        .map(|node| vec![SpecifiedLatency { node, latency: 0 }])
        .collect();

    for input_port in ports.inputs() {
        let start_node = SpecifiedLatency {
            node: *input_port,
            latency: 0,
        };

        let mut working_latencies =
            solution_memory.make_solution_with_initial_values(&[start_node]);
        working_latencies.latency_count_bellman_ford(fanouts);

        // We have to now remove all other inputs from the solution
        // (inputs that happened to be in the fanout of this input)
        for input_to_remove in ports.inputs() {
            if *input_to_remove != *input_port {
                working_latencies.solution[*input_to_remove] = UNSET;
            }
        }

        working_latencies.merge_port_groups(&mut port_groups, ports, &mut bad_ports);
    }

    if bad_ports.is_empty() {
        Ok(port_groups)
    } else {
        Err(LatencyCountingError::IndeterminablePortLatency { bad_ports })
    }
}

/// Solves the whole latency counting problem. No inference
///
/// Requires fanins to have [ListOfLists::add_extra_fanin_and_specified_latencies] to have been run with `specified_latencies`
///
/// The final solution is a combination of all partial solutions. To make these separate partial solutions more clear, we offset them by 1000 each.
/// Of course, one partial solution will contain the specified latencies, its offset is defined by these.
/// Other partial solutions are assigned increments of 1000 as their minimal absolute latency.
pub fn solve_latencies(
    fanins: ListOfLists<FanInOut>,
    ports: &LatencyCountingPorts,
    specified_latencies: &[SpecifiedLatency],
    mut ports_per_domain: Vec<Vec<usize>>,
) -> Result<Vec<i64>, LatencyCountingError> {
    if fanins.len() == 0 {
        return Ok(Vec::new());
    }

    find_positive_latency_cycle(&fanins, specified_latencies)
        .map_err(|e| e.to_lc_error(&fanins))?;

    if crate::debug::is_enabled("print-solve_latencies-test-case") {
        print_latency_test_case(&fanins, ports, specified_latencies);
    }

    let fanouts = fanins.faninout_complement();
    debug_assert!(!has_poison_edge(&fanins));
    debug_assert!(!has_poison_edge(&fanouts)); // Equivalent

    let mut mem = SolutionMemory::new(fanouts.len());
    let mut solution_seeds = solve_port_latencies(&fanouts, ports, &mut mem)?;

    if !specified_latencies.is_empty() {
        let mut working_latencies = mem.make_solution_with_initial_values(specified_latencies);
        let mut no_bad_port_errors = Vec::new();
        working_latencies.merge_port_groups(&mut solution_seeds, ports, &mut no_bad_port_errors);
        assert!(no_bad_port_errors.is_empty(), "Adding the specified latencies cannot create new bad port errors, because it only applies in the edge case that all specified ports are disjoint inputs, or outputs");
    }

    let mut final_solution = vec![UNSET; fanouts.len()];

    let mut hit_and_not_hit: Vec<(usize, Vec<usize>)> = Vec::new();

    let mut seed_start: i64 = if specified_latencies.is_empty() {
        0
    } else {
        SEPARATE_SEED_OFFSET
    };
    for mut seed in solution_seeds {
        let num_seed_nodes_already_present = seed
            .iter()
            .filter(|s| final_solution[s.node] != UNSET)
            .count();
        if num_seed_nodes_already_present != 0 {
            assert!(num_seed_nodes_already_present == seed.len());
            continue; // Skip this seed, as it's because of an earlier error, so we don't conflict on specified latencies
        }

        let offset = seed_start - seed[0].latency;
        for s in &mut seed {
            s.latency += offset;
        }
        seed_start += SEPARATE_SEED_OFFSET;

        let mut solution = mem.make_solution_with_initial_values(&seed);

        ports_per_domain.retain_mut(|cur_node_set| {
            let num_hit = cur_node_set.partition_point(|n| solution.solution[*n] != i64::MIN);

            if num_hit != 0 && num_hit != cur_node_set.len() {
                hit_and_not_hit.push((num_hit, std::mem::take(cur_node_set)));
            }

            num_hit == 0
        });

        solution.explore_all_connected_nodes(&fanins, &fanouts);

        // Of course, all other specified latencies are in the exact same solution
        if let Some(representative) = specified_latencies.first() {
            if is_valid(solution.solution[representative.node]) {
                solution.offset_to_pin_node_to(*representative);
                seed_start -= SEPARATE_SEED_OFFSET;
            }
        }

        solution.copy_to(&mut final_solution);
    }

    if !hit_and_not_hit.is_empty() {
        return Err(LatencyCountingError::UnreachablePortInThisDomain { hit_and_not_hit });
    }

    for potential_start in 0..fanins.len() {
        if final_solution[potential_start] == UNSET {
            let seed = SpecifiedLatency {
                node: potential_start,
                latency: seed_start,
            };
            seed_start += SEPARATE_SEED_OFFSET;
            let mut solution = mem.make_solution_with_initial_values(&[seed]);
            solution.explore_all_connected_nodes(&fanins, &fanouts);
            solution.copy_to(&mut final_solution);
        }
    }

    Ok(final_solution)
}

/// A candidate for latency inference. Passed to [try_infer_value_for] as a list of possibilities.
///
/// When performing said inference, we return the smallest valid candidate. All candidates _must_ try to provide a value.
#[derive(Debug)]
pub struct LatencyInferenceCandidate {
    pub multiply_var_by: i64,
    pub from_node: usize,
    pub to_node: usize,
    pub offset: i64,
    pub target_to_infer: LatencyCountInferenceVarID,
}

pub struct ValueToInfer<ID> {
    /// Initially Some([i64::MAX]), decreasing. Set to None when a [LatencyInferenceCandidate] targets it, but cannot be resolved
    inferred_value: Option<i64>,
    /// Represents if the variable is being used in an edge with a positive coefficient ('0 -> 'V), or a negative coefficient ('V -> '0)
    /// Used to see in what direction multiple inferences should be combined.
    /// This is because the resulting value should be as lax as possible.
    /// Example:
    /// ```sus
    /// module infer_me #(int V) {
    ///     interface a : bool iA'0 -> bool oA'V
    ///     interface b : bool iB'0 -> bool oB'V+3
    /// }
    /// module use_infer_me {
    ///     interface i : bool i'0 -> bool o'5
    ///
    ///     infer_me iii
    ///     o = iii.a(i) // Requires V <= 5
    ///     o = iii.b(i) // Requires V <= 2 // V becomes 2
    /// }
    /// ```
    /// If positive, then we take the max of the possible candidates, otherwise the min.
    /// If the user specifies both a negative, and a positive offset, then we can't infer without possibly breaking things, and so we .spoil() immediately
    pub linear_factor_is_positive: bool,
    pub back_reference: ID,
}

impl<ID> ValueToInfer<ID> {
    pub fn new(back_reference: ID, linear_factor_is_positive: bool) -> Self {
        Self {
            inferred_value: Some(if linear_factor_is_positive {
                i64::MAX
            } else {
                i64::MIN
            }),
            linear_factor_is_positive,
            back_reference,
        }
    }
    pub fn get(&self) -> Option<i64> {
        self.inferred_value
            .and_then(|v| (v != i64::MAX).then_some(v))
    }
    fn apply_candidate(&mut self, candidate_value: i64) {
        if let Some(v) = &mut self.inferred_value {
            *v = if self.linear_factor_is_positive {
                i64::min(*v, candidate_value)
            } else {
                i64::max(*v, candidate_value)
            };
        }
    }
    pub fn spoil(&mut self) {
        self.inferred_value = None;
    }
}

/// Tries to infer the inference edges given in [inference_candidates].
///
/// This method takes both the real ports of the module, as wel as inference pseudo-ports.
///
/// Every candidate in [inference_candidates] must start at an "output" port, and end at an "input" port, in [inference_ports]
///
/// We pass fanins by value, as this lets us more efficiently edit it in the implementation.
///
/// Requires fanins to have [ListOfLists::add_extra_fanin_and_specified_latencies] to have been run with `specified_latencies`
pub fn infer_unknown_latency_edges<ID>(
    fanins: ListOfLists<FanInOut>,
    ports: &LatencyCountingPorts,
    specified_latencies: &[SpecifiedLatency],
    inference_edges: &[LatencyInferenceCandidate],
    values_to_infer: &mut FlatAlloc<ValueToInfer<ID>, InferenceVarIDMarker>,
) -> Result<(), LatencyCountingError> {
    if crate::debug::is_enabled("print-infer_unknown_latency_edges-test-case") {
        print_inference_test_case(
            &fanins,
            ports,
            specified_latencies,
            inference_edges,
            values_to_infer,
        );
    }

    if fanins.len() == 0 || inference_edges.is_empty() {
        return Ok(()); // Could not infer anything
    }

    find_positive_latency_cycle(&fanins, specified_latencies)
        .map_err(|e| e.to_lc_error(&fanins))?;

    let fanouts = fanins.faninout_complement();

    let mut mem = SolutionMemory::new(fanouts.len());
    let partial_solutions = solve_port_latencies(&fanouts, ports, &mut mem)?;

    let mut new_edges = Vec::new();
    for partial_sol in partial_solutions {
        add_cycle_to_extra_fanin(&partial_sol, &mut new_edges);
    }

    let fanins = fanins.extend_lists_with_new_elements(new_edges);
    let fanouts = fanins.faninout_complement();

    let inference_ports =
        LatencyCountingPorts::new_from_inference_edges(inference_edges, fanins.len());

    if crate::debug::is_enabled("print-inference-as-latency-test-case") {
        print_latency_test_case(&fanins, &inference_ports, &[]);
    }

    let inference_port_solutions = solve_port_latencies(&fanouts, &inference_ports, &mut mem)?;

    for candidate in inference_edges {
        let mut infer_me = Some(&mut values_to_infer[candidate.target_to_infer]);

        for possible_port_solution_set in &inference_port_solutions {
            if let (Some(from), Some(to)) = (
                SpecifiedLatency::get_from_specify_list(
                    possible_port_solution_set,
                    candidate.from_node,
                ),
                SpecifiedLatency::get_from_specify_list(
                    possible_port_solution_set,
                    candidate.to_node,
                ),
            ) {
                let candidate_value = (to - from - candidate.offset) / candidate.multiply_var_by;
                let target_to_infer = infer_me.take().expect(
                    "At most one partial solution can have a possible value for the candidate",
                );
                target_to_infer.apply_candidate(candidate_value);
            }
        }

        if let Some(infer_was_not_found) = infer_me {
            infer_was_not_found.spoil();
        }
    }

    Ok(())
}

#[cfg(test)]
pub fn mk_fan(to_node: usize, delta_latency: i64) -> FanInOut {
    FanInOut {
        to_node,
        delta_latency: Some(delta_latency),
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    pub fn mk_poisoned(to_node: usize) -> FanInOut {
        FanInOut {
            to_node,
            delta_latency: None,
        }
    }

    impl LatencyCountingPorts {
        pub fn from_inputs_outputs(inputs: &[usize], outputs: &[usize]) -> Self {
            Self {
                port_nodes: inputs.iter().chain(outputs.iter()).cloned().collect(),
                outputs_start_at: inputs.len(),
            }
        }
    }

    fn solve_latencies_test_case(
        fanins: ListOfLists<FanInOut>,
        inputs: &[usize],
        outputs: &[usize],
        specified_latencies: &[SpecifiedLatency],
    ) -> Result<Vec<i64>, LatencyCountingError> {
        let ports = LatencyCountingPorts::from_inputs_outputs(inputs, outputs);

        let fanins =
            fanins.add_extra_fanin_and_specified_latencies(Vec::new(), specified_latencies);

        solve_latencies(fanins, &ports, specified_latencies, Vec::new())
    }

    pub fn infer_unknown_latency_edges_test_case<ID>(
        fanins: ListOfLists<FanInOut>,
        inputs: &[usize],
        outputs: &[usize],
        specified_latencies: &[SpecifiedLatency],
        inference_edges: &[LatencyInferenceCandidate],
        values_to_infer: &mut FlatAlloc<ValueToInfer<ID>, InferenceVarIDMarker>,
    ) -> Result<(), LatencyCountingError> {
        let ports = LatencyCountingPorts::from_inputs_outputs(inputs, outputs);

        let fanins =
            fanins.add_extra_fanin_and_specified_latencies(Vec::new(), specified_latencies);

        infer_unknown_latency_edges(
            fanins,
            &ports,
            specified_latencies,
            inference_edges,
            values_to_infer,
        )
    }

    #[track_caller]
    fn assert_latencies_match_exactly(found: &[i64], correct: &[i64]) {
        assert_eq!(found.len(), correct.len());

        assert!(
            std::iter::zip(found, correct).all(|(x, y)| *x == *y),
            "Latencies don't match exactly: {found:?} !=lat= {correct:?}"
        );
    }

    #[track_caller]
    fn assert_latency_nodes_match_exactly(found: &[i64], correct: &[i64]) {
        assert_eq!(found.len(), correct.len());

        assert!(
            std::iter::zip(found, correct).all(|(x, y)| { *x == *y }),
            "Latencies don't match exactly: {found:?} !=lat= {correct:?}"
        );
    }

    #[track_caller]
    /// Assert that all found latencies are valid, and that they match the given list
    ///
    /// This means that all the given latencies are "known", and also that
    fn assert_latencies_match(found: &[i64], correct: &[i64]) {
        assert_eq!(found.len(), correct.len());
        let diff = found[0] - correct[0];

        assert!(
            std::iter::zip(found, correct).all(|(x, y)| x - y == diff),
            "Latencies don't match even with an offset: {found:?} != {correct:?}"
        );
    }

    fn normalize_specified_latency_lists(list: &mut [Vec<SpecifiedLatency>]) {
        for l in list.iter_mut() {
            l.sort_by(|a, b| a.node.cmp(&b.node));
            let offset = l[0].latency;
            for v in l {
                v.latency -= offset;
            }
        }

        list.sort_by(|a, b| a[0].node.cmp(&b[0].node));
    }

    fn assert_specified_latency_lists_match(
        left: &mut [Vec<SpecifiedLatency>],
        right: &mut [Vec<SpecifiedLatency>],
    ) {
        normalize_specified_latency_lists(left);
        normalize_specified_latency_lists(right);

        assert_eq!(left, right);
    }

    #[test]
    fn check_correct_latency_basic() {
        let fanins: [&[FanInOut]; 7] = [
            /*0*/ &[],
            /*1*/ &[mk_fan(0, 0)],
            /*2*/ &[mk_fan(1, 1), mk_fan(5, 1)],
            /*3*/ &[mk_fan(2, 0)],
            /*4*/ &[],
            /*5*/ &[mk_fan(4, 0), mk_fan(1, 1)],
            /*6*/ &[mk_fan(5, 0)],
        ];
        let fanins = ListOfLists::from_slice_slice(&fanins);

        let correct_latencies = [0, 0, 2, 2, 1, 1, 1];

        let inputs = [0, 4];
        let outputs = [3, 6];
        let specified_latencies = [];

        let found_latencies =
            solve_latencies_test_case(fanins, &inputs, &outputs, &specified_latencies).unwrap();

        assert_latencies_match(&found_latencies, &correct_latencies);
    }

    #[test]
    fn check_correct_latency_backwards() {
        let fanins: [&[FanInOut]; 7] = [
            /*0*/ &[],
            /*1*/ &[mk_fan(0, 0)],
            /*2*/ &[mk_fan(1, 1), mk_fan(5, 1)],
            /*3*/ &[mk_fan(2, 0)],
            /*4*/ &[],
            /*5*/ &[mk_fan(4, 0), mk_fan(1, 1)],
            /*6*/ &[mk_fan(5, 0)],
        ];
        let fanins = ListOfLists::from_slice_slice(&fanins);

        let correct_latencies = [-1, -1, 1, 1, 0, 0, 0];

        let inputs = [0, 4];
        let outputs = [3, 6];
        let specified_latencies = [SpecifiedLatency {
            node: 6,
            latency: 0,
        }];

        let found_latencies =
            solve_latencies_test_case(fanins, &inputs, &outputs, &specified_latencies).unwrap();

        assert_latencies_match_exactly(&found_latencies, &correct_latencies);
    }

    #[test]
    fn check_correct_latency_from_any_start_node() {
        let fanins: [&[FanInOut]; 7] = [
            /*0*/ &[],
            /*1*/ &[mk_fan(0, 0)],
            /*2*/ &[mk_fan(1, 1), mk_fan(5, 1)],
            /*3*/ &[mk_fan(2, 0)],
            /*4*/ &[],
            /*5*/ &[mk_fan(4, 0), mk_fan(1, 1)],
            /*6*/ &[mk_fan(5, 0)],
        ];
        let fanins = ListOfLists::from_slice_slice(&fanins);

        let correct_latencies = [0, 0, 2, 2, 1, 1, 1];

        let inputs = [0, 4];
        let outputs = [3, 6];

        for starting_node in 0..7 {
            println!("starting_node: {starting_node}");

            // Apparently this edge case was fixed by including the specified latencies in future port traversals. Who could've predicted that???
            /*if starting_node == 5 {
                let err = solve_latencies_test_case(
                    &fanins,
                    &inputs,
                    &outputs,
                    &[SpecifiedLatency {
                        node: starting_node,
                        latency: 0,
                    }],
                );
                let Err(LatencyCountingError::IndeterminablePortLatency { bad_ports: _ }) = err
                else {
                    unreachable!("{err:?}")
                };
                continue;
            }*/
            let found_latencies = solve_latencies_test_case(
                fanins.clone(),
                &inputs,
                &outputs,
                &[SpecifiedLatency {
                    node: starting_node,
                    latency: 0,
                }],
            )
            .unwrap();

            assert_latencies_match(&found_latencies, &correct_latencies);
        }
    }

    #[test]
    /// Happens with constants in the code for instance
    fn check_correct_latency_with_superfluous_input() {
        let fanins: [&[FanInOut]; 8] = [
            /*0*/ &[],
            /*1*/ &[mk_fan(0, 0)],
            /*2*/ &[mk_fan(1, 1), mk_fan(5, 1)],
            /*3*/ &[mk_fan(2, 0)],
            /*4*/ &[],
            /*5*/ &[mk_fan(4, 0), mk_fan(1, 1), mk_fan(7, 2)],
            /*6*/ &[mk_fan(5, 0)],
            /*7*/ &[], // superfluous input: no-fanin node, but not marked as an input.
        ];
        let fanins = ListOfLists::from_slice_slice(&fanins);

        let correct_latencies = [0, 0, 2, 2, 1, 1, 1, -1];

        let inputs = [0, 4];
        let outputs = [3, 6];
        let specified_latencies = [];

        let found_latencies =
            solve_latencies_test_case(fanins, &inputs, &outputs, &specified_latencies).unwrap();

        assert_latencies_match(&found_latencies, &correct_latencies);
    }

    #[test]
    fn check_correct_latency_with_superfluous_output() {
        let fanins: [&[FanInOut]; 8] = [
            /*0*/ &[],
            /*1*/ &[mk_fan(0, 0)],
            /*2*/ &[mk_fan(1, 1), mk_fan(5, 1)],
            /*3*/ &[mk_fan(2, 0)],
            /*4*/ &[],
            /*5*/ &[mk_fan(4, 0), mk_fan(1, 1)],
            /*6*/ &[mk_fan(5, 0)],
            /*7*/ &[mk_fan(5, 2)], // superfluous output
        ];
        let fanins = ListOfLists::from_slice_slice(&fanins);

        let correct_latencies = [-1, -1, 1, 1, 0, 0, 0, 2];

        let inputs = [0, 4];
        let outputs = [3, 6];
        let specified_latencies = [];

        let found_latencies =
            solve_latencies_test_case(fanins, &inputs, &outputs, &specified_latencies).unwrap();

        assert_latencies_match(&found_latencies, &correct_latencies);
    }

    #[test]
    fn check_conflicting_port_latency() {
        let fanins: [&[FanInOut]; 7] = [
            /*0*/ &[],
            /*1*/ &[mk_fan(0, 0)],
            /*2*/ &[mk_fan(1, 3), mk_fan(5, 1)],
            /*3*/ &[mk_fan(2, 0)],
            /*4*/ &[],
            /*5*/ &[mk_fan(4, 0), mk_fan(1, 1)],
            /*6*/ &[mk_fan(5, 0)],
        ];
        let fanins = ListOfLists::from_slice_slice(&fanins);

        let should_be_err = solve_latencies_test_case(fanins, &[0, 4], &[3, 6], &[]);

        assert!(matches!(
            should_be_err,
            Err(LatencyCountingError::IndeterminablePortLatency { bad_ports: _ })
        ))
    }

    #[test]
    fn test_inputs_only() {
        let fanins: [&[FanInOut]; 7] = [
            /*0*/ &[],
            /*1*/ &[mk_fan(0, 0)],
            /*2*/ &[mk_fan(1, 1), mk_fan(5, 1)],
            /*3*/ &[mk_fan(2, 0)],
            /*4*/ &[],
            /*5*/ &[mk_fan(4, 0), mk_fan(1, 1)],
            /*6*/ &[mk_fan(5, 0)],
        ];
        let fanins = ListOfLists::from_slice_slice(&fanins);

        let found_latencies = solve_latencies_test_case(fanins, &[0, 4], &[], &[]).unwrap();

        assert_latencies_match(&found_latencies, &[0, 0, 2, 2, 1, 1, 1]);
    }

    #[test]
    fn test_outputs_only() {
        let fanins: [&[FanInOut]; 7] = [
            /*0*/ &[],
            /*1*/ &[mk_fan(0, 0)],
            /*2*/ &[mk_fan(1, 1), mk_fan(5, 1)],
            /*3*/ &[mk_fan(2, 0)],
            /*4*/ &[],
            /*5*/ &[mk_fan(4, 0), mk_fan(1, 1)],
            /*6*/ &[mk_fan(5, 0)],
        ];
        let fanins = ListOfLists::from_slice_slice(&fanins);

        let found_latencies = solve_latencies_test_case(fanins, &[], &[3, 6], &[]).unwrap();

        assert_latencies_match(&found_latencies, &[0, 0, 2, 2, 1, 1, 1]);
    }

    #[test]
    // Kinda outdated, because specified latencies no longer signify the start of the algorithm
    fn check_conflicting_port_latency_with_any_starting_node_does_error() {
        /*
            i0 - 1 - 2 - 3o
                  \ /
              i4 - 5 - 6o
        */
        let fanins: [&[FanInOut]; 7] = [
            /*0*/ &[],
            /*1*/ &[mk_fan(0, 0)],
            /*2*/ &[mk_fan(1, 3), mk_fan(5, 1)],
            /*3*/ &[mk_fan(2, 0)],
            /*4*/ &[],
            /*5*/ &[mk_fan(4, 0), mk_fan(1, 1)],
            /*6*/ &[mk_fan(5, 0)],
        ];
        let fanins = ListOfLists::from_slice_slice(&fanins);

        for starting_node in 0..7 {
            println!("starting_node: {starting_node}");
            solve_latencies_test_case(
                fanins.clone(),
                &[0, 4],
                &[3, 6],
                &[SpecifiedLatency {
                    node: starting_node,
                    latency: 0,
                }],
            )
            .unwrap_err();
        }
    }

    #[test]
    fn check_conflicting_port_latency_resolved() {
        let fanins: [&[FanInOut]; 7] = [
            /*0*/ &[],
            /*1*/ &[mk_fan(0, 0)],
            /*2*/ &[mk_fan(1, 3), mk_fan(5, 1)],
            /*3*/ &[mk_fan(2, 0)],
            /*4*/ &[],
            /*5*/ &[mk_fan(4, 0), mk_fan(1, 1)],
            /*6*/ &[mk_fan(5, 0)],
        ];
        let fanins = ListOfLists::from_slice_slice(&fanins);

        let specified_latencies = [
            SpecifiedLatency {
                node: 0,
                latency: 0,
            },
            SpecifiedLatency {
                node: 4,
                latency: 2,
            },
        ];

        let found_latencies =
            solve_latencies_test_case(fanins, &[0, 4], &[3, 6], &specified_latencies).unwrap();

        let correct_latencies = [0, 0, 3, 3, 2, 2, 2];

        assert_latencies_match_exactly(&found_latencies, &correct_latencies); // Can even do a strict check here, because we defined some of the latencies
    }

    #[test]
    fn loose_bad_cycle_gets_detected() {
        let fanins: [&[FanInOut]; 7] = [
            /*0*/ &[mk_fan(2, 2)], // Good cycle
            /*1*/ &[mk_fan(0, -1)],
            /*2*/ &[mk_fan(1, -2)],
            /*3*/ &[mk_fan(6, 3)], // Bad cycle
            /*4*/ &[mk_fan(3, -4)],
            /*5*/ &[mk_fan(4, 2)],
            /*6*/ &[mk_fan(5, 1)],
        ];
        let fanins = ListOfLists::from_slice_slice(&fanins);

        let should_be_err = solve_latencies_test_case(fanins, &[], &[], &[]);

        println!("{should_be_err:?}");
        let Err(LatencyCountingError::NetPositiveLatencyCycle {
            conflict_path,
            net_roundtrip_latency,
        }) = should_be_err
        else {
            unreachable!()
        };
        assert_eq!(net_roundtrip_latency, 2);
        assert!(conflict_path.len() == 4);
        let mut conflict_path_nodes: Vec<usize> = conflict_path.iter().map(|e| e.node).collect();
        conflict_path_nodes.sort();
        assert_eq!(&conflict_path_nodes, &[3, 4, 5, 6]);
        /*assert_eq!(
            conflict_path,
            &[
                SpecifiedLatency {
                    node: 3,
                    latency: 0
                },
                SpecifiedLatency {
                    node: 4,
                    latency: -4
                },
                SpecifiedLatency {
                    node: 5,
                    latency: -2
                },
                SpecifiedLatency {
                    node: 6,
                    latency: -1
                }
            ]
        );*/
    }

    #[test]
    fn check_conflicting_port_specifiers() {
        let fanins: [&[FanInOut]; 7] = [
            /*0*/ &[],
            /*1*/ &[mk_fan(0, 0)],
            /*2*/ &[mk_fan(1, 1), mk_fan(5, 1)],
            /*3*/ &[mk_fan(2, 0)],
            /*4*/ &[],
            /*5*/ &[mk_fan(4, 0), mk_fan(1, 1)],
            /*6*/ &[mk_fan(5, 0)],
        ];
        let fanins = ListOfLists::from_slice_slice(&fanins);
        let specified_latencies = [
            SpecifiedLatency {
                node: 0,
                latency: 10,
            },
            SpecifiedLatency {
                node: 3,
                latency: 11,
            },
        ];

        let should_be_err =
            solve_latencies_test_case(fanins, &[0, 4], &[3, 6], &specified_latencies);

        println!("{should_be_err:?}");
        let Err(LatencyCountingError::ConflictingSpecifiedLatencies { conflict_path }) =
            should_be_err
        else {
            unreachable!()
        };
        let path_latency =
            conflict_path.last().unwrap().latency - conflict_path.first().unwrap().latency;
        assert_eq!(path_latency, 2);
        assert_eq!(
            conflict_path,
            &[
                SpecifiedLatency {
                    node: 0,
                    latency: 10
                },
                SpecifiedLatency {
                    node: 1,
                    latency: 10
                },
                SpecifiedLatency {
                    node: 5,
                    latency: 11
                },
                SpecifiedLatency {
                    node: 2,
                    latency: 12
                },
                SpecifiedLatency {
                    node: 3,
                    latency: 12
                }
            ]
        );
    }

    #[test]
    fn check_conflicting_inline_specifiers() {
        let fanins: [&[FanInOut]; 7] = [
            /*0*/ &[],
            /*1*/ &[mk_fan(0, 0)],
            /*2*/ &[mk_fan(1, 1), mk_fan(5, 1)],
            /*3*/ &[mk_fan(2, 0)],
            /*4*/ &[],
            /*5*/ &[mk_fan(4, 0), mk_fan(1, 1)],
            /*6*/ &[mk_fan(5, 0)],
        ];
        let fanins = ListOfLists::from_slice_slice(&fanins);
        let specified_latencies = [
            SpecifiedLatency {
                node: 1,
                latency: -10,
            },
            SpecifiedLatency {
                node: 5,
                latency: -10,
            },
        ];

        let should_be_err =
            solve_latencies_test_case(fanins, &[0, 4], &[3, 6], &specified_latencies);

        let Err(LatencyCountingError::ConflictingSpecifiedLatencies { conflict_path }) =
            should_be_err
        else {
            unreachable!()
        };
        assert_eq!(
            conflict_path,
            &[
                SpecifiedLatency {
                    node: 1,
                    latency: -10
                },
                SpecifiedLatency {
                    node: 5,
                    latency: -9
                }
            ]
        );
        let path_latency =
            conflict_path.last().unwrap().latency - conflict_path.first().unwrap().latency;
        assert_eq!(path_latency, 1);
    }

    #[test]
    fn check_conflicting_inline_specifiers_bad_case() {
        let fanins: [&[FanInOut]; 3] = [
            /*0*/ &[],
            /*1*/ &[mk_fan(2, 1)],
            /*2*/ &[mk_fan(0, 1)],
        ];
        let fanins = ListOfLists::from_slice_slice(&fanins);
        let specified_latencies = [
            SpecifiedLatency {
                node: 0,
                latency: 3,
            },
            SpecifiedLatency {
                node: 1,
                latency: 4,
            },
        ];

        let should_be_err = solve_latencies_test_case(fanins, &[0], &[1], &specified_latencies);
        println!("{should_be_err:?}");

        let Err(LatencyCountingError::ConflictingSpecifiedLatencies { conflict_path }) =
            should_be_err
        else {
            unreachable!()
        };

        assert_eq!(
            conflict_path,
            &[
                SpecifiedLatency {
                    node: 0,
                    latency: 3
                },
                SpecifiedLatency {
                    node: 2,
                    latency: 4
                },
                SpecifiedLatency {
                    node: 1,
                    latency: 5
                }
            ]
        );
        let path_latency =
            conflict_path.last().unwrap().latency - conflict_path.first().unwrap().latency;
        assert_eq!(path_latency, 2);
    }

    #[test]
    fn check_disjoint() {
        let fanins: [&[FanInOut]; 7] = [
            /*0*/ &[],
            /*1*/ &[mk_fan(0, 0)],
            /*2*/ &[mk_fan(1, 3)],
            /*3*/ &[mk_fan(2, 0)],
            /*4*/ &[],
            /*5*/ &[mk_fan(4, 0)],
            /*6*/ &[mk_fan(5, 0)],
        ];
        let fanins = ListOfLists::from_slice_slice(&fanins);
        let specified_latencies = [SpecifiedLatency {
            node: 0,
            latency: 0,
        }];

        let result =
            solve_latencies_test_case(fanins, &[0, 4], &[3, 6], &specified_latencies).unwrap();

        let correct_latencies = [
            0,
            0,
            3,
            3,
            SEPARATE_SEED_OFFSET,
            SEPARATE_SEED_OFFSET,
            SEPARATE_SEED_OFFSET,
        ];
        assert_latency_nodes_match_exactly(&result, &correct_latencies)
    }

    /*
    These tests were commented out because joining non-strongly-connected ports is no longer supported

    #[test]
    fn check_disjoint_input_still_should_succeed() {
        /*
            i0 - 1 - 2o
                /
               6
                \
            i3 - 4 - 5o
        */
        let fanins: [&[FanInOut]; 7] = [
            /*0*/ &[], // First half
            /*1*/ &[mk_fan(0, 1), mk_fan(6, 0)],
            /*2*/ &[mk_fan(1, 1)],
            /*3*/ &[],
            /*4*/ &[mk_fan(3, 2), mk_fan(6, 3)],
            /*5*/ &[mk_fan(4, 2)],
            /*6*/ &[],
        ];
        let fanins = ListOfLists::from_slice_slice(&fanins);

        let inputs = [0, 3];
        let outputs = [2, 5];
        let specified_latencies = [];

        let partial_result =
            solve_latencies_test_case(fanins, &inputs, &outputs, &specified_latencies).unwrap();

        let correct_latencies = [0, 1, 2, 2, 4, 6, 1];
        assert_latencies_match(&partial_result, &correct_latencies)
    }

    #[test]
    fn check_disjoint_output_still_should_succeed() {
        /*
            i0 - 1 - 2o
                  \
                   6
                  /
            i3 - 4 - 5o
        */
        let fanins: [&[FanInOut]; 7] = [
            /*0*/ &[], // First half
            /*1*/ &[mk_fan(0, 1)],
            /*2*/ &[mk_fan(1, 1)],
            /*3*/ &[],
            /*4*/ &[mk_fan(3, 2)],
            /*5*/ &[mk_fan(4, 2)],
            /*6*/ &[mk_fan(1, 0), mk_fan(4, 3)],
        ];
        let fanins = ListOfLists::from_slice_slice(&fanins);

        let inputs = [0, 3];
        let outputs = [2, 5];
        let specified_latencies = [];

        let partial_result =
            solve_latencies_test_case(fanins, &inputs, &outputs, &specified_latencies).unwrap();

        let correct_latencies = [0, 1, 2, -4, -2, 0, 1];
        assert_latencies_match(&partial_result, &correct_latencies)
    }

    #[test]
    fn check_partial_solution_combination_error() {
        /*
            i0 - 1 - 2o
               / /
               6 7
               \ \
            i3 - 4 - 5o
        */
        let fanins: [&[FanInOut]; 8] = [
            /*0*/ &[], // First half
            /*1*/ &[mk_fan(0, 0), mk_fan(6, 0), mk_fan(7, 0)],
            /*2*/ &[mk_fan(1, 0)],
            /*3*/ &[],
            /*4*/ &[mk_fan(3, 0), mk_fan(6, 5), mk_fan(7, 2)],
            /*5*/ &[mk_fan(4, 0)],
            /*6*/ &[],
            /*7*/ &[],
        ];
        let fanins = ListOfLists::from_slice_slice(&fanins);

        let inputs = [0, 3];
        let outputs = [2, 5];
        let specified_latencies = [];

        let err_solution =
            solve_latencies_test_case(fanins, &inputs, &outputs, &specified_latencies);

        let Err(LatencyCountingError::PartialSolutionMergeConflict { bad_nodes: _ }) = err_solution
        else {
            panic!("{err_solution:?}")
        };
    }*/

    #[test]
    fn check_bad_cycle() {
        let fanins: [&[FanInOut]; 5] = [
            /*0*/ &[],
            /*1*/ &[mk_fan(0, 0), mk_fan(4, -4)],
            /*2*/ &[mk_fan(1, 3)],
            /*3*/ &[mk_fan(2, 0)],
            /*4*/ &[mk_fan(2, 2)],
        ];
        let fanins = ListOfLists::from_slice_slice(&fanins);
        let specified_latencies = [];

        let should_be_err = solve_latencies_test_case(fanins, &[0], &[3], &specified_latencies);

        let Err(LatencyCountingError::NetPositiveLatencyCycle {
            conflict_path: _,
            net_roundtrip_latency,
        }) = should_be_err
        else {
            unreachable!()
        };
        assert_eq!(net_roundtrip_latency, 1);
    }

    #[test]
    fn input_used_further() {
        let fanins: [&[FanInOut]; 4] = [
            /*0*/ &[],
            /*1*/ &[mk_fan(0, 1)],
            /*2*/ &[mk_fan(1, 1)],
            /*3*/ &[mk_fan(2, 1)],
        ];
        let fanins = ListOfLists::from_slice_slice(&fanins);
        let specified_latencies = [];

        let found_latencies =
            solve_latencies_test_case(fanins, &[0, 1], &[3], &specified_latencies).unwrap();

        let correct_latencies = [0, 1, 2, 3];
        assert_latencies_match_exactly(&found_latencies, &correct_latencies);
    }

    #[test]
    fn output_used_further() {
        let fanins: [&[FanInOut]; 4] = [
            /*0*/ &[],
            /*1*/ &[mk_fan(0, 1)],
            /*2*/ &[mk_fan(1, 1)],
            /*3*/ &[mk_fan(2, 1)],
        ];
        let fanins = ListOfLists::from_slice_slice(&fanins);
        let specified_latencies = [];

        let found_latencies =
            solve_latencies_test_case(fanins, &[0], &[2, 3], &specified_latencies).unwrap();

        let correct_latencies = [0, 1, 2, 3];
        assert_latencies_match_exactly(&found_latencies, &correct_latencies);
    }

    #[test]
    fn test_can_infer_through_specified_latencies() {
        /*
            0 -------- 3
             \        /
              1'0    2'5
        */
        let fanins: [&[FanInOut]; 4] = [
            /*0*/ &[],
            /*1*/ &[mk_fan(0, 0)],
            /*2*/ &[],
            /*3*/ &[mk_fan(0, 0), mk_fan(2, 0)],
        ];
        let fanins = ListOfLists::from_slice_slice(&fanins);

        let inputs = [0];
        let outputs = [3];
        let specified_latencies = [
            SpecifiedLatency {
                node: 1,
                latency: 0,
            },
            SpecifiedLatency {
                node: 2,
                latency: 5,
            },
        ];

        let found_latencies =
            solve_latencies_test_case(fanins, &inputs, &outputs, &specified_latencies).unwrap();

        let correct_latencies = [0, 0, 5, 5];
        assert_latencies_match_exactly(&found_latencies, &correct_latencies);
    }

    #[test]
    /// Checks that poison values properly propagate
    fn test_poison_edges() {
        let fanins: [&[FanInOut]; 7] = [
            /*0*/ &[],
            /*1*/ &[], // This is the node that should be poisoned
            /*2*/ &[mk_fan(0, 1)],
            /*3*/ &[mk_fan(0, 2), mk_poisoned(1)],
            /*4*/ &[mk_fan(2, 2), mk_fan(5, 2)], // And an inference edge from 3 -> 4
            /*5*/ &[mk_fan(3, 1), mk_fan(1, 3)],
            /*6*/ &[mk_fan(5, 3), mk_fan(4, 4)],
        ];
        let fanins = ListOfLists::from_slice_slice(&fanins);

        let inputs = [0, 1];
        let outputs = [6];

        let fanouts = fanins.faninout_complement();

        let ports = LatencyCountingPorts::from_inputs_outputs(&inputs, &outputs);
        let mut mem = SolutionMemory::new(fanouts.len());
        let mut partial_solutions = solve_port_latencies(&fanouts, &ports, &mut mem).unwrap();

        assert_specified_latency_lists_match(
            &mut partial_solutions,
            &mut [
                vec![
                    SpecifiedLatency {
                        node: 0,
                        latency: 0,
                    },
                    SpecifiedLatency {
                        node: 6,
                        latency: 9,
                    },
                ],
                vec![SpecifiedLatency {
                    node: 1,
                    latency: 0,
                }],
            ],
        );
    }

    #[test]
    /// Checks that poison values properly propagate
    fn test_inference_no_poison() {
        /*
                2 -\?A
               /      6
              1-3 -/?B \
             /          8 - 9 -?D 10
            0 - 4 -\?C /|
            | \       7 |
            |   5 -/?B  |
            ------------|
        */
        let fanins: [&[FanInOut]; 11] = [
            /*0*/ &[],
            /*1*/ &[mk_fan(0, 0)],
            /*2*/ &[mk_fan(1, 1)],
            /*3*/ &[mk_fan(1, 6)],
            /*4*/ &[mk_fan(0, 2)],
            /*5*/ &[mk_fan(0, 5)],
            /*6*/ &[], // inference_edge(2) for A, inference_edge(3) for B
            /*7*/ &[], // inference_edge(4) for C, inference_edge(5) for B
            /*8*/ &[mk_fan(6, 3), mk_fan(7, 2), mk_fan(0, 10)],
            /*9*/ &[mk_fan(8, 0)],
            /*10*/ &[], // inference_edge(9), disjoint so can't be inferred
        ];
        let fanins = ListOfLists::from_slice_slice(&fanins);

        let inputs = [0];
        let outputs = [8];
        let specified_latencies = [];

        let mut values_to_infer = FlatAlloc::new();
        let a = values_to_infer.alloc(ValueToInfer::new((), true));
        let b = values_to_infer.alloc(ValueToInfer::new((), true)); // Shared by two inference candidates
        let c = values_to_infer.alloc(ValueToInfer::new((), true));
        let d = values_to_infer.alloc(ValueToInfer::new((), true)); // Cannot be inferred

        let inference_edges = vec![
            LatencyInferenceCandidate {
                multiply_var_by: 1,
                from_node: 2,
                to_node: 6,
                offset: 0,
                target_to_infer: a,
            },
            LatencyInferenceCandidate {
                multiply_var_by: 1,
                from_node: 3,
                to_node: 6,
                offset: 0,
                target_to_infer: b,
            },
            LatencyInferenceCandidate {
                multiply_var_by: 1,
                from_node: 4,
                to_node: 7,
                offset: 0,
                target_to_infer: c,
            },
            LatencyInferenceCandidate {
                multiply_var_by: 1,
                from_node: 5,
                to_node: 7,
                offset: 0,
                target_to_infer: b,
            },
            LatencyInferenceCandidate {
                multiply_var_by: 1,
                from_node: 9,
                to_node: 10,
                offset: 0,
                target_to_infer: d,
            },
        ];

        infer_unknown_latency_edges_test_case(
            fanins,
            &inputs,
            &outputs,
            &specified_latencies,
            &inference_edges,
            &mut values_to_infer,
        )
        .unwrap();

        assert_eq!(values_to_infer[a].inferred_value, Some(6));
        assert_eq!(values_to_infer[b].inferred_value, Some(1));
        assert_eq!(values_to_infer[c].inferred_value, Some(6));
        assert_eq!(values_to_infer[d].inferred_value, None);
    }

    #[test]
    /// Checks that poison values properly propagate
    fn test_poison_edges_affect_inference() {
        /*
              ____        ____
             <    \      <    \
            0 ->?A 1 -> 3 ->?B 4
             <    /P          /P
              \--2           5
        */
        let fanins: [&[FanInOut]; 6] = [
            /*0*/ &[mk_fan(1, -3), mk_fan(2, -10)], // Backwards edges
            /*1*/ &[], // inference_edge(0) for A
            /*2*/ &[mk_poisoned(1)],
            /*3*/ &[mk_fan(1, 0), mk_fan(4, -3)],
            /*4*/ &[], // inference_edge(3) for B
            /*5*/ &[mk_poisoned(4)],
        ];
        let fanins = ListOfLists::from_slice_slice(&fanins);

        let inputs = [];
        let outputs = [];
        let specified_latencies = [];

        let mut values_to_infer = FlatAlloc::new();
        let a = values_to_infer.alloc(ValueToInfer::new((), true));
        let b = values_to_infer.alloc(ValueToInfer::new((), true));

        let inference_edges = vec![
            LatencyInferenceCandidate {
                multiply_var_by: 1,
                from_node: 0,
                to_node: 1,
                offset: 0,
                target_to_infer: a,
            },
            LatencyInferenceCandidate {
                multiply_var_by: 1,
                from_node: 3,
                to_node: 4,
                offset: 0,
                target_to_infer: b,
            },
        ];

        infer_unknown_latency_edges_test_case(
            fanins,
            &inputs,
            &outputs,
            &specified_latencies,
            &inference_edges,
            &mut values_to_infer,
        )
        .unwrap();

        assert_eq!(values_to_infer[a].inferred_value, None);
        assert_eq!(values_to_infer[b].inferred_value, Some(3));
    }

    #[test]
    /// I found an edge case, where the fact that inference edges do not show up as
    /// "real" means two inference attempts may each try to infer a maximal value, not taking into account the other inference edge.
    ///
    /// While both inferences would be correct in a vacuum,
    /// when they happen together it causes the total inferred amount to be greater than the edge capacity.
    fn test_conflicting_inference_ports() {
        /*    _____________________
            i0 --------   -------- 1o
              \        \ /        /
               2 -? 3 - 4 - 5 -? 6
        */
        let fanins: [&[FanInOut]; 7] = [
            /*0*/ &[],
            /*1*/ &[mk_fan(0, 3), mk_fan(6, 0), mk_fan(4, 1)],
            /*2*/ &[mk_fan(0, 0)],
            /*3*/ &[], // inference_edge(2)
            /*4*/ &[mk_fan(3, 0), mk_fan(0, 1)],
            /*5*/ &[mk_fan(4, 0)],
            /*6*/ &[], // inference_edge(5)
        ];
        let fanins = ListOfLists::from_slice_slice(&fanins);

        let inputs = [0];
        let outputs = [1];
        let specified_latencies = [];

        let mut values_to_infer = FlatAlloc::new();
        let a = values_to_infer.alloc(ValueToInfer::new((), true));
        let b = values_to_infer.alloc(ValueToInfer::new((), true));

        let inference_edges = vec![
            LatencyInferenceCandidate {
                multiply_var_by: 1,
                from_node: 2,
                to_node: 3,
                offset: 0,
                target_to_infer: a,
            },
            LatencyInferenceCandidate {
                multiply_var_by: 1,
                from_node: 5,
                to_node: 6,
                offset: 0,
                target_to_infer: b,
            },
        ];

        let err = infer_unknown_latency_edges_test_case(
            fanins,
            &inputs,
            &outputs,
            &specified_latencies,
            &inference_edges,
            &mut values_to_infer,
        );
        let Err(LatencyCountingError::IndeterminablePortLatency { bad_ports: _ }) = err else {
            panic!("{err:?}")
        };
    }

    /*
        ====== From here on it's examples that crashed in practical examples. ======
        ======                These crashes were then fixed                   ======
    */

    #[test]
    fn single_interface_fifo() {
        let fanins: [&[FanInOut]; 10] = [
            /*0*/ &[mk_fan(3, 0), mk_fan(7, 0), mk_fan(2, 0)],
            /*1*/ &[],
            /*2*/ &[],
            /*3*/ &[],
            /*4*/ &[mk_fan(9, 0), mk_fan(1, 0)],
            /*5*/ &[mk_fan(8, 0), mk_fan(1, 0)],
            /*6*/ &[],
            /*7*/ &[],
            /*8*/ &[mk_fan(6, 0), mk_fan(7, 0)],
            /*9*/ &[mk_fan(0, 0), mk_fan(6, 0)],
        ];
        let fanins = ListOfLists::from_slice_slice(&fanins);

        let inputs = [1, 2, 3];
        let outputs = [4, 5];
        let specified_latencies = [];
        let found_latencies =
            solve_latencies_test_case(fanins, &inputs, &outputs, &specified_latencies).unwrap();

        let correct_latencies = [0; 10];
        assert_latencies_match_exactly(&found_latencies, &correct_latencies);
    }

    #[test]
    fn two_interface_fifo() {
        let fanins: [&[FanInOut]; 8] = [
            /*0*/ &[mk_fan(1, 0), mk_fan(7, 0), mk_fan(2, 0)],
            /*1*/ &[],
            /*2*/ &[],
            /*3*/ &[],
            /*4*/ &[mk_fan(3, 0), mk_fan(0, 0), mk_fan(6, 0)],
            /*5*/ &[mk_fan(6, 0), mk_fan(7, 0), mk_fan(3, 0)],
            /*6*/ &[],
            /*7*/ &[],
        ];
        let fanins = ListOfLists::from_slice_slice(&fanins);

        let inputs = [1, 2, 3];
        let outputs = [4, 5];
        let specified_latencies = [SpecifiedLatency {
            node: 1,
            latency: 0,
        }];
        let found_latencies =
            solve_latencies_test_case(fanins, &inputs, &outputs, &specified_latencies).unwrap();

        let correct_latencies = [0; 8];
        assert_latencies_match_exactly(&found_latencies, &correct_latencies);
    }

    #[test]
    fn minimal_two_interface_fifo() {
        let fanins: [&[FanInOut]; 5] = [
            /*0*/ &[],
            /*1*/ &[mk_fan(0, 0), mk_fan(3, 0)],
            /*2*/ &[mk_fan(0, 0), mk_fan(3, 0), mk_fan(4, 0)],
            /*3*/ &[],
            /*4*/ &[],
        ];
        let fanins = ListOfLists::from_slice_slice(&fanins);

        let inputs = [0, 4];
        let outputs = [1, 2];
        let specified_latencies = [SpecifiedLatency {
            node: 4,
            latency: 0,
        }];
        let found_latencies =
            solve_latencies_test_case(fanins, &inputs, &outputs, &specified_latencies).unwrap();

        let correct_latencies = [0; 5];
        assert_latencies_match_exactly(&found_latencies, &correct_latencies);
    }

    #[test]
    fn fifo_use() {
        let fanins: [&[FanInOut]; 10] = [
            /*0*/ &[mk_fan(4, 0)],
            /*1*/ &[mk_fan(5, 0)],
            /*2*/ &[],
            /*3*/ &[mk_fan(2, 0)],
            /*4*/ &[mk_fan(3, 0)],
            /*5*/ &[mk_fan(3, 1)],
            /*6*/ &[mk_fan(9, 0)],
            /*7*/ &[mk_fan(0, 0)],
            /*8*/ &[mk_fan(1, 0)],
            /*9*/ &[mk_fan(7, -2), mk_fan(8, -2)],
        ];
        let fanins = ListOfLists::from_slice_slice(&fanins);

        let inputs = [];
        let outputs = [];
        let specified_latencies = [SpecifiedLatency {
            node: 0,
            latency: 0,
        }];
        let _found_latencies =
            solve_latencies_test_case(fanins, &inputs, &outputs, &specified_latencies).unwrap();
    }

    #[test]
    fn minimal_fifo_use() {
        let fanins: [&[FanInOut]; 4] = [
            /*0*/ &[mk_fan(2, 0)],
            /*1*/ &[mk_fan(2, 0)],
            /*2*/ &[],
            /*3*/ &[mk_fan(1, -1), mk_fan(0, -2)],
        ];
        let fanins = ListOfLists::from_slice_slice(&fanins);

        let inputs = [];
        let outputs = [];
        let specified_latencies = [SpecifiedLatency {
            node: 0,
            latency: 0,
        }];
        let _found_latencies =
            solve_latencies_test_case(fanins, &inputs, &outputs, &specified_latencies).unwrap();
    }

    #[test]
    fn use_infer_me_crash_inference_because_poison() {
        let fanins: [&[FanInOut]; 9] = [
            /*0*/ &[],
            /*1*/ &[],
            /*2*/ &[mk_fan(8, 0)],
            /*3*/ &[],
            /*4*/ &[mk_fan(1, 0), mk_fan(3, 0)],
            /*5*/ &[mk_fan(0, 0), mk_fan(4, 0)],
            /*6*/ &[mk_fan(5, 0)],
            /*7*/ &[mk_fan(1, 0)],
            /*8*/ &[mk_poisoned(7)],
        ];
        let fanins = ListOfLists::from_slice_slice(&fanins);
        let inputs = [0, 1, 8];
        let outputs = [2, 6];
        let specified_latencies = [
            SpecifiedLatency {
                node: 0,
                latency: 0,
            },
            SpecifiedLatency {
                node: 2,
                latency: 3,
            },
        ];
        let fanins =
            fanins.add_extra_fanin_and_specified_latencies(Vec::new(), &specified_latencies);

        let fanouts = fanins.faninout_complement();
        let ports = LatencyCountingPorts::from_inputs_outputs(&inputs, &outputs);
        let mut mem = SolutionMemory::new(fanouts.len());
        let _partial_solutions = solve_port_latencies(&fanouts, &ports, &mut mem).unwrap();
    }

    #[test]
    fn test_cant_infer_fifo() {
        let fanins: [&[FanInOut]; 7] = [
            /*0*/ &[mk_fan(2, 0)],
            /*1*/ &[],
            /*2*/ &[],
            /*3*/ &[mk_fan(1, 5), mk_fan(2, 5)],
            /*4*/ &[],
            /*5*/ &[mk_fan(6, 0), mk_fan(4, 0), mk_fan(2, 0)],
            /*6*/ &[mk_fan(5, 0), mk_fan(3, 0), mk_fan(2, 0)],
        ];
        let fanins = ListOfLists::from_slice_slice(&fanins);
        let inputs = [1];
        let outputs = [0];
        let specified_latencies = [
            SpecifiedLatency {
                node: 0,
                latency: 0,
            },
            SpecifiedLatency {
                node: 1,
                latency: 3,
            },
        ];
        let fanins =
            fanins.add_extra_fanin_and_specified_latencies(Vec::new(), &specified_latencies);

        let fanouts = fanins.faninout_complement();
        let ports = LatencyCountingPorts::from_inputs_outputs(&inputs, &outputs);
        let mut mem = SolutionMemory::new(fanouts.len());
        let _found_latencies = solve_port_latencies(&fanouts, &ports, &mut mem).unwrap();
    }

    #[test]
    fn test_inference_no_partial_solutions_error() {
        let fanins: [&[FanInOut]; 7] = [
            /*0*/ &[mk_fan(2, 0)],
            /*1*/ &[],
            /*2*/ &[],
            /*3*/ &[mk_fan(1, 5), mk_fan(2, 5)],
            /*4*/ &[],
            /*5*/ &[mk_fan(6, 0), mk_fan(4, 0), mk_fan(2, 0)],
            /*6*/ &[mk_fan(5, 0), mk_fan(3, 0), mk_fan(2, 0)],
        ];
        let fanins = ListOfLists::from_slice_slice(&fanins);
        let inputs = [1];
        let outputs = [0];
        let specified_latencies = [SpecifiedLatency {
            node: 0,
            latency: 0,
        }];
        let mut values_to_infer = FlatAlloc::new();
        let latency_0 = values_to_infer.alloc(ValueToInfer::new((), true));
        let inference_edges = vec![
            LatencyInferenceCandidate {
                multiply_var_by: -1,
                from_node: 5,
                to_node: 2,
                offset: 0,
                target_to_infer: latency_0,
            },
            LatencyInferenceCandidate {
                multiply_var_by: -1,
                from_node: 6,
                to_node: 2,
                offset: 0,
                target_to_infer: latency_0,
            },
        ];

        infer_unknown_latency_edges_test_case(
            fanins,
            &inputs,
            &outputs,
            &specified_latencies,
            &inference_edges,
            &mut values_to_infer,
        )
        .unwrap();
    }
}
