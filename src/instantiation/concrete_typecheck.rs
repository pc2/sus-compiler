use ibig::IBig;
use ibig::ops::DivRem;
use sus_proc_macro::get_builtin_type;

use crate::alloc::{zip_eq, zip_eq3};
use crate::dev_aid::dot_graphs::display_latency_count_graph;
use crate::instantiation::instantiator::InstantiateError;
use crate::latency::LatencyInferenceProblem;
use crate::latency::port_latency_inference::{
    InferenceCandidate, InferenceTarget, InferenceTargetPath, ValueInferStrategy,
};
use crate::to_string::display_all_infer_params;
use crate::typing::concrete_type::SubtypeRelation;
use crate::typing::template::TemplateKind;
use crate::typing::type_inference::UnifyResult;
use crate::typing::{concrete_type::ConcreteType, value_unifier::ValueUnifier};
use crate::util::{all_equal, ceil_div, floor_div};
use crate::value::MAX_SHIFT;

use crate::typing::unifyable_cell::{Unifier, UnifierTop};

use super::*;

macro_rules! assert_due_to_variable_clones {
    ($cond:expr) => {
        assert!($cond, "This assertion cannot fail, because the variables that caused the unification failure should have been cloned in execute")
    };
}

fn unify_rank<'inst>(
    rank: &'inst [UnifyableValue],
    mut typ: &'inst ConcreteType,
    unifier: &ValueUnifier<'inst>,
) -> bool {
    rank.iter().all(|r| {
        let_unwrap!(ConcreteType::Array(arr), typ);
        typ = &arr.0;
        unifier.unify(r, &arr.1) == UnifyResult::Success
    })
}

/// Walks a path `b == a[x][y][z:w]`, and returns `a[x][y][z:w]` and `b[]` (take one array from b). Also unifies this one array with w - z
pub fn co_walk_path<'inst>(
    mut a: &'inst ConcreteType,
    mut b: &'inst ConcreteType,
    path: &'inst [RealWirePathElem],
    unifier: &ValueUnifier<'inst>,
) -> (&'inst ConcreteType, &'inst ConcreteType) {
    for p in path {
        match p {
            RealWirePathElem::Index { .. } | RealWirePathElem::ConstIndex { .. } => {
                a = &a.unwrap_array().0;
            }
            RealWirePathElem::PartSelect { width, .. } => {
                a = &a.unwrap_array().0;
                let (new_b, b_sz) = b.unwrap_array();
                b = new_b;

                // Is checked in final_checks
                let _ = unifier.set(b_sz, width.clone());
            }
            RealWirePathElem::Slice { bounds, .. } => {
                let (new_a, a_sz) = a.unwrap_array();
                a = new_a;
                let (new_b, b_sz) = b.unwrap_array();
                b = new_b;

                match bounds {
                    PartialBound::Known(from, to) => {
                        // Is checked in final_checks
                        let _ = unifier.set(b_sz, to - from);
                    }
                    PartialBound::From(from) => {
                        unifier.delayed_constraint(move |unifier| {
                            let a_sz = unifier.resolve(a_sz)?.unwrap_integer();
                            // TODO #88, Slices of variable base offset
                            // Is checked in final_checks
                            let _ = unifier.set(b_sz, a_sz - from);
                            Ok(())
                        });
                    }
                    PartialBound::To(to) => {
                        // TODO #88, Slices of variable base offset
                        // Is checked in final_checks
                        let _ = unifier.set(b_sz, to.clone());
                    }
                    PartialBound::WholeSlice => {
                        // Is checked in final_checks
                        let _ = unifier.unify(b_sz, a_sz);
                    }
                }
            }
        }
    }

    (a, b)
}

/// Panics if `potentials.len() < 2`
fn set_min_max_with_min_max<'inst>(
    unifier: &ValueUnifier<'inst>,
    out_bounds: IntBounds<&'inst UnifyableValue>,
    potentials: impl IntoIterator<Item = IBig>,
) {
    let mut potentials = potentials.into_iter();
    let mut min = potentials.next().unwrap();
    let mut max = potentials.next().unwrap();
    if max < min {
        std::mem::swap(&mut min, &mut max);
    }
    for p in potentials {
        if p < min {
            min = p;
        } else if p > max {
            max = p;
        }
    }
    unifier.set(out_bounds.from, min).unwrap();
    unifier.set(out_bounds.to, max + 1).unwrap();
}

/// Combine both mutable context elements into a single struct, to avoid <https://github.com/someguynamedjosh/ouroboros/issues/138>
pub struct MutableContext<'inst> {
    unifier: ValueUnifier<'inst>,
    all_submod_ids: Vec<SubModuleID>,
}

#[ouroboros::self_referencing]
pub struct ModuleTypingSuperContext<'l> {
    ctx: ModuleTypingContext<'l>,
    #[borrows(ctx)]
    #[not_covariant]
    mutable_state: MutableContext<'this>,
}

impl<'l> ModuleTypingSuperContext<'l> {
    pub fn start_typechecking(ctx: ModuleTypingContext<'l>) -> Self {
        let mut result: ModuleTypingSuperContext<'l> = ModuleTypingSuperContextBuilder {
            ctx,
            mutable_state_builder: move |ctx| MutableContext {
                unifier: ValueUnifier::new(),
                all_submod_ids: ctx.submodules.id_range().iter().collect(),
            },
        }
        .build();

        result.with_mut(move |result| {
            // We do submodules first, such that we don't need to worry handle unification failure
            for (_, sm) in &result.ctx.submodules {
                result
                    .ctx
                    .add_submodule_subtype_constraints(sm, &result.mutable_state.unifier);
            }
            for (_, wire) in &result.ctx.wires {
                result
                    .ctx
                    .add_wire_subtype_constraints(wire, &result.mutable_state.unifier);
            }
        });

