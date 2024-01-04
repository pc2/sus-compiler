use std::{iter::zip, collections::VecDeque};

use crate::{arena_alloc::FlatAlloc, errors::ErrorCollector};

use super::{WireID, WireIDMarker, RealWire, SubModule, SubModuleIDMarker};




struct FanInOut {
    other : WireID,
    delta_latency : i64
}

/*
    Algorithm:
    Initialize all inputs at latency 0
    Perform full forward pass, making latencies the maximum of all incoming latencies
    Then backward pass, moving nodes forward in latency as much as possible. 
    Only moving forward is possible, and only when not confliciting with a later node
*/
struct LatencyComputer {
    fanins : FlatAlloc<Vec<FanInOut>, WireIDMarker>,
    fanouts : FlatAlloc<Vec<FanInOut>, WireIDMarker>
}

fn convert_fanin_to_fanout(fanins : &FlatAlloc<Vec<FanInOut>, WireIDMarker>) -> FlatAlloc<Vec<FanInOut>, WireIDMarker> {
    let mut fanouts : FlatAlloc<Vec<FanInOut>, WireIDMarker> = fanins.iter().map(|_| {
        Vec::new()
    }).collect();

    for (id, fin) in fanins {
        for f in fin {
            fanouts[f.other].push(FanInOut { other: id, delta_latency: f.delta_latency })
        }
    }

    fanouts
}

impl LatencyComputer {
    fn setup(wires : &FlatAlloc<RealWire, WireIDMarker>, submodules : &FlatAlloc<SubModule, SubModuleIDMarker>) -> Self {
        // Wire to wire Fanin
        let mut fanins : FlatAlloc<Vec<FanInOut>, WireIDMarker> = wires.iter().map(|(id, wire)| {
            let mut fanin = Vec::new();
            wire.source.iter_sources_with_min_latency(&mut |from, delta_latency| {
                fanin.push(FanInOut{other : from, delta_latency});
            });
            fanin
        }).collect();

        // Submodules Fanin
        assert!(submodules.is_empty());
        /*for (_id, sub_mod) in submodules {
            // All submodules must be fully valid
            assert!(!sub_mod.instance.errors.did_error());
            let sub_mod_interface = sub_mod.instance.interface.as_deref().unwrap();
            for (input_wire, input_port) in zip(&sub_mod.wires, sub_mod_interface) {
                if !input_port.is_input {continue;}
                for (output_wire, output_port) in zip(&sub_mod.wires, sub_mod_interface) {
                    if output_port.is_input {continue;}
                    
                    let delta_latency = output_port.absolute_latency - input_port.absolute_latency;

                    fanins[*output_wire].push(FanInOut{other: *input_wire, delta_latency});
                }
            }
        }*/

        // Process fanouts
        let fanouts = convert_fanin_to_fanout(&fanins);

        Self {fanins, fanouts}
    }

    fn compute_latencies_forward(&self) -> FlatAlloc<i64, WireIDMarker> {
        let mut latencies : FlatAlloc<i64, WireIDMarker> = self.fanins.iter().map(|_| 0).collect();

        let mut queue : VecDeque<WireID> = VecDeque::new();
        queue.reserve(self.fanins.len());

        let mut order : Vec<WireID> = Vec::new();
        order.reserve(self.fanins.len());

        for (id, v) in &self.fanouts {
            if v.is_empty() {
                queue.push_back(id);
                latencies[id] = 1; // Initialize with 1
            }
        }

        while let Some(s) = queue.pop_front() {
            let mut all_explored = false;
            for from in &self.fanins[s] {

            }
        }

        latencies
    }
}

struct RuntimeData {
    part_of_path : bool,
    current_absolute_latency : i64
}

fn process_node_recursive(runtime_data : &mut FlatAlloc<RuntimeData, WireIDMarker>, fanouts : &FlatAlloc<Vec<FanInOut>, WireIDMarker>, cur_node : WireID) {
    runtime_data[cur_node].part_of_path = true;
    for &FanInOut{other, delta_latency} in &fanouts[cur_node] {
        let to_node_min_latency = runtime_data[cur_node].current_absolute_latency + delta_latency;
        if to_node_min_latency > runtime_data[other].current_absolute_latency {
            if runtime_data[other].part_of_path {
                todo!("Cycles for positive net latency error!");
            } else {
                runtime_data[other].current_absolute_latency = to_node_min_latency;
                process_node_recursive(runtime_data, fanouts, other);
            }
        }
    }
    runtime_data[cur_node].part_of_path = false;
}

fn find_all_cycles_starting_from(fanouts : &FlatAlloc<Vec<FanInOut>, WireIDMarker>, starting_node : WireID) -> FlatAlloc<RuntimeData, WireIDMarker> {
    let mut runtime_data : FlatAlloc<RuntimeData, WireIDMarker> = fanouts.iter().map(|_| RuntimeData{
        part_of_path: false,
        current_absolute_latency : i64::MIN // Such that new nodes will always be overwritten
    }).collect();

    runtime_data[starting_node].current_absolute_latency = 0;
    process_node_recursive(&mut runtime_data, fanouts, starting_node);
    runtime_data
}
