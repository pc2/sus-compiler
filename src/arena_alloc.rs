use std::{ops::{IndexMut, Index}, marker::PhantomData, iter::Enumerate, fmt};

use crate::block_vector::{BlockVec, BlockVecIterMut, BlockVecIter};

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct UUID<IndexMarker : UUIDMarker>(usize, PhantomData<IndexMarker>);

pub trait UUIDMarker {
    const DISPLAY_NAME : &'static str;
}

impl<IndexMarker : UUIDMarker> fmt::Debug for UUID<IndexMarker> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(IndexMarker::DISPLAY_NAME)?;
        if self.0 == Self::INVALID.0 {
            f.write_str("INV")
        } else {
            self.0.fmt(f)
        }
    }
}

impl<IndexMarker : UUIDMarker> Default for UUID<IndexMarker> {
    fn default() -> Self {
        Self::INVALID
    }
}

const INVALID_UUID_VALUE : usize = usize::MAX;

impl<IndexMarker : UUIDMarker> UUID<IndexMarker> {
    pub const INVALID : Self = UUID(INVALID_UUID_VALUE, PhantomData);

    pub const fn from_hidden_value(v : usize) -> Self {
        UUID(v, PhantomData)
    }

    pub const fn get_hidden_value(&self) -> usize {
        self.0
    }
}

#[derive(Default)]
pub struct ArenaAllocator<T, IndexMarker> {
    data : Vec<Option<T>>,
    free_slots : Vec<usize>,
    _ph : PhantomData<IndexMarker>
}

impl<T, IndexMarker : UUIDMarker> ArenaAllocator<T, IndexMarker> {
    pub fn new() -> Self {
        Self{data : Vec::new(), free_slots : Vec::new(), _ph : PhantomData}
    }
    pub fn alloc(&mut self, v : T) -> UUID<IndexMarker> {
        UUID(if let Some(empty_slot) = self.free_slots.pop() {
            assert!(self.data[empty_slot].is_none());
            self.data[empty_slot] = Some(v);
            empty_slot
        } else {
            let l = self.data.len();
            self.data.push(Some(v));
            l
        }, PhantomData)
    }
    pub fn reserve(&mut self) -> UUID<IndexMarker> {
        UUID(if let Some(empty_slot) = self.free_slots.pop() {
            assert!(self.data[empty_slot].is_none());
            self.data[empty_slot] = None;
            empty_slot
        } else {
            let l = self.data.len();
            self.data.push(None);
            l
        }, PhantomData)
    }
    pub fn revert_to_reservation(&mut self, UUID(uuid, _) : UUID<IndexMarker>) {
        assert!(self.data[uuid].is_some());
        self.data[uuid] = None;
    }
    pub fn alloc_reservation(&mut self, UUID(uuid, _) : UUID<IndexMarker>, v : T) {
        assert!(self.data[uuid].is_none());
        self.data[uuid] = Some(v);
    }
    pub fn free(&mut self, UUID(uuid, _) : UUID<IndexMarker>) -> T {
        self.free_slots.push(uuid);
        std::mem::replace(&mut self.data[uuid], None).unwrap()
    }
    pub fn iter<'a>(&'a self) -> ArenaIterator<'a, T, IndexMarker> {
        self.into_iter()
    }
    pub fn iter_mut<'a>(&'a mut self) -> ArenaIteratorMut<'a, T, IndexMarker> {
        self.into_iter()
    }
}

impl<T, IndexMarker : UUIDMarker> Index<UUID<IndexMarker>> for ArenaAllocator<T, IndexMarker> {
    type Output = T;

    fn index(&self, UUID(uuid, _): UUID<IndexMarker>) -> &Self::Output {
        assert!(uuid != INVALID_UUID_VALUE, "Invalid UUID passed to index");
        assert!(self.data[uuid].is_some());
        self.data[uuid].as_ref().unwrap()
    }
}

impl<T, IndexMarker : UUIDMarker> IndexMut<UUID<IndexMarker>> for ArenaAllocator<T, IndexMarker> {
    fn index_mut(&mut self, UUID(uuid, _): UUID<IndexMarker>) -> &mut Self::Output {
        assert!(uuid != INVALID_UUID_VALUE, "Invalid UUID passed to index_mut");
        assert!(self.data[uuid].is_some());
        self.data[uuid].as_mut().unwrap()
    }
}

