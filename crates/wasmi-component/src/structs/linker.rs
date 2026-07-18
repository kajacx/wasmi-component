use anyhow::{Context, Result};
use wasmi::AsContextMut;
use wasmi_component_parser::FuncIdentifier;

use crate::{
    Component, ComponentValue, FuncSignature, FuncStorage, HostResult, Instance, Lower,
    MemoryAccessPre, StoreData, WasmValue,
};

pub struct Linker<T> {
    linker: wasmi::Linker<StoreData<T>>,
    imported_funcs: FuncStorage,
}

impl<T> Linker<T> {
    pub fn new(engine: &wasmi::Engine) -> Self {
        Self {
            linker: wasmi::Linker::new(engine),
            imported_funcs: FuncStorage::new(),
        }
    }

    pub fn func_new<Params: ComponentValue, Results: ComponentValue + Lower<Results>>(
        &mut self,
        module: impl Into<String>,
        name: impl Into<String>,
        callback: impl Fn(&mut T, Params::Borrowed<'_>) -> HostResult<Results> + Send + Sync + 'static,
    ) -> Result<&mut Self, wasmi::errors::LinkerError> {
        let ident = FuncIdentifier::new(module.into(), name.into());

        let mut params_ty = Params::arg_types();
        let mut results_ty = Results::arg_types();

        let results_ty_original = results_ty.clone();

        let params_len = params_ty.len();
        let results_len = results_ty.len();

        let has_external_result = results_ty.len() > 1;
        if has_external_result {
            params_ty.push(wasmi::ValType::I32);
            results_ty.clear();
        }

        self.linker.func_new(
            ident.imported_module_name(),
            &ident.name,
            wasmi::FuncType::new(params_ty, results_ty),
            move |mut caller, params_wasmi, results_wasmi| {
                let instance_id = caller
                    .data()
                    .current_call_instance()
                    .expect("instance call stack should not be empty");

                let memory_pre = *caller.data().get_memory(instance_id);
                let (bytes, store_data) = memory_pre
                    .memory
                    .data_and_store_mut(caller.as_context_mut());

                let mut params_wasm: [_; 16] = std::array::from_fn(|_| WasmValue::Unset);
                WasmValue::convert_from_wasmi(
                    &params_wasmi[0..params_len],
                    &mut params_wasm[0..params_len],
                );

                let user_data = store_data.data_mut();

                let params_user = Params::lift_args(&params_wasm[0..params_len], bytes)?;
                let results_user = callback(user_data, params_user)?;
                let mut memory_filled = memory_pre.fill(caller);

                if has_external_result {
                    let address = params_wasmi[params_len].i32().unwrap() as usize;
                    let range = address..(address + Results::byte_size());
                    results_user.lower_bytes(range, &mut memory_filled)?;
                } else {
                    let mut results_wasm = [WasmValue::Unset];
                    results_user
                        .lower_args(&mut results_wasm[0..results_len], &mut memory_filled)?;
                    WasmValue::convert_to_wasmi(
                        &results_wasm[0..results_len],
                        &results_ty_original,
                        results_wasmi,
                    )?;
                }

                Ok(())
            },
        )?;

        self.imported_funcs.insert(
            ident,
            FuncSignature::new(Params::value_type(), Results::value_type()),
        );

        Ok(self)
    }

    pub fn instantiate(
        &self,
        mut ctx: impl AsContextMut<Data = StoreData<T>>,
        component: &Component,
    ) -> Result<Instance> {
        for (ident, signature) in &component.imported_funcs.data {
            self.imported_funcs.verify_import(ident, signature)?;
        }

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

        Ok(Instance::new(
            instance,
            component.exported_funcs.clone(),
            memory_pre,
        ))
    }

    /// Enable overriding the same imported function with a new one.
    /// Disabled by default.
    pub fn allow_shadowing(&mut self, allow: bool) -> &mut Self {
        self.linker.allow_shadowing(allow);
        self
    }
}
