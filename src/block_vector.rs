use std::{
    cell::{RefCell, RefMut},
    cmp::Ordering,
    collections::VecDeque,
    fmt::{Debug, Formatter},
    hash::Hash,
    mem::MaybeUninit,
    ops::{Index, IndexMut, Range},
    write,
};

struct BlockVecDequeInner<T, const BLOCK_SIZE: usize = 64> {
    blocks: VecDeque<*mut [MaybeUninit<T>; BLOCK_SIZE]>,
    range: Range<i64>,
}

fn make_block<T, const BLOCK_SIZE: usize>() -> *mut [MaybeUninit<T>; BLOCK_SIZE] {
    Box::into_raw(Box::new([const { MaybeUninit::<T>::uninit() }; BLOCK_SIZE]))
}

fn free_block<T, const BLOCK_SIZE: usize>(block: *mut [MaybeUninit<T>; BLOCK_SIZE]) {
    unsafe { std::mem::drop(Box::from_raw(block)) }
}

impl<T, const BLOCK_SIZE: usize> BlockVecDequeInner<T, BLOCK_SIZE> {
    fn new() -> Self {
        Self {
            blocks: VecDeque::new(),
            range: 0..0,
        }
    }
    fn with_capacity(capacity: usize) -> Self {
        Self {
            blocks: VecDeque::with_capacity(capacity.div_ceil(BLOCK_SIZE) + 1),
            range: 0..0,
        }
    }
    fn len(&self) -> usize {
        (self.range.end - self.range.start) as usize
    }
    fn is_empty(&self) -> bool {
        self.range.is_empty()
    }
    fn push_front(&mut self, v: T) {
        self.range.start -= 1;
        let index_in_first_block = self.range.start.rem_euclid(BLOCK_SIZE as i64) as usize;
        if index_in_first_block == BLOCK_SIZE - 1 {
            self.blocks.push_front(make_block());
        }
        let first_block = *self.blocks.front().unwrap();
        unsafe {
            (*first_block)[index_in_first_block].write(v);
        }
    }
    fn push_back(&mut self, v: T) {
        let index_in_last_block = self.range.end.rem_euclid(BLOCK_SIZE as i64) as usize;
        if index_in_last_block == 0 {
            self.blocks.push_back(make_block());
        }
        let last_block = *self.blocks.back().unwrap();
        unsafe {
            (*last_block)[index_in_last_block].write(v);
        }
        self.range.end += 1;
    }
    /// It is an error for `make_default` to access this vector. Should be caught by the [RefCell]. It should also not panic
    fn get_mut(&mut self, idx: i64, mut make_default: impl FnMut() -> T) -> *mut T {
        while idx < self.range.start {
            self.push_front(make_default());
        }
        while idx >= self.range.end {
            self.push_back(make_default());
        }

        let first_block = self.range.start.div_euclid(BLOCK_SIZE as i64);
        let idx_block = idx.div_euclid(BLOCK_SIZE as i64);
        let idx_block_offset = idx.rem_euclid(BLOCK_SIZE as i64) as usize;
        let block = self.blocks[(idx_block - first_block) as usize];
        unsafe {
            block
                .cast::<MaybeUninit<T>>()
                .add(idx_block_offset)
                .cast::<T>()
        }
    }
    fn get(&mut self, idx: i64, make_default: impl FnMut() -> T) -> *const T {
        self.get_mut(idx, make_default)
    }
    fn get_existing_mut(&mut self, idx: i64) -> *mut T {
        self.get_existing(idx) as *mut T
    }
    fn get_existing(&self, idx: i64) -> *const T {
        assert!(self.range.contains(&idx));

        let first_block = self.range.start.div_euclid(BLOCK_SIZE as i64);
        let block_idx = idx.div_euclid(BLOCK_SIZE as i64);
        let idx_in_block = idx.rem_euclid(BLOCK_SIZE as i64) as usize;
        let block = self.blocks[(block_idx - first_block) as usize];
        unsafe { block.cast::<MaybeUninit<T>>().add(idx_in_block).cast::<T>() }
    }
}

impl<T, const BLOCK_SIZE: usize> Drop for BlockVecDequeInner<T, BLOCK_SIZE> {
    fn drop(&mut self) {
        if self.range.is_empty() {
            assert!(self.blocks.is_empty());
            return;
        }
        let last_elem = self.range.end - 1;

        let first_elem_offset = self.range.start.rem_euclid(BLOCK_SIZE as i64) as usize;
        let last_elem_offset = last_elem.rem_euclid(BLOCK_SIZE as i64) as usize;

        unsafe {
            let first_block = &mut *self.blocks.pop_front().unwrap();
            if let Some(last_block) = self.blocks.pop_back() {
                let last_block = &mut *last_block;
                // first block and last block are separate
                for v in &mut first_block[first_elem_offset..BLOCK_SIZE] {
                    v.assume_init_drop();
                }
                for v in &mut last_block[0..=last_elem_offset] {
                    v.assume_init_drop();
                }
                free_block(first_block);
                free_block(last_block);
            } else {
                // first block and last block are the same block
                for v in &mut first_block[first_elem_offset..=last_elem_offset] {
                    v.assume_init_drop();
                }
                free_block(first_block);
            }

            for middle_block in self.blocks.drain(..) {
                for v in &mut *middle_block {
                    v.assume_init_drop();
                }
                free_block(middle_block);
            }
        }
    }
}

