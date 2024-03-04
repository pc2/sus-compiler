use crate::list_of_lists::ListOfLists;


#[derive(Debug, Clone, Copy)]
pub struct SpecifiedLatency {
    pub wire : usize,
    pub latency : i64
}

#[derive(Debug)]
pub enum LatencyCountingError {
    ConflictingSpecifiedLatencies{conflict_path : Vec<SpecifiedLatency>},
    NetPositiveLatencyCycle{conflict_path : Vec<SpecifiedLatency>, net_roundtrip_latency : i64},
    IndeterminablePortLatency{bad_ports : Vec<(usize, i64, i64)>}
}

fn invert_lc_error(err : LatencyCountingError) -> LatencyCountingError {
    match err {
        LatencyCountingError::ConflictingSpecifiedLatencies { conflict_path:_} => {
            unreachable!("LatencyCountingError::ConflictingSpecifiedLatencies should not appear in backwards exploration, because port conflicts should have been found in the forward pass already");
            // conflict_path.reverse();
            // for c in &mut conflict_path {c.latency = -c.latency;}
            // LatencyCountingError::ConflictingSpecifiedLatencies { conflict_path, path_latency }
        }
        LatencyCountingError::NetPositiveLatencyCycle { mut conflict_path, net_roundtrip_latency } => {
            conflict_path.reverse();
            for c in &mut conflict_path {c.latency = -c.latency;}
            LatencyCountingError::NetPositiveLatencyCycle { conflict_path, net_roundtrip_latency } 
        }
        other => other
    }
}

#[derive(Debug, Clone, Copy)]
pub struct FanInOut {
    pub other : usize,
    pub delta_latency : i64
}

pub fn convert_fanin_to_fanout(fanins : &ListOfLists<FanInOut>) -> ListOfLists<FanInOut> {
    ListOfLists::from_random_access_iterator(
        fanins.len(),
        fanins.iter_flattened_by_bucket().map(|(bucket, &FanInOut{ other, delta_latency })| (other, FanInOut{other:bucket, delta_latency}))
    )
}

struct LatencyStackElem<'d> {
    node_idx : usize,
    remaining_fanout : std::slice::Iter<'d, FanInOut>
}

/*
    Algorithm:
    Initialize all inputs at latency 0
    Perform full forward pass, making latencies the maximum of all incoming latencies
    Then backward pass, moving nodes forward in latency as much as possible. 
    Only moving forward is possible, and only when not confliciting with a later node

    Leaves is_latency_pinned[start_node] == false
*/
fn count_latency<'d>(is_latency_pinned : &mut [bool], absolute_latency : &mut [i64], fanouts : &'d ListOfLists<FanInOut>, start_node : usize, stack : &mut Vec<LatencyStackElem<'d>>) -> Result<(), LatencyCountingError> {
    assert!(absolute_latency[start_node] != i64::MIN);
    
    assert!(stack.is_empty());

    stack.push(LatencyStackElem{node_idx : start_node, remaining_fanout : fanouts[start_node].iter()});

    while let Some(top) = stack.last_mut() {
        if let Some(&FanInOut{other, delta_latency}) = top.remaining_fanout.next() {
            let to_node_min_latency = absolute_latency[top.node_idx] + delta_latency;
            if to_node_min_latency > absolute_latency[other] {
                if is_latency_pinned[other] {
                    // Positive latency cycle error detected!
                    return Err(if let Some(conflict_begin) = stack.iter().position(|elem| elem.node_idx == other) {
                        let conflict_path = stack[conflict_begin..].iter().map(|elem| SpecifiedLatency{wire : elem.node_idx, latency : absolute_latency[elem.node_idx]}).collect();
                        LatencyCountingError::NetPositiveLatencyCycle { conflict_path, net_roundtrip_latency : to_node_min_latency - absolute_latency[start_node] }
                    } else {
                        let conflict_path = stack.iter().map(|elem| SpecifiedLatency{wire : elem.node_idx, latency : absolute_latency[elem.node_idx]}).chain(std::iter::once(SpecifiedLatency{wire : other, latency : to_node_min_latency})).collect();
                        LatencyCountingError::ConflictingSpecifiedLatencies { conflict_path }
                    });
                } else {
                    absolute_latency[other] = to_node_min_latency;
                    stack.push(LatencyStackElem{node_idx : other, remaining_fanout : fanouts[other].iter()});
                    is_latency_pinned[other] = true;
                }
            }
        } else {
            is_latency_pinned[top.node_idx] = false;
            stack.pop();
        }
    }

    Ok(())
}