pub struct ArenaIterator<'a, T, IndexMarker> {
    it: Enumerate<std::slice::Iter<'a, Option<T>>>,
    _ph : PhantomData<IndexMarker>
}

impl<'a, T, IndexMarker : UUIDMarker> Iterator for ArenaIterator<'a, T, IndexMarker> {
    type Item = (UUID<IndexMarker>, &'a T);

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match self.it.next() {
                None => {
                    return None;
                },
                Some((_pos, None)) => {},
                Some((pos, Some(val))) => {
                    return Some((UUID(pos, PhantomData), val));
                }
            }
        }
    }
}

pub struct ArenaIteratorMut<'a, T, IndexMarker> {
    it: Enumerate<std::slice::IterMut<'a, Option<T>>>,
    _ph : PhantomData<IndexMarker>
}

impl<'a, T, IndexMarker : UUIDMarker> Iterator for ArenaIteratorMut<'a, T, IndexMarker> {
    type Item = (UUID<IndexMarker>, &'a mut T);

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match self.it.next() {
                None => {
                    return None;
                },
                Some((_pos, None)) => {},
                Some((pos, Some(val))) => {
                    return Some((UUID(pos, PhantomData), val));
                }
            }
        }
    }
}

impl<'a, T, IndexMarker : UUIDMarker> IntoIterator for &'a ArenaAllocator<T, IndexMarker> {
    type Item = (UUID<IndexMarker>, &'a T);

    type IntoIter = ArenaIterator<'a, T, IndexMarker>;

    fn into_iter(self) -> Self::IntoIter {
        ArenaIterator{it : self.data.iter().enumerate(), _ph : PhantomData}
    }
}

impl<'a, T, IndexMarker : UUIDMarker> IntoIterator for &'a mut ArenaAllocator<T, IndexMarker> {
    type Item = (UUID<IndexMarker>, &'a mut T);

    type IntoIter = ArenaIteratorMut<'a, T, IndexMarker>;

    fn into_iter(self) -> Self::IntoIter {
        ArenaIteratorMut{it : self.data.iter_mut().enumerate(), _ph : PhantomData}
    }
}

pub struct ArenaVector<T, IndexMarker> {
    data : Vec<Option<T>>,
    _ph : PhantomData<IndexMarker>
}

impl<T, IndexMarker : UUIDMarker> ArenaVector<T, IndexMarker> {
    pub fn new() -> Self {
        Self{data : Vec::new(), _ph : PhantomData}
    }
    pub fn insert(&mut self, UUID(uuid, _) : UUID<IndexMarker>, value : T) {
        while uuid >= self.data.len() {
            self.data.push(None);
        }
        assert!(self.data[uuid].is_none());
        self.data[uuid] = Some(value);
    }
    pub fn remove(&mut self, UUID(uuid, _) : UUID<IndexMarker>) {
        self.data[uuid] = None;
    }
    pub fn iter<'a>(&'a self) -> ArenaIterator<'a, T, IndexMarker> {
        self.into_iter()
    }
    pub fn iter_mut<'a>(&'a mut self) -> ArenaIteratorMut<'a, T, IndexMarker> {
        self.into_iter()
    }
}

impl<T, IndexMarker : UUIDMarker> Index<UUID<IndexMarker>> for ArenaVector<T, IndexMarker> {
    type Output = T;

    fn index(&self, UUID(uuid, _): UUID<IndexMarker>) -> &Self::Output {
        assert!(uuid != INVALID_UUID_VALUE, "Invalid UUID passed to index");
        self.data[uuid].as_ref().unwrap()
    }
}

impl<T, IndexMarker : UUIDMarker> IndexMut<UUID<IndexMarker>> for ArenaVector<T, IndexMarker> {
    fn index_mut(&mut self, UUID(uuid, _): UUID<IndexMarker>) -> &mut Self::Output {
        assert!(uuid != INVALID_UUID_VALUE, "Invalid UUID passed to index_mut");
        self.data[uuid].as_mut().unwrap()
    }
}

