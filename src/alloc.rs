use std::{
    fmt::{Debug, Formatter},
    hash::{Hash, Hasher},
    iter::Enumerate,
    marker::PhantomData,
    ops::{Index, IndexMut},
    slice::GetDisjointMutError,
};

use crate::append_only_vec::AppendOnlyVec;

/// UUIDs are type-safe integers. They are used for [FlatAlloc] and [ArenaAllocator]
///
/// They don't support arithmetic, as they're just meant to represent pointers.
///
/// TODO add custom niche for more efficient Options, wait until custom niches are stabilized (https://internals.rust-lang.org/t/nonmaxusize-and-niche-value-optimisation/19661)
/// Maybe use NonZeroUsize (https://doc.rust-lang.org/std/num/struct.NonZeroUsize.html)
///
/// Fields are public such that get_builtin_constant!() and get_builtin_type!() can create them such that match can match over
#[allow(clippy::upper_case_acronyms)]
pub struct UUID<IndexMarker>(pub usize, pub PhantomData<IndexMarker>);

impl<IndexMarker> Clone for UUID<IndexMarker> {
    fn clone(&self) -> Self {
        *self
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

/// See [UUID]
pub trait UUIDMarker {
    const DISPLAY_NAME: &'static str;
}

impl<IndexMarker: UUIDMarker> Debug for UUID<IndexMarker> {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        f.write_str(IndexMarker::DISPLAY_NAME)?;
        self.0.fmt(f)
    }
}

impl<IndexMarker> UUID<IndexMarker> {
    pub const fn from_hidden_value(v: usize) -> Self {
        UUID(v, PhantomData)
    }

    pub const fn get_hidden_value(&self) -> usize {
        self.0
    }

    // Used to temporarily write a uuid to a field that's not known yet, such as the extent of a if statement. Should be overwritten later
    pub const PLACEHOLDER: Self = UUID(usize::MAX, PhantomData);
}

pub struct UUIDAllocator<IndexMarker> {
    cur: UUID<IndexMarker>,
}

impl<IndexMarker> Clone for UUIDAllocator<IndexMarker> {
    fn clone(&self) -> Self {
        Self { cur: self.cur }
    }
}

impl<IndexMarker> Default for UUIDAllocator<IndexMarker> {
    fn default() -> Self {
        Self::new()
    }
}

impl<IndexMarker> UUIDAllocator<IndexMarker> {
    pub fn new() -> Self {
        Self {
            cur: UUID(0, PhantomData),
        }
    }
    pub fn new_start_from(start: UUID<IndexMarker>) -> Self {
        Self { cur: start }
    }
    pub fn alloc(&mut self) -> UUID<IndexMarker> {
        let allocated_id = self.cur;
        self.cur.0 += 1;
        allocated_id
    }
    pub fn as_range(&self) -> UUIDRange<IndexMarker> {
        UUIDRange(UUID::from_hidden_value(0), self.cur)
    }
}

impl<IndexMarker: UUIDMarker> std::fmt::Debug for UUIDAllocator<IndexMarker> {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("UUIDAllocator")
            .field("count: ", &self.cur.0)
            .finish()
    }
}

impl<IndexMarker: UUIDMarker> IntoIterator for &UUIDAllocator<IndexMarker> {
    type Item = UUID<IndexMarker>;

    type IntoIter = UUIDRangeIter<IndexMarker>;

    fn into_iter(self) -> Self::IntoIter {
        UUIDRangeIter(UUID::from_hidden_value(0), self.cur)
    }
}

pub struct UUIDRange<IndexMarker>(pub UUID<IndexMarker>, pub UUID<IndexMarker>);

impl<IndexMarker> UUIDRange<IndexMarker> {
    pub const PLACEHOLDER: UUIDRange<IndexMarker> = UUIDRange(UUID::PLACEHOLDER, UUID::PLACEHOLDER);

