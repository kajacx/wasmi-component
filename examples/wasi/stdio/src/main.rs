use wasmi_component::wasmi::Engine;
use wasmi_component::{Component, Linker, Store};

mod bindings;

fn get_wasm() -> Vec<u8> {
    std::fs::read("../guests/target/wasm32-wasip2/debug/example_stdio_guest.wasm").unwrap()
}

#[derive(Default)]
struct HostData {}

pub fn main() {
    let _ = std::thread::Builder::new()
        .stack_size(128 * 1024 * 1024)
        .spawn(main_)
        .unwrap()
        .join()
        .unwrap();
}

pub fn main_() {
    let engine = Engine::default();
    let mut store = Store::new(&engine, HostData::default());

    let mut linker = Linker::new(store.engine());
    wasmi_component_wasi::add_wasi_p2_to_linker(&mut linker).unwrap();
    bindings::add_test_example_to_linker(&mut linker).unwrap();

    let component = Component::new(&engine, &get_wasm()).unwrap();
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
