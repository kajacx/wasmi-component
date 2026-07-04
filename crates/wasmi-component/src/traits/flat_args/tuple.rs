use std::cmp::max;

use wasmi::ValType;

use crate::{FlatArgs, round_up};

impl FlatArgs for () {
    fn arg_count() -> usize {
        0
    }

    fn arg_types() -> Vec<ValType> {
        vec![]
    }

    fn byte_align() -> usize {
        1
    }

    fn byte_size() -> usize {
        0
    }
}

impl<T: FlatArgs> FlatArgs for (T,) {
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

impl<T0: FlatArgs, T1: FlatArgs> FlatArgs for (T0, T1) {
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