        result
    }

    /// Returns a list of submodules that should be instantiated before proceeding with typechecking
    /// This does not need to be all the submodules in the module, as it could be that it doesn't know some parameters yet.
    /// If empty, no more progress can be made and the caller may stop calling [Self::typecheck_step].
    /// For every processed module this method returns the caller should call [Self::apply_instantiated_submodule]
    pub fn typecheck_step(&mut self) -> Vec<(SubModuleID, ConcreteGlobalReference<ModuleUUID>)> {
        self.with_mut(|self_mut| {
            self_mut.mutable_state.unifier.execute_ready_constraints();
            self_mut.ctx.try_infer_submodule_params(
                &self_mut.mutable_state.unifier,
                &mut self_mut.mutable_state.all_submod_ids,
            )
        })
    }

    pub fn apply_instantiated_submodule(
        &mut self,
        sm_id: SubModuleID,
        instance: Result<Rc<InstantiatedModule>, InstantiateError>,
    ) {
        self.with(|slf| {
            slf.ctx
                .apply_instantiated_submodule(sm_id, instance, &slf.mutable_state.unifier);
        })
    }

    pub fn finish(self) -> ModuleTypingContext<'l> {
        self.with(|fields| {
            fields
                .ctx
                .fully_substitute_everything(&fields.mutable_state.unifier);
            fields.mutable_state.unifier.decomission();
        });

        let mut ctx = self.into_heads().ctx;

        ctx.compute_latencies();
        ctx.finalize();
        ctx
    }
}

impl<'inst, 'l: 'inst> ModuleTypingContext<'l> {
    fn add_wire_subtype_constraints(
        &'inst self,
        out: &'inst RealWire,
        unifier: &ValueUnifier<'inst>,
    ) {
        let original_instr = &self.link_info.instructions[out.original_instruction];
        let span = original_instr.get_span();
        span.debug();

        match &out.source {
            RealWireDataSource::ReadOnly => {}
            RealWireDataSource::Multiplexer {
                is_state: _,
                sources,
            } => {
                // Temporary vector because borrow checker doesn't like constructing the iter with unifier
                let pairs: Vec<_> = sources
                    .iter()
                    .map(|s| {
                        let source_wire = &self.wires[s.from];
                        co_walk_path(&out.typ, &source_wire.typ, &s.to_path, unifier)
                    })
                    .collect();

                unifier.create_subtype_constraint(pairs.iter().copied());
            }
            RealWireDataSource::UnaryOp { op, rank, right } => {
                let right = &self.wires[*right];
                assert_due_to_variable_clones!(unify_rank(rank, &out.typ, unifier));
                if !unify_rank(rank, &right.typ, unifier) {
                    unifier.delayed_error(|unifier| {
                        self.errors
                            .error(right.get_span(self.link_info), format!("Incompatible multi-rank for higher-rank operator: Found {} but output is {}",
                            right.typ.display_substitute(self.globals, unifier),
                            out.typ.display_substitute(self.globals, unifier))
                        );
                    });
                }
                let out_root = out.typ.walk_rank(rank.len());
                let right_root = right.typ.walk_rank(rank.len());
                self.typecheck_unary(unifier, *op, out_root, right_root);
            }
            RealWireDataSource::BinaryOp {
                op,
                rank,
                left,
                right,
            } => {
                let left = &self.wires[*left];
                let right = &self.wires[*right];
                let out_root = out.typ.walk_rank(rank.len());
                let left_root = left.typ.walk_rank(rank.len());
                let right_root = right.typ.walk_rank(rank.len());
                assert_due_to_variable_clones!(unify_rank(rank, &out.typ, unifier));
                if !unify_rank(rank, &left.typ, unifier) {
                    unifier.delayed_error(|unifier| {
                        self.errors
                            .error(left.get_span(self.link_info), format!("Incompatible multi-rank for higher-rank operator: Found {} but output is {}",
                            left.typ.display_substitute(self.globals, unifier),
                            out.typ.display_substitute(self.globals, unifier))
                        ).info(right.get_span(self.link_info), format!("Right argument has type {}", right.typ.display_substitute(self.globals, unifier)));
                    });
                }
                if !unify_rank(rank, &right.typ, unifier) {
                    unifier.delayed_error(|unifier| {
                        self.errors
                            .error(right.get_span(self.link_info), format!("Incompatible multi-rank for higher-rank operator: Found {} but output is {}",
                            right.typ.display_substitute(self.globals, unifier),
                            out.typ.display_substitute(self.globals, unifier))
                        ).info(left.get_span(self.link_info), format!("Left argument has type {}", left.typ.display_substitute(self.globals, unifier)));
                    });
                }
                self.typecheck_binop(unifier, span, *op, out_root, left_root, right_root);
            }
            RealWireDataSource::Select { root, path } => {
                let root_wire = &self.wires[*root];

                let (found, expected) = co_walk_path(&root_wire.typ, &out.typ, path, unifier);

                // Checked in final_check
                let _ = unifier.unify_concrete_all(found, expected);
            }
            RealWireDataSource::ConstructArray { array_wires } => {
                let (array_content_supertyp, arr_sz) = out.typ.unwrap_array();

                unifier.create_subtype_constraint(array_wires.iter().map(|w_id| {
                    let w = &self.wires[*w_id];
                    (array_content_supertyp, &w.typ)
                }));

                // The output's size cannot have already been unified, this is the first time we see it
                unifier.set(arr_sz, IBig::from(array_wires.len())).unwrap();
            }
            // type is already set when the wire was created
            RealWireDataSource::Constant { value: _ } => {}
        };
    }

