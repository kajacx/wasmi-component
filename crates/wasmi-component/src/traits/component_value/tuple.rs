use wasmi::ValType;
use wasmi_component_parser::ValueType;

use crate::{ComponentValue, ConvertResult, WasmValue, helpers::round_up};

impl ComponentValue for () {
    type Borrowed<'a> = Self;

    fn value_type() -> ValueType {
        ValueType::Tuple(vec![])
    }

    fn arg_count() -> usize {
        0
    }

    fn arg_types() -> Vec<ValType> {
        vec![]
    }

    fn lift_args<'a>(args: &[WasmValue], _memory: &'a [u8]) -> ConvertResult<Self::Borrowed<'a>> {
        debug_assert_eq!(args.len(), Self::arg_count());

        Ok(())
    }

    fn byte_align() -> usize {
        1
    }

    fn byte_size() -> usize {
        0
    }

    fn lift_bytes<'a>(bytes: &[u8], _memory: &'a [u8]) -> ConvertResult<Self::Borrowed<'a>> {
        debug_assert_eq!(bytes.len(), Self::byte_size());

        Ok(())
    }
}

impl<T: ComponentValue> ComponentValue for (T,) {
    type Borrowed<'a> = (T::Borrowed<'a>,);

    fn value_type() -> ValueType {
        ValueType::Tuple(vec![T::value_type()])
    }

    fn arg_count() -> usize {
        T::arg_count()
    }

    fn arg_types() -> Vec<ValType> {
        T::arg_types()
    }

    fn lift_args<'a>(args: &[WasmValue], memory: &'a [u8]) -> ConvertResult<Self::Borrowed<'a>> {
        debug_assert_eq!(args.len(), Self::arg_count());

        Ok((T::lift_args(args, memory)?,))
    }

    fn byte_align() -> usize {
        T::byte_align()
    }

    fn byte_size() -> usize {
        T::byte_size()
    }

    fn lift_bytes<'a>(bytes: &[u8], memory: &'a [u8]) -> ConvertResult<Self::Borrowed<'a>> {
        debug_assert_eq!(bytes.len(), Self::byte_size());

        Ok((T::lift_bytes(bytes, memory)?,))
    }
}

impl<T0: ComponentValue, T1: ComponentValue> ComponentValue for (T0, T1) {
    type Borrowed<'a> = (T0::Borrowed<'a>, T1::Borrowed<'a>);

    fn value_type() -> ValueType {
        ValueType::Tuple(vec![T0::value_type(), T1::value_type()])
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

    fn lift_args<'a>(args: &[WasmValue], memory: &'a [u8]) -> ConvertResult<Self::Borrowed<'a>> {
        debug_assert_eq!(args.len(), Self::arg_count());

        let mut index = 0;

        let val0 = T0::lift_args(&args[index..(index + T0::arg_count())], memory)?;
        index += T0::arg_count();

        let val1 = T1::lift_args(&args[index..(index + T1::arg_count())], memory)?;
        index += T1::arg_count();

        debug_assert_eq!(index, Self::arg_count());

        Ok((val0, val1))
    }

    fn byte_align() -> usize {
        std::cmp::max(T0::byte_align(), T1::byte_size())
    }

    fn byte_size() -> usize {
        let align = Self::byte_align();
        round_up(T0::byte_size(), align) + round_up(T1::byte_size(), align)
    }

    fn lift_bytes<'a>(bytes: &[u8], memory: &'a [u8]) -> ConvertResult<Self::Borrowed<'a>> {
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
}

impl<T0: ComponentValue, T1: ComponentValue, T2: ComponentValue> ComponentValue for (T0, T1, T2) {
    type Borrowed<'a> = (T0::Borrowed<'a>, T1::Borrowed<'a>, T2::Borrowed<'a>);

    fn value_type() -> ValueType {
        ValueType::Tuple(vec![T0::value_type(), T1::value_type(), T2::value_type()])
    }

    fn arg_count() -> usize {
        T0::arg_count() + T1::arg_count() + T2::arg_count()
    }

    fn arg_types() -> Vec<ValType> {
        let mut params = vec![];
        params.extend(T0::arg_types());
        params.extend(T1::arg_types());
        params.extend(T2::arg_types());
        params
    }

    fn lift_args<'a>(args: &[WasmValue], memory: &'a [u8]) -> ConvertResult<Self::Borrowed<'a>> {
        debug_assert_eq!(args.len(), Self::arg_count());

        let mut index = 0;

        let val0 = T0::lift_args(&args[index..(index + T0::arg_count())], memory)?;
        index += T0::arg_count();

        let val1 = T1::lift_args(&args[index..(index + T1::arg_count())], memory)?;
        index += T1::arg_count();

        let val2 = T2::lift_args(&args[index..(index + T2::arg_count())], memory)?;
        index += T2::arg_count();

        debug_assert_eq!(index, Self::arg_count());

        Ok((val0, val1, val2))
    }

    fn byte_align() -> usize {
        let mut max = 0;
        max = std::cmp::max(max, T0::byte_align());
        max = std::cmp::max(max, T1::byte_align());
        max = std::cmp::max(max, T2::byte_align());
        max
    }

    fn byte_size() -> usize {
        let align = Self::byte_align();
        round_up(T0::byte_size(), align)
            + round_up(T1::byte_size(), align)
            + round_up(T2::byte_size(), align)
    }

    fn lift_bytes<'a>(bytes: &[u8], memory: &'a [u8]) -> ConvertResult<Self::Borrowed<'a>> {
        debug_assert_eq!(bytes.len(), Self::byte_size());

        let align = Self::byte_align();
        let mut index = 0;

        let val0 = T0::lift_bytes(&bytes[index..(index + T0::byte_size())], memory)?;
        index += round_up(T0::byte_size(), align);

        let val1 = T1::lift_bytes(&bytes[index..(index + T1::byte_size())], memory)?;
        index += round_up(T1::byte_size(), align);

        let val2 = T2::lift_bytes(&bytes[index..(index + T2::byte_size())], memory)?;
        index += round_up(T2::byte_size(), align);

        debug_assert_eq!(index, Self::byte_size());

        Ok((val0, val1, val2))
    }
}
