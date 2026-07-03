use crate::Lower;

impl Lower for i32 {
    fn params_count() -> usize {
        1
    }
}

impl Lower for u32 {
    fn params_count() -> usize {
        1
    }
}

impl Lower for f32 {
    fn params_count() -> usize {
        1
    }
}