/*
    Has the property that appends don't move other elements. References are always preserved, therefore append is const

    Critically, alloc takes a CONST self, because using this will not invalidate any references derived from this
    However, IndexMut still requires a mutable reference, since we can edit any arbitrary element, and the compiler can't check for overlap there

    The const iterator exists, though it is not recommended to append elements while iterating over it. The const iterator would continue even onto newer elements
    Existence of the mutable iterator disallows updating the container of course
*/
pub struct BlockVecDeque<T, const BLOCK_SIZE: usize = 64>(
    RefCell<BlockVecDequeInner<T, BLOCK_SIZE>>,
);

impl<T, const BLOCK_SIZE: usize> Default for BlockVecDeque<T, BLOCK_SIZE> {
    fn default() -> Self {
        Self::new()
    }
}
impl<T: Clone, const BLOCK_SIZE: usize> Clone for BlockVecDequeInner<T, BLOCK_SIZE> {
    fn clone(&self) -> Self {
        let blocks: VecDeque<_> = (0..self.blocks.len()).map(|_| make_block()).collect();

        let first_block = self.range.start.div_euclid(BLOCK_SIZE as i64);

        for i in self.range.clone() {
            let block_idx = i.div_euclid(BLOCK_SIZE as i64);
            let idx_in_block = i.rem_euclid(BLOCK_SIZE as i64) as usize;

            let from_block = self.blocks[(block_idx - first_block) as usize];
            let to_block = blocks[(block_idx - first_block) as usize];

            unsafe {
                let from_ptr = from_block.cast::<MaybeUninit<T>>().add(idx_in_block);
                let to_ptr = to_block.cast::<MaybeUninit<T>>().add(idx_in_block);

                (*to_ptr).write((*from_ptr).assume_init_ref().clone());
            }
        }

        BlockVecDequeInner {
            blocks,
            range: self.range.clone(),
        }
    }
}
impl<T: Clone, const BLOCK_SIZE: usize> Clone for BlockVecDeque<T, BLOCK_SIZE> {
    fn clone(&self) -> Self {
        let inner_borrow = self.0.borrow();
        Self(RefCell::new(inner_borrow.clone()))
    }
}
impl<T: Hash, const BLOCK_SIZE: usize> Hash for BlockVecDeque<T, BLOCK_SIZE> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        for e in self {
            e.hash(state);
        }
    }
}
impl<T: PartialEq, const BLOCK_SIZE: usize> PartialEq for BlockVecDeque<T, BLOCK_SIZE> {
    fn eq(&self, other: &Self) -> bool {
        if self.len() != other.len() {
            return false;
        }
        self.iter().zip(other.iter()).all(|(a, b)| a.eq(b))
    }
}
impl<T: Eq, const BLOCK_SIZE: usize> Eq for BlockVecDeque<T, BLOCK_SIZE> {}

impl<T: PartialOrd, const BLOCK_SIZE: usize> PartialOrd for BlockVecDeque<T, BLOCK_SIZE> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let found_order = self.len().cmp(&other.len());
        if found_order != Ordering::Equal {
            return Some(found_order);
        }
        for (a, b) in self.iter().zip(other.iter()) {
            match a.partial_cmp(b) {
                Some(Ordering::Less) => {
                    return Some(Ordering::Less);
                }
                Some(Ordering::Greater) => {
                    return Some(Ordering::Greater);
                }
                Some(Ordering::Equal) => {}
                None => return None,
            }
        }
        Some(Ordering::Equal)
    }
}
impl<T: Ord, const BLOCK_SIZE: usize> Ord for BlockVecDeque<T, BLOCK_SIZE> {
    fn cmp(&self, other: &Self) -> Ordering {
        let found_order = self.len().cmp(&other.len());
        if found_order != Ordering::Equal {
            return found_order;
        }
        for (a, b) in self.iter().zip(other.iter()) {
            match a.cmp(b) {
                Ordering::Less => {
                    return Ordering::Less;
                }
                Ordering::Greater => {
                    return Ordering::Greater;
                }
                Ordering::Equal => {}
            }
        }
        Ordering::Equal
    }
}

