use anyhow::Result;
use wasmi::{Val, ValType};

use crate::{CompValue, FatPtr};

impl CompValue for String {
    type Borrowed<'a> = &'a str;

    fn arg_count() -> usize {
        2
    }

    fn arg_types() -> Vec<ValType> {
        vec![ValType::I32, ValType::I32]
    }

    fn lift_args<'a>(vals: &[Val], memory: &'a [u8]) -> Result<Self::Borrowed<'a>> {
        debug_assert_eq!(vals.len(), Self::arg_count());

        let ptr = FatPtr::from_args(vals)?;
        let slice = ptr.try_index(memory, "String::lift_args")?;

        Ok(str::from_utf8(slice)?)
    }

    fn byte_align() -> usize {
        4
    }

    fn byte_size() -> usize {
        8
    }

    fn lift_bytes<'a>(bytes: &[u8], memory: &'a [u8]) -> Result<Self::Borrowed<'a>> {
        debug_assert_eq!(bytes.len(), Self::byte_size());

        let ptr = FatPtr::from_bytes(bytes)?;
        let slice = ptr.try_index(memory, "String::lift_bytes")?;

        Ok(str::from_utf8(slice)?)
    }

    fn into_owned(val: Self::Borrowed<'_>) -> Self {
        val.to_string()
    }
}
