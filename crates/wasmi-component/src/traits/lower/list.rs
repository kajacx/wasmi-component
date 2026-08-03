use crate::lib_structs::{LowerBytesWriter, LowerWriter, MemoryAccess};
use crate::{ComponentValue, ConvertResult, Lower};

impl<T: ComponentValue, L: Lower<T>> Lower<Vec<T>> for [L] {
    fn lower(&self, writer: &mut impl LowerWriter) -> ConvertResult<()> {
        let memory = writer.memory();

        let start = memory.allocate(T::byte_size() * self.len(), T::byte_align())?;

        let mut item_writer = LowerBytesWriter::new(memory, start);
        for item in self {
            item.lower(&mut item_writer)?;
        }

        (start as u32, self.len() as u32).lower(writer)
    }
}

impl<T: ComponentValue, E: Lower<T>, const N: usize> Lower<Vec<T>> for [E; N] {
    fn lower(&self, writer: &mut impl LowerWriter) -> ConvertResult<()> {
        self.as_slice().lower(writer)
    }
}

impl<T: ComponentValue, E: Lower<T>> Lower<Vec<T>> for Vec<E> {
    fn lower(&self, writer: &mut impl LowerWriter) -> ConvertResult<()> {
        self.as_slice().lower(writer)
    }
}
