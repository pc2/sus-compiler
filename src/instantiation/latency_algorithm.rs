

use crate::config::config;

use super::list_of_lists::ListOfLists;



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

#[derive(Debug, Clone, Copy)]
pub struct FanInOut {
    pub other : usize,
    pub delta_latency : i64
}

pub fn convert_fanin_to_fanout(fanins : &ListOfLists<FanInOut>) -> ListOfLists<FanInOut> {
    ListOfLists::from_random_access_iterator(
        fanins.len(),
        fanins.iter_flattened_by_bucket().map(|(bucket, &FanInOut{ other, delta_latency })| (other, FanInOut{other:bucket, delta_latency : -delta_latency}))
    )
}

struct LatencyStackElem<'d> {
    node_idx : usize,
    remaining_fanout : std::slice::Iter<'d, FanInOut>
}

// Attempt 3 for Latency Counting
// TODO make this only take up 8 bytes with bitfield
#[derive(Clone, Copy)]
struct LatencyNode{
    abs_lat : i64,
    pinned : bool
}

impl LatencyNode {
    const UNSET : LatencyNode = LatencyNode{abs_lat : i64::MIN, pinned : false};

    fn get(&self) -> i64 {
        assert!(self.is_set());
        self.abs_lat
    }
    fn get_maybe(&self) -> i64 {
        self.abs_lat
    }
    fn pin(&mut self) {
        assert!(!self.pinned);
        self.pinned = true;
    }
    fn unpin(&mut self) {
        assert!(self.pinned);
        self.pinned = false;
    }
    fn is_pinned(&self) -> bool {
        self.pinned
    }
    fn is_set(&self) -> bool {
        self.abs_lat != i64::MIN
    }
    fn new_pinned(abs_lat : i64) -> LatencyNode {
        assert!(abs_lat != i64::MIN);
        LatencyNode{abs_lat, pinned : true}
    }
    fn assert_is_set(&self) {
        assert!(self.is_set());
    }
    fn assert_is_set_and_pinned(&self) {
        assert!(self.is_set());
        assert!(self.pinned);
    }
    fn update_and_pin<const BACKWARDS : bool>(&mut self, from : LatencyNode, delta : i64) -> LatencyNodeUpdate {
        from.assert_is_set();
        assert!(delta != i64::MIN);

        let new_latency = from.abs_lat - delta;
        let should_update = if BACKWARDS {
            new_latency < self.abs_lat || self.abs_lat == i64::MIN
        } else {
            new_latency > self.abs_lat || self.abs_lat == i64::MIN
        };
        if should_update {
            if self.pinned {
                LatencyNodeUpdate::ErrorPinnedConflict{new_latency}
            } else {
                self.abs_lat = new_latency;
                self.pinned = true;
                LatencyNodeUpdate::Updated
            }
        } else {
            LatencyNodeUpdate::NoChange
        }
    }
}

enum LatencyNodeUpdate {
    NoChange,
    Updated,
    ErrorPinnedConflict{new_latency : i64}
}

