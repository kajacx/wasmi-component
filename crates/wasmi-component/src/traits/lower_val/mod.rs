use std::ops::Range;

use anyhow::Result;
use wasmi::Val;

use crate::{Lower, MemoryAccess};

mod primitive;
mod string;
mod tuple;

pub trait LowerVal {
    type Target: Lower;

    fn lower_args(&self, output: &mut [Val], memory: &mut impl MemoryAccess) -> Result<()>;

    fn lower_bytes(&self, range: Range<usize>, memory: &mut impl MemoryAccess) -> Result<()>;
}
