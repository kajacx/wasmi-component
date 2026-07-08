use std::marker::PhantomData;

#[derive(Debug)]
pub struct Borrow<T> {
    #[allow(unused)] // TODO: unused
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

impl<T> Clone for Borrow<T> {
    fn clone(&self) -> Self {
        Self::new(self.index)
    }
}

impl<T> Copy for Borrow<T> {}

#[derive(Debug)]
pub struct Own<T> {
    #[allow(unused)] // TODO: unused
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

impl<T> Clone for Own<T> {
    fn clone(&self) -> Self {
        Self::new(self.index)
    }
}

impl<T> Copy for Own<T> {}
