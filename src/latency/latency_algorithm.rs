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

use std::iter::zip;

use crate::{
    alloc::FlatAlloc,
    prelude::{LatencyCountInferenceVarID, LatencyCountInferenceVarIDMarker},
};

use super::list_of_lists::ListOfLists;

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
    PartialSolutionMergeConflict {
        bad_nodes: Vec<(usize, i64, i64)>,
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
                            delta_latency: delta_latency.map(|d| -d),
                        },
                    )
                },
            ),
        )
    }
    fn add_extra_fanin_and_specified_latencies(
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

struct LatencyStackElem<'d> {
    node: usize,
    remaining_fanout: std::slice::Iter<'d, FanInOut>,
}

/// The node for the latency-counting graph. See [solve_latencies]
#[derive(Clone, Copy)]
pub struct LatencyNode {
    /// We use [i64::MIN] to represent [LatencyNode::UNSET] out of convenience.
    /// Because the algorithm updates nodes by taking the max of the existing value with the new value,
    /// this makes it simple to update unset nodes the first time
    ///
    /// We use [i64::MAX] to represent [LatencyNode::POISONED] out of convenience too.
    /// Because the algorithm updates nodes by taking the max of the existing value with the new value,
    /// a poisoned node will never be updated
    abs_lat: i64,
    pinned: bool,
}

impl LatencyNode {
    const UNSET: LatencyNode = LatencyNode {
        abs_lat: i64::MIN,
        pinned: false,
    };
    const POISONED: LatencyNode = LatencyNode {
        abs_lat: i64::MAX,
        pinned: false,
    };
    fn new_pinned(abs_lat: i64) -> LatencyNode {
        let result = LatencyNode {
            abs_lat,
            pinned: true,
        };
        assert!(!result.is_unset());
        result
    }
    fn make_initial_latency_solution(
        size: usize,
        specified_latencies: &[SpecifiedLatency],
    ) -> Vec<LatencyNode> {
        let mut result = vec![LatencyNode::UNSET; size];
        for spec in specified_latencies {
            let target_node = &mut result[spec.node];
            assert!(target_node.is_unset());
            target_node.abs_lat = spec.latency;
            target_node.pinned = true;
            assert!(target_node.is_valid_and_pinned());
        }
        result
    }
    fn make_solution_and_count_latencies(
        fanouts: &ListOfLists<FanInOut>,
        specified_latencies: &[SpecifiedLatency],
        backwards: bool,
    ) -> Vec<LatencyNode> {
        let mut latencies =
            LatencyNode::make_initial_latency_solution(fanouts.len(), specified_latencies);

        // Now that we have all the ports, we can fill in the internal latencies
        for seed in specified_latencies {
            assert!(latencies[seed.node].pinned);

            count_latency(&mut latencies, fanouts, seed.node, backwards);
        }

        latencies
    }
    fn make_solution_forwards_then_backwards(
        fanins: &ListOfLists<FanInOut>,
        fanouts: &ListOfLists<FanInOut>,
        specified_latencies: &[SpecifiedLatency],
    ) -> Vec<LatencyNode> {
        let mut latencies =
            LatencyNode::make_solution_and_count_latencies(fanouts, specified_latencies, false);

        // So, we've got all the foward latencies starting from the given nodes.
        // Now we also want to add the backwards latencies, but only those in the fanin of the seeds
        let mut stack: Vec<LatencyStackElem> = specified_latencies
            .iter()
            .map(|s| {
                assert!(latencies[s.node].pinned);
                LatencyStackElem {
                    node: s.node,
                    remaining_fanout: fanins[s.node].iter(),
                }
            })
            .collect();

        while let Some(top) = stack.last_mut() {
            let Some(&FanInOut {
                to_node,
                delta_latency: _,
            }) = top.remaining_fanout.next()
            else {
                // We don't unpin the node we come from, because we want to pin all nodes in the fanin
                stack.pop().unwrap();
                continue;
            };

            let to_lat = &mut latencies[to_node];
            if !to_lat.pinned && !to_lat.is_unset() && !to_lat.is_poisoned() {
                stack.push(LatencyStackElem {
                    node: to_node,
                    remaining_fanout: fanins[to_node].iter(),
                });
                to_lat.pin();
            }
        }

        // Finally we add in the backwards latencies.
        for idx in 0..latencies.len() {
            if latencies[idx].pinned {
                count_latency(&mut latencies, fanins, idx, true);
            }
        }

        latencies
    }