impl<T, const BLOCK_SIZE: usize> BlockVecDeque<T, BLOCK_SIZE> {
    pub fn new() -> Self {
        Self(RefCell::new(BlockVecDequeInner::new()))
    }

    pub fn new_with_range(range: Range<i64>, mut make_default: impl FnMut() -> T) -> Self {
        assert!(range.end >= range.start);
        if range.is_empty() {
            return Self::new();
        }

        let mut inner: BlockVecDequeInner<T, BLOCK_SIZE> = BlockVecDequeInner::new();

        // First initialize the blockvec to the correct size
        let _ = inner.get_mut(range.start, &mut make_default);
        let _ = inner.get_mut(range.end - 1, make_default);

        Self(RefCell::new(inner))
    }

    fn with_capacity(capacity: usize) -> Self {
        Self(RefCell::new(BlockVecDequeInner::with_capacity(capacity)))
    }

    pub fn get_or_insert(&self, idx: i64, make_default: impl FnMut() -> T) -> &T {
        unsafe { &*self.0.borrow_mut().get(idx, make_default) }
    }
    pub fn get_or_insert_mut(&mut self, idx: i64, make_default: impl FnMut() -> T) -> &mut T {
        unsafe { &mut *self.0.borrow_mut().get_mut(idx, make_default) }
    }

    pub fn get(&self, idx: i64) -> Option<&T> {
        if self.range().contains(&idx) {
            unsafe { Some(&*self.0.borrow().get_existing(idx)) }
        } else {
            None
        }
    }
    pub fn get_mut(&mut self, idx: i64) -> Option<&mut T> {
        if self.range().contains(&idx) {
            unsafe { Some(&mut *self.0.borrow_mut().get_existing_mut(idx)) }
        } else {
            None
        }
    }
    pub fn push_back(&mut self, v: T) {
        self.0.borrow_mut().push_back(v);
    }
    pub fn push_front(&mut self, v: T) {
        self.0.borrow_mut().push_front(v);
    }

    pub fn len(&self) -> usize {
        self.0.borrow().len()
    }
    pub fn is_empty(&self) -> bool {
        self.0.borrow().is_empty()
    }
    pub fn range(&self) -> Range<i64> {
        self.0.borrow().range.clone()
    }

    pub fn iter<'s>(&'s self) -> BlockVecIter<'s, T, BLOCK_SIZE> {
        self.into_iter()
    }

    pub fn iter_mut<'s>(&'s mut self) -> BlockVecIterMut<'s, T, BLOCK_SIZE> {
        self.into_iter()
    }
}

impl<T: Debug, const BLOCK_SIZE: usize> Debug for BlockVecDeque<T, BLOCK_SIZE> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let range = self.range();
        writeln!(f, "BlockVec({range:?})[")?;
        for idx in range.clone() {
            write!(f, "[{idx}] = {:?}", self.index(idx))?;
            if idx != range.end - 1 {
                write!(f, ",")?;
            }
            writeln!(f)?;
        }
        writeln!(f, "]")
    }
}

impl<T, const BLOCK_SIZE: usize> Index<i64> for BlockVecDeque<T, BLOCK_SIZE> {
    type Output = T;

    fn index(&self, index: i64) -> &T {
        let inner = self.0.borrow();
        unsafe { &*inner.get_existing(index) }
    }
}
impl<T, const BLOCK_SIZE: usize> IndexMut<i64> for BlockVecDeque<T, BLOCK_SIZE> {
    fn index_mut(&mut self, index: i64) -> &mut T {
        let mut inner = self.0.borrow_mut();
        unsafe { &mut *inner.get_existing_mut(index) }
    }
}

#[derive(Clone)]
pub struct BlockVecIter<'bv, T, const BLOCK_SIZE: usize = 64> {
    block_vec: &'bv BlockVecDeque<T, BLOCK_SIZE>,
    range: Range<i64>,
}

impl<'bv, T, const BLOCK_SIZE: usize> Iterator for BlockVecIter<'bv, T, BLOCK_SIZE> {
    type Item = &'bv T;

    fn next(&mut self) -> Option<&'bv T> {
        if !self.range.is_empty() {
            let selected_idx = self.range.start;
            self.range.start += 1;

            let result = &self.block_vec[selected_idx];
            Some(result)
        } else {
            None
        }
    }
}

impl<'bv, T, const BLOCK_SIZE: usize> DoubleEndedIterator for BlockVecIter<'bv, T, BLOCK_SIZE> {
    fn next_back(&mut self) -> Option<Self::Item> {
        if !self.range.is_empty() {
            self.range.end -= 1;
            let selected_idx = self.range.end;

            let result = &self.block_vec[selected_idx];
            Some(result)
        } else {
            None
        }
    }
}

impl<'bv, T, const BLOCK_SIZE: usize> IntoIterator for &'bv BlockVecDeque<T, BLOCK_SIZE> {
    type Item = &'bv T;

