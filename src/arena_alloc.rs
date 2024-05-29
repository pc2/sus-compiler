use std::{
    cmp::Ordering, fmt::{Debug, Formatter, Result}, hash::{Hash, Hasher}, iter::Enumerate, marker::PhantomData, ops::{Index, IndexMut}
};

use crate::block_vector::{BlockVec, BlockVecIterMut, BlockVecIter};


// TODO add custom niche for more efficient Options, wait until custom niches are stabilized (https://internals.rust-lang.org/t/nonmaxusize-and-niche-value-optimisation/19661)
// Maybe use NonZeroUsize (https://doc.rust-lang.org/std/num/struct.NonZeroUsize.html)
pub struct UUID<IndexMarker>(usize, PhantomData<IndexMarker>);

impl<IndexMarker> Clone for UUID<IndexMarker> {
    fn clone(&self) -> Self {
        Self(self.0, PhantomData)
    }
}
impl<IndexMarker> Copy for UUID<IndexMarker> {}
impl<IndexMarker> PartialEq for UUID<IndexMarker> {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl<IndexMarker> Eq for UUID<IndexMarker> {}
impl<IndexMarker> Hash for UUID<IndexMarker> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.hash(state);
    }
}

pub trait UUIDMarker {
    const DISPLAY_NAME : &'static str;
}

impl<IndexMarker : UUIDMarker> Debug for UUID<IndexMarker> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        f.write_str(IndexMarker::DISPLAY_NAME)?;
        self.0.fmt(f)
    }
}

impl<IndexMarker : UUIDMarker> UUID<IndexMarker> {
    pub const fn from_hidden_value(v : usize) -> Self {
        UUID(v, PhantomData)
    }

    pub const fn get_hidden_value(&self) -> usize {
        self.0
    }

    // Used to temporarily write a uuid to a field that's not known yet, such as the extent of a if statement. Should be overwritten later
    pub const PLACEHOLDER : Self = UUID(usize::MAX, PhantomData);
}

pub struct UUIDRange<IndexMarker>(pub UUID<IndexMarker>, pub UUID<IndexMarker>);

impl<IndexMarker> UUIDRange<IndexMarker> {
    pub fn contains(&self, id : UUID<IndexMarker>) -> bool {
        self.0.0 >= id.0 && self.1.0 < id.0
    }
}

impl<IndexMarker : UUIDMarker> Debug for UUIDRange<IndexMarker> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        f.write_str(IndexMarker::DISPLAY_NAME)?;
        self.0.fmt(f)?;
        f.write_str("..")?;
        self.1.fmt(f)
    }
}

impl<IndexMarker> Clone for UUIDRange<IndexMarker> {
    fn clone(&self) -> Self {
        Self(self.0, self.1)
    }
}
impl<IndexMarker> Copy for UUIDRange<IndexMarker> {}
impl<IndexMarker> PartialEq for UUIDRange<IndexMarker> {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0 && self.1 == other.1
    }
}
impl<IndexMarker> Eq for UUIDRange<IndexMarker> {}
impl<IndexMarker> Hash for UUIDRange<IndexMarker> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.hash(state);
        self.1.hash(state);
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct UUIDRangeIter<IndexMarker : UUIDMarker>(UUID<IndexMarker>, UUID<IndexMarker>);

impl<IndexMarker : UUIDMarker> UUIDRange<IndexMarker> {
    pub fn empty() -> Self {
        UUIDRange(UUID(0, PhantomData), UUID(0, PhantomData))
    }
}

impl<IndexMarker : UUIDMarker> Iterator for UUIDRange<IndexMarker> {
    type Item = UUID<IndexMarker>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.0.0 == self.1.0 {
            None
        } else {
            let result = UUID(self.0.0, PhantomData);
            self.0.0 += 1;
            Some(result)
        }
    }
}

impl<IndexMarker : UUIDMarker> UUIDRange<IndexMarker> {
    pub fn skip_to(&mut self, to : UUID<IndexMarker>) {
        assert!(to.0 >= self.0.0);
        assert!(to.0 <= self.1.0);
        self.0 = to;
    }
    pub fn len(&self) -> usize {
        self.1.0 - self.0.0
    }
    pub fn is_empty(&self) -> bool {
        self.len() == 0
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
    pub fn clear(&mut self) {
        self.data.clear();
        self.free_slots.clear();
    }
    pub fn iter<'a>(&'a self) -> FlatOptionIterator<'a, T, IndexMarker> {
        self.into_iter()
    }
    pub fn iter_mut<'a>(&'a mut self) -> FlatOptionIteratorMut<'a, T, IndexMarker> {
        self.into_iter()
    }
}

