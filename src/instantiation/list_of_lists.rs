use std::{
    mem::MaybeUninit,
    ops::{Index, IndexMut},
};

#[derive(Debug, Clone)]
pub struct ListOfLists<T> {
    buf: Vec<T>,
    // A list of #groups+1 offsets in buf array. The end of each one is the start of the next one. They are laid out sequentially
    start_ends: Vec<usize>,
}

impl<T> ListOfLists<T> {
    pub fn new() -> Self {
        Self::new_with_groups_capacity(0)
    }

    pub fn new_with_groups_capacity(capacity: usize) -> Self {
        let mut start_ends = Vec::with_capacity(capacity + 1);
        start_ends.push(0);
        Self {
            buf: Vec::new(),
            start_ends,
        }
    }

    pub fn push_to_last_group(&mut self, item: T) {
        let last_group_end = self.start_ends.last_mut().unwrap();
        assert!(*last_group_end == self.buf.len());
        *last_group_end += 1;
        self.buf.push(item);
    }
    pub fn new_group(&mut self) {
        let last_group_end = self.start_ends.last().unwrap();
        self.start_ends.push(*last_group_end);
    }

    pub fn len(&self) -> usize {
        self.start_ends.len() - 1
    }
    pub fn iter_flattened(&self) -> std::slice::Iter<'_, T> {
        self.buf.iter()
    }
    pub fn iter_flattened_mut(&mut self) -> std::slice::IterMut<'_, T> {
        self.buf.iter_mut()
    }
    pub fn iter_flattened_by_bucket(&self) -> ListOfListsFlatOriginIter<'_, T> {
        ListOfListsFlatOriginIter {
            buf_iter: self.buf.iter().enumerate(),
            ends: &self.start_ends[1..],
            cur_slice_idx: 0,
        }
    }
    pub fn iter_flattened_by_bucket_mut(&mut self) -> ListOfListsFlatOriginIterMut<'_, T> {
        ListOfListsFlatOriginIterMut {
            buf_iter: self.buf.iter_mut().enumerate(),
            ends: &self.start_ends[1..],
            cur_slice_idx: 0,
        }
    }
    pub fn iter(&self) -> ListOfListsIter<'_, T> {
        self.into_iter()
    }
    pub fn iter_mut(&mut self) -> ListOfListsIterMut<'_, T> {
        self.into_iter()
    }

    /*
        Takes an iterator that produces a stream of locations and the item to store there.
        Runs through the entire iterator twice.
        Once to collect the size for each target group, and once to place all the results

        MUST pass a cloneable iterator that iterates through all elements you wish to add.
        A clone of the iterator may not behave differently
    */
    pub fn from_random_access_iterator<IterT: Iterator<Item = (usize, T)> + Clone>(
        num_groups: usize,
        iter: IterT,
    ) -> Self {
        // We'll be reusing this vector for the resulting start_ends vector, so already have it at the right size
        // First we use the memory to collect group sizes
        let mut start_ends: Vec<usize> = vec![0; num_groups + 1];

        for (to_idx, _) in iter.clone() {
            start_ends[to_idx + 1] += 1;
        }

        /*
            Transforms the group sizes vector into storing basically the starts of each group in that group's end.
            Once we finish adding the elements of each group, this vector will be a valid start_ends vector.
            So starting with for example group sizes [0, 2, 3, 1], this converts the vector to [0, 0, 2, 5] with cumulative_sum = 6.
            Finally adding all elements, this brings our vector to [0, 2, 5, 6], which is the correct start_end vector for this
        */
        let mut cumulative_sum = 0;
        for s in &mut start_ends {
            let found_value = *s;
            *s = cumulative_sum;
            cumulative_sum += found_value;
        }

        let mut partially_initialize_buf: Vec<MaybeUninit<T>> = (0..cumulative_sum)
            .into_iter()
            .map(|_| MaybeUninit::uninit())
            .collect();

        for (to_idx, data) in iter {
            let found_idx = &mut start_ends[to_idx + 1];

            partially_initialize_buf[*found_idx].write(data);
            *found_idx += 1;
        }

        /*
            SAFETY:
            Unless the user passes a ridiculous Iterator, where it's Clone-d version behaves differently,
            both passes should yield the exact same sequence of elements. In that case, we've properly
            reserved space in the buf vector for all of the elements, and thus every element got written to once.
            Vec<MaybeUninit<T>> is also compatible to transmute to Vec<T>
            (Caveat, nothing on DuckDuckGo I could find said anything about this)
        */
        let buf =
            unsafe { std::mem::transmute::<Vec<MaybeUninit<T>>, Vec<T>>(partially_initialize_buf) };

        Self { buf, start_ends }
    }
}

