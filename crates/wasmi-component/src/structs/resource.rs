use std::marker::PhantomData;

#[derive(Debug)]
pub struct Borrow<T> {
    pub(crate) index: usize,
    _data: PhantomData<T>,
}

impl<T> Borrow<T> {
    pub(crate) fn new(index: usize) -> Self {
        Self {
            index,
            _data: PhantomData,
        }
    }
}

#[derive(Debug)]
pub struct Own<T> {
    pub(crate) index: usize,
    _data: PhantomData<T>,
}

impl<T> Own<T> {
    pub(crate) fn new(index: usize) -> Self {
        Self {
            index,
            _data: PhantomData,
        }
    }
}
