use wasmi_component::anyhow::Result;
#[allow(unused)]
use wasmi_component::wasmi::{AsContext, AsContextMut, errors::LinkerError};
#[allow(unused)]
use wasmi_component::{
    CallResult, Component, ComponentValue, HostResult, Linker, ListAccessor, LowerValue, StoreData,
    TypedFunc,
};

#[allow(unused)]
pub trait TestExampleImports {
    fn list_s8(&mut self, value: ListAccessor<i8>) -> HostResult<Vec<i8>>;

    fn list_s16(&mut self, value: ListAccessor<i16>) -> HostResult<Vec<i16>>;

    fn list_s32(&mut self, value: ListAccessor<i32>) -> HostResult<Vec<i32>>;

    fn list_s64(&mut self, value: ListAccessor<i64>) -> HostResult<Vec<i64>>;

    fn list_u8(&mut self, value: ListAccessor<u8>) -> HostResult<Vec<u8>>;

    fn list_u16(&mut self, value: ListAccessor<u16>) -> HostResult<Vec<u16>>;

    fn list_u32(&mut self, value: ListAccessor<u32>) -> HostResult<Vec<u32>>;

    fn list_u64(&mut self, value: ListAccessor<u64>) -> HostResult<Vec<u64>>;

    fn list_f32(&mut self, value: ListAccessor<f32>) -> HostResult<Vec<f32>>;

    fn list_f64(&mut self, value: ListAccessor<f64>) -> HostResult<Vec<f64>>;

    fn list_bool(&mut self, value: ListAccessor<bool>) -> HostResult<Vec<bool>>;

    fn list_char(&mut self, value: ListAccessor<char>) -> HostResult<Vec<char>>;

    fn list_string(&mut self, value: ListAccessor<String>) -> HostResult<Vec<String>>;

    fn log(&mut self, message: &str) -> HostResult<()>;
}

#[allow(unused)]
pub struct TestExampleExports {
    pub list_s8: TypedFunc<(Vec<i8>,), Vec<i8>>,
    pub list_s16: TypedFunc<(Vec<i16>,), Vec<i16>>,
    pub list_s32: TypedFunc<(Vec<i32>,), Vec<i32>>,
    pub list_s64: TypedFunc<(Vec<i64>,), Vec<i64>>,
    pub list_u8: TypedFunc<(Vec<u8>,), Vec<u8>>,
    pub list_u16: TypedFunc<(Vec<u16>,), Vec<u16>>,
    pub list_u32: TypedFunc<(Vec<u32>,), Vec<u32>>,
    pub list_u64: TypedFunc<(Vec<u64>,), Vec<u64>>,
    pub list_f32: TypedFunc<(Vec<f32>,), Vec<f32>>,
    pub list_f64: TypedFunc<(Vec<f64>,), Vec<f64>>,
    pub list_bool: TypedFunc<(Vec<bool>,), Vec<bool>>,
    pub list_char: TypedFunc<(Vec<char>,), Vec<char>>,
    pub list_string: TypedFunc<(Vec<String>,), Vec<String>>,
}

#[allow(unused)]
impl TestExampleExports {
    pub fn call_list_s8<T>(
        &self,
        ctx: impl AsContextMut<Data = StoreData<T>>,
        value: impl LowerValue<Vec<i8>>,
    ) -> CallResult<Vec<i8>> {
        self.list_s8.call(ctx, (value,))
    }

    pub fn call_list_s8_with_results<T, R>(
        &self,
        ctx: impl AsContextMut<Data = StoreData<T>>,
        value: impl LowerValue<Vec<i8>>,
        callback: impl FnOnce(ListAccessor<i8>) -> R,
    ) -> CallResult<R> {
        self.list_s8.call_with_results(ctx, (value,), callback)
    }

    pub fn call_list_s16<T>(
        &self,
        ctx: impl AsContextMut<Data = StoreData<T>>,
        value: impl LowerValue<Vec<i16>>,
    ) -> CallResult<Vec<i16>> {
        self.list_s16.call(ctx, (value,))
    }

    pub fn call_list_s16_with_results<T, R>(
        &self,
        ctx: impl AsContextMut<Data = StoreData<T>>,
        value: impl LowerValue<Vec<i16>>,
        callback: impl FnOnce(ListAccessor<i16>) -> R,
    ) -> CallResult<R> {
        self.list_s16.call_with_results(ctx, (value,), callback)
    }

