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

    is_latency_pinned[start_node] = true;

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

    // Forwards are all performed in the same block. This block is then also used as the output latencies
    absolute_latencies_forward : Vec<i64>,
    
    precomputed_backwards_nodes : Vec<i64>,

    // To find input latencies based on output latencies, we use a separate block to go backwards. 
    // These are done one at a time, such that we can find conflicting latencies. 
    absolute_latencies_backward_temporary : Vec<i64>,
    touched_backwards : Vec<bool>,

    input_was_covered : Vec<bool>,
    output_was_covered : Vec<bool>,

    exactly_specified_nodes : Vec<usize>
}

impl<'d> LatencySolver<'d> {
    /*
        The user may specify an latency on any wire.
        If they do on an input, then we can build off of those latencies. 
        If they don't, then we should pick an arbitrary input and set it to latency 0. 
        We start counting from there. 

        TODO what if there's no inputs? 

        Right now, the user *has* to specify at least one input if they specify anything else. 

        Once it returns, at least one input has a latency specified. 
        Returns the initial list of forward starting points
    */
    fn new(fanins : &'d [Vec<FanInOut>], fanouts : &'d [Vec<FanInOut>], inputs : &'d [usize], outputs : &'d [usize], mut initial_specified_latencies : Vec<i64>) -> Result<Self, LatencyCountingError> {
        let mut is_latency_pinned = vec![false; fanouts.len()];

        let mut exactly_specified_nodes : Vec<usize> = 
            initial_specified_latencies
            .iter()
            .enumerate()
            .filter_map(|(idx, val)| (*val != i64::MIN).then_some(idx))
            .collect();

        if exactly_specified_nodes.is_empty() {
            let arbitrary_node = if let Some(first_input) = inputs.first() {
                *first_input
            } else if let Some(first_output) = outputs.first() {
                *first_output
            } else {
                // If neither an input, nor an output is available, then pick a random node. 
                // TODO check that this is valid reasoning
                0
            };
            initial_specified_latencies[arbitrary_node] = 0;
            exactly_specified_nodes.push(arbitrary_node)
        }

        for sp in &exactly_specified_nodes {
            is_latency_pinned[*sp] = true;
        }

        let mut precomputed_backwards_nodes : Vec<i64> = initial_specified_latencies.iter().map(|lat| if *lat == i64::MIN {i64::MIN} else {-*lat}).collect();

        // Attempt at minor optimization. Didn't work :(
        let input_was_covered = inputs.iter().map(|wire| initial_specified_latencies[*wire] != i64::MIN).collect();
        let output_was_covered = outputs.iter().map(|wire| initial_specified_latencies[*wire] != i64::MIN).collect();
        //let input_was_covered = vec![false; inputs.len()];
        //let output_was_covered = vec![false; outputs.len()];

        let mut absolute_latencies_forward = initial_specified_latencies;

        let mut stack = Vec::new();
        for initial in &exactly_specified_nodes {
            count_latency(&mut is_latency_pinned, &mut precomputed_backwards_nodes, fanins, *initial, &mut stack)?;
        }
        for initial in &exactly_specified_nodes {
            count_latency(&mut is_latency_pinned, &mut absolute_latencies_forward, fanouts, *initial, &mut stack)?;
        }

        let touched_backwards = precomputed_backwards_nodes.iter().map(|latency| *latency != i64::MIN).collect();

        // Initialize main buffers
        let result = Self{
            fanins,
            fanouts,
            inputs,
            outputs,
            exactly_specified_nodes,
            stack,
            is_latency_pinned,
            precomputed_backwards_nodes,
            absolute_latencies_forward,
            absolute_latencies_backward_temporary : vec![i64::MIN; fanouts.len()],
            touched_backwards,
            input_was_covered,
            output_was_covered
        };

        Ok(result)
    }
    
