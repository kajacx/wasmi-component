use wasmi::ValType;

use crate::Lower;

impl Lower for i32 {
    fn params_count() -> usize {
        1
    }

    fn imported_result() -> Vec<wasmi::ValType> {
        vec![ValType::I32]
    }
}

impl Lower for u32 {
    fn params_count() -> usize {
        1
    }

    fn imported_result() -> Vec<wasmi::ValType> {
        vec![ValType::I32]
    }
}

impl Lower for f32 {
    fn params_count() -> usize {
        1
    }

    fn imported_result() -> Vec<wasmi::ValType> {
        vec![ValType::F32]
    }
}
