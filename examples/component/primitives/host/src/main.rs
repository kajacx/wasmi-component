use wasmi_component::*;

const WASM: &[u8] =
    include_bytes!("../../guest/target/wasm32-wasip2/debug/wasmi_component_example_guest.wasm");

pub fn main() {
    let engine = Engine::default();
    let mut store = Store::new(&engine, ());

    let linker = Linker::default();

    let component = Component::new(&engine, WASM).unwrap();
    let _instance = linker.instantiate(&mut store, &component).unwrap();
}