fn invert_latency(latencies : &mut [i64]) {
    for lat in latencies.iter_mut() {
        if *lat != i64::MIN {
            *lat = -*lat;
        }
    }
}

struct PortData {
    wire : usize,
    already_covered : bool,
    absolute_latency : i64
}

struct LatencySolverSide<'d> {
    sources : Vec<PortData>,
    fanouts : &'d ListOfLists<FanInOut>,

    precomputed_seed_nodes : Vec<i64>,
}

impl<'d> LatencySolverSide<'d> {
    fn new(fanouts : &'d ListOfLists<FanInOut>, sources : &[usize]) -> Self {
        Self{fanouts, sources : sources.iter().map(|w| PortData{wire:*w, already_covered: false, absolute_latency: i64::MIN}).collect(), precomputed_seed_nodes : vec![i64::MIN; fanouts.len()]}
    }
    fn push_to_destination_ports(destination_ports : &mut [PortData], latency_buffer : &mut [i64]) -> Result<bool, LatencyCountingError> {
        let mut something_found = false;
        let mut bad_ports = Vec::new();
        for destination in destination_ports.iter_mut() {
            let found_latency = latency_buffer[destination.wire];
            if found_latency != i64::MIN {
                if destination.absolute_latency == i64::MIN {
                    destination.absolute_latency = -found_latency;
                    something_found = true;
                } else if destination.absolute_latency != -found_latency {
                    // Error because two outputs are attempting to create differing input latencies
                    bad_ports.push((destination.wire, -found_latency, destination.absolute_latency))
                }
            }
        }

        if bad_ports.is_empty() {
            Ok(something_found)
        } else {
            Err(LatencyCountingError::IndeterminablePortLatency{bad_ports})
        }
    }

    // Returns true if new nodes were discovered on the other side
    fn init_with_given_latencies(&mut self, given_latencies : &[SpecifiedLatency], destination : &mut Self, is_latency_pinned : &mut [bool], stack : &mut Vec<LatencyStackElem<'d>>) -> Result<bool, LatencyCountingError> {
        for n in self.precomputed_seed_nodes.iter_mut() {*n = i64::MIN}

        for lat in given_latencies {
            self.precomputed_seed_nodes[lat.wire] = lat.latency;
            is_latency_pinned[lat.wire] = true;
        }

        // If specified latencies happen to be the source ports of our module, we need not run them again. 
        for source in &mut self.sources {
            let v = self.precomputed_seed_nodes[source.wire];
            if v != i64::MIN {
                assert!(source.already_covered == false);
                source.already_covered = true;
                source.absolute_latency = v;
            }
        }

        // Fully precompute given latency buffer
        for lat in given_latencies {
            assert!(is_latency_pinned[lat.wire]);
            count_latency(is_latency_pinned, &mut self.precomputed_seed_nodes, self.fanouts, lat.wire, stack)?;
            is_latency_pinned[lat.wire] = true;
        }

        Self::push_to_destination_ports(&mut destination.sources, &mut self.precomputed_seed_nodes)
    }

    // Returns true if new nodes were discovered on the other side
    fn discover_connected_ports(&mut self, destination : &mut Self, temporary_buffer : &mut [i64], is_latency_pinned : &mut [bool], stack : &mut Vec<LatencyStackElem<'d>>) -> Result<bool, LatencyCountingError> {
        let mut something_found = false;

        for port in &mut self.sources {
            if port.absolute_latency == i64::MIN {continue} // Can't start the algorithm from an unsolved port
            if port.already_covered {continue} // Only ever explore from a given port once
            port.already_covered = true;

            // Reset temporary buffer
            temporary_buffer.copy_from_slice(&self.precomputed_seed_nodes);
            // new latency
            assert!(temporary_buffer[port.wire] == i64::MIN || temporary_buffer[port.wire] == port.absolute_latency);
            //assert!(self.is_latency_pinned[*output] == false);
            is_latency_pinned[port.wire] = true;
            temporary_buffer[port.wire] = port.absolute_latency;
            count_latency(is_latency_pinned, temporary_buffer, &self.fanouts, port.wire, stack)?;
            is_latency_pinned[port.wire] = true;
            
            something_found |= Self::push_to_destination_ports(&mut destination.sources, temporary_buffer)?;
        }

        Ok(something_found)
    }
}


