use std::{ops::{IndexMut, Index}, marker::PhantomData, iter::Enumerate, fmt};

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct UUID<IndexMarker>(usize, PhantomData<IndexMarker>);

impl<IndexMarker> fmt::Debug for UUID<IndexMarker> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.0 == Self::INVALID.0 {
            f.write_str("id_INV")
        } else {
            f.write_str("id_")?;
            self.0.fmt(f)
        }
    }
}

impl<IndexMarker> Default for UUID<IndexMarker> {
    fn default() -> Self {
        Self::INVALID
    }
}

impl<IndexMarker> UUID<IndexMarker> {
    pub const INVALID : Self = UUID(usize::MAX, PhantomData);
}

#[derive(Default)]
pub struct ArenaAllocator<T, IndexMarker> {
    data : Vec<Option<T>>,
    free_slots : Vec<usize>,
    _ph : PhantomData<IndexMarker>
}

impl<T, IndexMarker> ArenaAllocator<T, IndexMarker> {
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

impl<T, IndexMarker> Index<UUID<IndexMarker>> for ArenaAllocator<T, IndexMarker> {
    type Output = T;

    fn index(&self, UUID(uuid, _): UUID<IndexMarker>) -> &Self::Output {
        assert!(self.data[uuid].is_some());
        self.data[uuid].as_ref().unwrap()
    }
}

impl<T, IndexMarker> IndexMut<UUID<IndexMarker>> for ArenaAllocator<T, IndexMarker> {
    fn index_mut(&mut self, UUID(uuid, _): UUID<IndexMarker>) -> &mut Self::Output {
        assert!(self.data[uuid].is_some());
        self.data[uuid].as_mut().unwrap()
    }
}

pub struct ArenaIterator<'a, T, IndexMarker> {
    it: Enumerate<std::slice::Iter<'a, Option<T>>>,
    _ph : PhantomData<IndexMarker>
}

impl<'a, T, IndexMarker> Iterator for ArenaIterator<'a, T, IndexMarker> {
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

impl<'a, T, IndexMarker> Iterator for ArenaIteratorMut<'a, T, IndexMarker> {
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

impl<'a, T, IndexMarker> IntoIterator for &'a ArenaAllocator<T, IndexMarker> {
    type Item = (UUID<IndexMarker>, &'a T);

    type IntoIter = ArenaIterator<'a, T, IndexMarker>;

    fn into_iter(self) -> Self::IntoIter {
        ArenaIterator{it : self.data.iter().enumerate(), _ph : PhantomData}
    }
}

impl<'a, T, IndexMarker> IntoIterator for &'a mut ArenaAllocator<T, IndexMarker> {
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

impl<T, IndexMarker> ArenaVector<T, IndexMarker> {
    pub fn new() -> Self {
        Self{data : Vec::new(), _ph : PhantomData}
    }
    pub fn insert(&mut self, UUID(uuid, _) : UUID<IndexMarker>, value : T) {
        if uuid >= self.data.len() {
            self.data.resize_with(uuid+1, || None);
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

impl<T, IndexMarker> Index<UUID<IndexMarker>> for ArenaVector<T, IndexMarker> {
    type Output = T;

    fn index(&self, UUID(uuid, _): UUID<IndexMarker>) -> &Self::Output {
        self.data[uuid].as_ref().unwrap()
    }
}

impl<T, IndexMarker> IndexMut<UUID<IndexMarker>> for ArenaVector<T, IndexMarker> {
    fn index_mut(&mut self, UUID(uuid, _): UUID<IndexMarker>) -> &mut Self::Output {
        self.data[uuid].as_mut().unwrap()
    }
}

impl<'a, T, IndexMarker> IntoIterator for &'a ArenaVector<T, IndexMarker> {
    type Item = (UUID<IndexMarker>, &'a T);

    type IntoIter = ArenaIterator<'a, T, IndexMarker>;

    fn into_iter(self) -> Self::IntoIter {
        ArenaIterator{it : self.data.iter().enumerate(), _ph : PhantomData}
    }
}

impl<'a, T, IndexMarker> IntoIterator for &'a mut ArenaVector<T, IndexMarker> {
    type Item = (UUID<IndexMarker>, &'a mut T);

    type IntoIter = ArenaIteratorMut<'a, T, IndexMarker>;

    fn into_iter(self) -> Self::IntoIter {
        ArenaIteratorMut{it : self.data.iter_mut().enumerate(), _ph : PhantomData}
    }
}

#[derive(Debug)]
pub struct ListAllocator<T, IndexMarker> {
    data : Vec<T>,
    _ph : PhantomData<IndexMarker>
}

impl<T, IndexMarker> ListAllocator<T, IndexMarker> {
    pub fn new() -> Self {
        Self{data : Vec::new(), _ph : PhantomData}
    }
    pub fn alloc(&mut self, v : T) -> UUID<IndexMarker> {
        let uuid = UUID(self.data.len(), PhantomData);
        self.data.push(v);
        uuid
    }
    pub fn iter<'a>(&'a self) -> ListAllocIterator<'a, T, IndexMarker> {
        self.into_iter()
    }
    pub fn iter_mut<'a>(&'a mut self) -> ListAllocIteratorMut<'a, T, IndexMarker> {
        self.into_iter()
    }
    pub fn map<OutT, F : FnMut(UUID<IndexMarker>, &T) -> OutT>(&self, f : &mut F) -> ListAllocator<OutT, IndexMarker> {
        let data = self.iter().map(|(a, v)| f(a, v)).collect();
        ListAllocator{data, _ph : PhantomData}
    }
}

impl<T, IndexMarker> Index<UUID<IndexMarker>> for ListAllocator<T, IndexMarker> {
    type Output = T;

    fn index(&self, UUID(uuid, _): UUID<IndexMarker>) -> &Self::Output {
        &self.data[uuid]
    }
}

impl<T, IndexMarker> IndexMut<UUID<IndexMarker>> for ListAllocator<T, IndexMarker> {
    fn index_mut(&mut self, UUID(uuid, _): UUID<IndexMarker>) -> &mut Self::Output {
        &mut self.data[uuid]
    }
}

pub struct ListAllocIterator<'a, T, IndexMarker> {
    it: Enumerate<std::slice::Iter<'a, T>>,
    _ph : PhantomData<IndexMarker>
}

impl<'a, T, IndexMarker> Iterator for ListAllocIterator<'a, T, IndexMarker> {
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
    it: Enumerate<std::slice::IterMut<'a, T>>,
    _ph : PhantomData<IndexMarker>
}

impl<'a, T, IndexMarker> Iterator for ListAllocIteratorMut<'a, T, IndexMarker> {
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

impl<'a, T, IndexMarker> IntoIterator for &'a ListAllocator<T, IndexMarker> {
    type Item = (UUID<IndexMarker>, &'a T);

    type IntoIter = ListAllocIterator<'a, T, IndexMarker>;

    fn into_iter(self) -> Self::IntoIter {
        ListAllocIterator{it : self.data.iter().enumerate(), _ph : PhantomData}
    }
}

impl<'a, T, IndexMarker> IntoIterator for &'a mut ListAllocator<T, IndexMarker> {
    type Item = (UUID<IndexMarker>, &'a mut T);

    type IntoIter = ListAllocIteratorMut<'a, T, IndexMarker>;

    fn into_iter(self) -> Self::IntoIter {
        ListAllocIteratorMut{it : self.data.iter_mut().enumerate(), _ph : PhantomData}
    }
}
