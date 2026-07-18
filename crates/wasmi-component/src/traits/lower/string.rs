use std::ops::Range;

use crate::lib_structs::{MemoryAccess, WasmValue};
use crate::pointers::FatPtr;
use crate::{ComponentValue, ConvertResult, Lower};

impl Lower<String> for str {
    fn lower_args(
        &self,
        args: &mut [WasmValue],
        memory: &mut impl MemoryAccess,
    ) -> ConvertResult<()> {
        debug_assert_eq!(args.len(), String::arg_count());

        let contents = self.as_ref();
        let ptr = write_contents(contents, memory)?;
        ptr.write_to_args(args);

        Ok(())
    }

    fn lower_bytes(
        &self,
        range: Range<usize>,
        memory: &mut impl MemoryAccess,
    ) -> ConvertResult<()> {
        debug_assert_eq!(range.len(), String::byte_size());

        let contents = self.as_ref();
        let ptr = write_contents(contents, memory)?;
        ptr.write_to_bytes(memory.slice(range)?);

        Ok(())
    }
}

impl Lower<String> for String {
    fn lower_args(
        &self,
        args: &mut [WasmValue],
        memory: &mut impl MemoryAccess,
    ) -> ConvertResult<()> {
        self.as_str().lower_args(args, memory)
    }

    fn lower_bytes(
        &self,
        range: Range<usize>,
        memory: &mut impl MemoryAccess,
    ) -> ConvertResult<()> {
        self.as_str().lower_bytes(range, memory)
    }
}

fn write_contents(contents: &str, memory: &mut impl MemoryAccess) -> ConvertResult<FatPtr> {
    let index = memory.allocate(contents.len(), 1)?;
    let slice = memory.slice(index..(index + contents.len()))?;
    slice.copy_from_slice(contents.as_bytes());
    Ok(FatPtr::new(index, contents.len(), 1))
}