    /// TODO overloading
    fn typecheck_unary(
        &'inst self,
        unifier: &ValueUnifier<'inst>,
        op: UnaryOperator,
        out_root: &'inst ConcreteType,
        right_root: &'inst ConcreteType,
    ) {
        match op {
            UnaryOperator::Not => {
                assert_eq!(right_root.unwrap_named().id, get_builtin_type!("bool"));
                assert_eq!(out_root.unwrap_named().id, get_builtin_type!("bool"));
            }
            UnaryOperator::And | UnaryOperator::Or | UnaryOperator::Xor => {
                assert_eq!(
                    right_root.unwrap_array().0.unwrap_named().id,
                    get_builtin_type!("bool")
                );
                assert_eq!(out_root.unwrap_named().id, get_builtin_type!("bool"));
            }
            UnaryOperator::Negate => {
                let out = out_root.unwrap_int_bounds_unknown();
                let IntBounds { from, to } = right_root.unwrap_int_bounds_unknown();

                // 4:7 -> -6:-3
                unifier.delayed_constraint(|unifier| {
                    let to = unifier.resolve(to)?.unwrap_integer();
                    unifier.set(out.from, 1 - to).unwrap();
                    Ok(())
                });
                unifier.delayed_constraint(|unifier| {
                    let from = unifier.resolve(from)?.unwrap_integer();
                    unifier.set(out.to, 1 - from).unwrap();
                    Ok(())
                });
            }
            UnaryOperator::Sum => {
                let out = out_root.unwrap_int_bounds_unknown();
                let (content, sz) = right_root.unwrap_array();
                let IntBounds { from, to } = content.unwrap_int_bounds_unknown();

                unifier.delayed_constraint(|unifier| {
                    let from = unifier.resolve(from)?.unwrap_integer();
                    let sz = unifier.resolve(sz)?.unwrap_integer();
                    unifier.set(out.from, from * sz).unwrap();
                    Ok(())
                });
                unifier.delayed_constraint(|unifier| {
                    let to = unifier.resolve(to)?.unwrap_integer();
                    let sz = unifier.resolve(sz)?.unwrap_integer();
                    unifier.set(out.to, (to - 1) * sz + 1).unwrap();
                    Ok(())
                });
            }
            UnaryOperator::Product => {
                let out = out_root.unwrap_int_bounds_unknown();
                let (content, sz) = right_root.unwrap_array();
                let IntBounds { from, to } = content.unwrap_int_bounds_unknown();

                unifier.delayed_constraint(|unifier| {
                    let from = unifier.resolve(from)?.unwrap_integer();
                    let sz = unifier.resolve(sz)?.unwrap_integer();
                    let sz = usize::try_from(sz).unwrap();
                    unifier.set(out.from, from.pow(sz)).unwrap();
                    Ok(())
                });
                unifier.delayed_constraint(|unifier| {
                    let to = unifier.resolve(to)?.unwrap_integer();
                    let sz = unifier.resolve(sz)?.unwrap_integer();
                    let sz = usize::try_from(sz).unwrap();
                    let max: IBig = to - 1;
                    unifier.set(out.to, max.pow(sz) + 1).unwrap();
                    Ok(())
                });
            }
        }
    }

