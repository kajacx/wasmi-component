use wasmi_component::anyhow::{Context, Result};
#[allow(unused)]
use wasmi_component::wasi_p2::{add_wasi_p2_to_linker, resources::*};
#[allow(unused)]
use wasmi_component::wasmi::{AsContext, AsContextMut, Caller, FuncType, Linker, ValType};
#[allow(unused)]
use wasmi_component::{
    Borrow, CompValue, Component, HostResult, LowerVal, MemoryAccessPre, Own, StoreData, TypedFunc,
    anyhow_result_to_wasmi,
};

#[allow(unused)]
pub trait TestExampleImports {
    fn roundtrip_s32(&mut self, value_a: i32) -> HostResult<i32>;

    fn roundtrip_string(&mut self, value_a: &str) -> HostResult<impl LowerVal<String> + 'static>;

    fn roundtrip_multiple(
        &mut self,
        value_a: &str,
        value_b: i32,
    ) -> HostResult<impl LowerVal<String> + 'static>;

    fn no_arguments(&mut self) -> HostResult<()>;

    fn inline_add(&mut self, value_a: u32, value_b: u32) -> HostResult<u32>;

    fn add_import(&mut self, value_a: u32, value_b: u32) -> HostResult<u32>;
}

#[allow(unused)]
pub struct TestExampleExports {
    pub add_export: TypedFunc<(u32, u32), u32>,
    pub roundtrip_s32: TypedFunc<(i32,), i32>,
    pub roundtrip_string: TypedFunc<(String,), String>,
    pub roundtrip_multiple: TypedFunc<(String, i32), String>,
    pub no_arguments: TypedFunc<(), ()>,
    pub inline_add: TypedFunc<(u32, u32), u32>,
}

