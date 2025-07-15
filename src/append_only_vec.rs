use std::cell::UnsafeCell;

/// An append-only Vector. The contents cannot be looked at, unless the vector is explicitly consumed. This allows us to present a const-ref [Self::push], which has nice ergonomics
///
/// Basically a "vector" variant of [std::cell::Cell]
#[derive(Debug)]
pub struct AppendOnlyVec<T> {
    v: UnsafeCell<Vec<T>>,
}

impl<T> Default for AppendOnlyVec<T> {
    fn default() -> Self {
        Self {
            v: UnsafeCell::new(Vec::new()),
        }
    }
}

impl<T> From<Vec<T>> for AppendOnlyVec<T> {
    fn from(existing_vec: Vec<T>) -> Self {
        Self {
            v: UnsafeCell::new(existing_vec),
        }
    }
}

impl<T> AppendOnlyVec<T> {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn push(&self, data: T) {
        // SAFETY: AppendOnlyVec is made such that references to the content can only be made from exclusive references. Hence, no reference can be taken and then invalidated by a push
        unsafe {
            (*self.v.get()).push(data);
        }
    }
    pub fn len(&self) -> usize {
        unsafe { (*self.v.get()).len() }
    }
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// No clone_elem with similar reasoning as [std::cell::Cell]
    pub fn copy_elem(&self, idx: usize) -> T
    where
        T: Copy,
    {
        unsafe { (&*self.v.get())[idx] }
    }

    pub fn set_elem(&self, idx: usize, v: T) -> T {
        unsafe {
            let vec = &mut *self.v.get();
            std::mem::replace(&mut vec[idx], v)
        }
    }
}

impl<T> From<AppendOnlyVec<T>> for Vec<T> {
    fn from(val: AppendOnlyVec<T>) -> Self {
        val.v.into_inner()
    }
}