fn extract_solution<'d>(mut latencies : Vec<i64>, fanins : &'d ListOfLists<FanInOut>, is_latency_pinned : &mut [bool], stack : &mut Vec<LatencyStackElem<'d>>) -> Result<Vec<i64>, LatencyCountingError> {
    // Also add nodes in fanin not dependent on an input to this input-output cluster. 
    // Nodes in fanout are included implicitly due to forward being the default direction
    invert_latency(&mut latencies);
    for start_node in 0..fanins.len() {
        if latencies[start_node] != i64::MIN {
            count_latency(is_latency_pinned, &mut latencies, &fanins, start_node, stack)?;
        }
    }
    invert_latency(&mut latencies);

    Ok(latencies)
}

pub fn solve_latencies<'d>(fanins : &'d ListOfLists<FanInOut>, fanouts : &'d ListOfLists<FanInOut>, inputs : &'d [usize], outputs : &'d [usize], mut specified_latencies : Vec<SpecifiedLatency>) -> Result<Vec<i64>, LatencyCountingError> {
    assert!(fanins.len() == fanouts.len());
    if fanins.len() == 0 {
        return Ok(Vec::new());
    }

    let mut input_side = LatencySolverSide::new(fanouts, inputs);
    let mut output_side = LatencySolverSide::new(fanins, outputs);

    let mut temporary_buffer = vec![i64::MIN; fanins.len()];
    let mut is_latency_pinned = vec![false; fanins.len()];
    let mut stack = Vec::new();

    // Add arbitrary seed latency if no latencies given
    if specified_latencies.len() == 0 {
        let arbitrary_wire = *inputs.first().unwrap_or(outputs.first().unwrap_or(&0));
        specified_latencies.push(SpecifiedLatency{ wire: arbitrary_wire, latency: 0 })
    }

    let mut found_new_ports = input_side.init_with_given_latencies(&specified_latencies, &mut output_side, &mut is_latency_pinned, &mut stack)?;
    for l in &mut specified_latencies {
        l.latency = -l.latency;
    }
    found_new_ports |= output_side.init_with_given_latencies(&specified_latencies, &mut input_side, &mut is_latency_pinned, &mut stack).map_err(invert_lc_error)?;

    while found_new_ports {
        found_new_ports = input_side.discover_connected_ports(&mut output_side, &mut temporary_buffer, &mut is_latency_pinned, &mut stack)?;
        found_new_ports |= output_side.discover_connected_ports(&mut input_side, &mut temporary_buffer, &mut is_latency_pinned, &mut stack).map_err(invert_lc_error)?;
    }

    let mut resulting_forward_latencies = input_side.precomputed_seed_nodes;

    for source in &input_side.sources {
        if source.absolute_latency != i64::MIN {
            resulting_forward_latencies[source.wire] = source.absolute_latency;
        }
    }
    for source in &output_side.sources {
        if source.absolute_latency != i64::MIN {
            resulting_forward_latencies[source.wire] = -source.absolute_latency;
        }
    }
    for source in &input_side.sources {
        if source.absolute_latency != i64::MIN {
            count_latency(&mut is_latency_pinned, &mut resulting_forward_latencies, &fanouts, source.wire, &mut stack)?;
        }
    }

    extract_solution(resulting_forward_latencies, fanins, &mut is_latency_pinned, &mut stack)
}


#[cfg(test)]
mod tests {
    use super::*;

    fn mk_fan(other : usize, delta_latency : i64) -> FanInOut {
        FanInOut{other, delta_latency}
    }

    // makes inputs for fanins, outputs for fanouts
    fn infer_ports(fanins : &ListOfLists<FanInOut>) -> Vec<usize> {
        fanins.iter().enumerate().filter_map(|(idx, v)| v.is_empty().then_some(idx)).collect()
    }

    fn solve_latencies_infer_ports(fanins : &ListOfLists<FanInOut>, specified_latencies : Vec<SpecifiedLatency>) -> Result<Vec<i64>, LatencyCountingError> {
        let fanins = fanins.into();
        let fanouts = convert_fanin_to_fanout(fanins);
        
        let inputs = infer_ports(fanins);
        let outputs = infer_ports(&fanouts);
        
        solve_latencies(fanins, &fanouts, &inputs, &outputs, specified_latencies)
    }

