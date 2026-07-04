use wasmi::ValType;

use crate::FlatArgs;

impl FlatArgs for String {
    fn arg_count() -> usize {
        2
    }

    fn arg_types() -> Vec<ValType> {
        vec![ValType::I32, ValType::I32]
    }

    fn byte_align() -> usize {
        4
    }

    fn byte_size() -> usize {
        8
    }
}