    pub fn new(from: UUID<IndexMarker>, to: UUID<IndexMarker>) -> Self {
        Self(from, to)
    }
    pub fn new_with_length(len: usize) -> Self {
        UUIDRange(UUID(0, PhantomData), UUID(len, PhantomData))
    }
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
    pub fn contains(&self, id: UUID<IndexMarker>) -> bool {
        self.0.0 >= id.0 && self.1.0 < id.0
    }
    pub fn iter(&self) -> UUIDRangeIter<IndexMarker> {
        self.into_iter()
    }
    pub fn map<OT>(&self, f: impl FnMut(UUID<IndexMarker>) -> OT) -> FlatAlloc<OT, IndexMarker> {
        FlatAlloc {
            data: Vec::from_iter(self.iter().map(f)),
            _ph: PhantomData,
        }
    }
    pub fn try_map<OT, ErrT>(
        &self,
        mut f: impl FnMut(UUID<IndexMarker>) -> Result<OT, ErrT>,
    ) -> Result<FlatAlloc<OT, IndexMarker>, ErrT> {
        let mut data = Vec::with_capacity(self.len());
        for id in self.iter() {
            data.push(f(id)?);
        }
        Ok(FlatAlloc {
            data,
            _ph: PhantomData,
        })
    }
    pub fn len(&self) -> usize {
        self.1.0 - self.0.0
    }
    pub fn unwrap_len_1(&self) -> UUID<IndexMarker> {
        assert!(self.len() == 1);
        self.0
    }
    pub fn first(&self) -> Option<UUID<IndexMarker>> {
        (self.0.0 < self.1.0).then_some(self.0)
    }
    pub fn last(&self) -> Option<UUID<IndexMarker>> {
        (self.0.0 < self.1.0).then_some(UUID(self.1.0 - 1, PhantomData))
    }
}

impl<IndexMarker: UUIDMarker> Debug for UUIDRange<IndexMarker> {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        f.write_str(IndexMarker::DISPLAY_NAME)?;
        self.0.fmt(f)?;
        f.write_str("..")?;
        self.1.fmt(f)
    }
}

