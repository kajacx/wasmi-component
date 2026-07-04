use anyhow::{Context, Result};
use wasmi::{Val, ValType};

use crate::{FatPtr, Lift};

impl Lift for String {
    type Borrowed<'a> = &'a str;

    fn lift<'a>(vals: &[Val], memory: &'a [u8]) -> Result<Self::Borrowed<'a>> {
        let ptr = FatPtr::from_data(memory, vals[0].i32().context("Lifting String")? as usize);

        let str_bytes = memory
            .get(ptr.as_range())
            .context("Memory access out of bounds")?;

        Ok(str::from_utf8(str_bytes)?)
    }

    fn into_owned(val: Self::Borrowed<'_>) -> Self {
        val.to_string()
    }

    fn imported_params() -> Vec<ValType> {
        vec![ValType::I32, ValType::I32]
    }
}
