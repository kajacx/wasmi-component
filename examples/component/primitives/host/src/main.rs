use wasmi_component::*;

const WASM: &[u8] = include_bytes!(
    "../../guest/target/wasm32-unknown-unknown/debug/wasmi_component_example_guest.wasm"
);

pub fn main() {
    let engine = Engine::default();
    let mut store = Store::new(&engine, ());

    let linker = Linker::new(&engine);

    let component = Component::new(&engine, WASM).unwrap();
    let instance = linker.instantiate(&mut store, &component).unwrap();

    let add = instance
        .instance
        .get_typed_func::<(i32, i32), i32>(&store, "wasmi-component:examples/funcs@0.1.0#add-s32")
        .unwrap();

    let result = add.call(&mut store, (5, 7)).unwrap();
    println!("Result is: {result}");
}
