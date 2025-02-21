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

use super::list_of_lists::ListOfLists;

/// A wire for which a latency has been specified.
///
/// Provided as a list to [solve_latencies].
#[derive(Debug, Clone, Copy)]
pub struct SpecifiedLatency {
    pub wire: usize,
    pub latency: i64,
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
}

/// A graph connection from (respectively to) another wire, which specifies the minimal (respectively maximal) difference in latency between them.
#[derive(Debug, Clone, Copy)]
pub struct FanInOut {
    pub to_node: usize,
    /// If None, then this is a poisoned edge
    pub delta_latency: Option<i64>,
}

pub fn convert_fanin_to_fanout(fanins: &ListOfLists<FanInOut>) -> ListOfLists<FanInOut> {
    ListOfLists::from_random_access_iterator(
        fanins.len(),
        fanins.iter_flattened_by_bucket().map(
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

struct LatencyStackElem<'d> {
    node_idx: usize,
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

    fn unwrap(&self) -> i64 {
        assert!(!self.is_unset());
        assert!(!self.is_poisoned());
        self.abs_lat // Poison is not allowed
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
        if !self.pinned {
            f.write_str("NOT_PINNED ")?; // Pins are very common, so easier to spot the non-pins
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

/// I found that it was far to complex to try to do the whole of latency counting in a single pass of the graph algorithm
/// That's why I check the "bad graph" cases first, throw the necessary errors, and then do any processing assuming no net positive latency cycles, and no incompatible latency specifications
///
/// In this function, we set [LatencyNode::abs_lat] to [i64::MAX] to indicate that we have already visited a specific node.
fn find_net_positive_latency_cycles_and_incompatible_specified_latencies(
    specified_latencies: &[SpecifiedLatency],
    fanouts: &ListOfLists<FanInOut>,
) -> Result<(), LatencyCountingError> {
    let mut working_latencies = vec![LatencyNode::UNSET; fanouts.len()];

    // The stack is reused by sequential start_from calls, to save on allocations
    let mut stack = Vec::new();

    for spec_lat in specified_latencies {
        let wl = &mut working_latencies[spec_lat.wire];
        wl.abs_lat = spec_lat.latency;
        wl.pin();
    }

    fn start_from<'d>(
        working_latencies: &mut [LatencyNode],
        fanouts: &'d ListOfLists<FanInOut>,
        stack: &mut Vec<LatencyStackElem<'d>>,
        start_node: &SpecifiedLatency,
    ) -> Result<(), LatencyCountingError> {
        assert!(working_latencies[start_node.wire].is_valid_and_pinned());

        stack.push(LatencyStackElem {
            node_idx: start_node.wire,
            remaining_fanout: fanouts[start_node.wire].iter(),
        });

        while let Some(top) = stack.last_mut() {
            let from = &mut working_latencies[top.node_idx];
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
                            stack.iter().position(|elem| elem.node_idx == to_node)
                        {
                            LatencyCountingError::NetPositiveLatencyCycle {
                                conflict_path: stack[conflict_begin..]
                                    .iter()
                                    .map(|elem| SpecifiedLatency {
                                        wire: elem.node_idx,
                                        latency: working_latencies[elem.node_idx].unwrap(),
                                    })
                                    .collect(),
                                net_roundtrip_latency: new_latency - start_node.latency,
                            }
                        } else {
                            LatencyCountingError::ConflictingSpecifiedLatencies {
                                conflict_path: stack
                                    .iter()
                                    .map(|elem| SpecifiedLatency {
                                        wire: elem.node_idx,
                                        latency: working_latencies[elem.node_idx].unwrap(),
                                    })
                                    .chain(std::iter::once(SpecifiedLatency {
                                        wire: to_node,
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
                        node_idx: to_node,
                        remaining_fanout: fanouts[to_node].iter(),
                    });
                }
            }
        }
        Ok(())
    }

    // First handle all specified latencies
    for start_node in specified_latencies {
        start_from(&mut working_latencies, fanouts, &mut stack, start_node)?;
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
            wire: cur_start_point,
            latency: 0,
        };

        let wl = &mut working_latencies[cur_start_point];
        wl.abs_lat = 0;
        wl.pin();

        start_from(&mut working_latencies, fanouts, &mut stack, &start_node)?;
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
fn count_latency<'d>(
    working_latencies: &mut [LatencyNode],
    fanouts: &'d ListOfLists<FanInOut>,
    start_node: usize,
    stack: &mut Vec<LatencyStackElem<'d>>,
    backwards: bool,
) {
    assert!(working_latencies[start_node].is_valid_and_pinned());

    assert!(stack.is_empty());

    stack.push(LatencyStackElem {
        node_idx: start_node,
        remaining_fanout: fanouts[start_node].iter(),
    });

    while let Some(top) = stack.last_mut() {
        let from = &mut working_latencies[top.node_idx];
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
                node_idx: to_node,
                remaining_fanout: fanouts[to_node].iter(),
            });
        }
    }

    // Repin start node, because we unpinned it when unwinding the stack
    working_latencies[start_node].pin();
    assert!(working_latencies[start_node].is_valid_and_pinned());
}

#[derive(Default)]
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
}

/// All ports that still have to be assigned a latency are placed in a list of these
///
/// When a port first finds a valid latency for itself, it adopts this as it's permanent latency
///
/// If it then in the future sees another differing latency assignment for itself, it produces a [LatencyCountingError::IndeterminablePortLatency]
struct PortLatencyCandidate {
    wire: usize,
    latency_proposal: Option<i64>,
    is_input: bool,
}

fn inform_all_ports(
    ports: &mut [PortLatencyCandidate],
    working_latencies: &[LatencyNode],
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
    }
    if bad_ports.is_empty() {
        Ok(())
    } else {
        Err(LatencyCountingError::IndeterminablePortLatency { bad_ports })
    }
}

