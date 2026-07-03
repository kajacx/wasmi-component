use std::vec;

use crate::Lower;

impl Lower for () {
    fn params_count() -> usize {
        0
    }

    fn imported_result() -> Vec<wasmi::ValType> {
        vec![]
    }
}

impl<T0: Lower> Lower for (T0,) {
    fn params_count() -> usize {
        T0::params_count()
    }

    fn imported_result() -> Vec<wasmi::ValType> {
        T0::imported_result()
    }
}

impl<T0: Lower, T1: Lower> Lower for (T0, T1) {
    fn params_count() -> usize {
        T0::params_count() + T1::params_count()
    }

    fn imported_result() -> Vec<wasmi::ValType> {
        let mut result = vec![];
        result.extend(T0::imported_result());
        result.extend(T1::imported_result());
        if result.len() <= 1 { result } else { vec![] }
    }
}
