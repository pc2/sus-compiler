use std::fmt::Debug;

use crate::config::config;

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
    abs_lat: i64,
    /// Poisoned nodes still keep their abs_lat, so that we can still detect net-positive latency cycles
    poisoned: bool,
    pinned: bool,
}

impl LatencyNode {
    const UNSET: LatencyNode = LatencyNode {
        abs_lat: i64::MIN,
        poisoned: false,
        pinned: false,
    };
    fn new_pinned(abs_lat: i64) -> LatencyNode {
        let result = LatencyNode {
            abs_lat,
            poisoned: false,
            pinned: true,
        };
        assert!(result.is_set());
        result
    }

    fn unwrap(&self) -> i64 {
        assert!(self.is_set());
        assert!(!self.poisoned);
        self.abs_lat // Poison is not allowed
    }
    /// Returns the computed Absolute Latency for this node if it's not [Self::UNSET] and not [Self::POISONED]
    pub fn get_maybe(&self) -> Option<i64> {
        if self.abs_lat == i64::MIN || self.poisoned {
            None
        } else {
            Some(self.abs_lat)
        }
    }
    fn pin(&mut self) {
        assert!(self.is_set());
        assert!(!self.pinned);
        self.pinned = true;
    }
    fn unpin(&mut self) {
        assert!(self.is_set());
        assert!(self.pinned);
        self.pinned = false;
    }
    /// So Poisoned doesn't make a node [Self::UNSET]
    fn is_set(&self) -> bool {
        self.abs_lat != i64::MIN
    }
    /// To be a valid starting point for latency counting, the node must:
    /// - Be set
    /// - Be pinned
    /// - Not be poisoned
    fn assert_is_valid_starting_point(&self) {
        assert!(self.is_set());
        assert!(self.pinned);
        assert!(!self.poisoned);
    }
    fn update_and_pin<const BACKWARDS: bool>(
        &mut self,
        from: LatencyNode,
        delta: Option<i64>,
    ) -> LatencyNodeUpdate {
        let mut did_update = LatencyNodeUpdate::NoChange;
        // Handle poison cases
        if (from.poisoned || delta.is_none()) && !self.poisoned && !self.pinned {
            // !self.pinned, Otherwise, we're not allowed to update pinned values
            self.poisoned = true;
            did_update = LatencyNodeUpdate::Updated;
        }

        // Update node latency
        if let Some(delta) = delta {
            assert!(from.abs_lat != i64::MIN);
            assert!(delta != i64::MIN);

            let new_latency = from.abs_lat - delta;
            let should_update = self.abs_lat == i64::MIN
                || if BACKWARDS {
                    new_latency < self.abs_lat
                } else {
                    new_latency > self.abs_lat
                };
            if should_update {
                if self.pinned {
                    return LatencyNodeUpdate::ErrorPinnedConflict { new_latency };
                } else {
                    self.abs_lat = new_latency;
                    self.pinned = true;
                    did_update = LatencyNodeUpdate::Updated
                }
            }
        }

        did_update
    }
}

impl Debug for LatencyNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if !self.pinned {
            f.write_str("NOT_PINNED ")?; // Pins are very common, so easier to spot the non-pins
        }
        if self.poisoned {
            f.write_str("POISONED")?;
        }
        if self.abs_lat == i64::MIN {
            f.write_str("UNSET")
        } else {
            write!(f, "{}", self.abs_lat)
        }
    }
}

enum LatencyNodeUpdate {
    NoChange,
    Updated,
    ErrorPinnedConflict { new_latency: i64 },
}

