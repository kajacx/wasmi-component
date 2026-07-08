use std::ops::Range;

use anyhow::Result;
use wasmi::Val;

use crate::{ComponentValue, FatPtr, LowerVal, MemoryAccess};

impl<T: ComponentValue, S: AsSlice> LowerVal<Vec<T>> for S
where
    S::Target: LowerVal<T>,
{
    fn lower_args(&self, args: &mut [Val], memory: &mut impl MemoryAccess) -> Result<()> {
        debug_assert_eq!(args.len(), Vec::<T>::arg_count());

        let contents = self.as_slice();
        let ptr = write_contents(contents, memory)?;
        ptr.write_to_args(args);

        Ok(())
    }

    fn lower_bytes(&self, range: Range<usize>, memory: &mut impl MemoryAccess) -> Result<()> {
        debug_assert_eq!(range.len(), Vec::<T>::byte_size());

        let contents = self.as_slice();
        let ptr = write_contents(contents, memory)?;
        ptr.write_to_bytes(memory.slice(range)?);

        Ok(())
    }
}

fn write_contents<T: ComponentValue>(
    contents: &[impl LowerVal<T>],
    memory: &mut impl MemoryAccess,
) -> Result<FatPtr> {
    let len = T::byte_size() * contents.len();
    let start = memory.allocate(len, T::byte_align())?;
    let mut index = start;

    for item in contents {
        item.lower_bytes(index..(index + T::byte_size()), memory)?;
        index += T::byte_size();
    }

    Ok(FatPtr::new(start, contents.len(), T::byte_size()))
}

// Unfortunately cannot use AsRef<[T]> directly
pub trait AsSlice {
    type Target;

    fn as_slice(&self) -> &[Self::Target];
}

impl<S: AsSlice + ?Sized> AsSlice for &S {
    type Target = S::Target;

    fn as_slice(&self) -> &[S::Target] {
        S::as_slice(*self)
    }
}

impl<S: AsSlice + ?Sized> AsSlice for &mut S {
    type Target = S::Target;

    fn as_slice(&self) -> &[S::Target] {
        S::as_slice(*self)
    }
}

impl<T, const N: usize> AsSlice for [T; N] {
    type Target = T;

    fn as_slice(&self) -> &[Self::Target] {
        self
    }
}

impl<T> AsSlice for [T] {
    type Target = T;

    fn as_slice(&self) -> &[T] {
        self
    }
}

impl<T> AsSlice for Vec<T> {
    type Target = T;

    fn as_slice(&self) -> &[T] {
        self.as_slice()
    }
}
