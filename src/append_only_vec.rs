use std::cell::UnsafeCell;

/// An append-only Vector. The contents cannot be looked at, unless the vector is explicitly consumed. This allows us to present a const-ref [Self::push], which has nice ergonomics
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
    pub fn push(&self, data: T) {
        // SAFETY: AppendOnlyVec is made such that references to the content can only be made from exclusive references. Hence, no reference can be taken and then invalidated by a push
        unsafe {
            (*self.v.get()).push(data);
        }
    }
    pub fn new() -> Self {
        Self::default()
    }
}

impl<T> Into<Vec<T>> for AppendOnlyVec<T> {
    fn into(self) -> Vec<T> {
        self.v.into_inner()
    }
}

impl<T> IntoIterator for AppendOnlyVec<T> {
    type Item = T;
    type IntoIter = std::vec::IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        self.v.into_inner().into_iter()
    }
}
