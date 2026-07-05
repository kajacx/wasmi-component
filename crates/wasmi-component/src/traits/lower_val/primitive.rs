use std::ops::Range;

use anyhow::Result;
use wasmi::Val;

use crate::{CompValue, LowerVal, MemoryAccess};

impl LowerVal for i32 {
    type Target = Self;

    fn lower_args(&self, output: &mut [Val], _memory: &mut impl MemoryAccess) -> Result<()> {
        debug_assert_eq!(output.len(), Self::arg_count());

        output[0] = Val::from(*self);

        Ok(())
    }

    fn lower_bytes(&self, range: Range<usize>, memory: &mut impl MemoryAccess) -> Result<()> {
        debug_assert_eq!(range.len(), Self::byte_size());

        memory.slice(range)?.copy_from_slice(&self.to_le_bytes());

        Ok(())
    }
}

impl LowerVal for u32 {
    type Target = Self;

    fn lower_args(&self, output: &mut [Val], _memory: &mut impl MemoryAccess) -> Result<()> {
        debug_assert_eq!(output.len(), Self::arg_count());

        output[0] = Val::from(*self as i32);

        Ok(())
    }

    fn lower_bytes(&self, range: Range<usize>, memory: &mut impl MemoryAccess) -> Result<()> {
        debug_assert_eq!(range.len(), Self::byte_size());

        memory.slice(range)?.copy_from_slice(&self.to_le_bytes());

        Ok(())
    }
}

impl LowerVal for f32 {
    type Target = Self;

    fn lower_args(&self, output: &mut [Val], _memory: &mut impl MemoryAccess) -> Result<()> {
        debug_assert_eq!(output.len(), Self::arg_count());

        output[0] = Val::from(*self);

        Ok(())
    }

    fn lower_bytes(&self, range: Range<usize>, memory: &mut impl MemoryAccess) -> Result<()> {
        debug_assert_eq!(range.len(), Self::byte_size());

        memory.slice(range)?.copy_from_slice(&self.to_le_bytes());

        Ok(())
    }
}