impl<'a, T, IndexMarker : UUIDMarker> IntoIterator for &'a ArenaVector<T, IndexMarker> {
    type Item = (UUID<IndexMarker>, &'a T);

    type IntoIter = ArenaIterator<'a, T, IndexMarker>;

    fn into_iter(self) -> Self::IntoIter {
        ArenaIterator{it : self.data.iter().enumerate(), _ph : PhantomData}
    }
}

impl<'a, T, IndexMarker : UUIDMarker> IntoIterator for &'a mut ArenaVector<T, IndexMarker> {
    type Item = (UUID<IndexMarker>, &'a mut T);

    type IntoIter = ArenaIteratorMut<'a, T, IndexMarker>;

    fn into_iter(self) -> Self::IntoIter {
        ArenaIteratorMut{it : self.data.iter_mut().enumerate(), _ph : PhantomData}
    }
}

#[derive(Debug)]
pub struct ListAllocator<T, IndexMarker : UUIDMarker> {
    data : BlockVec<T>,
    _ph : PhantomData<IndexMarker>
}

impl<T, IndexMarker : UUIDMarker> ListAllocator<T, IndexMarker> {
    pub fn new() -> Self {
        Self{data : BlockVec::new(), _ph : PhantomData}
    }
    // Allocation is const because it doesn't invalidate existing references
    pub fn alloc(&self, v : T) -> UUID<IndexMarker> {
        UUID(self.data.alloc(v), PhantomData)
    }
    pub fn iter<'a>(&'a self) -> ListAllocIterator<'a, T, IndexMarker> {
        self.into_iter()
    }
    pub fn iter_mut<'a>(&'a mut self) -> ListAllocIteratorMut<'a, T, IndexMarker> {
        self.into_iter()
    }
}

impl<T, IndexMarker : UUIDMarker> Index<UUID<IndexMarker>> for ListAllocator<T, IndexMarker> {
    type Output = T;

    fn index(&self, UUID(uuid, _): UUID<IndexMarker>) -> &Self::Output {
        assert!(uuid != INVALID_UUID_VALUE, "Invalid UUID passed to index");
        &self.data[uuid]
    }
}

impl<T, IndexMarker : UUIDMarker> IndexMut<UUID<IndexMarker>> for ListAllocator<T, IndexMarker> {
    fn index_mut(&mut self, UUID(uuid, _): UUID<IndexMarker>) -> &mut Self::Output {
        assert!(uuid != INVALID_UUID_VALUE, "Invalid UUID passed to index_mut");
        &mut self.data[uuid]
    }
}

impl<T, IndexMarker : UUIDMarker> FromIterator<T> for ListAllocator<T, IndexMarker> {
    fn from_iter<Iter: IntoIterator<Item = T>>(iter: Iter) -> Self {
        Self { data: BlockVec::from_iter(iter), _ph: PhantomData }
    }
}

pub struct ListAllocIterator<'a, T, IndexMarker> {
    it: Enumerate<BlockVecIter<'a, T>>,
    _ph : PhantomData<IndexMarker>
}

impl<'a, T, IndexMarker : UUIDMarker> Iterator for ListAllocIterator<'a, T, IndexMarker> {
    type Item = (UUID<IndexMarker>, &'a T);

    fn next(&mut self) -> Option<Self::Item> {
        match self.it.next() {
            None => {
                return None;
            },
            Some((pos, val)) => {
                return Some((UUID(pos, PhantomData), val));
            }
        }
    }
}

pub struct ListAllocIteratorMut<'a, T, IndexMarker> {
    it: Enumerate<BlockVecIterMut<'a, T>>,
    _ph : PhantomData<IndexMarker>
}

