use std::ops::Range;

use crate::lib_structs::{MemoryAccess, WasmValue};
use crate::{ComponentValue, ConvertResult};

mod list;
mod option;
mod primitive;
mod string;
mod tuple;

#[blanket::blanket(derive(Ref, Mut, Box, Rc, Arc, Cow))]
pub trait Lower<T: ComponentValue> {
    fn lower_args(
        &self,
        args: &mut [WasmValue],
        memory: &mut impl MemoryAccess,
    ) -> ConvertResult<()>;

    fn lower_bytes(&self, range: Range<usize>, memory: &mut impl MemoryAccess)
    -> ConvertResult<()>;
}
