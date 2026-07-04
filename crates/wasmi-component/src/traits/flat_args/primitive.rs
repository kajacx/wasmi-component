use wasmi::ValType;

use crate::FlatArgs;

impl FlatArgs for i32 {
    fn arg_count() -> usize {
        1
    }

    fn arg_types() -> Vec<ValType> {
        vec![ValType::I32]
    }

    fn byte_align() -> usize {
        4
    }

    fn byte_size() -> usize {
        4
    }
}

impl FlatArgs for u32 {
    fn arg_count() -> usize {
        1
    }

    fn arg_types() -> Vec<ValType> {
        vec![ValType::I32]
    }

    fn byte_align() -> usize {
        4
    }

    fn byte_size() -> usize {
        4
    }
}

impl FlatArgs for f32 {
    fn arg_count() -> usize {
        1
    }

    fn arg_types() -> Vec<ValType> {
        vec![ValType::I32]
    }

    fn byte_align() -> usize {
        4
    }

    fn byte_size() -> usize {
        4
    }
}
