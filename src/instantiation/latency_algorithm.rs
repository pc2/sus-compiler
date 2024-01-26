use std::iter::zip;


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
fn count_latency_recursive(part_of_path : &mut [bool], absolute_latency : &mut [i64], fanouts : &Vec<Vec<FanInOut>>, cur_node : usize) {
    part_of_path[cur_node] = true;
    for &FanInOut{other, delta_latency} in &fanouts[cur_node] {
        let to_node_min_latency = absolute_latency[cur_node] + delta_latency;
        if to_node_min_latency > absolute_latency[other] {
            if part_of_path[other] {
                todo!("Cycles for positive net latency error!");
            } else {
                absolute_latency[other] = to_node_min_latency;
                count_latency_recursive(part_of_path, absolute_latency, fanouts, other);
            }
        }
    }
    part_of_path[cur_node] = false;
}

fn count_latency(part_of_path : &mut [bool], absolute_latency : &mut [i64], fanouts : &Vec<Vec<FanInOut>>, start_node : usize, start_value : i64) -> Option<()> {
    for p in part_of_path.iter() {assert!(!*p);}

    if absolute_latency[start_node] != i64::MIN {
        if absolute_latency[start_node] == start_value {
            Some(()) // Return with no error, latency is already set and is correct value
        } else {
            todo!("Report latency error");
            None // Latency error. One of the ends has a different new latency!
        }
    } else {
        absolute_latency[start_node] = start_value;
        count_latency_recursive(part_of_path, absolute_latency, fanouts, start_node);
    
        for p in part_of_path.iter() {assert!(!*p);}
        Some(())
    }
}

fn solve_latencies(fanins : &Vec<Vec<FanInOut>>) -> Option<Box<[i64]>> {
    let fanouts_holder = convert_fanin_to_fanout(fanins);
    let fanouts = &fanouts_holder;

    let inputs : Vec<usize> = fanins.iter().enumerate().filter_map(|(idx, v)| v.is_empty().then_some(idx)).collect();
    let outputs : Vec<usize> = fanouts.iter().enumerate().filter_map(|(idx, v)| v.is_empty().then_some(idx)).collect();

    let mut part_of_path : Box<[bool]> = fanouts.iter().map(|_| false).collect();
    let mut absolute_latencies_forward : Box<[i64]> = fanouts.iter().map(|_| i64::MIN).collect();
    let mut absolute_latencies_backward : Box<[i64]> = fanouts.iter().map(|_| i64::MIN).collect();

    let Some(starting_node) = inputs.get(0) else {todo!("Output-only modules")};
    
    absolute_latencies_backward[*starting_node] = 0; // Provide a seed to start the algorithm
    let mut last_num_valid_inputs = 0;
    loop {
        let mut num_valid_inputs = 0;
        // Copy over latencies from backward pass
        for input in &inputs {
            if absolute_latencies_backward[*input] != i64::MIN { // Once an extremity node has been assigned, its value can never change
                count_latency(&mut part_of_path, &mut absolute_latencies_forward, fanouts, *input, -absolute_latencies_backward[*input])?;
                num_valid_inputs += 1;
            }
        }

        // Check end conditions
        if num_valid_inputs == inputs.len() {
            break; // All inputs covered. Done! 
        }
        if num_valid_inputs == last_num_valid_inputs {
            // No change, we can't expore further, but haven't seen all inputs. 
            todo!("Proper error for disjoint inputs and outputs");
            return None;
        }
        last_num_valid_inputs = num_valid_inputs;

        // Copy over latencies from forward pass
        for output in &outputs {
            if absolute_latencies_forward[*output] != i64::MIN {
                count_latency(&mut part_of_path, &mut absolute_latencies_backward, fanins, *output, -absolute_latencies_forward[*output])?;
            }
        }
    }
    Some(absolute_latencies_forward)
}

fn convert_fanin_to_fanout(fanins : &Vec<Vec<FanInOut>>) -> Vec<Vec<FanInOut>> {
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

#[cfg(test)]
mod tests {
    use super::*;

    fn mk_fan(other : usize, delta_latency : i64) -> FanInOut {
        FanInOut{other, delta_latency}
    }
    
    #[test]
    fn check_correct_latency_basic() {
        let graph = vec![
            /*0*/vec![],
            /*1*/vec![mk_fan(0, 0)],
            /*2*/vec![mk_fan(1, 1),mk_fan(5, 1)],
            /*3*/vec![mk_fan(2, 0)],
            /*4*/vec![],
            /*5*/vec![mk_fan(4, 0),mk_fan(1, 1)],
            /*6*/vec![mk_fan(5, 0)]
        ];

        let correct_latencies = vec![-1,-1,1,1,0,0,0];

        let found_latencies = solve_latencies(&graph).unwrap();

        assert!(latencies_equal(&found_latencies, &correct_latencies));
    }
}

