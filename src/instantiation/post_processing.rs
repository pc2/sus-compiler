use super::*;

use crate::{
    instantiation::{ModuleTypingContext, path_ranges::PathRange},
    util::zip_eq,
};

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
        for wire_id in self.wires.id_range() {
            let w = &self.wires[wire_id];
            let RealWireDataSource::Multiplexer { is_state, sources } = &w.source else {
                continue;
            };

            if is_state.is_some() {
                continue; // Can't remove conditional assigns from state vars
            }

            let mut path_range: PathRange<usize> = PathRange::new(0);

            for path in sources {
                path_range.count_uses(&path.to_path, &self.wires, &w.typ);
            }

            let is_mux_unneededs: Vec<bool> = sources
                .iter()
                .map(|path| path_range.is_used_eactly_once(&path.to_path, &self.wires, &w.typ))
                .collect();

            let RealWireDataSource::Multiplexer { sources, .. } = &mut self.wires[wire_id].source
            else {
                unreachable!("Checking same mux again");
            };
            for (mux, unneeded) in zip_eq(sources, is_mux_unneededs) {
                if unneeded {
                    mux.condition = Box::new([]);
                }
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
                        PartialBound::WholeSlice => PartialBound::Known(IBig::from(0), sz.clone()),
                    };
                }
            }
        }
    }
}
