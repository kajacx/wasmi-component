use wasmi_component::wasmi::Engine;
use wasmi_component::{Component, HostResult, IntoOwned, ListAccessor, LowerVal, Store};

mod bindings;

const WASM: &[u8] = include_bytes!(
    "../../guest/target/wasm32-unknown-unknown/debug/wasmi_component_example_guest.wasm"
);

#[derive(Default)]
struct HostData {}

impl bindings::TestExampleImports for HostData {
    fn list_i32(
        &mut self,
        value: ListAccessor<i32>,
    ) -> HostResult<impl LowerVal<Vec<i32>> + 'static> {
        Ok(value.into_owned())
    }

    fn list_string(
        &mut self,
        value: ListAccessor<String>,
    ) -> HostResult<impl LowerVal<Vec<String>> + 'static> {
        Ok(value.into_owned())
    }

    fn log(&mut self, message: &str) -> HostResult<()> {
        println!("{message}");
        Ok(())
    }
}

pub fn main() {
    std::thread::Builder::new()
        .stack_size(128 * 1024 * 1024)
        .spawn(main_)
        .unwrap()
        .join()
        .unwrap();
}

pub fn main_() {
    let engine = Engine::default();
    let mut store = Store::new(&engine, HostData::default());

    let component = Component::new(&engine, WASM).unwrap();
    let exports = bindings::instantiate_test_example_world(&mut store, &component).unwrap();

    println!("Starting host execution");

    exports.init.call(&mut store, ()).unwrap();
    println!("Init called");

    let res = exports.list_i32.call(&mut store, ([1, 2, 3],)).unwrap();
    println!("Result is: {res:?}");

    let res = exports
        .list_string
        .call(&mut store, (["a", "b", "c"],))
        .unwrap();
    println!("Result is: {res:?}");
}
