use std::{marker::PhantomData, ops::Deref};

use ibig::IBig;
use sus_proc_macro::get_builtin_type;

use crate::{
    alloc::{UUID, UUIDRange, zip_eq},
    flattening::{DeclarationKind, Direction, ExpressionSource, WrittenType},
    prelude::*,
    typing::{
        abstract_type::PeanoType,
        concrete_type::{ConcreteType, SubtypeRelation},
        domain_type::DomainType,
        template::{Parameter, TVec, TemplateKind},
        value_unifier::UnifyableValue,
    },
    util::partition_in_place,
    value::Value,
};

use crate::flattening::{
    BinaryOperator, Instruction, Port, UnaryOperator, WireReference, WireReferenceRoot,
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
#[derive(Debug, Clone)]
pub enum EdgeInfo {
    ConstantOffset(IBig),
    /// |from| + value_of(id) * multiplier + offset = |to|
    Inferrable {
        target_to_infer: TemplateID,
        multiply_var_by: IBig,
        offset: IBig,
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
struct ParamLinearity {
    const_factor: IBig,
    arg_linear_factor: TVec<IBig>,
}
impl ParamLinearity {
    fn is_const(&self) -> bool {
        self.arg_linear_factor
            .iter()
            .all(|(_, v)| v == &IBig::from(0))
    }
    /// If it's of the form: SomeVar * X + Y, then returns Some((SomeVar, X, Y))
    fn try_into_one_variable(self) -> Option<(TemplateID, IBig, IBig)> {
        let mut found = None;

        for (some_var, x) in self.arg_linear_factor {
            if x != IBig::from(0) {
                if found.is_some() {
                    return None; // Two factors! No dice
                }

                found = Some((some_var, x))
            }
        }

        found.map(|(some_var, x)| (some_var, x, self.const_factor))
    }

    /// Checks if the two latency annotations are offset by a constant and exactly 1x a template variable
    fn is_pair_latency_candidate(from: &Self, to: &Self) -> EdgeInfo {
        let mut result = EdgeInfo::ConstantOffset(&to.const_factor - &from.const_factor);

        for (target_to_infer, a, b) in zip_eq(&from.arg_linear_factor, &to.arg_linear_factor) {
            let multiplier = b - a;
            if multiplier != IBig::from(0) {
                let EdgeInfo::ConstantOffset(offset) = result else {
                    result = EdgeInfo::Poison; // Offset was already by multiple template vars
                    break;
                };
                result = EdgeInfo::Inferrable {
                    target_to_infer,
                    multiply_var_by: multiplier,
                    offset,
                }
            }
        }

        result
    }
}
fn expression_to_param_linearity(
    instructions: &FlatAlloc<Instruction, FlatIDMarker>,
    cur_instr: FlatID,
    template_arg_ids: UUIDRange<TemplateIDMarker>,
) -> Option<ParamLinearity> {
    let expr = instructions[cur_instr].unwrap_subexpression();
    if expr.domain != DomainType::Generative {
        return None; // Early exit, the user can create an invalid interface, we just don't handle it
    }
    if let ExpressionSource::WireRef(WireReference {
        root: WireReferenceRoot::LocalDecl(decl_id),
        path,
        ..
    }) = &expr.source
    {
        if !path.is_empty() {
            return None;
        }
        let DeclarationKind::TemplateParameter(template_id) =
            instructions[*decl_id].unwrap_declaration().decl_kind
        else {
            return None;
        };
        let mut result = ParamLinearity {
            const_factor: IBig::from(0),
            arg_linear_factor: template_arg_ids.map(|_| IBig::from(0)),
        };
        result.arg_linear_factor[template_id] = IBig::from(1);
        return Some(result);
    }

    if !expr.typ.is_int_scalar() {
        return None; // We can only construct more complex linearities for int types, for other types this is sadly the end of the road
    }

    match &expr.source {
        ExpressionSource::UnaryOp {
            op: UnaryOperator::Negate,
            rank,
            right,
        } if rank.deref() == &PeanoType::Zero => {
            let mut right_v =
                expression_to_param_linearity(instructions, *right, template_arg_ids)?;
            right_v.const_factor = -right_v.const_factor;
            for (_, v) in &mut right_v.arg_linear_factor {
                *v = -&*v;
            }
            Some(right_v)
        }
        ExpressionSource::BinaryOp {
            op,
            rank,
            left,
            right,
        } if rank.deref() == &PeanoType::Zero => {
            let mut left_v = expression_to_param_linearity(instructions, *left, template_arg_ids)?;
            let mut right_v =
                expression_to_param_linearity(instructions, *right, template_arg_ids)?;
            match op {
                BinaryOperator::Add => {
                    left_v.const_factor += right_v.const_factor;
                    for ((_, a), (_, b)) in left_v
                        .arg_linear_factor
                        .iter_mut()
                        .zip(right_v.arg_linear_factor.iter())
                    {
                        *a += b;
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
                        *a -= b;
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
                        left_v.const_factor *= &right_v.const_factor;
                        for (_, a) in &mut left_v.arg_linear_factor {
                            *a *= &right_v.const_factor;
                        }
                        Some(left_v)
                    }
                }
                BinaryOperator::Divide => (left_v.is_const()
                    && right_v.is_const()
                    && right_v.const_factor != IBig::from(0))
                .then(|| {
                    left_v.const_factor /= right_v.const_factor;
                    left_v
                }),
                BinaryOperator::Modulo => (left_v.is_const()
                    && right_v.is_const()
                    && right_v.const_factor != IBig::from(0))
                .then(|| {
                    left_v.const_factor %= right_v.const_factor;
                    left_v
                }),
                _other => None,
            }
        }
        ExpressionSource::Literal(Value::Integer(i)) => Some(ParamLinearity {
            const_factor: i.clone(),
            arg_linear_factor: template_arg_ids.map(|_| IBig::from(0)),
        }),
        _other => None,
    }
}

#[derive(Debug, Clone)]
struct FullPortLatencyLinearity {
    domain: DomainID,
    direction: Direction,
    latency_linearity: Option<ParamLinearity>,
}

#[derive(Debug, Clone)]
pub enum SubtypeInferencePathElem {
    /// Go down one level of array, not a terminal elem
    DownArray,
    /// Terminal elem, must be last
    ArraySize,
    /// When inside path, can be [TemplateKind::Value] or [TemplateKind::Type] arg. When last element, must be [TemplateKind::Value] (of course)
    InNamed(TemplateID),
    // InTuple(usize)...
}

#[derive(Debug)]
pub struct InferenceTargetPath {
    pub port: PortID,
    pub path: Vec<SubtypeInferencePathElem>,
    pub span: Span,
}
#[derive(Debug)]
pub enum InferenceTarget {
    /// Drill down into the type of the given port
    /// in `Matrix #(T: int #(..., MAX))`: `MAX` would be vec![InNamed(0), InNamed(1)]
    Subtype(InferenceTargetPath),
    /// |to| - |from|
    /// Always corresponds to [SubtypeRelation::Min]
    PortLatency { from: PortID, to: PortID },
}

fn walk_type<'t>(path: &[SubtypeInferencePathElem], mut typ: &'t ConcreteType) -> &'t ConcreteType {
    for p in path {
        match p {
            SubtypeInferencePathElem::DownArray => typ = &typ.unwrap_array().0,
            SubtypeInferencePathElem::ArraySize => unreachable!("Only end of value path!"),
            SubtypeInferencePathElem::InNamed(arg_id) => {
                typ = typ.unwrap_named().template_args[*arg_id].unwrap_type()
            }
        }
    }
    typ
}
impl InferenceTargetPath {
    pub fn follow_value_path<'t>(&self, typ: &'t ConcreteType) -> &'t UnifyableValue {
        let (last, pre_path) = self.path.split_last().unwrap();
        let typ = walk_type(pre_path, typ);
        match last {
            SubtypeInferencePathElem::DownArray => unreachable!("Can't point to value!"),
            SubtypeInferencePathElem::ArraySize => &typ.unwrap_array().1,
            SubtypeInferencePathElem::InNamed(arg_id) => {
                typ.unwrap_named().template_args[*arg_id].unwrap_value()
            }
        }
    }
    pub fn follow_type_path<'t>(&self, typ: &'t ConcreteType) -> &'t ConcreteType {
        walk_type(&self.path, typ)
    }
}

