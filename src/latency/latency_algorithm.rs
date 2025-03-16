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

use std::{
    collections::VecDeque,
    io::{stdout, Write},
    iter::zip,
    ops::{Index, IndexMut},
};

use bitvector::BitVector;

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
    fn get_latency(list: &[SpecifiedLatency], node: usize) -> Option<i64> {
        for elem in list {
            if elem.node == node {
                return Some(elem.latency);
            }
        }
        None
    }
    fn has_duplicates(mut list: &[SpecifiedLatency]) -> bool {
        while let Some((fst, rest)) = list.split_first() {
            if rest.iter().any(|e| e.node == fst.node) {
                return true;
            }
            list = rest;
        }
        false
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

#[derive(Debug, Clone)]
pub struct Solution(Vec<LatencyNode>);

impl Solution {
    fn from_specified(size: usize, specified_latencies: &[SpecifiedLatency]) -> Self {
        let mut latencies = vec![LatencyNode::UNSET; size];
        for spec in specified_latencies {
            let target_node = &mut latencies[spec.node];
            assert!(target_node.is_unset(), "Duplicate Specified Latency");
            target_node.abs_lat = spec.latency;
            target_node.pinned = true;
            assert!(target_node.is_valid_and_pinned());
        }
        Self(latencies)
    }
    fn from_specified_and_fanouts(
        fanouts: &ListOfLists<FanInOut>,
        specified_latencies: &[SpecifiedLatency],
    ) -> Self {
        let mut result = Self::from_specified(fanouts.len(), specified_latencies);

        // Now that we have all the ports, we can fill in the internal latencies
        for seed in specified_latencies {
            assert!(result[seed.node].pinned);

            result.count_latency(fanouts, seed.node, false);
        }

        result
    }

    /// Creates a partial solution, where all nodes in the fanin of the ports are set.
    ///
    /// Latencies are defined to first be minimal from the fanout of the (input) ports,
    /// and then the nodes in the fanin of the (output) ports are as late as possible while respecting the existing nodes.
    fn make_initial_partial_solution(
        fanins: &ListOfLists<FanInOut>,
        fanouts: &ListOfLists<FanInOut>,
        specified_latencies: &[SpecifiedLatency],
    ) -> Self {
        let mut result = Self::from_specified_and_fanouts(fanouts, specified_latencies);

        result.pin_all_from(fanins, specified_latencies);

        result.remove_non_pinned_latencies();

        result.count_latencies_from_all_pinned_latencies(fanins, true);

        result
    }

    fn pin_all_from(
        &mut self,
        fanouts: &ListOfLists<FanInOut>,
        specified_latencies: &[SpecifiedLatency],
    ) {
        // So, we've got all the foward latencies starting from the given nodes.
        // Now we also want to add the backwards latencies, but only those in the fanin of the seeds
        let mut stack: Vec<LatencyStackElem> = specified_latencies
            .iter()
            .map(|s| {
                assert!(self[s.node].pinned);
                LatencyStackElem {
                    node: s.node,
                    remaining_fanout: fanouts[s.node].iter(),
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

            let to_lat = &mut self[to_node];
            if !to_lat.pinned && !to_lat.is_unset() && !to_lat.is_poisoned() {
                stack.push(LatencyStackElem {
                    node: to_node,
                    remaining_fanout: fanouts[to_node].iter(),
                });
                to_lat.pin();
            }
        }
    }

    fn pin_all_valid(&mut self) {
        for l in self.iter_mut() {
            l.pinned = !l.is_poisoned() && !l.is_unset();
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
    fn count_latency(
        &mut self,
        fanouts: &ListOfLists<FanInOut>,
        start_node: usize,
        backwards: bool,
    ) -> bool /*Was updated*/ {
        assert!(self[start_node].is_valid_and_pinned());

        let mut stack = vec![LatencyStackElem {
            node: start_node,
            remaining_fanout: fanouts[start_node].iter(),
        }];

        let mut was_updated = false;

        while let Some(top) = stack.last_mut() {
            let from = &mut self[top.node];
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
            if self[to_node].update_and_pin(from, delta_latency, backwards) {
                stack.push(LatencyStackElem {
                    node: to_node,
                    remaining_fanout: fanouts[to_node].iter(),
                });
                was_updated = true;
            }
        }

        // Repin start node, because we unpinned it when unwinding the stack
        self[start_node].pin();
        assert!(self[start_node].is_valid_and_pinned());

        was_updated
    }

    fn remove_non_pinned_latencies(&mut self) {
        for l in self.iter_mut() {
            if !l.pinned {
                *l = LatencyNode::UNSET;
            }
        }
    }

    fn count_latencies_from_all_pinned_latencies(
        &mut self,
        fanouts: &ListOfLists<FanInOut>,
        backwards: bool,
    ) -> bool /*Was updated*/ {
        let mut was_updated = false;
        for idx in 0..self.len() {
            if self[idx].pinned {
                was_updated |= self.count_latency(fanouts, idx, backwards);
            }
        }
        was_updated
    }
}

/// Vec<> equivalent methods
impl Solution {
    fn len(&self) -> usize {
        self.0.len()
    }
    pub fn iter(&self) -> std::slice::Iter<'_, LatencyNode> {
        self.0.iter()
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<'_, LatencyNode> {
        self.0.iter_mut()
    }
}

impl Index<usize> for Solution {
    type Output = LatencyNode;

    fn index(&self, index: usize) -> &LatencyNode {
        &self.0[index]
    }
}

impl IndexMut<usize> for Solution {
    fn index_mut(&mut self, index: usize) -> &mut LatencyNode {
        &mut self.0[index]
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
    impl Solution {
        fn check_for_errors_from(
            &mut self,
            fanouts: &ListOfLists<FanInOut>,
            start_node: &SpecifiedLatency,
        ) -> Result<(), LatencyCountingError> {
            assert!(self[start_node.node].is_valid_and_pinned());

            let mut stack = vec![LatencyStackElem {
                node: start_node.node,
                remaining_fanout: fanouts[start_node.node].iter(),
            }];

            while let Some(top) = stack.last_mut() {
                let from = &mut self[top.node];
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

                let to = &mut self[to_node];
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
                                            latency: self[elem.node].get_maybe().unwrap(),
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
                                            latency: self[elem.node].get_maybe().unwrap(),
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
    }

    let mut working_latencies = Solution::from_specified(fanouts.len(), specified_latencies);

    // First handle all specified latencies
    for start_node in specified_latencies {
        working_latencies.check_for_errors_from(fanouts, start_node)?;
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

        let Some(found) = working_latencies
            .iter()
            .skip(cur_start_point)
            .position(|n| !n.is_poisoned())
        else {
            break;
        };
        cur_start_point += found;

        let start_node = SpecifiedLatency {
            node: cur_start_point,
            latency: 0,
        };

        working_latencies[cur_start_point] = LatencyNode {
            abs_lat: 0,
            pinned: true,
        };

        working_latencies.check_for_errors_from(fanouts, &start_node)?;
    }

    // Okay, no net-positive latency cycles, or conflicting specified latencies

    Ok(())
}

/// Check if the graph contains any cycles or incompatible specified latencies.
fn find_positive_latency_cycle(
    fanouts: &ListOfLists<FanInOut>,
    specified_latencies: &[SpecifiedLatency],
) -> Result<(), LatencyCountingError> {
    #[derive(Clone, Copy)]
    struct BellmannFordNode {
        value: i64,
        parent: Option<usize>,
    }

    dbg!(fanouts);
    let mut nodes = vec![
        BellmannFordNode {
            parent: None,
            value: i64::MIN,
        };
        fanouts.len()
    ];

    /// Returns Some(a_node_in_cycle) if here is a cycle.
    fn check_parent_cycle(nodes: &[BellmannFordNode], start_from: usize) -> Option<usize> {
        let mut fast_ptr = start_from;
        let mut slow_ptr = start_from;

        loop {
            fast_ptr = nodes[fast_ptr].parent?;
            fast_ptr = nodes[fast_ptr].parent?;
            slow_ptr = nodes[slow_ptr].parent.unwrap();

            if fast_ptr == slow_ptr {
                return Some(fast_ptr);
            }
        }
    }

    fn collect_cycle_or_invalid_specified_latency_error(
        nodes: &[BellmannFordNode],
        fanouts: &ListOfLists<FanInOut>,
        start_from: usize,
        specified_latencies: &[SpecifiedLatency],
    ) -> LatencyCountingError {
        let mut conflict_path = Vec::new();

        let mut cur_idx = start_from;
        let mut is_bad_cycle = false;
        conflict_path.push(SpecifiedLatency {
            node: cur_idx,
            latency: i64::MIN,
        });
        while let Some(parent) = nodes[cur_idx].parent {
            conflict_path.push(SpecifiedLatency {
                node: parent,
                latency: i64::MIN,
            });
            if parent == start_from {
                is_bad_cycle = true;
                break;
            }
            cur_idx = parent;
        }
        conflict_path.reverse();
        let first_node = conflict_path.first_mut().unwrap();
        first_node.latency = if is_bad_cycle {
            0
        } else {
            SpecifiedLatency::get_from_specify_list(specified_latencies, first_node.node).unwrap()
        };

        let mut cur_node = *first_node;

        for next_node in &mut conflict_path[1..] {
            let mut max_delta = i64::MIN;
            for fout in &fanouts[cur_node.node] {
                if fout.to_node == next_node.node {
                    max_delta = i64::max(-fout.delta_latency.unwrap(), max_delta);
                }
            }
            assert!(max_delta != i64::MIN);

            next_node.latency = cur_node.latency + max_delta;
            cur_node = *next_node;
        }

        assert!(!conflict_path.is_empty());
        if is_bad_cycle {
            let SpecifiedLatency {
                node,
                latency: net_roundtrip_latency,
            } = conflict_path.pop().unwrap();
            assert_eq!(conflict_path.first().unwrap().node, node);

            assert!(net_roundtrip_latency > 0);
            LatencyCountingError::NetPositiveLatencyCycle {
                conflict_path,
                net_roundtrip_latency,
            }
        } else {
            LatencyCountingError::ConflictingSpecifiedLatencies { conflict_path }
        }
    }

    let mut is_specified = BitVector::new(fanouts.len());

    for spec in specified_latencies {
        is_specified.insert(spec.node);
        nodes[spec.node].value = spec.latency;
    }
    let mut from_list = is_specified.clone();
    let mut to_list = BitVector::new(fanouts.len());
    let mut ever_seen = BitVector::new(fanouts.len());

    let mut start_idx_iter = 0..fanouts.len();
    loop {
        // Used to bound the Bellmann-Ford algorithm more closely.
        let mut early_exit_squash_to_constant_time = 0;

        // Early exit
        while !from_list.is_empty() {
            for from_idx in from_list.iter() {
                let from_node_copy = nodes[from_idx];
                for edge in &fanouts[from_idx] {
                    let Some(delta) = edge.delta_latency else {
                        continue; // Ignore poison edges for this. Because their value is unknown, we can only work with the known edges
                    };
                    let new_value = from_node_copy.value - delta;
                    let target_node = &mut nodes[edge.to_node];
                    if new_value > target_node.value {
                        target_node.value = new_value;
                        target_node.parent = Some(from_idx);
                        to_list.insert(edge.to_node);

                        // Checks when we try to update a node, to see if we detect a positive latency cycle, or a incompatible specified latencies error.
                        if is_specified.contains(edge.to_node) {
                            // Can't blame a net-positive latency cycle, so it has to be conflicting specified latencies.
                            return Err(collect_cycle_or_invalid_specified_latency_error(
                                &nodes,
                                fanouts,
                                edge.to_node,
                                specified_latencies,
                            ));
                        }

                        // Only do the "are we in an infinite cycle" check once every |N| cycles, to amortize the complexity
                        early_exit_squash_to_constant_time += 1;
                        if early_exit_squash_to_constant_time >= fanouts.len() {
                            early_exit_squash_to_constant_time = 0;

                            // O(E*N) termination condition. (See Definition of Bellmann-Ford)
                            if let Some(cycle_start_node) = check_parent_cycle(&nodes, edge.to_node)
                            {
                                return Err(collect_cycle_or_invalid_specified_latency_error(
                                    &nodes,
                                    fanouts,
                                    cycle_start_node,
                                    specified_latencies,
                                ));
                            };
                        }
                    }
                }
            }

            ever_seen.union_inplace(&to_list);

            // Reset all target nodes
            std::mem::swap(&mut from_list, &mut to_list);
            to_list.clear();
        }

        // Clear all nodes that were reached, and set them to MAX, such that we never try to update them again.
        for node in &mut nodes {
            if node.value != i64::MIN {
                node.value = i64::MAX;
            }
        }

        let Some(_) = start_idx_iter.find(|&start_idx| {
            let start_node = &mut nodes[start_idx];
            if start_node.value == i64::MIN {
                start_node.value = 0;
                from_list.insert(start_idx);
                ever_seen.insert(start_idx);
                true
            } else {
                false
            }
        }) else {
            break;
        };
    }

    Ok(())
}

/// The graph given to this function must be solveable. (IE pass [check_if_unsolveable]), otherwise this will loop forever
/// Worst-case Complexity O(V*E), but about O(E) for average case
fn latency_count_bellman_ford(
    fanouts: &ListOfLists<FanInOut>,
    nodes: &mut [i64],
    nodes_to_process: &mut VecDeque<usize>,
) {
    fn dfs_fill_poison(fanouts: &ListOfLists<FanInOut>, nodes: &mut [i64], node: usize) {
        for edge in &fanouts[node] {
            let target_node = &mut nodes[edge.to_node];
            if *target_node != i64::MAX {
                *target_node = i64::MAX;
                dfs_fill_poison(fanouts, nodes, edge.to_node);
            }
        }
    }

    while let Some(node) = nodes_to_process.pop_front() {
        let from_latency = nodes[node];
        if from_latency == i64::MAX {
            // If this node is poisoned, then poison everything in its fanout immediately. That simplifies downstream logic
            dfs_fill_poison(fanouts, nodes, node);
        } else {
            for edge in &fanouts[node] {
                let target_node = &mut nodes[edge.to_node];
                let new_value = if let Some(delta) = edge.delta_latency {
                    from_latency - delta
                } else {
                    i64::MAX
                };

                if new_value > *target_node {
                    *target_node = new_value;
                    nodes_to_process.push_back(edge.to_node);
                }
            }
        }
    }
}

/// `result.len() == from.len()`.
///
/// Each result list contains the `to` nodes that were reached from the `from` node corresponding to the result list
fn find_all_to_all_paths(
    fanouts: &ListOfLists<FanInOut>,
    from: &[usize],
    to: &[usize],
) -> Vec<Vec<SpecifiedLatency>> {
    let mut nodes = vec![i64::MIN; fanouts.len()];
    let mut node_queue = VecDeque::new();
    from.iter()
        .map(|from_node| {
            nodes.fill(i64::MIN);

            nodes[*from_node] = 0;
            assert!(node_queue.is_empty());
            node_queue.push_back(*from_node);

            latency_count_bellman_ford(fanouts, &mut nodes, &mut node_queue);

            to.iter()
                .filter_map(|to_node| {
                    let latency = nodes[*to_node];
                    (latency != i64::MIN).then_some(SpecifiedLatency {
                        node: *to_node,
                        latency,
                    })
                })
                .collect()
        })
        .collect()
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
    latencies: Solution,
    conflicting_nodes: Vec<(usize, i64)>, // Node, and the desired latency of the conflict
    was_updated: bool,
}

impl PartialLatencyCountingSolution {
    fn offset_to_pin_node_to(&mut self, spec_lat: SpecifiedLatency) {
        let existing_latency_of_node = self.latencies[spec_lat.node].get_maybe().unwrap();
        let offset = spec_lat.latency - existing_latency_of_node;
        if offset == 0 {
            return; // Early exit, no change needed
        }

        for lat in self.latencies.iter_mut() {
            assert!(!lat.is_poisoned());
            if !lat.is_unset() {
                lat.abs_lat += offset;
            }
        }
        for conflict in &mut self.conflicting_nodes {
            conflict.1 += offset;
        }
    }
    fn try_merge(&mut self, from: &mut Self) -> bool {
        // Find a node both share
        let Some(joining_node) = self
            .latencies
            .iter()
            .zip(from.latencies.iter())
            .position(|(a, b)| a.get_maybe().is_some() && b.get_maybe().is_some())
        else {
            return false;
        };

        // Offset the vector we're merging to bring it in line with the target
        from.offset_to_pin_node_to(SpecifiedLatency {
            node: joining_node,
            latency: self.latencies[joining_node].get_maybe().unwrap(),
        });
        self.conflicting_nodes.append(&mut from.conflicting_nodes);

        let mut failure_to_merge = false;
        for (node, (to, from)) in zip(self.latencies.iter_mut(), from.latencies.iter()).enumerate()
        {
            if let (Some(to), Some(from)) = (to.get_maybe(), from.get_maybe()) {
                if to != from {
                    failure_to_merge = true;
                    self.conflicting_nodes.push((node, from));
                }
            }
        }

        if !failure_to_merge {
            for (to, from) in zip(self.latencies.iter_mut(), from.latencies.iter()) {
                if let (None, Some(from)) = (to.get_maybe(), from.get_maybe()) {
                    to.abs_lat = from;
                }
            }
            self.was_updated |= from.was_updated;
        }

        true
    }
}

fn solve_port_latencies(
    fanouts: &ListOfLists<FanInOut>,
    ports: &LatencyCountingPorts,
) -> Result<Vec<Vec<SpecifiedLatency>>, LatencyCountingError> {
    let mut bad_ports: Vec<(usize, i64, i64)> = Vec::new();

    let port_groups_iter = ports.inputs().iter().map(|input_port| {
        let start_node = SpecifiedLatency {
            node: *input_port,
            latency: 0,
        };

        let working_latencies = Solution::from_specified_and_fanouts(fanouts, &[start_node]);

        let result: Vec<SpecifiedLatency> = std::iter::once(start_node)
            .chain(ports.outputs().iter().filter_map(|output| {
                working_latencies[*output]
                    .get_maybe()
                    .map(|latency| SpecifiedLatency {
                        node: *output,
                        latency,
                    })
            }))
            .collect();

        debug_assert!(!SpecifiedLatency::has_duplicates(&result));

        result
    });

    let mut port_groups =
        merge_iter_into_disjoint_groups(port_groups_iter, |merge_to, merge_from| {
            debug_assert!(!SpecifiedLatency::has_duplicates(merge_to));
            debug_assert!(!SpecifiedLatency::has_duplicates(merge_from));

            let Some(offset) = merge_to.iter().find_map(|to| {
                SpecifiedLatency::get_latency(merge_from, to.node)
                    .map(|from_latency| to.latency - from_latency)
            }) else {
                return false;
            };

            for from_node in merge_from {
                from_node.latency += offset;

                if let Some(to_node_latency) =
                    SpecifiedLatency::get_latency(merge_to, from_node.node)
                {
                    if to_node_latency != from_node.latency {
                        bad_ports.push((from_node.node, to_node_latency, from_node.latency));
                    }
                } else {
                    merge_to.push(*from_node);
                }
            }

            debug_assert!(!SpecifiedLatency::has_duplicates(merge_to));
            true
        });

    for output_port in ports.outputs() {
        if !port_groups
            .iter()
            .any(|pg| SpecifiedLatency::get_latency(pg, *output_port).is_some())
        {
            port_groups.push(vec![SpecifiedLatency {
                node: *output_port,
                latency: 0,
            }]);
        }
    }

    if bad_ports.is_empty() {
        Ok(port_groups)
    } else {
        Err(LatencyCountingError::IndeterminablePortLatency { bad_ports })
    }
}

/// Solves the whole latency counting problem. No inference
pub fn solve_latencies(
    fanins: ListOfLists<FanInOut>,
    extra_fanin: Vec<(usize, FanInOut)>,
    ports: &LatencyCountingPorts,
    specified_latencies: &[SpecifiedLatency],
) -> Result<Solution, LatencyCountingError> {
    let fanouts = fanins.faninout_complement();
    find_positive_latency_cycle(&fanouts, specified_latencies)?;

    let fanins = fanins.add_extra_fanin_and_specified_latencies(extra_fanin, specified_latencies);

    // Cannot call config from a test case. See https://users.rust-lang.org/t/cargo-test-name-errors-with-error-invalid-value-name-for-files-file-does-not-exist/125855
    #[cfg(not(test))]
    if crate::config::config().debug_print_latency_graph {
        print_latency_test_case(&fanins, ports, specified_latencies);
    }

    if fanins.len() == 0 {
        return Ok(Solution(Vec::new()));
    }

    let fanouts = fanins.faninout_complement();
    debug_assert!(!has_poison_edge(&fanins));
    debug_assert!(!has_poison_edge(&fanouts)); // Equivalent

    let mut solution_seeds = solve_port_latencies(&fanouts, ports)?;

    if solution_seeds.is_empty() {
        solution_seeds.push(vec![specified_latencies.first().copied().unwrap_or(
            SpecifiedLatency {
                node: 0,
                latency: 0,
            },
        )]);
    }

    let mut partial_solutions: Vec<_> = solution_seeds
        .iter()
        .map(|seeds| PartialLatencyCountingSolution {
            latencies: Solution::make_initial_partial_solution(&fanins, &fanouts, seeds),
            conflicting_nodes: Vec::new(),
            was_updated: true,
        })
        .collect();

    let mut backwards = false;
    while partial_solutions.iter().any(|s| s.was_updated) {
        partial_solutions = merge_iter_into_disjoint_groups(
            partial_solutions.into_iter(),
            PartialLatencyCountingSolution::try_merge,
        );

        for psol in &mut partial_solutions {
            if psol.was_updated {
                // Then we can continue to explore on this solution
                psol.latencies.pin_all_valid();
                let fans = if backwards { &fanins } else { &fanouts };
                psol.was_updated = psol
                    .latencies
                    .count_latencies_from_all_pinned_latencies(fans, backwards)
            }
        }
        backwards = !backwards;
    }

    // Polish solution: if there were no specified latencies, then we make the latency of the first port '0
    // This is to shift the whole solution to one canonical absolute latency. Prefer:
    // - Specified latencies
    // - First port = '0
    // - Node 0 = '0
    // By doing the shift early, we make the port or node latency mismatch errors easier to understand as one of the options will be in the frame of the specified latencies.
    let reference_node = specified_latencies
        .first()
        .cloned()
        .unwrap_or_else(|| SpecifiedLatency {
            node: *ports.port_nodes.first().unwrap_or(&0),
            latency: 0,
        });

    if let Some(reference_solution_idx) = partial_solutions
        .iter()
        .position(|psol| !psol.latencies[reference_node.node].is_unset())
    {
        // Move the partial solution containing the reference node to the front, such that merge_where_possible always treats it as the reference.
        partial_solutions.swap(0, reference_solution_idx);
        partial_solutions[0].offset_to_pin_node_to(reference_node);
    }

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

/// [try_merge] should return true if the second argument was merged into the first argument.
fn merge_iter_into_disjoint_groups<T>(
    iter: impl Iterator<Item = T>,
    mut try_merge: impl FnMut(&mut T, &mut T) -> bool,
) -> Vec<T> {
    let mut result = Vec::new();

    for mut new_node in iter {
        result.retain_mut(|existing_elem| !try_merge(&mut new_node, existing_elem));
        result.push(new_node);
    }

    result
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

    let partial_solutions = solve_port_latencies(&fanouts, ports)?;

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

    let inference_port_solutions = solve_port_latencies(&fanouts, &inference_ports)?;

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
mod tests {
    use super::*;

    pub fn pinned_node(abs_lat: i64) -> LatencyNode {
        let result = LatencyNode {
            abs_lat,
            pinned: true,
        };
        assert!(!result.is_unset());
        result
    }

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
    ) -> Result<Solution, LatencyCountingError> {
        let ports = LatencyCountingPorts::from_inputs_outputs(inputs, outputs);

        solve_latencies(fanins, Vec::new(), &ports, specified_latencies)
    }

    #[track_caller]
    fn assert_latencies_match_exactly(found: &Solution, correct: &[i64]) {
        assert_eq!(found.len(), correct.len());

        assert!(
            std::iter::zip(found.iter(), correct.iter())
                .all(|(x, y)| x.get_maybe().is_some_and(|v| v == *y)),
            "Latencies don't match exactly: {found:?} !=lat= {correct:?}"
        );
    }

    #[track_caller]
    fn assert_latency_nodes_match_exactly(found: &Solution, correct: &[LatencyNode]) {
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
    fn assert_latencies_match(found: &Solution, correct: &[i64]) {
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

        let partial_result =
            solve_latencies_test_case(fanins, &[0, 4], &[3, 6], &specified_latencies).unwrap();

        let correct_latencies = [
            pinned_node(0), // [solve_latencies] returns pinned values
            pinned_node(0),
            pinned_node(3),
            pinned_node(3),
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
        let mut partial_solutions = solve_port_latencies(&fanouts, &ports).unwrap();

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
        let _partial_solutions = solve_port_latencies(&fanouts, &ports).unwrap();
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
        let _found_latencies = solve_port_latencies(&fanouts, &ports).unwrap();
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
