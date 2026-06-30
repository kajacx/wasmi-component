use wasmi::{AsContextMut, WasmParams, WasmResults};

pub struct TypedFunc<Params, Results> {
    inner: wasmi::TypedFunc<Params, Results>,
}

impl<Params, Results> TypedFunc<Params, Results> {
    pub fn new(inner: wasmi::TypedFunc<Params, Results>) -> Self {
        Self { inner }
    }

    pub fn call(&self, store: impl AsContextMut, params: Params) -> Result<Results, wasmi::Error>
    where
        Params: WasmParams,
        Results: WasmResults,
    {
        self.inner.call(store, params)
    }
}
