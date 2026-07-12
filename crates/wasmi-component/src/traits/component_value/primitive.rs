use wasmi::{Val, ValType};

use crate::{ComponentValue, ConvertError, ConvertResult, ValueType};

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

            fn lift_args<'a>(args: &[Val], _memory: &'a [u8]) -> ConvertResult<Self::Borrowed<'a>> {
                debug_assert_eq!(args.len(), Self::arg_count());

                // TODO: Again: check types with wasmi
                Ok((args[0].$wasmi_getter().unwrap() as Self))
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

impl ComponentValue for f32 {
    type Borrowed<'a> = Self;

    fn value_type() -> ValueType {
        ValueType::F32
    }

    fn arg_count() -> usize {
        1
    }

    fn arg_types() -> Vec<ValType> {
        vec![ValType::F32]
    }

    fn lift_args<'a>(args: &[Val], _memory: &'a [u8]) -> ConvertResult<Self::Borrowed<'a>> {
        debug_assert_eq!(args.len(), Self::arg_count());

        // TODO: result or other variants
        Ok(args[0].f32().map(|val| val.to_float()).unwrap())
    }

    fn byte_align() -> usize {
        std::mem::size_of::<Self>()
    }

    fn byte_size() -> usize {
        std::mem::size_of::<Self>()
    }

    fn lift_bytes<'a>(bytes: &[u8], _memory: &'a [u8]) -> ConvertResult<Self::Borrowed<'a>> {
        debug_assert_eq!(bytes.len(), Self::byte_size());

        Ok(Self::from_le_bytes(bytes.try_into().unwrap()))
    }
}

impl ComponentValue for f64 {
    type Borrowed<'a> = Self;

    fn value_type() -> ValueType {
        ValueType::F64
    }

    fn arg_count() -> usize {
        1
    }

    fn arg_types() -> Vec<ValType> {
        vec![ValType::F64]
    }

    fn lift_args<'a>(args: &[Val], _memory: &'a [u8]) -> ConvertResult<Self::Borrowed<'a>> {
        debug_assert_eq!(args.len(), Self::arg_count());

        // TODO: result or other variants
        Ok(args[0].f64().map(|val| val.to_float()).unwrap())
    }

    fn byte_align() -> usize {
        std::mem::size_of::<Self>()
    }

    fn byte_size() -> usize {
        std::mem::size_of::<Self>()
    }

    fn lift_bytes<'a>(bytes: &[u8], _memory: &'a [u8]) -> ConvertResult<Self::Borrowed<'a>> {
        debug_assert_eq!(bytes.len(), Self::byte_size());

        Ok(Self::from_le_bytes(bytes.try_into().unwrap()))
    }
}

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

    fn lift_args<'a>(args: &[Val], _memory: &'a [u8]) -> ConvertResult<Self::Borrowed<'a>> {
        debug_assert_eq!(args.len(), Self::arg_count());

        // TODO: result / variant
        match args[0].i32().unwrap() {
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

    fn lift_args<'a>(args: &[Val], _memory: &'a [u8]) -> ConvertResult<Self::Borrowed<'a>> {
        debug_assert_eq!(args.len(), Self::arg_count());

        // TODO: result / variant
        let value = args[0].i32().unwrap() as u32;
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
