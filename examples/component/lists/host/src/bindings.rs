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
    type ListI32Return<'a>: LowerVal<Vec<i32>> + 'a
    where
        Self: 'a;

    fn list_i32<'a>(&'a mut self, value: ListAccessor<i32>) -> HostResult<Self::ListI32Return<'a>>;

    type ListStringReturn: LowerVal<Vec<String>> + 'static;
    fn list_string(&mut self, value: ListAccessor<String>) -> HostResult<Self::ListStringReturn>;

    fn log(&mut self, message: &str) -> HostResult<()>;
}

#[allow(unused)]
pub struct TestExampleExports {
    pub init: TypedFunc<(), ()>,
    pub list_i32: TypedFunc<(Vec<i32>,), Vec<i32>>,
    pub list_string: TypedFunc<(Vec<String>,), Vec<String>>,
}

#[allow(unused)]
pub fn add_test_example_to_linker<T: TestExampleImports>(linker: &mut Linker<T>) -> Result<()> {
    linker.func_new::<(Vec<i32>,), Vec<i32>, T::ListI32Return>(
        "wasmi-component:component-examples/round-trip@0.1.0",
        "list-i32",
        |host_data, params| host_data.list_i32(params.0),
    )?;

    linker.func_new::<(Vec<String>,), Vec<String>, _>(
        "wasmi-component:component-examples/round-trip@0.1.0",
        "list-string",
        |host_data, params| host_data.list_string(params.0),
    )?;

    linker.func_new::<(String,), (), _>("$root", "log", |host_data, params| {
        host_data.log(params.0)
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

    let init = instance.get_typed_func(ctx.as_context(), "init")?;

    let list_i32 = instance.get_typed_func(
        ctx.as_context(),
        "wasmi-component:component-examples/round-trip@0.1.0#list-i32",
    )?;

    let list_string = instance.get_typed_func(
        ctx.as_context(),
        "wasmi-component:component-examples/round-trip@0.1.0#list-string",
    )?;

    Ok(TestExampleExports {
        init,
        list_i32,
        list_string,
    })
}
