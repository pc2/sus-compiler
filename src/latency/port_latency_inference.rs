use crate::{
    alloc::{zip_eq, UUIDRange},
    flattening::{DeclarationKind, ExpressionSource},
    instantiation::SubModulePort,
    prelude::*,
    typing::template::TVec,
    value::Value,
};

use crate::flattening::{
    BinaryOperator, Instruction, Port, UnaryOperator, WireReference, WireReferenceRoot,
};

use super::latency_algorithm::{
    FanInOut, LatencyCountingPorts, LatencyInferenceCandidate, ValueToInfer,
};

/*/// ports whose latency annotations require them to be at fixed predefined offsets
///
/// For example: portA'3, portB'7, portC'2
///
/// But also: portX'L+2, portY'L-2
///
/// These are in separate groups
///
/// See [PortGroupConnection]
#[derive(Debug, Clone)]
struct PortGroup {
    port: PortID,
    relative_latency_offset: i64,
}*/
#[derive(Debug, Clone, Copy)]
pub enum EdgeInfo<ID> {
    Known {
        delta_latency: i64,
    },
    /// |from| + value_of(id) * multiplier + offset = |to|
    Inferrable {
        target_to_infer: ID,
        multiply_var_by: i64,
        offset: i64,
    },
    Poison,
}

/// A candidate from which a template variable could be inferred
///
/// Basically, this struct is formed for a pair of ports like this:
///
/// portA'4 -> portB'L-3
///
/// That would create a inference candidate for template var 'L' with an offset of 7. (L == 7 iff |portA| - |portB| == 0)
///
/// So |from| + offset + L == |to|
///
/// L = |to| - |from| - offset
#[derive(Debug, Clone)]
struct PortLatencyLinearity {
    const_factor: i64,
    arg_linear_factor: TVec<i64>,
}
impl PortLatencyLinearity {
    fn is_const(&self) -> bool {
        self.arg_linear_factor.iter().all(|(_, v)| *v == 0)
    }
    /// Checks if the two latency annotations are offset by a constant and exactly 1x a template variable
    fn is_pair_latency_candidate(from: &Self, to: &Self) -> EdgeInfo<TemplateID> {
        let mut result = EdgeInfo::Known {
            delta_latency: to.const_factor - from.const_factor,
        };

        for (target_to_infer, a, b) in zip_eq(&from.arg_linear_factor, &to.arg_linear_factor) {
            let multiplier = *b - *a;
            if multiplier != 0 {
                let EdgeInfo::Known { delta_latency } = result else {
                    result = EdgeInfo::Poison; // Offset was already by multiple template vars
                    break;
                };
                result = EdgeInfo::Inferrable {
                    target_to_infer,
                    multiply_var_by: multiplier,
                    offset: delta_latency,
                }
            }
        }

        result
    }

