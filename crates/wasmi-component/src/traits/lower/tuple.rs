use crate::Lower;

impl Lower for () {
    fn params_count() -> usize {
        0
    }
}

impl<T0: Lower> Lower for (T0,) {
    fn params_count() -> usize {
        T0::params_count()
    }
}

impl<T0: Lower, T1: Lower> Lower for (T0, T1) {
    fn params_count() -> usize {
        T0::params_count() + T1::params_count()
    }
}
