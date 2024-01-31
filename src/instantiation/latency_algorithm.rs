use std::iter::zip;

#[derive(Debug)]
pub enum LatencyCountingError {
    PositiveNetLatencyCycle{nodes_involved : Vec<usize>},
    ConflictingPortLatency{bad_ports : Vec<(usize, i64, i64)>},
    DisjointNodes{nodes_not_reached : Vec<usize>}
}



pub struct FanInOut {
    pub other : usize,
    pub delta_latency : i64
}

/*
    Algorithm:
    Initialize all inputs at latency 0
    Perform full forward pass, making latencies the maximum of all incoming latencies
    Then backward pass, moving nodes forward in latency as much as possible. 
    Only moving forward is possible, and only when not confliciting with a later node
*/
fn count_latency_recursive(part_of_path : &mut [bool], absolute_latency : &mut [i64], fanouts : &[Vec<FanInOut>], cur_node : usize) -> Result<(), Vec<usize>> {
    part_of_path[cur_node] = true;
    for &FanInOut{other, delta_latency} in &fanouts[cur_node] {
        let to_node_min_latency = absolute_latency[cur_node] + delta_latency;
        if to_node_min_latency > absolute_latency[other] {
            if part_of_path[other] {
                //todo!("Cycles for positive net latency error!");
                return Err(vec![other]);
            } else {
                absolute_latency[other] = to_node_min_latency;
                if let Err(mut bad_cycle_nodes) = count_latency_recursive(part_of_path, absolute_latency, fanouts, other) {
                    bad_cycle_nodes.push(cur_node);
                    return Err(bad_cycle_nodes);
                }
            }
        }
    }
    part_of_path[cur_node] = false;
    Ok(())
}

fn count_latency(part_of_path : &mut [bool], absolute_latency : &mut [i64], fanouts : &[Vec<FanInOut>], start_node : usize, start_value : i64) -> Result<(), LatencyCountingError> {
    for p in part_of_path.iter() {assert!(!*p);}

    assert!(absolute_latency[start_node] == i64::MIN);
    absolute_latency[start_node] = start_value;
    count_latency_recursive(part_of_path, absolute_latency, fanouts, start_node).map_err(|mut nodes_involved| {
        let mut nodes_iter = nodes_involved.iter().enumerate();
        let first_node_in_cycle = nodes_iter.next().unwrap().1;
        for (idx, node) in nodes_iter {
            if node == first_node_in_cycle {
                nodes_involved.truncate(idx);
                break;
            }
        }
        return LatencyCountingError::PositiveNetLatencyCycle{nodes_involved}
    })?;

    for p in part_of_path.iter() {assert!(!*p);}
    Ok(())
}

fn solve_latencies(fanins : &[Vec<FanInOut>], fanouts : &[Vec<FanInOut>], inputs : &[usize], outputs : &[usize]) -> Result<Vec<i64>, LatencyCountingError> {
    assert!(fanins.len() == fanouts.len());

    let mut part_of_path : Vec<bool> = vec![false; fanouts.len()];

    // Forwards are all performed in the same block. This block is then also used as the output latencies
    let mut absolute_latencies_forward : Vec<i64> = vec![i64::MIN; fanouts.len()];
    let mut absolute_latencies_backward_combined : Vec<i64> = vec![i64::MAX; fanouts.len()];

    // To find input latencies based on output latencies, we use a separate block to go backwards. 
    // These are done one at a time, such that we can find conflicting latencies. 
    let mut absolute_latencies_backward_temporary : Vec<i64> = vec![i64::MIN; fanouts.len()];

    let mut output_was_covered : Vec<bool> = vec![false; outputs.len()];
    let mut input_node_assignments : Vec<i64> = vec![i64::MIN; inputs.len()];

    input_node_assignments[0] = 0; // Provide a seed to start the algorithm

    let mut last_num_valid_start_nodes = 0;
    loop {
        let mut cur_num_valid_start_nodes = 0;
        // Add newly discovered input assignments
        for (input_wire, assignment) in zip(inputs.iter(), input_node_assignments.iter()) {
            if *assignment != i64::MIN {
                if absolute_latencies_forward[*input_wire] == i64::MIN {
                    count_latency(&mut part_of_path, &mut absolute_latencies_forward, fanouts, *input_wire, *assignment)?;
                } else {
                    // Erroneous is unreachable, because conflicting assignments should have been caught when they're put into the input_node_assignments list
                    assert!(absolute_latencies_forward[*input_wire] == *assignment);
                }
                cur_num_valid_start_nodes += 1;
            }
        }
        if cur_num_valid_start_nodes == last_num_valid_start_nodes {
            break;
        }

        last_num_valid_start_nodes = cur_num_valid_start_nodes;

        // Find new backwards starting nodes
        let mut bad_ports = Vec::new();
        for (output, was_covered) in zip(outputs.iter(), output_was_covered.iter_mut()) {
            if absolute_latencies_forward[*output] != i64::MIN {
                if !*was_covered { // Little optimization, so we only every cover a backwards latency once
                    *was_covered = true;
                    // new latency
                    // Reset temporary buffer
                    for v in absolute_latencies_backward_temporary.iter_mut() {
                        *v = i64::MIN;
                    }
                    count_latency(&mut part_of_path, &mut absolute_latencies_backward_temporary, fanins, *output, -absolute_latencies_forward[*output])?;
                    
                    for (input, assignment) in zip(inputs.iter(), input_node_assignments.iter_mut()) {
                        let found_inv_latency = absolute_latencies_backward_temporary[*input];
                        if found_inv_latency != i64::MIN {
                            if *assignment == i64::MIN {
                                *assignment = -found_inv_latency;
                            } else {
                                if -found_inv_latency != *assignment {
                                    // Error because two outputs are attempting to create differing input latencies
                                    bad_ports.push((*output, -found_inv_latency, *assignment))
                                }// else we're fine
                            }
                        }
                    }

                    // Add backwards latencies to combined list
                    for (from, to) in zip(absolute_latencies_backward_temporary.iter(), absolute_latencies_backward_combined.iter_mut()) {
                        if *from != i64::MIN && -*from < *to {
                            *to = -*from;
                        }
                    }
                }
            }
        }

        if !bad_ports.is_empty() {
            return Err(LatencyCountingError::ConflictingPortLatency{bad_ports})
        }
    }


    // Check end conditions
    let nodes_not_reached : Vec<usize> = absolute_latencies_forward.iter().enumerate().filter_map(|(idx, v)| (*v == i64::MIN).then_some(idx)).collect();
    if nodes_not_reached.is_empty() {
        Ok(absolute_latencies_forward)
    } else {
        Err(LatencyCountingError::DisjointNodes{nodes_not_reached})
    }
}

