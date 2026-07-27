use wasmi_component::wasmi::Engine;
use wasmi_component::{Component, HostResult, Linker, ListAccessor, Store};

fn get_wasm() -> Vec<u8> {
    std::fs::read("../guests/target/wasm32-unknown-unknown/debug/example_dynamic_guest.wasm")
        .unwrap()
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
    
    linker.func_new(module, name, callback)

    let component = Component::new(&engine, &get_wasm()).unwrap();

    let exports =
        bindings::instantiate_test_example_world(&mut store, &linker, &component).unwrap();

    println!("Starting host execution\n");

    let result = exports.call_trip_s32(&mut store, 42).unwrap();
    assert_eq!(result, 42);
    println!("Result is: {result}\n");

    let result = exports.call_trip_string(&mut store, "Hello world").unwrap();
    assert_eq!(result, "Hello world");
    println!("Result is: {result}\n");
}