#[derive(Debug)]
/// Represents the formula `Self * mul_by + offset {relation} target`
///
/// Examples:
/// `input port_a'5, output port_b'3*V` in a context of max distance `|port_b| - |port_a| == 7`, becomes: `V * 3 - 5 <= (Min) 7`
///
/// `int#(FROM: FROM_VAR + 3, TO: TO_VAR)[ARR_SZ + 9] p` unified against `int#(FROM: 1, TO: 255)[20]`, becomes:
///     - `FROM_VAR * 1 + 3 <= (Min) 1`
///     - `TO_VAR * 1 + 3 >= (Max) 255`
///     - `ARR_SZ * 1 + 9 == (Exact) 20`
pub struct InferenceCandidate {
    pub mul_by: IBig,
    pub offset: IBig,
    pub relation: SubtypeRelation,
    pub target: InferenceTarget,
}

struct MakeAllEdgesResult {
    port_groups: Vec<Vec<(PortID, i64)>>,
    extra_poison: Vec<(PortID, PortID)>,
}

fn make_latency_inference_info(
    port_latency_linearities: &FlatAlloc<FullPortLatencyLinearity, PortIDMarker>,
    parameter_inference_candidates: &mut TVec<TemplateKind<TypeInferInfo, ValueInferInfo>>,
) -> MakeAllEdgesResult {
    let mut port_groups: Vec<Vec<(PortID, i64)>> = port_latency_linearities
        .iter()
        .filter_map(|(p_id, lin_info)| {
            if let Some(info) = &lin_info.latency_linearity
                && let Ok(lat_info) = i64::try_from(&info.const_factor)
            {
                Some(vec![(p_id, lat_info)])
            } else {
                None
            }
        })
        .collect();

    let mut extra_poison: Vec<(PortID, PortID)> = Vec::new();

    for (from_id, from) in port_latency_linearities {
        for (to_id, to) in port_latency_linearities {
            if from.domain != to.domain {
                continue;
            }

            if let (Some(from_linearity), Some(to_linearity)) =
                (&from.latency_linearity, &to.latency_linearity)
            {
                match ParamLinearity::is_pair_latency_candidate(from_linearity, to_linearity) {
                    EdgeInfo::Inferrable {
                        target_to_infer,
                        multiply_var_by,
                        offset,
                    } => {
                        if !(from.direction == Direction::Input
                            && to.direction == Direction::Output)
                        {
                            continue; // No inference is possible between Input/Input or Output/Output
                        }

                        assert!(multiply_var_by != IBig::from(0));

                        parameter_inference_candidates[target_to_infer]
                            .unwrap_value_mut()
                            .candidates
                            .push(InferenceCandidate {
                                mul_by: multiply_var_by,
                                offset,
                                relation: SubtypeRelation::Min,
                                target: InferenceTarget::PortLatency {
                                    from: from_id,
                                    to: to_id,
                                },
                            });
                    }
                    EdgeInfo::Poison => {
                        if from.direction == Direction::Output || to.direction == Direction::Input {
                            continue; // No inference is possible between Input/Input or Output/Output
                        }

                        extra_poison.push((from_id, to_id));
                    }
                    EdgeInfo::ConstantOffset(_) => {
                        let from_group_idx = port_groups
                            .iter()
                            .position(|g| g.iter().any(|elem| elem.0 == from_id))
                            .unwrap();

                        let to_group_idx = port_groups
                            .iter()
                            .position(|g| g.iter().any(|elem| elem.0 == to_id))
                            .unwrap();

                        if let Ok([from_group, to_group]) =
                            port_groups.get_disjoint_mut([from_group_idx, to_group_idx])
                        {
                            from_group.append(to_group);
                            port_groups.swap_remove(to_group_idx);
                        } else {
                            // The ports are already in the same group
                        }
                    }
                }
            } else {
                if from.direction == Direction::Output || to.direction == Direction::Input {
                    continue; // No inference is possible between Input/Input or Output/Output
                }

                extra_poison.push((from_id, to_id));
            }
        }
    }

    MakeAllEdgesResult {
        port_groups,
        extra_poison,
    }
}