fn convert_fanin_to_fanout(fanins : &[Vec<FanInOut>]) -> Vec<Vec<FanInOut>> {
    let mut fanouts : Vec<Vec<FanInOut>> = fanins.iter().map(|_| {
        Vec::new()
    }).collect();

    for (id, fin) in fanins.iter().enumerate() {
        for f in fin {
            fanouts[f.other].push(FanInOut { other: id, delta_latency: f.delta_latency })
        }
    }

    fanouts
}

#[cfg(test)]
mod tests {
    use super::*;

    fn mk_fan(other : usize, delta_latency : i64) -> FanInOut {
        FanInOut{other, delta_latency}
    }

    fn solve_latencies_infer_ports(fanins : &[Vec<FanInOut>]) -> Result<Vec<i64>, LatencyCountingError> {
        let fanouts = convert_fanin_to_fanout(fanins);
        
        let inputs : Box<[usize]> = fanins.iter().enumerate().filter_map(|(idx, v)| v.is_empty().then_some(idx)).collect();
        let outputs : Box<[usize]> = fanouts.iter().enumerate().filter_map(|(idx, v)| v.is_empty().then_some(idx)).collect();
        
        solve_latencies(fanins, &fanouts, &inputs, &outputs)
    }

    fn latencies_equal(a : &[i64], b : &[i64]) -> bool {
        assert!(a.len() == b.len());
    
        let diff = a[0] - b[0];
    
        for (x, y) in zip(a.iter(), b.iter()) {
            if *x - *y != diff {
                return false;
            }
        }
        return true;
    }
    
    #[test]
    fn check_correct_latency_basic() {
        let graph = [
            /*0*/vec![],
            /*1*/vec![mk_fan(0, 0)],
            /*2*/vec![mk_fan(1, 1),mk_fan(5, 1)],
            /*3*/vec![mk_fan(2, 0)],
            /*4*/vec![],
            /*5*/vec![mk_fan(4, 0),mk_fan(1, 1)],
            /*6*/vec![mk_fan(5, 0)]
        ];

        let correct_latencies = [-1,-1,1,1,0,0,0];

        let found_latencies = solve_latencies_infer_ports(&graph).unwrap();

        assert!(latencies_equal(&found_latencies, &correct_latencies), "{found_latencies:?} =lat= {correct_latencies:?}");
    }
    
    #[test]
    fn check_conflicting_port_latency() {
        let graph = [
            /*0*/vec![],
            /*1*/vec![mk_fan(0, 0)],
            /*2*/vec![mk_fan(1, 3),mk_fan(5, 1)],
            /*3*/vec![mk_fan(2, 0)],
            /*4*/vec![],
            /*5*/vec![mk_fan(4, 0),mk_fan(1, 1)],
            /*6*/vec![mk_fan(5, 0)]
        ];

        let should_be_err = solve_latencies_infer_ports(&graph);

        assert!(matches!(should_be_err, Err(LatencyCountingError::ConflictingPortLatency{bad_ports:_})))
    }
    
    #[test]
    fn check_disjoint() {
        let graph = [
            /*0*/vec![],
            /*1*/vec![mk_fan(0, 0)],
            /*2*/vec![mk_fan(1, 3)],
            /*3*/vec![mk_fan(2, 0)],
            /*4*/vec![],
            /*5*/vec![mk_fan(4, 0)],
            /*6*/vec![mk_fan(5, 0)]
        ];

        let should_be_err = solve_latencies_infer_ports(&graph);

        assert!(matches!(should_be_err, Err(LatencyCountingError::DisjointNodes{nodes_not_reached: _})))
    }
    
    #[test]
    fn check_bad_cycle() {
        let graph = [
            /*0*/vec![],
            /*1*/vec![mk_fan(0, 0), mk_fan(4, -4)],
            /*2*/vec![mk_fan(1, 3)],
            /*3*/vec![mk_fan(2, 0)],
            /*4*/vec![mk_fan(2, 2)],
        ];

        let should_be_err = solve_latencies_infer_ports(&graph);

        assert!(matches!(should_be_err, Err(LatencyCountingError::PositiveNetLatencyCycle{nodes_involved: _})))
    }
}