    pub fn call_list_s32<T>(
        &self,
        ctx: impl AsContextMut<Data = StoreData<T>>,
        value: impl LowerValue<Vec<i32>>,
    ) -> CallResult<Vec<i32>> {
        self.list_s32.call(ctx, (value,))
    }

    pub fn call_list_s32_with_results<T, R>(
        &self,
        ctx: impl AsContextMut<Data = StoreData<T>>,
        value: impl LowerValue<Vec<i32>>,
        callback: impl FnOnce(ListAccessor<i32>) -> R,
    ) -> CallResult<R> {
        self.list_s32.call_with_results(ctx, (value,), callback)
    }

    pub fn call_list_s64<T>(
        &self,
        ctx: impl AsContextMut<Data = StoreData<T>>,
        value: impl LowerValue<Vec<i64>>,
    ) -> CallResult<Vec<i64>> {
        self.list_s64.call(ctx, (value,))
    }

    pub fn call_list_s64_with_results<T, R>(
        &self,
        ctx: impl AsContextMut<Data = StoreData<T>>,
        value: impl LowerValue<Vec<i64>>,
        callback: impl FnOnce(ListAccessor<i64>) -> R,
    ) -> CallResult<R> {
        self.list_s64.call_with_results(ctx, (value,), callback)
    }

    pub fn call_list_u8<T>(
        &self,
        ctx: impl AsContextMut<Data = StoreData<T>>,
        value: impl LowerValue<Vec<u8>>,
    ) -> CallResult<Vec<u8>> {
        self.list_u8.call(ctx, (value,))
    }

    pub fn call_list_u8_with_results<T, R>(
        &self,
        ctx: impl AsContextMut<Data = StoreData<T>>,
        value: impl LowerValue<Vec<u8>>,
        callback: impl FnOnce(ListAccessor<u8>) -> R,
    ) -> CallResult<R> {
        self.list_u8.call_with_results(ctx, (value,), callback)
    }

    pub fn call_list_u16<T>(
        &self,
        ctx: impl AsContextMut<Data = StoreData<T>>,
        value: impl LowerValue<Vec<u16>>,
    ) -> CallResult<Vec<u16>> {
        self.list_u16.call(ctx, (value,))
    }

    pub fn call_list_u16_with_results<T, R>(
        &self,
        ctx: impl AsContextMut<Data = StoreData<T>>,
        value: impl LowerValue<Vec<u16>>,
        callback: impl FnOnce(ListAccessor<u16>) -> R,
    ) -> CallResult<R> {
        self.list_u16.call_with_results(ctx, (value,), callback)
    }

    pub fn call_list_u32<T>(
        &self,
        ctx: impl AsContextMut<Data = StoreData<T>>,
        value: impl LowerValue<Vec<u32>>,
    ) -> CallResult<Vec<u32>> {
        self.list_u32.call(ctx, (value,))
    }

    pub fn call_list_u32_with_results<T, R>(
        &self,
        ctx: impl AsContextMut<Data = StoreData<T>>,
        value: impl LowerValue<Vec<u32>>,
        callback: impl FnOnce(ListAccessor<u32>) -> R,
    ) -> CallResult<R> {
        self.list_u32.call_with_results(ctx, (value,), callback)
    }

    pub fn call_list_u64<T>(
        &self,
        ctx: impl AsContextMut<Data = StoreData<T>>,
        value: impl LowerValue<Vec<u64>>,
    ) -> CallResult<Vec<u64>> {
        self.list_u64.call(ctx, (value,))
    }

    pub fn call_list_u64_with_results<T, R>(
        &self,
        ctx: impl AsContextMut<Data = StoreData<T>>,
        value: impl LowerValue<Vec<u64>>,
        callback: impl FnOnce(ListAccessor<u64>) -> R,
    ) -> CallResult<R> {
        self.list_u64.call_with_results(ctx, (value,), callback)
    }

    pub fn call_list_f32<T>(
        &self,
        ctx: impl AsContextMut<Data = StoreData<T>>,
        value: impl LowerValue<Vec<f32>>,
    ) -> CallResult<Vec<f32>> {
        self.list_f32.call(ctx, (value,))
    }

