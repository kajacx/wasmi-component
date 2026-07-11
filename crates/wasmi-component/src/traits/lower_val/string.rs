use std::ops::Range;

use wasmi::Val;

use crate::{ComponentValue, ConvertResult, FatPtr, LowerVal, MemoryAccess};

impl<T: AsRef<str>> LowerVal<String> for T {
    fn lower_args(&self, args: &mut [Val], memory: &mut impl MemoryAccess) -> ConvertResult<()> {
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

fn write_contents(contents: &str, memory: &mut impl MemoryAccess) -> ConvertResult<FatPtr> {
    let index = memory.allocate(contents.len(), 1)?;
    let slice = memory.slice(index..(index + contents.len()))?;
    slice.copy_from_slice(contents.as_bytes());
    Ok(FatPtr::new(index, contents.len(), 1))
}
