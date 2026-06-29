use crate::*;

#[derive(Debug)]
pub struct Store<T> {
    pub data: T,
}

impl<T> Store<T> {
    pub fn new(_engine: &Engine, data: T) -> Self {
        Self { data }
    }
}
