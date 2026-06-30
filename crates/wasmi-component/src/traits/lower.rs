use wasmi::WasmTy;

pub trait Lower {
    type CoreType: WasmTy;

    fn lower(&self) -> Self::CoreType;
}

impl Lower for i32 {
    type CoreType = Self;

    fn lower(&self) -> Self::CoreType {
        *self
    }
}

impl Lower for u32 {
    type CoreType = Self;

    fn lower(&self) -> Self::CoreType {
        *self
    }
}

impl Lower for f32 {
    type CoreType = Self;

    fn lower(&self) -> Self::CoreType {
        *self
    }
}
