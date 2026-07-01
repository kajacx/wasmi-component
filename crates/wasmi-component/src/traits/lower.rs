use anyhow::Result;
use wasmi::Val;

use crate::MemoryAccess;

pub trait Lower {
    fn args_amount() -> usize;

    fn lower(&self, output: &mut [Val], memory: &mut impl MemoryAccess) -> Result<()>;
}

impl Lower for i32 {
    fn args_amount() -> usize {
        1
    }

    fn lower(&self, output: &mut [Val], _memory: &mut impl MemoryAccess) -> Result<()> {
        output[0] = Val::from(*self);
        Ok(())
    }
}

impl Lower for u32 {
    fn args_amount() -> usize {
        1
    }

    fn lower(&self, output: &mut [Val], _memory: &mut impl MemoryAccess) -> Result<()> {
        output[0] = Val::from(*self as i32);
        Ok(())
    }
}

impl Lower for f32 {
    fn args_amount() -> usize {
        1
    }

    fn lower(&self, output: &mut [Val], _memory: &mut impl MemoryAccess) -> Result<()> {
        output[0] = Val::from(*self);
        Ok(())
    }
}

impl Lower for String {
    fn args_amount() -> usize {
        2
    }

    fn lower(&self, output: &mut [Val], memory: &mut impl MemoryAccess) -> Result<()> {
        let (index, bytes) = memory.allocate(self.len())?;
        bytes.copy_from_slice(self.as_bytes());

        output[0] = Val::from(index as i32);
        output[1] = Val::from(self.len() as i32);

        Ok(())
    }
}

impl Lower for () {
    fn args_amount() -> usize {
        0
    }

    fn lower(&self, _output: &mut [Val], _memory: &mut impl MemoryAccess) -> Result<()> {
        Ok(())
    }
}

impl<T0: Lower, T1: Lower> Lower for (T0, T1) {
    fn args_amount() -> usize {
        T0::args_amount() + T1::args_amount()
    }

    fn lower(&self, output: &mut [Val], memory: &mut impl MemoryAccess) -> Result<()> {
        T0::lower(&self.0, &mut output[0..T0::args_amount()], memory)?;
        T1::lower(&self.1, &mut output[T0::args_amount()..], memory)?;
        Ok(())
    }
}