struct InferenceTypeWalker<'cand, 'instr> {
    parameter_inference_candidates: &'cand mut TVec<TemplateKind<TypeInferInfo, ValueInferInfo>>,
    port: PortID,
    cur_path: Vec<SubtypeInferencePathElem>,
    instructions: &'instr FlatAlloc<Instruction, FlatIDMarker>,
}

impl InferenceTypeWalker<'_, '_> {
    fn walk(&mut self, typ_expr: &WrittenType) {
        match typ_expr {
            WrittenType::TemplateVariable(_, type_param) => {
                self.parameter_inference_candidates[*type_param]
                    .unwrap_type_mut()
                    .candidates
                    .push(InferenceTargetPath {
                        port: self.port,
                        path: self.cur_path.clone(),
                        span: typ_expr.get_span(),
                    });
            }
            WrittenType::Named(named_global) => {
                for arg in &named_global.template_args {
                    if let Some(refers_to) = arg.refers_to.get() {
                        self.cur_path
                            .push(SubtypeInferencePathElem::InNamed(*refers_to));
                        match &arg.kind {
                            Some(TemplateKind::Type(typ_arg)) => self.walk(typ_arg),
                            Some(TemplateKind::Value(v_expr)) => {
                                let relation = match (named_global.id, *refers_to) {
                                    (get_builtin_type!("int"), UUID(0, PhantomData)) => {
                                        SubtypeRelation::Min
                                    }
                                    (get_builtin_type!("int"), UUID(1, PhantomData)) => {
                                        SubtypeRelation::Max
                                    }
                                    _ => SubtypeRelation::Exact,
                                };
                                self.try_add_infer_info(*v_expr, relation);
                            }
                            None => {}
                        }

                        self.cur_path.pop().unwrap();
                    }
                }
            }
            WrittenType::Array(_, arr_box) => {
                let (content, sz, _) = arr_box.deref();

                self.cur_path.push(SubtypeInferencePathElem::DownArray);
                self.walk(content);
                self.cur_path.pop().unwrap();

                self.cur_path.push(SubtypeInferencePathElem::ArraySize);
                self.try_add_infer_info(*sz, SubtypeRelation::Exact);
                self.cur_path.pop().unwrap();
            }
            WrittenType::Error(_) => {}
        }
    }

