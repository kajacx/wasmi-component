use wasmi_component::anyhow::Result;
#[allow(unused)]
use wasmi_component::wasmi::{AsContext, AsContextMut, errors::LinkerError};
#[allow(unused)]
use wasmi_component::{
    CallResult, Component, ComponentValue, HostResult, Instance, Linker, ListAccessor, Lower,
    StoreData, TypedFunc,
};

#[allow(unused)]
pub trait TestExampleImports {
    fn trip_s8(&mut self, value: i8) -> HostResult<i8>;

    fn trip_s16(&mut self, value: i16) -> HostResult<i16>;

    fn trip_s32(&mut self, value: i32) -> HostResult<i32>;

    fn trip_s64(&mut self, value: i64) -> HostResult<i64>;

    fn trip_u8(&mut self, value: u8) -> HostResult<u8>;

    fn trip_u16(&mut self, value: u16) -> HostResult<u16>;

    fn trip_u32(&mut self, value: u32) -> HostResult<u32>;

    fn trip_u64(&mut self, value: u64) -> HostResult<u64>;

    fn trip_f32(&mut self, value: f32) -> HostResult<f32>;

    fn trip_f64(&mut self, value: f64) -> HostResult<f64>;

    fn trip_bool(&mut self, value: bool) -> HostResult<bool>;

    fn trip_char(&mut self, value: char) -> HostResult<char>;

    fn trip_string(&mut self, value: &str) -> HostResult<String>;

    fn log(&mut self, message: &str) -> HostResult<()>;
}

#[allow(unused)]
#[derive(Clone, Debug)]
pub struct TestExampleExports {
    pub instance: Instance,
    pub trip_s8: TypedFunc<(i8,), i8>,
    pub trip_s16: TypedFunc<(i16,), i16>,
    pub trip_s32: TypedFunc<(i32,), i32>,
    pub trip_s64: TypedFunc<(i64,), i64>,
    pub trip_u8: TypedFunc<(u8,), u8>,
    pub trip_u16: TypedFunc<(u16,), u16>,
    pub trip_u32: TypedFunc<(u32,), u32>,
    pub trip_u64: TypedFunc<(u64,), u64>,
    pub trip_f32: TypedFunc<(f32,), f32>,
    pub trip_f64: TypedFunc<(f64,), f64>,
    pub trip_bool: TypedFunc<(bool,), bool>,
    pub trip_char: TypedFunc<(char,), char>,
    pub trip_string: TypedFunc<(String,), String>,
}

#[allow(unused)]
impl TestExampleExports {
    pub fn call_trip_s8<T>(
        &self,
        ctx: impl AsContextMut<Data = StoreData<T>>,
        value: i8,
    ) -> CallResult<i8> {
        self.trip_s8.call(ctx, (value,))
    }

    pub fn call_trip_s8_with_results<T, R>(
        &self,
        ctx: impl AsContextMut<Data = StoreData<T>>,
        value: i8,
        callback: impl FnOnce(&mut T, i8) -> R,
    ) -> CallResult<R> {
        self.trip_s8.call_with_results(ctx, (value,), callback)
    }

    pub fn call_trip_s16<T>(
        &self,
        ctx: impl AsContextMut<Data = StoreData<T>>,
        value: i16,
    ) -> CallResult<i16> {
        self.trip_s16.call(ctx, (value,))
    }

    pub fn call_trip_s16_with_results<T, R>(
        &self,
        ctx: impl AsContextMut<Data = StoreData<T>>,
        value: i16,
        callback: impl FnOnce(&mut T, i16) -> R,
    ) -> CallResult<R> {
        self.trip_s16.call_with_results(ctx, (value,), callback)
    }

    pub fn call_trip_s32<T>(
        &self,
        ctx: impl AsContextMut<Data = StoreData<T>>,
        value: i32,
    ) -> CallResult<i32> {
        self.trip_s32.call(ctx, (value,))
    }

