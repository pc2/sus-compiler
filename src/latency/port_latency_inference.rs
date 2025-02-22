use crate::{
    alloc::{zip_eq, UUIDRange},
    flattening::{DeclarationKind, ExpressionSource},
    prelude::*,
    typing::template::TVec,
    value::Value,
};

use crate::flattening::{
    BinaryOperator, Instruction, Port, UnaryOperator, WireReference, WireReferenceRoot,
};

use super::latency_algorithm::ValueToInfer;

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
#[derive(Debug)]
pub enum EdgeInfo<ID> {
    Known {
        offset: i64,
    },
    Inferrable {
        id: ID,
        multiplier: i64,
        offset: i64,
    },
    Poison,
}

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
            offset: to.const_factor - from.const_factor,
        };

        for (id, a, b) in zip_eq(&from.arg_linear_factor, &to.arg_linear_factor) {
            let multiplier = *b - *a;
            if multiplier != 0 {
                let EdgeInfo::Known { offset } = result else {
                    result = EdgeInfo::Poison; // Offset was already by multiple template vars
                    break;
                };
                result = EdgeInfo::Inferrable {
                    id,
                    multiplier,
                    offset,
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
struct LatencyInferenceCandidate {
    from: PortID,
    to: PortID,
    offset: i64,
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
    ) -> FlatAlloc<Vec<(PortID, PortID, EdgeInfo<LatencyCountInferenceVarID>)>, DomainIDMarker>
    {
        let mut updated_port_linearities = self.port_latency_linearities.clone();
        for (_, l) in &mut updated_port_linearities {
            if let Some(ll) = &mut l.latency_linearity {
                ll.update_with_known_variables(known_template_args)
            }
        }

        let mut local_variables = known_template_args.map(|_| None);

        domains.map(|d| {
            let mut result: Vec<(PortID, PortID, EdgeInfo<LatencyCountInferenceVarID>)> =
                Vec::new();
            for (from_id, from) in &updated_port_linearities {
                if from.domain != d || !from.is_input {
                    continue; // ports on different domains cannot be related in latency counting
                }
                for (to_id, to) in &updated_port_linearities {
                    if to.domain != d || to.is_input {
                        continue; // ports on different domains cannot be related in latency counting
                    }

                    result.push((
                        from_id,
                        to_id,
                        if let (Some(from_linearity), Some(to_linearity)) =
                            (&from.latency_linearity, &to.latency_linearity)
                        {
                            match PortLatencyLinearity::is_pair_latency_candidate(
                                from_linearity,
                                to_linearity,
                            ) {
                                EdgeInfo::Known { offset } => EdgeInfo::Known { offset },
                                EdgeInfo::Inferrable {
                                    id: template_var,
                                    multiplier,
                                    offset,
                                } => EdgeInfo::Inferrable {
                                    id: *local_variables[template_var].get_or_insert_with(|| {
                                        latency_inference_variables
                                            .alloc(ValueToInfer::new((submodule_id, template_var)))
                                    }),
                                    multiplier,
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
            result
        })
    }

    /*   let mut inference_candidates = FlatAlloc::with_size(num_template_args, Vec::new());

        for (from, from_info) in port_infos.iter_valids() {
            for (to, to_info) in port_infos.iter_valids() {
                if ports[from].domain != ports[to].domain {
                    continue; // ports on different domains cannot be related in latency counting
                }
                if let Some((template_id, offset)) =
                    PortLatencyLinearity::is_pair_latency_candidate(from_info, to_info)
                {
                    inference_candidates[template_id].push(LatencyInferenceCandidate {
                        from,
                        to,
                        offset,
                    });
                }
            }
        }

        PortLatencyInferenceInfo {
            inference_candidates,
        }
    }

    /// To infer a specific specific variable, we look at all
    pub fn try_infer_var(
        &self,
        template_var: TemplateID,
        port_latencies: &FlatAlloc<Option<i64>, PortIDMarker>,
    ) -> Option<i64> {
        let mut inferred_variable_value = i64::MAX;

        for candidate in &self.inference_candidates[template_var] {
            let from_latency = port_latencies[candidate.from]?;
            let to_latency = port_latencies[candidate.to]?;

            let this_pair_infers_to = to_latency - from_latency - candidate.offset;

            inferred_variable_value = i64::min(inferred_variable_value, this_pair_infers_to);
        }

        (inferred_variable_value != i64::MAX).then_some(inferred_variable_value)
    }*/
}

#[cfg(test)]
mod tests {
    use crate::{
        alloc::{FlatAlloc, UUIDRange},
        prelude::{DomainID, PortID, PortIDMarker},
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

                output bool z'5+A
                output bool w'B
            }
        */

        let domains = UUIDRange::new_with_length(1);
        let first_domain = domains.0;
        let mut port_latency_linearities: FlatAlloc<FullPortLatencyLinearity, PortIDMarker> =
            FlatAlloc::with_capacity(4);
        port_latency_linearities.alloc(mk_port_linearity(first_domain, true, 0, vec![0, 0]));
        port_latency_linearities.alloc(mk_port_linearity(first_domain, true, 3, vec![1, 0]));
        port_latency_linearities.alloc(mk_port_linearity(first_domain, false, 5, vec![1, 0]));
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
}
