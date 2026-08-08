use std::ops::{Deref as _, Range};

use ibig::error::OutOfBoundsError;

use super::*;

#[derive(Debug, Clone)]
/// Describes all "slots" of a wire.
/// For arrays this is all elements of the array
/// For structs, this is each field of the struct
pub enum PathRange<T: Clone> {
    Full(T),
    Partial(Vec<PathRange<T>>),
}

impl<T: Clone> PathRange<T> {
    pub fn new(default: T) -> Self {
        Self::Full(default)
    }

    pub fn apply_on_all_nested(&mut self, f: &mut impl FnMut(&mut T)) {
        match self {
            PathRange::Full(existing_v) => {
                f(existing_v);
            }
            PathRange::Partial(path_ranges) => {
                for p in path_ranges {
                    p.apply_on_all_nested(f);
                }
            }
        }
    }
    /// Returns Err(OutOfBoundsError) when casting IBigs to usize fails
    pub fn apply(
        &mut self,
        path: &[RealWirePathElem],
        wires: &FlatAlloc<RealWire, WireIDMarker>,
        typ: &ConcreteType,
        f: &mut impl FnMut(&mut T),
    ) -> Result<(), OutOfBoundsError> {
        let Some((first, rest)) = path.split_first() else {
            self.apply_on_all_nested(f);
            return Ok(());
        };

        let (content, sz) = typ.unwrap_array();
        let Some(sz) = sz.get() else {
            return Err(OutOfBoundsError);
        };
        let sz: usize = sz.unwrap_integer().try_into()?;

        let requested_range: Range<usize> = match first {
            RealWirePathElem::Index { idx_wire, .. } => {
                let idx_wire_bounds = wires[*idx_wire].typ.unwrap_int_bounds();
                let from: usize = idx_wire_bounds.from.try_into()?;
                let to: usize = idx_wire_bounds.to.try_into()?;
                from..to
            }
            RealWirePathElem::ConstIndex { idx, .. } => {
                let v: usize = idx.try_into()?;
                v..v + 1
            }
            RealWirePathElem::PartSelect {
                from_wire,
                width,
                direction,
                ..
            } => {
                let from_wire_bounds = wires[*from_wire].typ.unwrap_int_bounds();

                let select_range = direction.range_from_range(from_wire_bounds, width);

                let from: usize = select_range.from.try_into()?;
                let to: usize = select_range.to.try_into()?;
                from..to
            }
            RealWirePathElem::Slice { bounds, .. } => {
                let_unwrap!(PartialBound::Known(from, to), bounds);

                let from: usize = from.try_into()?;
                let to: usize = to.try_into()?;
                from..to
            }
        };

        match std::mem::replace(self, PathRange::Partial(Vec::new())) {
            PathRange::Full(shared_v) => {
                *self = PathRange::Partial(vec![PathRange::Full(shared_v); sz]);
            }
            PathRange::Partial(path_ranges) => {
                *self = PathRange::Partial(path_ranges);
            }
        }

        let PathRange::Partial(nested_ranges) = self else {
            unreachable!()
        };
        for idx in requested_range {
            nested_ranges[idx].apply(rest, wires, content, f)?;
        }
        Ok(())
    }
    pub fn all(&self, f: &mut impl FnMut(&T) -> bool) -> bool {
        match self {
            PathRange::Full(v) => f(v),
            PathRange::Partial(path_ranges) => path_ranges.iter().all(|sub_range| sub_range.all(f)),
        }
    }
}

impl PathRange<usize> {
    pub fn count_uses(
        &mut self,
        path: &[RealWirePathElem],
        wires: &FlatAlloc<RealWire, WireIDMarker>,
        typ: &ConcreteType,
    ) {
        let _ = self.apply(path, wires, typ, &mut |v| *v += 1);
    }
    pub fn is_used_eactly_once(
        &mut self,
        path: &[RealWirePathElem],
        wires: &FlatAlloc<RealWire, WireIDMarker>,
        typ: &ConcreteType,
    ) -> bool {
        let mut used_exactly_once = true;
        let _ = self.apply(path, wires, typ, &mut |v| {
            assert!(*v >= 1);
            if *v >= 2 {
                used_exactly_once = false;
            }
        });
        used_exactly_once
    }
    pub fn find_unused_path(&self, typ: &ConcreteType) -> Option<Vec<Range<usize>>> {
        match self {
            PathRange::Full(0) => Some(Vec::new()),
            PathRange::Full(_) => None,
            PathRange::Partial(sub_ranges) => {
                match typ {
                    ConcreteType::Named(_) => todo!("Structs"),
                    ConcreteType::Array(arr) => {
                        let (content, sz) = arr.deref();
                        let mut found_missing: Option<(usize, Vec<Range<usize>>)> = None;
                        for (idx, p) in sub_ranges.iter().enumerate() {
                            if let Some(found_missing) = &mut found_missing {
                                if let Some(found_sub_path) = p.find_unused_path(content)
                                    && found_sub_path == found_missing.1
                                {
                                } else {
                                    // The end of the unused region.
                                    let range_here = found_missing.0..idx;
                                    let mut found_missing = std::mem::take(&mut found_missing.1);
                                    found_missing.insert(0, range_here);
                                    return Some(found_missing);
                                }
                            } else if let Some(found_sub_path) = p.find_unused_path(content) {
                                found_missing = Some((idx, found_sub_path));
                            }
                        }
                    }
                }
                None
            }
        }
    }
}