    pub fn call_trip_s32_with_results<T, R>(
        &self,
        ctx: impl AsContextMut<Data = StoreData<T>>,
        value: i32,
        callback: impl FnOnce(&mut T, i32) -> R,
    ) -> CallResult<R> {
        self.trip_s32.call_with_results(ctx, (value,), callback)
    }

    pub fn call_trip_s64<T>(
        &self,
        ctx: impl AsContextMut<Data = StoreData<T>>,
        value: i64,
    ) -> CallResult<i64> {
        self.trip_s64.call(ctx, (value,))
    }

    pub fn call_trip_s64_with_results<T, R>(
        &self,
        ctx: impl AsContextMut<Data = StoreData<T>>,
        value: i64,
        callback: impl FnOnce(&mut T, i64) -> R,
    ) -> CallResult<R> {
        self.trip_s64.call_with_results(ctx, (value,), callback)
    }

    pub fn call_trip_u8<T>(
        &self,
        ctx: impl AsContextMut<Data = StoreData<T>>,
        value: u8,
    ) -> CallResult<u8> {
        self.trip_u8.call(ctx, (value,))
    }

    pub fn call_trip_u8_with_results<T, R>(
        &self,
        ctx: impl AsContextMut<Data = StoreData<T>>,
        value: u8,
        callback: impl FnOnce(&mut T, u8) -> R,
    ) -> CallResult<R> {
        self.trip_u8.call_with_results(ctx, (value,), callback)
    }

    pub fn call_trip_u16<T>(
        &self,
        ctx: impl AsContextMut<Data = StoreData<T>>,
        value: u16,
    ) -> CallResult<u16> {
        self.trip_u16.call(ctx, (value,))
    }

    pub fn call_trip_u16_with_results<T, R>(
        &self,
        ctx: impl AsContextMut<Data = StoreData<T>>,
        value: u16,
        callback: impl FnOnce(&mut T, u16) -> R,
    ) -> CallResult<R> {
        self.trip_u16.call_with_results(ctx, (value,), callback)
    }

    pub fn call_trip_u32<T>(
        &self,
        ctx: impl AsContextMut<Data = StoreData<T>>,
        value: u32,
    ) -> CallResult<u32> {
        self.trip_u32.call(ctx, (value,))
    }

    pub fn call_trip_u32_with_results<T, R>(
        &self,
        ctx: impl AsContextMut<Data = StoreData<T>>,
        value: u32,
        callback: impl FnOnce(&mut T, u32) -> R,
    ) -> CallResult<R> {
        self.trip_u32.call_with_results(ctx, (value,), callback)
    }

    pub fn call_trip_u64<T>(
        &self,
        ctx: impl AsContextMut<Data = StoreData<T>>,
        value: u64,
    ) -> CallResult<u64> {
        self.trip_u64.call(ctx, (value,))
    }

    pub fn call_trip_u64_with_results<T, R>(
        &self,
        ctx: impl AsContextMut<Data = StoreData<T>>,
        value: u64,
        callback: impl FnOnce(&mut T, u64) -> R,
    ) -> CallResult<R> {
        self.trip_u64.call_with_results(ctx, (value,), callback)
    }

    pub fn call_trip_f32<T>(
        &self,
        ctx: impl AsContextMut<Data = StoreData<T>>,
        value: f32,
    ) -> CallResult<f32> {
        self.trip_f32.call(ctx, (value,))
    }

    pub fn call_trip_f32_with_results<T, R>(
        &self,
        ctx: impl AsContextMut<Data = StoreData<T>>,
        value: f32,
        callback: impl FnOnce(&mut T, f32) -> R,
    ) -> CallResult<R> {
        self.trip_f32.call_with_results(ctx, (value,), callback)
    }

    pub fn call_trip_f64<T>(
        &self,
        ctx: impl AsContextMut<Data = StoreData<T>>,
        value: f64,
    ) -> CallResult<f64> {
        self.trip_f64.call(ctx, (value,))
    }

