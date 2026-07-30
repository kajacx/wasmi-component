use crate::lib_structs::LiftReader;
use crate::{ComponentValue, ConvertError, ConvertResult, ValueType};

impl<T: ComponentValue> ComponentValue for Option<T> {
    type Borrowed<'a> = Option<T::Borrowed<'a>>;

    fn value_type() -> ValueType {
        ValueType::new_option(T::value_type())
    }

    fn arg_count() -> usize {
        1 + T::arg_count()
    }

    fn byte_align() -> usize {
        T::byte_align()
    }

    fn byte_size() -> usize {
        Self::byte_align() + T::byte_size()
    }

    fn lift<'mem>(reader: &mut impl LiftReader<'mem>) -> ConvertResult<Self::Borrowed<'mem>> {
        reader.read_variant::<Self>(|reader, determinant| match determinant {
            0 => Ok(None),
            1 => Ok(Some(T::lift(reader)?)),
            other => Err(ConvertError::new(format!(
                "invalid determinant {other} in Option::lift"
            ))),
        })
    }
}

impl<T: ComponentValue, E: ComponentValue> ComponentValue for Result<T, E> {
    type Borrowed<'a> = Result<T::Borrowed<'a>, E::Borrowed<'a>>;

    fn value_type() -> ValueType {
        ValueType::new_result(T::value_type(), E::value_type())
    }

    fn arg_count() -> usize {
        1 + std::cmp::max(T::arg_count(), E::arg_count())
    }

    fn byte_align() -> usize {
        std::cmp::max(T::byte_align(), E::byte_align())
    }

    fn byte_size() -> usize {
        Self::byte_align() + std::cmp::max(T::byte_size(), E::byte_size())
    }

    fn lift<'mem>(reader: &mut impl LiftReader<'mem>) -> ConvertResult<Self::Borrowed<'mem>> {
        reader.read_variant::<Self>(|reader, determinant| match determinant {
            0 => Ok(Ok(T::lift(reader)?)),
            1 => Ok(Err(E::lift(reader)?)),
            other => Err(ConvertError::new(format!(
                "invalid determinant {other} in Result::lift"
            ))),
        })
    }
}
