use wasmi::ValType;

use crate::{ComponentValue, ConvertResult, FatPtr, ListAccessor, ValueType, WasmValue};

impl<T: ComponentValue> ComponentValue for Vec<T> {
    type Borrowed<'a> = ListAccessor<'a, T>;

    fn value_type() -> ValueType {
        ValueType::List(Box::new(T::value_type()))
    }

    fn arg_count() -> usize {
        2
    }

    fn arg_types() -> Vec<ValType> {
        vec![ValType::I32, ValType::I32]
    }

    fn lift_args<'a>(args: &[WasmValue], memory: &'a [u8]) -> ConvertResult<Self::Borrowed<'a>> {
        let ptr = FatPtr::from_args(args, T::byte_size())?;
        Ok(ListAccessor::new(ptr.try_index(memory)?, ptr.count, memory))
    }

    fn byte_align() -> usize {
        4
    }

    fn byte_size() -> usize {
        8
    }

    fn lift_bytes<'a>(bytes: &[u8], memory: &'a [u8]) -> ConvertResult<Self::Borrowed<'a>> {
        let ptr = FatPtr::from_bytes(bytes, T::byte_size())?;
        Ok(ListAccessor::new(ptr.try_index(memory)?, ptr.count, memory))
    }
}
