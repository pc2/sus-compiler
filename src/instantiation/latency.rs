use crate::arena_alloc::FlatAlloc;

use super::{latency_algorithm::FanInOut, RealWire, SubModule, SubModuleIDMarker, WireIDMarker};

struct LatencyComputer {
    
}
impl LatencyComputer {
    fn setup(wires : &FlatAlloc<RealWire, WireIDMarker>, submodules : &FlatAlloc<SubModule, SubModuleIDMarker>) -> Self {
        // Wire to wire Fanin
        let mut fanins : Vec<Vec<FanInOut>> = wires.iter().map(|(id, wire)| {
            let mut fanin = Vec::new();
            wire.source.iter_sources_with_min_latency(&mut |from, delta_latency| {
                fanin.push(FanInOut{other : from.get_hidden_value(), delta_latency});
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
        todo!()
    }
}
