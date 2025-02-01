use crate::{
    alloc::zip_eq,
    flattening::{DeclarationKind, ExpressionSource},
    prelude::*,
    typing::template::TVec,
    value::Value,
};

use super::{BinaryOperator, Instruction, Port, UnaryOperator, WireReference, WireReferenceRoot};

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

/// A candidate from which a template variable could be inferred
///
/// Basically, this struct is formed for a pair of ports like this:
///
/// portA'4 -> portB'L-3
///
/// That would create a inference candidate for template var 'L' with an offset of 7. (L == 7 iff portA' - portB' == 0)
#[derive(Debug, Clone)]
struct LatencyInferenceCandidate {
    template_id: TemplateID,
    from: PortID,
    to: PortID,
    offset: i64,
}

struct PortLatencyLinearity {
    offset: i64,
    arg_linear_factor: TVec<i64>,
}
impl PortLatencyLinearity {
    fn is_const(&self) -> bool {
        self.arg_linear_factor.iter().all(|(_, v)| *v == 0)
    }
    /// Checks if the two latency annotations are offset by a constant and exactly 1x a template variable
    fn is_pair_latency_candidate(from: &Self, to: &Self) -> Option<(TemplateID, i64)> {
        let mut found_var = None;

        for (template_var_id, a, b) in zip_eq(&from.arg_linear_factor, &to.arg_linear_factor) {
            if *b - *a == 1 {
                if found_var.is_some() {
                    return None;
                } // Offset by multiple template vars
                found_var = Some(template_var_id)
            }
        }

        found_var.map(|v| (v, to.offset - from.offset))
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
            right_v.offset = -right_v.offset;
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
                    left_v.offset += right_v.offset;
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
                    left_v.offset -= right_v.offset;
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
                        left_v.offset *= right_v.offset;
                        for (_, a) in &mut left_v.arg_linear_factor {
                            *a *= right_v.offset;
                        }
                        Some(left_v)
                    }
                }
                BinaryOperator::Divide => (left_v.is_const() && right_v.is_const()).then(|| {
                    left_v.offset /= right_v.offset;
                    left_v
                }),
                BinaryOperator::Modulo => (left_v.is_const() && right_v.is_const()).then(|| {
                    left_v.offset %= right_v.offset;
                    left_v
                }),
                _other => None,
            }
        }
        ExpressionSource::Constant(Value::Integer(i)) => {
            let offset: i64 = i.try_into().ok()?;
            Some(PortLatencyLinearity {
                offset,
                arg_linear_factor: TVec::with_size(num_template_args, 0),
            })
        }
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
                offset: 0,
                arg_linear_factor: TVec::with_size(num_template_args, 0),
            };
            result.arg_linear_factor[decl_template_id] = 1;
            Some(result)
        }
        _other => None,
    }
}

#[derive(Default, Debug)]
pub struct PortLatencyInferenceInfo {
    //port_latency_groups: Vec<Vec<PortGroup>>,
    inference_candidates: Vec<LatencyInferenceCandidate>,
}

pub fn make_port_latency_inference_info(
    ports: &FlatAlloc<Port, PortIDMarker>,
    instructions: &FlatAlloc<Instruction, FlatIDMarker>,
    num_template_args: usize,
) -> PortLatencyInferenceInfo {
    let port_infos = ports.map(|(_port_id, port)| {
        let decl = instructions[port.declaration_instruction].unwrap_declaration();

        decl.latency_specifier.and_then(|latency_spec| {
            recurse_down_expression(instructions, latency_spec, num_template_args)
        })
    });

    let mut inference_candidates = Vec::new();

    for (from, from_info) in port_infos.iter_valids() {
        for (to, to_info) in port_infos.iter_valids() {
            if let Some((template_id, offset)) =
                PortLatencyLinearity::is_pair_latency_candidate(from_info, to_info)
            {
                inference_candidates.push(LatencyInferenceCandidate {
                    template_id,
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
