use crate::lib_structs::LiftReader;
use crate::{ComponentValue, ConvertError, ConvertResult, ListAccessor, ValueType};

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
        if ptr.count > 0 && ptr.start % T::byte_align() != 0 {
            return Err(
                ConvertError::new("wrong alignment for a list pointer").with_additional(format!(
                    "pointer of type {} expected align of {}, but got start position {}",
                    std::any::type_name::<T>(),
                    T::byte_align(),
                    ptr.start
                )),
            );
        }

        let memory = reader.memory();
        Ok(ListAccessor::new(ptr.try_index(memory)?, ptr.count, memory))
    }
}
