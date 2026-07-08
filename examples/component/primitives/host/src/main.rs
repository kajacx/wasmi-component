use wasmi_component::wasmi::Engine;
use wasmi_component::{Component, HostResult, Linker, LowerVal, Store};

use crate::bindings::add_test_example_to_linker;

mod bindings;

const WASM: &[u8] = include_bytes!(
    "../../guest/target/wasm32-unknown-unknown/debug/wasmi_component_example_guest.wasm"
);

#[derive(Default)]
struct HostData {}

impl bindings::TestExampleImports for HostData {
    fn add_import(&mut self, value_a: u32, value_b: u32) -> HostResult<u32> {
        Ok(value_a + value_b)
    }

    fn no_arguments(&mut self) -> HostResult<()> {
        println!("No args called");
        Ok(())
    }

    type RoundtripStringReturn = String;
    fn roundtrip_multiple(
        &mut self,
        value_a: &str,
        value_b: i32,
    ) -> HostResult<Self::RoundtripStringReturn> {
        Ok(format!("Hello {value_a} and {value_b}!"))
    }

    fn roundtrip_s32(&mut self, value_a: i32) -> HostResult<i32> {
        Ok(value_a)
    }

    type RoundtripMultipleReturn = String;
    fn roundtrip_string(&mut self, value_a: &str) -> HostResult<Self::RoundtripMultipleReturn> {
        Ok(value_a.to_string())
    }

    fn inline_add(&mut self, value_a: u32, value_b: u32) -> HostResult<u32> {
        Ok(value_a + value_b)
    }
}

pub fn main() {
    let engine = Engine::default();
    let mut store = Store::new(&engine, HostData::default());

    let mut linker = Linker::new(store.engine());
    add_test_example_to_linker(&mut linker).unwrap();

    let component = Component::new(&engine, WASM).unwrap();

    let exports =
        bindings::instantiate_test_example_world(&mut store, &linker, &component).unwrap();

    println!("Starting host execution");

    let result = exports.add_export.call(&mut store, (8u32, 12u32)).unwrap();
    println!("Result is: {result}");

    let result = exports
        .roundtrip_multiple
        .call(&mut store, ("Hello", 42))
        .unwrap();
    println!("Result is: {result}");

    let result = exports
        .roundtrip_s32
        .call(&mut store, (67,)) // TODO: calling like this is awkward
        .unwrap();
    println!("Result is: {result}");

    let result = exports.roundtrip_s32.call(&mut store, (69,)).unwrap();
    println!("Result is: {result}");

    let result = exports.roundtrip_string.call(&mut store, ("",)).unwrap();
    println!("Result is: {result}");

    let result = exports
        .inline_add
        .call(&mut store, (420u32, 666u32))
        .unwrap();
    println!("Result is: {result}");

    exports
        .roundtrip_string
        .call_with_results(&mut store, ("world!",), |name| {
            println!("Hello {name}");
        })
        .unwrap();

    exports.no_arguments.call(&mut store, ()).unwrap();
}