    fn update_with_known_variables(
        &mut self,
        known_template_args: &FlatAlloc<Option<i64>, TemplateIDMarker>,
    ) {
        for (_, factor, known) in zip_eq(
            self.arg_linear_factor.iter_mut(),
            known_template_args.iter(),
        ) {
            if let Some(known_value) = known {
                self.const_factor += *factor * known_value;
                *factor = 0;
            }
        }
    }
}
fn recurse_down_expression(
    instructions: &FlatAlloc<Instruction, FlatIDMarker>,
    cur_instr: FlatID,
    num_template_args: usize,
) -> Option<PortLatencyLinearity> {
    match &instructions[cur_instr].unwrap_expression().source {
        ExpressionSource::UnaryOp {
            op: UnaryOperator::Negate,
            right,
        } => {
            let mut right_v = recurse_down_expression(instructions, *right, num_template_args)?;
            right_v.const_factor = -right_v.const_factor;
            for (_, v) in &mut right_v.arg_linear_factor {
                *v = -*v;
            }
            Some(right_v)
        }
        ExpressionSource::BinaryOp { op, left, right } => {
            let mut left_v = recurse_down_expression(instructions, *left, num_template_args)?;
            let mut right_v = recurse_down_expression(instructions, *right, num_template_args)?;
            match op {
                BinaryOperator::Add => {
                    left_v.const_factor += right_v.const_factor;
                    for ((_, a), (_, b)) in left_v
                        .arg_linear_factor
                        .iter_mut()
                        .zip(right_v.arg_linear_factor.iter())
                    {
                        *a += *b;
                    }
                    Some(left_v)
                }
                BinaryOperator::Subtract => {
                    left_v.const_factor -= right_v.const_factor;
                    for ((_, a), (_, b)) in left_v
                        .arg_linear_factor
                        .iter_mut()
                        .zip(right_v.arg_linear_factor.iter())
                    {
                        *a -= *b;
                    }
                    Some(left_v)
                }
                BinaryOperator::Multiply => {
                    if !left_v.is_const() && !right_v.is_const() {
                        None
                    } else {
                        if left_v.is_const() {
                            std::mem::swap(&mut left_v, &mut right_v);
                        }
                        left_v.const_factor *= right_v.const_factor;
                        for (_, a) in &mut left_v.arg_linear_factor {
                            *a *= right_v.const_factor;
                        }
                        Some(left_v)
                    }
                }
                BinaryOperator::Divide => (left_v.is_const() && right_v.is_const()).then(|| {
                    left_v.const_factor /= right_v.const_factor;
                    left_v
                }),
                BinaryOperator::Modulo => (left_v.is_const() && right_v.is_const()).then(|| {
                    left_v.const_factor %= right_v.const_factor;
                    left_v
                }),
                _other => None,
            }
        }
        ExpressionSource::Constant(Value::Integer(i)) => Some(PortLatencyLinearity {
            const_factor: i.try_into().ok()?,
            arg_linear_factor: TVec::with_size(num_template_args, 0),
        }),
        ExpressionSource::WireRef(WireReference {
            root: WireReferenceRoot::LocalDecl(decl_id, _span),
            path,
            is_generative,
        }) => {
            assert!(is_generative);
            if !path.is_empty() {
                return None;
            }
            let DeclarationKind::GenerativeInput(decl_template_id) =
                instructions[*decl_id].unwrap_declaration().decl_kind
            else {
                return None;
            };
            let mut result = PortLatencyLinearity {
                const_factor: 0,
                arg_linear_factor: TVec::with_size(num_template_args, 0),
            };
            result.arg_linear_factor[decl_template_id] = 1;
            Some(result)
        }
        _other => None,
    }
}

#[derive(Debug, Clone)]
struct FullPortLatencyLinearity {
    domain: DomainID,
    is_input: bool,
    latency_linearity: Option<PortLatencyLinearity>,
}

/// The basic way latency count inference works is as follows:
/// On the module interface, ports may be marked with latency annotations.
/// These annotations can be simple constants (portA'0, portB'-3, etc),
/// or larger expressions involving some template parameter, such as portC'L, portD'L+3-2
///
/// Whereever there is a difference in the latency annitation between two ports of exactly
/// 1x a variable + some constant offset, the port pair becomes eligible for latency inference
///
/// When the module is flattened, we can immediately construct for every template parameter,
/// a list of all port pairs that may be used to infer the value of this parameter.
///
/// Once we come to actually performing said inference [Self::try_infer_var], we take the list
/// of absolute latencies we know for these ports, and take the minimum latency we could find.
/// This ensuresport_latency_linearities that instantiating the module cannot ever expand beyond the context in which
/// it is inferred. Finally, all
#[derive(Default, Debug, Clone)]
pub struct PortLatencyInferenceInfo {
    //port_latency_groups: Vec<Vec<PortGroup>>,
    //inference_candidates: TVec<Vec<LatencyInferenceCandidate>>,
    port_latency_linearities: FlatAlloc<FullPortLatencyLinearity, PortIDMarker>,
}

impl PortLatencyInferenceInfo {
    pub fn make(
        ports: &FlatAlloc<Port, PortIDMarker>,
        instructions: &FlatAlloc<Instruction, FlatIDMarker>,
        num_template_args: usize,
    ) -> PortLatencyInferenceInfo {
        Self {
            port_latency_linearities: ports.map(|(_port_id, port)| {
                let decl = instructions[port.declaration_instruction].unwrap_declaration();

                let latency_linearity = decl.latency_specifier.and_then(|latency_spec| {
                    recurse_down_expression(instructions, latency_spec, num_template_args)
                });

                FullPortLatencyLinearity {
                    domain: port.domain,
                    is_input: port.is_input,
                    latency_linearity,
                }
            }),
        }
    }

