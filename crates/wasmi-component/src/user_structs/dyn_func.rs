use wasmi::AsContextMut;
use wasmi_component_parser::FuncIdentifier;

use crate::lib_structs::{
    FuncSignature, LiftArgsReader, LiftBytesReader, LowerArgsWriter, MemoryAccessPre, WasmValue,
};
use crate::pointers::FatPtr;
use crate::{CallResult, DynValue, DynValueParams, StoreData, dyn_lift};

#[derive(Debug, Clone)]
pub struct DynFunc {
    memory: MemoryAccessPre,

    inner: wasmi::Func,
    post_return: Option<wasmi::TypedFunc<i32, ()>>,

    ident: FuncIdentifier,
    signature: FuncSignature,
}

impl DynFunc {
    pub(crate) fn new(
        memory: MemoryAccessPre,
        inner: wasmi::Func,
        post_return: Option<wasmi::TypedFunc<i32, ()>>,
        ident: FuncIdentifier,
        signature: FuncSignature,
    ) -> Self {
        Self {
            memory,
            inner,
            post_return,
            ident,
            signature,
        }
    }

    pub fn call<T>(
        &self,
        mut ctx: impl AsContextMut<Data = StoreData<T>>,
        params: impl AsRef<[DynValue]>,
    ) -> CallResult<DynValue> {
        let params_user = DynValueParams::new(params.as_ref());
        params_user.check_params_signature(self.signature.params.as_ref(), &self.ident)?;

        let mut params_wasm: [_; 16] = std::array::from_fn(|_| WasmValue::Unused);
        let params_len = self.signature.params_as_tuple().arg_count();

        let memory_access = self.memory.fill(ctx.as_context_mut());

        let mut args_writer = LowerArgsWriter::new(memory_access, &mut params_wasm[0..params_len]);
        params_user.lower_args(self.signature.params.as_ref(), &mut args_writer)?;

        let mut params_wasmi: [_; 16] = std::array::from_fn(|_| wasmi::Val::I32(0));
        WasmValue::convert_to_wasmi(
            &params_wasm[0..params_len],
            self.inner.ty(ctx.as_context()).params(),
            &mut params_wasmi[0..params_len],
        )?;

        let result_ty = &self.signature.result;
        let mut results_wasmi = [wasmi::Val::I32(0)];
        let results_indirect = result_ty.arg_count() > 1;

        let results_slice = if results_indirect {
            &mut results_wasmi
        } else {
            &mut results_wasmi[0..result_ty.arg_count()]
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
        ); // Do not propagate error yet

        ctx.as_context_mut()
            .data_mut()
            .pop_call_instance(instance_id, depth)
            .expect("instance call stack should match");

        // Propagate error now, after the call stack has been updated
        call_result?;

        let bytes = self.memory.memory.data(ctx.as_context());
        let results_user = if results_indirect {
            let address = results_wasmi[0].i32().unwrap() as usize;
            let ptr = FatPtr::new(address, result_ty.byte_size(), 1);
            let slice = ptr.try_index(bytes)?;

            let mut byte_reader = LiftBytesReader::new(bytes, slice);
            dyn_lift(result_ty, &mut byte_reader)?
        } else {
            let mut results_wasm = [WasmValue::Unused];
            WasmValue::convert_from_wasmi(results_slice, &mut results_wasm[0..results_slice.len()]);

            let mut args_reader = LiftArgsReader::new(bytes, &results_wasm[0..results_slice.len()]);
            dyn_lift(result_ty, &mut args_reader)?
        };

        if let Some(post_return) = self.post_return {
            let address = results_wasmi[0]
                .i32()
                .expect("function with cleanup returning a single i32 was checked");
            post_return.call(ctx, address)?;
        }

        Ok(results_user)
    }
}
