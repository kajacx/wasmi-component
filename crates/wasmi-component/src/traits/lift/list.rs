use std::fmt::Debug;
use std::marker::PhantomData;

use crate::lib_structs::LiftBytesReader;
use crate::pointers::{PtrView, ptr_start};
use crate::{ComponentValue, ConvertError, ConvertResult, Lift};

/// T is the canonical type.
///
/// ListAccessor existing guarantees that `slice` has the correct size.
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

        let mut reader =
            LiftBytesReader::new(self.memory, &self.slice[start..(start + T::byte_size())]);
        T::lift(&mut reader)
    }

    pub fn iter(&self) -> impl Iterator<Item = ConvertResult<T::Borrowed<'a>>>
    where
        T: ComponentValue,
    {
        (0..self.len()).map(|index| self.get(index))
    }

    pub fn as_slice(&self) -> &[T]
    where
        T: TransmuteBytes,
    {
        T::transmute(self.slice)
    }
}

impl<T: ComponentValue> Lift<Vec<T>> for ListAccessor<'_, T> {
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

pub trait TransmuteBytes: Sized {
    fn transmute(bytes: &[u8]) -> &[Self];
}

macro_rules! impl_transmute_bytes {
    ($ty: ty) => {
        impl TransmuteBytes for $ty {
            fn transmute(bytes: &[u8]) -> &[Self] {
                // alignment was checked in list lift
                bytemuck::cast_slice(bytes)
            }
        }
    };
}

impl_transmute_bytes!(i8);
impl_transmute_bytes!(i16);
impl_transmute_bytes!(i32);
impl_transmute_bytes!(i64);

impl_transmute_bytes!(u8);
impl_transmute_bytes!(u16);
impl_transmute_bytes!(u32);
impl_transmute_bytes!(u64);

impl_transmute_bytes!(f32);
impl_transmute_bytes!(f64);