    pub fn call_trip_f64_with_results<T, R>(
        &self,
        ctx: impl AsContextMut<Data = StoreData<T>>,
        value: f64,
        callback: impl FnOnce(&mut T, f64) -> R,
    ) -> CallResult<R> {
        self.trip_f64.call_with_results(ctx, (value,), callback)
    }

    pub fn call_trip_bool<T>(
        &self,
        ctx: impl AsContextMut<Data = StoreData<T>>,
        value: bool,
    ) -> CallResult<bool> {
        self.trip_bool.call(ctx, (value,))
    }

    pub fn call_trip_bool_with_results<T, R>(
        &self,
        ctx: impl AsContextMut<Data = StoreData<T>>,
        value: bool,
        callback: impl FnOnce(&mut T, bool) -> R,
    ) -> CallResult<R> {
        self.trip_bool.call_with_results(ctx, (value,), callback)
    }

    pub fn call_trip_char<T>(
        &self,
        ctx: impl AsContextMut<Data = StoreData<T>>,
        value: char,
    ) -> CallResult<char> {
        self.trip_char.call(ctx, (value,))
    }

    pub fn call_trip_char_with_results<T, R>(
        &self,
        ctx: impl AsContextMut<Data = StoreData<T>>,
        value: char,
        callback: impl FnOnce(&mut T, char) -> R,
    ) -> CallResult<R> {
        self.trip_char.call_with_results(ctx, (value,), callback)
    }

    pub fn call_trip_string<T>(
        &self,
        ctx: impl AsContextMut<Data = StoreData<T>>,
        value: &str,
    ) -> CallResult<String> {
        self.trip_string.call(ctx, (value,))
    }

    pub fn call_trip_string_with_results<T, R>(
        &self,
        ctx: impl AsContextMut<Data = StoreData<T>>,
        value: &str,
        callback: impl FnOnce(&mut T, &str) -> R,
    ) -> CallResult<R> {
        self.trip_string.call_with_results(ctx, (value,), callback)
    }
}

#[allow(unused)]
pub fn add_test_example_to_linker<T: TestExampleImports>(
    linker: &mut Linker<T>,
) -> Result<(), LinkerError> {
    linker.func_new::<(i8,), i8>(
        "wasmi-component:component-examples/round-trip@0.1.0",
        "trip-s8",
        |host_data, params| host_data.trip_s8(params.0),
    )?;

    linker.func_new::<(i16,), i16>(
        "wasmi-component:component-examples/round-trip@0.1.0",
        "trip-s16",
        |host_data, params| host_data.trip_s16(params.0),
    )?;

    linker.func_new::<(i32,), i32>(
        "wasmi-component:component-examples/round-trip@0.1.0",
        "trip-s32",
        |host_data, params| host_data.trip_s32(params.0),
    )?;

    linker.func_new::<(i64,), i64>(
        "wasmi-component:component-examples/round-trip@0.1.0",
        "trip-s64",
        |host_data, params| host_data.trip_s64(params.0),
    )?;

    linker.func_new::<(u8,), u8>(
        "wasmi-component:component-examples/round-trip@0.1.0",
        "trip-u8",
        |host_data, params| host_data.trip_u8(params.0),
    )?;

    linker.func_new::<(u16,), u16>(
        "wasmi-component:component-examples/round-trip@0.1.0",
        "trip-u16",
        |host_data, params| host_data.trip_u16(params.0),
    )?;

    linker.func_new::<(u32,), u32>(
        "wasmi-component:component-examples/round-trip@0.1.0",
        "trip-u32",
        |host_data, params| host_data.trip_u32(params.0),
    )?;

    linker.func_new::<(u64,), u64>(
        "wasmi-component:component-examples/round-trip@0.1.0",
        "trip-u64",
        |host_data, params| host_data.trip_u64(params.0),
    )?;

    linker.func_new::<(f32,), f32>(
        "wasmi-component:component-examples/round-trip@0.1.0",
        "trip-f32",
        |host_data, params| host_data.trip_f32(params.0),
    )?;

    linker.func_new::<(f64,), f64>(
        "wasmi-component:component-examples/round-trip@0.1.0",
        "trip-f64",
        |host_data, params| host_data.trip_f64(params.0),
    )?;

    linker.func_new::<(bool,), bool>(
        "wasmi-component:component-examples/round-trip@0.1.0",
        "trip-bool",
        |host_data, params| host_data.trip_bool(params.0),
    )?;

    linker.func_new::<(char,), char>(
        "wasmi-component:component-examples/round-trip@0.1.0",
        "trip-char",
        |host_data, params| host_data.trip_char(params.0),
    )?;

    linker.func_new::<(String,), String>(
        "wasmi-component:component-examples/round-trip@0.1.0",
        "trip-string",
        |host_data, params| host_data.trip_string(params.0),
    )?;

    linker.func_new::<(String,), ()>("", "log", |host_data, params| host_data.log(params.0))?;

    Ok(())
}

