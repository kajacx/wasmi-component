use std::ops::Range;

use crate::lib_structs::{MemoryAccess, WasmValue};
use crate::pointers::FatPtr;
use crate::{ComponentValue, ConvertResult, Lower};

impl<T: ComponentValue, E: Lower<T>> Lower<Vec<T>> for [E] {
    fn lower_args(
        &self,
        args: &mut [WasmValue],
        memory: &mut impl MemoryAccess,
    ) -> ConvertResult<()> {
        debug_assert_eq!(args.len(), Vec::<T>::arg_count());

        let contents = self;
        let ptr = write_contents(contents, memory)?;
        ptr.write_to_args(args);

        Ok(())
    }

    fn lower_bytes(
        &self,
        range: Range<usize>,
        memory: &mut impl MemoryAccess,
    ) -> ConvertResult<()> {
        debug_assert_eq!(range.len(), Vec::<T>::byte_size());

        let contents = self;
        let ptr = write_contents(contents, memory)?;
        ptr.write_to_bytes(memory.slice(range)?);

        Ok(())
    }
}

impl<T: ComponentValue, E: Lower<T>> Lower<Vec<T>> for Vec<E> {
    fn lower_args(
        &self,
        args: &mut [WasmValue],
        memory: &mut impl MemoryAccess,
    ) -> ConvertResult<()> {
        self.as_slice().lower_args(args, memory)
    }

    fn lower_bytes(
        &self,
        range: Range<usize>,
        memory: &mut impl MemoryAccess,
    ) -> ConvertResult<()> {
        self.as_slice().lower_bytes(range, memory)
    }
}

impl<T: ComponentValue, E: Lower<T>, const N: usize> Lower<Vec<T>> for [E; N] {
    fn lower_args(
        &self,
        args: &mut [WasmValue],
        memory: &mut impl MemoryAccess,
    ) -> ConvertResult<()> {
        self.as_slice().lower_args(args, memory)
    }

    fn lower_bytes(
        &self,
        range: Range<usize>,
        memory: &mut impl MemoryAccess,
    ) -> ConvertResult<()> {
        self.as_slice().lower_bytes(range, memory)
    }
}

fn write_contents<T: ComponentValue>(
    contents: &[impl Lower<T>],
    memory: &mut impl MemoryAccess,
) -> ConvertResult<FatPtr> {
    let len = T::byte_size() * contents.len();
    let start = memory.allocate(len, T::byte_align())?;
    let mut index = start;

    for item in contents {
        item.lower_bytes(index..(index + T::byte_size()), memory)?;
        index += T::byte_size();
    }

    Ok(FatPtr::new(start, contents.len(), T::byte_size()))
}
