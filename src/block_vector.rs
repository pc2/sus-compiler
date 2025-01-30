use std::{
    cell::{Cell, UnsafeCell},
    mem::MaybeUninit,
    ops::{Deref, DerefMut, Index, IndexMut},
};

/// Has the property that appends don't move other elements. References are always preserved, therefore append is const
///
/// Critically, alloc takes a CONST self, because using this will not invalidate any references derived from this
/// However, IndexMut still requires a mutable reference, since we can edit any arbitrary element, and the compiler can't check for overlap there
///
/// The const iterator exists, though it is not recommended to append elements while iterating over it. The const iterator would continue even onto newer elements
/// Existence of the mutable iterator disallows updating the container of course
#[derive(Default)]
pub struct BlockVec<T, const BLOCK_SIZE: usize = 64> {
    blocks: UnsafeCell<Vec<Box<[MaybeUninit<T>; BLOCK_SIZE]>>>,
    length: Cell<usize>,
}

impl<T, const BLOCK_SIZE: usize> BlockVec<T, BLOCK_SIZE> {
    pub fn new() -> Self {
        Self {
            blocks: UnsafeCell::new(Vec::new()),
            length: Cell::new(0),
        }
    }

    pub fn alloc(&self, obj: T) -> usize {
        let b = self.blocks.get();

        let allocated_id = self.length.get();
        if allocated_id % BLOCK_SIZE == 0 {
            // New block

            let new_block: Box<MaybeUninit<[MaybeUninit<T>; BLOCK_SIZE]>> =
                Box::new(MaybeUninit::uninit());
            unsafe {
                let mut new_block_box = std::mem::transmute::<
                    Box<MaybeUninit<[MaybeUninit<T>; BLOCK_SIZE]>>,
                    Box<[MaybeUninit<T>; BLOCK_SIZE]>,
                >(new_block);
                let slice = new_block_box.as_mut();
                slice[0].write(obj);

                //slice[0].write(obj);
                assert!((*b).len() == (allocated_id / BLOCK_SIZE));
                (*b).push(new_block_box);
            }
        } else {
            unsafe {
                let last_block = (*b).last_mut().unwrap();
                last_block[allocated_id % BLOCK_SIZE].write(obj);
            }
        }
        self.length.set(allocated_id + 1);

        allocated_id
    }

    pub fn len(&self) -> usize {
        self.length.get()
    }

    pub fn is_empty(&self) -> bool {
        self.length.get() == 0
    }

    /// Critically, since appending to [BlockVec] is non-mutable, it is possible to do so while holding a [BlockVecIter].
    /// BlockVecIter only iterates up to the size the BlockVec had when [BlockVec::iter] was called
    pub fn iter(&self) -> BlockVecIter<'_, T, BLOCK_SIZE> {
        self.into_iter()
    }

    pub fn iter_mut(&mut self) -> BlockVecIterMut<'_, T, BLOCK_SIZE> {
        self.into_iter()
    }
}

impl<T, const BLOCK_SIZE: usize> Drop for BlockVec<T, BLOCK_SIZE> {
    fn drop(&mut self) {
        let num_full_blocks = self.length.get() / BLOCK_SIZE;
        let num_remaining = self.length.get() % BLOCK_SIZE;

        let block_vec = self.blocks.get_mut();
        for i in 0..num_full_blocks {
            for v in block_vec[i].deref_mut() {
                unsafe {
                    v.assume_init_drop();
                }
            }
        }

        if num_remaining > 0 {
            let last_block = block_vec[num_full_blocks].deref_mut();
            for i in 0..num_remaining {
                unsafe { last_block[i].assume_init_drop() };
            }
        }
    }
}

impl<T: std::fmt::Debug, const BLOCK_SIZE: usize> std::fmt::Debug for BlockVec<T, BLOCK_SIZE> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let len = self.length.get();
        f.write_fmt(format_args!("BlockVec(Size = {len})["))?;
        for item in self {
            item.fmt(f)?;
            f.write_str(", ")?;
        }
        f.write_str("]")
    }
}

impl<T, const BLOCK_SIZE: usize> Index<usize> for BlockVec<T, BLOCK_SIZE> {
    type Output = T;

    fn index(&self, index: usize) -> &T {
        let self_len = self.length.get();
        if index >= self_len {
            panic!("Index is out of bounds (idx is {index}, len is {self_len})")
        }

        let block = index / BLOCK_SIZE;
        let idx_in_block = index % BLOCK_SIZE;

        unsafe {
            let vec = self.blocks.get();
            (*vec)[block].deref()[idx_in_block].assume_init_ref()
        }
    }
}
impl<T, const BLOCK_SIZE: usize> IndexMut<usize> for BlockVec<T, BLOCK_SIZE> {
    fn index_mut(&mut self, index: usize) -> &mut T {
        let self_len = self.length.get();
        if index >= self_len {
            panic!("Index is out of bounds (idx is {index}, len is {self_len})")
        }

        let block = index / BLOCK_SIZE;
        let idx_in_block = index % BLOCK_SIZE;

        let vec = self.blocks.get_mut();
        unsafe { vec[block].deref_mut()[idx_in_block].assume_init_mut() }
    }
}