impl<IndexMarker> Clone for UUIDRange<IndexMarker> {
    fn clone(&self) -> Self {
        *self
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

impl<IndexMarker> IntoIterator for UUIDRange<IndexMarker> {
    type Item = UUID<IndexMarker>;

    type IntoIter = UUIDRangeIter<IndexMarker>;

    fn into_iter(self) -> Self::IntoIter {
        UUIDRangeIter(self.0, self.1)
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct UUIDRangeIter<IndexMarker>(UUID<IndexMarker>, UUID<IndexMarker>);

impl<IndexMarker> Iterator for UUIDRangeIter<IndexMarker> {
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
    fn size_hint(&self) -> (usize, Option<usize>) {
        let sz = self.len();
        (sz, Some(sz))
    }
}

impl<IndexMarker> ExactSizeIterator for UUIDRangeIter<IndexMarker> {
    fn len(&self) -> usize {
        self.1.0 - self.0.0
    }
}

impl<IndexMarker> UUIDRangeIter<IndexMarker> {
    pub fn skip_to(&mut self, to: UUID<IndexMarker>) {
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
    data: Vec<Option<T>>,
    free_slots: Vec<usize>,
    _ph: PhantomData<IndexMarker>,
}

impl<T, IndexMarker> ArenaAllocator<T, IndexMarker> {
    pub fn new() -> Self {
        Self {
            data: Vec::new(),
            free_slots: Vec::new(),
            _ph: PhantomData,
        }
    }
    pub fn alloc(&mut self, v: T) -> UUID<IndexMarker> {
        UUID(
            if let Some(empty_slot) = self.free_slots.pop() {
                assert!(self.data[empty_slot].is_none());
                self.data[empty_slot] = Some(v);
                empty_slot
            } else {
                let l = self.data.len();
                self.data.push(Some(v));
                l
            },
            PhantomData,
        )
    }
    pub fn reserve(&mut self) -> UUID<IndexMarker> {
        UUID(
            if let Some(empty_slot) = self.free_slots.pop() {
                assert!(self.data[empty_slot].is_none());
                self.data[empty_slot] = None;
                empty_slot
            } else {
                let l = self.data.len();
                self.data.push(None);
                l
            },
            PhantomData,
        )
    }
    pub fn free_reservation(&mut self, UUID(uuid, _): UUID<IndexMarker>) {
        assert!(self.data[uuid].is_none());
        self.free_slots.push(uuid);
    }
    pub fn revert_to_reservation(&mut self, UUID(uuid, _): UUID<IndexMarker>) {
        assert!(self.data[uuid].is_some());
        self.data[uuid] = None;
    }
    pub fn alloc_reservation(&mut self, UUID(uuid, _): UUID<IndexMarker>, v: T) {
        assert!(self.data[uuid].is_none());
        self.data[uuid] = Some(v);
    }
    pub fn free(&mut self, UUID(uuid, _): UUID<IndexMarker>) -> T {
        self.free_slots.push(uuid);
        self.data[uuid].take().unwrap()
    }
    pub fn clear(&mut self) {
        self.data.clear();
        self.free_slots.clear();
    }
    pub fn is_empty(&self) -> bool {
        self.data.len() == self.free_slots.len()
    }
    pub fn iter(&self) -> FlatOptionIterator<'_, T, IndexMarker> {
        self.into_iter()
    }
    pub fn iter_mut(&mut self) -> FlatOptionIteratorMut<'_, T, IndexMarker> {
        self.into_iter()
    }
    pub fn map<O>(
        &self,
        mut f: impl FnMut(UUID<IndexMarker>, &T) -> O,
    ) -> ArenaAllocator<O, IndexMarker> {
        let data = self
            .data
            .iter()
            .enumerate()
            .map(|(idx, v)| v.as_ref().map(|v| f(UUID::from_hidden_value(idx), v)))
            .collect();

        ArenaAllocator {
            data,
            free_slots: self.free_slots.clone(),
            _ph: PhantomData,
        }
    }
    pub fn find(
        &self,
        mut predicate: impl FnMut(UUID<IndexMarker>, &T) -> bool,
    ) -> Option<UUID<IndexMarker>> {
        self.iter()
            .find(|(id, v)| predicate(*id, v))
            .map(|(id, _)| id)
    }
    #[track_caller]
    pub fn get_disjoint_mut<const N: usize>(
        &mut self,
        ids: [UUID<IndexMarker>; N],
    ) -> Result<[&mut T; N], GetDisjointMutError> {
        let indices: [usize; N] = ids.map(|id| id.0);
        let many = self.data.get_disjoint_mut(indices)?;
        Ok(many.map(|r| r.as_mut().unwrap()))
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

pub struct FlatOptionIterator<'a, T, IndexMarker> {
    it: Enumerate<std::slice::Iter<'a, Option<T>>>,
    _ph: PhantomData<IndexMarker>,
}

impl<'a, T, IndexMarker> Iterator for FlatOptionIterator<'a, T, IndexMarker> {
    type Item = (UUID<IndexMarker>, &'a T);

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match self.it.next() {
                None => {
                    return None;
                }
                Some((_pos, None)) => {}
                Some((pos, Some(val))) => {
                    return Some((UUID(pos, PhantomData), val));
                }
            }
        }
    }
}

pub struct FlatOptionIteratorMut<'a, T, IndexMarker> {
    it: Enumerate<std::slice::IterMut<'a, Option<T>>>,
    _ph: PhantomData<IndexMarker>,
}

impl<'a, T, IndexMarker> Iterator for FlatOptionIteratorMut<'a, T, IndexMarker> {
    type Item = (UUID<IndexMarker>, &'a mut T);

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match self.it.next() {
                None => {
                    return None;
                }
                Some((_pos, None)) => {}
                Some((pos, Some(val))) => {
                    return Some((UUID(pos, PhantomData), val));
                }
            }
        }
    }
}

impl<'a, T, IndexMarker> IntoIterator for &'a ArenaAllocator<T, IndexMarker> {
    type Item = (UUID<IndexMarker>, &'a T);

    type IntoIter = FlatOptionIterator<'a, T, IndexMarker>;

    fn into_iter(self) -> Self::IntoIter {
        FlatOptionIterator {
            it: self.data.iter().enumerate(),
            _ph: PhantomData,
        }
    }
}

impl<'a, T, IndexMarker> IntoIterator for &'a mut ArenaAllocator<T, IndexMarker> {
    type Item = (UUID<IndexMarker>, &'a mut T);

    type IntoIter = FlatOptionIteratorMut<'a, T, IndexMarker>;

    fn into_iter(self) -> Self::IntoIter {
        FlatOptionIteratorMut {
            it: self.data.iter_mut().enumerate(),
            _ph: PhantomData,
        }
    }
}

impl<T, IndexMarker> IntoIterator for ArenaAllocator<T, IndexMarker> {
    type Item = (UUID<IndexMarker>, T);

