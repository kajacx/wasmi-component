use anyhow::{Context, Result};
use wasmi::{Val, ValType};

use crate::{Borrow, CompValue, Own, Resource};

impl<T: Resource> CompValue for Borrow<T> {
    type Borrowed<'a> = Self;

    fn arg_count() -> usize {
        1
    }

    fn arg_types() -> Vec<ValType> {
        vec![ValType::I32]
    }

    fn lift_args<'a>(vals: &[Val], _memory: &'a [u8]) -> Result<Self::Borrowed<'a>> {
        debug_assert_eq!(vals.len(), Self::arg_count());

        let index = vals[0].i32().context("Lifting Borrow")? as usize;

        Ok(Borrow::new(index))
    }

    fn byte_align() -> usize {
        4
    }

    fn byte_size() -> usize {
        4
    }

    fn lift_bytes<'a>(bytes: &[u8], _memory: &'a [u8]) -> Result<Self::Borrowed<'a>> {
        debug_assert_eq!(bytes.len(), Self::byte_size());

        let index = i32::from_le_bytes(bytes.try_into()?) as usize;

        Ok(Borrow::new(index))
    }
}

impl<T: Resource> CompValue for Own<T> {
    type Borrowed<'a> = Self;

    fn arg_count() -> usize {
        1
    }

    fn arg_types() -> Vec<ValType> {
        vec![ValType::I32]
    }

    fn lift_args<'a>(vals: &[Val], _memory: &'a [u8]) -> Result<Self::Borrowed<'a>> {
        debug_assert_eq!(vals.len(), Self::arg_count());

        let index = vals[0].i32().context("Lifting Borrow")? as usize;

        Ok(Own::new(index))
    }

    fn byte_align() -> usize {
        4
    }

    fn byte_size() -> usize {
        4
    }

    fn lift_bytes<'a>(bytes: &[u8], _memory: &'a [u8]) -> Result<Self::Borrowed<'a>> {
        debug_assert_eq!(bytes.len(), Self::byte_size());

        let index = i32::from_le_bytes(bytes.try_into()?) as usize;

        Ok(Own::new(index))
    }
}
