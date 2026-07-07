use wasmi_component::anyhow::{Context, Result};
#[allow(unused)]
use wasmi_component::wasi_p2::{add_wasi_p2_to_linker, resources::*};
#[allow(unused)]
use wasmi_component::wasmi::{AsContext, AsContextMut, Caller, FuncType, Linker, ValType};
#[allow(unused)]
use wasmi_component::{
    Borrow, CompValue, Component, HostResult, ListAccessor, LowerVal, MemoryAccessPre, Own,
    StoreData, TypedFunc, anyhow_result_to_wasmi,
};

#[allow(unused)]
pub trait TestExampleImports {
    fn list_i32(
        &mut self,
        value: ListAccessor<i32>,
    ) -> HostResult<impl LowerVal<Vec<i32>> + 'static>;

    fn list_string(
        &mut self,
        value: ListAccessor<String>,
    ) -> HostResult<impl LowerVal<Vec<String>> + 'static>;

    fn log(&mut self, message: &str) -> HostResult<()>;
}

#[allow(unused)]
pub struct TestExampleExports {
    pub init: TypedFunc<(), ()>,
    pub list_i32: TypedFunc<(Vec<i32>,), Vec<i32>>,
    pub list_string: TypedFunc<(Vec<String>,), Vec<String>>,
}

#[allow(unused)]
pub fn add_test_example_to_linker<D: TestExampleImports>(
    mut ctx: impl AsContextMut<Data = StoreData<D>>,
    linker: &mut Linker<StoreData<D>>,
    memory_index: usize,
) -> Result<()> {
    let mut params_ty = <(Vec<i32>,)>::arg_types();
    let mut result_ty = <Vec<i32>>::arg_types();
    let has_external_result = result_ty.len() > 1;
    if has_external_result {
        params_ty.push(ValType::I32);
        result_ty.clear();
    }

    linker.func_new(
        "wasmi-component:component-examples/round-trip@0.1.0",
        "list-i32",
        FuncType::new(params_ty, result_ty),
        move |mut caller, params, results| {
            let memory_pre = *caller.data().get_memory(memory_index);
            let (bytes, store_data) = memory_pre
                .memory
                .data_and_store_mut(caller.as_context_mut());

            let params_slice = if has_external_result {
                &params[0..(params.len() - 1)]
            } else {
                params
            };

            #[allow(unused)]
            let args = anyhow_result_to_wasmi(<(Vec<i32>,)>::lift_args(params_slice, bytes))?;
            let res = store_data.data_mut().list_i32(args.0)?;
            let mut memory_filled = memory_pre.fill(caller);

            if has_external_result {
                let address = params[params.len() - 1].i32().unwrap() as usize;
                let range = address..(address + <Vec<i32>>::byte_size());
                anyhow_result_to_wasmi(res.lower_bytes(range, &mut memory_filled))?;
            } else {
                anyhow_result_to_wasmi(res.lower_args(results, &mut memory_filled))?;
            }

            Ok(())
        },
    )?;

    let mut params_ty = <(Vec<String>,)>::arg_types();
    let mut result_ty = <Vec<String>>::arg_types();
    let has_external_result = result_ty.len() > 1;
    if has_external_result {
        params_ty.push(ValType::I32);
        result_ty.clear();
    }

    linker.func_new(
        "wasmi-component:component-examples/round-trip@0.1.0",
        "list-string",
        FuncType::new(params_ty, result_ty),
        move |mut caller, params, results| {
            let memory_pre = *caller.data().get_memory(memory_index);
            let (bytes, store_data) = memory_pre
                .memory
                .data_and_store_mut(caller.as_context_mut());

            let params_slice = if has_external_result {
                &params[0..(params.len() - 1)]
            } else {
                params
            };

            #[allow(unused)]
            let args = anyhow_result_to_wasmi(<(Vec<String>,)>::lift_args(params_slice, bytes))?;
            let res = store_data.data_mut().list_string(args.0)?;
            let mut memory_filled = memory_pre.fill(caller);

            if has_external_result {
                let address = params[params.len() - 1].i32().unwrap() as usize;
                let range = address..(address + <Vec<String>>::byte_size());
                anyhow_result_to_wasmi(res.lower_bytes(range, &mut memory_filled))?;
            } else {
                anyhow_result_to_wasmi(res.lower_args(results, &mut memory_filled))?;
            }

            Ok(())
        },
    )?;

    let mut params_ty = <(String,)>::arg_types();
    let mut result_ty = <()>::arg_types();
    let has_external_result = result_ty.len() > 1;
    if has_external_result {
        params_ty.push(ValType::I32);
        result_ty.clear();
    }

    linker.func_new(
        "$root",
        "log",
        FuncType::new(params_ty, result_ty),
        move |mut caller, params, results| {
            let memory_pre = *caller.data().get_memory(memory_index);
            let (bytes, store_data) = memory_pre
                .memory
                .data_and_store_mut(caller.as_context_mut());

            let params_slice = if has_external_result {
                &params[0..(params.len() - 1)]
            } else {
                params
            };

            #[allow(unused)]
            let args = anyhow_result_to_wasmi(<(String,)>::lift_args(params_slice, bytes))?;
            let res = store_data.data_mut().log(args.0)?;
            let mut memory_filled = memory_pre.fill(caller);

            if has_external_result {
                let address = params[params.len() - 1].i32().unwrap() as usize;
                let range = address..(address + <()>::byte_size());
                anyhow_result_to_wasmi(res.lower_bytes(range, &mut memory_filled))?;
            } else {
                anyhow_result_to_wasmi(res.lower_args(results, &mut memory_filled))?;
            }

            Ok(())
        },
    )?;

    Ok(())
}

