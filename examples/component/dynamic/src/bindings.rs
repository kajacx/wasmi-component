use wasmi_component::anyhow::Result;
#[allow(unused)]
use wasmi_component::wasmi::{AsContext, AsContextMut, errors::LinkerError};
#[allow(unused)]
use wasmi_component::{
    CallResult, Component, ComponentValue, HostResult, Instance, Linker, ListAccessor, Lower,
    StoreData, TypedFunc,
};

#[allow(unused)]
#[derive(Debug, Clone, Default, PartialEq, PartialOrd, ComponentValue)]
pub struct Person {
    pub id: u64,
    pub name: String,
    pub birthday: Option<String>,
}

#[allow(unused)]
pub trait TestExampleImports {
    fn trip_s32(&mut self, value: i32) -> HostResult<i32>;

    fn trip_string(&mut self, value: &str) -> HostResult<String>;

    fn trip_person(&mut self, value: PersonBorrowed<'_>) -> HostResult<Person>;

    fn log(&mut self, message: &str) -> HostResult<()>;
}

#[allow(unused)]
#[derive(Clone, Debug)]
pub struct TestExampleExports {
    pub instance: Instance,
    pub trip_s32: TypedFunc<(i32,), i32>,
    pub trip_string: TypedFunc<(String,), String>,
    pub trip_person: TypedFunc<(Person,), Person>,
}

#[allow(unused)]
impl TestExampleExports {
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

    pub fn call_trip_person<T>(
        &self,
        ctx: impl AsContextMut<Data = StoreData<T>>,
        value: &Person,
    ) -> CallResult<Person> {
        self.trip_person.call(ctx, (value,))
    }

    pub fn call_trip_person_with_results<T, R>(
        &self,
        ctx: impl AsContextMut<Data = StoreData<T>>,
        value: &Person,
        callback: impl FnOnce(&mut T, PersonBorrowed<'_>) -> R,
    ) -> CallResult<R> {
        self.trip_person.call_with_results(ctx, (value,), callback)
    }
}

#[allow(unused)]
pub fn add_test_example_to_linker<T: TestExampleImports>(
    linker: &mut Linker<T>,
) -> Result<(), LinkerError> {
    linker.func_typed::<(i32,), i32>(
        "wasmi-component:component-examples/round-trip@0.1.0",
        "trip-s32",
        |host_data, params| host_data.trip_s32(params.0),
    )?;

    linker.func_typed::<(String,), String>(
        "wasmi-component:component-examples/round-trip@0.1.0",
        "trip-string",
        |host_data, params| host_data.trip_string(params.0),
    )?;

    linker.func_typed::<(Person,), Person>(
        "wasmi-component:component-examples/round-trip@0.1.0",
        "trip-person",
        |host_data, params| host_data.trip_person(params.0),
    )?;

    linker.func_typed::<(String,), ()>("", "log", |host_data, params| host_data.log(params.0))?;

    Ok(())
}

#[allow(unused)]
pub fn instantiate_test_example_world<T>(
    mut ctx: impl AsContextMut<Data = StoreData<T>>,
    linker: &Linker<T>,
    component: &Component,
) -> Result<TestExampleExports> {
    let instance = linker.instantiate(ctx.as_context_mut(), &component)?;

    let trip_s32 = instance.get_typed_func(
        ctx.as_context(),
        "wasmi-component:component-examples/round-trip@0.1.0",
        "trip-s32",
    )?;

    let trip_string = instance.get_typed_func(
        ctx.as_context(),
        "wasmi-component:component-examples/round-trip@0.1.0",
        "trip-string",
    )?;

    let trip_person = instance.get_typed_func(
        ctx.as_context(),
        "wasmi-component:component-examples/round-trip@0.1.0",
        "trip-person",
    )?;

    Ok(TestExampleExports {
        instance,
        trip_s32,
        trip_string,
        trip_person,
    })
}
