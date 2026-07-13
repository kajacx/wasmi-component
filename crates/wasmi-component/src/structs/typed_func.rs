use std::marker::PhantomData;

use wasmi::AsContextMut;

use crate::{
    CallResult, ComponentValue, FatPtr, LowerValue, MemoryAccessPre, StoreData, View, WasmValue,
};

pub struct TypedFunc<Params, Results> {
    memory: MemoryAccessPre,

    inner: wasmi::Func,
    post_return: Option<wasmi::TypedFunc<i32, ()>>,

    param_types: Vec<wasmi::ValType>,
    _signature: PhantomData<fn(Params) -> Results>,
}

impl<Params: ComponentValue, Results: ComponentValue> TypedFunc<Params, Results> {
    pub(crate) fn new(
        memory: MemoryAccessPre,
        inner: wasmi::Func,
        post_return: Option<wasmi::TypedFunc<i32, ()>>,
    ) -> Self {
        Self {
            memory,
            inner,
            post_return,
            param_types: Params::arg_types(),
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
        let params_user = params;

        let mut params_wasm: [_; 16] = std::array::from_fn(|_| WasmValue::Unset);
        let params_len = Params::arg_count();
        debug_assert_eq!(self.param_types.len(), params_len);

        let mut memory_access = self.memory.fill(ctx.as_context_mut());
        params_user.lower_args(&mut params_wasm[0..params_len], &mut memory_access)?;
        drop(memory_access);

        let mut params_wasmi: [_; 16] = std::array::from_fn(|_| wasmi::Val::I32(0));
        WasmValue::convert_to_wasmi(
            &params_wasm[0..params_len],
            &self.param_types,
            &mut params_wasmi[0..params_len],
        )?;

        let mut results_wasmi = [wasmi::Val::I32(0)];
        let results_indirect = Results::arg_count() > 1;

        let results_slice = if results_indirect {
            &mut results_wasmi
        } else {
            &mut results_wasmi[0..Results::arg_count()]
        };

        let instance_id = self.memory.instance_id;
        let depth = ctx
            .as_context_mut()
            .data_mut()
            .push_call_instance(instance_id);

        let call_result = self.inner.call_resumable(
            ctx.as_context_mut(),
            &params_wasmi[0..params_len],
            results_slice,
        );

        ctx.as_context_mut()
            .data_mut()
            .pop_call_instance(instance_id, depth)
            .expect("TODO: ");

        call_result?;

        let bytes = self.memory.memory.data(ctx.as_context());
        let lifted = if results_indirect {
            // TODO: check params ... again
            let address = results_wasmi[0].i32().unwrap() as usize;
            let ptr = FatPtr::new(address, Results::byte_size(), 1);

            let slice = ptr.try_index(bytes)?;
            Results::lift_bytes(slice, bytes)?
        } else {
            let mut results = [WasmValue::Unset];
            WasmValue::convert_from_wasmi(results_slice, &mut results[0..results_slice.len()]);
            Results::lift_args(&results[0..results_slice.len()], bytes)?
        };

        let return_val = callback(lifted);

        if let Some(post_return) = self.post_return {
            let address = results_wasmi[0]
                .i32()
                .expect("function should return an i32 if it has a post return fn");
            post_return.call(ctx, address)?;
        }

        Ok(return_val)
    }
}
