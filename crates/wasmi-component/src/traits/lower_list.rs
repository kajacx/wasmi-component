use wasmi::WasmParams;

use crate::Lower;

pub trait LowerList {
    type CoreType: WasmParams;

    fn lower(&self) -> Self::CoreType;
}

impl LowerList for () {
    type CoreType = ();

    fn lower(&self) -> Self::CoreType {
        ()
    }
}

impl<T: Lower> LowerList for T {
    type CoreType = T::CoreType;

    fn lower(&self) -> Self::CoreType {
        self.lower()
    }
}

impl<T0: Lower, T1: Lower> LowerList for (T0, T1) {
    type CoreType = (T0::CoreType, T1::CoreType);

    fn lower(&self) -> Self::CoreType {
        (self.0.lower(), self.1.lower())
    }
}
