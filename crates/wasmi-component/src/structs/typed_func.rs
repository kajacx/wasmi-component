use std::marker::PhantomData;

use anyhow::{Context, Result};
use wasmi::{AsContextMut, Val};

use crate::{Lift, Lower, MemoryAccessFilled, MemoryAccessPre};

pub struct TypedFunc<Params, Results> {
    memory: MemoryAccessPre,
    inner: wasmi::Func,
    post_return: Option<wasmi::TypedFunc<i32, ()>>,
    _signature: PhantomData<fn(Params) -> Results>,
}

impl<Params: Lower, Results: Lift> TypedFunc<Params, Results> {
    pub fn new(
        memory: MemoryAccessPre,
        inner: wasmi::Func,
        post_return: Option<wasmi::TypedFunc<i32, ()>>,
    ) -> Self {
        Self {
            memory,
            inner,
            post_return,
            _signature: PhantomData,
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
        let mut args: [Val; 16] = std::array::from_fn(|_| Val::I32(0));
        let args_len = Params::args_amount();

        let mut memory_access = MemoryAccessFilled::new(&self.memory, ctx.as_context_mut());
        params.lower(&mut args[0..args_len], &mut memory_access)?;
        drop(memory_access);

        let mut results = [Val::I32(0)]; // TODO: this will be bad if the function doesn't return anything

        self.inner
            .call(ctx.as_context_mut(), &args[0..args_len], &mut results)?;

        let bytes = self.memory.memory.data(ctx.as_context());
        let lifted = Results::lift(results[0].clone(), bytes)?; // TODO: clone?

        let return_val = callback(lifted);

        if let Some(post_return) = self.post_return {
            let address = results[0]
                .i32()
                .context("Function with a cleanup method did not return an i32.")?;
            post_return.call(ctx, address)?;
        }

        Ok(return_val)
    }
}
