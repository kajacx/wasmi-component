use crate::lib_structs::{LowerWriter, MemoryAccess};
use crate::{ConvertResult, Lower};

impl Lower<String> for str {
    fn lower(&self, writer: &mut impl LowerWriter) -> ConvertResult<()> {
        let memory = writer.memory();

        let index = memory.allocate(self.len(), 1)?;
        let slice = memory.slice(index..(index + self.len()))?;

        slice.copy_from_slice(self.as_bytes());

        (index as u32, self.len() as u32).lower(writer)
    }
}

impl Lower<String> for String {
    fn lower(&self, writer: &mut impl LowerWriter) -> ConvertResult<()> {
        self.as_str().lower(writer)
    }
}
