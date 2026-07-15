use wasmi::ValType;
use wasmi_component_parser::ValueType;

use crate::{ComponentValue, ConvertError, ConvertResult, WasmValue};

macro_rules! impl_component_value_primitive {
    ($main_ty: ty, $wasmi_ty: expr, $wasmi_getter: ident , $value_type: ident) => {
        impl ComponentValue for $main_ty {
            type Borrowed<'a> = Self;

            fn value_type() -> ValueType {
                ValueType::$value_type
            }

            fn arg_count() -> usize {
                1
            }

            fn arg_types() -> Vec<ValType> {
                vec![$wasmi_ty]
            }

            fn lift_args<'a>(
                args: &[WasmValue],
                _memory: &'a [u8],
            ) -> ConvertResult<Self::Borrowed<'a>> {
                debug_assert_eq!(args.len(), Self::arg_count());

                Ok((args[0].$wasmi_getter()? as Self))
            }

            fn byte_align() -> usize {
                std::mem::size_of::<Self>()
            }

            fn byte_size() -> usize {
                std::mem::size_of::<Self>()
            }

            fn lift_bytes<'a>(
                bytes: &[u8],
                _memory: &'a [u8],
            ) -> ConvertResult<Self::Borrowed<'a>> {
                debug_assert_eq!(bytes.len(), Self::byte_size());

                Ok(Self::from_le_bytes(bytes.try_into().unwrap()))
            }
        }
    };
}

impl_component_value_primitive!(i8, ValType::I32, i32, S8);
impl_component_value_primitive!(i16, ValType::I32, i32, S16);
impl_component_value_primitive!(i32, ValType::I32, i32, S32);
impl_component_value_primitive!(i64, ValType::I64, i64, S64);

impl_component_value_primitive!(u8, ValType::I32, i32, U8);
impl_component_value_primitive!(u16, ValType::I32, i32, U16);
impl_component_value_primitive!(u32, ValType::I32, i32, U32);
impl_component_value_primitive!(u64, ValType::I64, i64, U64);

impl_component_value_primitive!(f32, ValType::F32, f32, F32);
impl_component_value_primitive!(f64, ValType::F64, f64, F64);

impl ComponentValue for bool {
    type Borrowed<'a> = Self;

    fn value_type() -> ValueType {
        ValueType::Bool
    }

    fn arg_count() -> usize {
        1
    }

    fn arg_types() -> Vec<ValType> {
        vec![ValType::I32]
    }

    fn lift_args<'a>(args: &[WasmValue], _memory: &'a [u8]) -> ConvertResult<Self::Borrowed<'a>> {
        debug_assert_eq!(args.len(), Self::arg_count());

        match args[0].i32()? {
            0 => Ok(false),
            1 => Ok(true),
            other => Err(ConvertError::new(format!("Unexpected bool value: {other}"))),
        }
    }

    fn byte_align() -> usize {
        std::mem::size_of::<Self>()
    }

    fn byte_size() -> usize {
        std::mem::size_of::<Self>()
    }

    fn lift_bytes<'a>(bytes: &[u8], _memory: &'a [u8]) -> ConvertResult<Self::Borrowed<'a>> {
        debug_assert_eq!(bytes.len(), Self::byte_size());

        match bytes[0] {
            0 => Ok(false),
            1 => Ok(true),
            other => Err(ConvertError::new(format!("Unexpected bool value: {other}"))),
        }
    }
}

impl ComponentValue for char {
    type Borrowed<'a> = Self;

    fn value_type() -> ValueType {
        ValueType::Char
    }

    fn arg_count() -> usize {
        1
    }

    fn arg_types() -> Vec<ValType> {
        vec![ValType::I32]
    }

    fn lift_args<'a>(args: &[WasmValue], _memory: &'a [u8]) -> ConvertResult<Self::Borrowed<'a>> {
        debug_assert_eq!(args.len(), Self::arg_count());

        let value = args[0].i32()? as u32;
        char::from_u32(value)
            .ok_or_else(|| ConvertError::new(format!("Invalid char value: 0x{:08x}", value)))
    }

    fn byte_align() -> usize {
        std::mem::size_of::<Self>()
    }

    fn byte_size() -> usize {
        std::mem::size_of::<Self>()
    }

    fn lift_bytes<'a>(bytes: &[u8], _memory: &'a [u8]) -> ConvertResult<Self::Borrowed<'a>> {
        debug_assert_eq!(bytes.len(), Self::byte_size());

        let value = u32::from_le_bytes(bytes.try_into().unwrap());
        char::from_u32(value)
            .ok_or_else(|| ConvertError::new(format!("Invalid char value: 0x{:08x}", value)))
    }
}
