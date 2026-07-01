use anyhow::{Context, Result};
use wasmi::{AsContextMut, Memory};

use crate::{Lift, LowerList};

pub struct TypedFunc<Params: LowerList, Results: Lift> {
    memory: Memory,
    inner: wasmi::TypedFunc<Params::CoreType, Results::CoreType>,
    post_return: Option<wasmi::TypedFunc<i32, ()>>,
}

impl<Params: LowerList, Results: Lift> TypedFunc<Params, Results> {
    pub fn new(
        memory: Memory,
        inner: wasmi::TypedFunc<Params::CoreType, Results::CoreType>,
        post_return: Option<wasmi::TypedFunc<i32, ()>>,
    ) -> Self {
        Self {
            memory,
            inner,
            post_return,
        }
    }

    pub fn call(&self, ctx: impl AsContextMut, params: Params) -> Result<Results> {
        self.call_with_results(ctx, params, Results::into_owned)
    }

    pub fn call_with_results<T>(
        &self,
        mut ctx: impl AsContextMut,
        params: Params,
        callback: impl FnOnce(Results::Borrowed<'_>) -> T,
    ) -> Result<T> {
        let result = self.inner.call(ctx.as_context_mut(), params.lower())?;
        let result_addr = Results::as_address(&result);

        let bytes = self.memory.data(ctx.as_context());
        let lifted = Results::lift(result, bytes)?;

        let return_val = callback(lifted);

        if let Some(post_return) = self.post_return {
            let address = result_addr.context("Wrong return type of function with a cleanup")?;
            post_return.call(ctx, address)?;
        }

        Ok(return_val)
    }
}
