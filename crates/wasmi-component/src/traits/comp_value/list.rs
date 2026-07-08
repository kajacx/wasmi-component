use anyhow::Result;
use wasmi::{Val, ValType};

use crate::{CompValue, FatPtr, ListAccessor};

impl<T: CompValue> CompValue for Vec<T> {
    type Borrowed<'a> = ListAccessor<'a, T>;

    fn arg_count() -> usize {
        2
    }

    fn arg_types() -> Vec<ValType> {
        vec![ValType::I32, ValType::I32]
    }

    fn lift_args<'a>(vals: &[Val], memory: &'a [u8]) -> Result<Self::Borrowed<'a>> {
        let ptr = FatPtr::from_args(vals, T::byte_size())?;
        Ok(ListAccessor::new(ptr.try_index(memory)?, ptr.count, memory))
    }

    fn byte_align() -> usize {
        4
    }

    fn byte_size() -> usize {
        8
    }

    fn lift_bytes<'a>(bytes: &[u8], memory: &'a [u8]) -> Result<Self::Borrowed<'a>> {
        let ptr = FatPtr::from_bytes(bytes, T::byte_size())?;
        Ok(ListAccessor::new(ptr.try_index(memory)?, ptr.count, memory))
    }
}
