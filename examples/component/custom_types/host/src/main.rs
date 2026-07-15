use wasmi_component::wasmi::Engine;
use wasmi_component::{Component, HostResult, Linker, Store, View};

use crate::bindings::Person;

mod bindings;
// mod expanded;

const WASM: &[u8] = include_bytes!(
    "../../guest/target/wasm32-unknown-unknown/debug/wasmi_component_example_guest.wasm"
);

#[derive(Default)]
struct HostData {}

impl bindings::TestExampleImports for HostData {
    fn trip_person(&mut self, value: bindings::PersonBorrowed) -> HostResult<bindings::Person> {
        println!("[HOST]: Receiving Person value {value:?}");
        let value = value.lift_owned()?;
        println!("[HOST]: Returning Person value {value:?}");
        Ok(value)
    }

    fn trip_data(&mut self, value: bindings::DataBorrowed) -> HostResult<bindings::Data> {
        println!("[HOST]: Receiving Data value {value:?}");
        let value = value.lift_owned()?;
        println!("[HOST]: Returning Data value {value:?}");
        Ok(value)
    }

    fn trip_mixed(
        &mut self,
        _a: bindings::PersonBorrowed,
        _b: i32,
        _c: wasmi_component::anyhow::Result<bindings::DataBorrowed, &str>,
    ) -> HostResult<()> {
        Ok(())
    }

    fn price(
        &mut self,
        item: wasmi_component::ListAccessor<(bindings::Fruit, u32)>,
    ) -> HostResult<f32> {
        println!("{item:?}");
        Ok(0.0)
    }

    fn log(&mut self, message: &str) -> HostResult<()> {
        println!("{message}");
        Ok(())
    }
}

pub fn main() {
    let _ = std::thread::Builder::new()
        .stack_size(128 * 1024 * 1024)
        .spawn(main_)
        .unwrap()
        .join();
}

pub fn main_() {
    let engine = Engine::default();
    let mut store = Store::new(&engine, HostData::default());

    let mut linker = Linker::new(store.engine());
    bindings::add_test_example_to_linker(&mut linker).unwrap();

    let component = Component::new(&engine, WASM).unwrap();

    let exports =
        bindings::instantiate_test_example_world(&mut store, &linker, &component).unwrap();

    println!("Starting host execution\n");

    let result = exports
        .call_trip_person(
            &mut store,
            &Person {
                id: 64,
                name: "Tom".to_string(),
            },
        )
        .unwrap();
    assert_eq!(result.id, 64);
    assert_eq!(result.name, "Tom");
    println!("Result is: {result:?}\n");

    let result = exports
        .call_trip_data(&mut store, &bindings::Data::Number(62.0))
        .unwrap();
    assert_eq!(result, bindings::Data::Number(62.0));
    println!("Result is: {result:?}\n");

    let result = exports
        .call_trip_data(&mut store, &bindings::Data::Text("Hello data".to_string()))
        .unwrap();
    assert_eq!(result, bindings::Data::Text("Hello data".to_string()));
    println!("Result is: {result:?}\n");
}
