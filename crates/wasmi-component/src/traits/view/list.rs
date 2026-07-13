use std::fmt::Debug;
use std::marker::PhantomData;

use crate::{ComponentValue, ConvertError, ConvertResult, LeBytesU8, View};

/// T is the canonical type
#[derive(Clone, Copy)]
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
        T: ComponentValue,
    {
        debug_assert_eq!(count * T::byte_size(), slice.len());

        // Re-check that slice is in range of memory
        debug_assert!(ptr_start(slice) >= ptr_start(memory));
        debug_assert!(ptr_start(slice) + slice.len() <= ptr_start(memory) + memory.len());

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

    pub fn get(&self, index: usize) -> ConvertResult<T::Borrowed<'a>>
    where
        T: ComponentValue,
    {
        if index >= self.len() {
            return Err(ConvertError::new(format!(
                "ListAccessor<{}>: index {index} is out of range {}",
                std::any::type_name::<T>(),
                self.len()
            )));
        }

        let start = index * T::byte_size();

        T::lift_bytes(&self.slice[start..(start + T::byte_size())], self.memory)
    }

    pub fn iter(&self) -> impl Iterator<Item = ConvertResult<T::Borrowed<'a>>>
    where
        T: ComponentValue,
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

impl<T: ComponentValue> View<Vec<T>> for ListAccessor<'_, T> {
    fn lift_owned(&self) -> ConvertResult<Vec<T>> {
        self.iter()
            .map(|value| value?.lift_owned())
            .try_fold(Vec::new(), |mut vec, value| {
                vec.push(value?);
                Ok(vec)
            })
    }

    fn lift_to(&self, target: &mut Vec<T>) -> ConvertResult<()> {
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

impl<T> Debug for ListAccessor<'_, T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ListAccessor")
            .field("type", &std::any::type_name::<T>())
            .field("count", &self.count)
            .field(
                "guest_slice",
                &PtrView::new(self.slice, ptr_start(self.memory)),
            )
            .field("host_memory", &PtrView::new(self.memory, 0))
            .finish()
    }
}

fn ptr_start<T>(pointer: &[T]) -> usize {
    pointer.as_ptr() as usize
}

struct PtrView<'a> {
    start: usize,
    data: &'a [u8],
}

impl<'a> PtrView<'a> {
    pub fn new(ptr: &'a [u8], offset: usize) -> Self {
        Self {
            start: ptr_start(ptr) - offset,
            data: ptr,
        }
    }
}

impl Debug for PtrView<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Ptr {{ start: 0x{:x}, len: {}",
            self.start,
            self.data.len()
        )?;
        if self.data.len() <= 32 {
            write!(f, ", data: 0x")?;
            for byte in self.data {
                write!(f, "{:02x}", byte)?;
            }
        }
        write!(f, " }}")
    }
}
