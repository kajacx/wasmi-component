use std::marker::PhantomData;

use anyhow::{Result, bail};
use wasmi::{Val, ValType};

use crate::{CompValue, FatPtr};

impl<T: CompValue> CompValue for Vec<T> {
    type Borrowed<'a> = ListAccessor<'a, T>;

    fn arg_count() -> usize {
        2
    }

    fn arg_types() -> Vec<ValType> {
        vec![ValType::I32, ValType::I32]
    }

    fn lift_args<'a>(vals: &[Val], memory: &'a [u8]) -> Result<Self::Borrowed<'a>> {
        let ptr = FatPtr::from_args(vals, T::byte_size())?;
        Ok(ListAccessor::new(ptr.try_index(memory)?, ptr.count))
    }

    fn byte_align() -> usize {
        4
    }

    fn byte_size() -> usize {
        8
    }

    fn lift_bytes<'a>(bytes: &[u8], memory: &'a [u8]) -> Result<Self::Borrowed<'a>> {
        let ptr = FatPtr::from_bytes(bytes, T::byte_size())?;
        Ok(ListAccessor::new(ptr.try_index(memory)?, ptr.count))
    }

    fn into_owned(val: Self::Borrowed<'_>) -> Self {
        val.to_owned()
    }
}

#[derive(Debug, Clone, Copy)]
pub struct ListAccessor<'a, T> {
    /// Byte slice containing exactly the data in the list
    slice: &'a [u8],

    /// Number of elements in the list
    count: usize,

    _data: PhantomData<T>,
}

impl<'a, T> ListAccessor<'a, T> {
    pub(crate) fn new(slice: &'a [u8], count: usize) -> Self
    where
        T: CompValue,
    {
        debug_assert_eq!(count * T::byte_size(), slice.len());

        Self {
            slice,
            count,
            _data: PhantomData,
        }
    }

    pub fn len(&self) -> usize {
        self.count
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn get(&self, index: usize) -> Result<T::Borrowed<'a>>
    where
        T: CompValue,
    {
        if index >= self.len() {
            bail!("ListAccessor: index {index} is out of range {}", self.len());
        }

        let start = index * T::byte_size();
        T::lift_bytes(&self.slice[start..(start + T::byte_size())], self.slice)
    }

    pub fn to_owned(&self) -> Vec<T>
    where
        T: CompValue,
    {
        (0..self.len())
            .map(|index| T::into_owned(self.get(index).unwrap()))
            .collect()
    }
}