    fn latencies_equal(a : &[i64], b : &[i64]) -> bool {
        assert_eq!(a.len(), b.len());
    
        let diff = a[0].wrapping_sub(b[0]);
    
        for (x, y) in std::iter::zip(a.iter(), b.iter()) {
            if x.wrapping_sub(*y) != diff {
                return false;
            }
        }
        return true;
    }
    
    #[test]
    fn check_correct_latency_basic() {
        let fanins : [&[FanInOut]; 7] = [
            /*0*/&[],
            /*1*/&[mk_fan(0, 0)],
            /*2*/&[mk_fan(1, 1),mk_fan(5, 1)],
            /*3*/&[mk_fan(2, 0)],
            /*4*/&[],
            /*5*/&[mk_fan(4, 0),mk_fan(1, 1)],
            /*6*/&[mk_fan(5, 0)]
        ];
        let fanins = ListOfLists::from_slice_slice(&fanins);

        let correct_latencies = [0,0,2,2,1,1,1];

        let fanouts = convert_fanin_to_fanout(&fanins);
        
        let inputs = vec![0, 4];
        let outputs = vec![3, 6];

        let found_latencies = solve_latencies(&fanins, &fanouts, &inputs, &outputs, Vec::new()).unwrap();

        assert!(latencies_equal(&found_latencies, &correct_latencies), "{found_latencies:?} =lat= {correct_latencies:?}");
    }
    
    #[test]
    fn check_correct_latency_backwards() {
        let fanins : [&[FanInOut]; 7] = [
            /*0*/&[],
            /*1*/&[mk_fan(0, 0)],
            /*2*/&[mk_fan(1, 1),mk_fan(5, 1)],
            /*3*/&[mk_fan(2, 0)],
            /*4*/&[],
            /*5*/&[mk_fan(4, 0),mk_fan(1, 1)],
            /*6*/&[mk_fan(5, 0)]
        ];
        let fanins = ListOfLists::from_slice_slice(&fanins);

        let correct_latencies = [-1,-1,1,1,0,0,0];

        let fanouts = convert_fanin_to_fanout(&fanins);
        
        let inputs = vec![0, 4];
        let outputs = vec![3, 6];

        let found_latencies = solve_latencies(&fanins, &fanouts, &inputs, &outputs, vec![SpecifiedLatency{wire:6,latency:0}]).unwrap();

        assert_eq!(found_latencies, correct_latencies);
    }
    
    #[test]
    fn check_correct_latency_from_any_start_node() {
        let fanins : [&[FanInOut]; 7] = [
            /*0*/&[],
            /*1*/&[mk_fan(0, 0)],
            /*2*/&[mk_fan(1, 1),mk_fan(5, 1)],
            /*3*/&[mk_fan(2, 0)],
            /*4*/&[],
            /*5*/&[mk_fan(4, 0),mk_fan(1, 1)],
            /*6*/&[mk_fan(5, 0)]
        ];
        let fanins = ListOfLists::from_slice_slice(&fanins);

        let correct_latencies = [0,0,2,2,1,1,1];

        let fanouts = convert_fanin_to_fanout(&fanins);
        
        let inputs = vec![0, 4];
        let outputs = vec![3, 6];

        for starting_node in 0..7 {
            println!("starting_node: {starting_node}");
            let found_latencies = solve_latencies(&fanins, &fanouts, &inputs, &outputs, vec![SpecifiedLatency{wire:starting_node,latency:0}]).unwrap();

            assert!(latencies_equal(&found_latencies, &correct_latencies), "{found_latencies:?} =lat= {correct_latencies:?}");
        }
    }
    
    #[test]
    fn check_correct_latency_with_superfluous_input() {
        let fanins : [&[FanInOut]; 8] = [
            /*0*/&[],
            /*1*/&[mk_fan(0, 0)],
            /*2*/&[mk_fan(1, 1),mk_fan(5, 1)],
            /*3*/&[mk_fan(2, 0)],
            /*4*/&[],
            /*5*/&[mk_fan(4, 0),mk_fan(1, 1),mk_fan(7, 2)],
            /*6*/&[mk_fan(5, 0)],
            /*7*/&[] // superfluous input
        ];
        let fanins = ListOfLists::from_slice_slice(&fanins);

        let correct_latencies = [0,0,2,2,1,1,1,-1];

        let fanouts = convert_fanin_to_fanout(&fanins);
        
        let inputs = vec![0, 4];
        let outputs = vec![3, 6];

        let found_latencies = solve_latencies(&fanins, &fanouts, &inputs, &outputs, Vec::new()).unwrap();

        assert!(latencies_equal(&found_latencies, &correct_latencies), "{found_latencies:?} =lat= {correct_latencies:?}");
    }
    
