use wasmi_component::wasmi::{Engine, Store};
use wasmi_component::{AsHostStorage, CompValue, Component, HostResult, HostStorage, LowerVal};

mod bindings;

const WASM: &[u8] = include_bytes!(
    "../../guest/target/wasm32-unknown-unknown/debug/wasmi_component_example_guest.wasm"
);

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

impl bindings::TestExampleImports for HostData {
    fn list_i32(
        &mut self,
        value: <Vec<i32> as CompValue>::Borrowed<'_>,
    ) -> HostResult<impl LowerVal<Vec<i32>> + 'static> {
        Ok(value.to_owned())
    }

    fn list_string(
        &mut self,
        value: <Vec<String> as CompValue>::Borrowed<'_>,
    ) -> HostResult<impl LowerVal<Vec<String>> + 'static> {
        Ok(value.to_owned())
    }

    fn log(&mut self, message: &str) -> HostResult<()> {
        println!("{message}");
        Ok(())
    }
}

pub fn main() {
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