    type IntoIter = BlockVecIter<'bv, T, BLOCK_SIZE>;

    fn into_iter(self) -> Self::IntoIter {
        let range = self.range();
        BlockVecIter {
            block_vec: self,
            range,
        }
    }
}

pub struct BlockVecIterMut<'bv, T, const BLOCK_SIZE: usize = 64> {
    block_vec: RefMut<'bv, BlockVecDequeInner<T, BLOCK_SIZE>>,
    range: Range<i64>,
}

impl<'bv, T, const BLOCK_SIZE: usize> Iterator for BlockVecIterMut<'bv, T, BLOCK_SIZE> {
    type Item = &'bv mut T;

    fn next(&mut self) -> Option<&'bv mut T> {
        if !self.range.is_empty() {
            let selected_idx = self.range.start;
            self.range.start += 1;

            unsafe {
                let result = &mut *self.block_vec.get_existing_mut(selected_idx);
                Some(result)
            }
        } else {
            None
        }
    }
}
impl<'bv, T, const BLOCK_SIZE: usize> DoubleEndedIterator for BlockVecIterMut<'bv, T, BLOCK_SIZE> {
    fn next_back(&mut self) -> Option<Self::Item> {
        if !self.range.is_empty() {
            self.range.end -= 1;
            let selected_idx = self.range.end;

            unsafe {
                let result = &mut *self.block_vec.get_existing_mut(selected_idx);
                Some(result)
            }
        } else {
            None
        }
    }
}

impl<'bv, T, const BLOCK_SIZE: usize> IntoIterator for &'bv mut BlockVecDeque<T, BLOCK_SIZE> {
    type Item = &'bv mut T;

    type IntoIter = BlockVecIterMut<'bv, T, BLOCK_SIZE>;

    fn into_iter(self) -> Self::IntoIter {
        let block_vec = self.0.borrow_mut();
        let range = block_vec.range.clone();
        BlockVecIterMut { block_vec, range }
    }
}

#[derive(Clone)]
pub struct BlockVecConsumingIter<T, const BLOCK_SIZE: usize = 64> {
    inner: BlockVecDequeInner<T, BLOCK_SIZE>,
}

impl<T, const BLOCK_SIZE: usize> Iterator for BlockVecConsumingIter<T, BLOCK_SIZE> {
    type Item = T;

    fn next(&mut self) -> Option<T> {
        if self.inner.range.is_empty() {
            None
        } else {
            let selected_idx = self.inner.range.start;
            self.inner.range.start += 1;
            let front_block = self.inner.blocks.front().unwrap();

            let index_in_first_block = selected_idx.div_euclid(BLOCK_SIZE as i64) as usize;
            let result = unsafe {
                let elem_ptr = front_block
                    .cast::<MaybeUninit<T>>()
                    .add(index_in_first_block);
                (*elem_ptr).assume_init_read()
            };

            if selected_idx.rem_euclid(BLOCK_SIZE as i64) == BLOCK_SIZE as i64 - 1 {
                free_block(self.inner.blocks.pop_front().unwrap());
            }
            Some(result)
        }
    }
}
impl<T, const BLOCK_SIZE: usize> DoubleEndedIterator for BlockVecConsumingIter<T, BLOCK_SIZE> {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.inner.range.is_empty() {
            None
        } else {
            self.inner.range.end -= 1;
            let selected_idx = self.inner.range.end;
            let front_block = self.inner.blocks.front().unwrap();

            let index_in_first_block = selected_idx.div_euclid(BLOCK_SIZE as i64) as usize;
            let result = unsafe {
                let elem_ptr = front_block
                    .cast::<MaybeUninit<T>>()
                    .add(index_in_first_block);
                (*elem_ptr).assume_init_read()
            };

            if selected_idx.rem_euclid(BLOCK_SIZE as i64) == BLOCK_SIZE as i64 - 1 {
                free_block(self.inner.blocks.pop_front().unwrap());
            }
            Some(result)
        }
    }
}

impl<T, const BLOCK_SIZE: usize> IntoIterator for BlockVecDeque<T, BLOCK_SIZE> {
    type Item = T;

    type IntoIter = BlockVecConsumingIter<T, BLOCK_SIZE>;

    fn into_iter(self) -> Self::IntoIter {
        BlockVecConsumingIter {
            inner: self.0.into_inner(),
        }
    }
}

impl<T, const BLOCK_SIZE: usize> FromIterator<T> for BlockVecDeque<T, BLOCK_SIZE> {
    fn from_iter<Iter: IntoIterator<Item = T>>(iter: Iter) -> Self {
        let iterator = iter.into_iter();

        let mut result = Self::with_capacity(iterator.size_hint().0);

        for (idx, v) in iterator.enumerate() {
            result[idx as i64] = v;
        }

        result
    }
}
