use std::{cell::{UnsafeCell, Cell}, mem::MaybeUninit, ops::{DerefMut, Deref, IndexMut, Index}};



/* Has the property that appends don't move other elements. References are always preserved, therefore append is const */
#[derive(Debug,Default)]
pub struct BlockVec<T, const BLOCK_SIZE : usize = 64> {
    blocks : UnsafeCell<Vec<Box<[MaybeUninit<T>; BLOCK_SIZE]>>>,
    length : Cell<usize>,
}

impl<T, const BLOCK_SIZE : usize> BlockVec<T, BLOCK_SIZE> {
    pub fn new() -> Self {
        Self{blocks : UnsafeCell::new(Vec::new()), length : Cell::new(0)}
    }

    /*
        Critically, takes a CONST self, because using this will not invalidate any references derived from this
        However, IndexMut still requires a mutable reference, since we can edit any arbitrary element. 
        Because it would conflict with this function, this class does not provide an immutable iterator. 
    */ 
    pub fn alloc(&self, obj : T) -> usize {
        let b = self.blocks.get();

        let allocated_id = self.length.get();
        if allocated_id % BLOCK_SIZE == 0 {
            // New block
            
            let new_block : Box<MaybeUninit<[MaybeUninit<T>; BLOCK_SIZE]>> = Box::new(MaybeUninit::uninit());
            unsafe {
                let mut new_block_box = std::mem::transmute::<Box<MaybeUninit<[MaybeUninit<T>; BLOCK_SIZE]>>, Box<[MaybeUninit<T>; BLOCK_SIZE]>>(new_block);
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
}

impl<T, const BLOCK_SIZE : usize> Drop for BlockVec<T, BLOCK_SIZE> {
    fn drop(&mut self) {
        let num_full_blocks = self.length.get() / BLOCK_SIZE;
        let num_remaining = self.length.get() % BLOCK_SIZE;

        let block_vec = self.blocks.get_mut();
        for i in 0..num_full_blocks {
            for v in block_vec[i].deref_mut() {
                unsafe{v.assume_init_drop();}
            }
        }

        if num_remaining > 0 {
            let last_block = block_vec[num_full_blocks].deref_mut();
            for i in 0..num_remaining {
                unsafe{last_block[i].assume_init_drop()};
            }
        }
    }
}

impl<T, const BLOCK_SIZE : usize> Index<usize> for BlockVec<T, BLOCK_SIZE> {
    type Output = T;

    fn index(&self, index: usize) -> &T {
        let self_len = self.length.get();
        if index >= self_len {panic!("Index is out of bounds (idx is {index}, len is {self_len})")}

        let block = index / BLOCK_SIZE;
        let idx_in_block = index % BLOCK_SIZE;

        unsafe{
            let vec = self.blocks.get();
            (*vec)[block].deref()[idx_in_block].assume_init_ref()
        }
    }
}
impl<T, const BLOCK_SIZE : usize> IndexMut<usize> for BlockVec<T, BLOCK_SIZE> {
    fn index_mut(&mut self, index: usize) -> &mut T {
        let self_len = self.length.get();
        if index >= self_len {panic!("Index is out of bounds (idx is {index}, len is {self_len})")}

        let block = index / BLOCK_SIZE;
        let idx_in_block = index % BLOCK_SIZE;

        let vec = self.blocks.get_mut();
        unsafe{vec[block].deref_mut()[idx_in_block].assume_init_mut()}
    }
}

pub struct BlockVecIterMut<'bv, T, const BLOCK_SIZE : usize> {
    block_vec_iter : <&'bv mut Vec<Box<[MaybeUninit<T>; BLOCK_SIZE]>> as IntoIterator>::IntoIter,
    current_block : std::slice::IterMut<'bv, MaybeUninit<T>>,
    remaining : usize
}

impl<'bv, T, const BLOCK_SIZE : usize> Iterator for BlockVecIterMut<'bv, T, BLOCK_SIZE> {
    type Item = &'bv mut T;

    fn next(&mut self) -> Option<&'bv mut T> {
        if self.remaining > 0 {
            self.remaining -= 1;
            
            let found = if let Some(elem) = self.current_block.next() {
                elem
            } else {
                self.current_block = self.block_vec_iter.next().unwrap().iter_mut();
                self.current_block.next().unwrap()
            };
            unsafe{Some(found.assume_init_mut())}
        } else {
            return None;
        }
    }
}

impl<'bv, T, const BLOCK_SIZE : usize> IntoIterator for &'bv mut BlockVec<T, BLOCK_SIZE> {
    type Item = &'bv mut T;

    type IntoIter = BlockVecIterMut<'bv, T, BLOCK_SIZE>;

    fn into_iter(self) -> Self::IntoIter {
        let block_vec_iter = self.blocks.get_mut().iter_mut();
        BlockVecIterMut{
            block_vec_iter,
            current_block : std::slice::IterMut::default(),
            remaining : self.length.get()
        }
    }
}

pub struct BlockVecConsumingIter<T, const BLOCK_SIZE : usize> {
    block_vec_iter : <Vec<Box<[MaybeUninit<T>; BLOCK_SIZE]>> as IntoIterator>::IntoIter,
    current_block : Option<Box<[MaybeUninit<T>; BLOCK_SIZE]>>,
    current_idx : usize,
    total_vec_size : usize
}

impl<T, const BLOCK_SIZE : usize> Iterator for BlockVecConsumingIter<T, BLOCK_SIZE> {
    type Item = T;

    fn next(&mut self) -> Option<T> {
        if self.current_idx < self.total_vec_size {
            let idx_in_block = self.current_idx % BLOCK_SIZE;
            self.current_idx += 1;
            
            if idx_in_block == 0 {
                self.current_block = Some(self.block_vec_iter.next().unwrap()); // Vec will always be big enough
            }

            let found = &mut self.current_block.as_mut().unwrap().as_mut_slice()[idx_in_block];
            unsafe{Some(found.assume_init_read())}
        } else {
            return None;
        }
    }
}

impl<T, const BLOCK_SIZE : usize> IntoIterator for BlockVec<T, BLOCK_SIZE> {
    type Item = T;

    type IntoIter = BlockVecConsumingIter<T, BLOCK_SIZE>;

    fn into_iter(mut self) -> Self::IntoIter {
        let total_vec_size = self.length.get();
        self.length.set(0);
        let block_vec = std::mem::replace(self.blocks.get_mut(), Vec::new());
        let block_vec_iter = block_vec.into_iter();
        BlockVecConsumingIter{
            block_vec_iter,
            current_block : None,
            current_idx : 0,
            total_vec_size
        }
    }
}

impl<T, const BLOCK_SIZE : usize> Drop for BlockVecConsumingIter<T, BLOCK_SIZE> {
    fn drop(&mut self) {
        while let Some(_) = self.next() {} // Automatically drops all remaining elements of the iterator
    }
}
