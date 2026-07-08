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
    type Value<'a>;

    fn lower_args(
        value: &Self::Value<'_>,
        args: &mut [Val],
        memory: &mut impl MemoryAccess,
    ) -> Result<()>;

    fn lower_bytes(
        value: &Self::Value<'_>,
        range: Range<usize>,
        memory: &mut impl MemoryAccess,
    ) -> Result<()>;
}
