use super::*;

use crate::instantiation::ModuleTypingContext;

impl<'l> ModuleTypingContext<'l> {
    pub fn post_process(&mut self) {
        self.finalize_all_partial_bounds();
        self.finalize_generative_vars();
        self.remove_unconditional_muxes();
    }
    fn finalize_all_partial_bounds(&mut self) {
        for w_id in self.wires.id_range() {
            let w = &mut self.wires[w_id];
            match &mut w.source {
                RealWireDataSource::Multiplexer {
                    sources,
                    is_state: _,
                } => {
                    for s in sources {
                        finalize_partial_bounds(&mut s.to_path, &w.typ);
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
                    finalize_partial_bounds(path, &root.typ);
                }
                _ => {}
            }
        }
    }
    fn finalize_generative_vars(&mut self) {
        for w_id in self.wires.id_range() {
            let w = &mut self.wires[w_id];
            match &mut w.source {
                RealWireDataSource::Multiplexer {
                    is_state: Some(is_state),
                    ..
                } => {
                    is_state.size_unsized_arrays(&w.typ);
                }
                RealWireDataSource::Constant { value } => {
                    value.size_unsized_arrays(&w.typ);
                }
                _ => {}
            }
        }
    }

    /// Remove effectively unconditional muxes. This could be made optional, if so desired at some point.
    /// It's needed because it seems that sometimes vivado can't optimize out the mux in `result = cond ? value : 'x`.
    /// It turned out in one case that this was needed, so I have the compiler itself do it.
    fn remove_unconditional_muxes(&mut self) {
        for (_, w) in &mut self.wires {
            let RealWireDataSource::Multiplexer { is_state, sources } = &mut w.source else {
                continue;
            };

            if is_state.is_some() {
                continue; // Can't remove conditional assigns from state vars
            }

            remove_unconditional_muxes(sources);
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
                        PartialBound::WholeSlice => PartialBound::Known(IBig::from(0), sz.clone()),
                    };
                }
            }
        }
    }
}

enum PathElemRange {
    All,
    Range(std::ops::Range<IBig>),
}

impl RealWirePathElem {
    fn get_path_elem_range(&self) -> PathElemRange {
        match self {
            RealWirePathElem::Index { .. } | RealWirePathElem::PartSelect { .. } => {
                PathElemRange::All
            }
            RealWirePathElem::ConstIndex { span: _, idx } => {
                PathElemRange::Range(idx.clone()..(idx + 1))
            }
            RealWirePathElem::Slice { span: _, bounds } => {
                let PartialBound::Known(from, to) = bounds else {
                    unreachable!("Bounds have been set to Known by finalize_partial_bounds");
                };
                PathElemRange::Range(from.clone()..to.clone())
            }
        }
    }
}

fn paths_intersect(a: &[RealWirePathElem], b: &[RealWirePathElem]) -> bool {
    for (path_a, path_b) in a.iter().zip(b.iter()) {
        let range_a = path_a.get_path_elem_range();
        let range_b = path_b.get_path_elem_range();

        match (range_a, range_b) {
            (PathElemRange::All, _) | (_, PathElemRange::All) => {}
            (PathElemRange::Range(range_a), PathElemRange::Range(range_b)) => {
                let bounds_dont_intersect =
                    range_a.start >= range_b.end || range_b.start >= range_a.end;

                if bounds_dont_intersect {
                    return false;
                }
            }
        }
    }
    true
}

fn any_paths_intersect(mux: &[MultiplexerSource]) -> bool {
    for (a_idx, a) in mux.iter().enumerate() {
        for (b_idx, b) in mux.iter().enumerate() {
            if a_idx == b_idx {
                continue;
            }
            if paths_intersect(&a.to_path, &b.to_path) {
                return true;
            }
        }
    }
    false
}

/// Technically an N^2 algorithm over the assignments. Let's hope the user doens't use too many.
fn remove_unconditional_muxes(mux: &mut [MultiplexerSource]) {
    if !any_paths_intersect(mux) {
        for m in mux {
            m.condition = Box::new([]);
        }
    }
}
