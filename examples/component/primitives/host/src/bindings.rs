use wasmi_component::anyhow::{Context, Result};
use wasmi_component::wasmi::{AsContextMut, Linker};
use wasmi_component::{Component, MemoryAccessPre, TypedFunc};

#[allow(unused)]
pub struct ExampleExports {
    pub funcs_add_s32: TypedFunc<(i32, i32, ), i32>,
    pub funcs_greet: TypedFunc<String, String>,
    pub add_u32: TypedFunc<(u32, u32, ), u32>,
    pub add_f32: TypedFunc<(f32, f32, ), f32>,
}

pub fn instantiate_example_world(mut ctx: impl AsContextMut, component: &Component) -> Result<ExampleExports> {
    let linker = Linker::new(ctx.as_context().engine());
    let instance = linker.instantiate_and_start(ctx.as_context_mut(), &component.core_module)?;

    let memory = instance.get_memory(ctx.as_context(), "memory").context("get memory")?;
    let cabi_realloc = instance.get_typed_func::<(i32, i32, i32, i32), i32>(ctx.as_context_mut(), "cabi_realloc")?;
    let memory_pre = MemoryAccessPre::new(memory, cabi_realloc);

    let module_func = instance.get_func(ctx.as_context_mut(), "wasmi-component:examples/funcs@0.1.0#add-s32").unwrap();
    let cleanup_func = instance.get_typed_func::<i32, ()>(ctx.as_context_mut(), "cabi_post_wasmi-component:examples/funcs@0.1.0#add-s32").ok();
    let funcs_add_s32 = TypedFunc::new(memory_pre.clone(), module_func, cleanup_func);

    let module_func = instance.get_func(ctx.as_context_mut(), "wasmi-component:examples/funcs@0.1.0#greet").unwrap();
    let cleanup_func = instance.get_typed_func::<i32, ()>(ctx.as_context_mut(), "cabi_post_wasmi-component:examples/funcs@0.1.0#greet").ok();
    let funcs_greet = TypedFunc::new(memory_pre.clone(), module_func, cleanup_func);

    let module_func = instance.get_func(ctx.as_context_mut(), "add-u32").unwrap();
    let cleanup_func = instance.get_typed_func::<i32, ()>(ctx.as_context_mut(), "cabi_post_add-u32").ok();
    let add_u32 = TypedFunc::new(memory_pre.clone(), module_func, cleanup_func);

    let module_func = instance.get_func(ctx.as_context_mut(), "additional#add-f32").unwrap();
    let cleanup_func = instance.get_typed_func::<i32, ()>(ctx.as_context_mut(), "cabi_post_additional#add-f32").ok();
    let add_f32 = TypedFunc::new(memory_pre.clone(), module_func, cleanup_func);

    Ok(ExampleExports {
        funcs_add_s32,
        funcs_greet,
        add_u32,
        add_f32,
    })
}
