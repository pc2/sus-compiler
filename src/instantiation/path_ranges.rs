use std::ops::Range;

use ibig::error::OutOfBoundsError;

use super::*;

#[derive(Debug, Clone)]
/// Describes all "slots" of a wire.
/// For arrays this is all elements of the array
/// For structs, this is each field of the struct
pub enum PathRange<T: Clone> {
    Full(T),
    Empty,
    Partial(Vec<PathRange<T>>),
}

impl<T: Clone> Default for PathRange<T> {
    fn default() -> Self {
        Self::new()
    }
}
impl<T: Clone> PathRange<T> {
    pub fn new() -> Self {
        Self::Empty
    }

    pub fn apply_on_all_nested(&mut self, default: T, f: &mut impl FnMut(&mut T)) {
        match self {
            PathRange::Full(existing_v) => {
                f(existing_v);
            }
            PathRange::Empty => {
                let mut new_v = default;
                f(&mut new_v);
                *self = PathRange::Full(new_v);
            }
            PathRange::Partial(path_ranges) => {
                for p in path_ranges {
                    p.apply_on_all_nested(default.clone(), f);
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
        default: T,
        f: &mut impl FnMut(&mut T),
    ) -> Result<(), OutOfBoundsError> {
        let Some((first, rest)) = path.split_first() else {
            self.apply_on_all_nested(default, f);
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

        match std::mem::replace(self, PathRange::Empty) {
            PathRange::Full(shared_v) => {
                *self = PathRange::Partial(vec![PathRange::Full(shared_v); sz]);
            }
            PathRange::Empty => {
                *self = PathRange::Partial(vec![PathRange::Full(default.clone()); sz]);
            }
            PathRange::Partial(path_ranges) => {
                *self = PathRange::Partial(path_ranges);
            }
        }

        let PathRange::Partial(nested_ranges) = self else {
            unreachable!()
        };
        for idx in requested_range {
            nested_ranges[idx].apply(rest, wires, content, default.clone(), f)?;
        }
        Ok(())
    }
}

impl PathRange<usize> {
    pub fn count_uses(
        &mut self,
        path: &[RealWirePathElem],
        wires: &FlatAlloc<RealWire, WireIDMarker>,
        typ: &ConcreteType,
    ) {
        let _ = self.apply(path, wires, typ, 0, &mut |v| *v += 1);
    }
    pub fn is_used_eactly_once(
        &mut self,
        path: &[RealWirePathElem],
        wires: &FlatAlloc<RealWire, WireIDMarker>,
        typ: &ConcreteType,
    ) -> bool {
        let mut used_exactly_once = true;
        let _ = self.apply(path, wires, typ, 0, &mut |v| {
            assert!(*v >= 1);
            if *v >= 2 {
                used_exactly_once = false;
            }
        });
        used_exactly_once
    }
}
