

use std::{cmp::max, iter::zip};

use crate::{
    arena_alloc::{FlatAlloc, UUIDRange},
    flattening::{FlatIDMarker, Instruction, WriteModifiers},
    instantiation::latency_algorithm::{convert_fanin_to_fanout, solve_latencies, FanInOut, LatencyCountingError}
};

use self::list_of_lists::ListOfLists;

use super::*;

struct PathMuxSource<'s> {
    to_wire : &'s RealWire,
    to_latency : i64,
    mux_input : &'s MultiplexerSource
}

fn write_path_elem_to_string(result : &mut String, decl_name : &str, to_absolute_latency : i64, prev_absolute_latency : i64) {
    use std::fmt::Write;

    let delta_latency = to_absolute_latency - prev_absolute_latency;

    let plus_sign = if delta_latency >= 0 {"+"} else {""};

    writeln!(result, "-> {decl_name}'{to_absolute_latency} ({plus_sign}{delta_latency})").unwrap();
}

fn make_path_info_string(writes : &[PathMuxSource<'_>], from_latency : i64, from_name : &str) -> String {
   let mut prev_decl_absolute_latency = from_latency;
    let mut result = format!("{from_name}'{prev_decl_absolute_latency}\n");

    for wr in writes {
        let decl_name = &wr.to_wire.name;

        let to_absolute_latency = wr.to_latency;
        
        write_path_elem_to_string(&mut result, &decl_name, to_absolute_latency, prev_decl_absolute_latency);

        prev_decl_absolute_latency = to_absolute_latency;
    }

    result
}

fn filter_unique_write_flats<'w>(writes : &'w [PathMuxSource<'w>], instructions : &'w FlatAlloc<Instruction, FlatIDMarker>) -> Vec<&'w crate::flattening::Write> {
    let mut result : Vec<&'w crate::flattening::Write> = Vec::new();
    for w in writes {
        if let Instruction::Write(original_write) = &instructions[w.mux_input.from.original_connection] {
            if !result.iter().any(|found_write| std::ptr::eq(*found_write, original_write)) {result.push(original_write)}
        }
    }
    result
}

struct WireToLatencyMap {
    map_wire_to_latency_node : FlatAlloc<usize, WireIDMarker>,
    domain_infos : FlatAlloc<LatencyDomainInfo, DomainIDMarker>
}

/// A map from the latency nodes (usize) to the original objects that created them
/// 
/// Up for future reworking to add other things latency nodes can refer to. 
/// Such as split latencies
enum LatencyMeaning {
    Wire(WireID)
}

impl LatencyMeaning {
    #[track_caller]
    fn unwrap_wire(&self) -> WireID {
        let Self::Wire(w) = self;
        *w
    }
}

struct LatencyDomainInfo {
    latency_node_meanings : Vec<LatencyMeaning>,
    initial_values : Vec<SpecifiedLatency>,
    input_ports : Vec<usize>,
    output_ports : Vec<usize>
}

impl WireToLatencyMap {
    fn compute(wires : &FlatAlloc<RealWire, WireIDMarker>, domains : UUIDRange<DomainIDMarker>, interface_ports : &FlatAlloc<Option<InstantiatedPort>, PortIDMarker>) -> Self {
        const PLACEHOLDER : usize = usize::MAX;
    
        let mut map_wire_to_latency_node : FlatAlloc<usize, WireIDMarker> = wires.iter().map(|_| PLACEHOLDER).collect();
        
        let mut domain_infos : FlatAlloc<LatencyDomainInfo, DomainIDMarker> = domains.iter().map(|domain| {
            let mut latency_node_meanings = Vec::with_capacity(wires.len());
            let mut initial_values = Vec::new();

            for (w_id, w) in wires {
                if w.domain == domain {
                    let new_idx = latency_node_meanings.len();
                    latency_node_meanings.push(LatencyMeaning::Wire(w_id));
                    debug_assert!(map_wire_to_latency_node[w_id] == PLACEHOLDER);
                    map_wire_to_latency_node[w_id] = new_idx;
    
                    if w.absolute_latency != CALCULATE_LATENCY_LATER {
                        initial_values.push(SpecifiedLatency { wire: new_idx, latency: w.absolute_latency });
                    }
                }
            }
    
            LatencyDomainInfo{latency_node_meanings, initial_values, input_ports : Vec::new(), output_ports : Vec::new()}
        }).collect();

        for (_id, p) in interface_ports.iter_valids() {
            let domain_to_edit = &mut domain_infos[p.interface];
            let latency_node = map_wire_to_latency_node[p.wire];
            if p.is_input {
                &mut domain_to_edit.input_ports
            } else {
                &mut domain_to_edit.output_ports
            }.push(latency_node);
        }

        // Every wire has been covered
        debug_assert!(map_wire_to_latency_node.iter().all(|(_id, v)| *v != PLACEHOLDER));

        Self {
            map_wire_to_latency_node,
            domain_infos
        }
    }
}

