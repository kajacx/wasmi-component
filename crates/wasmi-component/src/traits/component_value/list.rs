use crate::lib_structs::LiftReader;
use crate::{ComponentValue, ConvertResult, ListAccessor, ValueType};

impl<T: ComponentValue> ComponentValue for Vec<T> {
    type Borrowed<'a> = ListAccessor<'a, T>;

    fn value_type() -> ValueType {
        ValueType::new_list(T::value_type())
    }

    fn arg_count() -> usize {
        2
    }

    fn byte_align() -> usize {
        4
    }

    fn byte_size() -> usize {
        8
    }

    fn lift<'mem>(reader: &mut impl LiftReader<'mem>) -> ConvertResult<Self::Borrowed<'mem>> {
        let ptr = reader.read_fat_ptr(T::byte_size());
        let memory = reader.memory();

        Ok(ListAccessor::new(ptr.try_index(memory)?, ptr.count, memory))
    }
}
