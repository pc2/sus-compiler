use std::{iter::zip, collections::VecDeque};

use crate::arena_alloc::FlatAlloc;

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

impl LatencyComputer {
    fn setup(wires : &FlatAlloc<RealWire, WireIDMarker>, submodules : &FlatAlloc<SubModule, SubModuleIDMarker>) -> Self {
        // Wire to wire Fanin
        let mut fanins : FlatAlloc<Vec<FanInOut>, WireIDMarker> = wires.iter().map(|(id, wire)| {
            let mut fanin = Vec::new();
            wire.source.iter_sources_with_min_latency(|from, delta_latency| {
                fanin.push(FanInOut{other : from, delta_latency});
            });
            fanin
        }).collect();

        // Submodules Fanin
        for (_id, sub_mod) in submodules {
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
        }

        // Process fanouts
        let mut fanouts : FlatAlloc<Vec<FanInOut>, WireIDMarker> = wires.iter().map(|(id, wire)| {
            Vec::new()
        }).collect();

        for (id, fin) in &fanins {
            for f in fin {
                fanouts[f.other].push(FanInOut { other: id, delta_latency: f.delta_latency })
            }
        }

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
    eliminated : bool,
    maps_to : WireID
}
struct GraphDecycler<'f> {
    runtime_data : FlatAlloc<RuntimeData, WireIDMarker>,
    fanins : &'f FlatAlloc<Vec<FanInOut>, WireIDMarker>
}

impl<'f> GraphDecycler<'f> {
    fn new(fanins : &FlatAlloc<Vec<FanInOut>, WireIDMarker>) -> GraphDecycler {
        GraphDecycler {
            runtime_data : fanins.iter().map(|(maps_to, _)| RuntimeData{ part_of_path: false, eliminated: false, maps_to }).collect(),
            fanins
        }
    }

    fn is_part_of_cycle(&mut self, id : WireID) -> Option<WireID> {
        if self.runtime_data[id].eliminated {return None;}

        if self.runtime_data[id].part_of_path {
            // TODO Handle start removing cycle
            return Some(id); // New node was part of path, remove it!
        }
        
        self.runtime_data[id].part_of_path = true;

        for fi in &self.fanins[id] {
            if let Some(cycle_root) = self.is_part_of_cycle(fi.other) {
                if id == cycle_root {
                    // We have returned to the root
                    // Cycle is now removed
                    // So we just continue
                } else {
                    // Part of the chain towards the root
                    return Some(cycle_root)
                }
            }
        }

        self.runtime_data[id].eliminated = true; // Once we finish a node, eliminate it
        None
    }

    fn eliminate_cycles(&mut self) {
        for (id, wire_fanin) in self.fanins {
            if wire_fanin.is_empty() {
                // New root to iterate from
                self.is_part_of_cycle(id);
            }

            
        }

        todo!()
    }
}
