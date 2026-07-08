use wasmi_component::wasmi::Engine;
use wasmi_component::{Component, HostResult, Linker, ListAccessor, Store, View};

mod bindings;

const WASM: &[u8] = include_bytes!(
    "../../guest/target/wasm32-unknown-unknown/debug/wasmi_component_example_guest.wasm"
);

#[derive(Default)]
struct HostData {
    holder_list_i32: Vec<i32>,
    holder_list_string: Vec<String>,
}

impl bindings::TestExampleImports for HostData {
    type ListI32Return<'a> = &'a [i32];
    fn list_i32<'a>(&'a mut self, value: ListAccessor<i32>) -> HostResult<Self::ListI32Return<'a>> {
        println!("[HOST]: Calling list_i32");

        value.lift_to(&mut self.holder_list_i32);

        Ok(&self.holder_list_i32)
    }

    type ListStringReturn<'a> = &'a [&'a String];
    fn list_string(&mut self, value: ListAccessor<String>) -> HostResult<Self::ListStringReturn> {
        println!("[HOST]: Calling list_string");

        value.lift_to(&mut self.holder_list_string);

        Ok(&self.holder_list_string)
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

    let mut linker = Linker::new(store.engine());
    bindings::add_test_example_to_linker(&mut linker).unwrap();

    let component = Component::new(&engine, WASM).unwrap();
    let exports =
        bindings::instantiate_test_example_world(&mut store, &linker, &component).unwrap();

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