    #[test]
    fn check_correct_latency_with_superfluous_output() {
        let fanins : [&[FanInOut]; 8] = [
            /*0*/&[],
            /*1*/&[mk_fan(0, 0)],
            /*2*/&[mk_fan(1, 1),mk_fan(5, 1)],
            /*3*/&[mk_fan(2, 0)],
            /*4*/&[],
            /*5*/&[mk_fan(4, 0),mk_fan(1, 1)],
            /*6*/&[mk_fan(5, 0)],
            /*7*/&[mk_fan(5, 2)] // superfluous output
        ];
        let fanins = ListOfLists::from_slice_slice(&fanins);

        let correct_latencies = [-1,-1,1,1,0,0,0,2];

        let fanouts = convert_fanin_to_fanout(&fanins);
        
        let inputs = vec![0, 4];
        let outputs = vec![3, 6];

        let found_latencies = solve_latencies(&fanins, &fanouts, &inputs, &outputs, Vec::new()).unwrap();

        assert!(latencies_equal(&found_latencies, &correct_latencies), "{found_latencies:?} =lat= {correct_latencies:?}");
    }
    
    #[test]
    fn check_conflicting_port_latency() {
        let fanins : [&[FanInOut]; 7] = [
            /*0*/&[],
            /*1*/&[mk_fan(0, 0)],
            /*2*/&[mk_fan(1, 3),mk_fan(5, 1)],
            /*3*/&[mk_fan(2, 0)],
            /*4*/&[],
            /*5*/&[mk_fan(4, 0),mk_fan(1, 1)],
            /*6*/&[mk_fan(5, 0)]
        ];
        let fanins = ListOfLists::from_slice_slice(&fanins);

        let should_be_err = solve_latencies_infer_ports(&fanins, Vec::new());

        assert!(matches!(should_be_err, Err(LatencyCountingError::IndeterminablePortLatency{bad_ports:_})))
    }
    
    #[test]
    fn check_conflicting_port_latency_with_any_starting_node_does_error() {
        let fanins : [&[FanInOut]; 7] = [
            /*0*/&[],
            /*1*/&[mk_fan(0, 0)],
            /*2*/&[mk_fan(1, 3),mk_fan(5, 1)],
            /*3*/&[mk_fan(2, 0)],
            /*4*/&[],
            /*5*/&[mk_fan(4, 0),mk_fan(1, 1)],
            /*6*/&[mk_fan(5, 0)]
        ];
        let fanins = ListOfLists::from_slice_slice(&fanins);

        for starting_node in 0..7 {
            println!("starting_node: {starting_node}");
            solve_latencies_infer_ports(&fanins, vec![SpecifiedLatency{wire:starting_node,latency:0}]).unwrap_err();
        }
    }
    
    #[test]
    fn check_conflicting_port_latency_resolved() {
        let fanins : [&[FanInOut]; 7] = [
            /*0*/&[],
            /*1*/&[mk_fan(0, 0)],
            /*2*/&[mk_fan(1, 3),mk_fan(5, 1)],
            /*3*/&[mk_fan(2, 0)],
            /*4*/&[],
            /*5*/&[mk_fan(4, 0),mk_fan(1, 1)],
            /*6*/&[mk_fan(5, 0)]
        ];
        let fanins = ListOfLists::from_slice_slice(&fanins);

        let found_latencies = solve_latencies_infer_ports(&fanins, vec![SpecifiedLatency{wire:0,latency:0}, SpecifiedLatency{wire:4,latency:2}]).unwrap();

        let correct_latencies = [0,0,3,3,2,2,2];

        assert_eq!(found_latencies, correct_latencies); // Can even do a strict check here, because we defined some of the latencies
    }
    
