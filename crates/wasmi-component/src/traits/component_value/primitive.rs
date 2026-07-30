use crate::lib_structs::LiftReader;
use crate::{ComponentValue, ConvertError, ConvertResult, ValueType};

macro_rules! impl_component_value_primitive {
    ($main_ty: ty, $value_type: ident, $accessor_fn: ident) => {
        impl ComponentValue for $main_ty {
            type Borrowed<'a> = Self;

            fn value_type() -> ValueType {
                ValueType::$value_type
            }

            fn arg_count() -> usize {
                1
            }

            fn byte_align() -> usize {
                std::mem::size_of::<Self>()
            }

            fn byte_size() -> usize {
                std::mem::size_of::<Self>()
            }

            fn lift<'mem>(reader: &mut impl LiftReader<'mem>) -> ConvertResult<Self> {
                Ok(reader.$accessor_fn() as _)
            }
        }
    };
}

impl_component_value_primitive!(i8, S8, read_u8);
impl_component_value_primitive!(i16, S16, read_u16);
impl_component_value_primitive!(i32, S32, read_u32);
impl_component_value_primitive!(i64, S64, read_u64);

impl_component_value_primitive!(u8, U8, read_u8);
impl_component_value_primitive!(u16, U16, read_u16);
impl_component_value_primitive!(u32, U32, read_u32);
impl_component_value_primitive!(u64, U64, read_u64);

impl_component_value_primitive!(f32, F32, read_f32);
impl_component_value_primitive!(f64, F64, read_f64);

impl ComponentValue for bool {
    type Borrowed<'a> = Self;

    fn value_type() -> ValueType {
        ValueType::Bool
    }

    fn arg_count() -> usize {
        1
    }

    fn byte_align() -> usize {
        std::mem::size_of::<Self>()
    }

    fn byte_size() -> usize {
        std::mem::size_of::<Self>()
    }

    fn lift<'mem>(reader: &mut impl LiftReader<'mem>) -> ConvertResult<Self> {
        match reader.read_u8() {
            0 => Ok(false),
            1 => Ok(true),
            other => Err(ConvertError::new(format!("unexpected bool value {other}"))),
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

    fn byte_align() -> usize {
        std::mem::size_of::<Self>()
    }

    fn byte_size() -> usize {
        std::mem::size_of::<Self>()
    }

    fn lift<'mem>(reader: &mut impl LiftReader<'mem>) -> ConvertResult<Self> {
        let value = reader.read_u32();
        char::from_u32(value)
            .ok_or_else(|| ConvertError::new(format!("invalid char value 0x{:08x}", value)))
    }
}