    pub fn call_list_f32_with_results<T, R>(
        &self,
        ctx: impl AsContextMut<Data = StoreData<T>>,
        value: impl LowerValue<Vec<f32>>,
        callback: impl FnOnce(ListAccessor<f32>) -> R,
    ) -> CallResult<R> {
        self.list_f32.call_with_results(ctx, (value,), callback)
    }

    pub fn call_list_f64<T>(
        &self,
        ctx: impl AsContextMut<Data = StoreData<T>>,
        value: impl LowerValue<Vec<f64>>,
    ) -> CallResult<Vec<f64>> {
        self.list_f64.call(ctx, (value,))
    }

    pub fn call_list_f64_with_results<T, R>(
        &self,
        ctx: impl AsContextMut<Data = StoreData<T>>,
        value: impl LowerValue<Vec<f64>>,
        callback: impl FnOnce(ListAccessor<f64>) -> R,
    ) -> CallResult<R> {
        self.list_f64.call_with_results(ctx, (value,), callback)
    }

    pub fn call_list_bool<T>(
        &self,
        ctx: impl AsContextMut<Data = StoreData<T>>,
        value: impl LowerValue<Vec<bool>>,
    ) -> CallResult<Vec<bool>> {
        self.list_bool.call(ctx, (value,))
    }

    pub fn call_list_bool_with_results<T, R>(
        &self,
        ctx: impl AsContextMut<Data = StoreData<T>>,
        value: impl LowerValue<Vec<bool>>,
        callback: impl FnOnce(ListAccessor<bool>) -> R,
    ) -> CallResult<R> {
        self.list_bool.call_with_results(ctx, (value,), callback)
    }

    pub fn call_list_char<T>(
        &self,
        ctx: impl AsContextMut<Data = StoreData<T>>,
        value: impl LowerValue<Vec<char>>,
    ) -> CallResult<Vec<char>> {
        self.list_char.call(ctx, (value,))
    }

    pub fn call_list_char_with_results<T, R>(
        &self,
        ctx: impl AsContextMut<Data = StoreData<T>>,
        value: impl LowerValue<Vec<char>>,
        callback: impl FnOnce(ListAccessor<char>) -> R,
    ) -> CallResult<R> {
        self.list_char.call_with_results(ctx, (value,), callback)
    }

    pub fn call_list_string<T>(
        &self,
        ctx: impl AsContextMut<Data = StoreData<T>>,
        value: impl LowerValue<Vec<String>>,
    ) -> CallResult<Vec<String>> {
        self.list_string.call(ctx, (value,))
    }

    pub fn call_list_string_with_results<T, R>(
        &self,
        ctx: impl AsContextMut<Data = StoreData<T>>,
        value: impl LowerValue<Vec<String>>,
        callback: impl FnOnce(ListAccessor<String>) -> R,
    ) -> CallResult<R> {
        self.list_string.call_with_results(ctx, (value,), callback)
    }
}

