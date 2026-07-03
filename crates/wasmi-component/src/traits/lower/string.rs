use crate::Lower;

impl Lower for String {
    fn params_count() -> usize {
        2
    }

    fn imported_result() -> Vec<wasmi::ValType> {
        vec![]
    }
}
