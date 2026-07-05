use anyhow::Result;
use wasmi::Val;

use crate::{CompValue, LowerVal, MemoryAccess, round_up};

impl LowerVal for () {
    type Target = Self;

    fn lower_args(&self, output: &mut [Val], _memory: &mut impl MemoryAccess) -> Result<()> {
        debug_assert_eq!(output.len(), Self::Target::arg_count());

        Ok(())
    }

    fn lower_bytes(
        &self,
        range: std::ops::Range<usize>,
        _memory: &mut impl MemoryAccess,
    ) -> Result<()> {
        debug_assert_eq!(range.len(), Self::Target::byte_size());

        Ok(())
    }
}

impl<T: LowerVal> LowerVal for (T,) {
    type Target = (T::Target,);

    fn lower_args(&self, output: &mut [Val], memory: &mut impl MemoryAccess) -> Result<()> {
        debug_assert_eq!(output.len(), Self::Target::arg_count());

        T::lower_args(&self.0, output, memory)
    }

    fn lower_bytes(
        &self,
        range: std::ops::Range<usize>,
        memory: &mut impl MemoryAccess,
    ) -> Result<()> {
        debug_assert_eq!(range.len(), Self::Target::byte_size());

        T::lower_bytes(&self.0, range, memory)
    }
}

impl<T0: LowerVal, T1: LowerVal> LowerVal for (T0, T1) {
    type Target = (T0::Target, T1::Target);

    fn lower_args(&self, output: &mut [Val], memory: &mut impl MemoryAccess) -> Result<()> {
        debug_assert_eq!(output.len(), Self::Target::arg_count());

        let mut index = 0;

        T0::lower_args(
            &self.0,
            &mut output[index..(index + T0::Target::arg_count())],
            memory,
        )?;
        index += T0::Target::arg_count();

        T1::lower_args(
            &self.1,
            &mut output[index..(index + T1::Target::arg_count())],
            memory,
        )?;
        index += T1::Target::arg_count();

        debug_assert_eq!(index, Self::Target::arg_count());

        Ok(())
    }

    fn lower_bytes(
        &self,
        range: std::ops::Range<usize>,
        memory: &mut impl MemoryAccess,
    ) -> Result<()> {
        debug_assert_eq!(range.len(), Self::Target::byte_size());

        let align = Self::Target::byte_align();
        let mut index = range.start;

        T0::lower_bytes(&self.0, index..(index + T0::Target::byte_size()), memory)?;
        index += round_up(T0::Target::byte_size(), align);

        T1::lower_bytes(&self.1, index..(index + T1::Target::byte_size()), memory)?;
        index += round_up(T1::Target::byte_size(), align);

        debug_assert_eq!(index, range.end);

        Ok(())
    }
}