    /// Returns the computed Absolute Latency for this node if it's not [Self::UNSET] and not [Self::POISONED]
    pub fn get_maybe(&self) -> Option<i64> {
        if self.abs_lat == i64::MIN || self.abs_lat == i64::MAX {
            None
        } else {
            Some(self.abs_lat)
        }
    }
    fn pin(&mut self) {
        assert!(!self.is_unset());
        assert!(!self.pinned);
        self.pinned = true;
    }
    fn unpin(&mut self) {
        assert!(!self.is_unset());
        assert!(self.pinned);
        self.pinned = false;
    }
    fn is_unset(&self) -> bool {
        self.abs_lat == i64::MIN
    }
    fn is_poisoned(&self) -> bool {
        self.abs_lat == i64::MAX
    }
    /// To be a valid starting point for latency counting, the node must:
    /// - Be set
    /// - Be pinned
    /// - Not be poisoned
    fn is_valid_and_pinned(&self) -> bool {
        !self.is_unset() && !self.is_poisoned() && self.pinned
    }
    /// Returns `true` if the node was updated
    fn update_and_pin(&mut self, from: LatencyNode, delta: Option<i64>, backwards: bool) -> bool {
        let new_latency = if let (false, Some(delta)) = (from.abs_lat == i64::MAX, delta) {
            from.abs_lat - delta
        } else {
            i64::MAX // Poison
        };

        let new_latency_may_update = if backwards {
            new_latency < self.abs_lat
        } else {
            new_latency > self.abs_lat
        };

        if self.pinned && new_latency == i64::MAX {
            return false; // Only a get-out-of-jail-free card for poison, normal traversal should still hit the assert if it's incorrect
                          // The early exit is only here, because poison will *always* try to overwrite, and we don't want that.
        }

        if (self.abs_lat == i64::MIN || new_latency == i64::MAX || new_latency_may_update)
            && self.abs_lat != i64::MAX
        {
            // Any incompatible latency errors should have been caught by [find_net_positive_latency_cycles_and_incompatible_specified_latencies]
            assert!(!self.pinned);
            self.abs_lat = new_latency;
            self.pinned = true;
            true
        } else {
            false
        }
    }
}

impl std::fmt::Debug for LatencyNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.pinned {
            f.write_str("!")?;
        }
        if self.abs_lat == i64::MIN {
            f.write_str("UNSET")
        } else if self.abs_lat == i64::MAX {
            f.write_str("POISONED")
        } else {
            write!(f, "{}", self.abs_lat)
        }
    }
}

/// Gathers inference info per domain.
///
/// So for each module, the parts of its subdomains must be added
/// to the collector associated with the global domain this domain is connected to
#[derive(Default)]
pub struct PartialSubmoduleInfo {
    pub inference_edges: Vec<LatencyInferenceCandidate>,
    pub extra_fanin: Vec<(usize, FanInOut)>,
}