    type IntoIter = FlatOptionConsumingIterator<T, IndexMarker>;

    fn into_iter(self) -> Self::IntoIter {
        FlatOptionConsumingIterator {
            it: self.data.into_iter().enumerate(),
            _ph: PhantomData,
        }
    }
}

pub struct FlatOptionConsumingIterator<T, IndexMarker> {
    it: Enumerate<std::vec::IntoIter<Option<T>>>,
    _ph: PhantomData<IndexMarker>,
}
impl<T, IndexMarker> Iterator for FlatOptionConsumingIterator<T, IndexMarker> {
    type Item = (UUID<IndexMarker>, T);

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match self.it.next() {
                None => {
                    return None;
                }
                Some((_pos, None)) => {}
                Some((pos, Some(val))) => {
                    return Some((UUID(pos, PhantomData), val));
                }
            }
        }
    }
}

pub struct ArenaVector<T, IndexMarker> {
    data: Vec<Option<T>>,
    _ph: PhantomData<IndexMarker>,
}

impl<T, IndexMarker> Default for ArenaVector<T, IndexMarker> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T, IndexMarker> ArenaVector<T, IndexMarker> {
    pub fn new() -> Self {
        Self {
            data: Vec::new(),
            _ph: PhantomData,
        }
    }
    pub fn insert(&mut self, UUID(uuid, _): UUID<IndexMarker>, value: T) {
        while uuid >= self.data.len() {
            self.data.push(None);
        }
        assert!(self.data[uuid].is_none());
        self.data[uuid] = Some(value);
    }
    pub fn remove(&mut self, UUID(uuid, _): UUID<IndexMarker>) {
        self.data[uuid] = None;
    }
    pub fn iter(&self) -> FlatOptionIterator<'_, T, IndexMarker> {
        self.into_iter()
    }
    pub fn iter_mut(&mut self) -> FlatOptionIteratorMut<'_, T, IndexMarker> {
        self.into_iter()
    }
    pub fn find(
        &self,
        mut predicate: impl FnMut(UUID<IndexMarker>, &T) -> bool,
    ) -> Option<UUID<IndexMarker>> {
        self.iter()
            .find(|(id, v)| predicate(*id, v))
            .map(|(id, _)| id)
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

    type IntoIter = FlatOptionIterator<'a, T, IndexMarker>;

    fn into_iter(self) -> Self::IntoIter {
        FlatOptionIterator {
            it: self.data.iter().enumerate(),
            _ph: PhantomData,
        }
    }
}

impl<'a, T, IndexMarker> IntoIterator for &'a mut ArenaVector<T, IndexMarker> {
    type Item = (UUID<IndexMarker>, &'a mut T);

    type IntoIter = FlatOptionIteratorMut<'a, T, IndexMarker>;

    fn into_iter(self) -> Self::IntoIter {
        FlatOptionIteratorMut {
            it: self.data.iter_mut().enumerate(),
            _ph: PhantomData,
        }
    }
}

pub struct FlatAlloc<T, IndexMarker> {
    data: Vec<T>,
    _ph: PhantomData<IndexMarker>,
}

impl<T, IndexMarker> Default for FlatAlloc<T, IndexMarker> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T, IndexMarker> FlatAlloc<T, IndexMarker> {
    pub const EMPTY_FLAT_ALLOC: Self = Self::new();

