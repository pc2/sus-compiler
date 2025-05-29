use std::fmt::Debug;

/*pub fn all_equal_in_iter<T: Eq + Debug>(iter: impl IntoIterator<Item = T>) -> Option<T> {
    let mut iter = iter.into_iter();
    let first = iter.next()?;
    for v in iter {
        assert_eq!(first, v);
    }
    Some(first)
}*/

pub fn all_equal<T: Eq + Debug, const N: usize>(vs: [T; N]) -> T {
    // const _: () = const { assert!(N > 0) };
    let mut iter = vs.into_iter();
    let first = iter.next().unwrap();
    for v in iter {
        assert_eq!(first, v);
    }
    first
}

#[track_caller]
pub fn unwrap_single_element<T>(collection: impl IntoIterator<Item = T>) -> T {
    let mut iter = collection.into_iter();
    let result = iter.next().unwrap();
    assert!(iter.next().is_none());
    result
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

/// Merges a and b, order is unimportant
pub fn merge_vec_into<T>(a: &mut Vec<T>, mut b: Vec<T>) {
    if a.len() < b.len() {
        std::mem::swap(a, &mut b);
    }
    a.append(&mut b);
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

#[macro_export]
macro_rules! let_unwrap {
    ($pat:pat, $val:expr) => {
        // First try matching by reference to avoid consuming
        let let_unwrap_executed_expr = $val;
        #[allow(unused_variables)]
        let $pat = &let_unwrap_executed_expr
        else {
            panic!(
                "let_unwrap! failed: expected {}, found {:?}",
                stringify!($pat),
                let_unwrap_executed_expr
            );
        };
        // Now match by value (may move)
        let $pat = let_unwrap_executed_expr else {
            unreachable!("let_unwrap! internal error: pattern matched by ref, but failed by value")
        };
    };
}
