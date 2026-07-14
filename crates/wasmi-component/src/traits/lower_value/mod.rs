use std::ops::Range;

use crate::{ComponentValue, ConvertResult, MemoryAccess, WasmValue};

mod list;
mod option;
mod primitive;
mod string;
mod tuple;

#[blanket::blanket(derive(Ref, Mut, Box, Rc, Arc, Cow))]
pub trait LowerValue<T: ComponentValue> {
    fn lower_args(
        &self,
        args: &mut [WasmValue],
        memory: &mut impl MemoryAccess,
    ) -> ConvertResult<()>;

    fn lower_bytes(&self, range: Range<usize>, memory: &mut impl MemoryAccess)
    -> ConvertResult<()>;
}
