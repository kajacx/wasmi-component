use wasmi_component::anyhow::Result;
#[allow(unused)]
use wasmi_component::wasi_p2::{add_wasi_p2_to_linker, resources::*};
#[allow(unused)]
use wasmi_component::wasmi::{AsContext, AsContextMut};
#[allow(unused)]
use wasmi_component::{
    Borrow, Component, ComponentValue, HostResult, Linker, ListAccessor, LowerVal, Own, StoreData,
    TypedFunc,
};

#[allow(unused)]
pub trait TestExampleImports {
    fn roundtrip_s32(&mut self, value_a: i32) -> HostResult<i32>;

    fn roundtrip_string(&mut self, value_a: &str) -> HostResult<String>;

    fn roundtrip_multiple(&mut self, value_a: &str, value_b: i32) -> HostResult<String>;

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
pub fn add_test_example_to_linker<T: TestExampleImports>(linker: &mut Linker<T>) -> Result<()> {
    linker.func_new::<(i32,), i32>(
        "wasmi-component:component-examples/common-funcs@0.1.0",
        "roundtrip-s32",
        |host_data, params| host_data.roundtrip_s32(params.0),
    )?;

    linker.func_new::<(String,), String>(
        "wasmi-component:component-examples/common-funcs@0.1.0",
        "roundtrip-string",
        |host_data, params| host_data.roundtrip_string(params.0),
    )?;

    linker.func_new::<(String, i32), String>(
        "wasmi-component:component-examples/common-funcs@0.1.0",
        "roundtrip-multiple",
        |host_data, params| host_data.roundtrip_multiple(params.0, params.1),
    )?;

    linker.func_new::<(), ()>(
        "wasmi-component:component-examples/common-funcs@0.1.0",
        "no-arguments",
        |host_data, params| host_data.no_arguments(),
    )?;

    linker.func_new::<(u32, u32), u32>("inline-imports", "inline-add", |host_data, params| {
        host_data.inline_add(params.0, params.1)
    })?;

    linker.func_new::<(u32, u32), u32>("$root", "add-import", |host_data, params| {
        host_data.add_import(params.0, params.1)
    })?;

    Ok(())
}

#[allow(unused)]
pub fn instantiate_test_example_world<T>(
    mut ctx: impl AsContextMut<Data = StoreData<T>>,
    linker: &Linker<T>,
    component: &Component,
) -> Result<TestExampleExports> {
    let instance = linker.instantiate(ctx.as_context_mut(), &component)?;

    let add_export = instance.get_typed_func(ctx.as_context(), "add-export")?;

    let roundtrip_s32 = instance.get_typed_func(
        ctx.as_context(),
        "wasmi-component:component-examples/common-funcs@0.1.0#roundtrip-s32",
    )?;

    let roundtrip_string = instance.get_typed_func(
        ctx.as_context(),
        "wasmi-component:component-examples/common-funcs@0.1.0#roundtrip-string",
    )?;

    let roundtrip_multiple = instance.get_typed_func(
        ctx.as_context(),
        "wasmi-component:component-examples/common-funcs@0.1.0#roundtrip-multiple",
    )?;

    let no_arguments = instance.get_typed_func(
        ctx.as_context(),
        "wasmi-component:component-examples/common-funcs@0.1.0#no-arguments",
    )?;

    let inline_add = instance.get_typed_func(ctx.as_context(), "inline-exports#inline-add")?;

    Ok(TestExampleExports {
        add_export,
        roundtrip_s32,
        roundtrip_string,
        roundtrip_multiple,
        no_arguments,
        inline_add,
    })
}