impl<T, IndexMarker : UUIDMarker> Index<UUID<IndexMarker>> for ArenaAllocator<T, IndexMarker> {
    type Output = T;

    fn index(&self, UUID(uuid, _): UUID<IndexMarker>) -> &Self::Output {
        assert!(self.data[uuid].is_some());
        self.data[uuid].as_ref().unwrap()
    }
}

impl<T, IndexMarker : UUIDMarker> IndexMut<UUID<IndexMarker>> for ArenaAllocator<T, IndexMarker> {
    fn index_mut(&mut self, UUID(uuid, _): UUID<IndexMarker>) -> &mut Self::Output {
        assert!(self.data[uuid].is_some());
        self.data[uuid].as_mut().unwrap()
    }
}

pub struct FlatOptionIterator<'a, T, IndexMarker> {
    it: Enumerate<std::slice::Iter<'a, Option<T>>>,
    _ph : PhantomData<IndexMarker>
}

impl<'a, T, IndexMarker : UUIDMarker> Iterator for FlatOptionIterator<'a, T, IndexMarker> {
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

pub struct FlatOptionIteratorMut<'a, T, IndexMarker> {
    it: Enumerate<std::slice::IterMut<'a, Option<T>>>,
    _ph : PhantomData<IndexMarker>
}

impl<'a, T, IndexMarker : UUIDMarker> Iterator for FlatOptionIteratorMut<'a, T, IndexMarker> {
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

    type IntoIter = FlatOptionIterator<'a, T, IndexMarker>;

    fn into_iter(self) -> Self::IntoIter {
        FlatOptionIterator{it : self.data.iter().enumerate(), _ph : PhantomData}
    }
}

impl<'a, T, IndexMarker : UUIDMarker> IntoIterator for &'a mut ArenaAllocator<T, IndexMarker> {
    type Item = (UUID<IndexMarker>, &'a mut T);

    type IntoIter = FlatOptionIteratorMut<'a, T, IndexMarker>;

    fn into_iter(self) -> Self::IntoIter {
        FlatOptionIteratorMut{it : self.data.iter_mut().enumerate(), _ph : PhantomData}
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
    pub fn iter<'a>(&'a self) -> FlatOptionIterator<'a, T, IndexMarker> {
        self.into_iter()
    }
    pub fn iter_mut<'a>(&'a mut self) -> FlatOptionIteratorMut<'a, T, IndexMarker> {
        self.into_iter()
    }
}

impl<T, IndexMarker : UUIDMarker> Index<UUID<IndexMarker>> for ArenaVector<T, IndexMarker> {
    type Output = T;

    fn index(&self, UUID(uuid, _): UUID<IndexMarker>) -> &Self::Output {
        self.data[uuid].as_ref().unwrap()
    }
}

impl<T, IndexMarker : UUIDMarker> IndexMut<UUID<IndexMarker>> for ArenaVector<T, IndexMarker> {
    fn index_mut(&mut self, UUID(uuid, _): UUID<IndexMarker>) -> &mut Self::Output {
        self.data[uuid].as_mut().unwrap()
    }
}

