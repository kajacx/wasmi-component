#[allow(unused)]
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
}

#[allow(unused)]
#[derive(Debug, Clone, PartialEq, PartialOrd, ComponentValue)]
pub enum Data {
    Number(f64),
    Text(String),
}

#[allow(unused)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, ComponentValue)]
#[component_value_attrs(copy)]
pub enum Status {
    Ok,
    Error,
}

#[allow(unused)]
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, ComponentValue)]
#[component_value_attrs(copy)]
pub enum Outcome {
    Ok,
    Error(i32),
}

#[allow(unused)]
#[derive(Debug, Clone, Default, PartialEq, PartialOrd, ComponentValue)]
pub struct Fruit {
    pub kind: String,
    pub weight: f32,
}

#[allow(unused)]
#[derive(Debug, Clone, Default, PartialEq, PartialOrd, ComponentValue)]
pub struct Animal {
    pub species: String,
    pub age: u32,
}

#[allow(unused)]
pub trait TestExampleImports {
    fn trip_person(&mut self, value: PersonBorrowed<'_>) -> HostResult<Person>;

    fn trip_data(&mut self, value: DataBorrowed<'_>) -> HostResult<Data>;

    fn trip_status(&mut self, value: Status) -> HostResult<Status>;

    fn trip_mixed(
        &mut self,
        a: PersonBorrowed<'_>,
        b: i32,
        c: Result<DataBorrowed<'_>, &str>,
    ) -> HostResult<()>;

    fn price(&mut self, item: ListAccessor<'_, (Fruit, u32)>) -> HostResult<f32>;

    fn log(&mut self, message: &str) -> HostResult<()>;
}

#[allow(unused)]
#[derive(Clone, Debug)]
pub struct TestExampleExports {
    pub instance: Instance,
    pub init: TypedFunc<(Vec<String>,), Outcome>,
    pub trip_person: TypedFunc<(Person,), Person>,
    pub trip_data: TypedFunc<(Data,), Data>,
    pub trip_status: TypedFunc<(Status,), Status>,
    pub trip_mixed: TypedFunc<(Person, i32, Result<Data, String>), ()>,
    pub pet: TypedFunc<(Animal, u32), Result<(), String>>,
}

#[allow(unused)]
impl TestExampleExports {
    pub fn call_init<T>(
        &self,
        ctx: impl AsContextMut<Data = StoreData<T>>,
        args: impl Lower<Vec<String>>,
    ) -> CallResult<Outcome> {
        self.init.call(ctx, (args,))
    }

    pub fn call_init_with_results<T, R>(
        &self,
        ctx: impl AsContextMut<Data = StoreData<T>>,
        args: impl Lower<Vec<String>>,
        callback: impl FnOnce(&mut T, Outcome) -> R,
    ) -> CallResult<R> {
        self.init.call_with_results(ctx, (args,), callback)
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

    pub fn call_trip_data<T>(
        &self,
        ctx: impl AsContextMut<Data = StoreData<T>>,
        value: &Data,
    ) -> CallResult<Data> {
        self.trip_data.call(ctx, (value,))
    }

    pub fn call_trip_data_with_results<T, R>(
        &self,
        ctx: impl AsContextMut<Data = StoreData<T>>,
        value: &Data,
        callback: impl FnOnce(&mut T, DataBorrowed<'_>) -> R,
    ) -> CallResult<R> {
        self.trip_data.call_with_results(ctx, (value,), callback)
    }

    pub fn call_trip_status<T>(
        &self,
        ctx: impl AsContextMut<Data = StoreData<T>>,
        value: Status,
    ) -> CallResult<Status> {
        self.trip_status.call(ctx, (value,))
    }

    pub fn call_trip_status_with_results<T, R>(
        &self,
        ctx: impl AsContextMut<Data = StoreData<T>>,
        value: Status,
        callback: impl FnOnce(&mut T, Status) -> R,
    ) -> CallResult<R> {
        self.trip_status.call_with_results(ctx, (value,), callback)
    }

    pub fn call_trip_mixed<T>(
        &self,
        ctx: impl AsContextMut<Data = StoreData<T>>,
        a: &Person,
        b: i32,
        c: impl Lower<Result<Data, String>>,
    ) -> CallResult<()> {
        self.trip_mixed.call(ctx, (a, b, c))
    }

    pub fn call_trip_mixed_with_results<T, R>(
        &self,
        ctx: impl AsContextMut<Data = StoreData<T>>,
        a: &Person,
        b: i32,
        c: impl Lower<Result<Data, String>>,
        callback: impl FnOnce(&mut T, ()) -> R,
    ) -> CallResult<R> {
        self.trip_mixed.call_with_results(ctx, (a, b, c), callback)
    }

    pub fn call_pet<T>(
        &self,
        ctx: impl AsContextMut<Data = StoreData<T>>,
        target: &Animal,
        pets: u32,
    ) -> CallResult<Result<(), String>> {
        self.pet.call(ctx, (target, pets))
    }

    pub fn call_pet_with_results<T, R>(
        &self,
        ctx: impl AsContextMut<Data = StoreData<T>>,
        target: &Animal,
        pets: u32,
        callback: impl FnOnce(&mut T, Result<(), &str>) -> R,
    ) -> CallResult<R> {
        self.pet.call_with_results(ctx, (target, pets), callback)
    }
}

#[allow(unused)]
pub fn add_test_example_to_linker<T: TestExampleImports>(
    linker: &mut Linker<T>,
) -> Result<(), LinkerError> {
    linker.func_typed::<(Person,), Person>(
        "wasmi-component:component-examples/round-trip@0.1.0",
        "trip-person",
        |host_data, params| host_data.trip_person(params.0),
    )?;

    linker.func_typed::<(Data,), Data>(
        "wasmi-component:component-examples/round-trip@0.1.0",
        "trip-data",
        |host_data, params| host_data.trip_data(params.0),
    )?;

    linker.func_typed::<(Status,), Status>(
        "wasmi-component:component-examples/round-trip@0.1.0",
        "trip-status",
        |host_data, params| host_data.trip_status(params.0),
    )?;

    linker.func_typed::<(Person, i32, Result<Data, String>), ()>(
        "wasmi-component:component-examples/round-trip@0.1.0",
        "trip-mixed",
        |host_data, params| host_data.trip_mixed(params.0, params.1, params.2),
    )?;

    linker.func_typed::<(Vec<(Fruit, u32)>,), f32>(
        "additional-imports",
        "price",
        |host_data, params| host_data.price(params.0),
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

    let init = instance.get_typed_func(ctx.as_context(), "", "init")?;

    let trip_person = instance.get_typed_func(
        ctx.as_context(),
        "wasmi-component:component-examples/round-trip@0.1.0",
        "trip-person",
    )?;

    let trip_data = instance.get_typed_func(
        ctx.as_context(),
        "wasmi-component:component-examples/round-trip@0.1.0",
        "trip-data",
    )?;

    let trip_status = instance.get_typed_func(
        ctx.as_context(),
        "wasmi-component:component-examples/round-trip@0.1.0",
        "trip-status",
    )?;

    let trip_mixed = instance.get_typed_func(
        ctx.as_context(),
        "wasmi-component:component-examples/round-trip@0.1.0",
        "trip-mixed",
    )?;

    let pet = instance.get_typed_func(ctx.as_context(), "additional-exports", "pet")?;

    Ok(TestExampleExports {
        instance,
        init,
        trip_person,
        trip_data,
        trip_status,
        trip_mixed,
        pet,
    })
}
