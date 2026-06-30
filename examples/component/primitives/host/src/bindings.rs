use wasmi_component::wasmi::{AsContextMut, Linker};
use wasmi_component::{Component, Lift, LowerList, TypedFunc};

#[allow(unused)]
pub struct ExampleExports {
    pub funcs_add_s32: TypedFunc<(i32, i32, ), i32>,
    pub funcs_get_name: TypedFunc<(), String>,
    pub add_u32: TypedFunc<(u32, u32, ), u32>,
    pub add_f32: TypedFunc<(f32, f32, ), f32>,
}

pub fn instantiate_example_world(mut ctx: impl AsContextMut, component: &Component) -> ExampleExports {
    let linker = Linker::new(ctx.as_context().engine());
    let instance = linker.instantiate_and_start(ctx.as_context_mut(), &component.core_module).unwrap();

    let module_func = instance.get_typed_func::<<(i32, i32, ) as LowerList>::CoreType, <i32 as Lift>::CoreType>(ctx.as_context_mut(), "wasmi-component:examples/funcs@0.1.0#add-s32").unwrap();
    let cleanup_func = instance.get_typed_func::<i32, ()>(ctx.as_context_mut(), "cabi_post_wasmi-component:examples/funcs@0.1.0#add-s32").ok();
    let funcs_add_s32 = TypedFunc::new(module_func, cleanup_func);

    let module_func = instance.get_typed_func::<<() as LowerList>::CoreType, <String as Lift>::CoreType>(ctx.as_context_mut(), "wasmi-component:examples/funcs@0.1.0#get-name").unwrap();
    let cleanup_func = instance.get_typed_func::<i32, ()>(ctx.as_context_mut(), "cabi_post_wasmi-component:examples/funcs@0.1.0#get-name").ok();
    let funcs_get_name = TypedFunc::new(module_func, cleanup_func);

    let module_func = instance.get_typed_func::<<(u32, u32, ) as LowerList>::CoreType, <u32 as Lift>::CoreType>(ctx.as_context_mut(), "add-u32").unwrap();
    let cleanup_func = instance.get_typed_func::<i32, ()>(ctx.as_context_mut(), "cabi_post_add-u32").ok();
    let add_u32 = TypedFunc::new(module_func, cleanup_func);

    let module_func = instance.get_typed_func::<<(f32, f32, ) as LowerList>::CoreType, <f32 as Lift>::CoreType>(ctx.as_context_mut(), "additional#add-f32").unwrap();
    let cleanup_func = instance.get_typed_func::<i32, ()>(ctx.as_context_mut(), "cabi_post_additional#add-f32").ok();
    let add_f32 = TypedFunc::new(module_func, cleanup_func);

    ExampleExports {
        funcs_add_s32,
        funcs_get_name,
        add_u32,
        add_f32,
    }
}