    pub const fn new() -> Self {
        Self {
            data: Vec::new(),
            _ph: PhantomData,
        }
    }
    pub fn with_capacity(cap: usize) -> Self {
        Self {
            data: Vec::with_capacity(cap),
            _ph: PhantomData,
        }
    }
    pub fn with_size(size: usize, v: T) -> Self
    where
        T: Clone,
    {
        let mut data = Vec::new();
        data.resize(size, v);
        Self {
            data,
            _ph: PhantomData,
        }
    }
    pub fn from_vec(data: Vec<T>) -> Self {
        Self {
            data,
            _ph: PhantomData,
        }
    }
    pub fn into_vec(self) -> Vec<T> {
        self.data
    }
    pub fn get_next_alloc_id(&self) -> UUID<IndexMarker> {
        let uuid = self.data.len();
        UUID(uuid, PhantomData)
    }
    pub fn last_id(&self) -> UUID<IndexMarker> {
        assert!(
            !self.data.is_empty(),
            "Can't get last_id on empty FlatAlloc"
        );
        UUID(self.data.len() - 1, PhantomData)
    }
    pub fn alloc(&mut self, value: T) -> UUID<IndexMarker> {
        let uuid = self.data.len();
        self.data.push(value);
        UUID(uuid, PhantomData)
    }
    #[track_caller]
    pub fn alloc_next_alloc_id(&mut self, id: UUID<IndexMarker>, value: T)
    where
        IndexMarker: UUIDMarker,
    {
        let found_id = self.alloc(value);
        assert_eq!(
            id, found_id,
            "There was an element inserted between a call to [FlatAlloc::get_next_alloc_id] and [FlatAlloc::alloc_next_alloc_id]"
        )
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
    pub fn iter(&self) -> FlatAllocIter<'_, T, IndexMarker> {
        self.into_iter()
    }
    pub fn iter_mut(&mut self) -> FlatAllocIterMut<'_, T, IndexMarker> {
        self.into_iter()
    }
    pub fn map<'slf, OT>(
        &'slf self,
        f: impl FnMut((UUID<IndexMarker>, &'slf T)) -> OT,
    ) -> FlatAlloc<OT, IndexMarker> {
        FlatAlloc {
            data: Vec::from_iter(self.iter().map(f)),
            _ph: PhantomData,
        }
    }
    pub fn map2<'s1, 's2, T2, OT>(
        &'s1 self,
        second: &'s2 FlatAlloc<T2, IndexMarker>,
        f: impl FnMut((UUID<IndexMarker>, &'s1 T, &'s2 T2)) -> OT,
    ) -> FlatAlloc<OT, IndexMarker> {
        FlatAlloc {
            data: Vec::from_iter(zip_eq(self.iter(), second.iter()).map(f)),
            _ph: PhantomData,
        }
    }
    pub fn try_map<OT, ErrT>(
        &self,
        mut f: impl FnMut((UUID<IndexMarker>, &T)) -> Result<OT, ErrT>,
    ) -> Result<FlatAlloc<OT, IndexMarker>, ErrT> {
        let mut data = Vec::with_capacity(self.len());
        for id_v in self {
            data.push(f(id_v)?);
        }
        Ok(FlatAlloc {
            data,
            _ph: PhantomData,
        })
    }
    pub fn try_map2<T2, OT, ET>(
        &self,
        second: &FlatAlloc<T2, IndexMarker>,
        mut f: impl FnMut((UUID<IndexMarker>, &T, &T2)) -> Result<OT, ET>,
    ) -> Result<FlatAlloc<OT, IndexMarker>, ET> {
        let mut data = Vec::with_capacity(self.len());
        for v in zip_eq(self.iter(), second.iter()) {
            data.push(f(v)?);
        }
        Ok(FlatAlloc {
            data,
            _ph: PhantomData,
        })
    }
    pub fn try_map3<T2, T3, OT, ET>(
        &self,
        second: &FlatAlloc<T2, IndexMarker>,
        third: &FlatAlloc<T3, IndexMarker>,
        mut f: impl FnMut((UUID<IndexMarker>, &T, &T2, &T3)) -> Result<OT, ET>,
    ) -> Result<FlatAlloc<OT, IndexMarker>, ET> {
        let mut data = Vec::with_capacity(self.len());
        for v in zip_eq3(self.iter(), second.iter(), third.iter()) {
            data.push(f(v)?);
        }
        Ok(FlatAlloc {
            data,
            _ph: PhantomData,
        })
    }
    pub fn cast_to_array<const N: usize>(&self) -> &[T; N] {
        self.data.as_slice().try_into().unwrap()
    }
    pub fn cast_to_array_mut<const N: usize>(&mut self) -> &mut [T; N] {
        self.data.as_mut_slice().try_into().unwrap()
    }
    pub fn try_cast_to_array<const N: usize>(&self) -> Option<&[T; N]> {
        self.data.as_slice().try_into().ok()
    }
    pub fn try_cast_to_array_mut<const N: usize>(&mut self) -> Option<&mut [T; N]> {
        self.data.as_mut_slice().try_into().ok()
    }
    pub fn find(
        &self,
        mut predicate: impl FnMut(UUID<IndexMarker>, &T) -> bool,
    ) -> Option<UUID<IndexMarker>> {
        self.iter()
            .find(|(id, v)| predicate(*id, v))
            .map(|(id, _)| id)
    }
    pub fn range_since(&self, id: UUID<IndexMarker>) -> UUIDRange<IndexMarker> {
        UUIDRange(id, UUID(self.data.len(), PhantomData))
    }

    #[track_caller]
    pub fn get_disjoint_mut<const N: usize>(
        &mut self,
        ids: [UUID<IndexMarker>; N],
    ) -> Result<[&mut T; N], GetDisjointMutError> {
        let indices: [usize; N] = ids.map(|id| id.0);
        self.data.get_disjoint_mut(indices)
    }
    pub fn get(&self, id: UUID<IndexMarker>) -> Option<&T> {
        self.data.get(id.0)
    }
    pub fn get_mut(&mut self, id: UUID<IndexMarker>) -> Option<&mut T> {
        self.data.get_mut(id.0)
    }
}

impl<T, IndexMarker> FlatAlloc<Option<T>, IndexMarker> {
    pub fn iter_valids(&self) -> FlatOptionIterator<'_, T, IndexMarker> {
        FlatOptionIterator {
            it: self.data.iter().enumerate(),
            _ph: PhantomData,
        }
    }
    pub fn iter_valids_mut(&mut self) -> FlatOptionIteratorMut<'_, T, IndexMarker> {
        FlatOptionIteratorMut {
            it: self.data.iter_mut().enumerate(),
            _ph: PhantomData,
        }
    }
    pub fn unwrap_to_array<const N: usize>(&self) -> [&T; N] {
        assert!(self.data.len() == N);
        std::array::from_fn(|i| self.data[i].as_ref().unwrap())
    }
}

