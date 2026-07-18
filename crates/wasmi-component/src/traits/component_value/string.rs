use wasmi::ValType;
use wasmi_component_parser::ValueType;

use crate::lib_structs::WasmValue;
use crate::pointers::{FatPtr, PtrView, ptr_start};
use crate::{ComponentValue, ConvertError, ConvertResult};

impl ComponentValue for String {
    type Borrowed<'a> = &'a str;

    fn value_type() -> ValueType {
        ValueType::String
    }

    fn arg_count() -> usize {
        2
    }

    fn arg_types() -> Vec<ValType> {
        vec![ValType::I32, ValType::I32]
    }

    fn lift_args<'a>(args: &[WasmValue], memory: &'a [u8]) -> ConvertResult<Self::Borrowed<'a>> {
        debug_assert_eq!(args.len(), Self::arg_count());

        let ptr = FatPtr::from_args(args, 1)?;
        convert_slice(ptr, memory)
    }

    fn byte_align() -> usize {
        4
    }

    fn byte_size() -> usize {
        8
    }

    fn lift_bytes<'a>(bytes: &[u8], memory: &'a [u8]) -> ConvertResult<Self::Borrowed<'a>> {
        debug_assert_eq!(bytes.len(), Self::byte_size());

        let ptr = FatPtr::from_bytes(bytes, 1)?;
        convert_slice(ptr, memory)
    }
}

fn convert_slice<'a>(ptr: FatPtr, memory: &'a [u8]) -> ConvertResult<&'a str> {
    let slice = ptr.try_index(memory)?;
    str::from_utf8(slice).map_err(|err| {
        ConvertError::new(format!("string isn't valid utf-8"))
            .with_additional(format!("{:?}", PtrView::new(slice, ptr_start(memory))))
            .with_cause(Box::new(err))
    })
}
