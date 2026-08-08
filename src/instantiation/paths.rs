use std::{
    fmt::Display,
    ops::{Deref as _, Range},
};

use ibig::error::OutOfBoundsError;

use super::*;
use crate::{
    instantiation::execute::ExecutionResult,
    to_string::{FmtWrapper, display_maybe},
    util::zip_eq,
};

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

        match first {
            RealWirePathElem::Index { idx_wire, .. } => {
                let idx_wire_bounds = wires[*idx_wire].typ.unwrap_int_bounds();
                let from: usize = idx_wire_bounds.from.try_into()?;
                let to: usize = idx_wire_bounds.to.try_into()?;

                self.apply_range(wires, rest, typ, from..to, f)
            }
            RealWirePathElem::ConstIndex { idx, .. } => {
                let v: usize = idx.try_into()?;

                self.apply_range(wires, rest, typ, v..v + 1, f)
            }
            RealWirePathElem::PartSelect {
                from_wire,
                width,
                direction,
                ..
            } => {
                let from_wire_bounds = wires[*from_wire].typ.unwrap_int_bounds();

                let from: usize = from_wire_bounds.from.try_into()?;
                let to: usize = from_wire_bounds.to.try_into()?;
                let width: usize = width.try_into()?;

                for idx in from..to {
                    let select_range = direction.range_from(idx, width);
                    self.apply_range(wires, rest, typ, select_range.from..select_range.to, f)?;
                }
                Ok(())
            }
            RealWirePathElem::Slice { bounds, .. } => {
                let_unwrap!(PartialBound::Known(from, to), bounds);

                let from: usize = from.try_into()?;
                let to: usize = to.try_into()?;

                self.apply_range(wires, rest, typ, from..to, f)
            }
        }
    }
    fn apply_range(
        &mut self,
        wires: &FlatAlloc<RealWire, WireIDMarker>,
        rest: &[RealWirePathElem],
        typ: &ConcreteType,
        requested_range: Range<usize>,
        f: &mut impl FnMut(&mut T),
    ) -> Result<(), OutOfBoundsError> {
        let (content, sz) = typ.unwrap_array();
        let Some(sz) = sz.get() else {
            return Err(OutOfBoundsError);
        };
        let sz: usize = sz.unwrap_integer().try_into()?;

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
                        let (content, _sz) = arr.deref();
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

pub enum GenerativeWireRefPathElem {
    ArrayAccess {
        idx: IBig,
        span: Span,
    },
    Slice {
        from: Option<IBig>,
        to: Option<IBig>,
        span: Span,
    },
}
impl GenerativeWireRefPathElem {
    pub fn display_path(path: &[GenerativeWireRefPathElem]) -> impl Display {
        FmtWrapper(move |f| {
            for p in path {
                match p {
                    GenerativeWireRefPathElem::ArrayAccess { idx, span: _ } => {
                        write!(f, "[{idx}]")?
                    }
                    GenerativeWireRefPathElem::Slice { from, to, span: _ } => {
                        let from = display_maybe(from.as_ref(), |f, from| from.fmt(f));
                        let to = display_maybe(to.as_ref(), |f, to| to.fmt(f));
                        write!(f, "[{from}:{to}]")?
                    }
                }
            }
            Ok(())
        })
    }
}

impl Value {
    pub fn write(
        &mut self,
        path: Vec<GenerativeWireRefPathElem>,
        to_write: Value,
    ) -> ExecutionResult<()> {
        fn array_access<'t>(
            tgt_ref: &'t mut Value,
            idx: &IBig,
            span: Span,
        ) -> (ExecutionResult<()>, &'t mut Value) {
            let idx_as_usize = usize::try_from(idx).ok();

            let Value::Array(tgt_arr) = tgt_ref else {
                unreachable!()
            };
            let arr_sz = tgt_arr.len();

            if idx_as_usize.and_then(|idx| tgt_arr.get_mut(idx)).is_some() {
                // Once we know we're safe, we have to do the little dance again, such that this time we *consume* tgt_ref
                let Value::Array(tgt_arr) = tgt_ref else {
                    unreachable!()
                };
                (Ok(()), tgt_arr.get_mut(idx_as_usize.unwrap()).unwrap())
            } else {
                let err = Err(CompileError::error(
                    span,
                    format!("Index {idx} out of bounds for array of size {arr_sz}"),
                ));
                (err, tgt_ref)
            }
        }

        let mut cur_targets: Vec<(&mut Value, Value)> = vec![(self, to_write)];

        for path_elem in path {
            match path_elem {
                GenerativeWireRefPathElem::ArrayAccess { idx, span } => {
                    for target in &mut cur_targets {
                        replace_with::replace_with_or_abort_and_return(&mut target.0, |tgt| {
                            array_access(tgt, &idx, span)
                        })?;
                    }
                }
                GenerativeWireRefPathElem::Slice { from, to, span } => {
                    let slice =
                        make_array_bounds(from, to, cur_targets.iter().map(|t| &*t.0), span)?;

                    let new_len = cur_targets.len() * slice.len();

                    let mut new_targets = Vec::with_capacity(new_len);

                    for (target, value) in cur_targets {
                        let_unwrap!(Value::Array(target), target);
                        let Value::Array(value) = value else {
                            unreachable!()
                        };

                        let slice_len = slice.len();
                        let from_len = value.len();
                        if from_len != slice_len {
                            let from = slice.start;
                            let to = slice.end;
                            return Err(CompileError::error(
                                span,
                                format!(
                                    "Attempting to write to this slice {from}:{to} (length {slice_len}) with an array of length {from_len}."
                                ),
                            ));
                        }
                        for new_pair in zip_eq(
                            target.get_slice_mut(slice.clone()).unwrap(),
                            value.into_iter(),
                        ) {
                            new_targets.push(new_pair)
                        }
                    }

                    cur_targets = new_targets;
                }
            }
        }

        for (t, f) in cur_targets {
            *t = f;
        }

        Ok(())
    }
}

