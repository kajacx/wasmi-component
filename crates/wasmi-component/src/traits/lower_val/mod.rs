use std::ops::Range;

use anyhow::Result;
use wasmi::Val;

use crate::{ComponentValue, MemoryAccess};

mod list;
mod primitive;
mod resource;
mod result;
mod string;
mod tuple;

pub trait LowerVal<T: ComponentValue> {
    fn lower_args(&self, args: &mut [Val], memory: &mut impl MemoryAccess) -> Result<()>;

    fn lower_bytes(&self, range: Range<usize>, memory: &mut impl MemoryAccess) -> Result<()>;
}
