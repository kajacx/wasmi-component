use wasmi_component::anyhow::{Context, Result};
#[allow(unused)]
use wasmi_component::wasmi::{AsContext, AsContextMut, Caller, FuncType, Linker, ValType};
#[allow(unused)]
use wasmi_component::{
    AsHostStorage, Component, FlatArgs, HostResult, Lift, Lower, LowerVal, MemoryAccessPre,
    TypedFunc, anyhow_result_to_wasmi,
};

pub trait TestExampleImports {
    fn common_funcs_roundtrip_s32(
        &mut self,
        value_a: <i32 as Lift>::Borrowed<'_>,
    ) -> HostResult<impl LowerVal<Target = i32> + 'static>;

    fn common_funcs_roundtrip_string(
        &mut self,
        value_a: <String as Lift>::Borrowed<'_>,
    ) -> HostResult<impl LowerVal<Target = String> + 'static>;

    fn common_funcs_roundtrip_multiple(
        &mut self,
        value_a: <String as Lift>::Borrowed<'_>,
        value_b: <i32 as Lift>::Borrowed<'_>,
    ) -> HostResult<impl LowerVal<Target = String> + 'static>;

    fn common_funcs_no_arguments(&mut self) -> HostResult<impl LowerVal<Target = ()> + 'static>;

    fn add_import(
        &mut self,
        value_a: <u32 as Lift>::Borrowed<'_>,
        value_b: <u32 as Lift>::Borrowed<'_>,
    ) -> HostResult<impl LowerVal<Target = u32> + 'static>;

    fn inline_imports_inline_add(
        &mut self,
        value_a: <u32 as Lift>::Borrowed<'_>,
        value_b: <u32 as Lift>::Borrowed<'_>,
    ) -> HostResult<impl LowerVal<Target = u32> + 'static>;
}

#[allow(unused)]
pub struct TestExampleExports {
    pub common_funcs_roundtrip_s32: TypedFunc<(i32,), i32>,
    pub common_funcs_roundtrip_string: TypedFunc<(String,), String>,
    pub common_funcs_roundtrip_multiple: TypedFunc<(String, i32), String>,
    pub common_funcs_no_arguments: TypedFunc<(), ()>,
    pub add_export: TypedFunc<(u32, u32), u32>,
    pub inline_exports_inline_add: TypedFunc<(u32, u32), u32>,
}

