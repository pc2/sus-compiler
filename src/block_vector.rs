use std::{
    cell::{Ref, RefCell, RefMut},
    collections::VecDeque,
    fmt::{Debug, Formatter},
    mem::MaybeUninit,
    ops::{Index, IndexMut, Range},
    write,
};

struct BlockVecDequeInner<T, const BLOCK_SIZE: usize = 64> {
    blocks: VecDeque<*mut [MaybeUninit<T>; BLOCK_SIZE]>,
    length: i64,
    first: i64,
}

fn make_block<T, const BLOCK_SIZE: usize>() -> *mut [MaybeUninit<T>; BLOCK_SIZE] {
    Box::into_raw(Box::new([const { MaybeUninit::<T>::uninit() }; BLOCK_SIZE]))
}

unsafe fn free_block<T, const BLOCK_SIZE: usize>(block: *mut [MaybeUninit<T>; BLOCK_SIZE]) {
    unsafe { std::mem::drop(Box::from_raw(block)) }
}

impl<T, const BLOCK_SIZE: usize> BlockVecDequeInner<T, BLOCK_SIZE> {
    fn new() -> Self {
        Self {
            blocks: VecDeque::new(),
            length: 0,
            first: 0,
        }
    }
    pub fn range(&self) -> Range<i64> {
        self.first..(self.first + self.length)
    }
    /// It is an error for `make_default` to access this vector. Should be caught by the [RefCell]. It should also not panic
    fn get_mut(&mut self, idx: i64, mut make_default: impl FnMut() -> T) -> *mut T {
        let idx_block_index = idx.div_euclid(BLOCK_SIZE as i64);
        let idx_block_offset = idx.rem_euclid(BLOCK_SIZE as i64) as usize;
        let block: *mut [MaybeUninit<T>; BLOCK_SIZE] = if self.length == 0 {
            self.length = 1;
            self.first = idx;
            self.blocks.push_back(make_block());
            self.blocks[0]
        } else {
            let last_idx = self.first + self.length - 1;

            let first_stored_block = self.first.div_euclid(BLOCK_SIZE as i64);
            let last_stored_block = last_idx.div_euclid(BLOCK_SIZE as i64);
            let new_data_range = if idx < self.first {
                let num_extra_blocks = (first_stored_block - idx_block_index) as usize;
                self.blocks.reserve(num_extra_blocks);

                for _ in 0..num_extra_blocks {
                    self.blocks.push_front(make_block());
                }

                Some((idx..self.first, idx, last_idx - 1 - self.first))
            } else if idx > last_idx {
                let num_extra_blocks = (idx_block_index - last_stored_block) as usize;
                self.blocks.reserve(num_extra_blocks);

                for _ in 0..num_extra_blocks {
                    self.blocks.push_back(make_block());
                }

                Some((last_idx..idx, self.first, idx + 1 - self.first))
            } else {
                None
            };

            if let Some((new_data_range, new_first, new_length)) = new_data_range {
                self.first = new_first;
                self.length = new_length;

                let first_block = self.first.div_euclid(BLOCK_SIZE as i64);
                for v in new_data_range {
                    let v_block_idx = v.div_euclid(BLOCK_SIZE as i64);
                    let v_idx_in_block = v.rem_euclid(BLOCK_SIZE as i64) as usize;

                    let block = self.blocks[(v_block_idx - first_block) as usize];
                    unsafe {
                        let elem_ptr = block.cast::<MaybeUninit<T>>().add(v_idx_in_block);
                        (*elem_ptr).write(make_default());
                    }
                }
            }

            self.blocks[(idx_block_index - first_stored_block) as usize]
        };

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
        let valid_range = self.range();
        assert!(valid_range.contains(&idx));

        let first_block = self.first.div_euclid(BLOCK_SIZE as i64);
        let block_idx = idx.div_euclid(BLOCK_SIZE as i64);
        let idx_in_block = idx.rem_euclid(BLOCK_SIZE as i64) as usize;
        let block = self.blocks[(block_idx - first_block) as usize];
        unsafe { block.cast::<MaybeUninit<T>>().add(idx_in_block).cast::<T>() }
    }
}

