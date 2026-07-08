use std::marker::PhantomData;

use anyhow::{Result, bail};

use crate::{CompValue, LeBytesU8, View};

/// T is the canonical type
#[derive(Debug, Clone, Copy)]
pub struct ListAccessor<'a, T> {
    /// Byte slice containing exactly the data in the list
    slice: &'a [u8],

    /// Number of elements in the list
    count: usize,

    /// The entire module's memory
    memory: &'a [u8],

    _data: PhantomData<T>,
}

impl<'a, T> ListAccessor<'a, T> {
    pub(crate) fn new(slice: &'a [u8], count: usize, memory: &'a [u8]) -> Self
    where
        T: CompValue,
    {
        debug_assert_eq!(count * T::byte_size(), slice.len());
        // TODO: check that slice is in range of memory

        Self {
            slice,
            count,
            memory,
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

        T::lift_bytes(&self.slice[start..(start + T::byte_size())], self.memory)
    }

    pub fn iter(&self) -> impl Iterator<Item = Result<T::Borrowed<'a>>>
    where
        T: CompValue,
    {
        (0..self.len()).map(|index| self.get(index))
    }

    pub fn as_u8_slice(&self) -> &[u8]
    where
        T: LeBytesU8,
    {
        self.slice
    }
}

impl<T: CompValue> View<Vec<T>> for ListAccessor<'_, T> {
    fn lift_owned(&self) -> Result<Vec<T>> {
        self.iter()
            .map(|val| val?.lift_owned())
            .try_fold(Vec::new(), |mut vec, value| {
                vec.push(value?);
                Ok(vec)
            })
    }

    fn lift_to(&self, target: &mut Vec<T>) -> Result<()> {
        for (index, item) in self.iter().enumerate() {
            if index < target.len() {
                item?.lift_to(&mut target[index])?;
            } else {
                target.push(item?.lift_owned()?);
            }
        }

        target.truncate(self.len());

        Ok(())
    }
}
