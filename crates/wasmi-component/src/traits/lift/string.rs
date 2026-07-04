use anyhow::Result;
use wasmi::Val;

use crate::{FatPtr, FlatArgs, Lift};

impl Lift for String {
    type Borrowed<'a> = &'a str;

    fn lift_args<'a>(vals: &[Val], memory: &'a [u8]) -> Result<Self::Borrowed<'a>> {
        debug_assert_eq!(vals.len(), Self::arg_count());

        dbg!(vals);

        let ptr = FatPtr::from_args(vals)?;
        let slice = ptr.try_index(memory, "String::lift")?;
        Ok(str::from_utf8(slice)?)
    }

    fn lift_bytes<'a>(bytes: &[u8], memory: &'a [u8]) -> Result<Self::Borrowed<'a>> {
        debug_assert_eq!(bytes.len(), Self::byte_size());

        let ptr = FatPtr::from_bytes(bytes)?;
        let slice = ptr.try_index(memory, "String::lift")?;
        Ok(str::from_utf8(slice)?)
    }

    fn into_owned(val: Self::Borrowed<'_>) -> Self {
        val.to_string()
    }
}