impl<T, IndexMarker> Index<UUID<IndexMarker>> for FlatAlloc<T, IndexMarker> {
    type Output = T;

    fn index(&self, UUID(uuid, _): UUID<IndexMarker>) -> &Self::Output {
        &self.data[uuid]
    }
}

impl<T, IndexMarker> IndexMut<UUID<IndexMarker>> for FlatAlloc<T, IndexMarker> {
    fn index_mut(&mut self, UUID(uuid, _): UUID<IndexMarker>) -> &mut Self::Output {
        &mut self.data[uuid]
    }
}

impl<T: Debug, IndexMarker> Debug for FlatAlloc<T, IndexMarker> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("FlatAlloc::from_vec(vec!")?;
        self.data.fmt(f)?;
        f.write_str(")")
    }
}

impl<T: Clone, IndexMarker> Clone for FlatAlloc<T, IndexMarker> {
    fn clone(&self) -> Self {
        Self {
            data: self.data.clone(),
            _ph: PhantomData,
        }
    }
}

impl<T: PartialEq, IndexMarker> PartialEq for FlatAlloc<T, IndexMarker> {
    fn eq(&self, other: &Self) -> bool {
        self.data == other.data
    }
}
impl<T: Eq, IndexMarker> Eq for FlatAlloc<T, IndexMarker> {}

impl<T: Hash, IndexMarker> std::hash::Hash for FlatAlloc<T, IndexMarker> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.data.hash(state);
    }
}