fn clear_unpinned_latencies(working_latencies : &mut [LatencyNode]) {
    for l in working_latencies {
        if !l.pinned {
            l.abs_lat = i64::MIN;
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
/// Requires working_latencies[start_node].is_pinned() == true
/// 
/// Leaves working_latencies[start_node].is_pinned() == true
fn count_latency<'d, const BACKWARDS : bool>(working_latencies : &mut [LatencyNode], fanouts : &'d ListOfLists<FanInOut>, start_node : usize, stack : &mut Vec<LatencyStackElem<'d>>) -> Result<(), LatencyCountingError> {
    working_latencies[start_node].assert_is_set_and_pinned();
    
    assert!(stack.is_empty());

    stack.push(LatencyStackElem{node_idx : start_node, remaining_fanout : fanouts[start_node].iter()});

    while let Some(top) = stack.last_mut() {
        if let Some(&FanInOut{other: to_node, delta_latency}) = top.remaining_fanout.next() {
            working_latencies[top.node_idx].assert_is_set();

            match working_latencies[to_node].update_and_pin::<BACKWARDS>(working_latencies[top.node_idx], delta_latency) {
                LatencyNodeUpdate::NoChange => {} // Nothing
                LatencyNodeUpdate::Updated => {
                    stack.push(LatencyStackElem{node_idx : to_node, remaining_fanout : fanouts[to_node].iter()});
                }
                LatencyNodeUpdate::ErrorPinnedConflict{new_latency} => {
                    // Decide if this is a net positive latency cycle or a conflicting specified latencies error
                    if let Some(conflict_begin) = stack.iter().position(|elem| elem.node_idx == to_node) {
                        let mut conflict_path : Vec<SpecifiedLatency> = 
                            stack[conflict_begin..]
                            .iter()
                            .map(|elem| SpecifiedLatency{wire : elem.node_idx, latency : working_latencies[elem.node_idx].get()})
                            .collect();
                        if BACKWARDS {
                            conflict_path.reverse();
                        }
                        return Err(LatencyCountingError::NetPositiveLatencyCycle {
                            conflict_path,
                            net_roundtrip_latency : new_latency - working_latencies[start_node].get()
                        });
                    } else {
                        assert!(!BACKWARDS, "This should not appear in backwards exploration, because port conflicts should have been found in the forward pass already");
                        
                        let conflict_path = 
                            stack
                            .iter()
                            .map(|elem| SpecifiedLatency{wire : elem.node_idx, latency : working_latencies[elem.node_idx].get()})
                            .chain(std::iter::once(SpecifiedLatency{wire : to_node, latency : new_latency}))
                            .collect();
                        return Err(LatencyCountingError::ConflictingSpecifiedLatencies { conflict_path });
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

fn count_latency_all_in_list<'d, const BACKWARDS : bool>(working_latencies : &mut [LatencyNode], fanouts : &'d ListOfLists<FanInOut>, nodes : &[SpecifiedLatency], stack : &mut Vec<LatencyStackElem<'d>>) -> Result<(), LatencyCountingError> {
    for start_node in nodes {
        working_latencies[start_node.wire].assert_is_set_and_pinned();
    }

    for start_node in nodes {
        working_latencies[start_node.wire].assert_is_set_and_pinned();
        count_latency::<BACKWARDS>(working_latencies, fanouts, start_node.wire, stack)?;
        working_latencies[start_node.wire].assert_is_set_and_pinned();
    }

    for start_node in nodes {
        working_latencies[start_node.wire].assert_is_set_and_pinned();
    }

    Ok(())
}

/// All ports that still have to be assigned a latency are placed in a list of these
/// 
/// When a port first finds a valid latency for itself, it adopts this as it's permanent latency
/// 
/// If it then in the future sees another differing latency assignment for itself, it produces a [LatencyCountingError::IndeterminablePortLatency]
struct PortLatencyCandidate {
    pub wire : usize,
    pub latency_proposal : i64,
    pub is_input : bool,
}

fn inform_all_ports(ports : &mut [PortLatencyCandidate], working_latencies : &[LatencyNode]) -> Result<(), LatencyCountingError> {
    let mut bad_ports = Vec::new();
    for p in ports {
        // Ports to use can't yet be pinned of course
        debug_assert!(!working_latencies[p.wire].is_pinned());
        let found_latency = working_latencies[p.wire].get_maybe();
        if p.latency_proposal == i64::MIN {
            p.latency_proposal = found_latency; // Set first latency
        } else if found_latency != i64::MIN && found_latency != p.latency_proposal {
            // Conflicting latencies
            bad_ports.push((p.wire, p.latency_proposal, found_latency));
        }
    }
    if bad_ports.is_empty() {
        Ok(())
    } else {
        Err(LatencyCountingError::IndeterminablePortLatency { bad_ports })
    }
}

/// Finds a port in the given list that has a defined latency, remove it from the ports list and return it
fn pop_a_port(ports : &mut Vec<PortLatencyCandidate>) -> Option<PortLatencyCandidate> {
    let found_idx = ports.iter().position(|port| port.latency_proposal != i64::MIN)?;
    Some(ports.swap_remove(found_idx))
}

/// All elements in latencies must initially be [LatencyNode::UNSET] or pinned known values
pub fn solve_latencies(fanins : &ListOfLists<FanInOut>, fanouts : &ListOfLists<FanInOut>, inputs : &[usize], outputs : &[usize], mut specified_latencies : Vec<SpecifiedLatency>) -> Result<Vec<i64>, LatencyCountingError> {
    if config().debug_print_latency_graph {
        print_latency_test_case(fanins, inputs, outputs, &specified_latencies);
    }

    if fanins.len() == 0 {
        return Ok(Vec::new())
    }

    // The current set of latencies
    let mut working_latencies = vec![LatencyNode::UNSET; fanins.len()];
    // This stack is reused by [count_latency] calls
    let mut stack = Vec::new();
    // This list contains all ports that still need to be placed. This list gathers port assignments as they happen, 
    // and reports errors if port conflicts arise
    let mut ports_to_place = Vec::with_capacity(inputs.len() + outputs.len());
    
    // If no latencies are given, we have to initialize an arbitrary one ourselves. Prefer input ports over output ports over regular wires
    if specified_latencies.len() == 0 {
        let wire = *inputs.first().unwrap_or(outputs.first().unwrap_or(&0));
        specified_latencies.push(SpecifiedLatency{ wire, latency: 0 });
    }

    // Set up the specified latencies
    for spec_lat in &specified_latencies {
        working_latencies[spec_lat.wire] = LatencyNode::new_pinned(spec_lat.latency);
    }

    
    for i in inputs {
        if !working_latencies[*i].is_pinned() {
            ports_to_place.push(PortLatencyCandidate{ wire: *i, latency_proposal: i64::MIN, is_input: true });
        }
    }
    for o in outputs {
        if !working_latencies[*o].is_pinned() {
            ports_to_place.push(PortLatencyCandidate{ wire: *o, latency_proposal: i64::MIN, is_input: false });
        }
    }

    // First forward run from the initial latency assignment to discover other ports
    count_latency_all_in_list::<false>(&mut working_latencies, &fanouts, &specified_latencies, &mut stack)?;
    inform_all_ports(&mut ports_to_place, &working_latencies)?;
    clear_unpinned_latencies(&mut working_latencies);
    
    // Then backward run
    count_latency_all_in_list::<true>(&mut working_latencies, &fanins, &specified_latencies, &mut stack)?;
    inform_all_ports(&mut ports_to_place, &working_latencies)?;
    clear_unpinned_latencies(&mut working_latencies);

    // Finally, we start specifying each unspecified port in turn, and checking for any conflicts with other ports
    while let Some(chosen_port) = pop_a_port(&mut ports_to_place) {
        working_latencies[chosen_port.wire] = LatencyNode::new_pinned(chosen_port.latency_proposal);

        if chosen_port.is_input {
            count_latency::<false>(&mut working_latencies, &fanouts, chosen_port.wire, &mut stack)?;
        } else {
            count_latency::<true>(&mut working_latencies, &fanins, chosen_port.wire, &mut stack)?;
        }
        inform_all_ports(&mut ports_to_place, &working_latencies)?;
        clear_unpinned_latencies(&mut working_latencies);
    }
    // It may be that some ports are leftover after this while loop. That just means they weren't connected to a port we have seen. 
    // TODO multi-cluster ports

    // Now that we have all the ports, we can fill in the internal latencies
    for idx in 0..working_latencies.len() {
        if working_latencies[idx].is_pinned() { // it's a defined latency!
            count_latency::<false>(&mut working_latencies, &fanouts, idx, &mut stack)?;
        }
    }
    
    // Finally we add in the backwards latencies. TODO maybe be more conservative here?
    for idx in 0..working_latencies.len() {
        if working_latencies[idx].is_set() { // it's a defined latency!
            if !working_latencies[idx].is_pinned() { // Just to avoid the is_pinned check in pin()
                working_latencies[idx].pin();
            }
            count_latency::<true>(&mut working_latencies, &fanins, idx, &mut stack)?;
        }
    }

    Ok(working_latencies.into_iter().map(|v| v.get_maybe()).collect())
}

fn print_latency_test_case(fanins: &ListOfLists<FanInOut>, inputs : &[usize], outputs : &[usize], specified_latencies : &[SpecifiedLatency]) {
    println!("==== BEGIN LATENCY TEST CASE ====");
    println!("#[test]");
    println!("fn new_test_case() {{");
    println!("    let fanins : [&[FanInOut]; {}] = [", fanins.len());
    for (idx, fin) in fanins.iter().enumerate() {
        print!("        /*{idx}*/&[");
        for FanInOut { other, delta_latency } in fin {
            print!("mk_fan({other}, {delta_latency}),")
        }
        println!("],");
    }
    println!("    ];");
    println!("    let fanins = ListOfLists::from_slice_slice(&fanins);");
    println!("    let fanouts = convert_fanin_to_fanout(&fanins);");
    println!("    let inputs = vec!{inputs:?};");
    println!("    let outputs = vec!{outputs:?};");
    println!("    let specified_latencies = vec!{specified_latencies:?};");
    println!("    let found_latencies = solve_latencies(&fanins, &fanouts, &inputs, &outputs, specified_latencies).unwrap();");
    println!("}}");
    println!("==== END LATENCY TEST CASE ====");
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
            if starting_node == 5 {
                let err = solve_latencies(&fanins, &fanouts, &inputs, &outputs, vec![SpecifiedLatency{wire:starting_node,latency:0}]);
                let Err(LatencyCountingError::IndeterminablePortLatency { bad_ports:_ }) = err else {unreachable!("{err:?}")};
                continue;
            }
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
    
    #[test]
    fn input_used_further() {
        let fanins : [&[FanInOut]; 4] = [
            /*0*/&[],
            /*1*/&[mk_fan(0, 1)],
            /*2*/&[mk_fan(1, 1)],
            /*3*/&[mk_fan(2, 1)],
        ];
        let fanins = ListOfLists::from_slice_slice(&fanins);
        let fanouts = convert_fanin_to_fanout(&fanins);

        let latencies = solve_latencies(&fanins, &fanouts, &[0, 1], &[3], Vec::new()).unwrap();

        assert_eq!(latencies, &[0, 1, 2, 3]); 
    }
    
    #[test]
    fn output_used_further() {
        let fanins : [&[FanInOut]; 4] = [
            /*0*/&[],
            /*1*/&[mk_fan(0, 1)],
            /*2*/&[mk_fan(1, 1)],
            /*3*/&[mk_fan(2, 1)],
        ];
        let fanins = ListOfLists::from_slice_slice(&fanins);
        let fanouts = convert_fanin_to_fanout(&fanins);

        let latencies = solve_latencies(&fanins, &fanouts, &[0], &[2, 3], Vec::new()).unwrap();

        assert_eq!(latencies, &[0, 1, 2, 3]); 
    }
    
    #[test]
    fn single_interface_fifo_no_crash() {
        let fanins : [&[FanInOut]; 10] = [
            /*0*/&[mk_fan(3, 0),mk_fan(7, 0),mk_fan(2, 0),],
            /*1*/&[],
            /*2*/&[],
            /*3*/&[],
            /*4*/&[mk_fan(9, 0),mk_fan(1, 0),],
            /*5*/&[mk_fan(8, 0),mk_fan(1, 0),],
            /*6*/&[],
            /*7*/&[],
            /*8*/&[mk_fan(6, 0),mk_fan(7, 0),],
            /*9*/&[mk_fan(0, 0),mk_fan(6, 0),],
        ];
        let fanins = ListOfLists::from_slice_slice(&fanins);
        let fanouts = convert_fanin_to_fanout(&fanins);
        let inputs = vec![1, 2, 3];
        let outputs = vec![4, 5];
        let specified_latencies = vec![];
        let found_latencies = solve_latencies(&fanins, &fanouts, &inputs, &outputs, specified_latencies).unwrap();

        assert_eq!(found_latencies, [0; 10]);
    }

    #[test]
    fn two_interface_fifo_do_crash() {
        let fanins : [&[FanInOut]; 8] = [
            /*0*/&[mk_fan(1, 0),mk_fan(7, 0),mk_fan(2, 0),],
            /*1*/&[],
            /*2*/&[],
            /*3*/&[],
            /*4*/&[mk_fan(3, 0),mk_fan(0, 0),mk_fan(6, 0),],
            /*5*/&[mk_fan(6, 0),mk_fan(7, 0),mk_fan(3, 0),],
            /*6*/&[],
            /*7*/&[],
        ];
        let fanins = ListOfLists::from_slice_slice(&fanins);
        let fanouts = convert_fanin_to_fanout(&fanins);
        let inputs = vec![1, 2, 3];
        let outputs = vec![4, 5];
        let specified_latencies = vec![SpecifiedLatency{ wire: 1, latency: 0 }];
        let found_latencies = solve_latencies(&fanins, &fanouts, &inputs, &outputs, specified_latencies).unwrap();
        
        assert_eq!(found_latencies, [0; 8]);
    }

    #[test]
    fn minimal_crash() {
        let fanins : [&[FanInOut]; 5] = [
            /*0*/&[],
            /*1*/&[mk_fan(0, 0), mk_fan(3, 0)],
            /*2*/&[mk_fan(0, 0), mk_fan(3, 0), mk_fan(4, 0)],
            /*3*/&[],
            /*4*/&[]
        ];
        let fanins = ListOfLists::from_slice_slice(&fanins);
        let fanouts = convert_fanin_to_fanout(&fanins);
        let inputs = vec![0, 4];
        let outputs = vec![1, 2];
        let specified_latencies = vec![SpecifiedLatency{ wire: 4, latency: 0 }];
        let found_latencies = solve_latencies(&fanins, &fanouts, &inputs, &outputs, specified_latencies).unwrap();

        assert_eq!(found_latencies, [0; 5]);
    }

    #[test]
    fn crashing_fifo_use() {
        let fanins : [&[FanInOut]; 10] = [
            /*0*/&[mk_fan(4, 0),],
            /*1*/&[mk_fan(5, 0),],
            /*2*/&[],
            /*3*/&[mk_fan(2, 0),],
            /*4*/&[mk_fan(3, 0),],
            /*5*/&[mk_fan(3, 1),],
            /*6*/&[mk_fan(9, 0),],
            /*7*/&[mk_fan(0, 0),],
            /*8*/&[mk_fan(1, 0),],
            /*9*/&[mk_fan(7, -2),mk_fan(8, -2),],
        ];
        let fanins = ListOfLists::from_slice_slice(&fanins);
        let fanouts = convert_fanin_to_fanout(&fanins);
        let inputs = vec![];
        let outputs = vec![];
        let specified_latencies = vec![SpecifiedLatency{wire : 0, latency : 0}];
        let found_latencies = solve_latencies(&fanins, &fanouts, &inputs, &outputs, specified_latencies).unwrap();
    }
}

