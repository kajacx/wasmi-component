use anyhow::Result;
use wasmi::Val;

use crate::{Lower, LowerVal, MemoryAccess};

impl LowerVal for () {
    type Target = Self;

    fn lower(&self, _output: &mut [Val], _memory: &mut impl MemoryAccess) -> Result<()> {
        Ok(())
    }
}

impl<T0: LowerVal> LowerVal for (T0,) {
    type Target = (T0::Target,);

    fn lower(&self, output: &mut [Val], memory: &mut impl MemoryAccess) -> Result<()> {
        T0::lower(&self.0, output, memory)
    }
}

impl<T0: LowerVal, T1: LowerVal> LowerVal for (T0, T1) {
    type Target = (T0::Target, T1::Target);

    fn lower(&self, output: &mut [Val], memory: &mut impl MemoryAccess) -> Result<()> {
        T0::lower(&self.0, &mut output[0..T0::Target::params_count()], memory)?;
        T1::lower(&self.1, &mut output[T0::Target::params_count()..], memory)?;
        Ok(())
    }
}
