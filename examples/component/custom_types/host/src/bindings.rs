use wasmi_component::anyhow::Result;
#[allow(unused)]
use wasmi_component::wasmi::{AsContext, AsContextMut, errors::LinkerError};
#[allow(unused)]
use wasmi_component::{
    CallResult, Component, ComponentValue, HostResult, Linker, ListAccessor, LowerValue, StoreData,
    TypedFunc,
};

#[allow(unused)]
#[derive(Debug, Clone, PartialEq, PartialOrd, ComponentValue)]
pub struct Person {
    pub id: u64,
    pub name: String,
}

#[allow(unused)]
#[derive(Debug, Clone, PartialEq, PartialOrd, ComponentValue)]
pub enum Data {
    Number(f64),
    Text(String),
}

#[allow(unused)]
pub trait TestExampleImports {
    fn trip_person(&mut self, value: Person) -> HostResult<Person>;

    fn trip_data(&mut self, value: Data) -> HostResult<Data>;

    fn log(&mut self, message: &str) -> HostResult<()>;
}

#[allow(unused)]
pub struct TestExampleExports {
    pub trip_person: TypedFunc<(Person,), Person>,
    pub trip_data: TypedFunc<(Data,), Data>,
}

#[allow(unused)]
impl TestExampleExports {
    pub fn call_trip_person<T>(
        &self,
        ctx: impl AsContextMut<Data = StoreData<T>>,
        value: Person,
    ) -> CallResult<Person> {
        self.trip_person.call(ctx, (value,))
    }

    pub fn call_trip_person_with_results<T, R>(
        &self,
        ctx: impl AsContextMut<Data = StoreData<T>>,
        value: Person,
        callback: impl FnOnce(&mut T, Person) -> R,
    ) -> CallResult<R> {
        self.trip_person.call_with_results(ctx, (value,), callback)
    }

    pub fn call_trip_data<T>(
        &self,
        ctx: impl AsContextMut<Data = StoreData<T>>,
        value: Data,
    ) -> CallResult<Data> {
        self.trip_data.call(ctx, (value,))
    }

    pub fn call_trip_data_with_results<T, R>(
        &self,
        ctx: impl AsContextMut<Data = StoreData<T>>,
        value: Data,
        callback: impl FnOnce(&mut T, Data) -> R,
    ) -> CallResult<R> {
        self.trip_data.call_with_results(ctx, (value,), callback)
    }
}

#[allow(unused)]
pub fn add_test_example_to_linker<T: TestExampleImports>(
    linker: &mut Linker<T>,
) -> Result<(), LinkerError> {
    linker.func_new::<(Person,), Person>(
        "wasmi-component:component-examples/round-trip@0.1.0",
        "trip-person",
        |host_data, params| host_data.trip_person(params.0),
    )?;

    linker.func_new::<(Data,), Data>(
        "wasmi-component:component-examples/round-trip@0.1.0",
        "trip-data",
        |host_data, params| host_data.trip_data(params.0),
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

    let trip_person = instance.get_typed_func(
        ctx.as_context(),
        "wasmi-component:component-examples/round-trip@0.1.0#trip-person",
    )?;

    let trip_data = instance.get_typed_func(
        ctx.as_context(),
        "wasmi-component:component-examples/round-trip@0.1.0#trip-data",
    )?;

    Ok(TestExampleExports {
        trip_person,
        trip_data,
    })
}