impl<'fl, 'l> InstantiationContext<'fl, 'l> {
    fn iter_sources_with_min_latency<F : FnMut(WireID, i64)>(&self, wire_source : &RealWireDataSource, mut f : F) {
        match wire_source {
            RealWireDataSource::ReadOnly => {}
            RealWireDataSource::OutPort { sub_module_id, port_id } => {
                let sub_mod = &self.submodules[*sub_module_id];
                let Some(inst) = &sub_mod.instance else {return}; // This module hasn't been instantiated yet
                let this_output_port = inst.interface_ports[*port_id].as_ref().unwrap();
                assert!(!this_output_port.is_input);

                for (input_port_id, attached_wire) in sub_mod.port_map.iter_valids() {
                    let input_port = inst.interface_ports[input_port_id].as_ref().unwrap(); // Non-present port should have been caught once the submodule was instantiated
                    if !input_port.is_input {continue} // Skip other outputs
                    if input_port.interface != this_output_port.interface {continue} // Skip ports of different interfaces

                    f(attached_wire.maps_to_wire, this_output_port.absolute_latency - input_port.absolute_latency);
                } 
            }
            RealWireDataSource::Multiplexer { is_state: _, sources } => {
                for s in sources {
                    f(s.from.from, s.from.num_regs);
                    RealWirePathElem::for_each_wire_in_path(&s.to_path, |w| {f(w, s.from.num_regs)});
                    if let Some(c) = s.from.condition {
                        f(c, s.from.num_regs);
                    }
                }
            }
            RealWireDataSource::UnaryOp { op: _, right } => {
                f(*right, 0);
            }
            RealWireDataSource::BinaryOp { op: _, left, right } => {
                f(*left, 0);
                f(*right, 0);
            }
            RealWireDataSource::Select { root, path } => {
                f(*root, 0);
                RealWirePathElem::for_each_wire_in_path(path, |w| {f(w, 0)});
            }
            RealWireDataSource::Constant { value: _ } => {}
        }
    }

    fn make_fanins(&self, latency_node_mapper : &WireToLatencyMap, latency_node_to_wire_map : &[LatencyMeaning], domain_id : DomainID) -> ListOfLists<FanInOut> {
        let mut fanins : ListOfLists<FanInOut> = ListOfLists::new_with_groups_capacity(latency_node_to_wire_map.len());
        
        // Wire to wire Fanin
        for target in latency_node_to_wire_map {
            fanins.new_group();
            match target {
                LatencyMeaning::Wire(wire_id) => {
                    self.iter_sources_with_min_latency(&self.wires[*wire_id].source, |from, delta_latency| {
                        assert!(self.wires[from].domain == domain_id);
                        fanins.push_to_last_group(FanInOut{other : latency_node_mapper.map_wire_to_latency_node[from], delta_latency});
                    });
                }
            }
        }

        fanins
    }

    // Returns a proper interface if all ports involved did not produce an error. If a port did produce an error then returns None. 
    // Computes all latencies involved
    pub fn compute_latencies(&mut self) {
        let latency_node_mapper = WireToLatencyMap::compute(&self.wires, self.md.domains.id_range(), &self.interface_ports);

        for (domain_id, domain_info) in &latency_node_mapper.domain_infos {
            let fanins = self.make_fanins(&latency_node_mapper, &domain_info.latency_node_meanings, domain_id);
            
            // Process fanouts
            let fanouts = convert_fanin_to_fanout(&fanins);
            
            match solve_latencies(&fanins, &fanouts, &domain_info.input_ports, &domain_info.output_ports, domain_info.initial_values.clone()) {
                Ok(latencies) => {
                    for (_id, (node, lat)) in zip(domain_info.latency_node_meanings.iter(), latencies.iter()).enumerate() {
                        let wire = &mut self.wires[node.unwrap_wire()];
                        wire.absolute_latency = *lat;
                        if *lat == CALCULATE_LATENCY_LATER {
                            let source_location = self.md.get_instruction_span(wire.original_instruction);
                            self.errors.error(source_location, format!("Latency Counting couldn't reach this node"));
                        }
                    }
                }
                Err(err) => {
                    self.report_error(&domain_info.latency_node_meanings, err);
                }
            };
            
            // Compute needed_untils
            for (latency_idx, wire_id) in domain_info.latency_node_meanings.iter().enumerate() {
                let wire = &self.wires[wire_id.unwrap_wire()];
                let mut needed_until = wire.absolute_latency;
                for target_fanout in &fanouts[latency_idx] {
                    let target_wire = &self.wires[domain_info.latency_node_meanings[target_fanout.other].unwrap_wire()];
                    
                    needed_until = max(needed_until, target_wire.absolute_latency);
                }
                self.wires[wire_id.unwrap_wire()].needed_until = needed_until;
            }
        }
            
        // Finally update interface absolute latencies
        for (_id, port) in self.interface_ports.iter_valids_mut() {
            port.absolute_latency = self.wires[port.wire].absolute_latency;
        }
    }

