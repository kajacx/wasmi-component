use std::cmp::max;

use anyhow::Result;
use wasmi::{Val, ValType};

use crate::{CompValue, round_up};

impl CompValue for () {
    type Borrowed<'a> = Self;

    fn arg_count() -> usize {
        0
    }

    fn arg_types() -> Vec<ValType> {
        vec![]
    }

    fn lift_args<'a>(vals: &[Val], _memory: &'a [u8]) -> Result<Self::Borrowed<'a>> {
        debug_assert_eq!(vals.len(), Self::arg_count());

        Ok(())
    }

    fn byte_align() -> usize {
        1
    }

    fn byte_size() -> usize {
        0
    }

    fn lift_bytes<'a>(bytes: &[u8], _memory: &'a [u8]) -> Result<Self::Borrowed<'a>> {
        debug_assert_eq!(bytes.len(), Self::byte_size());

        Ok(())
    }

    fn into_owned(_val: Self::Borrowed<'_>) -> Self {
        ()
    }
}

impl<T: CompValue> CompValue for (T,) {
    type Borrowed<'a> = (T::Borrowed<'a>,);

    fn lift_args<'a>(vals: &[Val], memory: &'a [u8]) -> Result<Self::Borrowed<'a>> {
        debug_assert_eq!(vals.len(), Self::arg_count());

        Ok((T::lift_args(vals, memory)?,))
    }

    fn lift_bytes<'a>(bytes: &[u8], memory: &'a [u8]) -> Result<Self::Borrowed<'a>> {
        debug_assert_eq!(bytes.len(), Self::byte_size());

        Ok((T::lift_bytes(bytes, memory)?,))
    }

    fn into_owned(val: Self::Borrowed<'_>) -> Self {
        (T::into_owned(val.0),)
    }
    fn arg_count() -> usize {
        T::arg_count()
    }

    fn arg_types() -> Vec<ValType> {
        T::arg_types()
    }

    fn byte_align() -> usize {
        T::byte_align()
    }

    fn byte_size() -> usize {
        T::byte_size()
    }
}

impl<T0: CompValue, T1: CompValue> CompValue for (T0, T1) {
    type Borrowed<'a> = (T0::Borrowed<'a>, T1::Borrowed<'a>);

    fn lift_args<'a>(vals: &[Val], memory: &'a [u8]) -> Result<Self::Borrowed<'a>> {
        debug_assert_eq!(vals.len(), Self::arg_count());

        let mut index = 0;

        let val0 = T0::lift_args(&vals[index..(index + T0::arg_count())], memory)?;
        index += T0::arg_count();

        let val1 = T1::lift_args(&vals[index..(index + T1::arg_count())], memory)?;
        index += T1::arg_count();

        debug_assert_eq!(index, Self::arg_count());

        Ok((val0, val1))
    }

    fn lift_bytes<'a>(bytes: &[u8], memory: &'a [u8]) -> Result<Self::Borrowed<'a>> {
        debug_assert_eq!(bytes.len(), Self::byte_size());

        let align = Self::byte_align();
        let mut index = 0;

        let val0 = T0::lift_bytes(&bytes[index..(index + T0::byte_size())], memory)?;
        index += round_up(T0::byte_size(), align);

        let val1 = T1::lift_bytes(&bytes[index..(index + T1::byte_size())], memory)?;
        index += round_up(T1::byte_size(), align);

        debug_assert_eq!(index, Self::byte_size());

        Ok((val0, val1))
    }

    fn into_owned(val: Self::Borrowed<'_>) -> Self {
        (T0::into_owned(val.0), T1::into_owned(val.1))
    }

    fn arg_count() -> usize {
        T0::arg_count() + T1::arg_count()
    }

    fn arg_types() -> Vec<ValType> {
        let mut params = vec![];
        params.extend(T0::arg_types());
        params.extend(T1::arg_types());
        params
    }

    fn byte_align() -> usize {
        max(T0::byte_align(), T1::byte_size())
    }

    fn byte_size() -> usize {
        let align = Self::byte_align();
        round_up(T0::byte_size(), align) + round_up(T1::byte_size(), align)
    }
}