#[derive(Debug)]
pub struct FlatAllocIter<'a, T, IndexMarker> {
    iter: Enumerate<std::slice::Iter<'a, T>>,
    _ph: PhantomData<IndexMarker>,
}

impl<'a, T, IndexMarker> Iterator for FlatAllocIter<'a, T, IndexMarker> {
    type Item = (UUID<IndexMarker>, &'a T);

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|(id, v)| (UUID(id, PhantomData), v))
    }
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }
}
impl<T, IndexMarker> ExactSizeIterator for FlatAllocIter<'_, T, IndexMarker> {
    fn len(&self) -> usize {
        self.iter.len()
    }
}

impl<'a, T, IndexMarker> IntoIterator for &'a FlatAlloc<T, IndexMarker> {
    type Item = (UUID<IndexMarker>, &'a T);

    type IntoIter = FlatAllocIter<'a, T, IndexMarker>;

    fn into_iter(self) -> Self::IntoIter {
        FlatAllocIter {
            iter: self.data.iter().enumerate(),
            _ph: PhantomData,
        }
    }
}

#[derive(Debug)]
pub struct FlatAllocIterMut<'a, T, IndexMarker> {
    iter: Enumerate<std::slice::IterMut<'a, T>>,
    _ph: PhantomData<IndexMarker>,
}

impl<'a, T, IndexMarker> Iterator for FlatAllocIterMut<'a, T, IndexMarker> {
    type Item = (UUID<IndexMarker>, &'a mut T);

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|(id, v)| (UUID(id, PhantomData), v))
    }
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }
}
impl<T, IndexMarker> ExactSizeIterator for FlatAllocIterMut<'_, T, IndexMarker> {
    fn len(&self) -> usize {
        self.iter.len()
    }
}

impl<'a, T, IndexMarker> IntoIterator for &'a mut FlatAlloc<T, IndexMarker> {
    type Item = (UUID<IndexMarker>, &'a mut T);

    type IntoIter = FlatAllocIterMut<'a, T, IndexMarker>;

    fn into_iter(self) -> Self::IntoIter {
        FlatAllocIterMut {
            iter: self.data.iter_mut().enumerate(),
            _ph: PhantomData,
        }
    }
}

#[derive(Debug)]
pub struct FlatAllocConsumingIter<T, IndexMarker> {
    iter: Enumerate<std::vec::IntoIter<T>>,
    _ph: PhantomData<IndexMarker>,
}

impl<T, IndexMarker> Iterator for FlatAllocConsumingIter<T, IndexMarker> {
    type Item = (UUID<IndexMarker>, T);

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|(id, v)| (UUID(id, PhantomData), v))
    }
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }
}
impl<T, IndexMarker> ExactSizeIterator for FlatAllocConsumingIter<T, IndexMarker> {
    fn len(&self) -> usize {
        self.iter.len()
    }
}

impl<T, IndexMarker> IntoIterator for FlatAlloc<T, IndexMarker> {
    type Item = (UUID<IndexMarker>, T);

    type IntoIter = FlatAllocConsumingIter<T, IndexMarker>;

    fn into_iter(self) -> Self::IntoIter {
        FlatAllocConsumingIter {
            iter: self.data.into_iter().enumerate(),
            _ph: PhantomData,
        }
    }
}

#[derive(Debug)]
pub struct ZippedIterator<
    IDMarker,
    OA,
    OB,
    IterA: Iterator<Item = (UUID<IDMarker>, OA)>,
    IterB: Iterator<Item = (UUID<IDMarker>, OB)>,
> {
    iter_a: IterA,
    iter_b: IterB,
}

impl<
    IDMarker,
    OA,
    OB,
    IterA: Iterator<Item = (UUID<IDMarker>, OA)>,
    IterB: Iterator<Item = (UUID<IDMarker>, OB)>,
