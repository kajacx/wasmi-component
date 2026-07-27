use wasmi_component::anyhow::Result;
#[allow(unused)]
use wasmi_component::wasmi::{AsContext, AsContextMut, errors::LinkerError};
#[allow(unused)]
use wasmi_component::{
    CallResult, Component, ComponentValue, HostResult, Instance, Linker, ListAccessor, Lower,
    StoreData, TypedFunc,
};

#[allow(unused)]
#[derive(Debug, Clone, Default, PartialEq, PartialOrd)]
pub struct Person {
    pub id: u64,
    pub name: String,
    pub birthday: Option<String>,
}

impl wasmi_component::ComponentValue for Person {
    type Borrowed<'a> = PersonBorrowed<'a>;
    fn value_type() -> wasmi_component::ValueType {
        wasmi_component::ValueType::Record {
            name: std::rc::Rc::from("Person"),
            fields: std::rc::Rc::from([
                (std::rc::Rc::from("id"), <u64>::value_type()),
                (std::rc::Rc::from("name"), <String>::value_type()),
                (
                    std::rc::Rc::from("birthday"),
                    <Option<String>>::value_type(),
                ),
            ]),
        }
    }
    fn arg_count() -> usize {
        0 + <u64>::arg_count() + <String>::arg_count() + <Option<String>>::arg_count()
    }
    fn arg_types() -> Vec<wasmi_component::wasmi::ValType> {
        let mut types = Vec::new();
        types.extend(<u64>::arg_types());
        types.extend(<String>::arg_types());
        types.extend(<Option<String>>::arg_types());
        types
    }
    fn lift_args<'a>(
        args: &[wasmi_component::lib_structs::WasmValue],
        memory: &'a [u8],
    ) -> wasmi_component::ConvertResult<Self::Borrowed<'a>> {
        let mut index = 0;
        let id = <u64>::lift_args(&args[index..(index + <u64>::arg_count())], memory)?;
        index += <u64>::arg_count();
        let name = <String>::lift_args(&args[index..(index + <String>::arg_count())], memory)?;
        index += <String>::arg_count();
        let birthday = <Option<String>>::lift_args(
            &args[index..(index + <Option<String>>::arg_count())],
            memory,
        )?;
        index += <Option<String>>::arg_count();
        Ok(PersonBorrowed { id, name, birthday })
    }
    fn byte_align() -> usize {
        let mut result = 0;
        result = std::cmp::max(result, <u64>::byte_align());
        result = std::cmp::max(result, <String>::byte_align());
        result = std::cmp::max(result, <Option<String>>::byte_align());
        result
    }
    fn byte_size() -> usize {
        let align = Self::byte_align();
        let mut result = 0;
        result += wasmi_component::helpers::round_up(<u64>::byte_size(), align);
        result += wasmi_component::helpers::round_up(<String>::byte_size(), align);
        result += wasmi_component::helpers::round_up(<Option<String>>::byte_size(), align);
        result
    }
    fn lift_bytes<'a>(
        bytes: &[u8],
        memory: &'a [u8],
    ) -> wasmi_component::ConvertResult<Self::Borrowed<'a>> {
        let align = Self::byte_align();
        let mut index = 0;
        let id = <u64>::lift_bytes(&bytes[index..(index + <u64>::byte_size())], memory)?;
        index += wasmi_component::helpers::round_up(<u64>::byte_size(), align);
        let name = <String>::lift_bytes(&bytes[index..(index + <String>::byte_size())], memory)?;
        index += wasmi_component::helpers::round_up(<String>::byte_size(), align);
        let birthday = <Option<String>>::lift_bytes(
            &bytes[index..(index + <Option<String>>::byte_size())],
            memory,
        )?;
        index += wasmi_component::helpers::round_up(<Option<String>>::byte_size(), align);
        Ok(PersonBorrowed { id, name, birthday })
    }
}
#[derive(Clone, Debug)]
pub struct PersonBorrowed<'a> {
    pub id: <u64 as wasmi_component::ComponentValue>::Borrowed<'a>,
    pub name: <String as wasmi_component::ComponentValue>::Borrowed<'a>,
    pub birthday: <Option<String> as wasmi_component::ComponentValue>::Borrowed<'a>,
}
impl wasmi_component::Lift<Person> for PersonBorrowed<'_> {
    fn lift_owned(&self) -> wasmi_component::ConvertResult<Person> {
        Ok(Person {
            id: self.id.lift_owned()?,
            name: self.name.lift_owned()?,
            birthday: self.birthday.lift_owned()?,
        })
    }
    fn lift_to(&self, target: &mut Person) -> wasmi_component::ConvertResult<()> {
        self.id.lift_to(&mut target.id)?;
        self.name.lift_to(&mut target.name)?;
        self.birthday.lift_to(&mut target.birthday)?;
        Ok(())
    }
}
impl wasmi_component::Lower<Self> for Person {
    fn lower_args(
        &self,
        args: &mut [wasmi_component::lib_structs::WasmValue],
        memory: &mut impl wasmi_component::lib_structs::MemoryAccess,
    ) -> wasmi_component::ConvertResult<()> {
        let mut index = 0;
        <u64>::lower_args(
            &self.id,
            &mut args[index..(index + <u64>::arg_count())],
            memory,
        )?;
        index += <u64>::arg_count();
        <String>::lower_args(
            &self.name,
            &mut args[index..(index + <String>::arg_count())],
            memory,
        )?;
        index += <String>::arg_count();
        <Option<String>>::lower_args(
            &self.birthday,
            &mut args[index..(index + <Option<String>>::arg_count())],
            memory,
        )?;
        index += <Option<String>>::arg_count();
        Ok(())
    }
    fn lower_bytes(
        &self,
        range: std::ops::Range<usize>,
        memory: &mut impl wasmi_component::lib_structs::MemoryAccess,
    ) -> wasmi_component::ConvertResult<()> {
        let align = Self::byte_align();
        let mut index = range.start;
        <u64>::lower_bytes(&self.id, index..(index + <u64>::byte_size()), memory)?;
        index += wasmi_component::helpers::round_up(<u64>::arg_count(), align);
        <String>::lower_bytes(&self.name, index..(index + <String>::byte_size()), memory)?;
        index += wasmi_component::helpers::round_up(<String>::arg_count(), align);
        <Option<String>>::lower_bytes(
            &self.birthday,
            index..(index + <Option<String>>::byte_size()),
            memory,
        )?;
        index += wasmi_component::helpers::round_up(<Option<String>>::arg_count(), align);
        Ok(())
    }
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
