pub fn all_equal<T, O: Eq + std::fmt::Debug>(
    iter: impl IntoIterator<Item = T>,
    mut f: impl FnMut(T) -> O,
) -> Option<O> {
    let mut iter = iter.into_iter();
    let first = f(iter.next()?);
    for v in iter {
        assert_eq!(first, f(v));
    }
    Some(first)
}

pub fn zip_eq<OA, OB>(
    iter_a: impl IntoIterator<Item = OA>,
    iter_b: impl IntoIterator<Item = OB>,
) -> impl Iterator<Item = (OA, OB)> {
    ZippedIterator {
        iter_a: iter_a.into_iter(),
        iter_b: iter_b.into_iter(),
    }
}

#[derive(Debug)]
pub struct ZippedIterator<OA, OB, IterA: Iterator<Item = OA>, IterB: Iterator<Item = OB>> {
    iter_a: IterA,
    iter_b: IterB,
}

impl<OA, OB, IterA: Iterator<Item = OA>, IterB: Iterator<Item = OB>> Iterator
    for ZippedIterator<OA, OB, IterA, IterB>
{
    type Item = (OA, OB);

    #[track_caller]
    fn next(&mut self) -> Option<Self::Item> {
        match (self.iter_a.next(), self.iter_b.next()) {
            (None, None) => None,
            (Some(a), Some(b)) => Some((a, b)),
            _ => unreachable!("Unbalanced Iterators"),
        }
    }
}