#[allow(unused)]
pub fn instantiate_test_example_world<D: TestExampleImports>(
    mut ctx: impl AsContextMut<Data = StoreData<D>>,
    component: &Component,
) -> Result<TestExampleExports> {
    #[allow(unused_mut)]
    let mut linker = Linker::<StoreData<D>>::new(ctx.as_context().engine());
    let memory_index = ctx.as_context_mut().data_mut().next_memory_index();

    if component.is_wasi_p2() {
        add_wasi_p2_to_linker(ctx.as_context_mut(), &mut linker, memory_index)?;
    }

    add_test_example_to_linker(ctx.as_context_mut(), &mut linker, memory_index)?;

    let instance = linker.instantiate_and_start(ctx.as_context_mut(), &component.core_module)?;

    let memory = instance
        .get_memory(ctx.as_context(), "memory")
        .context("get memory")?;
    let cabi_realloc = instance
        .get_typed_func::<(i32, i32, i32, i32), i32>(ctx.as_context_mut(), "cabi_realloc")?;

    let memory_pre = MemoryAccessPre::new(memory, cabi_realloc);
    ctx.as_context_mut()
        .data_mut()
        .insert_memory(memory_index, memory_pre);

    let module_func = instance.get_func(ctx.as_context_mut(), "init").unwrap();
    let cleanup_func = instance
        .get_typed_func::<i32, ()>(ctx.as_context_mut(), "cabi_post_init")
        .ok();
    let init = TypedFunc::new(memory_pre.clone(), module_func, cleanup_func);

    let module_func = instance
        .get_func(
            ctx.as_context_mut(),
            "wasmi-component:component-examples/round-trip@0.1.0#list-i32",
        )
        .unwrap();
    let cleanup_func = instance
        .get_typed_func::<i32, ()>(
            ctx.as_context_mut(),
            "cabi_post_wasmi-component:component-examples/round-trip@0.1.0#list-i32",
        )
        .ok();
    let list_i32 = TypedFunc::new(memory_pre.clone(), module_func, cleanup_func);

    let module_func = instance
        .get_func(
            ctx.as_context_mut(),
            "wasmi-component:component-examples/round-trip@0.1.0#list-string",
        )
        .unwrap();
    let cleanup_func = instance
        .get_typed_func::<i32, ()>(
            ctx.as_context_mut(),
            "cabi_post_wasmi-component:component-examples/round-trip@0.1.0#list-string",
        )
        .ok();
    let list_string = TypedFunc::new(memory_pre.clone(), module_func, cleanup_func);

    Ok(TestExampleExports {
        init,
        list_i32,
        list_string,
    })
}
