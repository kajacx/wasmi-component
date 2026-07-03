use wasmi_component::anyhow::{Context, Result};
#[allow(unused)]
use wasmi_component::wasmi::{AsContextMut, Caller, Linker};
#[allow(unused)]
use wasmi_component::{Component, HostResult, MemoryAccessPre, TypedFunc, WitString};

#[allow(unused)]
pub struct TestExampleExports {
    pub common_funcs_roundtrip_s32: TypedFunc<(i32, ), i32>,
    pub common_funcs_roundtrip_string: TypedFunc<(WitString, ), WitString>,
    pub common_funcs_roundtrip_multiple: TypedFunc<(WitString, i32, ), WitString>,
    pub common_funcs_no_arguments: TypedFunc<(), ()>,
    pub add_export: TypedFunc<(u32, u32, ), u32>,
    pub inline_exports_inline_add: TypedFunc<(u32, u32, ), u32>,
}

pub trait TestExampleImports {
    fn common_funcs_roundtrip_s32(&mut self, value_a: i32, ) -> HostResult<i32>;
    fn common_funcs_roundtrip_string(&mut self, value_a: WitString, ) -> HostResult<WitString>;
    fn common_funcs_roundtrip_multiple(&mut self, value_a: WitString, value_b: i32, ) -> HostResult<WitString>;
    fn common_funcs_no_arguments(&mut self, ) -> HostResult<()>;
    fn add_import(&mut self, value_a: u32, value_b: u32, ) -> HostResult<u32>;
    fn inline_imports_inline_add(&mut self, value_a: u32, value_b: u32, ) -> HostResult<u32>;
}

pub fn instantiate_test_example_world<D: TestExampleImports>(mut ctx: impl AsContextMut<Data = D>, component: &Component) -> Result<TestExampleExports> {
    #[allow(unused_mut)]
    let mut linker = Linker::new(ctx.as_context().engine());

    linker.func_wrap("$root", "roundtrip-s32", |mut caller: Caller<D>, value_a: i32, | caller.data_mut().common_funcs_roundtrip_s32(value_a, ))?;
    linker.func_wrap("$root", "roundtrip-string", |mut caller: Caller<D>, value_a: WitString, | caller.data_mut().common_funcs_roundtrip_string(value_a, ))?;
    linker.func_wrap("$root", "roundtrip-multiple", |mut caller: Caller<D>, value_a: WitString, value_b: i32, | caller.data_mut().common_funcs_roundtrip_multiple(value_a, value_b, ))?;
    linker.func_wrap("$root", "no-arguments", |mut caller: Caller<D>, | caller.data_mut().common_funcs_no_arguments())?;
    linker.func_wrap("$root", "add-import", |mut caller: Caller<D>, value_a: u32, value_b: u32, | caller.data_mut().add_import(value_a, value_b, ))?;
    linker.func_wrap("$root", "inline-add", |mut caller: Caller<D>, value_a: u32, value_b: u32, | caller.data_mut().inline_imports_inline_add(value_a, value_b, ))?;
    let instance = linker.instantiate_and_start(ctx.as_context_mut(), &component.core_module)?;

    let memory = instance.get_memory(ctx.as_context(), "memory").context("get memory")?;
    let cabi_realloc = instance.get_typed_func::<(i32, i32, i32, i32), i32>(ctx.as_context_mut(), "cabi_realloc")?;
    let memory_pre = MemoryAccessPre::new(memory, cabi_realloc);

    let module_func = instance.get_func(ctx.as_context_mut(), "wasmi-component:component-examples/common-funcs@0.1.0#roundtrip-s32").unwrap();
    let cleanup_func = instance.get_typed_func::<i32, ()>(ctx.as_context_mut(), "cabi_post_wasmi-component:component-examples/common-funcs@0.1.0#roundtrip-s32").ok();
    let common_funcs_roundtrip_s32 = TypedFunc::new(memory_pre.clone(), module_func, cleanup_func);

    let module_func = instance.get_func(ctx.as_context_mut(), "wasmi-component:component-examples/common-funcs@0.1.0#roundtrip-string").unwrap();
    let cleanup_func = instance.get_typed_func::<i32, ()>(ctx.as_context_mut(), "cabi_post_wasmi-component:component-examples/common-funcs@0.1.0#roundtrip-string").ok();
    let common_funcs_roundtrip_string = TypedFunc::new(memory_pre.clone(), module_func, cleanup_func);

    let module_func = instance.get_func(ctx.as_context_mut(), "wasmi-component:component-examples/common-funcs@0.1.0#roundtrip-multiple").unwrap();
    let cleanup_func = instance.get_typed_func::<i32, ()>(ctx.as_context_mut(), "cabi_post_wasmi-component:component-examples/common-funcs@0.1.0#roundtrip-multiple").ok();
    let common_funcs_roundtrip_multiple = TypedFunc::new(memory_pre.clone(), module_func, cleanup_func);

    let module_func = instance.get_func(ctx.as_context_mut(), "wasmi-component:component-examples/common-funcs@0.1.0#no-arguments").unwrap();
    let cleanup_func = instance.get_typed_func::<i32, ()>(ctx.as_context_mut(), "cabi_post_wasmi-component:component-examples/common-funcs@0.1.0#no-arguments").ok();
    let common_funcs_no_arguments = TypedFunc::new(memory_pre.clone(), module_func, cleanup_func);

    let module_func = instance.get_func(ctx.as_context_mut(), "add-export").unwrap();
    let cleanup_func = instance.get_typed_func::<i32, ()>(ctx.as_context_mut(), "cabi_post_add-export").ok();
    let add_export = TypedFunc::new(memory_pre.clone(), module_func, cleanup_func);

    let module_func = instance.get_func(ctx.as_context_mut(), "inline-exports#inline-add").unwrap();
    let cleanup_func = instance.get_typed_func::<i32, ()>(ctx.as_context_mut(), "cabi_post_inline-exports#inline-add").ok();
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