fn clear_unpinned_latencies(working_latencies: &mut [LatencyNode]) {
    for l in working_latencies {
        if !l.pinned {
            *l = LatencyNode::UNSET;
        }
    }
}

/// All elements in latencies must initially be [LatencyNode::UNSET] or pinned known values
fn solve_latencies_for_ports(
    fanins: &ListOfLists<FanInOut>,
    fanouts: &ListOfLists<FanInOut>,
    ports_to_place: &mut Vec<PortLatencyCandidate>,
    specified_latencies: &[SpecifiedLatency],
) -> Result<PartialLatencyCountingSolution, LatencyCountingError> {
    let mut seeds: Vec<usize> = specified_latencies.iter().map(|s| s.wire).collect();

    // This stack is reused by [count_latency] calls to reduce memory allocations
    let mut stack = Vec::new();

    let mut working_latencies = vec![LatencyNode::UNSET; fanins.len()];

    // Set up the specified latencies
    for spec_lat in specified_latencies {
        working_latencies[spec_lat.wire] = LatencyNode::new_pinned(spec_lat.latency);
    }

    // First forward run from the initial latency assignment to discover other ports
    for (backwards, fans) in [(false, fanouts), (true, fanins)] {
        for start_node in specified_latencies {
            count_latency(
                &mut working_latencies,
                fans,
                start_node.wire,
                &mut stack,
                backwards,
            );
        }

        inform_all_ports(ports_to_place, &working_latencies)?;
        clear_unpinned_latencies(&mut working_latencies);
    }

    /// Finds a port in the given list that has a defined latency, remove it from the ports list and return it
    ///
    /// The returned port has `port.latency_proposal.is_some()`
    fn pop_a_port(ports: &mut Vec<PortLatencyCandidate>) -> Option<PortLatencyCandidate> {
        let found_idx = ports
            .iter()
            .position(|port| port.latency_proposal.is_some())?;
        Some(ports.swap_remove(found_idx))
    }

    // Finally, we start specifying each unspecified port in turn, and checking for any conflicts with other ports
    while let Some(chosen_port) = pop_a_port(ports_to_place) {
        seeds.push(chosen_port.wire);

        working_latencies[chosen_port.wire] =
            LatencyNode::new_pinned(chosen_port.latency_proposal.unwrap());

        let (backwards, fans) = if chosen_port.is_input {
            (false, fanouts)
        } else {
            (true, fanins)
        };
        count_latency(
            &mut working_latencies,
            fans,
            chosen_port.wire,
            &mut stack,
            backwards,
        );

        inform_all_ports(ports_to_place, &working_latencies)?;
        clear_unpinned_latencies(&mut working_latencies);
    }

    Ok(PartialLatencyCountingSolution {
        latencies: working_latencies,
        seeds,
    })
}

