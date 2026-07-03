use anyhow::Result;
use wasmi::Val;

use crate::{CanonicalWitType, MemoryAccess};

pub trait Lower {
    type WitType: CanonicalWitType;

    fn lower(&self, output: &mut [Val], memory: &mut impl MemoryAccess) -> Result<()>;
}

impl Lower for i32 {
    type WitType = Self;

    fn lower(&self, output: &mut [Val], _memory: &mut impl MemoryAccess) -> Result<()> {
        output[0] = Val::from(*self);
        Ok(())
    }
}

impl Lower for u32 {
    type WitType = Self;

    fn lower(&self, output: &mut [Val], _memory: &mut impl MemoryAccess) -> Result<()> {
        output[0] = Val::from(*self as i32);
        Ok(())
    }
}

impl Lower for f32 {
    type WitType = Self;

    fn lower(&self, output: &mut [Val], _memory: &mut impl MemoryAccess) -> Result<()> {
        output[0] = Val::from(*self);
        Ok(())
    }
}

impl Lower for () {
    type WitType = Self;

    fn lower(&self, _output: &mut [Val], _memory: &mut impl MemoryAccess) -> Result<()> {
        Ok(())
    }
}

impl<T0: Lower> Lower for (T0,) {
    type WitType = T0::WitType;

    fn lower(&self, output: &mut [Val], memory: &mut impl MemoryAccess) -> Result<()> {
        T0::lower(&self.0, output, memory)
    }
}

impl<T0: Lower, T1: Lower> Lower for (T0, T1) {
    type WitType = (T0::WitType, T1::WitType);

    fn lower(&self, output: &mut [Val], memory: &mut impl MemoryAccess) -> Result<()> {
        T0::lower(
            &self.0,
            &mut output[0..T0::WitType::argument_count()],
            memory,
        )?;
        T1::lower(
            &self.1,
            &mut output[T0::WitType::argument_count()..],
            memory,
        )?;
        Ok(())
    }
}
