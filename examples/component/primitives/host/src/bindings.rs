use wasmi_component::{TypedFunc, Component};
use wasmi_component::wasmi::{AsContextMut, Linker};

pub struct ExampleExports {
    pub funcs_add_s32: TypedFunc<(i32, i32, ), i32>,
}

pub fn instantiate_example_world(mut ctx: impl AsContextMut, component: &Component) -> ExampleExports {
    let linker = Linker::new(ctx.as_context().engine());
    let instance = linker.instantiate_and_start(ctx.as_context_mut(), &component.core_module).unwrap();

    let module_func = instance.get_typed_func::<(i32, i32, ), i32>(ctx.as_context_mut(), "wasmi-component:examples/funcs@0.1.0#add-s32").unwrap();
    let funcs_add_s32 = TypedFunc::new(module_func);

    ExampleExports {
        funcs_add_s32,
    }
}