fn has_poison_edge(fanouts: &ListOfLists<FanInOut>) -> bool {
    !fanouts
        .iter()
        .all(|fanout_list| fanout_list.iter().all(|f| f.delta_latency.is_some()))
}

/// All the ports should have pinned latencies assigned in the working_latencies parameter, and nothing else.
fn fill_in_internal_latencies(
    fanins: &ListOfLists<FanInOut>,
    fanouts: &ListOfLists<FanInOut>,
    PartialLatencyCountingSolution { latencies, seeds }: &mut PartialLatencyCountingSolution,
) {
    let mut stack = Vec::new();

    debug_assert!(!has_poison_edge(fanins));
    debug_assert!(!has_poison_edge(fanouts)); // Equivalent

    // Now that we have all the ports, we can fill in the internal latencies
    for &idx in seeds.iter() {
        if latencies[idx].pinned {
            // it's a defined latency!
            count_latency(latencies, fanouts, idx, &mut stack, false);
            // These should have already been caught when exploring the ports
        }
    }

    // So, we've got all the foward latencies starting from the given nodes.
    // Now we also want to add the backwards latencies, but only those in the fanin of the seeds
    stack.extend(seeds.iter().map(|s| {
        assert!(latencies[*s].pinned);
        LatencyStackElem {
            node_idx: *s,
            remaining_fanout: fanins[*s].iter(),
        }
    }));
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
                node_idx: to_node,
                remaining_fanout: fanouts[to_node].iter(),
            });
            to_lat.pin();
        }
    }

    // Finally we add in the backwards latencies.
    for idx in 0..latencies.len() {
        if latencies[idx].pinned {
            count_latency(latencies, fanins, idx, &mut stack, true);
            // These should have already been caught when exploring the ports
        }
    }
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
    println!("    let specified_latencies = vec!{specified_latencies:?};");
    println!("    let _found_latencies = solve_latencies(&fanins, &fanouts, &inputs, &outputs, specified_latencies).unwrap();");
    println!("}}");
    println!("==== END LATENCY TEST CASE ====");
}

struct PartialLatencyCountingSolution {
    /// Only the nodes marked by [Self::seeds] may be set. They must be [LatencyNode::is_valid_and_pinned]
    ///
    /// All other nodes must be [LatencyNode::UNSET]
    latencies: Vec<LatencyNode>,
    seeds: Vec<usize>,
}

fn solve_port_latencies(
    fanins: &ListOfLists<FanInOut>,
    fanouts: &ListOfLists<FanInOut>,
    ports: &LatencyCountingPorts,
    specified_latencies: &[SpecifiedLatency],
) -> Result<Vec<PartialLatencyCountingSolution>, LatencyCountingError> {
    assert!(fanins.len() >= 1);

    find_net_positive_latency_cycles_and_incompatible_specified_latencies(
        specified_latencies,
        fanouts,
    )?;

    // This list contains all ports that still need to be placed. This list gathers port assignments as they happen,
    // and reports errors if port conflicts arise
    let mut ports_to_place: Vec<PortLatencyCandidate> = ports // Only ports that don't already have their latency explicitly specified must of course still be figured out
        .port_nodes
        .iter()
        .enumerate()
        .filter_map(|(idx, &wire)| {
            specified_latencies // Number of specified latencies is tiny. O(N²) is no biggie
                .iter()
                .all(|spec_lat| spec_lat.wire != wire)
                .then_some(PortLatencyCandidate {
                    wire,
                    latency_proposal: None,
                    is_input: idx < ports.outputs_start_at,
                })
        })
        .collect();

    // First the partial solution for the specified latencies
    let mut all_partial_solutions = Vec::new();
    if !specified_latencies.is_empty() {
        all_partial_solutions.push(solve_latencies_for_ports(
            fanins,
            fanouts,
            &mut ports_to_place,
            specified_latencies,
        )?);
    }

    // Then we go through the remaining ports to find other partial solutions
    while let Some(found_port) = (!ports_to_place.is_empty()).then(|| ports_to_place.remove(0)) {
        all_partial_solutions.push(solve_latencies_for_ports(
            fanins,
            fanouts,
            &mut ports_to_place,
            &[SpecifiedLatency {
                wire: found_port.wire,
                latency: 0,
            }],
        )?);
    }

    Ok(all_partial_solutions)
}

