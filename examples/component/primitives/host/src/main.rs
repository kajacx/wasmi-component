use wasmi_component::wasmi::Store;
use wasmi_component::*;

mod bindings;

const WASM: &[u8] = include_bytes!(
    "../../guest/target/wasm32-unknown-unknown/debug/wasmi_component_example_guest.wasm"
);

pub fn main() {
    let engine = Engine::default();
    let mut store = Store::new(&engine, ());

    let component = Component::new(&engine, WASM).unwrap();
    let exports = bindings::instantiate_example_world(&mut store, &component);
    // let instance = linker
    //     .instantiate_and_start(&mut store, &component)
    //     .unwrap();

    // let add = instance
    //     .get_typed_func::<(i32, i32), i32>(&store, "wasmi-component:examples/funcs@0.1.0#add-s32")
    //     .unwrap();

    // let result = add.call(&mut store, (5, 7)).unwrap();
    let result = exports.funcs_add_s32.call(&mut store, (8, 12)).unwrap();
    println!("Result is: {result}");

    let result = exports.add_u32.call(&mut store, (8, 30)).unwrap();
    println!("Result is: {result}");
}
