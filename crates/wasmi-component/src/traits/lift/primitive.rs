use anyhow::{Context, Result};
use wasmi::{Val, ValType};

use crate::Lift;

impl Lift for i32 {
    type Borrowed<'a> = Self;

    fn lift<'a>(vals: &[Val], _memory: &'a [u8]) -> Result<Self::Borrowed<'a>> {
        vals[0].i32().context("Lifting i32")
    }

    fn into_owned(val: Self::Borrowed<'_>) -> Self {
        val
    }

    fn imported_params() -> Vec<ValType> {
        vec![ValType::I32]
    }
}

impl Lift for u32 {
    type Borrowed<'a> = Self;

    fn lift<'a>(vals: &[Val], _memory: &'a [u8]) -> Result<Self::Borrowed<'a>> {
        vals[0].i32().map(|val| val as u32).context("Lifting u32")
    }

    fn into_owned(val: Self::Borrowed<'_>) -> Self {
        val
    }

    fn imported_params() -> Vec<ValType> {
        vec![ValType::I32]
    }
}

impl Lift for f32 {
    type Borrowed<'a> = Self;

    fn lift<'a>(vals: &[Val], _memory: &'a [u8]) -> Result<Self::Borrowed<'a>> {
        vals[0]
            .f32()
            .map(|val| val.to_float())
            .context("Lifting f32")
    }

    fn into_owned(val: Self::Borrowed<'_>) -> Self {
        val
    }

    fn imported_params() -> Vec<ValType> {
        vec![ValType::F32]
    }
}