    /// We already use the values for the template arguments we know to work out as much of the LC graph as possible
    pub fn get_inference_edges<ID: Copy>(
        &self,
        known_template_args: &TVec<Option<i64>>,
        domains: UUIDRange<DomainIDMarker>,
        submodule_id: ID,
        latency_inference_variables: &mut FlatAlloc<
            ValueToInfer<(ID, TemplateID)>,
            LatencyCountInferenceVarIDMarker,
        >,
    ) -> FlatAlloc<InferenceEdgesForDomain, DomainIDMarker> {
        let mut updated_port_linearities = self.port_latency_linearities.clone();
        for (_, l) in &mut updated_port_linearities {
            if let Some(ll) = &mut l.latency_linearity {
                ll.update_with_known_variables(known_template_args)
            }
        }

        let mut local_variables = known_template_args.map(|_| None);

        domains.map(|d| {
            let mut edges: Vec<(PortID, PortID, EdgeInfo<LatencyCountInferenceVarID>)> = Vec::new();
            for (from_id, from) in &updated_port_linearities {
                if from.domain != d || !from.is_input {
                    continue; // ports on different domains cannot be related in latency counting
                }
                for (to_id, to) in &updated_port_linearities {
                    if to.domain != d || to.is_input {
                        continue; // ports on different domains cannot be related in latency counting
                    }

                    edges.push((
                        from_id,
                        to_id,
                        if let (Some(from_linearity), Some(to_linearity)) =
                            (&from.latency_linearity, &to.latency_linearity)
                        {
                            match PortLatencyLinearity::is_pair_latency_candidate(
                                from_linearity,
                                to_linearity,
                            ) {
                                EdgeInfo::Known { delta_latency } => {
                                    EdgeInfo::Known { delta_latency }
                                }
                                EdgeInfo::Inferrable {
                                    target_to_infer,
                                    multiply_var_by: multiplier,
                                    offset,
                                } => EdgeInfo::Inferrable {
                                    target_to_infer: *local_variables[target_to_infer]
                                        .get_or_insert_with(|| {
                                            latency_inference_variables.alloc(ValueToInfer::new((
                                                submodule_id,
                                                target_to_infer,
                                            )))
                                        }),
                                    multiply_var_by: multiplier,
                                    offset,
                                },
                                EdgeInfo::Poison => EdgeInfo::Poison,
                            }
                        } else {
                            EdgeInfo::Poison
                        },
                    ));
                }
            }
            InferenceEdgesForDomain { edges }
        })
    }
}

#[derive(Debug)]
pub struct InferenceEdgesForDomain {
    edges: Vec<(PortID, PortID, EdgeInfo<LatencyCountInferenceVarID>)>,
}

/// Gathers inference info per domain.
///
/// So for each module, the parts of it's subdomains must be added
/// to the collector associated with the global domain this domain is connected to
pub struct PerDomainInferenceInfo {
    pub inference_edges: Vec<LatencyInferenceCandidate>,
    pub extra_fanin: Vec<Vec<FanInOut>>,
    pub ports: LatencyCountingPorts,
}

impl PerDomainInferenceInfo {
    pub fn new(num_latency_counting_nodes: usize, ports: LatencyCountingPorts) -> Self {
        Self {
            inference_edges: Vec::new(),
            extra_fanin: vec![Vec::new(); num_latency_counting_nodes],
            ports,
        }
    }
}

