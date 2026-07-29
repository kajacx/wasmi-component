use wasmi_component::wasmi::Engine;
use wasmi_component::{Component, Linker, Store};

fn get_wasm() -> Vec<u8> {
    std::fs::read("../guests/target/wasm32-unknown-unknown/debug/example_guest_dynamic.wasm")
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

#[allow(unused)]
pub fn main_() {
    let engine = Engine::default();
    let mut store = Store::new(&engine, HostData::default());

    let mut linker = Linker::<HostData>::new(store.engine());

    let component = Component::new(&engine, &get_wasm()).unwrap();

    println!("Starting host execution\n");
}
