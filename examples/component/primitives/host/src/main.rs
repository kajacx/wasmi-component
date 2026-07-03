use wasmi_component::wasmi::{Engine, Store};
use wasmi_component::{Component, HostResult};

mod bindings;

const WASM: &[u8] = include_bytes!(
    "../../guest/target/wasm32-unknown-unknown/debug/wasmi_component_example_guest.wasm"
);

impl bindings::TestExampleImports for () {
    fn add_import(&mut self, value_a: u32, value_b: u32) -> HostResult<u32> {
        Ok(value_a + value_b)
    }

    fn common_funcs_no_arguments(&mut self) -> HostResult<()> {
        println!("No args called");
        Ok(())
    }

    fn common_funcs_roundtrip_multiple(
        &mut self,
        value_a: wasmi_component::WitString,
        value_b: i32,
    ) -> HostResult<wasmi_component::WitString> {
        println!("pls: {value_b}");
        Ok(value_a)
    }

    fn common_funcs_roundtrip_s32(&mut self, value_a: i32) -> HostResult<i32> {
        Ok(value_a)
    }

    fn common_funcs_roundtrip_string(
        &mut self,
        value_a: wasmi_component::WitString,
    ) -> HostResult<wasmi_component::WitString> {
        Ok(value_a)
    }

    fn inline_imports_inline_add(&mut self, value_a: u32, value_b: u32) -> HostResult<u32> {
        Ok(value_a + value_b)
    }
}

pub fn main() {
    let engine = Engine::default();
    let mut store = Store::new(&engine, ());

    let component = Component::new(&engine, WASM).unwrap();
    let exports = bindings::instantiate_test_example_world(&mut store, &component).unwrap();

    let result = exports.add_export.call(&mut store, (8, 12)).unwrap();
    println!("Result is: {result}");

    let result = exports
        .common_funcs_roundtrip_multiple
        .call(&mut store, ("Hello", 42))
        .unwrap();
    println!("Result is: {result}");

    let result = exports
        .common_funcs_roundtrip_s32
        .call(&mut store, 67)
        .unwrap();
    println!("Result is: {result}");

    let result = exports
        .common_funcs_roundtrip_s32
        .call(&mut store, (69,))
        .unwrap();
    println!("Result is: {result}");

    let result = exports
        .common_funcs_roundtrip_string
        .call(&mut store, "Hello")
        .unwrap();
    println!("Result is: {result}");

    let result = exports
        .inline_exports_inline_add
        .call(&mut store, (420, 666))
        .unwrap();
    println!("Result is: {result}");

    exports
        .common_funcs_roundtrip_string
        .call_with_results(&mut store, "world!", |name| {
            println!("Hello {name}");
        })
        .unwrap();

    exports
        .common_funcs_no_arguments
        .call(&mut store, ())
        .unwrap();
}
