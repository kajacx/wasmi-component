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
pub trait TestExampleImports {}

#[allow(unused)]
pub struct TestExampleExports {
    pub print_stdout: TypedFunc<(String,), ()>,
    pub print_stderr: TypedFunc<(String,), ()>,
}

#[allow(unused)]
impl TestExampleExports {
    pub fn call_print_stdout<T>(
        &self,
        ctx: impl AsContextMut<Data = StoreData<T>>,
        text: impl LowerVal<String>,
    ) -> Result<()> {
        self.print_stdout.call(ctx, (text,))
    }

    pub fn call_print_stdout_with_results<T, R>(
        &self,
        ctx: impl AsContextMut<Data = StoreData<T>>,
        text: impl LowerVal<String>,
        callback: impl FnOnce(()) -> R,
    ) -> Result<R> {
        self.print_stdout.call_with_results(ctx, (text,), callback)
    }

    pub fn call_print_stderr<T>(
        &self,
        ctx: impl AsContextMut<Data = StoreData<T>>,
        text: impl LowerVal<String>,
    ) -> Result<()> {
        self.print_stderr.call(ctx, (text,))
    }

    pub fn call_print_stderr_with_results<T, R>(
        &self,
        ctx: impl AsContextMut<Data = StoreData<T>>,
        text: impl LowerVal<String>,
        callback: impl FnOnce(()) -> R,
    ) -> Result<R> {
        self.print_stderr.call_with_results(ctx, (text,), callback)
    }
}

#[allow(unused)]
pub fn add_test_example_to_linker<T>(linker: &mut Linker<T>) -> Result<()> {
    Ok(())
}

#[allow(unused)]
pub fn instantiate_test_example_world<T>(
    mut ctx: impl AsContextMut<Data = StoreData<T>>,
    linker: &Linker<T>,
    component: &Component,
) -> Result<TestExampleExports> {
    let instance = linker.instantiate(ctx.as_context_mut(), &component)?;

    let print_stdout = instance.get_typed_func(
        ctx.as_context(),
        "wasmi-component:wasi-examples/exported-funcs@0.1.0#print-stdout",
    )?;

    let print_stderr = instance.get_typed_func(
        ctx.as_context(),
        "wasmi-component:wasi-examples/exported-funcs@0.1.0#print-stderr",
    )?;

    Ok(TestExampleExports {
        print_stdout,
        print_stderr,
    })
}
