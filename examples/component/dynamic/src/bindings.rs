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

    fn trip_person(&mut self, value: PersonBorrowed) -> HostResult<Person>;

    fn list_s32(&mut self, value: ListAccessor<i32>) -> HostResult<Vec<i32>>;

    fn list_string(&mut self, value: ListAccessor<String>) -> HostResult<Vec<String>>;

    fn list_person(&mut self, value: ListAccessor<Person>) -> HostResult<Vec<Person>>;

    fn result_s32(&mut self, value: Result<i32, i32>) -> HostResult<Result<i32, i32>>;

    fn result_string(&mut self, value: Result<&str, &str>) -> HostResult<Result<String, String>>;

    fn result_person(
        &mut self,
        value: Result<PersonBorrowed, PersonBorrowed>,
    ) -> HostResult<Result<Person, Person>>;

    fn log(&mut self, message: &str) -> HostResult<()>;
}

#[allow(unused)]
#[derive(Clone, Debug)]
pub struct TestExampleExports {
    pub instance: Instance,
    pub trip_s32: TypedFunc<(i32,), i32>,
    pub trip_string: TypedFunc<(String,), String>,
    pub trip_person: TypedFunc<(Person,), Person>,
    pub list_s32: TypedFunc<(Vec<i32>,), Vec<i32>>,
    pub list_string: TypedFunc<(Vec<String>,), Vec<String>>,
    pub list_person: TypedFunc<(Vec<Person>,), Vec<Person>>,
    pub result_s32: TypedFunc<(Result<i32, i32>,), Result<i32, i32>>,
    pub result_string: TypedFunc<(Result<String, String>,), Result<String, String>>,
    pub result_person: TypedFunc<(Result<Person, Person>,), Result<Person, Person>>,
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
        callback: impl FnOnce(&mut T, PersonBorrowed) -> R,
    ) -> CallResult<R> {
        self.trip_person.call_with_results(ctx, (value,), callback)
    }

    pub fn call_list_s32<T>(
        &self,
        ctx: impl AsContextMut<Data = StoreData<T>>,
        value: impl Lower<Vec<i32>>,
    ) -> CallResult<Vec<i32>> {
        self.list_s32.call(ctx, (value,))
    }

    pub fn call_list_s32_with_results<T, R>(
        &self,
        ctx: impl AsContextMut<Data = StoreData<T>>,
        value: impl Lower<Vec<i32>>,
        callback: impl FnOnce(&mut T, ListAccessor<i32>) -> R,
    ) -> CallResult<R> {
        self.list_s32.call_with_results(ctx, (value,), callback)
    }

    pub fn call_list_string<T>(
        &self,
        ctx: impl AsContextMut<Data = StoreData<T>>,
        value: impl Lower<Vec<String>>,
    ) -> CallResult<Vec<String>> {
        self.list_string.call(ctx, (value,))
    }

    pub fn call_list_string_with_results<T, R>(
        &self,
        ctx: impl AsContextMut<Data = StoreData<T>>,
        value: impl Lower<Vec<String>>,
        callback: impl FnOnce(&mut T, ListAccessor<String>) -> R,
    ) -> CallResult<R> {
        self.list_string.call_with_results(ctx, (value,), callback)
    }

    pub fn call_list_person<T>(
        &self,
        ctx: impl AsContextMut<Data = StoreData<T>>,
        value: impl Lower<Vec<Person>>,
    ) -> CallResult<Vec<Person>> {
        self.list_person.call(ctx, (value,))
    }

    pub fn call_list_person_with_results<T, R>(
        &self,
        ctx: impl AsContextMut<Data = StoreData<T>>,
        value: impl Lower<Vec<Person>>,
        callback: impl FnOnce(&mut T, ListAccessor<Person>) -> R,
    ) -> CallResult<R> {
        self.list_person.call_with_results(ctx, (value,), callback)
    }

    pub fn call_result_s32<T>(
        &self,
        ctx: impl AsContextMut<Data = StoreData<T>>,
        value: impl Lower<Result<i32, i32>>,
    ) -> CallResult<Result<i32, i32>> {
        self.result_s32.call(ctx, (value,))
    }

    pub fn call_result_s32_with_results<T, R>(
        &self,
        ctx: impl AsContextMut<Data = StoreData<T>>,
        value: impl Lower<Result<i32, i32>>,
        callback: impl FnOnce(&mut T, Result<i32, i32>) -> R,
    ) -> CallResult<R> {
        self.result_s32.call_with_results(ctx, (value,), callback)
    }

    pub fn call_result_string<T>(
        &self,
        ctx: impl AsContextMut<Data = StoreData<T>>,
        value: impl Lower<Result<String, String>>,
    ) -> CallResult<Result<String, String>> {
        self.result_string.call(ctx, (value,))
    }

    pub fn call_result_string_with_results<T, R>(
        &self,
        ctx: impl AsContextMut<Data = StoreData<T>>,
        value: impl Lower<Result<String, String>>,
        callback: impl FnOnce(&mut T, Result<&str, &str>) -> R,
    ) -> CallResult<R> {
        self.result_string
            .call_with_results(ctx, (value,), callback)
    }

    pub fn call_result_person<T>(
        &self,
        ctx: impl AsContextMut<Data = StoreData<T>>,
        value: impl Lower<Result<Person, Person>>,
    ) -> CallResult<Result<Person, Person>> {
        self.result_person.call(ctx, (value,))
    }

    pub fn call_result_person_with_results<T, R>(
        &self,
        ctx: impl AsContextMut<Data = StoreData<T>>,
        value: impl Lower<Result<Person, Person>>,
        callback: impl FnOnce(&mut T, Result<PersonBorrowed, PersonBorrowed>) -> R,
    ) -> CallResult<R> {
        self.result_person
            .call_with_results(ctx, (value,), callback)
    }
}