    /// TODO overloading
    fn typecheck_binop(
        &'inst self,
        unifier: &ValueUnifier<'inst>,
        span: Span,
        op: BinaryOperator,
        out_root: &'inst ConcreteType,
        left_root: &'inst ConcreteType,
        right_root: &'inst ConcreteType,
    ) {
        match op {
            BinaryOperator::And | BinaryOperator::Or | BinaryOperator::Xor => {
                assert_eq!(left_root.unwrap_named().id, get_builtin_type!("bool"));
                assert_eq!(right_root.unwrap_named().id, get_builtin_type!("bool"));
                assert_eq!(out_root.unwrap_named().id, get_builtin_type!("bool"));
            }
            BinaryOperator::Add => {
                let IntBounds { from: lf, to: lt } = left_root.unwrap_int_bounds_unknown();
                let IntBounds { from: rf, to: rt } = right_root.unwrap_int_bounds_unknown();
                let out = out_root.unwrap_int_bounds_unknown();
                unifier.delayed_constraint(move |unifier| {
                    let lf = unifier.resolve(lf)?.unwrap_integer();
                    let rf = unifier.resolve(rf)?.unwrap_integer();
                    unifier.set(out.from, lf + rf).unwrap();
                    Ok(())
                });
                unifier.delayed_constraint(move |unifier| {
                    let lt = unifier.resolve(lt)?.unwrap_integer();
                    let rt = unifier.resolve(rt)?.unwrap_integer();
                    unifier.set(out.to, lt + rt - 1).unwrap();
                    Ok(())
                });
            }
            BinaryOperator::Subtract => {
                let IntBounds { from: lf, to: lt } = left_root.unwrap_int_bounds_unknown();
                let IntBounds { from: rf, to: rt } = right_root.unwrap_int_bounds_unknown();
                let out = out_root.unwrap_int_bounds_unknown();
                unifier.delayed_constraint(move |unifier| {
                    let lf = unifier.resolve(lf)?.unwrap_integer();
                    let rt = unifier.resolve(rt)?.unwrap_integer();
                    unifier.set(out.from, lf - (rt - 1)).unwrap();
                    Ok(())
                });
                unifier.delayed_constraint(move |unifier| {
                    let lt = unifier.resolve(lt)?.unwrap_integer();
                    let rf = unifier.resolve(rf)?.unwrap_integer();
                    unifier.set(out.to, lt - rf).unwrap();
                    Ok(())
                });
            }
            BinaryOperator::Multiply => {
                let IntBounds { from: lf, to: lt } = left_root.unwrap_int_bounds_unknown();
                let IntBounds { from: rf, to: rt } = right_root.unwrap_int_bounds_unknown();
                let out = out_root.unwrap_int_bounds_unknown();
                unifier.delayed_constraint(move |unifier| {
                    let lf = unifier.resolve(lf)?.unwrap_integer();
                    let lt = unifier.resolve(lt)?.unwrap_integer();
                    let rf = unifier.resolve(rf)?.unwrap_integer();
                    let rt = unifier.resolve(rt)?.unwrap_integer();
                    let lmax = lt - 1;
                    let rmax = rt - 1;

                    let potentials = [lf * rf, &lmax * &rmax, lf * rmax, lmax * rf];
                    set_min_max_with_min_max(unifier, out, potentials);
                    Ok(())
                });
            }
            BinaryOperator::Divide => {
                let IntBounds { from: lf, to: lt } = left_root.unwrap_int_bounds_unknown();
                let IntBounds { from: rf, to: rt } = right_root.unwrap_int_bounds_unknown();
                let out = out_root.unwrap_int_bounds_unknown();
                unifier.delayed_constraint(move |unifier| {
                    let lf = unifier.resolve(lf)?.unwrap_integer();
                    let lt = unifier.resolve(lt)?.unwrap_integer();
                    let rf = unifier.resolve(rf)?.unwrap_integer();
                    let rt = unifier.resolve(rt)?.unwrap_integer();
                    let right_bounds = IntBounds { from: rf, to: rt };

                    if !right_bounds.is_valid_non_empty() {
                        return Ok(()); // Invalid bounds errors are reported by final_checks.rs
                    }
                    if right_bounds.contains(&IBig::from(0)) {
                        self.errors.error(
                            span,
                            format!(
                                "Possible divide by 0, right argument bounds are {right_bounds}"
                            ),
                        );
                        return Ok(());
                    }
                    let lmax = lt - 1;
                    let rmax = rt - 1;

                    let potentials = [lf / rf, &lmax / &rmax, lf / rmax, lmax / rf];
                    set_min_max_with_min_max(unifier, out, potentials);
                    Ok(())
                });
            }
            BinaryOperator::Remainder => {
                let IntBounds { from: lf, to: lt } = left_root.unwrap_int_bounds_unknown();
                let IntBounds { from: rf, to: rt } = right_root.unwrap_int_bounds_unknown();
                let out = out_root.unwrap_int_bounds_unknown();
                unifier.delayed_constraint(move |unifier| {
                    let lf = unifier.resolve(lf)?.unwrap_integer();
                    let lt = unifier.resolve(lt)?.unwrap_integer();
                    let rf = unifier.resolve(rf)?.unwrap_integer();
                    let rt = unifier.resolve(rt)?.unwrap_integer();
                    let right_bounds = IntBounds { from: rf, to: rt };

                    if !right_bounds.is_valid_non_empty() {
                        return Ok(()); // Invalid bounds errors are reported by final_checks.rs
                    }
                    if right_bounds.contains(&IBig::from(0)) {
                        self.errors.error(
                            span,
                            format!(
                                "Possible divide by 0, right argument bounds are {right_bounds}"
                            ),
                        );
                        return Ok(());
                    }
                    let remainder_max = if rf >= &IBig::from(0) { rt - 1 } else { -rf };

                    let ot = lt.clone().clamp(IBig::from(1), remainder_max.clone());
                    let of = lf.clone().clamp(-remainder_max + 1, IBig::from(0));
                    unifier.set(out.from, of).unwrap();
                    unifier.set(out.to, ot).unwrap();
                    Ok(())
                });
            }
            BinaryOperator::Modulo => {
                let _ = left_root.unwrap_int_bounds_unknown();
                let IntBounds { from: rf, to: rt } = right_root.unwrap_int_bounds_unknown();
                let out = out_root.unwrap_int_bounds_unknown();
                unifier.set(out.from, IBig::from(0)).unwrap();
                unifier.delayed_constraint(move |unifier| {
                    let rf = unifier.resolve(rf)?.unwrap_integer();
                    let rt = unifier.resolve(rt)?.unwrap_integer();
                    let right_bounds = IntBounds { from: rf, to: rt };
                    if !right_bounds.is_valid_non_empty() {
                        return Ok(()); // Invalid bounds errors are reported by final_checks.rs
                    }
                    if rf <= &IBig::from(0) {
                        self.errors.error(span, format!("Modulus must be strictly positive, right argument bounds are {right_bounds}"));
                        return Ok(());
                    }
                    unifier.set(out.to, rt - 1).unwrap();
                    Ok(())
                });
            }
            BinaryOperator::ShiftLeft => {
                let IntBounds { from: lf, to: lt } = left_root.unwrap_int_bounds_unknown();
                let IntBounds { from: rf, to: rt } = right_root.unwrap_int_bounds_unknown();
                let out = out_root.unwrap_int_bounds_unknown();
                unifier.delayed_constraint(move |unifier| {
                    let lf = unifier.resolve(lf)?.unwrap_integer();
                    let lt = unifier.resolve(lt)?.unwrap_integer();
                    let rf = unifier.resolve(rf)?.unwrap_integer();
                    let rt = unifier.resolve(rt)?.unwrap_integer();
                    let right_bounds = IntBounds { from: rf, to: rt };
                    if !right_bounds.is_valid_non_empty() {
                        return Ok(()); // Invalid bounds errors are reported by final_checks.rs
                    }
                    if rf < &IBig::from(0) {
                        self.errors.error(
                            span,
                            format!(
                                "Shifts must be positive, right argument bounds are {right_bounds}"
                            ),
                        );
                        return Ok(());
                    }
                    if rt > &IBig::from(MAX_SHIFT) {
                        self.errors.error(
                            span,
                            format!("The top bound of this shift is too large: {right_bounds}"),
                        );
                        return Ok(());
                    }
                    let min_shift = usize::try_from(rf).unwrap();
                    let max_shift = usize::try_from(rt - 1).unwrap();

                    let lmin = lf;
                    let lmax: IBig = lt - 1;

                    let potentials = [
                        lmin << min_shift,
                        lmin << max_shift,
                        &lmax << min_shift,
                        lmax << max_shift,
                    ];
                    set_min_max_with_min_max(unifier, out, potentials);
                    Ok(())
                });
            }
            BinaryOperator::ShiftRight => {
                let IntBounds { from: lf, to: lt } = left_root.unwrap_int_bounds_unknown();
                let IntBounds { from: rf, to: rt } = right_root.unwrap_int_bounds_unknown();
                let out = out_root.unwrap_int_bounds_unknown();
                unifier.delayed_constraint(move |unifier| {
                    let lf = unifier.resolve(lf)?.unwrap_integer();
                    let lt = unifier.resolve(lt)?.unwrap_integer();
                    let rf = unifier.resolve(rf)?.unwrap_integer();
                    let rt = unifier.resolve(rt)?.unwrap_integer();
                    let right_bounds = IntBounds { from: rf, to: rt };
                    if !right_bounds.is_valid_non_empty() {
                        return Ok(()); // Invalid bounds errors are reported by final_checks.rs
                    }
                    if rf < &IBig::from(0) {
                        self.errors.error(
                            span,
                            format!(
                                "Shifts must be positive, right argument bounds are {right_bounds}"
                            ),
                        );
                        return Ok(());
                    }
                    if rt > &IBig::from(MAX_SHIFT) {
                        self.errors.error(
                            span,
                            format!("The top bound of this shift is too large: {right_bounds}"),
                        );
                        return Ok(());
                    }
                    let min_shift = usize::try_from(rf).unwrap();
                    let max_shift = usize::try_from(rt - 1).unwrap();

                    let lmin = lf;
                    let lmax: IBig = lt - 1;

                    let potentials = [
                        lmin >> min_shift,
                        lmin >> max_shift,
                        &lmax >> min_shift,
                        lmax >> max_shift,
                    ];
                    set_min_max_with_min_max(unifier, out, potentials);
                    Ok(())
                });
            }
            BinaryOperator::Equals | BinaryOperator::NotEquals => {
                if !unifier.unify_concrete_only_exact(left_root, right_root) {
                    unifier.delayed_error(move |unifier| {
                        self.errors.type_error(
                            "operator ==",
                            span,
                            right_root.display_substitute(self.globals, unifier),
                            left_root.display_substitute(self.globals, unifier),
                        );
                    });
                }
                assert_eq!(out_root.unwrap_named().id, get_builtin_type!("bool"));
            }
            BinaryOperator::GreaterEq
            | BinaryOperator::Greater
            | BinaryOperator::LesserEq
            | BinaryOperator::Lesser => {
                assert_eq!(left_root.unwrap_named().id, get_builtin_type!("int"));
                assert_eq!(right_root.unwrap_named().id, get_builtin_type!("int"));
                assert_eq!(out_root.unwrap_named().id, get_builtin_type!("bool"));
            }
        }
    }

