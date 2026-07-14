use wasmi_component::wasmi::Engine;
use wasmi_component::{Component, HostResult, Linker, ListAccessor, Store, View};

mod bindings;

const WASM: &[u8] = include_bytes!(
    "../../guest/target/wasm32-unknown-unknown/debug/wasmi_component_example_guest.wasm"
);

#[derive(Default)]
struct HostData {
    names: Vec<String>,
}

impl bindings::TestExampleImports for HostData {
    fn list_s8(&mut self, value: ListAccessor<i8>) -> HostResult<Vec<i8>> {
        println!("[HOST]: Receiving ListAccessor<i8> value {value:?}");
        let result = value.lift_owned()?;
        println!("[HOST]: Returning Vec<i8> value {result:?}");
        Ok(result)
    }

    fn list_s16(&mut self, value: ListAccessor<i16>) -> HostResult<Vec<i16>> {
        println!("[HOST]: Receiving ListAccessor<i16> value {value:?}");
        let result = value.lift_owned()?;
        println!("[HOST]: Returning Vec<i16> value {result:?}");
        Ok(result)
    }

    fn list_s32(&mut self, value: ListAccessor<i32>) -> HostResult<Vec<i32>> {
        println!("[HOST]: Receiving ListAccessor<i32> value {value:?}");
        let result = value.lift_owned()?;
        println!("[HOST]: Returning Vec<i32> value {result:?}");
        Ok(result)
    }

    fn list_s64(&mut self, value: ListAccessor<i64>) -> HostResult<Vec<i64>> {
        println!("[HOST]: Receiving ListAccessor<i64> value {value:?}");
        let result = value.lift_owned()?;
        println!("[HOST]: Returning Vec<i64> value {result:?}");
        Ok(result)
    }

    fn list_u8(&mut self, value: ListAccessor<u8>) -> HostResult<Vec<u8>> {
        println!("[HOST]: Receiving ListAccessor<u8> value {value:?}");
        let result = value.lift_owned()?;
        println!("[HOST]: Returning Vec<u8> value {result:?}");
        Ok(result)
    }

    fn list_u16(&mut self, value: ListAccessor<u16>) -> HostResult<Vec<u16>> {
        println!("[HOST]: Receiving ListAccessor<u16> value {value:?}");
        let result = value.lift_owned()?;
        println!("[HOST]: Returning Vec<u16> value {result:?}");
        Ok(result)
    }

    fn list_u32(&mut self, value: ListAccessor<u32>) -> HostResult<Vec<u32>> {
        println!("[HOST]: Receiving ListAccessor<u32> value {value:?}");
        let result = value.lift_owned()?;
        println!("[HOST]: Returning Vec<u32> value {result:?}");
        Ok(result)
    }

    fn list_u64(&mut self, value: ListAccessor<u64>) -> HostResult<Vec<u64>> {
        println!("[HOST]: Receiving ListAccessor<u64> value {value:?}");
        let result = value.lift_owned()?;
        println!("[HOST]: Returning Vec<u64> value {result:?}");
        Ok(result)
    }

    fn list_f32(&mut self, value: ListAccessor<f32>) -> HostResult<Vec<f32>> {
        println!("[HOST]: Receiving ListAccessor<f32> value {value:?}");
        let result = value.lift_owned()?;
        println!("[HOST]: Returning Vec<f32> value {result:?}");
        Ok(result)
    }

    fn list_f64(&mut self, value: ListAccessor<f64>) -> HostResult<Vec<f64>> {
        println!("[HOST]: Receiving ListAccessor<f64> value {value:?}");
        let result = value.lift_owned()?;
        println!("[HOST]: Returning Vec<f64> value {result:?}");
        Ok(result)
    }

    fn list_bool(&mut self, value: ListAccessor<bool>) -> HostResult<Vec<bool>> {
        println!("[HOST]: Receiving ListAccessor<bool> value {value:?}");
        let result = value.lift_owned()?;
        println!("[HOST]: Returning Vec<bool> value {result:?}");
        Ok(result)
    }