    fn try_add_infer_info(&mut self, expr_id: FlatID, relation: SubtypeRelation) {
        let Some(param_linearity) = expression_to_param_linearity(
            self.instructions,
            expr_id,
            self.parameter_inference_candidates.id_range(),
        ) else {
            return;
        };
        let Some((var, mul_by, offset)) = param_linearity.try_into_one_variable() else {
            return;
        };
        let span = self.instructions[expr_id].unwrap_subexpression().span;
        self.parameter_inference_candidates[var]
            .unwrap_value_mut()
            .candidates
            .push(InferenceCandidate {
                mul_by,
                offset,
                relation,
                target: InferenceTarget::Subtype(InferenceTargetPath {
                    port: self.port,
                    path: self.cur_path.clone(),
                    span,
                }),
            });
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ValueInferStrategy {
    /// Only [ValueInferStrategy::Unify] may refer to non-int parameters. If used, then associated [InferenceCandidate::mul_by] == 1 and [InferenceCandidate::offset] == 0
    Unify,
    Exact,
    Min,
    Max,
}

#[derive(Debug)]
pub struct ValueInferInfo {
    pub candidates: Vec<InferenceCandidate>,
    /// Iterate `candidates[0..total_inference_upto]` for all valid candidate inference points
    pub total_inference_upto: usize,
    pub total_inference_strategy: ValueInferStrategy,
}

#[derive(Debug)]
pub struct TypeInferInfo {
    /// Partitioned into input ports (`candidates[0..num_subtype_candidates]`) and output ports (`candidates[num_subtype_candidates..]`)
    ///
    /// Input ports must be handled with subtyping relations. Outputs should be handled with simple unification.
    pub candidates: Vec<InferenceTargetPath>,
    pub num_inputs: usize,
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
#[derive(Default, Debug)]
pub struct PortLatencyInferenceInfo {
    /// These are only relevant for Value args. Type args partake in regular subtyping
    pub parameter_inference_candidates: TVec<TemplateKind<TypeInferInfo, ValueInferInfo>>,

    pub port_groups: Vec<Vec<(PortID, i64)>>,
    pub extra_poison: Vec<(PortID, PortID)>,
}

impl PortLatencyInferenceInfo {
    pub fn make(
        ports: &FlatAlloc<Port, PortIDMarker>,
        instructions: &FlatAlloc<Instruction, FlatIDMarker>,
        template_args: &TVec<Parameter>,
    ) -> PortLatencyInferenceInfo {
        let mut parameter_inference_candidates = template_args.map(|(_, arg)| match &arg.kind {
            TemplateKind::Type(_) => TemplateKind::Type(TypeInferInfo {
                candidates: Vec::new(),
                num_inputs: 0, // PLACEHOLDER
            }),
            TemplateKind::Value(_) => TemplateKind::Value(ValueInferInfo {
                candidates: Vec::new(),
                total_inference_upto: usize::MAX, // PLACEHOLDER
                total_inference_strategy: ValueInferStrategy::Unify, // PLACEHOLDER
            }),
        });

        // Port to port latencies. Handle constant-offset port groups, inference possibilities and poison edges
        let port_latency_linearities = ports.map(|(_port_id, port)| {
            let latency_linearity = port.latency_specifier.and_then(|latency_spec| {
                expression_to_param_linearity(instructions, latency_spec, template_args.id_range())
            });

            FullPortLatencyLinearity {
                domain: port.domain,
                direction: port.direction,
                latency_linearity,
            }
        });
        let MakeAllEdgesResult {
            port_groups,
            extra_poison,
        } = make_latency_inference_info(
            &port_latency_linearities,
            &mut parameter_inference_candidates,
        );

        // Ways template args can be inferred from port types
        for (port_id, p) in ports {
            match &instructions[p.declaration_instruction] {
                Instruction::Declaration(decl) => {
                    let mut inference_walker = InferenceTypeWalker {
                        parameter_inference_candidates: &mut parameter_inference_candidates,
                        port: port_id,
                        cur_path: Vec::new(),
                        instructions,
                    };
                    inference_walker.walk(&decl.typ_expr);
                }
                Instruction::Interface(_) => {} // Ignore, no type
                _ => unreachable!("Can't be a port"),
            }
        }

        for (_, param) in &mut parameter_inference_candidates {
            match param {
                TemplateKind::Type(t_param) => {
                    t_param.num_inputs = partition_in_place(&mut t_param.candidates, |c| {
                        ports[c.port].direction == Direction::Input
                    });
                }
                TemplateKind::Value(v_param) => {
                    let num_exacts = partition_in_place(&mut v_param.candidates, |c| {
                        c.relation == SubtypeRelation::Exact
                    });
                    (
                        v_param.total_inference_strategy,
                        v_param.total_inference_upto,
                    ) = if v_param.candidates.is_empty() {
                        // Does nothing since there's no targets
                        (ValueInferStrategy::Unify, 0)
                    } else if num_exacts == 0 {
                        let num_with_max_constraint = v_param
                            .candidates
                            .iter()
                            .filter(|c| {
                                (c.mul_by >= IBig::from(0)) == (c.relation == SubtypeRelation::Max)
                            })
                            .count();

                        if num_with_max_constraint == 0 {
                            (ValueInferStrategy::Min, v_param.candidates.len())
                        } else if num_with_max_constraint == v_param.candidates.len() {
                            (ValueInferStrategy::Max, v_param.candidates.len())
                        } else {
                            // Does nothing since there's no targets
                            (ValueInferStrategy::Unify, 0)
                        }
                    } else {
                        // At least one exact constraint overrides all Min/Max constraints.
                        if v_param.candidates.iter().all(|c| {
                            c.mul_by == IBig::from(1)
                                && c.offset == IBig::from(0)
                                && matches!(c.target, InferenceTarget::Subtype(_))
                        }) {
                            (ValueInferStrategy::Unify, num_exacts)
                        } else {
                            (ValueInferStrategy::Exact, num_exacts)
                        }
                    };
                }
            }
        }

        Self {
            parameter_inference_candidates,
            port_groups,
            extra_poison,
        }
    }
}

#[cfg(false)] // TODO fix these tests to reuse them
#[cfg(test)]
mod tests {
    use crate::{
        alloc::{FlatAlloc, UUIDRange},
        flattening::Direction,
        instantiation::SubModulePort,
        latency::{
            latency_algorithm::{
                FanInOut, LatencyCountingPorts, infer_unknown_latency_edges, mk_fan,
            },
            list_of_lists::ListOfLists,
        },
        prelude::{DomainID, PortIDMarker, WireID},
    };

    use super::{FullPortLatencyLinearity, ParamLinearity, PortLatencyInferenceInfo};

    fn mk_input_linearity(
        domain: DomainID,
        const_factor: i64,
        arg_factors: Vec<i64>,
    ) -> FullPortLatencyLinearity {
        FullPortLatencyLinearity {
            domain,
            direction: Direction::Input,
            latency_linearity: Some(ParamLinearity {
                const_factor,
                arg_linear_factor: FlatAlloc::from_vec(arg_factors),
            }),
        }
    }
    fn mk_output_linearity(
        domain: DomainID,
        const_factor: i64,
        arg_factors: Vec<i64>,
    ) -> FullPortLatencyLinearity {
        FullPortLatencyLinearity {
            domain,
            direction: Direction::Output,
            latency_linearity: Some(ParamLinearity {
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
        port_latency_linearities.alloc(mk_input_linearity(first_domain, 0, vec![0, 0]));
        port_latency_linearities.alloc(mk_input_linearity(first_domain, 3, vec![1, 0]));
        port_latency_linearities.alloc(mk_output_linearity(first_domain, 5, vec![3, 0]));
        port_latency_linearities.alloc(mk_output_linearity(first_domain, 0, vec![0, 1]));

        let latency_info = PortLatencyInferenceInfo {
            port_latency_linearities,
        };

        let known_template_args = FlatAlloc::from_vec(vec![None, None]);

        let mut variables = FlatAlloc::new();

        let result = latency_info.get_inference_edges(&known_template_args, (), &mut variables);

        dbg!(&result);
    }

    /// ```sus
    /// module infer_mex #(int A, int B, int C) {
    ///     domain x
    ///     interface x : bool v2'-A, bool v3'-B -> bool v6'0
    ///     domain y
    ///     interface y : bool v4'-C+3, bool v5'-B -> bool v7'0
    /// }
    /// module use_infer_mex {
    ///     interface use_infer_mex : bool v0'0 -> bool v8'10
    ///     bool v1 = v0
    ///     
    ///     reg bool v2 = v1
    ///     reg reg reg reg reg reg bool v3 = v1
    ///     reg reg bool v4 = v0
    ///     reg reg reg reg reg bool v5 = v0
    ///     
    ///     infer_mex inf
    ///     bool v6 = inf.x(v2, v3)
    ///     bool v7 = inf.y(v4, v5)
    ///     
    ///     reg reg reg v8 = v6
    ///     reg reg v8 = v7
    /// }
    /// ```
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
        port_latency_linearities.alloc(mk_input_linearity(domain_x, 0, vec![-1, 0, 0])); // 2
        port_latency_linearities.alloc(mk_input_linearity(domain_x, 0, vec![0, -1, 0])); // 3
        port_latency_linearities.alloc(mk_input_linearity(domain_y, 3, vec![0, 0, -1])); // 4'C+3, which makes C 3 smaller than in [latency_algorithm::tests::test_inference_no_poison]
        port_latency_linearities.alloc(mk_input_linearity(domain_y, 0, vec![0, -1, 0])); // 5
        port_latency_linearities.alloc(mk_output_linearity(domain_x, 0, vec![0, 0, 0])); // 6
        port_latency_linearities.alloc(mk_output_linearity(domain_y, 0, vec![0, 0, 0])); // 7

        let latency_info = PortLatencyInferenceInfo {
            port_latency_linearities,
        };

        let known_template_args = FlatAlloc::from_vec(vec![None, None, None]);

        let mut values_to_infer = FlatAlloc::new();

        let local_inference_edges =
            latency_info.get_inference_edges(&known_template_args, (), &mut values_to_infer);

        // Because both domains of the submodule are part of the outer domain, we just merge both

        let port_to_wire_map = FlatAlloc::from_vec(vec![2, 3, 4, 5, 6, 7]).map(|(_, v)| {
            Some(SubModulePort {
                maps_to_wire: WireID::from_hidden_value(*v),
                name_refs: Vec::new(),
            })
        });
        let wire_to_node_map = FlatAlloc::from_vec(vec![0, 1, 2, 3, 4, 5, 6, 7, 8]);

        let mut extra_fanin = Vec::new();
        let mut inference_edges = Vec::new();
        local_inference_edges.apply_to_global_domain(
            &port_to_wire_map,
            &wire_to_node_map,
            &mut extra_fanin,
            &mut inference_edges,
        );

        let specified_latencies = [];

        let fanins =
            fanins.add_extra_fanin_and_specified_latencies(Vec::new(), &specified_latencies);

        infer_unknown_latency_edges(
            fanins,
            &ports,
            &specified_latencies,
            &inference_edges,
            &mut values_to_infer,
        );

        assert_eq!(
            values_to_infer.map(|v| v.1.get()).into_vec(),
            [Some(6), Some(1), Some(9)] // C 3 smaller due to offset on port 4
        );
    }
}