/// I found that it was far to complex to try to do the whole of latency counting in a single pass of the graph algorithm
/// That's why I check the "bad graph" cases first, throw the necessary errors, and then do any processing assuming no net positive latency cycles, and no incompatible latency specifications
///
/// In this function, we set [LatencyNode::abs_lat] to [i64::MAX] to indicate that we have already visited a specific node.
fn find_net_positive_latency_cycles_and_incompatible_specified_latencies(
    specified_latencies: &[SpecifiedLatency],
    fanouts: &ListOfLists<FanInOut>,
) -> Result<(), LatencyCountingError> {
    fn check_for_errors_from<'d>(
        working_latencies: &mut [LatencyNode],
        fanouts: &'d ListOfLists<FanInOut>,
        stack: &mut Vec<LatencyStackElem<'d>>,
        start_node: &SpecifiedLatency,
    ) -> Result<(), LatencyCountingError> {
        assert!(working_latencies[start_node.node].is_valid_and_pinned());

        stack.push(LatencyStackElem {
            node: start_node.node,
            remaining_fanout: fanouts[start_node.node].iter(),
        });

        while let Some(top) = stack.last_mut() {
            let from = &mut working_latencies[top.node];
            assert!(!from.is_unset());
            assert!(!from.is_poisoned());

            let Some(&FanInOut {
                to_node,
                delta_latency,
            }) = top.remaining_fanout.next()
            else {
                from.unpin();
                stack.pop().unwrap();
                continue;
            };
            let Some(delta) = delta_latency else { continue }; // We can safely ignore poison edges here

            // Update node latency
            let new_latency = from.abs_lat - delta;

            let to = &mut working_latencies[to_node];
            // Always overwrites UNSET, never overwrites POISONED
            if new_latency > to.abs_lat {
                if to.pinned {
                    // Decide if this is a net positive latency cycle or a conflicting specified latencies error
                    return Err(
                        if let Some(conflict_begin) =
                            stack.iter().position(|elem| elem.node == to_node)
                        {
                            LatencyCountingError::NetPositiveLatencyCycle {
                                conflict_path: stack[conflict_begin..]
                                    .iter()
                                    .map(|elem| SpecifiedLatency {
                                        node: elem.node,
                                        latency: working_latencies[elem.node].get_maybe().unwrap(),
                                    })
                                    .collect(),
                                net_roundtrip_latency: new_latency - start_node.latency,
                            }
                        } else {
                            LatencyCountingError::ConflictingSpecifiedLatencies {
                                conflict_path: stack
                                    .iter()
                                    .map(|elem| SpecifiedLatency {
                                        node: elem.node,
                                        latency: working_latencies[elem.node].get_maybe().unwrap(),
                                    })
                                    .chain(std::iter::once(SpecifiedLatency {
                                        node: to_node,
                                        latency: new_latency,
                                    }))
                                    .collect(),
                            }
                        },
                    );
                } else {
                    to.abs_lat = new_latency;
                    to.pinned = true;

                    stack.push(LatencyStackElem {
                        node: to_node,
                        remaining_fanout: fanouts[to_node].iter(),
                    });
                }
            }
        }
        Ok(())
    }

    let mut working_latencies =
        LatencyNode::make_initial_latency_solution(fanouts.len(), specified_latencies);

    // The stack is reused by sequential start_from calls, to save on allocations
    let mut stack = Vec::new();

    // First handle all specified latencies
    for start_node in specified_latencies {
        check_for_errors_from(&mut working_latencies, fanouts, &mut stack, start_node)?;
    }

    // Then fill in all remaining nodes
    let mut cur_start_point = 0;
    loop {
        // First replace all nodes that have been explored by POISONED.
        // POISONED tells our algorithm that this node is not part of a net-positive latency cycle
        // And thus doesn't need to be checked anymore.
        // Because POISONED == i64::MAX, that means the algorithm naturally won't visit these nodes anymore.
        for l in working_latencies.iter_mut() {
            if l.abs_lat != i64::MIN {
                *l = LatencyNode::POISONED;
            }
        }

        let Some(found) = working_latencies[cur_start_point..]
            .iter()
            .position(|n| !n.is_poisoned())
        else {
            break;
        };
        cur_start_point += found;

        let start_node = SpecifiedLatency {
            node: cur_start_point,
            latency: 0,
        };

        let wl = &mut working_latencies[cur_start_point];
        wl.abs_lat = 0;
        wl.pin();

        check_for_errors_from(&mut working_latencies, fanouts, &mut stack, &start_node)?;
    }

    // Okay, no net-positive latency cycles, or conflicting specified latencies

    Ok(())
}

/// Algorithm:
/// Initialize all inputs at latency 0
/// Perform full forward pass, making latencies the maximum of all incoming latencies
/// Then backward pass, moving nodes forward in latency as much as possible.
/// Only moving forward is possible, and only when not confliciting with a later node
///
/// Keep reusing the same stack to reduce memory allocations
///
/// Requires working_latencies[start_node].pinned == true
///
/// Leaves working_latencies[start_node].pinned == true
fn count_latency(
    working_latencies: &mut [LatencyNode],
    fanouts: &ListOfLists<FanInOut>,
    start_node: usize,
    backwards: bool,
) {
    assert!(working_latencies[start_node].is_valid_and_pinned());

    let mut stack = vec![LatencyStackElem {
        node: start_node,
        remaining_fanout: fanouts[start_node].iter(),
    }];

    while let Some(top) = stack.last_mut() {
        let from = &mut working_latencies[top.node];
        assert!(!from.is_unset());
        assert!(from.pinned);
        // Poison edges are allowed now
        // assert!(!from.is_poisoned());

        let Some(&FanInOut {
            to_node,
            delta_latency,
        }) = top.remaining_fanout.next()
        else {
            from.unpin();
            stack.pop().unwrap();
            continue;
        };

        let from = *from; // Break the mutable borrow
        if working_latencies[to_node].update_and_pin(from, delta_latency, backwards) {
            stack.push(LatencyStackElem {
                node: to_node,
                remaining_fanout: fanouts[to_node].iter(),
            });
        }
    }

    // Repin start node, because we unpinned it when unwinding the stack
    working_latencies[start_node].pin();
    assert!(working_latencies[start_node].is_valid_and_pinned());
}

#[derive(Default, Clone)]
pub struct LatencyCountingPorts {
    /// All inputs come first, then all outputs
    port_nodes: Vec<usize>,
    outputs_start_at: usize,
}

impl LatencyCountingPorts {
    pub fn push(&mut self, node: usize, is_input: bool) {
        if is_input {
            self.port_nodes.insert(self.outputs_start_at, node);
            self.outputs_start_at += 1;
        } else {
            self.port_nodes.push(node);
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
                None => result.push(edge.to_node, true),
                Some(false) => {
                    unreachable!("Inference port cannot be both input and output")
                }
                Some(true) => {}
            }
        }
        for edge in inference_edges {
            match std::mem::replace(&mut was_port_seen[edge.from_node], Some(false)) {
                None => result.push(edge.from_node, false),
                Some(true) => {
                    unreachable!("Inference port cannot be both input and output")
                }
                Some(false) => {}
            }
        }
        result
    }
}

