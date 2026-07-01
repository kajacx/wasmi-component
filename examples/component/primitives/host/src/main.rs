use wasmi_component::Component;
use wasmi_component::wasmi::{Engine, Store};

mod bindings;

const WASM: &[u8] = include_bytes!(
    "../../guest/target/wasm32-unknown-unknown/debug/wasmi_component_example_guest.wasm"
);

pub fn main() {
    let engine = Engine::default();
    let mut store = Store::new(&engine, ());

    let component = Component::new(&engine, WASM).unwrap();
    let exports = bindings::instantiate_example_world(&mut store, &component).unwrap();

    let result = exports.funcs_add_s32.call(&mut store, (8, 12)).unwrap();
    println!("Result is: {result}");

    let result = exports.add_u32.call(&mut store, (8, 30)).unwrap();
    println!("Result is: {result}");

    let result = exports.add_f32.call(&mut store, (4.5, 9.0)).unwrap();
    println!("Result is: {result}");

    let result = exports
        .funcs_greet
        .call(&mut store, "kajacx".to_string())
        .unwrap();
    println!("Result is: {result}");

    // exports
    //     .funcs_get_name
    //     .call_with_results(&mut store, (), |name| {
    //         println!("Printing name without any allocations skibidi {name}");
    //     })
    //     .unwrap();
}