    fn get_port_typ(&self, sm: &SubModule, port_id: PortID) -> Option<&ConcreteType> {
        let port = &sm.port_map[port_id].as_ref()?;
        let port_wire = &self.wires[port.maps_to_wire];
        Some(&port_wire.typ)
    }

    fn get_port_value(
        &self,
        sm: &SubModule,
        path: &InferenceTargetPath,
    ) -> Option<&UnifyableValue> {
        let port = &sm.port_map[path.port].as_ref()?;
        let port_wire = &self.wires[port.maps_to_wire];
        Some(path.follow_value_path(&port_wire.typ))
    }

    fn add_submodule_subtype_constraints(
        &'inst self,
        sm: &'inst SubModule,
        unifier: &ValueUnifier<'inst>,
    ) {
        assert!(sm.instance.get().is_none());

        let sm_md = &self.globals[sm.refers_to.id];

        for (_, concrete_param, infer_info) in crate::alloc::zip_eq(
            &sm.refers_to.template_args,
            &sm_md.inference_info.parameter_inference_candidates,
        ) {
            match concrete_param.and_by_ref(infer_info) {
                TemplateKind::Type((t_param, t_info)) => {
                    let (inputs, outputs) = t_info.candidates.split_at(t_info.num_inputs);
                    unifier.create_subtype_constraint(inputs.iter().filter_map(|c| {
                        let port = &sm.port_map[c.port].as_ref()?;
                        let port_wire = &self.wires[port.maps_to_wire];
                        Some((t_param, c.follow_type_path(&port_wire.typ)))
                    }));
                    for c in outputs {
                        let Some(port_typ) = self.get_port_typ(sm, c.port) else {
                            continue;
                        };
                        assert!(
                            unifier.unify_concrete_all(t_param, c.follow_type_path(port_typ)),
                            "Since this is the first time outputs are unified, this cannot fail"
                        )
                    }
                }
                TemplateKind::Value((v_param, v_info)) => {
                    if v_info.total_inference_strategy != ValueInferStrategy::Unify {
                        continue;
                        // Others handled by [Self::try_infer_submodule_params]
                    };
                    for c in &v_info.candidates[..v_info.total_inference_upto] {
                        assert_eq!(c.mul_by, IBig::from(1));
                        assert_eq!(c.offset, IBig::from(0));
                        assert_eq!(c.relation, SubtypeRelation::Exact);
                        let_unwrap!(InferenceTarget::Subtype(target_path), &c.target);
                        let Some(target_value) = &self.get_port_value(sm, target_path) else {
                            continue;
                        };
                        unifier.unify(v_param, target_value).expect(
                            "Since this is the first time values are unified, this cannot fail",
                        );
                    }
                }
            }
        }
    }

