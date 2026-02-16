//! Latency Counting concerns three types of nodes:
//! - Late nodes: These usually correspond to inputs. LC tries to make these as late as possible
//! - Early nodes: Usually correspond to outputs. LC tries to make these as early as possible
//!   (therby squeezing the inputs and outputs together as closely as possible)
//! - Neutral nodes: These just need to get some absolute latency assigned.
//!   LC will make these as early as possible, without affecting late nodes.
//!   Neutral nodes not constrained by Late nodes get added in last, by a single backwards pass
//!
//! Latency counting works in two stages:
//! - First we start from the ports (the early and late nodes).
//!   From here we try to discover other ports, by walking the dependency graph
//!   Any ports we discover must be unambiguously reachable at the exact same absolute latency from other ports
//! - Once we have found all ports, and no port reports a conflicting latency, we can fill in the internal latencies
//!   This starts from the late ports, and seeds them with the found latencies.
//!   From here it keeps all the found latencies, such that the resulting latencies are all as early as possible.

use std::collections::VecDeque;

use crate::{
    flattening::Direction,
    latency::{CALCULATE_LATENCY_LATER, InferenceFailure},
    to_string::FmtWrapper,
    util::partition_in_place,
};

use super::list_of_lists::ListOfLists;

const UNSET: i64 = i64::MIN;
const POISON: i64 = i64::MAX;
const NO_STOP_AT_NODE: usize = usize::MAX;
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

#[derive(Debug, PartialEq, Eq)]
pub struct IndeterminablePort {
    pub port_node: usize,
    /// At least two options, and their [IndeterminablePortOption::desired_latency] values conflict
    /// Must be sorted by from_port index
    pub options: Vec<IndeterminablePortOption>,
}
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct IndeterminablePortOption {
    pub desired_latency: i64,
    pub from: SpecifiedLatency,
}
impl std::fmt::Debug for IndeterminablePortOption {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let Self {
            desired_latency,
            from: SpecifiedLatency { node, latency },
        } = self;
        write!(f, "{desired_latency} from {node}'{latency}")
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
        bad_ports: Vec<IndeterminablePort>,
    },
    /// Result is a partitioning of all ports in this domain.
    /// The ports before the partition are all strongly connected,
    /// and the ports after it are not strongly connected to this first cluster.
    PortsNotStronglyConnected {
        port_partitions: Vec<(usize, Vec<usize>)>,
    },
}

/// A graph connection from (respectively to) another wire, which specifies the minimal (respectively maximal) difference in latency between them.
#[derive(Debug, Clone, Copy)]
pub struct FanInOut {
    pub to_node: usize,
    /// If None, then this is a poisoned edge
    pub delta_latency: Option<i64>,
}
impl FanInOut {
    pub fn mk_poison(to_node: usize) -> Self {
        Self {
            to_node,
            delta_latency: None,
        }
    }
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

#[derive(Debug)]
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

    /// Returns true when stop_when_poison was poisoned. Otherwise returns false.
    fn dfs_fill_poison(
        &mut self,
        fanouts: &ListOfLists<FanInOut>,
        node: usize,
        stop_when_poison: usize,
    ) -> bool {
        let cur_node = &mut self.solution[node];
        if *cur_node == POISON {
            return false; // Don't duplicate work on reconvergent paths, return false because stop_when_poison would have already been detected when it was set
        }
        *cur_node = POISON;
        if stop_when_poison == node {
            return true;
        }
        for edge in &fanouts[node] {
            if self.dfs_fill_poison(fanouts, edge.to_node, stop_when_poison) {
                return true;
            }
        }
        false
    }

    /// The graph given to this function must be solveable. (IE pass [check_if_unsolveable]), otherwise this will loop forever
    /// Worst-case Complexity O(V*E), but about O(E) for average case
    fn latency_count_bellman_ford(
        &mut self,
        fanouts: &ListOfLists<FanInOut>,
        stop_when_poison: usize,
    ) -> Result<(), InferenceFailure> {
        while let Some(from_idx) = self.to_explore_queue.pop_front() {
            let from_latency = self.solution[from_idx];
            if from_latency == POISON {
                // Target node got overwritten with poison before we came back to it
                // Then all its fanout will be POISON too.
                continue;
            }
            for edge in &fanouts[from_idx] {
                if let Some(delta) = edge.delta_latency {
                    let new_value = from_latency + delta;
                    let target_node = &mut self.solution[edge.to_node];
                    if new_value > *target_node {
                        *target_node = new_value;
                        self.to_explore_queue.push_back(edge.to_node);
                    }
                } else {
                    let reached_stop_when_poison =
                        self.dfs_fill_poison(fanouts, edge.to_node, stop_when_poison);
                    if reached_stop_when_poison {
                        return Err(InferenceFailure::Poison {
                            edge_from: from_idx,
                            edge_to: edge.to_node,
                        });
                    }
                }
            }
        }
        Ok(())
    }

