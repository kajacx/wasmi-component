use anyhow::{Context, Result};
use wasmi::{AsContextMut, errors::LinkerError};

use crate::{
    Component, ComponentValue, HostResult, Instance, LowerValue, MemoryAccessPre, StoreData,
};

pub struct Linker<T> {
    linker: wasmi::Linker<StoreData<T>>,
}

impl<T> Linker<T> {
    pub fn new(engine: &wasmi::Engine) -> Self {
        Self {
            linker: wasmi::Linker::new(engine),
        }
    }

    pub fn func_new<Params: ComponentValue, Results: ComponentValue + LowerValue<Results>>(
        &mut self,
        module: &str,
        name: &str,
        callback: impl Fn(&mut T, Params::Borrowed<'_>) -> HostResult<Results> + Send + Sync + 'static,
    ) -> Result<&mut Self, LinkerError> {
        let mut params_ty = Params::arg_types();
        let mut result_ty = Results::arg_types();
        let has_external_result = result_ty.len() > 1;
        if has_external_result {
            params_ty.push(wasmi::ValType::I32);
            result_ty.clear();
        }

        self.linker.func_new(
            module,
            name,
            wasmi::FuncType::new(params_ty, result_ty),
            move |mut caller, params, results| {
                let instance_id = caller
                    .data()
                    .current_call_instance()
                    .expect("instance call stack should not be empty");

                let memory_pre = *caller.data().get_memory(instance_id);
                let (bytes, store_data) = memory_pre
                    .memory
                    .data_and_store_mut(caller.as_context_mut());

                let params_slice = if has_external_result {
                    &params[0..(params.len() - 1)]
                } else {
                    params
                };

                let user_data = store_data.data_mut();

                let args = Params::lift_args(params_slice, bytes)?;
                let res = callback(user_data, args)?;
                let mut memory_filled = memory_pre.fill(caller);

                if has_external_result {
                    // TODO: check type
                    let address = params[params.len() - 1].i32().unwrap() as usize;
                    let range = address..(address + Results::byte_size());
                    res.lower_bytes(range, &mut memory_filled)?;
                } else {
                    res.lower_args(results, &mut memory_filled)?;
                }

                Ok(())
            },
        )?;

        Ok(self)
    }

    pub fn instantiate(
        &self,
        mut ctx: impl AsContextMut<Data = StoreData<T>>,
        component: &Component,
    ) -> Result<Instance> {
        let memory_index = ctx.as_context_mut().data_mut().next_memory_index();

        let instance = self
            .linker
            .instantiate_and_start(ctx.as_context_mut(), &component.core_module)?;

        let memory = instance
            .get_memory(ctx.as_context(), "memory")
            .context("get memory")?;

        let cabi_realloc = instance
            .get_typed_func::<(i32, i32, i32, i32), i32>(ctx.as_context_mut(), "cabi_realloc")?;

        let memory_pre = MemoryAccessPre::new(memory_index, memory, cabi_realloc);

        ctx.as_context_mut()
            .data_mut()
            .insert_memory(memory_index, memory_pre);

        Ok(Instance::new(instance, memory_pre))
    }
}