    fn get_infer_target_value(
        &'inst self,
        sm: &SubModule,
        candidate: &InferenceCandidate,
        unifier: &ValueUnifier<'inst>,
        latency_infer_problem: &mut LatencyInferenceProblem,
    ) -> InferenceResult {
        match &candidate.target {
            InferenceTarget::Subtype(path) => {
                let Some(port) = &sm.port_map[path.port] else {
                    return InferenceResult::PortNotUsed;
                };
                let port_wire = &self.wires[port.maps_to_wire];
                let v = path.follow_value_path(&port_wire.typ);
                let Ok(v) = unifier.resolve(v) else {
                    return InferenceResult::NotFound;
                };
                InferenceResult::Found(v.unwrap_integer().clone())
            }
            InferenceTarget::PortLatency { from, to } => {
                let (Some(from), Some(to)) = (&sm.port_map[*from], &sm.port_map[*to]) else {
                    return InferenceResult::PortNotUsed;
                };
                match latency_infer_problem.infer(from.maps_to_wire, to.maps_to_wire) {
                    Ok(result) => InferenceResult::Found(IBig::from(result)),
                    Err(InferenceFailure::BadProblem) => InferenceResult::LatencyBadProblem,
                    Err(InferenceFailure::NotReached) => InferenceResult::LatencyNotReached,
                    Err(InferenceFailure::Poison { edge_from, edge_to }) => {
                        let from_wire_id = latency_infer_problem
                            .latency_count_problem
                            .map_latency_node_to_wire[edge_from];
                        let to_wire_id = latency_infer_problem
                            .latency_count_problem
                            .map_latency_node_to_wire[edge_to];

                        let w_from = &self.wires[from_wire_id];
                        let w_to = &self.wires[to_wire_id];

                        let_unwrap!(
                            IsPort::SubmodulePort(in_submod_id, port_from, Direction::Input),
                            w_from.is_port
                        );
                        let_unwrap!(
                            IsPort::SubmodulePort(out_submod_id, port_to, Direction::Output),
                            w_to.is_port
                        );
                        let submod = all_equal([in_submod_id, out_submod_id]);

                        //w_from.is_port

                        InferenceResult::LatencyPoison {
                            submod,
                            port_from,
                            port_to,
                        }
                    }
                }
            }
        }
    }