impl<T, ProducedIterators: IntoIterator<Item = T>> FromIterator<ProducedIterators>
    for ListOfLists<T>
{
    fn from_iter<IterT: IntoIterator<Item = ProducedIterators>>(iter: IterT) -> Self {
        let iter = iter.into_iter();
        let (lower, upper) = iter.size_hint();
        let estimated_size = upper.unwrap_or(lower);
        let mut result = ListOfLists::new_with_groups_capacity(estimated_size);
        for v in iter {
            result.new_group();
            for vv in v.into_iter() {
                result.push_to_last_group(vv)
            }
        }
        result
    }
}

impl<T: Clone> ListOfLists<T> {
    pub fn from_slice_slice(slice_slice: &[&[T]]) -> Self {
        slice_slice
            .iter()
            .map(|sub_slice| sub_slice.into_iter().cloned())
            .collect()
    }
}

impl<'a, T> Index<usize> for ListOfLists<T> {
    type Output = [T];

    fn index(&self, index: usize) -> &[T] {
        assert!(index < self.len());
        &self.buf[self.start_ends[index]..self.start_ends[index + 1]]
    }
}

impl<'a, T> IndexMut<usize> for ListOfLists<T> {
    fn index_mut(&mut self, index: usize) -> &mut [T] {
        assert!(index < self.len());
        &mut self.buf[self.start_ends[index]..self.start_ends[index + 1]]
    }
}

#[derive(Debug, Clone)]
pub struct ListOfListsFlatOriginIter<'a, T> {
    buf_iter: std::iter::Enumerate<std::slice::Iter<'a, T>>,
    ends: &'a [usize],
    cur_slice_idx: usize,
}

impl<'a, T> Iterator for ListOfListsFlatOriginIter<'a, T> {
    type Item = (usize, &'a T);

    fn next(&mut self) -> Option<(usize, &'a T)> {
        let Some((idx, item)) = self.buf_iter.next() else {
            return None;
        };

        // Skip through blocks of 0 size
        while idx == self.ends[self.cur_slice_idx] {
            self.cur_slice_idx += 1;
        }
        Some((self.cur_slice_idx, item))
    }
}

#[derive(Debug)]
pub struct ListOfListsFlatOriginIterMut<'a, T> {
    buf_iter: std::iter::Enumerate<std::slice::IterMut<'a, T>>,
    ends: &'a [usize],
    cur_slice_idx: usize,
}

impl<'a, T> Iterator for ListOfListsFlatOriginIterMut<'a, T> {
    type Item = (usize, &'a mut T);

    fn next(&mut self) -> Option<(usize, &'a mut T)> {
        let Some((idx, item)) = self.buf_iter.next() else {
            return None;
        };

        // Skip through blocks of 0 size
        while idx == self.ends[self.cur_slice_idx] {
            self.cur_slice_idx += 1;
        }
        Some((self.cur_slice_idx, item))
    }
}

// Basic iterators

#[derive(Debug, Clone)]
pub struct ListOfListsIter<'a, T> {
    buf: &'a [T],
    start: usize,
    ends_iter: std::slice::Iter<'a, usize>,
}

impl<'a, T> Iterator for ListOfListsIter<'a, T> {
    type Item = &'a [T];

    fn next(&mut self) -> Option<Self::Item> {
        let next_end = *self.ends_iter.next()?;
        let result = &self.buf[self.start..next_end];
        self.start = next_end;
        Some(result)
    }
}

#[derive(Debug)]
pub struct ListOfListsIterMut<'a, T> {
    buf: &'a mut [T],
    start: usize,
    ends_iter: std::slice::Iter<'a, usize>,
}

impl<'a, T> Iterator for ListOfListsIterMut<'a, T> {
    type Item = &'a mut [T];

    fn next(&mut self) -> Option<Self::Item> {
        let next_end = *self.ends_iter.next()?;
        let result: *mut [T] = &mut self.buf[self.start..next_end];
        self.start = next_end;
        // SAFETY: Slices produced by this iterator don't overlap.
        // Therefore we're allowed to cast away the self lifetime that attached itself to our mutable borrow
        Some(unsafe { &mut *result })
    }
}

impl<'a, T> IntoIterator for &'a ListOfLists<T> {
    type Item = &'a [T];

    type IntoIter = ListOfListsIter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        ListOfListsIter {
            buf: &self.buf,
            start: 0,
            ends_iter: self.start_ends[1..].into_iter(),
        }
    }
}

impl<'a, T> IntoIterator for &'a mut ListOfLists<T> {
    type Item = &'a mut [T];

    type IntoIter = ListOfListsIterMut<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        ListOfListsIterMut {
            buf: &mut self.buf,
            start: 0,
            ends_iter: self.start_ends[1..].into_iter(),
        }
    }
}