/// All ports that still have to be assigned a latency are placed in a list of these
///
/// When a port first finds a valid latency for itself, it adopts this as it's permanent latency
///
/// If it then in the future sees another differing latency assignment for itself, it produces a [LatencyCountingError::IndeterminablePortLatency]
struct PortLatencyCandidate {
    wire: usize,
    latency_proposal: Option<i64>,
    /// We must not use the specified latencies pass-through when the source port itself was not found by the specified latencies themselves.
    /// Othewise we miss out on possibilities of detecting bad ports because it's hidden by the specified latencies.
    found_by_specified_latencies: bool,
    is_input: bool,
}

fn inform_all_ports(
    ports: &mut [PortLatencyCandidate],
    working_latencies: &[LatencyNode],
    found_by_specified_latencies: bool,
) -> Result<(), LatencyCountingError> {
    let mut bad_ports = Vec::new();
    for p in ports {
        // Ports to use can't yet be pinned of course
        debug_assert!(!working_latencies[p.wire].pinned);
        let Some(found_latency) = working_latencies[p.wire].get_maybe() else {
            continue;
        };
        match p.latency_proposal {
            None => p.latency_proposal = Some(found_latency), // First time, set the latency
            Some(latency_proposal) => {
                if found_latency != latency_proposal {
                    // Conflicting latency options for this port
                    bad_ports.push((p.wire, latency_proposal, found_latency));
                }
            }
        }
        p.found_by_specified_latencies |= found_by_specified_latencies;
    }
    if bad_ports.is_empty() {
        Ok(())
    } else {
        Err(LatencyCountingError::IndeterminablePortLatency { bad_ports })
    }
}

