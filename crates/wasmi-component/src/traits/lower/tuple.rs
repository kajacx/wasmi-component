use crate::Lower;

impl Lower for () {}

impl<T: Lower> Lower for (T,) {}

impl<T0: Lower, T1: Lower> Lower for (T0, T1) {}
