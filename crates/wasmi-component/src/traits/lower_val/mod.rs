use std::ops::Range;

use wasmi::Val;

use crate::{ComponentValue, ConvertResult, MemoryAccess};

mod list;
mod primitive;
mod result;
mod string;
mod tuple;

pub trait LowerVal<T: ComponentValue> {
    fn lower_args(&self, args: &mut [Val], memory: &mut impl MemoryAccess) -> ConvertResult<()>;

    fn lower_bytes(&self, range: Range<usize>, memory: &mut impl MemoryAccess)
    -> ConvertResult<()>;
}