fn clear_unpinned_latencies(working_latencies: &mut [LatencyNode]) {
    for l in working_latencies {
        if !l.pinned {
            *l = LatencyNode::UNSET;
        }
    }
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
fn count_latency<'d, const BACKWARDS: bool>(
    working_latencies: &mut [LatencyNode],
    fanouts: &'d ListOfLists<FanInOut>,
    start_node: usize,
    stack: &mut Vec<LatencyStackElem<'d>>,
) -> Result<(), LatencyCountingError> {
    working_latencies[start_node].assert_is_valid_starting_point();

    assert!(stack.is_empty());

    stack.push(LatencyStackElem {
        node_idx: start_node,
        remaining_fanout: fanouts[start_node].iter(),
    });

    while let Some(top) = stack.last_mut() {
        if let Some(&FanInOut {
            to_node,
            delta_latency,
        }) = top.remaining_fanout.next()
        {
            assert!(working_latencies[top.node_idx].is_set());

            match working_latencies[to_node]
                .update_and_pin::<BACKWARDS>(working_latencies[top.node_idx], delta_latency)
            {
                LatencyNodeUpdate::NoChange => {} // Nothing
                LatencyNodeUpdate::Updated => {
                    stack.push(LatencyStackElem {
                        node_idx: to_node,
                        remaining_fanout: fanouts[to_node].iter(),
                    });
                }
                LatencyNodeUpdate::ErrorPinnedConflict { new_latency } => {
                    // Decide if this is a net positive latency cycle or a conflicting specified latencies error
                    if let Some(conflict_begin) =
                        stack.iter().position(|elem| elem.node_idx == to_node)
                    {
                        let mut conflict_path: Vec<SpecifiedLatency> = stack[conflict_begin..]
                            .iter()
                            .map(|elem| SpecifiedLatency {
                                wire: elem.node_idx,
                                latency: working_latencies[elem.node_idx].unwrap(),
                            })
                            .collect();
                        if BACKWARDS {
                            conflict_path.reverse();
                        }
                        return Err(LatencyCountingError::NetPositiveLatencyCycle {
                            conflict_path,
                            net_roundtrip_latency: new_latency
                                - working_latencies[start_node].unwrap(),
                        });
                    } else {
                        assert!(!BACKWARDS, "This should not appear in backwards exploration, because port conflicts should have been found in the forward pass already");

                        let conflict_path = stack
                            .iter()
                            .map(|elem| SpecifiedLatency {
                                wire: elem.node_idx,
                                latency: working_latencies[elem.node_idx].unwrap(),
                            })
                            .chain(std::iter::once(SpecifiedLatency {
                                wire: to_node,
                                latency: new_latency,
                            }))
                            .collect();
                        return Err(LatencyCountingError::ConflictingSpecifiedLatencies {
                            conflict_path,
                        });
                    };
                }
            }
        } else {
            working_latencies[top.node_idx].unpin();
            stack.pop();
        }
    }

    // Repin start node, because we unpinned it when unwinding the stack
    working_latencies[start_node].pin();

    Ok(())
}