#[allow(unused)]
pub fn add_test_example_to_linker<T: TestExampleImports>(
    linker: &mut Linker<T>,
) -> Result<(), LinkerError> {
    linker.func_new::<(Vec<i8>,), Vec<i8>>(
        "wasmi-component:component-examples/round-trip@0.1.0",
        "list-s8",
        |host_data, params| host_data.list_s8(params.0),
    )?;

    linker.func_new::<(Vec<i16>,), Vec<i16>>(
        "wasmi-component:component-examples/round-trip@0.1.0",
        "list-s16",
        |host_data, params| host_data.list_s16(params.0),
    )?;

    linker.func_new::<(Vec<i32>,), Vec<i32>>(
        "wasmi-component:component-examples/round-trip@0.1.0",
        "list-s32",
        |host_data, params| host_data.list_s32(params.0),
    )?;

    linker.func_new::<(Vec<i64>,), Vec<i64>>(
        "wasmi-component:component-examples/round-trip@0.1.0",
        "list-s64",
        |host_data, params| host_data.list_s64(params.0),
    )?;

    linker.func_new::<(Vec<u8>,), Vec<u8>>(
        "wasmi-component:component-examples/round-trip@0.1.0",
        "list-u8",
        |host_data, params| host_data.list_u8(params.0),
    )?;

    linker.func_new::<(Vec<u16>,), Vec<u16>>(
        "wasmi-component:component-examples/round-trip@0.1.0",
        "list-u16",
        |host_data, params| host_data.list_u16(params.0),
    )?;

    linker.func_new::<(Vec<u32>,), Vec<u32>>(
        "wasmi-component:component-examples/round-trip@0.1.0",
        "list-u32",
        |host_data, params| host_data.list_u32(params.0),
    )?;

    linker.func_new::<(Vec<u64>,), Vec<u64>>(
        "wasmi-component:component-examples/round-trip@0.1.0",
        "list-u64",
        |host_data, params| host_data.list_u64(params.0),
    )?;

    linker.func_new::<(Vec<f32>,), Vec<f32>>(
        "wasmi-component:component-examples/round-trip@0.1.0",
        "list-f32",
        |host_data, params| host_data.list_f32(params.0),
    )?;

    linker.func_new::<(Vec<f64>,), Vec<f64>>(
        "wasmi-component:component-examples/round-trip@0.1.0",
        "list-f64",
        |host_data, params| host_data.list_f64(params.0),
    )?;

    linker.func_new::<(Vec<bool>,), Vec<bool>>(
        "wasmi-component:component-examples/round-trip@0.1.0",
        "list-bool",
        |host_data, params| host_data.list_bool(params.0),
    )?;

    linker.func_new::<(Vec<char>,), Vec<char>>(
        "wasmi-component:component-examples/round-trip@0.1.0",
        "list-char",
        |host_data, params| host_data.list_char(params.0),
    )?;

    linker.func_new::<(Vec<String>,), Vec<String>>(
        "wasmi-component:component-examples/round-trip@0.1.0",
        "list-string",
        |host_data, params| host_data.list_string(params.0),
    )?;

    linker
        .func_new::<(String,), ()>("$root", "log", |host_data, params| host_data.log(params.0))?;

    Ok(())
}

#[allow(unused)]
pub fn instantiate_test_example_world<T>(
    mut ctx: impl AsContextMut<Data = StoreData<T>>,
    linker: &Linker<T>,
    component: &Component,
) -> Result<TestExampleExports> {
    let instance = linker.instantiate(ctx.as_context_mut(), &component)?;

    let list_s8 = instance.get_typed_func(
        ctx.as_context(),
        "wasmi-component:component-examples/round-trip@0.1.0#list-s8",
    )?;

    let list_s16 = instance.get_typed_func(
        ctx.as_context(),
        "wasmi-component:component-examples/round-trip@0.1.0#list-s16",
    )?;

    let list_s32 = instance.get_typed_func(
        ctx.as_context(),
        "wasmi-component:component-examples/round-trip@0.1.0#list-s32",
    )?;

    let list_s64 = instance.get_typed_func(
        ctx.as_context(),
        "wasmi-component:component-examples/round-trip@0.1.0#list-s64",
    )?;

    let list_u8 = instance.get_typed_func(
        ctx.as_context(),
        "wasmi-component:component-examples/round-trip@0.1.0#list-u8",
    )?;

    let list_u16 = instance.get_typed_func(
        ctx.as_context(),
        "wasmi-component:component-examples/round-trip@0.1.0#list-u16",
    )?;

    let list_u32 = instance.get_typed_func(
        ctx.as_context(),
        "wasmi-component:component-examples/round-trip@0.1.0#list-u32",
    )?;

    let list_u64 = instance.get_typed_func(
        ctx.as_context(),
        "wasmi-component:component-examples/round-trip@0.1.0#list-u64",
    )?;

    let list_f32 = instance.get_typed_func(
        ctx.as_context(),
        "wasmi-component:component-examples/round-trip@0.1.0#list-f32",
    )?;

    let list_f64 = instance.get_typed_func(
        ctx.as_context(),
        "wasmi-component:component-examples/round-trip@0.1.0#list-f64",
    )?;

    let list_bool = instance.get_typed_func(
        ctx.as_context(),
        "wasmi-component:component-examples/round-trip@0.1.0#list-bool",
    )?;

    let list_char = instance.get_typed_func(
        ctx.as_context(),
        "wasmi-component:component-examples/round-trip@0.1.0#list-char",
    )?;

    let list_string = instance.get_typed_func(
        ctx.as_context(),
        "wasmi-component:component-examples/round-trip@0.1.0#list-string",
    )?;

    Ok(TestExampleExports {
        list_s8,
        list_s16,
        list_s32,
        list_s64,
        list_u8,
        list_u16,
        list_u32,
        list_u64,
        list_f32,
        list_f64,
        list_bool,
        list_char,
        list_string,
    })
}