    #[test]
    fn check_conflicting_port_specifiers() {
        let fanins : [&[FanInOut]; 7] = [
            /*0*/&[],
            /*1*/&[mk_fan(0, 0)],
            /*2*/&[mk_fan(1, 1),mk_fan(5, 1)],
            /*3*/&[mk_fan(2, 0)],
            /*4*/&[],
            /*5*/&[mk_fan(4, 0),mk_fan(1, 1)],
            /*6*/&[mk_fan(5, 0)]
        ];
        let fanins = ListOfLists::from_slice_slice(&fanins);

        let should_be_err = solve_latencies_infer_ports(&fanins, vec![SpecifiedLatency{wire: 0, latency : 0}, SpecifiedLatency{wire: 3, latency : 1}]);

        println!("{should_be_err:?}");
        let Err(LatencyCountingError::ConflictingSpecifiedLatencies{conflict_path}) = should_be_err else {unreachable!()};
        let path_latency = conflict_path.last().unwrap().latency - conflict_path.first().unwrap().latency;
        assert_eq!(path_latency, 2);
    }
    
    #[test]
    fn check_conflicting_inline_specifiers() {
        let fanins : [&[FanInOut]; 7] = [
            /*0*/&[],
            /*1*/&[mk_fan(0, 0)],
            /*2*/&[mk_fan(1, 1),mk_fan(5, 1)],
            /*3*/&[mk_fan(2, 0)],
            /*4*/&[],
            /*5*/&[mk_fan(4, 0),mk_fan(1, 1)],
            /*6*/&[mk_fan(5, 0)]
        ];
        let fanins = ListOfLists::from_slice_slice(&fanins);

        let should_be_err = solve_latencies_infer_ports(&fanins, vec![SpecifiedLatency{wire: 1, latency : 0}, SpecifiedLatency{wire: 5, latency : 0}]);

        let Err(LatencyCountingError::ConflictingSpecifiedLatencies{conflict_path}) = should_be_err else {unreachable!()};
        let path_latency = conflict_path.last().unwrap().latency - conflict_path.first().unwrap().latency;
        assert_eq!(path_latency, 1);
    }
    
    #[test]
    fn check_conflicting_inline_specifiers_bad_case() {
        let fanins : [&[FanInOut]; 3] = [
            /*0*/&[],
            /*1*/&[mk_fan(2, 1)],
            /*2*/&[mk_fan(0, 1)],
        ];
        let fanins = ListOfLists::from_slice_slice(&fanins);

        let should_be_err = solve_latencies_infer_ports(&fanins, vec![SpecifiedLatency{wire: 0, latency : 0}, SpecifiedLatency{wire: 1, latency : 1}]);
        println!("{should_be_err:?}");

        
        let Err(LatencyCountingError::ConflictingSpecifiedLatencies{conflict_path}) = should_be_err else {unreachable!()};
        let path_latency = conflict_path.last().unwrap().latency - conflict_path.first().unwrap().latency;
        assert_eq!(path_latency, 2);
        
        assert_eq!(conflict_path[0].wire, 0);
        assert_eq!(conflict_path[1].wire, 2);
        assert_eq!(conflict_path[2].wire, 1);
    }
    
    #[test]
    fn check_disjoint() {
        let fanins : [&[FanInOut]; 7] = [
            /*0*/&[],
            /*1*/&[mk_fan(0, 0)],
            /*2*/&[mk_fan(1, 3)],
            /*3*/&[mk_fan(2, 0)],
            /*4*/&[],
            /*5*/&[mk_fan(4, 0)],
            /*6*/&[mk_fan(5, 0)]
        ];
        let fanins = ListOfLists::from_slice_slice(&fanins);

        let partial_result = solve_latencies_infer_ports(&fanins, vec![SpecifiedLatency{ wire: 0, latency: 0 }]).unwrap();

        assert_eq!(partial_result, &[0, 0, 3, 3, i64::MIN, i64::MIN, i64::MIN])
    }
    
    #[test]
    fn check_bad_cycle() {
        let fanins : [&[FanInOut]; 5] = [
            /*0*/&[],
            /*1*/&[mk_fan(0, 0), mk_fan(4, -4)],
            /*2*/&[mk_fan(1, 3)],
            /*3*/&[mk_fan(2, 0)],
            /*4*/&[mk_fan(2, 2)],
        ];
        let fanins = ListOfLists::from_slice_slice(&fanins);

        let should_be_err = solve_latencies_infer_ports(&fanins, Vec::new());

        let Err(LatencyCountingError::NetPositiveLatencyCycle{conflict_path:_, net_roundtrip_latency}) = should_be_err else {unreachable!()};
        assert_eq!(net_roundtrip_latency, 1);
    }
}

