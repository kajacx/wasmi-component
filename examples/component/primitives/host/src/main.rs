use wasmi_component::wasmi::Engine;
use wasmi_component::{Component, HostResult, Linker, Store};

use crate::bindings::add_test_example_to_linker;

mod bindings;

const WASM: &[u8] = include_bytes!(
    "../../guest/target/wasm32-unknown-unknown/debug/wasmi_component_example_guest.wasm"
);

#[derive(Default)]
struct HostData {}

impl bindings::TestExampleImports for HostData {
    fn trip_s8(&mut self, value: i8) -> HostResult<i8> {
        println!("[HOST]: Receiving i8 value {value}");
        Ok(value)
    }

    fn trip_s16(&mut self, value: i16) -> HostResult<i16> {
        println!("[HOST]: Receiving i16 value {value}");
        Ok(value)
    }

    fn trip_s32(&mut self, value: i32) -> HostResult<i32> {
        println!("[HOST]: Receiving i32 value {value}");
        Ok(value)
    }

    fn trip_s64(&mut self, value: i64) -> HostResult<i64> {
        println!("[HOST]: Receiving i64 value {value}");
        Ok(value)
    }

    fn trip_u8(&mut self, value: u8) -> HostResult<u8> {
        println!("[HOST]: Receiving u8 value {value}");
        Ok(value)
    }

    fn trip_u16(&mut self, value: u16) -> HostResult<u16> {
        println!("[HOST]: Receiving u16 value {value}");
        Ok(value)
    }

    fn trip_u32(&mut self, value: u32) -> HostResult<u32> {
        println!("[HOST]: Receiving u32 value {value}");
        Ok(value)
    }

    fn trip_u64(&mut self, value: u64) -> HostResult<u64> {
        println!("[HOST]: Receiving u64 value {value}");
        Ok(value)
    }

    fn trip_f32(&mut self, value: f32) -> HostResult<f32> {
        println!("[HOST]: Receiving f32 value {value}");
        Ok(value)
    }

    fn trip_f64(&mut self, value: f64) -> HostResult<f64> {
        println!("[HOST]: Receiving f64 value {value}");
        Ok(value)
    }

    fn trip_bool(&mut self, value: bool) -> HostResult<bool> {
        println!("[HOST]: Receiving bool value {value}");
        Ok(value)
    }

    fn trip_char(&mut self, value: char) -> HostResult<char> {
        println!("[HOST]: Receiving char value {value}");
        Ok(value)
    }

    fn trip_string(&mut self, value: &str) -> HostResult<String> {
        println!("[HOST]: Receiving string value {value}");
        Ok(value.to_string())
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
    add_test_example_to_linker(&mut linker).unwrap();

    let component = Component::new(&engine, WASM).unwrap();

    let exports =
        bindings::instantiate_test_example_world(&mut store, &linker, &component).unwrap();

    println!("Starting host execution\n");

    let result = exports.call_trip_s8(&mut store, 42).unwrap();
    assert_eq!(result, 42);
    println!("Result is: {result}\n");

    let result = exports.call_trip_s16(&mut store, 42).unwrap();
    assert_eq!(result, 42);
    println!("Result is: {result}\n");

    let result = exports.call_trip_s32(&mut store, 42).unwrap();
    assert_eq!(result, 42);
    println!("Result is: {result}\n");

    let result = exports.call_trip_s64(&mut store, 42).unwrap();
    assert_eq!(result, 42);
    println!("Result is: {result}\n");

    let result = exports.call_trip_u8(&mut store, 42).unwrap();
    assert_eq!(result, 42);
    println!("Result is: {result}\n");

    let result = exports.call_trip_u16(&mut store, 42).unwrap();
    assert_eq!(result, 42);
    println!("Result is: {result}\n");

    let result = exports.call_trip_u32(&mut store, 42).unwrap();
    assert_eq!(result, 42);
    println!("Result is: {result}\n");

    let result = exports.call_trip_u64(&mut store, 42).unwrap();
    assert_eq!(result, 42);
    println!("Result is: {result}\n");

    let result = exports.call_trip_f32(&mut store, 42.0).unwrap();
    assert_eq!(result, 42.0);
    println!("Result is: {result}\n");

    let result = exports.call_trip_f64(&mut store, 42.0).unwrap();
    assert_eq!(result, 42.0);
    println!("Result is: {result}\n");

    let result = exports.call_trip_bool(&mut store, true).unwrap();
    assert_eq!(result, true);
    println!("Result is: {result}\n");

    let result = exports.call_trip_char(&mut store, '#').unwrap();
    assert_eq!(result, '#');
    println!("Result is: {result}\n");

    let result = exports.call_trip_string(&mut store, "Hello world").unwrap();
    assert_eq!(result, "Hello world");
    println!("Result is: {result}\n");

    let len = exports
        .call_trip_string_with_results(&mut store, "Zero copy", |result| {
            assert_eq!(result, "Zero copy");
            println!("Result is: {result}\n");
            result.len()
        })
        .unwrap();
    assert_eq!(len, "Zero copy".len())
}