impl InferenceEdgesForDomain {
    pub fn apply_to_global_domain(
        &self,
        port_to_wire_map: &FlatAlloc<Option<SubModulePort>, PortIDMarker>,
        wire_to_node_map: &FlatAlloc<usize, WireIDMarker>, // These mappings are just due to implementation details of instantiation
        this_domain_inference_info: &mut PerDomainInferenceInfo,
        ports_range: UUIDRange<PortIDMarker>,
    ) {
        let mut port_has_already_been_registered = ports_range.map(|_| false);

        for (from, to, edge_info) in &self.edges {
            let (Some(from_port), Some(to_port)) =
                (&port_to_wire_map[*from], &port_to_wire_map[*to])
            else {
                continue; // Can't infer based on ports that aren't used
            };
            let from_node = wire_to_node_map[from_port.maps_to_wire];
            let to_node = wire_to_node_map[to_port.maps_to_wire];

            match *edge_info {
                EdgeInfo::Known { delta_latency } => {
                    this_domain_inference_info.extra_fanin[to_node].push(FanInOut {
                        to_node: from_node, // Because it's fan-in
                        delta_latency: Some(delta_latency),
                    })
                }
                EdgeInfo::Poison => {
                    this_domain_inference_info.extra_fanin[to_node].push(FanInOut {
                        to_node: from_node, // Because it's fan-in
                        delta_latency: None,
                    })
                }
                EdgeInfo::Inferrable {
                    target_to_infer,
                    multiply_var_by,
                    offset,
                } => {
                    this_domain_inference_info
                        .inference_edges
                        .push(LatencyInferenceCandidate {
                            multiply_var_by,
                            from_node,
                            to_node,
                            offset,
                            target_to_infer,
                        });
                    if !std::mem::replace(&mut port_has_already_been_registered[*from], true) {
                        this_domain_inference_info.ports.push(from_node, false);
                        // Module inputs are outputs outside, of course
                    }
                    if !std::mem::replace(&mut port_has_already_been_registered[*to], true) {
                        this_domain_inference_info.ports.push(to_node, true); // Module outputs are inputs outside, of course
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        alloc::{FlatAlloc, UUIDRange},
        instantiation::SubModulePort,
        latency::{
            latency_algorithm::{
                infer_unknown_latency_edges, mk_fan, FanInOut, LatencyCountingPorts,
            },
            list_of_lists::ListOfLists,
            port_latency_inference::PerDomainInferenceInfo,
        },
        prelude::{DomainID, PortIDMarker, WireID},
    };

    use super::{FullPortLatencyLinearity, PortLatencyInferenceInfo, PortLatencyLinearity};

    fn mk_port_linearity(
        domain: DomainID,
        is_input: bool,
        const_factor: i64,
        arg_factors: Vec<i64>,
    ) -> FullPortLatencyLinearity {
        FullPortLatencyLinearity {
            domain,
            is_input,
            latency_linearity: Some(PortLatencyLinearity {
                const_factor,
                arg_linear_factor: FlatAlloc::from_vec(arg_factors),
            }),
        }
    }

    #[test]
    fn test_get_inference_edges() {
        /*
            module #(int A, int B) {
                input bool x'0
                input bool y'3+A

                output bool z'5+3*A
                output bool w'B
            }
        */

        let domains = UUIDRange::new_with_length(1);
        let first_domain = domains.0;
        let mut port_latency_linearities: FlatAlloc<FullPortLatencyLinearity, PortIDMarker> =
            FlatAlloc::with_capacity(4);
        port_latency_linearities.alloc(mk_port_linearity(first_domain, true, 0, vec![0, 0]));
        port_latency_linearities.alloc(mk_port_linearity(first_domain, true, 3, vec![1, 0]));
        port_latency_linearities.alloc(mk_port_linearity(first_domain, false, 5, vec![3, 0]));
        port_latency_linearities.alloc(mk_port_linearity(first_domain, false, 0, vec![0, 1]));

        let latency_info = PortLatencyInferenceInfo {
            port_latency_linearities,
        };

        let known_template_args = FlatAlloc::from_vec(vec![None, None]);

        let mut variables = FlatAlloc::new();

        let result =
            latency_info.get_inference_edges(&known_template_args, domains, (), &mut variables);

        dbg!(&result);
    }

    #[test]
    fn test_infernce_end_to_end() {
        // Outer scope
        /*
                2 -\?A
               /      6
              1-3 -/?B \
             /          8
            0 - 4 -\?C /|
            | \       7 |
            |   5 -/?B  |
            ------------|
        */
        let fanins: [&[FanInOut]; 9] = [
            /*0*/ &[],
            /*1*/ &[mk_fan(0, 0)],
            /*2*/ &[mk_fan(1, 1)],
            /*3*/ &[mk_fan(1, 6)],
            /*4*/ &[mk_fan(0, 2)],
            /*5*/ &[mk_fan(0, 5)],
            /*6*/ &[], // inference_edge(2) for A, inference_edge(3) for B
            /*7*/ &[], // inference_edge(4) for C, inference_edge(5) for B
            /*8*/ &[mk_fan(6, 3), mk_fan(7, 2), mk_fan(0, 10)],
        ];
        let fanins = ListOfLists::from_slice_slice(&fanins);

        let ports = LatencyCountingPorts::from_inputs_outputs(&[0], &[8]);

        // Inner scope, to be inferred

        /*
            domain x
            2: input'-A
            3: input'-B
            6: output'0

            domain y
            4: input'-C+3
            5: input'-B
            7: output'0
        */

        let domains = UUIDRange::new_with_length(2);
        let mut domains_iter = domains.iter();
        let domain_x = domains_iter.next().unwrap();
        let domain_y = domains_iter.next().unwrap();

        let mut port_latency_linearities: FlatAlloc<FullPortLatencyLinearity, PortIDMarker> =
            FlatAlloc::with_capacity(6);
        port_latency_linearities.alloc(mk_port_linearity(domain_x, true, 0, vec![-1, 0, 0])); // 2
        port_latency_linearities.alloc(mk_port_linearity(domain_x, true, 0, vec![0, -1, 0])); // 3
        port_latency_linearities.alloc(mk_port_linearity(domain_y, true, 3, vec![0, 0, -1])); // 4'C+3, which makes C 3 smaller than in [latency_algorithm::tests::test_inference_no_poison]
        port_latency_linearities.alloc(mk_port_linearity(domain_y, true, 0, vec![0, -1, 0])); // 5
        port_latency_linearities.alloc(mk_port_linearity(domain_x, false, 0, vec![0, 0, 0])); // 6
        port_latency_linearities.alloc(mk_port_linearity(domain_y, false, 0, vec![0, 0, 0])); // 7

        let latency_info = PortLatencyInferenceInfo {
            port_latency_linearities,
        };

        let known_template_args = FlatAlloc::from_vec(vec![None, None, None]);

        let mut values_to_infer = FlatAlloc::new();

        let per_domain_edges = latency_info.get_inference_edges(
            &known_template_args,
            domains,
            (),
            &mut values_to_infer,
        );

        // Because both domains of the submodule are part of the outer domain, we just merge both

        let port_to_wire_map = FlatAlloc::from_vec(vec![2, 3, 4, 5, 6, 7]).map(|(_, v)| {
            Some(SubModulePort {
                maps_to_wire: WireID::from_hidden_value(*v),
                name_refs: Vec::new(),
            })
        });
        let wire_to_node_map = FlatAlloc::from_vec(vec![0, 1, 2, 3, 4, 5, 6, 7, 7, 8]);

        let mut domain_inference_info = PerDomainInferenceInfo::new(fanins.len(), ports);
        for (_, this_domain_edges) in &per_domain_edges {
            this_domain_edges.apply_to_global_domain(
                &port_to_wire_map,
                &wire_to_node_map,
                &mut domain_inference_info,
                latency_info.port_latency_linearities.id_range(),
            );
        }

        let expected_inputs = [0, 6, 7]; // Inputs needed for inference
        let expected_outputs = [8, 2, 3, 4, 5]; // Outputs needed for inferece

        assert_eq!(domain_inference_info.ports.inputs(), expected_inputs);
        assert_eq!(domain_inference_info.ports.outputs(), expected_outputs);

        let specified_latencies = [];

        infer_unknown_latency_edges(
            &fanins,
            &domain_inference_info.ports,
            &specified_latencies,
            &domain_inference_info.inference_edges,
            &mut values_to_infer,
        )
        .unwrap();

        assert_eq!(
            values_to_infer.map(|(_, v)| v.inferred_value),
            FlatAlloc::from_vec(vec![Some(6), Some(1), Some(3)]) // C 3 smaller due to offset on port 4
        );
    }
}