impl<'a, T, IndexMarker : UUIDMarker> Iterator for ListAllocIteratorMut<'a, T, IndexMarker> {
    type Item = (UUID<IndexMarker>, &'a mut T);

    fn next(&mut self) -> Option<Self::Item> {
        match self.it.next() {
            None => {
                return None;
            },
            Some((pos, val)) => {
                return Some((UUID(pos, PhantomData), val));
            }
        }
    }
}

impl<'a, T, IndexMarker : UUIDMarker> IntoIterator for &'a ListAllocator<T, IndexMarker> {
    type Item = (UUID<IndexMarker>, &'a T);

    type IntoIter = ListAllocIterator<'a, T, IndexMarker>;

    fn into_iter(self) -> Self::IntoIter {
        ListAllocIterator{it : self.data.iter().enumerate(), _ph : PhantomData}
    }
}

impl<'a, T, IndexMarker : UUIDMarker> IntoIterator for &'a mut ListAllocator<T, IndexMarker> {
    type Item = (UUID<IndexMarker>, &'a mut T);

    type IntoIter = ListAllocIteratorMut<'a, T, IndexMarker>;

    fn into_iter(self) -> Self::IntoIter {
        ListAllocIteratorMut{it : self.data.iter_mut().enumerate(), _ph : PhantomData}
    }
}



#[derive(Debug)]
pub struct FlatAlloc<T, IndexMarker> {
    data : Vec<T>,
    _ph : PhantomData<IndexMarker>
}

impl<T, IndexMarker : UUIDMarker> FlatAlloc<T, IndexMarker> {
    pub fn new() -> Self {
        Self{data : Vec::new(), _ph : PhantomData}
    }
    pub fn alloc(&mut self, value : T) -> UUID<IndexMarker> {
        let uuid = self.data.len();
        self.data.push(value);
        UUID(uuid, PhantomData)
    }
    pub fn iter<'a>(&'a self) -> FlatAllocIter<'a, T, IndexMarker> {
        self.into_iter()
    }
    pub fn iter_mut<'a>(&'a mut self) -> FlatAllocIterMut<'a, T, IndexMarker> {
        self.into_iter()
    }
}

impl<T, IndexMarker : UUIDMarker> Index<UUID<IndexMarker>> for FlatAlloc<T, IndexMarker> {
    type Output = T;

    fn index(&self, UUID(uuid, _): UUID<IndexMarker>) -> &Self::Output {
        assert!(uuid != INVALID_UUID_VALUE, "Invalid UUID passed to index");
        &self.data[uuid]
    }
}

impl<T, IndexMarker : UUIDMarker> IndexMut<UUID<IndexMarker>> for FlatAlloc<T, IndexMarker> {
    fn index_mut(&mut self, UUID(uuid, _): UUID<IndexMarker>) -> &mut Self::Output {
        assert!(uuid != INVALID_UUID_VALUE, "Invalid UUID passed to index_mut");
        &mut self.data[uuid]
    }
}

#[derive(Debug)]
pub struct FlatAllocIter<'a, T, IndexMarker : UUIDMarker> {
    iter : Enumerate<std::slice::Iter<'a, T>>,
    _ph : PhantomData<IndexMarker>
}

impl<'a, T, IndexMarker : UUIDMarker> Iterator for FlatAllocIter<'a, T, IndexMarker> {
    type Item = (UUID<IndexMarker>, &'a T);

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|(id, v)| (UUID(id, PhantomData), v))
    }
}

impl<'a, T, IndexMarker : UUIDMarker> IntoIterator for &'a FlatAlloc<T, IndexMarker> {
    type Item = (UUID<IndexMarker>, &'a T);

    type IntoIter = FlatAllocIter<'a, T, IndexMarker>;

    fn into_iter(self) -> Self::IntoIter {
        FlatAllocIter{iter : self.data.iter().enumerate(), _ph : PhantomData}
    }
}

#[derive(Debug)]
pub struct FlatAllocIterMut<'a, T, IndexMarker : UUIDMarker> {
    iter : Enumerate<std::slice::IterMut<'a, T>>,
    _ph : PhantomData<IndexMarker>
}

impl<'a, T, IndexMarker : UUIDMarker> Iterator for FlatAllocIterMut<'a, T, IndexMarker> {
    type Item = (UUID<IndexMarker>, &'a mut T);

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|(id, v)| (UUID(id, PhantomData), v))
    }
}

impl<'a, T, IndexMarker : UUIDMarker> IntoIterator for &'a mut FlatAlloc<T, IndexMarker> {
    type Item = (UUID<IndexMarker>, &'a mut T);

    type IntoIter = FlatAllocIterMut<'a, T, IndexMarker>;

    fn into_iter(self) -> Self::IntoIter {
        FlatAllocIterMut{iter : self.data.iter_mut().enumerate(), _ph : PhantomData}
    }
}

impl<'a, T, IndexMarker : UUIDMarker> FromIterator<T> for FlatAlloc<T, IndexMarker> {
    fn from_iter<Iter: IntoIterator<Item = T>>(iter: Iter) -> Self {
        Self { data: Vec::from_iter(iter), _ph: PhantomData }
    }
}