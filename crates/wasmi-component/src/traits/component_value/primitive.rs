use anyhow::{Context, Result, bail};
use wasmi::{Val, ValType};

use crate::ComponentValue;

macro_rules! impl_component_value_primitive {
    ($main_ty: ty, $wasmi_ty: expr, $wasmi_getter: ident ) => {
        impl ComponentValue for $main_ty {
            type Borrowed<'a> = Self;

            fn arg_count() -> usize {
                1
            }

            fn arg_types() -> Vec<ValType> {
                vec![$wasmi_ty]
            }

            fn lift_args<'a>(vals: &[Val], _memory: &'a [u8]) -> Result<Self::Borrowed<'a>> {
                debug_assert_eq!(vals.len(), Self::arg_count());

                Ok((vals[0].$wasmi_getter().context("Lifting value")? as Self))
            }

            fn byte_align() -> usize {
                std::mem::size_of::<Self>()
            }

            fn byte_size() -> usize {
                std::mem::size_of::<Self>()
            }

            fn lift_bytes<'a>(bytes: &[u8], _memory: &'a [u8]) -> Result<Self::Borrowed<'a>> {
                debug_assert_eq!(bytes.len(), Self::byte_size());

                Ok(Self::from_le_bytes(bytes.try_into()?))
            }
        }
    };
}

impl_component_value_primitive!(i8, ValType::I32, i32);
impl_component_value_primitive!(i16, ValType::I32, i32);
impl_component_value_primitive!(i32, ValType::I32, i32);
impl_component_value_primitive!(i64, ValType::I64, i64);

impl_component_value_primitive!(u8, ValType::I32, i32);
impl_component_value_primitive!(u16, ValType::I32, i32);
impl_component_value_primitive!(u32, ValType::I32, i32);
impl_component_value_primitive!(u64, ValType::I64, i64);

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
        vec![ValType::F32]
    }

    fn byte_align() -> usize {
        std::mem::size_of::<Self>()
    }

    fn byte_size() -> usize {
        std::mem::size_of::<Self>()
    }

    fn lift_bytes<'a>(bytes: &[u8], _memory: &'a [u8]) -> Result<Self::Borrowed<'a>> {
        debug_assert_eq!(bytes.len(), Self::byte_size());

        Ok(Self::from_le_bytes(bytes.try_into()?))
    }
}

impl ComponentValue for f64 {
    type Borrowed<'a> = Self;

    fn arg_count() -> usize {
        1
    }

    fn lift_args<'a>(vals: &[Val], _memory: &'a [u8]) -> Result<Self::Borrowed<'a>> {
        debug_assert_eq!(vals.len(), Self::arg_count());

        vals[0]
            .f64()
            .map(|val| val.to_float())
            .context("Lifting f64")
    }

    fn arg_types() -> Vec<ValType> {
        vec![ValType::F64]
    }

    fn byte_align() -> usize {
        std::mem::size_of::<Self>()
    }

    fn byte_size() -> usize {
        std::mem::size_of::<Self>()
    }

    fn lift_bytes<'a>(bytes: &[u8], _memory: &'a [u8]) -> Result<Self::Borrowed<'a>> {
        debug_assert_eq!(bytes.len(), Self::byte_size());

        Ok(Self::from_le_bytes(bytes.try_into()?))
    }
}

impl ComponentValue for bool {
    type Borrowed<'a> = Self;

    fn arg_count() -> usize {
        1
    }

    fn arg_types() -> Vec<ValType> {
        vec![ValType::I32]
    }

    fn lift_args<'a>(vals: &[Val], _memory: &'a [u8]) -> Result<Self::Borrowed<'a>> {
        debug_assert_eq!(vals.len(), Self::arg_count());

        match vals[0].i32().context("Lifting bool")? {
            0 => Ok(false),
            1 => Ok(true),
            other => bail!("Unexpected bool value: {other}"),
        }
    }

    fn byte_align() -> usize {
        std::mem::size_of::<Self>()
    }

    fn byte_size() -> usize {
        std::mem::size_of::<Self>()
    }

    fn lift_bytes<'a>(bytes: &[u8], _memory: &'a [u8]) -> Result<Self::Borrowed<'a>> {
        debug_assert_eq!(bytes.len(), Self::byte_size());

        match bytes[0] {
            0 => Ok(false),
            1 => Ok(true),
            other => bail!("Unexpected bool value: {other}"),
        }
    }
}

impl ComponentValue for char {
    type Borrowed<'a> = Self;

    fn arg_count() -> usize {
        1
    }

    fn arg_types() -> Vec<ValType> {
        vec![ValType::I32]
    }

    fn lift_args<'a>(vals: &[Val], _memory: &'a [u8]) -> Result<Self::Borrowed<'a>> {
        debug_assert_eq!(vals.len(), Self::arg_count());

        char::from_u32(vals[0].i32().context("Lifting char")? as u32).context("Checking char value")
    }

    fn byte_align() -> usize {
        std::mem::size_of::<Self>()
    }

    fn byte_size() -> usize {
        std::mem::size_of::<Self>()
    }

    fn lift_bytes<'a>(bytes: &[u8], _memory: &'a [u8]) -> Result<Self::Borrowed<'a>> {
        debug_assert_eq!(bytes.len(), Self::byte_size());

        char::from_u32(u32::from_le_bytes(bytes.try_into()?)).context("Checking char value")
    }
}
