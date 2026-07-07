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
pub trait TestExampleImports {}

#[allow(unused)]
pub struct TestExampleExports {
    pub print_stdout: TypedFunc<(String,), ()>,
    pub print_stderr: TypedFunc<(String,), ()>,
}

#[allow(unused)]
pub fn add_test_example_to_linker<D>(
    mut ctx: impl AsContextMut<Data = StoreData<D>>,
    linker: &mut Linker<StoreData<D>>,
    memory_index: usize,
) -> Result<()> {
    Ok(())
}

#[allow(unused)]
pub fn instantiate_test_example_world<D>(
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
        .get_func(
            ctx.as_context_mut(),
            "wasmi-component:wasi-examples/exported-funcs@0.1.0#print-stdout",
        )
        .unwrap();
    let cleanup_func = instance
        .get_typed_func::<i32, ()>(
            ctx.as_context_mut(),
            "cabi_post_wasmi-component:wasi-examples/exported-funcs@0.1.0#print-stdout",
        )
        .ok();
    let print_stdout = TypedFunc::new(memory_pre.clone(), module_func, cleanup_func);

    let module_func = instance
        .get_func(
            ctx.as_context_mut(),
            "wasmi-component:wasi-examples/exported-funcs@0.1.0#print-stderr",
        )
        .unwrap();
    let cleanup_func = instance
        .get_typed_func::<i32, ()>(
            ctx.as_context_mut(),
            "cabi_post_wasmi-component:wasi-examples/exported-funcs@0.1.0#print-stderr",
        )
        .ok();
    let print_stderr = TypedFunc::new(memory_pre.clone(), module_func, cleanup_func);

    Ok(TestExampleExports {
        print_stdout,
        print_stderr,
    })
}
