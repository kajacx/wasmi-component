use wasmi_component::{TypedFunc, Component};
use wasmi_component::wasmi::{AsContextMut, Linker};

#[allow(unused)]
pub struct ExampleExports {
    pub funcs_add_s32: TypedFunc<(i32, i32, ), i32>,
    pub add_u32: TypedFunc<(u32, u32, ), u32>,
    pub add_f32: TypedFunc<(f32, f32, ), f32>,
}

pub fn instantiate_example_world(mut ctx: impl AsContextMut, component: &Component) -> ExampleExports {
    let linker = Linker::new(ctx.as_context().engine());
    let instance = linker.instantiate_and_start(ctx.as_context_mut(), &component.core_module).unwrap();

    let module_func = instance.get_typed_func::<(i32, i32, ), i32>(ctx.as_context_mut(), "wasmi-component:examples/funcs@0.1.0#add-s32").unwrap();
    let funcs_add_s32 = TypedFunc::new(module_func);

    let module_func = instance.get_typed_func::<(u32, u32, ), u32>(ctx.as_context_mut(), "add-u32").unwrap();
    let add_u32 = TypedFunc::new(module_func);

    let module_func = instance.get_typed_func::<(f32, f32, ), f32>(ctx.as_context_mut(), "additional#add-f32").unwrap();
    let add_f32 = TypedFunc::new(module_func);

    ExampleExports {
        funcs_add_s32,
        add_u32,
        add_f32,
    }
}