#[allow(unused)]
pub fn instantiate_test_example_world<T>(
    mut ctx: impl AsContextMut<Data = StoreData<T>>,
    linker: &Linker<T>,
    component: &Component,
) -> Result<TestExampleExports> {
    let instance = linker.instantiate(ctx.as_context_mut(), &component)?;

    let trip_s8 = instance.get_typed_func(
        ctx.as_context(),
        "wasmi-component:component-examples/round-trip@0.1.0",
        "trip-s8",
    )?;

    let trip_s16 = instance.get_typed_func(
        ctx.as_context(),
        "wasmi-component:component-examples/round-trip@0.1.0",
        "trip-s16",
    )?;

    let trip_s32 = instance.get_typed_func(
        ctx.as_context(),
        "wasmi-component:component-examples/round-trip@0.1.0",
        "trip-s32",
    )?;

    let trip_s64 = instance.get_typed_func(
        ctx.as_context(),
        "wasmi-component:component-examples/round-trip@0.1.0",
        "trip-s64",
    )?;

    let trip_u8 = instance.get_typed_func(
        ctx.as_context(),
        "wasmi-component:component-examples/round-trip@0.1.0",
        "trip-u8",
    )?;

    let trip_u16 = instance.get_typed_func(
        ctx.as_context(),
        "wasmi-component:component-examples/round-trip@0.1.0",
        "trip-u16",
    )?;

    let trip_u32 = instance.get_typed_func(
        ctx.as_context(),
        "wasmi-component:component-examples/round-trip@0.1.0",
        "trip-u32",
    )?;

    let trip_u64 = instance.get_typed_func(
        ctx.as_context(),
        "wasmi-component:component-examples/round-trip@0.1.0",
        "trip-u64",
    )?;

    let trip_f32 = instance.get_typed_func(
        ctx.as_context(),
        "wasmi-component:component-examples/round-trip@0.1.0",
        "trip-f32",
    )?;

    let trip_f64 = instance.get_typed_func(
        ctx.as_context(),
        "wasmi-component:component-examples/round-trip@0.1.0",
        "trip-f64",
    )?;

    let trip_bool = instance.get_typed_func(
        ctx.as_context(),
        "wasmi-component:component-examples/round-trip@0.1.0",
        "trip-bool",
    )?;

    let trip_char = instance.get_typed_func(
        ctx.as_context(),
        "wasmi-component:component-examples/round-trip@0.1.0",
        "trip-char",
    )?;

    let trip_string = instance.get_typed_func(
        ctx.as_context(),
        "wasmi-component:component-examples/round-trip@0.1.0",
        "trip-string",
    )?;

    Ok(TestExampleExports {
        instance,
        trip_s8,
        trip_s16,
        trip_s32,
        trip_s64,
        trip_u8,
        trip_u16,
        trip_u32,
        trip_u64,
        trip_f32,
        trip_f64,
        trip_bool,
        trip_char,
        trip_string,
    })
}
