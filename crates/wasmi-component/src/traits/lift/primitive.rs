use anyhow::{Context, Result};
use wasmi::Val;

use crate::{FlatArgs, Lift};

impl Lift for i32 {
    type Borrowed<'a> = Self;

    fn lift_args<'a>(vals: &[Val], _memory: &'a [u8]) -> Result<Self::Borrowed<'a>> {
        debug_assert_eq!(vals.len(), Self::arg_count());

        vals[0].i32().context("Lifting i32")
    }

    fn lift_bytes<'a>(bytes: &[u8], _memory: &'a [u8]) -> Result<Self::Borrowed<'a>> {
        debug_assert_eq!(bytes.len(), Self::byte_size());

        Ok(Self::from_le_bytes(bytes.try_into()?))
    }

    fn into_owned(val: Self::Borrowed<'_>) -> Self {
        val
    }
}

impl Lift for u32 {
    type Borrowed<'a> = Self;

    fn lift_args<'a>(vals: &[Val], _memory: &'a [u8]) -> Result<Self::Borrowed<'a>> {
        debug_assert_eq!(vals.len(), Self::arg_count());

        vals[0].i32().map(|val| val as u32).context("Lifting u32")
    }

    fn lift_bytes<'a>(bytes: &[u8], _memory: &'a [u8]) -> Result<Self::Borrowed<'a>> {
        debug_assert_eq!(bytes.len(), Self::byte_size());

        Ok(Self::from_le_bytes(bytes.try_into()?))
    }

    fn into_owned(val: Self::Borrowed<'_>) -> Self {
        val
    }
}

impl Lift for f32 {
    type Borrowed<'a> = Self;

    fn lift_args<'a>(vals: &[Val], _memory: &'a [u8]) -> Result<Self::Borrowed<'a>> {
        debug_assert_eq!(vals.len(), Self::arg_count());

        vals[0]
            .f32()
            .map(|val| val.to_float())
            .context("Lifting f32")
    }

    fn lift_bytes<'a>(bytes: &[u8], _memory: &'a [u8]) -> Result<Self::Borrowed<'a>> {
        debug_assert_eq!(bytes.len(), Self::byte_size());

        Ok(Self::from_le_bytes(bytes.try_into()?))
    }

    fn into_owned(val: Self::Borrowed<'_>) -> Self {
        val
    }
}