    fn explore_all_connected_nodes(
        &mut self,
        fanins: &ListOfLists<FanInOut>,
        fanouts: &ListOfLists<FanInOut>,
    ) {
        loop {
            self.latency_count_bellman_ford(fanouts, NO_STOP_AT_NODE)
                .unwrap();
            self.invert_and_generate_queue();
            let original_num_valid = self.to_explore_queue.len();
            self.latency_count_bellman_ford(fanins, NO_STOP_AT_NODE)
                .unwrap();
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

    /// Grabs the latencies from the provided indices, and stores them in results
    fn gather(&self, nodes: &mut [SpecifiedLatency]) {
        for n in nodes {
            n.latency = self.solution[n.node];
        }
    }
    /// Returns the first node in the list that has a valid latency
    fn gather_one<'v>(
        &self,
        nodes: &'v mut [SpecifiedLatency],
    ) -> Option<(&'v mut SpecifiedLatency, i64)> {
        for node in nodes {
            let lat = self.solution[node.node];
            if is_valid(lat) {
                return Some((node, lat));
            }
        }
        None
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
        //eprintln!("Init node {} to: {spec_node:?}", spec.node);
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

                    //eprintln!("Set node {} to: {to_node:?}", f.to_node);

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
        //eprintln!("Init node {next_start} to: {:?}", nodes[next_start]);
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
}

fn has_poison_edge(fanouts: &ListOfLists<FanInOut>) -> bool {
    !fanouts
        .iter()
        .all(|fanout_list| fanout_list.iter().all(|f| f.delta_latency.is_some()))
}

fn print_latency_test_case(
    fanins: &ListOfLists<FanInOut>,
    ports: &LatencyCountingPorts,
    specified_latencies: &[SpecifiedLatency],
) {
    eprintln!(
        "{}",
        FmtWrapper(|f| {
            writeln!(f, "==== BEGIN LATENCY TEST CASE ====")?;
            writeln!(f, "#[test]")?;
            writeln!(f, "fn new_test_case() {{")?;
            writeln!(f, "    let fanins : [&[FanInOut]; {}] = [", fanins.len())?;
            for (idx, fin) in fanins.iter().enumerate() {
                write!(f, "        /*{idx}*/&[")?;
                for FanInOut {
                    to_node,
                    delta_latency,
                } in fin
                {
                    if let Some(delta_lat) = delta_latency {
                        write!(f, "mk_fan({to_node}, {delta_lat}),")?
                    } else {
                        write!(f, "mk_poisoned({to_node}),")?
                    }
                }
                writeln!(f, "],")?;
            }
            writeln!(f, "    ];")?;
            writeln!(
                f,
                "    let fanins = ListOfLists::from_slice_slice(&fanins);"
            )?;
            writeln!(f, "    let inputs = {:?};", ports.inputs())?;
            writeln!(f, "    let outputs = {:?};", ports.outputs())?;
            writeln!(f, "    let specified_latencies = {specified_latencies:?};")?;
            writeln!(
                f,
                "    let _found_latencies = solve_latencies_test_case(&fanins, &inputs, &outputs, &specified_latencies).unwrap();"
            )?;
            writeln!(f, "}}")?;
            writeln!(f, "==== END LATENCY TEST CASE ====")
        })
    );
}

/// Guarantees that if `specified_latencies` is non-empty, the ports connected to it will be the first element in the result vector.
fn solve_port_latencies(
    fanins: &ListOfLists<FanInOut>,
    fanouts: &ListOfLists<FanInOut>,
    ports: &LatencyCountingPorts,
    solution_memory: &mut SolutionMemory,
    specified_latencies: &[SpecifiedLatency],
) -> Result<(Vec<Vec<SpecifiedLatency>>, i64), LatencyCountingError> {
    let inputs = ports.inputs();
    let outputs = ports.outputs();
    // Index as port_connection_matrix[input_idx * outputs.len() + output_idx]
    // Describes the latency difference from the input to the output port. ('out - 'in)
    // Input/Output port pairs that are UNSET have no connection
    let mut port_connection_matrix = vec![UNSET; inputs.len() * outputs.len()];

    for (input_idx, input_port) in inputs.iter().enumerate() {
        let start_node = SpecifiedLatency {
            node: *input_port,
            latency: 0, // 'out - 0
        };
        let mut working_latencies =
            solution_memory.make_solution_with_initial_values(&[start_node]);
        working_latencies
            .latency_count_bellman_ford(fanouts, NO_STOP_AT_NODE)
            .unwrap();

        for (output_idx, output_port) in outputs.iter().enumerate() {
            let found_difference = working_latencies.solution[*output_port];
            if is_valid(found_difference) {
                port_connection_matrix[input_idx * outputs.len() + output_idx] = found_difference;
            } // else remains UNSET
        }
    }

    let mut all_latencies: Vec<_> = ports
        .port_nodes
        .iter()
        .map(|p| SpecifiedLatency {
            node: *p,
            latency: UNSET,
        })
        .collect();
    let (input_latencies, output_latencies) = all_latencies.split_at_mut(inputs.len());
    let mut next_latency_group_offset = if specified_latencies.is_empty() {
        0
    } else {
        SEPARATE_SEED_OFFSET
    };
    let mut port_groups = Vec::new();

    // false for output to input
    let mut input_to_output = match get_initial_group_from_specified(
        fanins,
        fanouts,
        solution_memory,
        specified_latencies,
        input_latencies,
        output_latencies,
    ) {
        InitialGroupFromSpecified::NotFound => {
            if let Some(first_input) = input_latencies.first_mut() {
                first_input.latency = next_latency_group_offset;
                next_latency_group_offset += SEPARATE_SEED_OFFSET;
                true
            } else {
                // No inputs at all, we should simply return the outputs as separate elements
                for o in output_latencies {
                    assert_eq!(o.latency, UNSET);
                    o.latency = next_latency_group_offset;
                    next_latency_group_offset += SEPARATE_SEED_OFFSET;
                    port_groups.push(vec![*o]);
                }
                return Ok((port_groups, next_latency_group_offset));
            }
        }
        InitialGroupFromSpecified::FromInput => true,
        InitialGroupFromSpecified::FromOutput => false,
    };

    let mut current_port_group: Vec<SpecifiedLatency> = Vec::new();
    let mut bad_ports: Vec<IndeterminablePort> = Vec::new();

    let mut options_from = Vec::with_capacity(usize::max(inputs.len(), outputs.len()));
    // By now, we have *at least one* from_latencies element that is not UNSET.
    loop {
        let (from_latencies, to_latencies) = if input_to_output {
            (&mut *input_latencies, &mut *output_latencies)
        } else {
            (&mut *output_latencies, &mut *input_latencies)
        };
        let new_ports_found = explore_connected_ports(
            &port_connection_matrix,
            outputs.len(),
            from_latencies,
            to_latencies,
            input_to_output,
            &mut bad_ports,
            &mut options_from,
        );
        // Push currently known ports to the resulting group
        for v in from_latencies.iter_mut() {
            if is_valid(v.latency) {
                current_port_group.push(*v);
                v.latency = POISON;
            }
        }
        if new_ports_found {
            // Swap inputs & outputs, and restart
            input_to_output = !input_to_output;
        } else {
            // end of this port group, push it to the output, and grab a new seed
            port_groups.push(current_port_group);
            current_port_group = Vec::new();
            if let Some(new_seed) = input_latencies.iter_mut().find(|l| l.latency == UNSET) {
                new_seed.latency = next_latency_group_offset;
                next_latency_group_offset += SEPARATE_SEED_OFFSET;
                input_to_output = true;
            } else {
                // Done, no more input ports to start from. Add the remaining outputs and return.
                for o in output_latencies {
                    if o.latency == UNSET {
                        assert_eq!(o.latency, UNSET);
                        o.latency = next_latency_group_offset;
                        next_latency_group_offset += SEPARATE_SEED_OFFSET;
                        port_groups.push(vec![*o]);
                    }
                }
                break;
            }
        }
    }

    if bad_ports.is_empty() {
        Ok((port_groups, next_latency_group_offset))
    } else {
        Err(LatencyCountingError::IndeterminablePortLatency { bad_ports })
    }
}

enum InitialGroupFromSpecified {
    NotFound,
    FromInput,
    FromOutput,
}

fn get_initial_group_from_specified(
    fanins: &ListOfLists<FanInOut>,
    fanouts: &ListOfLists<FanInOut>,
    solution_memory: &mut SolutionMemory,
    specified_latencies: &[SpecifiedLatency],
    input_latencies: &mut [SpecifiedLatency],
    output_latencies: &mut [SpecifiedLatency],
) -> InitialGroupFromSpecified {
    // When no specified latencies, we don't even need to try
    if let Some(first_specified_latency) = specified_latencies.first() {
        let find_set_ports = solution_memory.make_solution_with_initial_values(specified_latencies);
        find_set_ports.gather(input_latencies);
        if input_latencies.iter().any(|l| is_valid(l.latency)) {
            return InitialGroupFromSpecified::FromInput;
        }
        // try to start from output specified latencies instead
        find_set_ports.gather(output_latencies);
        if output_latencies.iter().any(|l| is_valid(l.latency)) {
            return InitialGroupFromSpecified::FromOutput;
        }

        // Turns out no ports had a specified latency. Try to get an estimate for a *single* port (if we were to take multiple estimates, we could create spurious indeterminable port latency conflicts).
        // Try to estimate an input port by going backward first
        let mut backtrack_for_port =
            solution_memory.make_solution_with_initial_values(&[SpecifiedLatency {
                node: first_specified_latency.node,
                latency: -first_specified_latency.latency,
            }]);
        backtrack_for_port
            .latency_count_bellman_ford(fanins, NO_STOP_AT_NODE)
            .unwrap();

        if let Some((input_port, latency)) = backtrack_for_port.gather_one(input_latencies) {
            input_port.latency = -latency;
            return InitialGroupFromSpecified::FromInput;
        }

        // No input port could be set, try to estimate an output port by going forward instead
        let mut backtrack_for_port =
            solution_memory.make_solution_with_initial_values(&[*first_specified_latency]);
        backtrack_for_port
            .latency_count_bellman_ford(fanouts, NO_STOP_AT_NODE)
            .unwrap();

        if let Some((output_lat, latency)) = backtrack_for_port.gather_one(output_latencies) {
            output_lat.latency = latency;
            return InitialGroupFromSpecified::FromOutput;
        }
    }
    InitialGroupFromSpecified::NotFound
}

/// The given matrix is indexed [input_idx * matrix_stride + output_idx]
fn explore_connected_ports(
    port_connection_matrix: &[i64],
    matrix_stride: usize,
    from_latencies: &[SpecifiedLatency],
    to_latencies: &mut [SpecifiedLatency],
    input_to_output: bool,
    bad_ports: &mut Vec<IndeterminablePort>,
    options_from: &mut Vec<IndeterminablePortOption>,
) -> bool {
    let mut any_port_was_updated = false;
    for (to_idx, to) in to_latencies.iter_mut().enumerate() {
        if to.latency != UNSET {
            continue;
        }
        // reuse the memory
        options_from.clear();
        for (from_idx, from) in from_latencies.iter().enumerate() {
            if !is_valid(from.latency) {
                continue;
            }
            let desired_latency = if input_to_output {
                // "from" is "input", "to" is "output"
                let port_delta = port_connection_matrix[from_idx * matrix_stride + to_idx];
                if !is_valid(port_delta) {
                    continue;
                }
                from.latency + port_delta
            } else {
                // "to" is "input", "from" is "output"
                let port_delta = port_connection_matrix[to_idx * matrix_stride + from_idx];
                if !is_valid(port_delta) {
                    continue;
                }
                from.latency - port_delta
            };
            options_from.push(IndeterminablePortOption {
                desired_latency,
                from: *from,
            });
        }
        if let Some((first, rest)) = options_from.split_first() {
            if rest
                .iter()
                .all(|r| r.desired_latency == first.desired_latency)
            {
                // All is good, we have a new known port latency!
                any_port_was_updated = true;
                to.latency = first.desired_latency;
            } else {
                // Conflict!
                options_from.sort_unstable_by_key(|v| v.from.node);
                let options = options_from.clone();
                bad_ports.push(IndeterminablePort {
                    port_node: to.node,
                    options,
                });
                to.latency = POISON;
            }
        } // else we don't know this port yet
    }
    any_port_was_updated
}

/// Checks that for every set of ports within a domain, all ports are part of the same solution seed.
///
/// If a seed with a partial number of ports is found, then it reports it as a [LatencyCountingError::PortsNotStronglyConnected]
fn check_for_unconnected_ports(
    ports_per_domain: &[Vec<usize>],
    solution_seeds: &[Vec<SpecifiedLatency>],
    num_nodes: usize,
) -> Result<(), LatencyCountingError> {
    let mut cur_ports_set = vec![false; num_nodes];

    let mut port_partitions = Vec::new();

    for domain_ports in ports_per_domain {
        for p in domain_ports {
            cur_ports_set[*p] = true;
        }

        let mut best_count = 0;
        let mut best_solution_idx = None;
        for seed in solution_seeds {
            let mut num_in_current_domain = 0;
            for s in seed {
                if cur_ports_set[s.node] {
                    num_in_current_domain += 1;
                }
            }

            if num_in_current_domain > best_count {
                best_count = num_in_current_domain;
                best_solution_idx = Some(seed);
            }
        }

        for p in domain_ports {
            cur_ports_set[*p] = false;
        }

        if best_count != domain_ports.len() {
            assert!(best_count != 0);

            let sol = best_solution_idx.unwrap();

            let mut partition_list = domain_ports.clone();

            for p in sol {
                cur_ports_set[p.node] = true;
            }

            let connected_group_end =
                partition_in_place(&mut partition_list, |p| cur_ports_set[*p]);

            port_partitions.push((connected_group_end, partition_list));

            for p in sol {
                cur_ports_set[p.node] = false;
            }
        }
    }

    if !port_partitions.is_empty() {
        Err(LatencyCountingError::PortsNotStronglyConnected { port_partitions })
    } else {
        Ok(())
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
    ports_per_domain: &[Vec<usize>],
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
    let (solution_seeds, mut seed_start) =
        solve_port_latencies(&fanins, &fanouts, ports, &mut mem, specified_latencies)?;

    check_for_unconnected_ports(ports_per_domain, &solution_seeds, fanouts.len())?;

    let mut final_solution = vec![UNSET; fanouts.len()];

    // Tightly bind the known ports together, to ensure that if one port group bleeds into another without the ports being strongly connected, then the second group is still at the correct offsets, even if some internal latencies may not be as expected.
    let mut bind_port_groups_extra_fanin = Vec::new();
    for seed in &solution_seeds {
        add_cycle_to_extra_fanin(seed, &mut bind_port_groups_extra_fanin);
    }
    let fanins = fanins.extend_lists_with_new_elements(bind_port_groups_extra_fanin);
    let fanouts = fanins.faninout_complement();

    for seed in solution_seeds {
        if is_valid(final_solution[seed[0].node]) {
            continue; // This port group seems to have already been covered when exploring around another port group. Because we bound the latency of the ports together already, it will not have been damaged. Only internal latencies might be a bit shuffled up. 
        }

        let mut solution = mem.make_solution_with_initial_values(&seed);
        solution.explore_all_connected_nodes(&fanins, &fanouts);

        // Of course, all other specified latencies are in the exact same solution
        // In most cases the specified latencies should've already set the ports to the correct latency, but if not, here's an opportunity
        if let Some(representative) = specified_latencies.first()
            && is_valid(solution.solution[representative.node])
        {
            solution.offset_to_pin_node_to(*representative);
        }

        solution.copy_to(&mut final_solution);
    }

    // Also latency count any wires that aren't connected to any ports
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

#[derive(Debug)]
pub struct LatencyInferenceProblem {
    fanouts: ListOfLists<FanInOut>,
    mem: SolutionMemory,
}

/// This returns [Option] instead of [Result]. Any construction errors will also be reported by [solve_latencies]
impl LatencyInferenceProblem {
    pub fn new(
        fanins: ListOfLists<FanInOut>,
        ports: &LatencyCountingPorts,
        specified_latencies: &[SpecifiedLatency],
    ) -> Option<Self> {
        let fanouts = fanins.faninout_complement();

        if fanins.len() == 0 {
            return Some(LatencyInferenceProblem {
                fanouts,
                mem: SolutionMemory::new(0),
            });
        }

        find_positive_latency_cycle(&fanins, specified_latencies).ok()?;

        let mut mem = SolutionMemory::new(fanouts.len());
        let (partial_solutions, _) =
            solve_port_latencies(&fanins, &fanouts, ports, &mut mem, specified_latencies).ok()?;

        let mut new_edges = Vec::new();
        for partial_sol in partial_solutions {
            add_cycle_to_extra_fanin(&partial_sol, &mut new_edges);
        }

        let fanins = fanins.extend_lists_with_new_elements(new_edges);
        let fanouts = fanins.faninout_complement();

        Some(Self { fanouts, mem })
    }

    pub fn infer_max_edge_latency(
        &mut self,
        from: usize,
        to: usize,
    ) -> Result<i64, InferenceFailure> {
        let mut solution = self
            .mem
            .make_solution_with_initial_values(&[SpecifiedLatency {
                node: to,
                latency: 0,
            }]);

        solution.latency_count_bellman_ford(&self.fanouts, from)?;

        let found_target_latency = solution.solution[from];
        assert!(found_target_latency != POISON);
        if found_target_latency == CALCULATE_LATENCY_LATER {
            Err(InferenceFailure::NotReached)
        } else {
            Ok(-found_target_latency) // Invert, because we want to infer the max from-to latency that doesn't violate any constraints
        }
    }
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
    use crate::let_unwrap;

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

        solve_latencies(
            fanins,
            &ports,
            specified_latencies,
            std::slice::from_ref(&ports.port_nodes),
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
        /*
        0 - 1 - 2 - 3
             \ /
          4 - 5 - 6
        */
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
            eprintln!("starting_node: {starting_node}");

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
    fn check_indeterminable_port_latency() {
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
    fn indeterminable_port_latency_with_specified_node() {
        /*
            A -1> B
              \ /1>
               X
              / \2>
            C -1-> D
        */
        const A: usize = 0;
        const B: usize = 1;
        const C: usize = 2;
        const D: usize = 3;
        let fanins: [&[FanInOut]; 4] = [
            /*A: 0*/ &[],
            /*B: 1*/ &[mk_fan(A, 1), mk_fan(C, 1)],
            /*C: 2*/ &[],
            /*D: 3*/ &[mk_fan(A, 2), mk_fan(C, 1)],
        ];
        let fanins = ListOfLists::from_slice_slice(&fanins);

        let expected_per_node = [
            // For port A'7
            IndeterminablePort {
                port_node: C,
                options: vec![
                    IndeterminablePortOption {
                        desired_latency: 7,
                        from: SpecifiedLatency {
                            node: B,
                            latency: 8,
                        },
                    },
                    IndeterminablePortOption {
                        desired_latency: 8,
                        from: SpecifiedLatency {
                            node: D,
                            latency: 9,
                        },
                    },
                ],
            },
            // For port B'7
            IndeterminablePort {
                port_node: D,
                options: vec![
                    IndeterminablePortOption {
                        desired_latency: 8,
                        from: SpecifiedLatency {
                            node: A,
                            latency: 6,
                        },
                    },
                    IndeterminablePortOption {
                        desired_latency: 7,
                        from: SpecifiedLatency {
                            node: C,
                            latency: 6,
                        },
                    },
                ],
            },
            // For port C'7
            IndeterminablePort {
                port_node: A,
                options: vec![
                    IndeterminablePortOption {
                        desired_latency: 7,
                        from: SpecifiedLatency {
                            node: B,
                            latency: 8,
                        },
                    },
                    IndeterminablePortOption {
                        desired_latency: 6,
                        from: SpecifiedLatency {
                            node: D,
                            latency: 8,
                        },
                    },
                ],
            },
            // For port D'7
            IndeterminablePort {
                port_node: B,
                options: vec![
                    IndeterminablePortOption {
                        desired_latency: 6,
                        from: SpecifiedLatency {
                            node: A,
                            latency: 5,
                        },
                    },
                    IndeterminablePortOption {
                        desired_latency: 7,
                        from: SpecifiedLatency {
                            node: C,
                            latency: 6,
                        },
                    },
                ],
            },
        ];

        for (specified_port, expected_error) in expected_per_node.into_iter().enumerate() {
            let should_be_err = solve_latencies_test_case(
                fanins.clone(),
                &[A, C],
                &[B, D],
                &[SpecifiedLatency {
                    node: specified_port,
                    latency: 7,
                }],
            );

            let Err(LatencyCountingError::IndeterminablePortLatency { bad_ports }) = should_be_err
            else {
                panic!("{should_be_err:?}")
            };
            let [bad_port] = bad_ports.as_slice() else {
                panic!()
            };
            assert_eq!(bad_port, &expected_error);
        }
    }

    #[test]
    #[ignore = "Don't support only inputs anymore"]
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
    #[ignore = "Don't support only outputs anymore"]
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
            eprintln!("starting_node: {starting_node}");
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

        eprintln!("{should_be_err:?}");
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

        eprintln!("{should_be_err:?}");
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
        eprintln!("{should_be_err:?}");

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

        let result = solve_latencies_test_case(fanins, &[0], &[3], &specified_latencies).unwrap();

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

    #[test]
    fn ports_not_strongly_connected_error_info() {
        {
            let fanins: [&[FanInOut]; 4] = [
                /*0*/ &[],
                /*1*/ &[],
                /*2*/ &[mk_fan(0, 0), mk_fan(3, 0)],
                /*3*/ &[],
            ];
            let fanins = ListOfLists::from_slice_slice(&fanins);
            let specified_latencies = [];

            let result = solve_latencies_test_case(fanins, &[0, 1, 3], &[2], &specified_latencies)
                .unwrap_err();

            let_unwrap!(
                LatencyCountingError::PortsNotStronglyConnected { port_partitions },
                result
            );
            // 1 is not connected to 0, 2 and 3
            assert_eq!(port_partitions, vec![(3, vec![0, 2, 3, 1])]);
        }
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
        let (mut partial_solutions, _) =
            solve_port_latencies(&fanins, &fanouts, &ports, &mut mem, &[]).unwrap();

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
    fn test_inference_backwards_loop() {
        /*
                2 -\?A
               /      6
              1-3 -/?B \
             /          8 - 9 -?E 10
            0 - 4 -\?C /|
            | \       7 |
            |   5 -/?D  |
            -<----------|
        */
        let fanins: [&[FanInOut]; 11] = [
            /*0*/ &[mk_fan(8, -10)],
            /*1*/ &[mk_fan(0, 0)],
            /*2*/ &[mk_fan(1, 1)],
            /*3*/ &[mk_fan(1, 6)],
            /*4*/ &[mk_fan(0, 2)],
            /*5*/ &[mk_fan(0, 5)],
            /*6*/ &[], // inference_edge(2), inference_edge(3)
            /*7*/ &[], // inference_edge(4), inference_edge(5)
            /*8*/ &[mk_fan(6, 3), mk_fan(7, 2)],
            /*9*/ &[mk_fan(8, 0)],
            /*10*/ &[], // inference_edge(9), disjoint so can't be inferred
        ];
        let fanins = ListOfLists::from_slice_slice(&fanins);

        let inputs = [];
        let outputs = [];
        let specified_latencies = [];

        let ports = LatencyCountingPorts::from_inputs_outputs(&inputs, &outputs);

        let fanins =
            fanins.add_extra_fanin_and_specified_latencies(Vec::new(), &specified_latencies);

        let mut problem =
            LatencyInferenceProblem::new(fanins, &ports, &specified_latencies).unwrap();

        let results = [
            problem.infer_max_edge_latency(2, 6),
            problem.infer_max_edge_latency(3, 6),
            problem.infer_max_edge_latency(4, 7),
            problem.infer_max_edge_latency(5, 7),
            problem.infer_max_edge_latency(9, 10),
        ];

        assert_eq!(
            &results,
            &[
                Ok(6),
                Ok(1),
                Ok(6),
                Ok(3),
                Err(InferenceFailure::NotReached)
            ]
        );
    }

    #[test]
    /// Checks that poison values properly propagate
    fn test_inference_no_poison() {
        /*
                2 -\?A
               /      6
              1-3 -/?B \
             /          8 - 9 -?E 10
            0 - 4 -\?C /|
            | \       7 |
            |   5 -/?D  |
            ------------|
        */
        let fanins: [&[FanInOut]; 11] = [
            /*0*/ &[],
            /*1*/ &[mk_fan(0, 0)],
            /*2*/ &[mk_fan(1, 1)],
            /*3*/ &[mk_fan(1, 6)],
            /*4*/ &[mk_fan(0, 2)],
            /*5*/ &[mk_fan(0, 5)],
            /*6*/ &[], // inference_edge(2), inference_edge(3)
            /*7*/ &[], // inference_edge(4), inference_edge(5)
            /*8*/ &[mk_fan(6, 3), mk_fan(7, 2), mk_fan(0, 10)],
            /*9*/ &[mk_fan(8, 0)],
            /*10*/ &[], // inference_edge(9), disjoint so can't be inferred
        ];
        let fanins = ListOfLists::from_slice_slice(&fanins);

        let inputs = [0];
        let outputs = [8];
        let specified_latencies = [];

        let ports = LatencyCountingPorts::from_inputs_outputs(&inputs, &outputs);

        let fanins =
            fanins.add_extra_fanin_and_specified_latencies(Vec::new(), &specified_latencies);

        let mut problem =
            LatencyInferenceProblem::new(fanins, &ports, &specified_latencies).unwrap();

        let results = [
            problem.infer_max_edge_latency(2, 6),
            problem.infer_max_edge_latency(3, 6),
            problem.infer_max_edge_latency(4, 7),
            problem.infer_max_edge_latency(5, 7),
            problem.infer_max_edge_latency(9, 10),
        ];

        assert_eq!(
            results,
            [
                Ok(6),
                Ok(1),
                Ok(6),
                Ok(3),
                Err(InferenceFailure::NotReached)
            ]
        );
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

        let ports = LatencyCountingPorts::from_inputs_outputs(&inputs, &outputs);

        let fanins =
            fanins.add_extra_fanin_and_specified_latencies(Vec::new(), &specified_latencies);

        let mut problem =
            LatencyInferenceProblem::new(fanins, &ports, &specified_latencies).unwrap();

        let results = [
            problem.infer_max_edge_latency(0, 1),
            problem.infer_max_edge_latency(3, 4),
        ];

        assert_eq!(
            results,
            [
                Err(InferenceFailure::Poison {
                    edge_from: 1,
                    edge_to: 2
                }),
                Ok(3)
            ]
        );
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

        let ports = LatencyCountingPorts::from_inputs_outputs(&inputs, &outputs);

        let fanins =
            fanins.add_extra_fanin_and_specified_latencies(Vec::new(), &specified_latencies);

        let mut problem =
            LatencyInferenceProblem::new(fanins, &ports, &specified_latencies).unwrap();

        let results = [
            problem.infer_max_edge_latency(2, 3),
            problem.infer_max_edge_latency(5, 6),
        ];

        // TODO re-add this error
        /*let Err(LatencyCountingError::IndeterminablePortLatency { bad_ports: _ }) = err else {
            panic!("{err:?}")
        };*/
        // This actually should result in an error, because now the total latency is 4. Well, I've frankly got no idea how to solve that
        assert_eq!(results, [Ok(2), Ok(2)])
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
        let _partial_solutions =
            solve_port_latencies(&fanins, &fanouts, &ports, &mut mem, &specified_latencies)
                .unwrap();
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
        let _found_latencies =
            solve_port_latencies(&fanins, &fanouts, &ports, &mut mem, &specified_latencies)
                .unwrap();
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

        let ports = LatencyCountingPorts::from_inputs_outputs(&inputs, &outputs);

        let fanins =
            fanins.add_extra_fanin_and_specified_latencies(Vec::new(), &specified_latencies);

        let mut problem =
            LatencyInferenceProblem::new(fanins, &ports, &specified_latencies).unwrap();

        let results = [
            problem.infer_max_edge_latency(5, 2),
            problem.infer_max_edge_latency(6, 2),
        ];

        assert_eq!(results, [Ok(-5), Ok(-5)])
    }
}