#[allow(unused)]
pub fn add_test_example_to_linker<D: TestExampleImports>(
    mut ctx: impl AsContextMut<Data = StoreData<D>>,
    linker: &mut Linker<StoreData<D>>,
    memory_index: usize,
) -> Result<()> {
    let mut params_ty = <(i32,)>::arg_types();
    let mut result_ty = <i32>::arg_types();
    let has_external_result = result_ty.len() > 1;
    if has_external_result {
        params_ty.push(ValType::I32);
        result_ty.clear();
    }

    linker.func_new(
        "wasmi-component:component-examples/common-funcs@0.1.0",
        "roundtrip-s32",
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
            let args = anyhow_result_to_wasmi(<(i32,)>::lift_args(params_slice, bytes))?;
            let res = store_data.data_mut().roundtrip_s32(args.0)?;
            let mut memory_filled = memory_pre.fill(caller);

            if has_external_result {
                let address = params[params.len() - 1].i32().unwrap() as usize;
                let range = address..(address + <i32>::byte_size());
                anyhow_result_to_wasmi(res.lower_bytes(range, &mut memory_filled))?;
            } else {
                anyhow_result_to_wasmi(res.lower_args(results, &mut memory_filled))?;
            }

            Ok(())
        },
    )?;

    let mut params_ty = <(String,)>::arg_types();
    let mut result_ty = <String>::arg_types();
    let has_external_result = result_ty.len() > 1;
    if has_external_result {
        params_ty.push(ValType::I32);
        result_ty.clear();
    }

    linker.func_new(
        "wasmi-component:component-examples/common-funcs@0.1.0",
        "roundtrip-string",
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
            let res = store_data.data_mut().roundtrip_string(args.0)?;
            let mut memory_filled = memory_pre.fill(caller);

            if has_external_result {
                let address = params[params.len() - 1].i32().unwrap() as usize;
                let range = address..(address + <String>::byte_size());
                anyhow_result_to_wasmi(res.lower_bytes(range, &mut memory_filled))?;
            } else {
                anyhow_result_to_wasmi(res.lower_args(results, &mut memory_filled))?;
            }

            Ok(())
        },
    )?;

    let mut params_ty = <(String, i32)>::arg_types();
    let mut result_ty = <String>::arg_types();
    let has_external_result = result_ty.len() > 1;
    if has_external_result {
        params_ty.push(ValType::I32);
        result_ty.clear();
    }

    linker.func_new(
        "wasmi-component:component-examples/common-funcs@0.1.0",
        "roundtrip-multiple",
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
            let args = anyhow_result_to_wasmi(<(String, i32)>::lift_args(params_slice, bytes))?;
            let res = store_data.data_mut().roundtrip_multiple(args.0, args.1)?;
            let mut memory_filled = memory_pre.fill(caller);

            if has_external_result {
                let address = params[params.len() - 1].i32().unwrap() as usize;
                let range = address..(address + <String>::byte_size());
                anyhow_result_to_wasmi(res.lower_bytes(range, &mut memory_filled))?;
            } else {
                anyhow_result_to_wasmi(res.lower_args(results, &mut memory_filled))?;
            }

            Ok(())
        },
    )?;

    let mut params_ty = <()>::arg_types();
    let mut result_ty = <()>::arg_types();
    let has_external_result = result_ty.len() > 1;
    if has_external_result {
        params_ty.push(ValType::I32);
        result_ty.clear();
    }

    linker.func_new(
        "wasmi-component:component-examples/common-funcs@0.1.0",
        "no-arguments",
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
            let args = anyhow_result_to_wasmi(<()>::lift_args(params_slice, bytes))?;
            let res = store_data.data_mut().no_arguments()?;
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

    let mut params_ty = <(u32, u32)>::arg_types();
    let mut result_ty = <u32>::arg_types();
    let has_external_result = result_ty.len() > 1;
    if has_external_result {
        params_ty.push(ValType::I32);
        result_ty.clear();
    }

    linker.func_new(
        "inline-imports",
        "inline-add",
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
            let args = anyhow_result_to_wasmi(<(u32, u32)>::lift_args(params_slice, bytes))?;
            let res = store_data.data_mut().inline_add(args.0, args.1)?;
            let mut memory_filled = memory_pre.fill(caller);

            if has_external_result {
                let address = params[params.len() - 1].i32().unwrap() as usize;
                let range = address..(address + <u32>::byte_size());
                anyhow_result_to_wasmi(res.lower_bytes(range, &mut memory_filled))?;
            } else {
                anyhow_result_to_wasmi(res.lower_args(results, &mut memory_filled))?;
            }

            Ok(())
        },
    )?;

    let mut params_ty = <(u32, u32)>::arg_types();
    let mut result_ty = <u32>::arg_types();
    let has_external_result = result_ty.len() > 1;
    if has_external_result {
        params_ty.push(ValType::I32);
        result_ty.clear();
    }

    linker.func_new(
        "$root",
        "add-import",
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
            let args = anyhow_result_to_wasmi(<(u32, u32)>::lift_args(params_slice, bytes))?;
            let res = store_data.data_mut().add_import(args.0, args.1)?;
            let mut memory_filled = memory_pre.fill(caller);

            if has_external_result {
                let address = params[params.len() - 1].i32().unwrap() as usize;
                let range = address..(address + <u32>::byte_size());
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

    let module_func = instance
        .get_func(ctx.as_context_mut(), "add-export")
        .unwrap();
    let cleanup_func = instance
        .get_typed_func::<i32, ()>(ctx.as_context_mut(), "cabi_post_add-export")
        .ok();
    let add_export = TypedFunc::new(memory_pre.clone(), module_func, cleanup_func);

    let module_func = instance
        .get_func(
            ctx.as_context_mut(),
            "wasmi-component:component-examples/common-funcs@0.1.0#roundtrip-s32",
        )
        .unwrap();
    let cleanup_func = instance
        .get_typed_func::<i32, ()>(
            ctx.as_context_mut(),
            "cabi_post_wasmi-component:component-examples/common-funcs@0.1.0#roundtrip-s32",
        )
        .ok();
    let roundtrip_s32 = TypedFunc::new(memory_pre.clone(), module_func, cleanup_func);

    let module_func = instance
        .get_func(
            ctx.as_context_mut(),
            "wasmi-component:component-examples/common-funcs@0.1.0#roundtrip-string",
        )
        .unwrap();
    let cleanup_func = instance
        .get_typed_func::<i32, ()>(
            ctx.as_context_mut(),
            "cabi_post_wasmi-component:component-examples/common-funcs@0.1.0#roundtrip-string",
        )
        .ok();
    let roundtrip_string = TypedFunc::new(memory_pre.clone(), module_func, cleanup_func);

    let module_func = instance
        .get_func(
            ctx.as_context_mut(),
            "wasmi-component:component-examples/common-funcs@0.1.0#roundtrip-multiple",
        )
        .unwrap();
    let cleanup_func = instance
        .get_typed_func::<i32, ()>(
            ctx.as_context_mut(),
            "cabi_post_wasmi-component:component-examples/common-funcs@0.1.0#roundtrip-multiple",
        )
        .ok();
    let roundtrip_multiple = TypedFunc::new(memory_pre.clone(), module_func, cleanup_func);

    let module_func = instance
        .get_func(
            ctx.as_context_mut(),
            "wasmi-component:component-examples/common-funcs@0.1.0#no-arguments",
        )
        .unwrap();
    let cleanup_func = instance
        .get_typed_func::<i32, ()>(
            ctx.as_context_mut(),
            "cabi_post_wasmi-component:component-examples/common-funcs@0.1.0#no-arguments",
        )
        .ok();
    let no_arguments = TypedFunc::new(memory_pre.clone(), module_func, cleanup_func);

    let module_func = instance
        .get_func(ctx.as_context_mut(), "inline-exports#inline-add")
        .unwrap();
    let cleanup_func = instance
        .get_typed_func::<i32, ()>(ctx.as_context_mut(), "cabi_post_inline-exports#inline-add")
        .ok();
    let inline_add = TypedFunc::new(memory_pre.clone(), module_func, cleanup_func);

    Ok(TestExampleExports {
        add_export,
        roundtrip_s32,
        roundtrip_string,
        roundtrip_multiple,
        no_arguments,
        inline_add,
    })
}
