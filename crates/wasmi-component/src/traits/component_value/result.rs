use wasmi::{Val, ValType};

use crate::{ComponentValue, ConvertError, ConvertResult, ValueType};

impl<T: ComponentValue, E: ComponentValue> ComponentValue for Result<T, E> {
    type Borrowed<'a> = Result<T::Borrowed<'a>, E::Borrowed<'a>>;

    fn value_type() -> ValueType {
        ValueType::Result(Box::new(T::value_type()), Box::new(E::value_type()))
    }

    fn arg_count() -> usize {
        1 + std::cmp::max(T::arg_count(), E::arg_count())
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

    fn lift_args<'a>(args: &[Val], memory: &'a [u8]) -> ConvertResult<Self::Borrowed<'a>> {
        debug_assert_eq!(args.len(), Self::arg_count());

        match args[0].i32() {
            Some(0) => Ok(Ok(T::lift_args(&args[1..(T::arg_count() + 1)], memory)?)),
            Some(1) => Ok(Err(E::lift_args(&args[1..(E::arg_count() + 1)], memory)?)),
            other => Err(ConvertError::new(format!(
                "Invalid determinant in Result::lift_args: {:?}",
                other
            ))),
        }
    }

    fn byte_align() -> usize {
        std::cmp::max(T::byte_align(), E::byte_align())
    }

    fn byte_size() -> usize {
        Self::byte_align() + std::cmp::max(T::byte_size(), E::byte_size())
    }

    fn lift_bytes<'a>(bytes: &[u8], memory: &'a [u8]) -> ConvertResult<Self::Borrowed<'a>> {
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
            other => Err(ConvertError::new(format!(
                "Invalid determinant in Result::lift_bytes: {:?}",
                other
            ))),
        }
    }
}

impl<T: ComponentValue> ComponentValue for Option<T> {
    type Borrowed<'a> = Option<T::Borrowed<'a>>;

    fn value_type() -> ValueType {
        ValueType::Option(Box::new(T::value_type()))
    }

    fn arg_count() -> usize {
        1 + T::arg_count()
    }

    fn arg_types() -> Vec<ValType> {
        let mut types = vec![ValType::I32];
        types.extend(T::arg_types());
        types
    }

    fn lift_args<'a>(args: &[Val], memory: &'a [u8]) -> ConvertResult<Self::Borrowed<'a>> {
        debug_assert_eq!(args.len(), Self::arg_count());

        match args[0].i32() {
            Some(0) => Ok(None),
            Some(1) => Ok(Some(T::lift_args(args, memory)?)),
            other => Err(ConvertError::new(format!(
                "Invalid determinant in Option::lift_args: {:?}",
                other
            ))),
        }
    }

    fn byte_align() -> usize {
        T::byte_align()
    }

    fn byte_size() -> usize {
        Self::byte_align() + T::byte_size()
    }

    fn lift_bytes<'a>(bytes: &[u8], memory: &'a [u8]) -> ConvertResult<Self::Borrowed<'a>> {
        debug_assert_eq!(bytes.len(), Self::byte_size());

        let offset = Self::byte_align();

        match bytes[0] {
            0 => Ok(None),
            1 => Ok(Some(T::lift_bytes(
                &bytes[offset..(offset + T::byte_size())],
                memory,
            )?)),
            other => Err(ConvertError::new(format!(
                "Invalid determinant in Option::lift_bytes: {:?}",
                other
            ))),
        }
    }
}