/// Constructs a new vector of specified latencies, from the given specified latencies, extended with any ports it could find.
fn solve_latencies_for_ports(
    fanins: &ListOfLists<FanInOut>,
    fanouts: &ListOfLists<FanInOut>,
    ports_to_place: &mut Vec<PortLatencyCandidate>,
    specified_latencies: &[SpecifiedLatency],
) -> Result<Vec<SpecifiedLatency>, LatencyCountingError> {
    // First forward run from the initial latency assignment to discover other ports
    let specified_latencies_forward =
        LatencyNode::make_solution_and_count_latencies(fanouts, specified_latencies, false);
    let specified_latencies_backward =
        LatencyNode::make_solution_and_count_latencies(fanins, specified_latencies, true);

    inform_all_ports(ports_to_place, &specified_latencies_forward, true)?;
    inform_all_ports(ports_to_place, &specified_latencies_backward, true)?;

    /// Finds a port in the given list that has a defined latency, remove it from the ports list and return it
    ///
    /// The returned port has `port.latency_proposal.is_some()`
    fn pop_a_port(ports: &mut Vec<PortLatencyCandidate>) -> Option<PortLatencyCandidate> {
        let found_idx = ports
            .iter()
            .position(|port| port.latency_proposal.is_some())?;
        Some(ports.swap_remove(found_idx))
    }

    let mut result: Vec<SpecifiedLatency> = specified_latencies.to_vec();
    // Finally, we start specifying each unspecified port in turn, and checking for any conflicts with other ports
    while let Some(chosen_port) = pop_a_port(ports_to_place) {
        let new_specified_latency = SpecifiedLatency {
            node: chosen_port.wire,
            latency: chosen_port.latency_proposal.unwrap(),
        };
        result.push(new_specified_latency);

        let (latencies_source, backwards, fans) = if chosen_port.is_input {
            (&specified_latencies_forward, false, fanouts)
        } else {
            (&specified_latencies_backward, true, fanins)
        };

        // We may only involve the specified latencies if the fanout of the port would actually "go through" the specified latency nodes.
        // Otherwise, we can't use them, since that might cause badly specified ports not to be detected
        let mut working_latencies = if chosen_port.found_by_specified_latencies {
            latencies_source.clone()
        } else {
            vec![LatencyNode::UNSET; fanins.len()]
        };

        working_latencies[new_specified_latency.node] =
            LatencyNode::new_pinned(new_specified_latency.latency);

        count_latency(&mut working_latencies, fans, chosen_port.wire, backwards);

        inform_all_ports(ports_to_place, &working_latencies, false)?;
    }

    Ok(result)
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
    values_to_infer: &FlatAlloc<ValueToInfer<ID>, LatencyCountInferenceVarIDMarker>,
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

#[derive(Debug, Clone)]
struct PartialLatencyCountingSolution {
    /// Only the nodes marked by [Self::seeds] may be set. They must be [LatencyNode::is_valid_and_pinned]
    ///
    /// All other nodes must be [LatencyNode::UNSET]
    latencies: Vec<LatencyNode>,
    conflicting_nodes: Vec<(usize, i64)>, // Node, and the desired latency of the conflict
}

impl PartialLatencyCountingSolution {
    fn offset_to_pin_node_to(&mut self, spec_lat: SpecifiedLatency) {
        let existing_latency_of_node = self.latencies[spec_lat.node].get_maybe().unwrap();
        let offset = spec_lat.latency - existing_latency_of_node;
        if offset == 0 {
            return; // Early exit, no change needed
        }

        for lat in &mut self.latencies {
            assert!(!lat.is_poisoned());
            if !lat.is_unset() {
                lat.abs_lat += offset;
            }
        }
        for conflict in &mut self.conflicting_nodes {
            conflict.1 += offset;
        }
    }
}

fn solve_port_latencies(
    fanins: &ListOfLists<FanInOut>,
    fanouts: &ListOfLists<FanInOut>,
    ports: &LatencyCountingPorts,
) -> Result<Vec<Vec<SpecifiedLatency>>, LatencyCountingError> {
    // This list contains all ports that still need to be placed. This list gathers port assignments as they happen,
    // and reports errors if port conflicts arise
    let mut ports_to_place: Vec<PortLatencyCandidate> = ports
        .port_nodes
        .iter()
        .enumerate()
        .map(|(idx, &wire)| PortLatencyCandidate {
            wire,
            latency_proposal: None,
            found_by_specified_latencies: false,
            is_input: idx < ports.outputs_start_at,
        })
        .collect();

    let mut all_partial_solutions = Vec::new();
    while let Some(found_port) = (!ports_to_place.is_empty()).then(|| ports_to_place.remove(0)) {
        all_partial_solutions.push(solve_latencies_for_ports(
            fanins,
            fanouts,
            &mut ports_to_place,
            &[SpecifiedLatency {
                node: found_port.wire,
                latency: 0,
            }],
        )?);
    }
    Ok(all_partial_solutions)
}

/// Solves the whole latency counting problem. No inference
pub fn solve_latencies(
    fanins: ListOfLists<FanInOut>,
    extra_fanin: Vec<(usize, FanInOut)>,
    ports: &LatencyCountingPorts,
    specified_latencies: &[SpecifiedLatency],
) -> Result<Vec<LatencyNode>, LatencyCountingError> {
    let fanouts = fanins.faninout_complement();
    find_net_positive_latency_cycles_and_incompatible_specified_latencies(
        specified_latencies,
        &fanouts,
    )?;

    let fanins = fanins.add_extra_fanin_and_specified_latencies(extra_fanin, specified_latencies);

    // Cannot call config from a test case. See https://users.rust-lang.org/t/cargo-test-name-errors-with-error-invalid-value-name-for-files-file-does-not-exist/125855
    #[cfg(not(test))]
    if crate::config::config().debug_print_latency_graph {
        print_latency_test_case(&fanins, ports, specified_latencies);
    }

    if fanins.len() == 0 {
        return Ok(Vec::new());
    }

    let fanouts = fanins.faninout_complement();
    debug_assert!(!has_poison_edge(&fanins));
    debug_assert!(!has_poison_edge(&fanouts)); // Equivalent

    let mut partial_solutions = solve_port_latencies(&fanins, &fanouts, ports)?;

    if partial_solutions.is_empty() {
        partial_solutions.push(vec![SpecifiedLatency {
            node: 0,
            latency: 0,
        }]);
    }

    let mut partial_solutions: Vec<PartialLatencyCountingSolution> = partial_solutions
        .into_iter()
        .map(|seeds| PartialLatencyCountingSolution {
            latencies: LatencyNode::make_solution_forwards_then_backwards(
                &fanins, &fanouts, &seeds,
            ),
            conflicting_nodes: Vec::new(),
        })
        .collect();

    if partial_solutions.len() >= 2 {
        merge_partial_solutions(&fanins, &mut partial_solutions);
    }

    // Polish solution: if there were no specified latencies, then we make the latency of the first port '0
    // This is to shift the whole solution to one canonical absolute latency. Prefer:
    // - Specified latencies
    // - First port = '0
    // - Node 0 = '0
    let pinned_node = if let Some(spec) = specified_latencies.first() {
        *spec
    } else {
        SpecifiedLatency {
            node: *ports.port_nodes.first().unwrap_or(&0),
            latency: 0,
        }
    };

    let mut was_found = false;
    for psol in &mut partial_solutions {
        if !psol.latencies[pinned_node.node].is_unset() {
            assert!(
                !was_found,
                "The non-merged partial solutions are of course fully disjoint"
            );
            was_found = true;
            psol.offset_to_pin_node_to(pinned_node);
        }
    }
    assert!(was_found);

    let mut solution_iter = partial_solutions.into_iter();

    let first_solution = solution_iter.next().unwrap();

    if solution_iter.next().is_some() {
        // More than one unmerged solution remaining. Must assert that there's missing nodes in the first solution
        assert!(first_solution.latencies.iter().any(|l| l.is_unset()))
    }

    if first_solution.conflicting_nodes.is_empty() {
        Ok(first_solution.latencies)
    } else {
        Err(LatencyCountingError::PartialSolutionMergeConflict {
            bad_nodes: first_solution
                .conflicting_nodes
                .into_iter()
                .map(|(node, desired)| {
                    (
                        node,
                        first_solution.latencies[node].get_maybe().unwrap(),
                        desired,
                    )
                })
                .collect(),
        })
    }
}

fn merge_partial_solutions(
    fanins: &ListOfLists<FanInOut>,
    partial_solutions: &mut Vec<PartialLatencyCountingSolution>,
) {
    for node in 0..fanins.len() {
        let mut connecting_node: Option<(i64, usize)> = None;
        let mut partial_solution_idx = 0;
        while partial_solution_idx < partial_solutions.len() {
            if let Some(from_latency) =
                partial_solutions[partial_solution_idx].latencies[node].get_maybe()
            {
                // Merge this partial solution into the target partial solution
                if let Some((to_latency, target_idx)) = connecting_node {
                    let mut merge_from = partial_solutions.swap_remove(partial_solution_idx);
                    partial_solution_idx -= 1; // Compensate for +1 on next iteration

                    let merge_into = &mut partial_solutions[target_idx]; // Okay, because target_idx will always be smaller than partial_solution_idx

                    merge_from.offset_to_pin_node_to(SpecifiedLatency {
                        node,
                        latency: to_latency,
                    });
                    merge_into
                        .conflicting_nodes
                        .append(&mut merge_from.conflicting_nodes);

                    for (idx, (to, from)) in
                        zip(merge_into.latencies.iter_mut(), merge_from.latencies.iter())
                            .enumerate()
                    {
                        match (to.get_maybe(), from.get_maybe()) {
                            (_, None) => {} // Do nothing
                            (None, Some(from)) => to.abs_lat = from,
                            (Some(to), Some(from)) => {
                                if to != from {
                                    merge_into.conflicting_nodes.push((idx, from));
                                }
                            }
                        }
                    }
                } else {
                    connecting_node = Some((from_latency, partial_solution_idx));
                }
            }
            partial_solution_idx += 1;
        }
    }
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
    pub back_reference: ID,
}

impl<ID> ValueToInfer<ID> {
    pub fn new(back_reference: ID) -> Self {
        Self {
            inferred_value: Some(i64::MAX),
            back_reference,
        }
    }
    pub fn get(&self) -> Option<i64> {
        self.inferred_value
            .and_then(|v| (v != i64::MAX).then_some(v))
    }
    fn apply_candidate(&mut self, candidate_value: i64) {
        if let Some(v) = &mut self.inferred_value {
            *v = i64::min(*v, candidate_value);
        }
    }
    fn spoil(&mut self) {
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
pub fn infer_unknown_latency_edges<ID>(
    fanins: ListOfLists<FanInOut>,
    ports: &LatencyCountingPorts,
    specified_latencies: &[SpecifiedLatency],
    partial_submodule_info: PartialSubmoduleInfo,
    values_to_infer: &mut FlatAlloc<ValueToInfer<ID>, LatencyCountInferenceVarIDMarker>,
) -> Result<(), LatencyCountingError> {
    let fanins = fanins.add_extra_fanin_and_specified_latencies(
        partial_submodule_info.extra_fanin,
        specified_latencies,
    );

    // Cannot call config from a test case. See https://users.rust-lang.org/t/cargo-test-name-errors-with-error-invalid-value-name-for-files-file-does-not-exist/125855
    #[cfg(not(test))]
    if crate::config::config().debug_print_latency_graph {
        print_inference_test_case(
            &fanins,
            ports,
            specified_latencies,
            &partial_submodule_info.inference_edges,
            values_to_infer,
        );
    }

    if fanins.len() == 0 || partial_submodule_info.inference_edges.is_empty() {
        return Ok(()); // Could not infer anything
    }

    let fanouts = fanins.faninout_complement();

    let partial_solutions = solve_port_latencies(&fanins, &fanouts, ports)?;

    let mut new_edges = Vec::new();
    for partial_sol in partial_solutions {
        add_cycle_to_extra_fanin(&partial_sol, &mut new_edges);
    }

    let fanins = fanins.extend_lists_with_new_elements(new_edges);
    let fanouts = fanins.faninout_complement();

    let inference_ports = LatencyCountingPorts::new_from_inference_edges(
        &partial_submodule_info.inference_edges,
        fanins.len(),
    );

    #[cfg(not(test))]
    if crate::config::config().debug_print_latency_graph {
        print_latency_test_case(&fanins, &inference_ports, &[]);
    }

    let inference_port_solutions = solve_port_latencies(&fanins, &fanouts, &inference_ports)?;

    for candidate in &partial_submodule_info.inference_edges {
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
                let candidate_value = (to - from + candidate.offset) / candidate.multiply_var_by;
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
pub fn mk_poisoned(to_node: usize) -> FanInOut {
    FanInOut {
        to_node,
        delta_latency: None,
    }
}

// makes inputs for fanins, outputs for fanouts
#[cfg(test)]
pub fn infer_ports(fanins: &ListOfLists<FanInOut>) -> Vec<usize> {
    fanins
        .iter()
        .enumerate()
        .filter_map(|(idx, v)| v.is_empty().then_some(idx))
        .collect()
}

#[cfg(test)]
impl LatencyCountingPorts {
    pub fn from_inputs_outputs(inputs: &[usize], outputs: &[usize]) -> Self {
        Self {
            port_nodes: inputs.iter().chain(outputs.iter()).cloned().collect(),
            outputs_start_at: inputs.len(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn solve_latencies_test_case(
        fanins: ListOfLists<FanInOut>,
        inputs: &[usize],
        outputs: &[usize],
        specified_latencies: &[SpecifiedLatency],
    ) -> Result<Vec<LatencyNode>, LatencyCountingError> {
        let ports = LatencyCountingPorts::from_inputs_outputs(inputs, outputs);

        solve_latencies(fanins, Vec::new(), &ports, specified_latencies)
    }

    /// Any node with no fanin is an input
    /// Any node with no fanout is an output
    fn solve_latencies_infer_ports(
        fanins: ListOfLists<FanInOut>,
        specified_latencies: &[SpecifiedLatency],
    ) -> Result<Vec<LatencyNode>, LatencyCountingError> {
        let fanouts = fanins.faninout_complement();

        let inputs = infer_ports(&fanins);
        let outputs = infer_ports(&fanouts);

        solve_latencies_test_case(fanins, &inputs, &outputs, specified_latencies)
    }

    #[track_caller]
    fn assert_latencies_match_exactly(found: &[LatencyNode], correct: &[i64]) {
        assert_eq!(found.len(), correct.len());

        assert!(
            std::iter::zip(found.iter(), correct.iter())
                .all(|(x, y)| x.get_maybe().is_some_and(|v| v == *y)),
            "Latencies don't match exactly: {found:?} !=lat= {correct:?}"
        );
    }

    #[track_caller]
    fn assert_latency_nodes_match_exactly(found: &[LatencyNode], correct: &[LatencyNode]) {
        assert_eq!(found.len(), correct.len());

        assert!(
            std::iter::zip(found.iter(), correct.iter()).all(|(x, y)| { x.abs_lat == y.abs_lat }),
            "Latencies don't match exactly: {found:?} !=lat= {correct:?}"
        );
    }

    #[track_caller]
    /// Assert that all found latencies are valid, and that they match the given list
    ///
    /// This means that all the given latencies are "known", and also that
    fn assert_latencies_match(found: &[LatencyNode], correct: &[i64]) {
        assert_eq!(found.len(), correct.len());

        assert!(
            found[0].get_maybe().is_some_and(|first_found_lat| {
                let diff = first_found_lat - correct[0];

                std::iter::zip(found.iter(), correct.iter())
                    .all(|(x, y)| x.get_maybe().is_some_and(|v| v - y == diff))
            }),
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

        assert!(left == right);
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

        let should_be_err = solve_latencies_infer_ports(fanins, &[]);

        assert!(matches!(
            should_be_err,
            Err(LatencyCountingError::IndeterminablePortLatency { bad_ports: _ })
        ))
    }

    #[test]
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
            solve_latencies_infer_ports(
                fanins.clone(),
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

        let found_latencies = solve_latencies_infer_ports(fanins, &specified_latencies).unwrap();

        let correct_latencies = [0, 0, 3, 3, 2, 2, 2];

        assert_latencies_match_exactly(&found_latencies, &correct_latencies); // Can even do a strict check here, because we defined some of the latencies
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
                latency: 0,
            },
            SpecifiedLatency {
                node: 3,
                latency: 1,
            },
        ];

        let should_be_err = solve_latencies_infer_ports(fanins, &specified_latencies);

        println!("{should_be_err:?}");
        let Err(LatencyCountingError::ConflictingSpecifiedLatencies { conflict_path }) =
            should_be_err
        else {
            unreachable!()
        };
        let path_latency =
            conflict_path.last().unwrap().latency - conflict_path.first().unwrap().latency;
        assert_eq!(path_latency, 2);
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
                latency: 0,
            },
            SpecifiedLatency {
                node: 5,
                latency: 0,
            },
        ];

        let should_be_err = solve_latencies_infer_ports(fanins, &specified_latencies);

        let Err(LatencyCountingError::ConflictingSpecifiedLatencies { conflict_path }) =
            should_be_err
        else {
            unreachable!()
        };
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
                latency: 0,
            },
            SpecifiedLatency {
                node: 1,
                latency: 1,
            },
        ];

        let should_be_err = solve_latencies_infer_ports(fanins, &specified_latencies);
        println!("{should_be_err:?}");

        let Err(LatencyCountingError::ConflictingSpecifiedLatencies { conflict_path }) =
            should_be_err
        else {
            unreachable!()
        };
        let path_latency =
            conflict_path.last().unwrap().latency - conflict_path.first().unwrap().latency;
        assert_eq!(path_latency, 2);

        assert_eq!(conflict_path[0].node, 0);
        assert_eq!(conflict_path[1].node, 2);
        assert_eq!(conflict_path[2].node, 1);
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

        let partial_result = solve_latencies_infer_ports(fanins, &specified_latencies).unwrap();

        let correct_latencies = [
            LatencyNode::new_pinned(0), // [solve_latencies] returns pinned values
            LatencyNode::new_pinned(0),
            LatencyNode::new_pinned(3),
            LatencyNode::new_pinned(3),
            LatencyNode::UNSET,
            LatencyNode::UNSET,
            LatencyNode::UNSET,
        ];
        assert_latency_nodes_match_exactly(&partial_result, &correct_latencies)
    }

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
                / \
               6   7
                \ /
            i3 - 4 - 5o
        */
        let fanins: [&[FanInOut]; 8] = [
            /*0*/ &[], // First half
            /*1*/ &[mk_fan(0, 0), mk_fan(6, 0)],
            /*2*/ &[mk_fan(1, 0)],
            /*3*/ &[],
            /*4*/ &[mk_fan(3, 0), mk_fan(6, 3)],
            /*5*/ &[mk_fan(4, 0)],
            /*6*/ &[],
            /*7*/ &[mk_fan(1, 0), mk_fan(4, 2)],
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
    }

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

        let should_be_err = solve_latencies_infer_ports(fanins, &specified_latencies);

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
        let mut partial_solutions = solve_port_latencies(&fanins, &fanouts, &ports).unwrap();

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
        let a = values_to_infer.alloc(ValueToInfer::new(()));
        let b = values_to_infer.alloc(ValueToInfer::new(())); // Shared by two inference candidates
        let c = values_to_infer.alloc(ValueToInfer::new(()));
        let d = values_to_infer.alloc(ValueToInfer::new(())); // Cannot be inferred

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

        let ports = LatencyCountingPorts::from_inputs_outputs(&inputs, &outputs);

        let partial_submodule_info = PartialSubmoduleInfo {
            inference_edges,
            extra_fanin: Vec::new(),
        };

        infer_unknown_latency_edges(
            fanins,
            &ports,
            &specified_latencies,
            partial_submodule_info,
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
        let a = values_to_infer.alloc(ValueToInfer::new(()));
        let b = values_to_infer.alloc(ValueToInfer::new(()));

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

        let ports = LatencyCountingPorts::from_inputs_outputs(&inputs, &outputs);

        let partial_submodule_info = PartialSubmoduleInfo {
            inference_edges,
            extra_fanin: Vec::new(),
        };

        infer_unknown_latency_edges(
            fanins,
            &ports,
            &specified_latencies,
            partial_submodule_info,
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
        let a = values_to_infer.alloc(ValueToInfer::new(()));
        let b = values_to_infer.alloc(ValueToInfer::new(()));

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

        let ports = LatencyCountingPorts::from_inputs_outputs(&inputs, &outputs);
        let partial_submodule_info = PartialSubmoduleInfo {
            inference_edges,
            extra_fanin: Vec::new(),
        };

        let err = infer_unknown_latency_edges(
            fanins,
            &ports,
            &specified_latencies,
            partial_submodule_info,
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
        let _partial_solutions = solve_port_latencies(&fanins, &fanouts, &ports).unwrap();
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
        let _found_latencies = solve_port_latencies(&fanins, &fanouts, &ports).unwrap();
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
        let ports = LatencyCountingPorts::from_inputs_outputs(&[1], &[0]);
        let specified_latencies = [SpecifiedLatency {
            node: 0,
            latency: 0,
        }];
        let mut values_to_infer = FlatAlloc::new();
        let latency_0 = values_to_infer.alloc(ValueToInfer::new(()));
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

        let partial_submodule_info = PartialSubmoduleInfo {
            inference_edges,
            extra_fanin: Vec::new(),
        };

        infer_unknown_latency_edges(
            fanins,
            &ports,
            &specified_latencies,
            partial_submodule_info,
            &mut values_to_infer,
        )
        .unwrap();
    }
}