> Iterator for ZippedIterator<IDMarker, OA, OB, IterA, IterB>
{
    type Item = (UUID<IDMarker>, OA, OB);

    #[track_caller]
    fn next(&mut self) -> Option<Self::Item> {
        match (self.iter_a.next(), self.iter_b.next()) {
            (None, None) => None,
            (Some((id_a, a)), Some((id_b, b))) => {
                assert!(id_a == id_b);
                Some((id_a, a, b))
            }
            _ => unreachable!("Unbalanced Iterators"),
        }
    }
}

pub fn zip_eq<IDMarker, OA, OB>(
    iter_a: impl IntoIterator<Item = (UUID<IDMarker>, OA)>,
    iter_b: impl IntoIterator<Item = (UUID<IDMarker>, OB)>,
) -> impl Iterator<Item = (UUID<IDMarker>, OA, OB)> {
    ZippedIterator {
        iter_a: iter_a.into_iter(),
        iter_b: iter_b.into_iter(),
    }
}

#[derive(Debug)]
pub struct ZippedIterator3<
    IDMarker,
    OA,
    OB,
    OC,
    IterA: Iterator<Item = (UUID<IDMarker>, OA)>,
    IterB: Iterator<Item = (UUID<IDMarker>, OB)>,
    IterC: Iterator<Item = (UUID<IDMarker>, OC)>,
> {
    iter_a: IterA,
    iter_b: IterB,
    iter_c: IterC,
}

impl<
    IDMarker,
    OA,
    OB,
    OC,
    IterA: Iterator<Item = (UUID<IDMarker>, OA)>,
    IterB: Iterator<Item = (UUID<IDMarker>, OB)>,
    IterC: Iterator<Item = (UUID<IDMarker>, OC)>,
> Iterator for ZippedIterator3<IDMarker, OA, OB, OC, IterA, IterB, IterC>
{
    type Item = (UUID<IDMarker>, OA, OB, OC);

    #[track_caller]
    fn next(&mut self) -> Option<Self::Item> {
        match (self.iter_a.next(), self.iter_b.next(), self.iter_c.next()) {
            (None, None, None) => None,
            (Some((id_a, a)), Some((id_b, b)), Some((id_c, c))) => {
                assert!(id_a == id_b);
                assert!(id_a == id_c);
                Some((id_a, a, b, c))
            }
            _ => unreachable!("Unbalanced Iterators"),
        }
    }
}

pub fn zip_eq3<IDMarker, OA, OB, OC>(
    iter_a: impl IntoIterator<Item = (UUID<IDMarker>, OA)>,
    iter_b: impl IntoIterator<Item = (UUID<IDMarker>, OB)>,
    iter_c: impl IntoIterator<Item = (UUID<IDMarker>, OC)>,
) -> impl Iterator<Item = (UUID<IDMarker>, OA, OB, OC)> {
    ZippedIterator3 {
        iter_a: iter_a.into_iter(),
        iter_b: iter_b.into_iter(),
        iter_c: iter_c.into_iter(),
    }
}

#[allow(unused)]
pub struct AppendOnlyAlloc<T, IndexMarker: UUIDMarker> {
    data: AppendOnlyVec<T>,
    _ph: PhantomData<IndexMarker>,
}

impl<T, IndexMarker: UUIDMarker> Default for AppendOnlyAlloc<T, IndexMarker> {
    fn default() -> Self {
        Self::new()
    }
}

#[allow(unused)]
impl<T, IndexMarker: UUIDMarker> AppendOnlyAlloc<T, IndexMarker> {
    pub fn new() -> Self {
        Self {
            data: AppendOnlyVec::new(),
            _ph: PhantomData,
        }
    }
    pub fn alloc(&self, v: T) -> UUID<IndexMarker> {
        let idx = UUID(self.data.len(), PhantomData);
        self.data.push(v);
        idx
    }
    pub fn len(&self) -> usize {
        self.data.len()
    }
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    /// No clone_elem with similar reasoning as [std::cell::Cell]
    pub fn copy_elem(&self, idx: UUID<IndexMarker>) -> T
    where
        T: Copy,
    {
        self.data.copy_elem(idx.0)
    }

    /// Returns the old value
    pub fn set_elem(&self, idx: UUID<IndexMarker>, v: T) -> T {
        self.data.set_elem(idx.0, v)
    }
}
