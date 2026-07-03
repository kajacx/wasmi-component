use anyhow::Result;
use wasmi::Val;

use crate::{Lower, MemoryAccess};

mod primitive;
mod string;
mod tuple;

pub trait LowerVal {
    type Target: Lower;

    fn lower(&self, output: &mut [Val], memory: &mut impl MemoryAccess) -> Result<()>;
}