#[allow(unused)]
pub fn add_test_example_to_linker<T: TestExampleImports>(
    linker: &mut Linker<T>,
) -> Result<(), LinkerError> {
    linker.func_new::<(i32,), i32>(
        "wasmi-component:component-examples/round-trip@0.1.0",
        "trip-s32",
        |host_data, params| host_data.trip_s32(params.0),
    )?;

    linker.func_new::<(String,), String>(
        "wasmi-component:component-examples/round-trip@0.1.0",
        "trip-string",
        |host_data, params| host_data.trip_string(params.0),
    )?;

    linker.func_new::<(Person,), Person>(
        "wasmi-component:component-examples/round-trip@0.1.0",
        "trip-person",
        |host_data, params| host_data.trip_person(params.0),
    )?;

    linker.func_new::<(Vec<i32>,), Vec<i32>>(
        "wasmi-component:component-examples/round-trip@0.1.0",
        "list-s32",
        |host_data, params| host_data.list_s32(params.0),
    )?;

    linker.func_new::<(Vec<String>,), Vec<String>>(
        "wasmi-component:component-examples/round-trip@0.1.0",
        "list-string",
        |host_data, params| host_data.list_string(params.0),
    )?;

    linker.func_new::<(Vec<Person>,), Vec<Person>>(
        "wasmi-component:component-examples/round-trip@0.1.0",
        "list-person",
        |host_data, params| host_data.list_person(params.0),
    )?;

    linker.func_new::<(Result<i32, i32>,), Result<i32, i32>>(
        "wasmi-component:component-examples/round-trip@0.1.0",
        "result-s32",
        |host_data, params| host_data.result_s32(params.0),
    )?;

    linker.func_new::<(Result<String, String>,), Result<String, String>>(
        "wasmi-component:component-examples/round-trip@0.1.0",
        "result-string",
        |host_data, params| host_data.result_string(params.0),
    )?;

    linker.func_new::<(Result<Person, Person>,), Result<Person, Person>>(
        "wasmi-component:component-examples/round-trip@0.1.0",
        "result-person",
        |host_data, params| host_data.result_person(params.0),
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

    let list_s32 = instance.get_typed_func(
        ctx.as_context(),
        "wasmi-component:component-examples/round-trip@0.1.0",
        "list-s32",
    )?;

    let list_string = instance.get_typed_func(
        ctx.as_context(),
        "wasmi-component:component-examples/round-trip@0.1.0",
        "list-string",
    )?;

    let list_person = instance.get_typed_func(
        ctx.as_context(),
        "wasmi-component:component-examples/round-trip@0.1.0",
        "list-person",
    )?;

    let result_s32 = instance.get_typed_func(
        ctx.as_context(),
        "wasmi-component:component-examples/round-trip@0.1.0",
        "result-s32",
    )?;

    let result_string = instance.get_typed_func(
        ctx.as_context(),
        "wasmi-component:component-examples/round-trip@0.1.0",
        "result-string",
    )?;

    let result_person = instance.get_typed_func(
        ctx.as_context(),
        "wasmi-component:component-examples/round-trip@0.1.0",
        "result-person",
    )?;

    Ok(TestExampleExports {
        instance,
        trip_s32,
        trip_string,
        trip_person,
        list_s32,
        list_string,
        list_person,
        result_s32,
        result_string,
        result_person,
    })
}
