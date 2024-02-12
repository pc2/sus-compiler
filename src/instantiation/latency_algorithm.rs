use std::iter::zip;

#[derive(Debug)]
pub enum LatencyCountingError {
    InsufficientLatencySlackBetweenSetWires{conflict_nodes : Vec<usize>},
    PositiveNetLatencyCycle{conflict_nodes : Vec<usize>},
    ConflictingPortLatency{bad_ports : Vec<(usize, i64, i64)>},
    DisjointNodes{start_node : usize, nodes_not_reached : Vec<usize>},
    NotImplemented
}

pub struct FanInOut {
    pub other : usize,
    pub delta_latency : i64
}

pub fn convert_fanin_to_fanout(fanins : &[Vec<FanInOut>]) -> Vec<Vec<FanInOut>> {
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
*/
fn count_latency<'d>(is_latency_pinned : &mut [bool], absolute_latency : &mut [i64], fanouts : &'d [Vec<FanInOut>], start_node : usize, stack : &mut Vec<LatencyStackElem<'d>>) -> Result<(), LatencyCountingError> {
    assert!(absolute_latency[start_node] != i64::MIN);
    
    assert!(stack.is_empty());

    is_latency_pinned[start_node] = true;
    stack.push(LatencyStackElem{node_idx : start_node, remaining_fanout : fanouts[start_node].iter()});

    while let Some(top) = stack.last_mut() {
        if let Some(&FanInOut{other, delta_latency}) = top.remaining_fanout.next() {
            let to_node_min_latency = absolute_latency[top.node_idx] + delta_latency;
            if to_node_min_latency > absolute_latency[other] {
                if is_latency_pinned[other] {
                    // Positive latency cycle error detected!
                    return Err(if let Some(conflict_begin) = stack.iter().position(|elem| elem.node_idx == other) {
                        let conflict_nodes = stack[conflict_begin..].iter().map(|elem| elem.node_idx).collect();
                        LatencyCountingError::PositiveNetLatencyCycle { conflict_nodes }
                    } else {
                        let conflict_nodes = stack.iter().map(|elem| elem.node_idx).collect();
                        LatencyCountingError::InsufficientLatencySlackBetweenSetWires { conflict_nodes }
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

fn count_latency_from<'d>(is_latency_pinned : &mut [bool], absolute_latency : &mut [i64], fanouts : &'d [Vec<FanInOut>], start_node : usize, start_value : i64, stack : &mut Vec<LatencyStackElem<'d>>) -> Result<(), LatencyCountingError> {
    assert!(absolute_latency[start_node] == i64::MIN);
    absolute_latency[start_node] = start_value;
    
    count_latency(is_latency_pinned, absolute_latency, fanouts, start_node, stack)?;
    Ok(())
}

fn invert_latency(latencies : &mut [i64]) {
    for lat in latencies.iter_mut() {
        if *lat != i64::MIN {
            *lat = -*lat;
        }
    }
}

struct LatencySolver<'d> {
    fanins : &'d [Vec<FanInOut>],
    fanouts : &'d [Vec<FanInOut>],
    inputs : &'d [usize],
    outputs : &'d [usize],

    is_latency_pinned : Vec<bool>,

    stack : Vec<LatencyStackElem<'d>>,

    // To find input latencies based on output latencies, we use a separate block to go backwards. 
    // These are done one at a time, such that we can find conflicting latencies. 
    absolute_latencies_backward_temporary : Vec<i64>,
    touched_backwards : Vec<bool>,

    output_was_covered : Vec<bool>,
    input_node_assignments : Vec<i64>,
}

impl<'d> LatencySolver<'d> {
    fn new(fanins : &'d [Vec<FanInOut>], fanouts : &'d [Vec<FanInOut>], inputs : &'d [usize], outputs : &'d [usize]) -> Self {
        assert!(fanins.len() == fanouts.len());
        
        for i in inputs {
            assert!(fanins[*i].is_empty());
        }
        // Actually, outputs *can* have fanout. You can still use the value of an output to compute other stuff
        // Inputs however, cannot have fanin
        /*for o in outputs {
            assert!(fanouts[*o].is_empty());
        }*/

        // Initialize main buffers
        let is_latency_pinned : Vec<bool> = vec![false; fanouts.len()];
        let absolute_latencies_backward_temporary : Vec<i64> = vec![i64::MIN; fanouts.len()];
        let touched_backwards : Vec<bool> = vec![false; fanouts.len()];
        let output_was_covered : Vec<bool> = vec![false; outputs.len()];
        let input_node_assignments : Vec<i64> = vec![i64::MIN; inputs.len()];

        Self{fanins, fanouts, inputs, outputs, stack : Vec::new(), is_latency_pinned, absolute_latencies_backward_temporary, touched_backwards, output_was_covered, input_node_assignments}
    }
    
    fn seed(&mut self) -> Result<(), LatencyCountingError> {
        if let Some(v) = self.input_node_assignments.get_mut(0) { // Provide a seed to start the algorithm
            *v = 0;
            Ok(())
        } else {
            Err(LatencyCountingError::NotImplemented)
        }
    }

    fn solve_latencies(&mut self) -> Result<Vec<i64>, LatencyCountingError> {
        // Forwards are all performed in the same block. This block is then also used as the output latencies
        let mut absolute_latencies_forward : Vec<i64> = vec![i64::MIN; self.fanouts.len()];

        for t in self.touched_backwards.iter_mut() {*t = false;}

        let mut last_num_valid_start_nodes = 0;
        loop {
            let mut cur_num_valid_start_nodes = 0;
            // Add newly discovered input assignments
            for (input_wire, assignment) in zip(self.inputs.iter(), self.input_node_assignments.iter()) {
                if *assignment != i64::MIN {
                    if absolute_latencies_forward[*input_wire] == i64::MIN {
                        count_latency_from(&mut self.is_latency_pinned, &mut absolute_latencies_forward, self.fanouts, *input_wire, *assignment, &mut self.stack)?;
                    } else {
                        // Erroneous is unreachable, because conflicting assignments should have been caught when they're put into the input_node_assignments list
                        assert!(absolute_latencies_forward[*input_wire] == *assignment);
                    }
                    cur_num_valid_start_nodes += 1;
                }
            }
            if cur_num_valid_start_nodes == last_num_valid_start_nodes {break;}
            last_num_valid_start_nodes = cur_num_valid_start_nodes;

            // Find new backwards starting nodes
            let mut bad_ports = Vec::new();
            for (output, was_covered) in zip(self.outputs.iter(), self.output_was_covered.iter_mut()) {
                if absolute_latencies_forward[*output] != i64::MIN {
                    if !*was_covered { // Little optimization, so we only every cover a backwards latency once
                        *was_covered = true;
                        // new latency
                        // Reset temporary buffer
                        for v in &self.absolute_latencies_backward_temporary {assert!(*v == i64::MIN);}
                        count_latency_from(&mut self.is_latency_pinned, &mut self.absolute_latencies_backward_temporary, self.fanins, *output, -absolute_latencies_forward[*output], &mut self.stack)?;
                        
                        for (input, assignment) in zip(self.inputs.iter(), self.input_node_assignments.iter_mut()) {
                            let found_inv_latency = self.absolute_latencies_backward_temporary[*input];
                            if found_inv_latency != i64::MIN {
                                if *assignment == i64::MIN {
                                    *assignment = -found_inv_latency;
                                } else {
                                    if -found_inv_latency != *assignment {
                                        // Error because two outputs are attempting to create differing input latencies
                                        bad_ports.push((*input, -found_inv_latency, *assignment))
                                    }// else we're fine
                                }
                            }
                        }
                        for (t_b, v) in zip(self.touched_backwards.iter_mut(), self.absolute_latencies_backward_temporary.iter_mut()) {
                            if *v != i64::MIN {
                                *v = i64::MIN;
                                *t_b = true;
                            }
                        }
                    }
                }
            }

            if !bad_ports.is_empty() {
                return Err(LatencyCountingError::ConflictingPortLatency{bad_ports})
            }
        }

        // Also add nodes in fanin not dependent on an input to this input-output cluster. 
        // Nodes in fanout are included implicitly due to forward being the default direction
        invert_latency(&mut absolute_latencies_forward);
        for (start_node, fanin_of_output) in self.touched_backwards.iter().enumerate() {
            if *fanin_of_output && (absolute_latencies_forward[start_node] != i64::MIN) {
                count_latency(&mut self.is_latency_pinned, &mut absolute_latencies_forward, self.fanins, start_node, &mut self.stack)?;
            }
        }
        invert_latency(&mut absolute_latencies_forward);

        let nodes_not_reached : Vec<usize> = absolute_latencies_forward.iter().enumerate().filter_map(|(idx, v)| (*v == i64::MIN).then_some(idx)).collect();
        if nodes_not_reached.is_empty() {
            Ok(absolute_latencies_forward)
        } else {
            Err(LatencyCountingError::DisjointNodes{start_node: self.inputs[0], nodes_not_reached})
        }
    }
}

pub fn solve_latencies(fanins : &[Vec<FanInOut>], fanouts : &[Vec<FanInOut>], inputs : &[usize], outputs : &[usize]) -> Result<Vec<i64>, LatencyCountingError> {
    let mut latency_solver = LatencySolver::new(fanins, fanouts, inputs, outputs);
    latency_solver.seed()?;
    latency_solver.solve_latencies()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn mk_fan(other : usize, delta_latency : i64) -> FanInOut {
        FanInOut{other, delta_latency}
    }

    // makes inputs for fanins, outputs for fanouts
    fn infer_ports(fanins : &[Vec<FanInOut>]) -> Vec<usize> {
        fanins.iter().enumerate().filter_map(|(idx, v)| v.is_empty().then_some(idx)).collect()
    }

    fn solve_latencies_infer_ports(fanins : &[Vec<FanInOut>]) -> Result<Vec<i64>, LatencyCountingError> {
        let fanouts = convert_fanin_to_fanout(fanins);
        
        let inputs = infer_ports(&fanins);
        let outputs = infer_ports(&fanouts);
        
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
        let fanins = [
            /*0*/vec![],
            /*1*/vec![mk_fan(0, 0)],
            /*2*/vec![mk_fan(1, 1),mk_fan(5, 1)],
            /*3*/vec![mk_fan(2, 0)],
            /*4*/vec![],
            /*5*/vec![mk_fan(4, 0),mk_fan(1, 1)],
            /*6*/vec![mk_fan(5, 0)]
        ];

        let correct_latencies = [-1,-1,1,1,0,0,0];

        let fanouts = convert_fanin_to_fanout(&fanins);
        
        let inputs = vec![0, 4];
        let outputs = vec![3, 6];

        let found_latencies = solve_latencies(&fanins, &fanouts, &inputs, &outputs).unwrap();

        assert!(latencies_equal(&found_latencies, &correct_latencies), "{found_latencies:?} =lat= {correct_latencies:?}");
    }
    
    #[test]
    fn check_correct_latency_with_superfluous_input() {
        let fanins = [
            /*0*/vec![],
            /*1*/vec![mk_fan(0, 0)],
            /*2*/vec![mk_fan(1, 1),mk_fan(5, 1)],
            /*3*/vec![mk_fan(2, 0)],
            /*4*/vec![],
            /*5*/vec![mk_fan(4, 0),mk_fan(1, 1),mk_fan(7, 2)],
            /*6*/vec![mk_fan(5, 0)],
            /*7*/vec![] // superfluous input
        ];

        let correct_latencies = [-1,-1,1,1,0,0,0,-2];

        let fanouts = convert_fanin_to_fanout(&fanins);
        
        let inputs = vec![0, 4];
        let outputs = vec![3, 6];

        let found_latencies = solve_latencies(&fanins, &fanouts, &inputs, &outputs).unwrap();

        assert!(latencies_equal(&found_latencies, &correct_latencies), "{found_latencies:?} =lat= {correct_latencies:?}");
    }
    
    #[test]
    fn check_correct_latency_with_superfluous_output() {
        let fanins = [
            /*0*/vec![],
            /*1*/vec![mk_fan(0, 0)],
            /*2*/vec![mk_fan(1, 1),mk_fan(5, 1)],
            /*3*/vec![mk_fan(2, 0)],
            /*4*/vec![],
            /*5*/vec![mk_fan(4, 0),mk_fan(1, 1)],
            /*6*/vec![mk_fan(5, 0)],
            /*7*/vec![mk_fan(5, 2)] // superfluous output
        ];

        let correct_latencies = [-1,-1,1,1,0,0,0,2];

        let fanouts = convert_fanin_to_fanout(&fanins);
        
        let inputs = vec![0, 4];
        let outputs = vec![3, 6];

        let found_latencies = solve_latencies(&fanins, &fanouts, &inputs, &outputs).unwrap();

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

        assert!(matches!(should_be_err, Err(LatencyCountingError::DisjointNodes{start_node: _, nodes_not_reached: _})))
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

        assert!(matches!(should_be_err, Err(LatencyCountingError::PositiveNetLatencyCycle{conflict_nodes: _})))
    }
}