impl<'a, T, IndexMarker : UUIDMarker> IntoIterator for &'a ArenaVector<T, IndexMarker> {
    type Item = (UUID<IndexMarker>, &'a T);

    type IntoIter = FlatOptionIterator<'a, T, IndexMarker>;

    fn into_iter(self) -> Self::IntoIter {
        FlatOptionIterator{it : self.data.iter().enumerate(), _ph : PhantomData}
    }
}

impl<'a, T, IndexMarker : UUIDMarker> IntoIterator for &'a mut ArenaVector<T, IndexMarker> {
    type Item = (UUID<IndexMarker>, &'a mut T);

    type IntoIter = FlatOptionIteratorMut<'a, T, IndexMarker>;

    fn into_iter(self) -> Self::IntoIter {
        FlatOptionIteratorMut{it : self.data.iter_mut().enumerate(), _ph : PhantomData}
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
    pub fn get_next_alloc_id(&self) -> UUID<IndexMarker> {
        UUID(self.data.len(), PhantomData)
    }
    pub fn id_range(&self) -> UUIDRange<IndexMarker> {
        UUIDRange(UUID(0, PhantomData), UUID(self.data.len(), PhantomData))
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
        &self.data[uuid]
    }
}

impl<T, IndexMarker : UUIDMarker> IndexMut<UUID<IndexMarker>> for ListAllocator<T, IndexMarker> {
    fn index_mut(&mut self, UUID(uuid, _): UUID<IndexMarker>) -> &mut Self::Output {
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



pub struct FlatAlloc<T, IndexMarker> {
    data : Vec<T>,
    _ph : PhantomData<IndexMarker>
}

impl<T, IndexMarker : UUIDMarker> FlatAlloc<T, IndexMarker> {
    pub fn new() -> Self {
        Self{data : Vec::new(), _ph : PhantomData}
    }
    pub fn with_capacity(cap : usize) -> Self {
        Self{data : Vec::with_capacity(cap), _ph : PhantomData}
    }
    pub fn get_next_alloc_id(&self) -> UUID<IndexMarker> {
        let uuid = self.data.len();
        UUID(uuid, PhantomData)
    }
    pub fn alloc(&mut self, value : T) -> UUID<IndexMarker> {
        let uuid = self.data.len();
        self.data.push(value);
        UUID(uuid, PhantomData)
    }
    pub fn len(&self) -> usize {
        self.data.len()
    }
    pub fn id_range(&self) -> UUIDRange<IndexMarker> {
        UUIDRange(UUID(0, PhantomData), UUID(self.data.len(), PhantomData))
    }
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }
    pub fn clear(&mut self) {
        self.data.clear();
    }
    pub fn iter<'a>(&'a self) -> FlatAllocIter<'a, T, IndexMarker> {
        self.into_iter()
    }
    pub fn iter_mut<'a>(&'a mut self) -> FlatAllocIterMut<'a, T, IndexMarker> {
        self.into_iter()
    }
    pub fn range_since(&self, id : UUID<IndexMarker>) -> UUIDRange<IndexMarker> {
        UUIDRange(id, UUID(self.data.len(), PhantomData))
    }
    /// TODO replace once get_many_mut stabilizes
    pub fn get2_mut(&mut self, id_a : UUID<IndexMarker>, id_b : UUID<IndexMarker>) -> Option<(&mut T, &mut T)> {
        match id_b.0.cmp(&id_a.0) {
            Ordering::Equal => None,
            Ordering::Less => {
                let (l, r) = self.data.split_at_mut(id_a.0);
                Some((&mut r[0], &mut l[id_b.0]))
            }
            Ordering::Greater => {
                let (l, r) = self.data.split_at_mut(id_b.0);
                Some((&mut l[id_a.0], &mut r[0]))
            }
        }
    }
    pub fn get(&self, id : UUID<IndexMarker>) -> Option<&T> {
        self.data.get(id.0)
    }
    pub fn get_mut(&mut self, id : UUID<IndexMarker>) -> Option<&mut T> {
        self.data.get_mut(id.0)
    }
}

impl<T, IndexMarker : UUIDMarker> FlatAlloc<Option<T>, IndexMarker> {
    pub fn iter_valids<'a>(&'a self) -> FlatOptionIterator<'a, T, IndexMarker> {
        FlatOptionIterator{ it: self.data.iter().enumerate(), _ph: PhantomData }
    }
    pub fn iter_valids_mut<'a>(&'a mut self) -> FlatOptionIteratorMut<'a, T, IndexMarker> {
        FlatOptionIteratorMut{ it: self.data.iter_mut().enumerate(), _ph: PhantomData }
    }
}

impl<T, IndexMarker : UUIDMarker> Index<UUID<IndexMarker>> for FlatAlloc<T, IndexMarker> {
    type Output = T;

    fn index(&self, UUID(uuid, _): UUID<IndexMarker>) -> &Self::Output {
        &self.data[uuid]
    }
}

impl<T, IndexMarker : UUIDMarker> IndexMut<UUID<IndexMarker>> for FlatAlloc<T, IndexMarker> {
    fn index_mut(&mut self, UUID(uuid, _): UUID<IndexMarker>) -> &mut Self::Output {
        &mut self.data[uuid]
    }
}

impl<T : Debug, IndexMarker> Debug for FlatAlloc<T, IndexMarker> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        self.data.fmt(f)
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
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }
}
impl<'a, T, IndexMarker : UUIDMarker> ExactSizeIterator for FlatAllocIter<'a, T, IndexMarker> {
    fn len(&self) -> usize {
        self.iter.len()
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
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }
}
impl<'a, T, IndexMarker : UUIDMarker> ExactSizeIterator for FlatAllocIterMut<'a, T, IndexMarker> {
    fn len(&self) -> usize {
        self.iter.len()
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