    fn try_infer_submodule_params(
        &'inst self,
        unifier: &ValueUnifier<'inst>,
        sm_ids: &mut Vec<SubModuleID>,
    ) -> Vec<(SubModuleID, ConcreteGlobalReference<ModuleUUID>)> {
        let mut lat_inf = LatencyInferenceProblem::new(self);

        if crate::debug::is_enabled("dot-latency-infer") {
            display_latency_count_graph(
                &lat_inf.latency_count_problem,
                &self.wires,
                &self.submodules,
                self.globals,
                None,
                &self.name,
                "inference_problem",
            );
        }

        for sm_id in sm_ids.iter() {
            let sm = &self.submodules[*sm_id];
            let sm_md = &self.globals[sm.refers_to.id];

            for (_, concrete_param, last_vals, infer_info) in crate::alloc::zip_eq3(
                &sm.refers_to.template_args,
                sm.last_infer_values.borrow_mut().iter_mut(),
                &sm_md.inference_info.parameter_inference_candidates,
            ) {
                let TemplateKind::Value((concrete_param, infer_info)) =
                    concrete_param.and_by_ref(infer_info)
                else {
                    continue;
                };
                match infer_info.total_inference_strategy {
                    ValueInferStrategy::Unify => continue,
                    ValueInferStrategy::Exact
                    | ValueInferStrategy::Min
                    | ValueInferStrategy::Max => {
                        for (info, stored) in crate::util::zip_eq(
                            &infer_info.candidates[..infer_info.total_inference_upto],
                            last_vals.iter_mut(),
                        ) {
                            *stored = self.get_infer_target_value(sm, info, unifier, &mut lat_inf);
                        }
                    }
                }
                if unifier.resolve(concrete_param).is_ok() {
                    continue;
                }
                let value_iter = crate::util::zip_eq(
                    &infer_info.candidates[..infer_info.total_inference_upto],
                    last_vals,
                );
                let mut total = None;
                match infer_info.total_inference_strategy {
                    ValueInferStrategy::Unify => {
                        continue; // Handled by [Self::add_submodule_subtype_constraints]
                    }
                    ValueInferStrategy::Exact => {
                        for (info, last) in value_iter {
                            match last {
                                InferenceResult::PortNotUsed => continue,
                                InferenceResult::NotFound
                                | InferenceResult::LatencyBadProblem
                                | InferenceResult::LatencyNotReached
                                | InferenceResult::LatencyPoison { .. } => {
                                    continue;
                                }
                                InferenceResult::Found(v) => {
                                    let (div, rem) = (&*v - &info.offset).div_rem(&info.mul_by);
                                    if rem == IBig::from(0) {
                                        // Ignore exact values that don't divide properly
                                        total = Some(div);
                                        break; // Success! We've found *a* value for the parameter
                                    }
                                }
                            }
                        }
                    }
                    ValueInferStrategy::Min => {
                        // Constraints:
                        // V * 3 + 5 >= 6 -> V at least 1
                        // V * -2 + 3 <= 6 -> V at least -1
                        //
                        // => Minimize Integer V such that
                        // V >= 1 / 3
                        // V >= 3 / -2 -> ceil-div
                        for (info, last) in value_iter {
                            match last {
                                InferenceResult::PortNotUsed => continue, // Missing ports are okay, just skip their inference value
                                InferenceResult::NotFound
                                | InferenceResult::LatencyBadProblem
                                | InferenceResult::LatencyNotReached
                                | InferenceResult::LatencyPoison { .. } => {
                                    total = None;
                                    break; // Missing value means failed inference
                                }
                                InferenceResult::Found(v) => {
                                    let needed = ceil_div(&*v - &info.offset, &info.mul_by);
                                    total = Some(if let Some(cur_total) = total {
                                        IBig::min(cur_total, needed)
                                    } else {
                                        needed
                                    });
                                }
                            }
                        }
                    }
                    ValueInferStrategy::Max => {
                        // Constraints:
                        // V * 3 + 5 <= 6 -> V at most 0
                        // V * -2 + 3 >= 6 -> V at most -2
                        //
                        // => Maximize Integer V such that
                        // V <= 1 / 3
                        // V <= 3 / -2 -> floor-div
                        for (info, last) in value_iter {
                            match last {
                                InferenceResult::PortNotUsed => continue, // Missing ports are okay, just skip their inference value
                                InferenceResult::NotFound
                                | InferenceResult::LatencyBadProblem
                                | InferenceResult::LatencyNotReached
                                | InferenceResult::LatencyPoison { .. } => {
                                    total = None;
                                    break; // Missing value means failed inference
                                }
                                InferenceResult::Found(v) => {
                                    let needed = floor_div(&*v - &info.offset, &info.mul_by);
                                    total = Some(if let Some(cur_total) = total {
                                        IBig::max(cur_total, needed)
                                    } else {
                                        needed
                                    });
                                }
                            }
                        }
                    }
                }
                if let Some(total) = total {
                    unifier.set(concrete_param, Value::Integer(total)).unwrap();
                    // Success! We found the inferred value!
                }
            }
        }

        let mut recursive_submodules_to_instantiate = Vec::new();
        // And now instantiate the modules we can
        sm_ids.retain(|id| {
            let sm = &self.submodules[*id];

            // Doing can_fully_substitute first saves on quite a few clones. TODO remove once we switch to unifyable_cell.rs, as that can do fully_substitute without needing &mut.
            if sm.refers_to.fully_substitute(unifier).is_ok() {
                recursive_submodules_to_instantiate.push((*id, sm.refers_to.clone()));

                false
            } else {
                true
            }
        });

        recursive_submodules_to_instantiate
    }
    fn apply_instantiated_submodule(
        &'inst self,
        sm_id: SubModuleID,
        instance: Result<Rc<InstantiatedModule>, InstantiateError>,
        unifier: &ValueUnifier<'inst>,
    ) {
        let sm = &self.submodules[sm_id];
        let submod_instr = &self.link_info.instructions[sm.original_instruction];

        let instance = match instance {
            Ok(instance) => instance,
            Err(InstantiateError::ErrorInModule) => {
                self.errors.set_did_error();
                // Don't report errors on submodule instantiation erroring. This should reduce the "error overload" certainly in recursive modules. (See #146)
                /*self.errors
                .error(submod_instr.get_span(), "Error instantiating submodule");*/
                return;
            }
            Err(InstantiateError::RecursionLimitExceeded { message }) => {
                self.errors.error(submod_instr.get_span(), message);
                return;
            }
        };
        let sub_module = &self.globals.modules[sm.refers_to.id];

        sm.instance.set(instance).unwrap();
        let instance = sm.instance.get().unwrap();
        for (_port_id, concrete_port, source_code_port, connecting_wire) in
            zip_eq3(&instance.interface_ports, &sub_module.ports, &sm.port_map)
        {
            match (concrete_port, connecting_wire) {
                (None, None) => {} // Invalid port not connected, good!
                (None, Some(connecting_wire)) => {
                    // Port is not enabled, but attempted to be used
                    // A question may be "What if no port was in the source code? There would be no error reported"
                    // But this is okay, because nonvisible ports are only possible for function calls
                    // We have a second routine that reports invalid interfaces.
                    for span in &connecting_wire.name_refs {
                        let port_name = &source_code_port.name;
                        let err = format!(
                            "Port '{port_name}' is used, but the instantiated module has this port disabled"
                        );
                        self.errors
                            .error(*span, err)
                            .info_obj(source_code_port)
                            .info_obj(submod_instr);
                    }
                }
                (Some(_concrete_port), None) => {
                    // Port is enabled, but not used
                    self.errors
                        .warn(
                            submod_instr.get_span(),
                            format!("Unused port '{}'", source_code_port.name),
                        )
                        .info_obj(source_code_port)
                        .info_obj(submod_instr);
                }
                (Some(concrete_port), Some(connecting_wire)) => {
                    let wire = &self.wires[connecting_wire.maps_to_wire];
                    let instance_wire = &instance.wires[concrete_port.wire];
                    assert!(instance_wire.typ.is_valid());
                    // Failures are reported in final_checks
                    let _ = match source_code_port.direction {
                        // Subtype relations always flow FORWARD.
                        Direction::Input => {
                            unifier.unify_concrete_only_exact(&wire.typ, &instance_wire.typ)
                        }
                        Direction::Output => {
                            unifier.unify_concrete_all(&wire.typ, &instance_wire.typ)
                        }
                    };
                }
            }
        }
    }
    /// Calls [ConcreteType::fully_substitute] on everything. After this the unifier may be decomissioned
    fn fully_substitute_everything(&'inst self, unifier: &ValueUnifier<'inst>) {
        // Don't report "could not figure out" errors if *any* other error has been reported before, because it confuses the user. Fixing the other error may resolve this one.
        // This is mostly from my own experience chasing down "could not infer latency parameter" errors, that were due to a bad problem -_-
        let did_already_error = self.errors.did_error();

        for (_, w) in &self.wires {
            if w.typ.fully_substitute(unifier).is_err() {
                let span = w.get_span(self.link_info);
                span.debug();
                if !did_already_error {
                    self.errors.error(
                        span,
                        format!(
                            "Some parameters of '{}' were still unknown: {}",
                            w.name,
                            w.typ.display(self.globals)
                        ),
                    );
                }
            } else if !w.typ.is_valid() {
                self.errors.error(
                    w.get_span(self.link_info),
                    format!(
                        "The type of '{}' is invalid! {}",
                        w.name,
                        w.typ.display(self.globals)
                    ),
                );
            }
            match &w.source {
                RealWireDataSource::UnaryOp { rank, .. }
                | RealWireDataSource::BinaryOp { rank, .. } => {
                    for r in rank {
                        // Rank not fully substituting is caught by the fully_substitute calls on w
                        let _ = r.fully_substitute(unifier);
                    }
                }
                _ => {}
            }
        }

        for sm_id in self.submodules.id_range() {
            let sm = &self.submodules[sm_id];
            let fully_substitute_result = sm.refers_to.fully_substitute(unifier);
            if !did_already_error {
                if fully_substitute_result.is_err() {
                    let sm_name = &sm.name;
                    let mut err = self.errors.error(
                        sm.get_span(self.link_info),
                        format!(
                            "Some submodule parameters of {sm_name} were still unknown: {}\n{}",
                            sm.refers_to.display(self.globals),
                            display_all_infer_params(self.globals, &self.submodules, sm)
                        ),
                    );
                    for (template_id, known_values) in sm.last_infer_values.borrow().iter() {
                        for known_v in known_values {
                            match known_v {
                                InferenceResult::LatencyPoison {
                                    submod,
                                    port_from,
                                    port_to,
                                } => {
                                    let sm_md = &self.globals.modules[sm.refers_to.id];
                                    let template_name =
                                        &sm_md.link_info.parameters[template_id].name;

                                    let poison_sm = &self.submodules[*submod];
                                    let poison_submod_md =
                                        &self.globals.modules[poison_sm.refers_to.id];

                                    let sm_name = &sm.name;
                                    let poison_sm_name = &poison_sm.name;
                                    let poison_sm_refer_to =
                                        poison_sm.refers_to.display(self.globals);
                                    let from_port_name = &poison_submod_md.ports[*port_from].name;
                                    let to_port_name = &poison_submod_md.ports[*port_to].name;
                                    err.info(
                                        poison_sm.get_span(self.link_info),
                                        format!("{sm_name}.{template_name} could not be resolved due to the unknown latency from {poison_sm_name}.{from_port_name} to {poison_sm_name}.{to_port_name}. ({poison_sm_refer_to} {poison_sm_name})"),
                                    );
                                }
                                InferenceResult::PortNotUsed
                                | InferenceResult::NotFound
                                | InferenceResult::LatencyBadProblem
                                | InferenceResult::LatencyNotReached
                                | InferenceResult::Found(_) => {}
                            }
                        }
                    }
                } else if let Err(reason) = sm.refers_to.report_if_errors(
                    self.globals,
                    "Invalid arguments found in a submodule's template arguments",
                ) {
                    self.errors.error(sm.get_span(self.link_info), reason);
                }
            }
        }
    }
    fn finalize(&mut self) {
        for w_id in self.wires.id_range() {
            let w = &mut self.wires[w_id];
            match &mut w.source {
                RealWireDataSource::Multiplexer { sources, .. } => {
                    for s in sources {
                        Self::finalize_partial_bounds(&mut s.to_path, &w.typ);
                    }
                }
                RealWireDataSource::Select { root, .. } => {
                    let root_id = *root;
                    let target_id = w_id;
                    let [target, root] = self.wires.get_disjoint_mut([target_id, root_id]).unwrap();
                    let_unwrap!(
                        RealWireDataSource::Select { root: _, path },
                        &mut target.source
                    );
                    Self::finalize_partial_bounds(path, &root.typ);
                }
                _ => {}
            }
        }

        for (_, sm) in &mut self.submodules {
            if let Some(instance) = sm.instance.get() {
                for (_port_id, concrete_port, connecting_wire) in
                    zip_eq(&instance.interface_ports, &sm.port_map)
                {
                    let (Some(concrete_port), Some(connecting_wire)) =
                        (concrete_port, connecting_wire)
                    else {
                        continue;
                    };

                    // We overwrite the ports, to have the Multiplexer inputs correctly call type errors on submodule inputs
                    let connecting_wire = &mut self.wires[connecting_wire.maps_to_wire];
                    let instance_wire = &instance.wires[concrete_port.wire];
                    assert!(instance_wire.typ.is_valid());
                    connecting_wire.typ.clone_from(&instance_wire.typ);
                }
            }
        }
    }

    fn finalize_partial_bounds(path: &mut [RealWirePathElem], mut typ: &ConcreteType) {
        for pe in path {
            match pe {
                RealWirePathElem::Index { .. } | RealWirePathElem::ConstIndex { .. } => {
                    typ = &typ.unwrap_array().0;
                }
                RealWirePathElem::PartSelect { .. } => {
                    typ = &typ.unwrap_array().0;
                }
                RealWirePathElem::Slice { bounds, .. } => {
                    // TODO: #88: Variable base arrays, that's why this is part here
                    let (new_typ, sz) = typ.unwrap_array();
                    typ = new_typ;

                    if let Some(sz) = sz.get() {
                        let sz = sz.unwrap_integer();
                        *bounds = match std::mem::replace(bounds, PartialBound::WholeSlice) {
                            PartialBound::Known(from, to) => PartialBound::Known(from, to),
                            PartialBound::From(from) => PartialBound::Known(from, sz.clone()),
                            PartialBound::To(to) => PartialBound::Known(IBig::from(0), to),
                            PartialBound::WholeSlice => {
                                PartialBound::Known(IBig::from(0), sz.clone())
                            }
                        };
                    }
                }
            }
        }
    }
}