    fn solve_latencies(&mut self) -> Result<(), LatencyCountingError> {
        // Add initial backwards pass, but don't include them in new_start_nodes yet. This allows us to still detect it when the user specifies a undeterminable middle node as only starting point
        for input in self.inputs {
            if self.precomputed_backwards_nodes[*input] != i64::MIN {
                if self.absolute_latencies_forward[*input] == i64::MIN {
                    self.absolute_latencies_forward[*input] = -self.precomputed_backwards_nodes[*input];
                } else {
                    assert_eq!(self.absolute_latencies_forward[*input], -self.precomputed_backwards_nodes[*input]);
                }
            }
        }

        loop {
            // Find new backwards starting nodes
            let mut bad_ports = Vec::new();
            for (output, was_covered) in zip(self.outputs.iter(), self.output_was_covered.iter_mut()) {
                if self.absolute_latencies_forward[*output] == i64::MIN {continue} // Can't use this output to 
                if *was_covered {continue} // Little optimization, so we only every cover a backwards latency once
                *was_covered = true;

                // Reset temporary buffer
                for (t_b, v) in zip(self.touched_backwards.iter_mut(), self.absolute_latencies_backward_temporary.iter_mut()) {
                    if *v != i64::MIN {
                        *t_b = true;
                    }
                    //*v = i64::MIN;
                }
                self.absolute_latencies_backward_temporary.copy_from_slice(&self.precomputed_backwards_nodes);
                // new latency
                assert!(self.absolute_latencies_backward_temporary[*output] == i64::MIN || self.absolute_latencies_backward_temporary[*output] == -self.absolute_latencies_forward[*output]);
                //assert!(self.is_latency_pinned[*output] == false);
                self.is_latency_pinned[*output] = true;
                self.absolute_latencies_backward_temporary[*output] = -self.absolute_latencies_forward[*output];
                count_latency(&mut self.is_latency_pinned, &mut self.absolute_latencies_backward_temporary, self.fanins, *output, &mut self.stack)?;
                
                for input in self.inputs {
                    let assignment = &mut self.absolute_latencies_forward[*input];
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
            }

            if !bad_ports.is_empty() {
                return Err(LatencyCountingError::ConflictingPortLatency{bad_ports})
            }

            // Add newly discovered input assignments
            let mut new_nodes_added = false;
            for (input, was_covered) in zip(self.inputs.iter(), self.input_was_covered.iter_mut()) {
                if self.absolute_latencies_forward[*input] == i64::MIN {continue}
                if *was_covered {continue}; // Optimization, we only need to add each valid input once
                *was_covered = true;
                count_latency(&mut self.is_latency_pinned, &mut self.absolute_latencies_forward, self.fanouts, *input, &mut self.stack)?;
                new_nodes_added = true;
            }
            if !new_nodes_added {break}
        }

        Ok(())
    }

    fn extract_solution(mut self) -> Result<Vec<i64>, LatencyCountingError> {
        // Also add nodes in fanin not dependent on an input to this input-output cluster. 
        // Nodes in fanout are included implicitly due to forward being the default direction
        invert_latency(&mut self.absolute_latencies_forward);
        for (start_node, fanin_of_output) in self.touched_backwards.iter().enumerate() {
            if *fanin_of_output && (self.absolute_latencies_forward[start_node] != i64::MIN) {
                count_latency(&mut self.is_latency_pinned, &mut self.absolute_latencies_forward, self.fanins, start_node, &mut self.stack)?;
            }
        }
        invert_latency(&mut self.absolute_latencies_forward);

        let nodes_not_reached : Vec<usize> = self.absolute_latencies_forward.iter().enumerate().filter_map(|(idx, v)| (*v == i64::MIN).then_some(idx)).collect();
        if nodes_not_reached.is_empty() {
            Ok(self.absolute_latencies_forward)
        } else {
            Err(LatencyCountingError::DisjointNodes{start_node: self.exactly_specified_nodes[0], nodes_not_reached})
        }
    }
}

pub fn solve_latencies(fanins : &[Vec<FanInOut>], fanouts : &[Vec<FanInOut>], inputs : &[usize], outputs : &[usize], initial_latencies : Vec<i64>) -> Result<Vec<i64>, LatencyCountingError> {
    assert_eq!(fanins.len(), fanouts.len());
    assert_eq!(fanins.len(), initial_latencies.len());

    for i in inputs {
        assert!(fanins[*i].is_empty());
    }
    // Actually, outputs *can* have fanout. You can still use the value of an output to compute other stuff
    // Inputs however, cannot have fanin
    /*for o in outputs {
        assert!(fanouts[*o].is_empty());
    }*/
    
    let mut latency_solver = LatencySolver::new(fanins, fanouts, inputs, outputs, initial_latencies)?;
    
    latency_solver.solve_latencies()?;
    latency_solver.extract_solution()
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

    fn solve_latencies_infer_ports(fanins : &[Vec<FanInOut>], specified_latencies : Vec<i64>) -> Result<Vec<i64>, LatencyCountingError> {
        let fanouts = convert_fanin_to_fanout(fanins);
        
        let inputs = infer_ports(&fanins);
        let outputs = infer_ports(&fanouts);
        
        solve_latencies(fanins, &fanouts, &inputs, &outputs, specified_latencies)
    }

    fn latencies_equal(a : &[i64], b : &[i64]) -> bool {
        assert_eq!(a.len(), b.len());
    
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

        let found_latencies = solve_latencies(&fanins, &fanouts, &inputs, &outputs, vec![i64::MIN; fanins.len()]).unwrap();

        assert!(latencies_equal(&found_latencies, &correct_latencies), "{found_latencies:?} =lat= {correct_latencies:?}");
    }
    
    #[test]
    fn check_correct_latency_backwards() {
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

        let mut specified_latencies = vec![i64::MIN; fanins.len()];
        specified_latencies[6] = 0;

        let found_latencies = solve_latencies(&fanins, &fanouts, &inputs, &outputs, specified_latencies).unwrap();

        assert_eq!(found_latencies, correct_latencies);
    }
    
    #[test]
    fn check_correct_latency_from_any_start_node() {
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

        for starting_node in 0..7 {
            let mut specified_latencies = vec![i64::MIN; fanins.len()];
            specified_latencies[starting_node] = 0;

            let found_latencies = solve_latencies(&fanins, &fanouts, &inputs, &outputs, specified_latencies).unwrap();

            assert!(latencies_equal(&found_latencies, &correct_latencies), "{found_latencies:?} =lat= {correct_latencies:?}");
        }
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

        let found_latencies = solve_latencies(&fanins, &fanouts, &inputs, &outputs, vec![i64::MIN; fanins.len()]).unwrap();

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

        let found_latencies = solve_latencies(&fanins, &fanouts, &inputs, &outputs, vec![i64::MIN; fanins.len()]).unwrap();

        assert!(latencies_equal(&found_latencies, &correct_latencies), "{found_latencies:?} =lat= {correct_latencies:?}");
    }
    
    #[test]
    fn check_conflicting_port_latency() {
        let fanins = [
            /*0*/vec![],
            /*1*/vec![mk_fan(0, 0)],
            /*2*/vec![mk_fan(1, 3),mk_fan(5, 1)],
            /*3*/vec![mk_fan(2, 0)],
            /*4*/vec![],
            /*5*/vec![mk_fan(4, 0),mk_fan(1, 1)],
            /*6*/vec![mk_fan(5, 0)]
        ];

        let should_be_err = solve_latencies_infer_ports(&fanins, vec![i64::MIN; fanins.len()]);

        assert!(matches!(should_be_err, Err(LatencyCountingError::ConflictingPortLatency{bad_ports:_})))
    }
    
    #[test]
    #[ignore = "Won't pass yet. Still need to rework how inputs and outputs are explored. Make it more symmetrical"]
    fn check_conflicting_port_latency_with_any_starting_node_does_error() {
        let fanins = [
            /*0*/vec![],
            /*1*/vec![mk_fan(0, 0)],
            /*2*/vec![mk_fan(1, 3),mk_fan(5, 1)],
            /*3*/vec![mk_fan(2, 0)],
            /*4*/vec![],
            /*5*/vec![mk_fan(4, 0),mk_fan(1, 1)],
            /*6*/vec![mk_fan(5, 0)]
        ];

        for start_node in 0..7 {
            println!("start_node: {start_node}");
            let mut specified_latencies = vec![i64::MIN; fanins.len()];
            specified_latencies[start_node] = 0;
    
            solve_latencies_infer_ports(&fanins, specified_latencies).unwrap_err();
        }
    }
    
    #[test]
    fn check_conflicting_port_latency_resolved() {
        let fanins = [
            /*0*/vec![],
            /*1*/vec![mk_fan(0, 0)],
            /*2*/vec![mk_fan(1, 3),mk_fan(5, 1)],
            /*3*/vec![mk_fan(2, 0)],
            /*4*/vec![],
            /*5*/vec![mk_fan(4, 0),mk_fan(1, 1)],
            /*6*/vec![mk_fan(5, 0)]
        ];

        let mut specified_latencies = vec![i64::MIN; fanins.len()];
        specified_latencies[0] = 0;
        specified_latencies[4] = 2;
        let found_latencies = solve_latencies_infer_ports(&fanins, specified_latencies).unwrap();

        let correct_latencies = [0,0,3,3,2,2,2];

        assert_eq!(found_latencies, correct_latencies); // Can even do a strict check here, because we defined some of the latencies
    }
    
    #[test]
    fn check_disjoint() {
        let fanins = [
            /*0*/vec![],
            /*1*/vec![mk_fan(0, 0)],
            /*2*/vec![mk_fan(1, 3)],
            /*3*/vec![mk_fan(2, 0)],
            /*4*/vec![],
            /*5*/vec![mk_fan(4, 0)],
            /*6*/vec![mk_fan(5, 0)]
        ];

        let should_be_err = solve_latencies_infer_ports(&fanins, vec![i64::MIN; fanins.len()]);

        assert!(matches!(should_be_err, Err(LatencyCountingError::DisjointNodes{start_node: _, nodes_not_reached: _})))
    }
    
    #[test]
    fn check_bad_cycle() {
        let fanins = [
            /*0*/vec![],
            /*1*/vec![mk_fan(0, 0), mk_fan(4, -4)],
            /*2*/vec![mk_fan(1, 3)],
            /*3*/vec![mk_fan(2, 0)],
            /*4*/vec![mk_fan(2, 2)],
        ];

        let should_be_err = solve_latencies_infer_ports(&fanins, vec![i64::MIN; fanins.len()]);

        assert!(matches!(should_be_err, Err(LatencyCountingError::PositiveNetLatencyCycle{conflict_nodes: _})))
    }
}