pub fn instantiate_test_example_world<D: AsHostStorage + TestExampleImports>(
    mut ctx: impl AsContextMut<Data = D>,
    component: &Component,
) -> Result<TestExampleExports> {
    #[allow(unused_mut)]
    let mut linker = Linker::<D>::new(ctx.as_context().engine());
    let memory_index = ctx
        .as_context_mut()
        .data_mut()
        .as_host_storage_mut()
        .next_memory_index();

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
            let memory_pre = *caller.data().as_host_storage().get_memory(memory_index);
            let (bytes, user_data) = memory_pre
                .memory
                .data_and_store_mut(caller.as_context_mut());

            #[allow(unused)]
            let params = anyhow_result_to_wasmi(<(i32,)>::lift_args(params, bytes))?;
            let res = user_data.common_funcs_roundtrip_s32(params.0)?;
            let mut memory_filled = memory_pre.fill(caller);
            anyhow_result_to_wasmi(res.lower_args(results, &mut memory_filled))?;

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
            let memory_pre = *caller.data().as_host_storage().get_memory(memory_index);
            let (bytes, user_data) = memory_pre
                .memory
                .data_and_store_mut(caller.as_context_mut());

            #[allow(unused)]
            let params = anyhow_result_to_wasmi(<(String,)>::lift_args(params, bytes))?;
            let res = user_data.common_funcs_roundtrip_string(params.0)?;
            let mut memory_filled = memory_pre.fill(caller);
            anyhow_result_to_wasmi(res.lower_args(results, &mut memory_filled))?;

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
            let memory_pre = *caller.data().as_host_storage().get_memory(memory_index);
            let (bytes, user_data) = memory_pre
                .memory
                .data_and_store_mut(caller.as_context_mut());

            #[allow(unused)]
            let params = anyhow_result_to_wasmi(<(String, i32)>::lift_args(params, bytes))?;
            let res = user_data.common_funcs_roundtrip_multiple(params.0, params.1)?;
            let mut memory_filled = memory_pre.fill(caller);
            anyhow_result_to_wasmi(res.lower_args(results, &mut memory_filled))?;

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
            let memory_pre = *caller.data().as_host_storage().get_memory(memory_index);
            let (bytes, user_data) = memory_pre
                .memory
                .data_and_store_mut(caller.as_context_mut());

            #[allow(unused)]
            let params = anyhow_result_to_wasmi(<()>::lift_args(params, bytes))?;
            let res = user_data.common_funcs_no_arguments()?;
            let mut memory_filled = memory_pre.fill(caller);
            anyhow_result_to_wasmi(res.lower_args(results, &mut memory_filled))?;

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
            let memory_pre = *caller.data().as_host_storage().get_memory(memory_index);
            let (bytes, user_data) = memory_pre
                .memory
                .data_and_store_mut(caller.as_context_mut());

            #[allow(unused)]
            let params = anyhow_result_to_wasmi(<(u32, u32)>::lift_args(params, bytes))?;
            let res = user_data.add_import(params.0, params.1)?;
            let mut memory_filled = memory_pre.fill(caller);
            anyhow_result_to_wasmi(res.lower_args(results, &mut memory_filled))?;

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
            let memory_pre = *caller.data().as_host_storage().get_memory(memory_index);
            let (bytes, user_data) = memory_pre
                .memory
                .data_and_store_mut(caller.as_context_mut());

            #[allow(unused)]
            let params = anyhow_result_to_wasmi(<(u32, u32)>::lift_args(params, bytes))?;
            let res = user_data.inline_imports_inline_add(params.0, params.1)?;
            let mut memory_filled = memory_pre.fill(caller);
            anyhow_result_to_wasmi(res.lower_args(results, &mut memory_filled))?;

            Ok(())
        },
    )?;

    let instance = linker.instantiate_and_start(ctx.as_context_mut(), &component.core_module)?;

    let memory = instance
        .get_memory(ctx.as_context(), "memory")
        .context("get memory")?;
    let cabi_realloc = instance
        .get_typed_func::<(i32, i32, i32, i32), i32>(ctx.as_context_mut(), "cabi_realloc")?;

    let memory_pre = MemoryAccessPre::new(memory, cabi_realloc);
    ctx.as_context_mut()
        .data_mut()
        .as_host_storage_mut()
        .insert_memory(memory_index, memory_pre);
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
    let common_funcs_roundtrip_s32 = TypedFunc::new(memory_pre.clone(), module_func, cleanup_func);

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
    let common_funcs_roundtrip_string =
        TypedFunc::new(memory_pre.clone(), module_func, cleanup_func);

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
    let common_funcs_roundtrip_multiple =
        TypedFunc::new(memory_pre.clone(), module_func, cleanup_func);

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
    let common_funcs_no_arguments = TypedFunc::new(memory_pre.clone(), module_func, cleanup_func);

    let module_func = instance
        .get_func(ctx.as_context_mut(), "add-export")
        .unwrap();
    let cleanup_func = instance
        .get_typed_func::<i32, ()>(ctx.as_context_mut(), "cabi_post_add-export")
        .ok();
    let add_export = TypedFunc::new(memory_pre.clone(), module_func, cleanup_func);

    let module_func = instance
        .get_func(ctx.as_context_mut(), "inline-exports#inline-add")
        .unwrap();
    let cleanup_func = instance
        .get_typed_func::<i32, ()>(ctx.as_context_mut(), "cabi_post_inline-exports#inline-add")
        .ok();
    let inline_exports_inline_add = TypedFunc::new(memory_pre.clone(), module_func, cleanup_func);

    Ok(TestExampleExports {
        common_funcs_roundtrip_s32,
        common_funcs_roundtrip_string,
        common_funcs_roundtrip_multiple,
        common_funcs_no_arguments,
        add_export,
        inline_exports_inline_add,
    })
}