pub struct BlockVecIter<'bv, T, const BLOCK_SIZE: usize = 64> {
    block_vec: &'bv BlockVec<T, BLOCK_SIZE>,
    cur_idx: usize,
}

impl<'bv, T, const BLOCK_SIZE: usize> Iterator for BlockVecIter<'bv, T, BLOCK_SIZE> {
    type Item = &'bv T;

    fn next(&mut self) -> Option<&'bv T> {
        if self.cur_idx < self.block_vec.length.get() {
            let selected_idx = self.cur_idx;
            self.cur_idx += 1;

            Some(self.block_vec.index(selected_idx))
        } else {
            None
        }
    }
}

impl<'bv, T, const BLOCK_SIZE: usize> IntoIterator for &'bv BlockVec<T, BLOCK_SIZE> {
    type Item = &'bv T;

    type IntoIter = BlockVecIter<'bv, T, BLOCK_SIZE>;

    fn into_iter(self) -> Self::IntoIter {
        BlockVecIter {
            block_vec: self,
            cur_idx: 0,
        }
    }
}

pub struct BlockVecIterMut<'bv, T, const BLOCK_SIZE: usize = 64> {
    block_vec: &'bv mut BlockVec<T, BLOCK_SIZE>,
    cur_idx: usize,
}

impl<'bv, T, const BLOCK_SIZE: usize> Iterator for BlockVecIterMut<'bv, T, BLOCK_SIZE> {
    type Item = &'bv mut T;

    fn next(&mut self) -> Option<&'bv mut T> {
        if self.cur_idx < self.block_vec.length.get() {
            let selected_idx = self.cur_idx;
            self.cur_idx += 1;

            // SAFETY
            // Have to cast away the added 'self lifetime. The borrow checker adds it on self.block_vec to prevent us from mutably referencing the same location twice.
            // This code always indexes unique elements, and thus we can safely cast away this lifetime.
            let original_ref: *mut T = self.block_vec.index_mut(selected_idx);
            Some(unsafe { &mut *original_ref })
        } else {
            None
        }
    }
}

impl<'bv, T, const BLOCK_SIZE: usize> IntoIterator for &'bv mut BlockVec<T, BLOCK_SIZE> {
    type Item = &'bv mut T;

    type IntoIter = BlockVecIterMut<'bv, T, BLOCK_SIZE>;

    fn into_iter(self) -> Self::IntoIter {
        BlockVecIterMut {
            block_vec: self,
            cur_idx: 0,
        }
    }
}

pub struct BlockVecConsumingIter<T, const BLOCK_SIZE: usize = 64> {
    block_vec_iter: <Vec<Box<[MaybeUninit<T>; BLOCK_SIZE]>> as IntoIterator>::IntoIter,
    current_block: Option<Box<[MaybeUninit<T>; BLOCK_SIZE]>>,
    current_idx: usize,
    total_vec_size: usize,
}

impl<T, const BLOCK_SIZE: usize> Iterator for BlockVecConsumingIter<T, BLOCK_SIZE> {
    type Item = T;

    fn next(&mut self) -> Option<T> {
        if self.current_idx < self.total_vec_size {
            let idx_in_block = self.current_idx % BLOCK_SIZE;
            self.current_idx += 1;

            if idx_in_block == 0 {
                self.current_block = Some(self.block_vec_iter.next().unwrap()); // Vec will always be big enough
            }

            let found = &mut self.current_block.as_mut().unwrap().as_mut_slice()[idx_in_block];
            unsafe { Some(found.assume_init_read()) }
        } else {
            None
        }
    }
}

impl<T, const BLOCK_SIZE: usize> IntoIterator for BlockVec<T, BLOCK_SIZE> {
    type Item = T;

    type IntoIter = BlockVecConsumingIter<T, BLOCK_SIZE>;

    fn into_iter(mut self) -> Self::IntoIter {
        let total_vec_size = self.length.get();
        self.length.set(0);
        let block_vec = std::mem::take(self.blocks.get_mut());
        let block_vec_iter = block_vec.into_iter();
        BlockVecConsumingIter {
            block_vec_iter,
            current_block: None,
            current_idx: 0,
            total_vec_size,
        }
    }
}

impl<T, const BLOCK_SIZE: usize> Drop for BlockVecConsumingIter<T, BLOCK_SIZE> {
    fn drop(&mut self) {
        for _ in self.by_ref() {} // Automatically drops all remaining elements of the iterator
    }
}

impl<'bv, T, const BLOCK_SIZE: usize> FromIterator<T> for BlockVec<T, BLOCK_SIZE> {
    fn from_iter<Iter: IntoIterator<Item = T>>(iter: Iter) -> Self {
        let new_coll = BlockVec::new();
        for v in iter {
            new_coll.alloc(v);
        }
        new_coll
    }
}
