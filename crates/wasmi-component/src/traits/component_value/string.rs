use crate::lib_structs::LiftReader;
use crate::pointers::{PtrView, ptr_start};
use crate::{ComponentValue, ConvertError, ConvertResult, ValueType};

impl ComponentValue for String {
    type Borrowed<'a> = &'a str;

    fn value_type() -> ValueType {
        ValueType::String
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
        let ptr = reader.read_fat_ptr(1);
        let memory = reader.memory();
        let slice = ptr.try_index(memory)?;

        str::from_utf8(slice).map_err(|err| {
            ConvertError::new(format!("string isn't valid utf-8"))
                .with_additional(format!(
                    "byte contents: {:?}",
                    PtrView::new(slice, ptr_start(memory))
                ))
                .with_cause(Box::new(err))
        })
    }
}