/// Solves the whole latency counting problem. No inference
pub fn solve_latencies(
    fanins: &ListOfLists<FanInOut>,
    ports: &LatencyCountingPorts,
    specified_latencies: &[SpecifiedLatency],
) -> Result<Vec<LatencyNode>, LatencyCountingError> {
    // Cannot call config from a test case. See https://users.rust-lang.org/t/cargo-test-name-errors-with-error-invalid-value-name-for-files-file-does-not-exist/125855
    #[cfg(not(test))]
    if crate::config::config().debug_print_latency_graph {
        print_latency_test_case(fanins, ports, specified_latencies);
    }

    if fanins.len() == 0 {
        return Ok(Vec::new());
    }

    let fanouts = convert_fanin_to_fanout(fanins);

    let mut partial_solutions = solve_port_latencies(fanins, &fanouts, ports, specified_latencies)?;

    for partial_solution in &mut partial_solutions {
        fill_in_internal_latencies(fanins, &fanouts, partial_solution);
    }

    // Get first solution TEMPORARY
    Ok(partial_solutions.into_iter().next().unwrap().latencies)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn mk_fan(to_node: usize, delta_latency: i64) -> FanInOut {
        FanInOut {
            to_node,
            delta_latency: Some(delta_latency),
        }
    }

    fn mk_poisoned(to_node: usize) -> FanInOut {
        FanInOut {
            to_node,
            delta_latency: None,
        }
    }

    // makes inputs for fanins, outputs for fanouts
    fn infer_ports(fanins: &ListOfLists<FanInOut>) -> Vec<usize> {
        fanins
            .iter()
            .enumerate()
            .filter_map(|(idx, v)| v.is_empty().then_some(idx))
            .collect()
    }

    impl LatencyCountingPorts {
        fn from_inputs_outputs(inputs: &[usize], outputs: &[usize]) -> Self {
            Self {
                port_nodes: inputs.iter().chain(outputs.iter()).cloned().collect(),
                outputs_start_at: inputs.len(),
            }
        }
    }

    fn solve_latencies_test_case(
        fanins: &ListOfLists<FanInOut>,
        inputs: &[usize],
        outputs: &[usize],
        specified_latencies: &[SpecifiedLatency],
    ) -> Result<Vec<LatencyNode>, LatencyCountingError> {
        let ports = LatencyCountingPorts::from_inputs_outputs(inputs, outputs);

        solve_latencies(fanins, &ports, specified_latencies)
    }

    /// Any node with no fanin is an input
    /// Any node with no fanout is an output
    fn solve_latencies_infer_ports(
        fanins: &ListOfLists<FanInOut>,
        specified_latencies: &[SpecifiedLatency],
    ) -> Result<Vec<LatencyNode>, LatencyCountingError> {
        let fanouts = convert_fanin_to_fanout(fanins);

        let inputs = infer_ports(fanins);
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
            solve_latencies_test_case(&fanins, &inputs, &outputs, &specified_latencies).unwrap();

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
            wire: 6,
            latency: 0,
        }];

        let found_latencies =
            solve_latencies_test_case(&fanins, &inputs, &outputs, &specified_latencies).unwrap();

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
            if starting_node == 5 {
                let err = solve_latencies_test_case(
                    &fanins,
                    &inputs,
                    &outputs,
                    &[SpecifiedLatency {
                        wire: starting_node,
                        latency: 0,
                    }],
                );
                let Err(LatencyCountingError::IndeterminablePortLatency { bad_ports: _ }) = err
                else {
                    unreachable!("{err:?}")
                };
                continue;
            }
            let found_latencies = solve_latencies_test_case(
                &fanins,
                &inputs,
                &outputs,
                &[SpecifiedLatency {
                    wire: starting_node,
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
            solve_latencies_test_case(&fanins, &inputs, &outputs, &specified_latencies).unwrap();

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
            solve_latencies_test_case(&fanins, &inputs, &outputs, &specified_latencies).unwrap();

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

        let should_be_err = solve_latencies_infer_ports(&fanins, &[]);

        assert!(matches!(
            should_be_err,
            Err(LatencyCountingError::IndeterminablePortLatency { bad_ports: _ })
        ))
    }

    #[test]
    fn check_conflicting_port_latency_with_any_starting_node_does_error() {
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
                &fanins,
                &[SpecifiedLatency {
                    wire: starting_node,
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
                wire: 0,
                latency: 0,
            },
            SpecifiedLatency {
                wire: 4,
                latency: 2,
            },
        ];

        let found_latencies = solve_latencies_infer_ports(&fanins, &specified_latencies).unwrap();

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
                wire: 0,
                latency: 0,
            },
            SpecifiedLatency {
                wire: 3,
                latency: 1,
            },
        ];

        let should_be_err = solve_latencies_infer_ports(&fanins, &specified_latencies);

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
                wire: 1,
                latency: 0,
            },
            SpecifiedLatency {
                wire: 5,
                latency: 0,
            },
        ];

        let should_be_err = solve_latencies_infer_ports(&fanins, &specified_latencies);

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
                wire: 0,
                latency: 0,
            },
            SpecifiedLatency {
                wire: 1,
                latency: 1,
            },
        ];

        let should_be_err = solve_latencies_infer_ports(&fanins, &specified_latencies);
        println!("{should_be_err:?}");

        let Err(LatencyCountingError::ConflictingSpecifiedLatencies { conflict_path }) =
            should_be_err
        else {
            unreachable!()
        };
        let path_latency =
            conflict_path.last().unwrap().latency - conflict_path.first().unwrap().latency;
        assert_eq!(path_latency, 2);

        assert_eq!(conflict_path[0].wire, 0);
        assert_eq!(conflict_path[1].wire, 2);
        assert_eq!(conflict_path[2].wire, 1);
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
            wire: 0,
            latency: 0,
        }];

        let partial_result = solve_latencies_infer_ports(&fanins, &specified_latencies).unwrap();

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

        let should_be_err = solve_latencies_infer_ports(&fanins, &specified_latencies);

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
            solve_latencies_test_case(&fanins, &[0, 1], &[3], &specified_latencies).unwrap();

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
            solve_latencies_test_case(&fanins, &[0], &[2, 3], &specified_latencies).unwrap();

        let correct_latencies = [0, 1, 2, 3];
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
        let specified_latencies = [SpecifiedLatency {
            wire: 0,
            latency: 0,
        }];

        let fanouts = convert_fanin_to_fanout(&fanins);

        let ports = LatencyCountingPorts::from_inputs_outputs(&inputs, &outputs);
        let found_latencies =
            solve_port_latencies(&fanins, &fanouts, &ports, &specified_latencies).unwrap();

        assert!(found_latencies[0].latencies[0].get_maybe() == Some(0));
        assert!(found_latencies[0].latencies[1].get_maybe().is_none());
        assert!(found_latencies[0].latencies[6].get_maybe() == Some(9));
        assert!(found_latencies[1].latencies[0].get_maybe().is_none());
        assert!(found_latencies[1].latencies[1].get_maybe() == Some(0));
        assert!(found_latencies[1].latencies[6].get_maybe().is_none());
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
            solve_latencies_test_case(&fanins, &inputs, &outputs, &specified_latencies).unwrap();

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
            wire: 1,
            latency: 0,
        }];
        let found_latencies =
            solve_latencies_test_case(&fanins, &inputs, &outputs, &specified_latencies).unwrap();

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
            wire: 4,
            latency: 0,
        }];
        let found_latencies =
            solve_latencies_test_case(&fanins, &inputs, &outputs, &specified_latencies).unwrap();

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
            wire: 0,
            latency: 0,
        }];
        let _found_latencies =
            solve_latencies_test_case(&fanins, &inputs, &outputs, &specified_latencies).unwrap();
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
            wire: 0,
            latency: 0,
        }];
        let _found_latencies =
            solve_latencies_test_case(&fanins, &inputs, &outputs, &specified_latencies).unwrap();
    }
}