pub fn make_array_bounds<'v>(
    from_maybe: Option<IBig>,
    to_maybe: Option<IBig>,
    mut values: impl Iterator<Item = &'v Value>,
    span: Span,
) -> ExecutionResult<Range<usize>> {
    if let Some(first) = values.next() {
        let_unwrap!(Value::Array(arr), first);

        let arr_sz = arr.len();

        let is_dynamic_range = from_maybe.is_none() || to_maybe.is_none();

        let from = from_maybe.unwrap_or_else(|| IBig::from(0));
        let to = to_maybe.unwrap_or_else(|| IBig::from(arr_sz));

        if from > to {
            return Err(CompileError::error(
                span,
                format!("Slice {from}:{to} has a negative length."),
            ));
        }

        let (from_valid, to_valid) = match (usize::try_from(&from), usize::try_from(&to)) {
            (Ok(from), Ok(to)) if to <= arr_sz => (from, to), // && from >= 0, but it's usize
            _ => {
                let e = format!(
                    "Slice {from}:{to} is out of bounds. The size of this array is {arr_sz}"
                );
                return Err(CompileError::error(span, e));
            }
        };

        for v in values {
            let_unwrap!(Value::Array(arr), v);

            let other_arr_sz = arr.len();

            if is_dynamic_range && other_arr_sz != arr_sz {
                let e = "Using a variable index on a jagged array".to_string();
                return Err(CompileError::error(span, e));
            }

            if to_valid > other_arr_sz {
                let e = format!(
                    "Slice {from}:{to} is out of bounds. The size of this array is {other_arr_sz}"
                );
                return Err(CompileError::error(span, e));
            }
        }

        Ok(from_valid..to_valid)
    } else {
        Ok(0..0)
    }
}
