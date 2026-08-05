use wasmi_component::wasmi::Engine;
use wasmi_component::{DynValue, Linker, Store};

// Not used here, but the build pipeline gets confused if it's missing
mod bindings;

fn get_wasm() -> Vec<u8> {
    let path = "guests/target/wasm32-unknown-unknown/debug/example_guest_dynamic.wasm";
    std::fs::read(path)
        .or_else(|_| std::fs::read(format!("../{path}")))
        .unwrap()
}

#[derive(Default)]
struct HostData {}

pub fn main() {
    std::thread::Builder::new()
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

    let component = store.new_component(&get_wasm()).unwrap();

    linker.func_dyn(
        "wasmi-component:component-examples/round-trip@0.1.0",
        "trip-s32",
        &component,
        |data, values: &[DynValue]| {
            println!("[HOST]: Receiving s32 value {:?}", values);
            let value = values[0].as_s32().unwrap();
            println!("[HOST]: Returning s32 value {:?}", value);
            Ok(DynValue::new_s32(value))
        },
    );

    linker
        .func_dyn(
            "wasmi-component:component-examples/round-trip@0.1.0",
            "trip-string",
            &component,
            |data, values: &[DynValue]| {
                println!("[HOST]: Receiving string value {:?}", values);
                let value = values[0].as_string().unwrap();
                println!("[HOST]: Returning string value {:?}", value);
                Ok(DynValue::new_string(value))
            },
        )
        .unwrap();

    linker
        .func_dyn(
            "wasmi-component:component-examples/round-trip@0.1.0",
            "trip-person",
            &component,
            |data, values: &[DynValue]| {
                println!("[HOST]: Receiving person value {:?}", values);
                let value = values[0].as_record().unwrap();
                println!("[HOST]: Returning person value {:?}", value);
                Ok(DynValue::new_record(value.fields.iter().cloned()))
            },
        )
        .unwrap();

    linker
        .func_dyn("", "log", &component, |data, values: &[DynValue]| {
            println!("{}", values[0].as_string().unwrap());
            Ok(DynValue::unit())
        })
        .unwrap();

    let instance = linker.instantiate(&mut store, &component).unwrap();

    println!("Starting host execution\n");

    let result = instance
        .get_dyn_func(
            &store,
            "wasmi-component:component-examples/round-trip@0.1.0",
            "trip-s32",
        )
        .unwrap()
        .call(&mut store, [DynValue::new_s32(42)])
        .unwrap();
    assert_eq!(result, DynValue::new_s32(42));
    println!("Result is: {result:?}\n");

    let result = instance
        .get_dyn_func(
            &store,
            "wasmi-component:component-examples/round-trip@0.1.0",
            "trip-string",
        )
        .unwrap()
        .call(&mut store, [DynValue::new_string("Hello")])
        .unwrap();
    assert_eq!(result, DynValue::new_string("Hello"));
    println!("Result is: {result:?}\n");

    let person = DynValue::new_record([
        ("id".into(), DynValue::new_u64(50)),
        ("name".into(), DynValue::new_string("Tom")),
        ("birthday".into(), DynValue::new_option(None)),
    ]);
    let result = instance
        .get_dyn_func(
            &store,
            "wasmi-component:component-examples/round-trip@0.1.0",
            "trip-person",
        )
        .unwrap()
        .call(&mut store, [person.clone()])
        .unwrap();
    assert_eq!(result, person);
    println!("Result is: {result:?}\n");
}
