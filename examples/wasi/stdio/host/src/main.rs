use wasmi_component::wasmi::Engine;
use wasmi_component::{Component, Linker, Store};

use crate::bindings::add_test_example_to_linker;

mod bindings;

const WASM: &[u8] =
    include_bytes!("../../guest/target/wasm32-wasip2/debug/wasmi_component_example_guest.wasm");

#[derive(Default)]
struct HostData {}

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
    wasmi_component_wasi::add_wasi_p2_to_linker(&mut linker).unwrap();
    add_test_example_to_linker(&mut linker).unwrap();

    let component = Component::new(&engine, WASM).unwrap();
    let exports =
        bindings::instantiate_test_example_world(&mut store, &linker, &component).unwrap();

    println!("Starting host execution");

    exports
        .print_stdout
        .call(&mut store, ("Message to stdout",))
        .unwrap();

    println!("Host execution returned");

    exports
        .print_stdout
        .call(&mut store, ("Message to stdout again",))
        .unwrap();

    exports
        .print_stderr
        .call(&mut store, ("Message to stderr",))
        .unwrap();
}
