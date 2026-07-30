use std::rc::Rc;

use crate::lib_structs::LiftReader;
use crate::{ComponentValue, ConvertResult, ValueType, helpers::round_up};

impl ComponentValue for () {
    type Borrowed<'a> = Self;

    fn value_type() -> ValueType {
        ValueType::Tuple(Rc::from([]))
    }

    fn arg_count() -> usize {
        0
    }

    fn byte_align() -> usize {
        1
    }

    fn byte_size() -> usize {
        0
    }

    fn lift<'mem>(_reader: &mut impl LiftReader<'mem>) -> ConvertResult<Self::Borrowed<'mem>> {
        Ok(())
    }
}

impl<T: ComponentValue> ComponentValue for (T,) {
    type Borrowed<'a> = (T::Borrowed<'a>,);

    fn value_type() -> ValueType {
        ValueType::Tuple(Rc::from([T::value_type()]))
    }

    fn arg_count() -> usize {
        T::arg_count()
    }

    fn byte_align() -> usize {
        T::byte_align()
    }

    fn byte_size() -> usize {
        T::byte_size()
    }

    fn lift<'mem>(reader: &mut impl LiftReader<'mem>) -> ConvertResult<Self::Borrowed<'mem>> {
        Ok((T::lift(reader)?,))
    }
}

impl<T0: ComponentValue, T1: ComponentValue> ComponentValue for (T0, T1) {
    type Borrowed<'a> = (T0::Borrowed<'a>, T1::Borrowed<'a>);

    fn value_type() -> ValueType {
        ValueType::Tuple(Rc::from([T0::value_type(), T1::value_type()]))
    }

    fn arg_count() -> usize {
        T0::arg_count() + T1::arg_count()
    }

    fn byte_align() -> usize {
        std::cmp::max(T0::byte_align(), T1::byte_size())
    }

    fn byte_size() -> usize {
        let align = Self::byte_align();
        round_up(T0::byte_size(), align) + round_up(T1::byte_size(), align)
    }

    fn lift<'mem>(reader: &mut impl LiftReader<'mem>) -> ConvertResult<Self::Borrowed<'mem>> {
        let align = Self::byte_align();
        Ok((
            reader.read_record_field::<T0>(align)?,
            reader.read_record_field::<T1>(align)?,
        ))
    }
}

impl<T0: ComponentValue, T1: ComponentValue, T2: ComponentValue> ComponentValue for (T0, T1, T2) {
    type Borrowed<'a> = (T0::Borrowed<'a>, T1::Borrowed<'a>, T2::Borrowed<'a>);

    fn value_type() -> ValueType {
        ValueType::Tuple(Rc::from([
            T0::value_type(),
            T1::value_type(),
            T2::value_type(),
        ]))
    }

    fn arg_count() -> usize {
        T0::arg_count() + T1::arg_count() + T2::arg_count()
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

    fn lift<'mem>(reader: &mut impl LiftReader<'mem>) -> ConvertResult<Self::Borrowed<'mem>> {
        let align = Self::byte_align();
        Ok((
            reader.read_record_field::<T0>(align)?,
            reader.read_record_field::<T1>(align)?,
            reader.read_record_field::<T2>(align)?,
        ))
    }
}
