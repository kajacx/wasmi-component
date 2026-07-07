use std::cmp::max;

use anyhow::{Result, bail};
use wasmi::{Val, ValType};

use crate::{CompValue, IntoOwned};

impl<T: CompValue, E: CompValue> CompValue for Result<T, E> {
    type Borrowed<'a> = Result<T::Borrowed<'a>, E::Borrowed<'a>>;

    fn arg_count() -> usize {
        1 + max(T::arg_count(), E::arg_count())
    }

    fn arg_types() -> Vec<ValType> {
        // TODO: check this somehow
        let types = if T::arg_types().len() > E::arg_types().len() {
            T::arg_types()
        } else {
            E::arg_types()
        };

        let mut result = vec![ValType::I32];
        result.extend(types);
        result
    }

    fn lift_args<'a>(vals: &[Val], memory: &'a [u8]) -> Result<Self::Borrowed<'a>> {
        debug_assert_eq!(vals.len(), Self::arg_count());

        match vals[0].i32() {
            Some(0) => Ok(Ok(T::lift_args(&vals[1..(T::arg_count() + 1)], memory)?)),
            Some(1) => Ok(Err(E::lift_args(&vals[1..(E::arg_count() + 1)], memory)?)),
            other => bail!("Invalid determinant in Result::lift_args: {:?}", other),
        }
    }

    fn byte_align() -> usize {
        max(T::byte_align(), E::byte_align())
    }

    fn byte_size() -> usize {
        Self::byte_align() + max(T::byte_size(), E::byte_size())
    }

    fn lift_bytes<'a>(bytes: &[u8], memory: &'a [u8]) -> Result<Self::Borrowed<'a>> {
        debug_assert_eq!(bytes.len(), Self::byte_size());

        let offset = Self::byte_align();

        match bytes[0] {
            0 => Ok(Ok(T::lift_bytes(
                &bytes[offset..(T::byte_size() + offset)],
                memory,
            )?)),
            1 => Ok(Err(E::lift_bytes(
                &bytes[offset..(E::arg_count() + offset)],
                memory,
            )?)),
            other => bail!("Invalid determinant in Result::lift_bytes: {other}"),
        }
    }
}

impl<'a, T: CompValue, E: CompValue> IntoOwned<Result<T, E>>
    for Result<T::Borrowed<'a>, E::Borrowed<'a>>
{
    fn into_owned(self) -> Result<T, E> {
        self.map(T::Borrowed::into_owned)
            .map_err(E::Borrowed::into_owned)
    }
}

impl<T: CompValue> CompValue for Option<T> {
    type Borrowed<'a> = Option<T::Borrowed<'a>>;

    fn arg_count() -> usize {
        1 + T::arg_count()
    }

    fn arg_types() -> Vec<ValType> {
        let mut types = vec![ValType::I32];
        types.extend(T::arg_types());
        types
    }

    fn lift_args<'a>(vals: &[Val], memory: &'a [u8]) -> Result<Self::Borrowed<'a>> {
        debug_assert_eq!(vals.len(), Self::arg_count());

        match vals[0].i32() {
            Some(0) => Ok(None),
            Some(1) => Ok(Some(T::lift_args(vals, memory)?)),
            other => bail!("Invalid determinant in Option::lift_args: {:?}", other),
        }
    }

    fn byte_align() -> usize {
        T::byte_align()
    }

    fn byte_size() -> usize {
        Self::byte_align() + T::byte_size()
    }

    fn lift_bytes<'a>(bytes: &[u8], memory: &'a [u8]) -> Result<Self::Borrowed<'a>> {
        debug_assert_eq!(bytes.len(), Self::byte_size());

        let offset = Self::byte_align();

        match bytes[0] {
            0 => Ok(None),
            1 => Ok(Some(T::lift_bytes(
                &bytes[offset..(offset + T::byte_size())],
                memory,
            )?)),
            other => bail!("Invalid determinant in Option::lift_bytes: {other}"),
        }
    }
}

impl<'a, T: CompValue> IntoOwned<Option<T>> for Option<T::Borrowed<'a>> {
    fn into_owned(self) -> Option<T> {
        self.map(T::Borrowed::into_owned)
    }
}
