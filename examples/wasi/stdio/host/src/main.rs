use wasmi_component::wasmi::{Engine, Store};
use wasmi_component::{AsHostStorage, Component, HostStorage};

mod bindings;

const WASM: &[u8] =
    include_bytes!("../../guest/target/wasm32-wasip2/debug/wasmi_component_example_guest.wasm");

#[derive(Default)]
struct HostData {
    storage: HostStorage,
}

impl AsHostStorage for HostData {
    fn as_host_storage(&self) -> &HostStorage {
        &self.storage
    }

    fn as_host_storage_mut(&mut self) -> &mut HostStorage {
        &mut self.storage
    }
}

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