fn count_latency_all_in_list<'d, const BACKWARDS: bool>(
    working_latencies: &mut [LatencyNode],
    fanouts: &'d ListOfLists<FanInOut>,
    nodes: &[SpecifiedLatency],
    stack: &mut Vec<LatencyStackElem<'d>>,
) -> Result<(), LatencyCountingError> {
    for start_node in nodes {
        working_latencies[start_node.wire].assert_is_valid_starting_point();
    }

    for start_node in nodes {
        working_latencies[start_node.wire].assert_is_valid_starting_point();
        count_latency::<BACKWARDS>(working_latencies, fanouts, start_node.wire, stack)?;
        working_latencies[start_node.wire].assert_is_valid_starting_point();
    }

    for start_node in nodes {
        working_latencies[start_node.wire].assert_is_valid_starting_point();
    }

    Ok(())
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

/// Finds a port in the given list that has a defined latency, remove it from the ports list and return it
///
/// The returned port has `port.latency_proposal.is_some()`
fn pop_a_port(ports: &mut Vec<PortLatencyCandidate>) -> Option<PortLatencyCandidate> {
    let found_idx = ports
        .iter()
        .position(|port| port.latency_proposal.is_some())?;
    Some(ports.swap_remove(found_idx))
}

/// All elements in latencies must initially be [LatencyNode::UNSET] or pinned known values
fn solve_latencies_for_ports(
    fanins: &ListOfLists<FanInOut>,
    fanouts: &ListOfLists<FanInOut>,
    ports: &LatencyCountingPorts,
    mut specified_latencies: Vec<SpecifiedLatency>,
) -> Result<Vec<LatencyNode>, LatencyCountingError> {
    if config().debug_print_latency_graph {
        print_latency_test_case(fanins, ports, &specified_latencies);
    }

    if fanins.len() == 0 {
        return Ok(Vec::new());
    }

    // The current set of latencies
    let mut working_latencies = vec![LatencyNode::UNSET; fanins.len()];
    // This stack is reused by [count_latency] calls
    let mut stack = Vec::new();

    // If no latencies are given, we have to initialize an arbitrary one ourselves. Prefer input ports over output ports over regular wires
    if specified_latencies.is_empty() {
        let wire = *ports.port_nodes.first().unwrap_or(&0);
        specified_latencies.push(SpecifiedLatency { wire, latency: 0 });
    }

    // Set up the specified latencies
    for spec_lat in &specified_latencies {
        working_latencies[spec_lat.wire] = LatencyNode::new_pinned(spec_lat.latency);
    }

    // This list contains all ports that still need to be placed. This list gathers port assignments as they happen,
    // and reports errors if port conflicts arise
    let mut ports_to_place: Vec<PortLatencyCandidate> = ports
        .port_nodes
        .iter()
        .enumerate()
        .filter_map(|(idx, &wire)| -> Option<PortLatencyCandidate> {
            (!working_latencies[wire].pinned).then_some(PortLatencyCandidate {
                wire,
                latency_proposal: None,
                is_input: idx < ports.outputs_start_at,
            })
        })
        .collect();

    // First forward run from the initial latency assignment to discover other ports
    count_latency_all_in_list::<false>(
        &mut working_latencies,
        fanouts,
        &specified_latencies,
        &mut stack,
    )?;
    inform_all_ports(&mut ports_to_place, &working_latencies)?;
    clear_unpinned_latencies(&mut working_latencies);

    // Then backward run
    count_latency_all_in_list::<true>(
        &mut working_latencies,
        fanins,
        &specified_latencies,
        &mut stack,
    )?;
    inform_all_ports(&mut ports_to_place, &working_latencies)?;
    clear_unpinned_latencies(&mut working_latencies);

    // Finally, we start specifying each unspecified port in turn, and checking for any conflicts with other ports
    while let Some(chosen_port) = pop_a_port(&mut ports_to_place) {
        working_latencies[chosen_port.wire] =
            LatencyNode::new_pinned(chosen_port.latency_proposal.unwrap());

        if chosen_port.is_input {
            count_latency::<false>(
                &mut working_latencies,
                fanouts,
                chosen_port.wire,
                &mut stack,
            )?;
        } else {
            count_latency::<true>(&mut working_latencies, fanins, chosen_port.wire, &mut stack)?;
        }
        inform_all_ports(&mut ports_to_place, &working_latencies)?;
        clear_unpinned_latencies(&mut working_latencies);
    }
    // It may be that some ports are leftover after this while loop. That just means they weren't connected to a port we have seen.
    // TODO multi-cluster ports

    Ok(working_latencies)
}

/// All the ports should have pinned latencies assigned in the working_latencies parameter, and nothing else.
fn fill_in_internal_latencies(
    fanins: &ListOfLists<FanInOut>,
    fanouts: &ListOfLists<FanInOut>,
    working_latencies: &mut [LatencyNode],
) {
    let mut stack = Vec::new();

    // Now that we have all the ports, we can fill in the internal latencies
    for idx in 0..working_latencies.len() {
        if working_latencies[idx].pinned {
            // it's a defined latency!
            count_latency::<false>(working_latencies, fanouts, idx, &mut stack).unwrap();
            // These should have already been caught when exploring the ports
        }
    }

    // First pin all these latencies
    for latency in working_latencies.iter_mut() {
        if latency.is_set() && !latency.poisoned {
            // it's a defined latency!
            if !latency.pinned {
                // Avoid the assert on latency.pin()
                latency.pin();
            }
        }
    }

    // Finally we add in the backwards latencies. TODO maybe be more conservative here?
    for idx in 0..working_latencies.len() {
        if working_latencies[idx].pinned {
            count_latency::<true>(working_latencies, fanins, idx, &mut stack).unwrap();
            // These should have already been caught when exploring the ports
        }
    }
}

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

pub fn solve_latencies(
    fanins: &ListOfLists<FanInOut>,
    ports: &LatencyCountingPorts,
    specified_latencies: Vec<SpecifiedLatency>,
) -> Result<Vec<LatencyNode>, LatencyCountingError> {
    let fanouts = convert_fanin_to_fanout(fanins);

    let mut latency_solution =
        solve_latencies_for_ports(fanins, &fanouts, ports, specified_latencies)?;

    fill_in_internal_latencies(fanins, &fanouts, &mut latency_solution);

    Ok(latency_solution)
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
        fanouts: &ListOfLists<FanInOut>,
        inputs: &[usize],
        outputs: &[usize],
        specified_latencies: Vec<SpecifiedLatency>,
    ) -> Result<Vec<LatencyNode>, LatencyCountingError> {
        let ports = LatencyCountingPorts::from_inputs_outputs(inputs, outputs);

        let mut latency_solution =
            solve_latencies_for_ports(fanins, fanouts, &ports, specified_latencies)?;

        fill_in_internal_latencies(fanins, fanouts, &mut latency_solution);

        Ok(latency_solution)
    }

    fn solve_latencies_infer_ports(
        fanins: &ListOfLists<FanInOut>,
        specified_latencies: Vec<SpecifiedLatency>,
    ) -> Result<Vec<LatencyNode>, LatencyCountingError> {
        let fanouts = convert_fanin_to_fanout(fanins);

        let inputs = infer_ports(fanins);
        let outputs = infer_ports(&fanouts);

        solve_latencies_test_case(fanins, &fanouts, &inputs, &outputs, specified_latencies)
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

        let fanouts = convert_fanin_to_fanout(&fanins);

        let inputs = [0, 4];
        let outputs = [3, 6];

        let found_latencies =
            solve_latencies_test_case(&fanins, &fanouts, &inputs, &outputs, Vec::new()).unwrap();

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

        let fanouts = convert_fanin_to_fanout(&fanins);

        let inputs = [0, 4];
        let outputs = [3, 6];

        let found_latencies = solve_latencies_test_case(
            &fanins,
            &fanouts,
            &inputs,
            &outputs,
            vec![SpecifiedLatency {
                wire: 6,
                latency: 0,
            }],
        )
        .unwrap();

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

        let fanouts = convert_fanin_to_fanout(&fanins);

        let inputs = [0, 4];
        let outputs = [3, 6];

        for starting_node in 0..7 {
            println!("starting_node: {starting_node}");
            if starting_node == 5 {
                let err = solve_latencies_test_case(
                    &fanins,
                    &fanouts,
                    &inputs,
                    &outputs,
                    vec![SpecifiedLatency {
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
                &fanouts,
                &inputs,
                &outputs,
                vec![SpecifiedLatency {
                    wire: starting_node,
                    latency: 0,
                }],
            )
            .unwrap();

            assert_latencies_match(&found_latencies, &correct_latencies);
        }
    }

    #[test]
    fn check_correct_latency_with_superfluous_input() {
        let fanins: [&[FanInOut]; 8] = [
            /*0*/ &[],
            /*1*/ &[mk_fan(0, 0)],
            /*2*/ &[mk_fan(1, 1), mk_fan(5, 1)],
            /*3*/ &[mk_fan(2, 0)],
            /*4*/ &[],
            /*5*/ &[mk_fan(4, 0), mk_fan(1, 1), mk_fan(7, 2)],
            /*6*/ &[mk_fan(5, 0)],
            /*7*/ &[], // superfluous input
        ];
        let fanins = ListOfLists::from_slice_slice(&fanins);

        let correct_latencies = [0, 0, 2, 2, 1, 1, 1, -1];

        let fanouts = convert_fanin_to_fanout(&fanins);

        let inputs = [0, 4];
        let outputs = [3, 6];

        let found_latencies =
            solve_latencies_test_case(&fanins, &fanouts, &inputs, &outputs, Vec::new()).unwrap();

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

        let fanouts = convert_fanin_to_fanout(&fanins);

        let inputs = [0, 4];
        let outputs = [3, 6];

        let found_latencies =
            solve_latencies_test_case(&fanins, &fanouts, &inputs, &outputs, Vec::new()).unwrap();

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

        let should_be_err = solve_latencies_infer_ports(&fanins, Vec::new());

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
                vec![SpecifiedLatency {
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

        let found_latencies = solve_latencies_infer_ports(
            &fanins,
            vec![
                SpecifiedLatency {
                    wire: 0,
                    latency: 0,
                },
                SpecifiedLatency {
                    wire: 4,
                    latency: 2,
                },
            ],
        )
        .unwrap();

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

        let should_be_err = solve_latencies_infer_ports(
            &fanins,
            vec![
                SpecifiedLatency {
                    wire: 0,
                    latency: 0,
                },
                SpecifiedLatency {
                    wire: 3,
                    latency: 1,
                },
            ],
        );

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

        let should_be_err = solve_latencies_infer_ports(
            &fanins,
            vec![
                SpecifiedLatency {
                    wire: 1,
                    latency: 0,
                },
                SpecifiedLatency {
                    wire: 5,
                    latency: 0,
                },
            ],
        );

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

        let should_be_err = solve_latencies_infer_ports(
            &fanins,
            vec![
                SpecifiedLatency {
                    wire: 0,
                    latency: 0,
                },
                SpecifiedLatency {
                    wire: 1,
                    latency: 1,
                },
            ],
        );
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

        let partial_result = solve_latencies_infer_ports(
            &fanins,
            vec![SpecifiedLatency {
                wire: 0,
                latency: 0,
            }],
        )
        .unwrap();

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

        let should_be_err = solve_latencies_infer_ports(&fanins, Vec::new());

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
        let fanouts = convert_fanin_to_fanout(&fanins);

        let found_latencies =
            solve_latencies_test_case(&fanins, &fanouts, &[0, 1], &[3], Vec::new()).unwrap();

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
        let fanouts = convert_fanin_to_fanout(&fanins);

        let found_latencies =
            solve_latencies_test_case(&fanins, &fanouts, &[0], &[2, 3], Vec::new()).unwrap();

        let correct_latencies = [0, 1, 2, 3];
        assert_latencies_match_exactly(&found_latencies, &correct_latencies);
    }

    // From here on it's examples that crashed in practical examples. These crashes were then fixed

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
        let fanouts = convert_fanin_to_fanout(&fanins);
        let inputs = [1, 2, 3];
        let outputs = [4, 5];
        let specified_latencies = vec![];
        let found_latencies =
            solve_latencies_test_case(&fanins, &fanouts, &inputs, &outputs, specified_latencies)
                .unwrap();

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
        let fanouts = convert_fanin_to_fanout(&fanins);
        let inputs = [1, 2, 3];
        let outputs = [4, 5];
        let specified_latencies = vec![SpecifiedLatency {
            wire: 1,
            latency: 0,
        }];
        let found_latencies =
            solve_latencies_test_case(&fanins, &fanouts, &inputs, &outputs, specified_latencies)
                .unwrap();

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
        let fanouts = convert_fanin_to_fanout(&fanins);
        let inputs = [0, 4];
        let outputs = [1, 2];
        let specified_latencies = vec![SpecifiedLatency {
            wire: 4,
            latency: 0,
        }];
        let found_latencies =
            solve_latencies_test_case(&fanins, &fanouts, &inputs, &outputs, specified_latencies)
                .unwrap();

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
        let fanouts = convert_fanin_to_fanout(&fanins);
        let inputs = [];
        let outputs = [];
        let specified_latencies = vec![SpecifiedLatency {
            wire: 0,
            latency: 0,
        }];
        let _found_latencies =
            solve_latencies_test_case(&fanins, &fanouts, &inputs, &outputs, specified_latencies)
                .unwrap();
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
        let fanouts = convert_fanin_to_fanout(&fanins);
        let inputs = [];
        let outputs = [];
        let specified_latencies = vec![SpecifiedLatency {
            wire: 0,
            latency: 0,
        }];
        let _found_latencies =
            solve_latencies_test_case(&fanins, &fanouts, &inputs, &outputs, specified_latencies)
                .unwrap();
    }
}