impl<T, const BLOCK_SIZE: usize> Drop for BlockVecDequeInner<T, BLOCK_SIZE> {
    fn drop(&mut self) {
        if self.length == 0 {
            assert!(self.blocks.is_empty());
            return;
        }
        let last_elem = self.first + (self.length - 1) as i64;

        let first_elem_offset = self.first.rem_euclid(BLOCK_SIZE as i64) as usize;
        let last_elem_offset = last_elem.rem_euclid(BLOCK_SIZE as i64) as usize;

        unsafe {
            let first_block = &mut *self.blocks.pop_front().unwrap();
            if let Some(last_block) = self.blocks.pop_back() {
                let last_block = &mut *last_block;
                // first block and last block are separate
                for v in first_elem_offset..BLOCK_SIZE {
                    first_block[v].assume_init_drop();
                }
                for v in 0..=last_elem_offset {
                    last_block[v].assume_init_drop();
                }
            } else {
                // first block and last block are the same block
                for v in first_elem_offset..=last_elem_offset {
                    first_block[v].assume_init_drop();
                }
            }

            for middle_block in self.blocks.drain(..) {
                let middle_block = &mut *middle_block;
                for v in 0..BLOCK_SIZE {
                    middle_block[v].assume_init_drop();
                }
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
impl<T, const BLOCK_SIZE: usize> BlockVecDeque<T, BLOCK_SIZE> {
    pub fn new() -> Self {
        Self(RefCell::new(BlockVecDequeInner::new()))
    }

    pub fn new_with_size(
        start_offset: i64,
        length: usize,
        mut make_default: impl FnMut() -> T,
    ) -> Self {
        let length = length as i64;
        if length == 0 {
            return Self::new();
        }

        let mut inner: BlockVecDequeInner<T, BLOCK_SIZE> = BlockVecDequeInner::new();

        // First initialize the blockvec to the correct size
        let _ = inner.get_mut(start_offset, &mut make_default);
        let _ = inner.get_mut(start_offset + length - 1, make_default);

        Self(RefCell::new(inner))
    }

    pub fn len(&self) -> usize {
        self.0.borrow().length as usize
    }
    pub fn range(&self) -> Range<i64> {
        self.0.borrow().range()
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

pub struct BlockVecIter<'bv, T, const BLOCK_SIZE: usize = 64> {
    block_vec: Ref<'bv, BlockVecDequeInner<T, BLOCK_SIZE>>,
    cur_idx: i64,
    end_idx: i64,
}

impl<'bv, T, const BLOCK_SIZE: usize> Iterator for BlockVecIter<'bv, T, BLOCK_SIZE> {
    type Item = (i64, &'bv T);

    fn next(&mut self) -> Option<(i64, &'bv T)> {
        if self.cur_idx < self.end_idx {
            let selected_idx = self.cur_idx;
            self.cur_idx += 1;

            unsafe {
                let result = &*self.block_vec.get_existing(selected_idx);
                Some((selected_idx, result))
            }
        } else {
            None
        }
    }
}

impl<'bv, T, const BLOCK_SIZE: usize> IntoIterator for &'bv BlockVecDeque<T, BLOCK_SIZE> {
    type Item = (i64, &'bv T);

    type IntoIter = BlockVecIter<'bv, T, BLOCK_SIZE>;

    fn into_iter(self) -> Self::IntoIter {
        let block_vec = self.0.borrow();
        let cur_idx = block_vec.first;
        let end_idx = block_vec.first + block_vec.length;
        BlockVecIter {
            block_vec,
            cur_idx,
            end_idx,
        }
    }
}

pub struct BlockVecIterMut<'bv, T, const BLOCK_SIZE: usize = 64> {
    block_vec: RefMut<'bv, BlockVecDequeInner<T, BLOCK_SIZE>>,
    cur_idx: i64,
    end_idx: i64,
}

impl<'bv, T, const BLOCK_SIZE: usize> Iterator for BlockVecIterMut<'bv, T, BLOCK_SIZE> {
    type Item = (i64, &'bv mut T);

    fn next(&mut self) -> Option<(i64, &'bv mut T)> {
        if self.cur_idx < self.end_idx {
            let selected_idx = self.cur_idx;
            self.cur_idx += 1;

            unsafe {
                let result = &mut *self.block_vec.get_existing_mut(selected_idx);
                Some((selected_idx, result))
            }
        } else {
            None
        }
    }
}

impl<'bv, T, const BLOCK_SIZE: usize> IntoIterator for &'bv mut BlockVecDeque<T, BLOCK_SIZE> {
    type Item = (i64, &'bv mut T);

    type IntoIter = BlockVecIterMut<'bv, T, BLOCK_SIZE>;

    fn into_iter(self) -> Self::IntoIter {
        let block_vec = self.0.borrow_mut();
        let cur_idx = block_vec.first;
        let end_idx = block_vec.first + block_vec.length;
        BlockVecIterMut {
            block_vec,
            cur_idx,
            end_idx,
        }
    }
}

pub struct BlockVecConsumingIter<T, const BLOCK_SIZE: usize = 64> {
    inner: BlockVecDequeInner<T, BLOCK_SIZE>,
}

impl<T, const BLOCK_SIZE: usize> Iterator for BlockVecConsumingIter<T, BLOCK_SIZE> {
    type Item = (i64, T);

    fn next(&mut self) -> Option<(i64, T)> {
        if self.inner.length == 0 {
            None
        } else {
            let selected_idx = self.inner.first;
            self.inner.first += 1;
            self.inner.length -= 1;
            let front_block = self.inner.blocks.front().unwrap();

            let index_in_first_block = selected_idx.div_euclid(BLOCK_SIZE as i64) as usize;
            let result = unsafe {
                let elem_ptr = front_block
                    .cast::<MaybeUninit<T>>()
                    .add(index_in_first_block);
                (*elem_ptr).assume_init_read()
            };

            if selected_idx.rem_euclid(BLOCK_SIZE as i64) == BLOCK_SIZE as i64 - 1 {
                self.inner.blocks.pop_front();
            }
            Some((selected_idx, result))
        }
    }
}

impl<T, const BLOCK_SIZE: usize> IntoIterator for BlockVecDeque<T, BLOCK_SIZE> {
    type Item = (i64, T);

    type IntoIter = BlockVecConsumingIter<T, BLOCK_SIZE>;

    fn into_iter(self) -> Self::IntoIter {
        BlockVecConsumingIter {
            inner: self.0.into_inner(),
        }
    }
}

impl<T, const BLOCK_SIZE: usize> Drop for BlockVecConsumingIter<T, BLOCK_SIZE> {
    fn drop(&mut self) {
        while let Some(_) = self.next() {} // Automatically drops all remaining elements of the iterator
    }
}
