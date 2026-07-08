use std::ops::Range;

use anyhow::Result;
use wasmi::Val;

use crate::{ComponentValue, LowerVal, MemoryAccess};

impl LowerVal<Self> for i32 {
    fn lower_args(&self, args: &mut [Val], _memory: &mut impl MemoryAccess) -> Result<()> {
        debug_assert_eq!(args.len(), Self::arg_count());

        args[0] = Val::from(*self);

        Ok(())
    }

    fn lower_bytes(&self, range: Range<usize>, memory: &mut impl MemoryAccess) -> Result<()> {
        debug_assert_eq!(range.len(), Self::byte_size());

        memory.slice(range)?.copy_from_slice(&self.to_le_bytes());

        Ok(())
    }
}

impl LowerVal<Self> for u8 {
    fn lower_args(&self, args: &mut [Val], _memory: &mut impl MemoryAccess) -> Result<()> {
        debug_assert_eq!(args.len(), Self::arg_count());

        args[0] = Val::from(*self as i32);

        Ok(())
    }

    fn lower_bytes(&self, range: Range<usize>, memory: &mut impl MemoryAccess) -> Result<()> {
        debug_assert_eq!(range.len(), Self::byte_size());

        memory.slice(range)?.copy_from_slice(&self.to_le_bytes());

        Ok(())
    }
}

impl LowerVal<Self> for u32 {
    fn lower_args(&self, args: &mut [Val], _memory: &mut impl MemoryAccess) -> Result<()> {
        debug_assert_eq!(args.len(), Self::arg_count());

        args[0] = Val::from(*self as i32);

        Ok(())
    }

    fn lower_bytes(&self, range: Range<usize>, memory: &mut impl MemoryAccess) -> Result<()> {
        debug_assert_eq!(range.len(), Self::byte_size());

        memory.slice(range)?.copy_from_slice(&self.to_le_bytes());

        Ok(())
    }
}

impl LowerVal<Self> for u64 {
    fn lower_args(&self, args: &mut [Val], _memory: &mut impl MemoryAccess) -> Result<()> {
        debug_assert_eq!(args.len(), Self::arg_count());

        args[0] = Val::from(*self as i64);

        Ok(())
    }

    fn lower_bytes(&self, range: Range<usize>, memory: &mut impl MemoryAccess) -> Result<()> {
        debug_assert_eq!(range.len(), Self::byte_size());

        memory.slice(range)?.copy_from_slice(&self.to_le_bytes());

        Ok(())
    }
}

impl LowerVal<Self> for f32 {
    fn lower_args(&self, args: &mut [Val], _memory: &mut impl MemoryAccess) -> Result<()> {
        debug_assert_eq!(args.len(), Self::arg_count());

        args[0] = Val::from(*self);

        Ok(())
    }

    fn lower_bytes(&self, range: Range<usize>, memory: &mut impl MemoryAccess) -> Result<()> {
        debug_assert_eq!(range.len(), Self::byte_size());

        memory.slice(range)?.copy_from_slice(&self.to_le_bytes());

        Ok(())
    }
}
