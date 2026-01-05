use std::cell::UnsafeCell;

/// An append-only Vector. The contents cannot be looked at, unless the vector is explicitly consumed, or taken using [Self::take]. This allows us to present a const-ref [Self::push], which has nice ergonomics
///
/// Basically a "vector" variant of [std::cell::Cell].
///
/// For all `unsafe` blocks:
/// SAFETY: Vector elements cannot be looked at. They can only be added or removed. The whole vector can be extracted with [Self::take].
#[derive(Debug)]
pub struct AppendOnlyVec<T>(UnsafeCell<Vec<T>>);

impl<T> Default for AppendOnlyVec<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> From<Vec<T>> for AppendOnlyVec<T> {
    fn from(existing_vec: Vec<T>) -> Self {
        Self(UnsafeCell::new(existing_vec))
    }
}

impl<T> From<AppendOnlyVec<T>> for Vec<T> {
    fn from(v: AppendOnlyVec<T>) -> Self {
        v.0.into_inner()
    }
}

impl<T> AppendOnlyVec<T> {
    pub fn new() -> Self {
        Self(UnsafeCell::new(Vec::new()))
    }
    pub fn push(&self, data: T) {
        unsafe { (*self.0.get()).push(data) }
    }
    pub fn pop(&self) -> Option<T> {
        unsafe { (*self.0.get()).pop() }
    }
    pub fn clear(&self) {
        unsafe { (*self.0.get()).clear() }
    }
    pub fn len(&self) -> usize {
        unsafe { (*self.0.get()).len() }
    }
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// No clone_elem with similar reasoning as [std::cell::Cell]
    pub fn copy_elem(&self, idx: usize) -> T
    where
        T: Copy,
    {
        unsafe { (&*self.0.get())[idx] }
    }

    pub fn set_elem(&self, idx: usize, v: T) -> T {
        unsafe {
            let vec = self.0.get();
            std::mem::replace(&mut (&mut *vec)[idx], v)
        }
    }

    /// Takes the current contents of the vector, and resets the vector to empty
    pub fn take(&self) -> Vec<T> {
        unsafe {
            let vec = self.0.get();
            std::mem::take(&mut *vec)
        }
    }
}
