use std::marker::PhantomData;

use anyhow::{Context, Result};
use wasmi::{AsContextMut, Val};

use crate::{
    ComponentValue, FatPtr, LowerVal, MemoryAccessFilled, MemoryAccessPre, StoreData, View,
};

pub struct TypedFunc<Params, Results> {
    memory: MemoryAccessPre,
    inner: wasmi::Func,
    post_return: Option<wasmi::TypedFunc<i32, ()>>,
    _signature: PhantomData<fn(Params) -> Results>,
}

impl<Params: ComponentValue, Results: ComponentValue> TypedFunc<Params, Results> {
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

    pub fn call<T>(
        &self,
        ctx: impl AsContextMut<Data = StoreData<T>>,
        params: impl LowerVal<Params>,
    ) -> Result<Results> {
        self.call_with_results(ctx, params, |res| res.lift_owned())?
    }

    pub fn call_with_results<T, P: LowerVal<Params>, R>(
        &self,
        mut ctx: impl AsContextMut<Data = StoreData<T>>,
        params: P,
        callback: impl FnOnce(Results::Borrowed<'_>) -> R,
    ) -> Result<R> {
        let mut args: [Val; 16] = std::array::from_fn(|_| Val::I32(0));
        let args_len = Params::arg_count();

        let mut memory_access = MemoryAccessFilled::new(&self.memory, ctx.as_context_mut());
        params.lower_args(&mut args[0..args_len], &mut memory_access)?;
        drop(memory_access);

        let mut results = [Val::I32(0)];
        let results_indirect = Results::arg_count() > 1;

        let results_slice = if results_indirect {
            &mut results
        } else {
            &mut results[0..Results::arg_count()]
        };

        let instance_id = self.memory.instance_id;
        let depth = ctx
            .as_context_mut()
            .data_mut()
            .push_call_instance(instance_id);

        self.inner
            .call(ctx.as_context_mut(), &args[0..args_len], results_slice)?;

        ctx.as_context_mut()
            .data_mut()
            .pop_call_instance(instance_id, depth)?;

        let bytes = self.memory.memory.data(ctx.as_context());
        let lifted = if results_indirect {
            let address = results[0].i32().context("i32 address return")? as usize;
            let ptr = FatPtr::new(address, Results::byte_size(), 1);

            let slice = ptr.try_index(bytes)?;
            Results::lift_bytes(slice, bytes)?
        } else {
            Results::lift_args(results_slice, bytes)?
        };

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
