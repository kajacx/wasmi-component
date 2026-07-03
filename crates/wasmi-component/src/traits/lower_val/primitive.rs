use anyhow::Result;
use wasmi::Val;

use crate::{LowerVal, MemoryAccess};

impl LowerVal for i32 {
    type Target = Self;

    fn lower(&self, output: &mut [Val], _memory: &mut impl MemoryAccess) -> Result<()> {
        output[0] = Val::from(*self);
        Ok(())
    }
}

impl LowerVal for u32 {
    type Target = Self;

    fn lower(&self, output: &mut [Val], _memory: &mut impl MemoryAccess) -> Result<()> {
        output[0] = Val::from(*self as i32);
        Ok(())
    }
}

impl LowerVal for f32 {
    type Target = Self;

    fn lower(&self, output: &mut [Val], _memory: &mut impl MemoryAccess) -> Result<()> {
        output[0] = Val::from(*self);
        Ok(())
    }
}
