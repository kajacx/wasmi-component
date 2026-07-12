use std::marker::PhantomData;

use wasmi::{AsContextMut, Val};

use crate::{CallResult, ComponentValue, FatPtr, LowerValue, MemoryAccessPre, StoreData, View};

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
        params: impl LowerValue<Params>,
    ) -> CallResult<Results> {
        let result = self.call_with_results(ctx, params, |res| res.lift_owned());
        Ok(result??)
    }

    pub fn call_with_results<T, R>(
        &self,
        mut ctx: impl AsContextMut<Data = StoreData<T>>,
        params: impl LowerValue<Params>,
        callback: impl FnOnce(Results::Borrowed<'_>) -> R,
    ) -> CallResult<R> {
        let mut args: [Val; 16] = std::array::from_fn(|_| Val::I32(0));
        let args_len = Params::arg_count();

        let mut memory_access = self.memory.fill(ctx.as_context_mut());
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

        let call_result =
            self.inner
                .call_resumable(ctx.as_context_mut(), &args[0..args_len], results_slice);

        ctx.as_context_mut()
            .data_mut()
            .pop_call_instance(instance_id, depth)
            .expect("TODO: ");

        call_result?;

        let bytes = self.memory.memory.data(ctx.as_context());
        let lifted = if results_indirect {
            // TODO: check params ... again
            let address = results[0].i32().unwrap() as usize;
            let ptr = FatPtr::new(address, Results::byte_size(), 1);

            let slice = ptr.try_index(bytes)?;
            Results::lift_bytes(slice, bytes)?
        } else {
            Results::lift_args(results_slice, bytes)?
        };

        let return_val = callback(lifted);

        if let Some(post_return) = self.post_return {
            let address = results[0].i32().expect("TODO: ");
            post_return.call(ctx, address)?;
        }

        Ok(return_val)
    }
}
