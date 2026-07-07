use wasmi_component::wasmi::Engine;
use wasmi_component::{Component, Store};

mod bindings;

const WASM: &[u8] =
    include_bytes!("../../guest/target/wasm32-wasip2/debug/wasmi_component_example_guest.wasm");

#[derive(Default)]
struct HostData {}

pub fn main() {
    let engine = Engine::default();
    let mut store = Store::new(&engine, HostData::default());

    let component = Component::new(&engine, WASM).unwrap();
    let exports = bindings::instantiate_test_example_world(&mut store, &component).unwrap();

    println!("Starting host execution");

    exports
        .print_stdout
        .call(&mut store, ("Message to stdout",))
        .unwrap();

    exports
        .print_stderr
        .call(&mut store, ("Message to stderr",))
        .unwrap();
}
