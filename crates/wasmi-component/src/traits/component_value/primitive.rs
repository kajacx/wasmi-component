use anyhow::{Context, Result};
use wasmi::{Val, ValType};

use crate::ComponentValue;

impl ComponentValue for i32 {
    type Borrowed<'a> = Self;

    fn arg_count() -> usize {
        1
    }

    fn arg_types() -> Vec<ValType> {
        vec![ValType::I32]
    }

    fn lift_args<'a>(vals: &[Val], _memory: &'a [u8]) -> Result<Self::Borrowed<'a>> {
        debug_assert_eq!(vals.len(), Self::arg_count());

        vals[0].i32().context("Lifting i32")
    }

    fn byte_align() -> usize {
        4
    }

    fn byte_size() -> usize {
        4
    }

    fn lift_bytes<'a>(bytes: &[u8], _memory: &'a [u8]) -> Result<Self::Borrowed<'a>> {
        debug_assert_eq!(bytes.len(), Self::byte_size());

        Ok(Self::from_le_bytes(bytes.try_into()?))
    }
}

impl ComponentValue for u8 {
    type Borrowed<'a> = Self;

    fn arg_count() -> usize {
        1
    }

    fn arg_types() -> Vec<ValType> {
        vec![ValType::I32]
    }

    fn lift_args<'a>(vals: &[Val], _memory: &'a [u8]) -> Result<Self::Borrowed<'a>> {
        debug_assert_eq!(vals.len(), Self::arg_count());

        vals[0].i32().map(|val| val as u8).context("Lifting u8")
    }

    fn byte_align() -> usize {
        1
    }

    fn byte_size() -> usize {
        1
    }

    fn lift_bytes<'a>(bytes: &[u8], _memory: &'a [u8]) -> Result<Self::Borrowed<'a>> {
        debug_assert_eq!(bytes.len(), Self::byte_size());

        Ok(Self::from_le_bytes(bytes.try_into()?))
    }
}

impl ComponentValue for u32 {
    type Borrowed<'a> = Self;

    fn arg_count() -> usize {
        1
    }

    fn arg_types() -> Vec<ValType> {
        vec![ValType::I32]
    }

    fn lift_args<'a>(vals: &[Val], _memory: &'a [u8]) -> Result<Self::Borrowed<'a>> {
        debug_assert_eq!(vals.len(), Self::arg_count());

        vals[0].i32().map(|val| val as u32).context("Lifting u32")
    }

    fn byte_align() -> usize {
        4
    }

    fn byte_size() -> usize {
        4
    }

    fn lift_bytes<'a>(bytes: &[u8], _memory: &'a [u8]) -> Result<Self::Borrowed<'a>> {
        debug_assert_eq!(bytes.len(), Self::byte_size());

        Ok(Self::from_le_bytes(bytes.try_into()?))
    }
}

impl ComponentValue for u64 {
    type Borrowed<'a> = Self;

    fn arg_count() -> usize {
        1
    }

    fn arg_types() -> Vec<ValType> {
        vec![ValType::I64]
    }

    fn lift_args<'a>(vals: &[Val], _memory: &'a [u8]) -> Result<Self::Borrowed<'a>> {
        debug_assert_eq!(vals.len(), Self::arg_count());

        vals[0].i64().map(|val| val as u64).context("Lifting u64")
    }

    fn byte_align() -> usize {
        8
    }

    fn byte_size() -> usize {
        8
    }

    fn lift_bytes<'a>(bytes: &[u8], _memory: &'a [u8]) -> Result<Self::Borrowed<'a>> {
        debug_assert_eq!(bytes.len(), Self::byte_size());

        Ok(Self::from_le_bytes(bytes.try_into()?))
    }
}

impl ComponentValue for f32 {
    type Borrowed<'a> = Self;

    fn arg_count() -> usize {
        1
    }

    fn lift_args<'a>(vals: &[Val], _memory: &'a [u8]) -> Result<Self::Borrowed<'a>> {
        debug_assert_eq!(vals.len(), Self::arg_count());

        vals[0]
            .f32()
            .map(|val| val.to_float())
            .context("Lifting f32")
    }

    fn arg_types() -> Vec<ValType> {
        vec![ValType::I32]
    }

    fn byte_align() -> usize {
        4
    }

    fn byte_size() -> usize {
        4
    }

    fn lift_bytes<'a>(bytes: &[u8], _memory: &'a [u8]) -> Result<Self::Borrowed<'a>> {
        debug_assert_eq!(bytes.len(), Self::byte_size());

        Ok(Self::from_le_bytes(bytes.try_into()?))
    }
}
