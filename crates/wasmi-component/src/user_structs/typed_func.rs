use std::marker::PhantomData;

use wasmi::AsContextMut;

use crate::lib_structs::{LiftArgsReader, LiftBytesReader, MemoryAccessPre, WasmValue, wasm_args};
use crate::pointers::FatPtr;
use crate::{CallResult, ComponentValue, Lift, Lower, StoreData};

#[derive(Debug, Clone)]
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
            param_types: wasm_args(&Params::value_type()),
            _signature: PhantomData,
        }
    }

    pub fn call<T>(
        &self,
        ctx: impl AsContextMut<Data = StoreData<T>>,
        params: impl Lower<Params>,
    ) -> CallResult<Results> {
        let result = self.call_with_results(ctx, params, |_data, res| res.lift_owned());
        Ok(result??)
    }

    pub fn call_with_results<T, R>(
        &self,
        mut ctx: impl AsContextMut<Data = StoreData<T>>,
        params: impl Lower<Params>,
        callback: impl FnOnce(&mut T, Results::Borrowed<'_>) -> R,
    ) -> CallResult<R> {
        let params_user = params;

        let mut params_wasm: [_; 16] = std::array::from_fn(|_| WasmValue::Unset);
        let params_len = Params::arg_count();
        debug_assert_eq!(self.param_types.len(), params_len);

        let mut memory_access = self.memory.fill(ctx.as_context_mut());
        params_user.lower_args(&mut params_wasm[0..params_len], &mut memory_access)?;
        drop(memory_access);

        // TODO: more than 16 flat args
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

        // Do not propagate error yet
        let call_result = self.inner.call_resumable(
            ctx.as_context_mut(),
            &params_wasmi[0..params_len],
            results_slice,
        );

        ctx.as_context_mut()
            .data_mut()
            .pop_call_instance(instance_id, depth)
            .expect("instance call stack should match");

        // Propagate error now, after the call stack has been updated
        call_result?;

        let (bytes, store_data) = self.memory.memory.data_and_store_mut(ctx.as_context_mut());
        let results_user = if results_indirect {
            let address = results_wasmi[0].i32().unwrap() as usize;
            let ptr = FatPtr::new(address, Results::byte_size(), 1);
            let slice = ptr.try_index(bytes)?;

            let mut byte_reader = LiftBytesReader::new(bytes, slice);
            Results::lift(&mut byte_reader)?
        } else {
            let mut results_wasm = [WasmValue::Unset];
            WasmValue::convert_from_wasmi(results_slice, &mut results_wasm[0..results_slice.len()]);

            let mut args_reader = LiftArgsReader::new(bytes, &results_wasm[0..results_slice.len()]);
            Results::lift(&mut args_reader)?
        };

        let return_value = callback(store_data.data_mut(), results_user);

        if let Some(post_return) = self.post_return {
            let address = results_wasmi[0]
                .i32()
                .expect("function with cleanup returning a single i32 was checked");
            post_return.call(ctx, address)?;
        }

        Ok(return_value)
    }
}