    fn list_char(&mut self, value: ListAccessor<char>) -> HostResult<Vec<char>> {
        println!("[HOST]: Receiving ListAccessor<char> value {value:?}");
        let result = value.lift_owned()?;
        println!("[HOST]: Returning Vec<char> value {result:?}");
        Ok(result)
    }

    fn list_string(&mut self, value: ListAccessor<String>) -> HostResult<Vec<String>> {
        println!("[HOST]: Receiving ListAccessor<String> value {value:?}");
        let result = value.lift_owned()?;
        println!("[HOST]: Returning Vec<String> value {result:?}");
        Ok(result)
    }

    fn log(&mut self, message: &str) -> HostResult<()> {
        println!("{message}");
        Ok(())
    }
}

pub fn main() {
    let _ = std::thread::Builder::new()
        .stack_size(128 * 1024 * 1024)
        .spawn(main_)
        .unwrap()
        .join();
}

pub fn main_() {
    let engine = Engine::default();
    let mut store = Store::new(&engine, HostData::default());

    let mut linker = Linker::new(store.engine());
    bindings::add_test_example_to_linker(&mut linker).unwrap();

    let component = Component::new(&engine, WASM).unwrap();
    let exports =
        bindings::instantiate_test_example_world(&mut store, &linker, &component).unwrap();

    println!("Starting host execution\n");

    let result = exports.call_list_s8(&mut store, [42, 67, 69]).unwrap();
    assert_eq!(result, [42, 67, 69]);
    println!("Result is: {result:?}\n");

    let result = exports.call_list_s16(&mut store, [42, 67, 69]).unwrap();
    assert_eq!(result, [42, 67, 69]);
    println!("Result is: {result:?}\n");

    let result = exports.call_list_s32(&mut store, [42, 67, 69]).unwrap();
    assert_eq!(result, [42, 67, 69]);
    println!("Result is: {result:?}\n");

    let result = exports.call_list_s64(&mut store, [42, 67, 69]).unwrap();
    assert_eq!(result, [42, 67, 69]);
    println!("Result is: {result:?}\n");

    let result = exports.call_list_s8(&mut store, [42, 67, 69]).unwrap();
    assert_eq!(result, [42, 67, 69]);
    println!("Result is: {result:?}\n");

    let result = exports.call_list_s16(&mut store, [42, 67, 69]).unwrap();
    assert_eq!(result, [42, 67, 69]);
    println!("Result is: {result:?}\n");

    let result = exports.call_list_s32(&mut store, [42, 67, 69]).unwrap();
    assert_eq!(result, [42, 67, 69]);
    println!("Result is: {result:?}\n");

    let result = exports.call_list_s64(&mut store, [42, 67, 69]).unwrap();
    assert_eq!(result, [42, 67, 69]);
    println!("Result is: {result:?}\n");

    let result = exports
        .call_list_f32(&mut store, [42.0, 67.0, 69.0])
        .unwrap();
    assert_eq!(result, [42.0, 67.0, 69.0]);
    println!("Result is: {result:?}\n");

    let result = exports
        .call_list_f64(&mut store, [42.0, 67.0, 69.0])
        .unwrap();
    assert_eq!(result, [42.0, 67.0, 69.0]);
    println!("Result is: {result:?}\n");

    let result = exports
        .call_list_bool(&mut store, [true, false, true])
        .unwrap();
    assert_eq!(result, [true, false, true]);
    println!("Result is: {result:?}\n");

    let result = exports.call_list_char(&mut store, ['$', '&', '*']).unwrap();
    assert_eq!(result, ['$', '&', '*']);
    println!("Result is: {result:?}\n");

    let result = exports
        .call_list_string(&mut store, ["Hello", "beautiful", "world"])
        .unwrap();
    assert_eq!(result, ["Hello", "beautiful", "world"]);
    println!("Result is: {result:?}\n");

    exports
        .call_list_string_with_results(
            &mut store,
            ["Hello", "zero", "copy"],
            |store_data, result| {
                result.lift_to(&mut store_data.names).unwrap();
            },
        )
        .unwrap();
    assert_eq!(store.data().names, ["Hello", "zero", "copyy"]);
}
