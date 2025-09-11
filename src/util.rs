use std::collections::HashSet;
use std::fmt::Debug;

use ibig::IBig;
use ibig::ops::DivRem;

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

pub fn contains_duplicates<T: Eq + std::hash::Hash>(iter: impl IntoIterator<Item = T>) -> bool {
    let mut seen = HashSet::new();
    for item in iter {
        if !seen.insert(item) {
            return true;
        }
    }
    false
}

/// Partitions a mutable slice in place according to a predicate.
///
/// All elements for which `pred` returns `true` are moved to the front of the slice,
/// and all elements for which it returns `false` are moved to the back.
/// The relative order of elements is **not** preserved.
///
/// Returns the index of the first element in the `false` partition.
///
/// # Example
///
/// ```
/// let mut arr = [1, 2, 3, 4, 5, 6];
/// let mid = partition_in_place(&mut arr, |&x| x % 2 == 0);
/// assert_eq!(mid, 3); // three even numbers at the start
/// assert!(arr[..mid].iter().all(|&x| x % 2 == 0));
/// assert!(arr[mid..].iter().all(|&x| x % 2 != 0));
/// ```
pub fn partition_in_place<T, F>(slice: &mut [T], mut pred: F) -> usize
where
    F: FnMut(&T) -> bool,
{
    let mut left = 0;
    let mut right = slice.len();

    while left != right {
        if pred(&slice[left]) {
            left += 1;
        } else {
            right -= 1;
            slice.swap(left, right);
        }
    }
    left
}

pub fn floor_div(a: IBig, b: &IBig) -> IBig {
    let different_signs = (a < IBig::from(0)) ^ (b < &IBig::from(0));
    let (div, rem) = a.div_rem(b);
    if rem != IBig::from(0) && different_signs {
        div - 1
    } else {
        div
    }
}
pub fn ceil_div(a: IBig, b: &IBig) -> IBig {
    let same_signs = (a < IBig::from(0)) == (b < &IBig::from(0));
    let (div, rem) = a.div_rem(b);
    if rem != IBig::from(0) && same_signs {
        div + 1
    } else {
        div
    }
}
#[test]
fn test_floor_and_ceil_div() {
    for ai in -10..=10 {
        for bi in -10..=10 {
            if bi == 0 {
                continue; // skip division by zero
            }
            let a = IBig::from(ai);
            let b = IBig::from(bi);

            // expected values from floating-point floor/ceil
            let af = ai as f64;
            let bf = bi as f64;
            let expected_floor = (af / bf).floor() as i64;
            let expected_ceil = (af / bf).ceil() as i64;

            let floor_result = floor_div(a.clone(), &b);
            let ceil_result = ceil_div(a, &b);

            assert_eq!(
                floor_result,
                IBig::from(expected_floor),
                "floor_div failed for a={ai}, b={bi}"
            );
            assert_eq!(
                ceil_result,
                IBig::from(expected_ceil),
                "ceil_div failed for a={ai}, b={bi}"
            );
        }
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

#[macro_export]
macro_rules! let_unwrap {
    ($pat:pat, $val:expr) => {
        // First try matching by reference to avoid consuming
        let __val = $val;
        #[allow(unused_variables)]
        let $pat = &__val else {
            panic!(
                "let_unwrap! failed: expected {}, found {:?}",
                stringify!($pat),
                __val
            );
        };
        // Now match by value (may move)
        let $pat = __val else {
            unreachable!("let_unwrap! internal error: pattern matched by ref, but failed by value")
        };
    };
}

#[macro_export]
macro_rules! unwrap_variant {
    ($variant:path, $val:expr) => {{
        // First try matching by reference to avoid consuming
        let __val = $val;
        #[allow(unused_variables)]
        let $variant(__content) = &__val else {
            panic!(
                "let_unwrap! failed: expected {}(_), found {:?}",
                stringify!($variant),
                __val
            );
        };
        // Now match by value (may move)
        let $variant(__content) = __val else {
            unreachable!("let_unwrap! internal error: pattern matched by ref, but failed by value")
        };
        __content
    }};
}