    fn gather_all_mux_inputs(&self, latency_node_meanings : &[LatencyMeaning], conflict_iter : &[SpecifiedLatency]) -> Vec<PathMuxSource<'_>> {
        let mut connection_list = Vec::new();
        for window in conflict_iter.windows(2) {
            let [from, to] = window else {unreachable!()};
            let LatencyMeaning::Wire(from_wire_id) = latency_node_meanings[from.wire];
            //let from_wire = &self.wires[from_wire_id];
            let LatencyMeaning::Wire(to_wire_id) = latency_node_meanings[to.wire];
            let to_wire = &self.wires[to_wire_id];
            let RealWireDataSource::Multiplexer { is_state:_, sources } = &to_wire.source else {continue}; // We can only name multiplexers
    
            for s in sources {
                let mut predecessor_found = false;
                let mut predecessor_adder = |source| {
                    if source == from_wire_id {
                        predecessor_found = true;
                    }
                };
                predecessor_adder(s.from.from);
                RealWirePathElem::for_each_wire_in_path(&s.to_path, predecessor_adder);
                if predecessor_found {
                    connection_list.push(PathMuxSource{to_wire, mux_input : s, to_latency : to.latency});
                }
            }
        }
        connection_list
    }

    fn report_error(&self, latency_node_meanings : &[LatencyMeaning], err: LatencyCountingError) {
        match err {
            LatencyCountingError::NetPositiveLatencyCycle { conflict_path, net_roundtrip_latency } => {
                let writes_involved = self.gather_all_mux_inputs(latency_node_meanings, &conflict_path);
                assert!(!writes_involved.is_empty());
                let (first_write, later_writes) = writes_involved.split_first().unwrap();
                let first_write_desired_latency = first_write.to_latency + net_roundtrip_latency;
                let mut path_message = make_path_info_string(later_writes, first_write.to_latency, &first_write.to_wire.name);
                write_path_elem_to_string(&mut path_message, &first_write.to_wire.name, first_write_desired_latency, writes_involved.last().unwrap().to_latency);
                let unique_write_instructions = filter_unique_write_flats(&writes_involved, &self.md.instructions);
                let rest_of_message = format!(" part of a net-positive latency cycle of +{net_roundtrip_latency}\n\n{path_message}\nWhich conflicts with the starting latency");
        
                let mut did_place_error = false;
                for wr in &unique_write_instructions {
                    match wr.write_modifiers {
                        WriteModifiers::Connection { num_regs, regs_span } => {
                            if num_regs >= 1 {
                                did_place_error = true;
                                let this_register_plural = if num_regs == 1 {"This register is"} else {"These registers are"};
                                self.errors.error(regs_span, format!("{this_register_plural}{rest_of_message}"));
                            }
                        }
                        WriteModifiers::Initial{initial_kw_span : _} => {unreachable!("Initial assignment can only be from compile-time constant. Cannot be part of latency loop. ")}
                    }
                }
                // Fallback if no register annotations used
                if !did_place_error {
                    for wr in unique_write_instructions {
                        self.errors.error(wr.to_span, format!("This write is{rest_of_message}"));
                    }
                }
            }
            LatencyCountingError::IndeterminablePortLatency { bad_ports } => {
                for port in bad_ports {
                    let port_decl = self.md.instructions[self.wires[latency_node_meanings[port.0].unwrap_wire()].original_instruction].unwrap_wire_declaration();
                    self.errors.error(port_decl.name_span, format!("Cannot determine port latency. Options are {} and {}\nTry specifying an explicit latency or rework the module to remove this ambiguity", port.1, port.2));
                }
            }
            LatencyCountingError::ConflictingSpecifiedLatencies { conflict_path } => {
                let start_wire = &self.wires[latency_node_meanings[conflict_path.first().unwrap().wire].unwrap_wire()];
                let end_wire = &self.wires[latency_node_meanings[conflict_path.last().unwrap().wire].unwrap_wire()];
                let start_decl = self.md.instructions[start_wire.original_instruction].unwrap_wire_declaration();
                let end_decl = self.md.instructions[end_wire.original_instruction].unwrap_wire_declaration();
                let end_latency_decl = self.md.instructions[end_decl.latency_specifier.unwrap()].unwrap_wire();
        
    
                let writes_involved = self.gather_all_mux_inputs(latency_node_meanings, &conflict_path);
                let path_message = make_path_info_string(&writes_involved, start_wire.absolute_latency, &start_wire.name);
                //assert!(!writes_involved.is_empty());
    
                let end_name = &end_wire.name;
                let specified_end_latency = end_wire.absolute_latency;
                self.errors
                    .error(end_latency_decl.span, format!("Conflicting specified latency\n\n{path_message}\nBut this was specified as {end_name}'{specified_end_latency}"))
                    .info_obj_same_file(start_decl);
            }
        }
    }
}
