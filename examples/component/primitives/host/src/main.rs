use wasmi_component::wasmi::{Engine, Store};
use wasmi_component::{Component, HostResult, HostStorage, LowerVal};

mod bindings;

const WASM: &[u8] = include_bytes!(
    "../../guest/target/wasm32-unknown-unknown/debug/wasmi_component_example_guest.wasm"
);

// struct HostData {
//     storage: HostStorage
// }

impl bindings::TestExampleImports for HostStorage {
    fn add_import(
        &mut self,
        value_a: <u32 as wasmi_component::Lift>::Borrowed<'_>,
        value_b: <u32 as wasmi_component::Lift>::Borrowed<'_>,
    ) -> HostResult<impl LowerVal<Target = u32> + 'static> {
        Ok(value_a + value_b)
    }

    fn common_funcs_no_arguments(&mut self) -> HostResult<impl LowerVal<Target = ()> + 'static> {
        println!("No args called");
        Ok(())
    }

    fn common_funcs_roundtrip_multiple(
        &mut self,
        value_a: <String as wasmi_component::Lift>::Borrowed<'_>,
        value_b: <i32 as wasmi_component::Lift>::Borrowed<'_>,
    ) -> HostResult<impl LowerVal<Target = String> + 'static> {
        Ok(format!("Hello {value_a} and {value_b}!"))
    }

    fn common_funcs_roundtrip_s32(
        &mut self,
        value_a: <i32 as wasmi_component::Lift>::Borrowed<'_>,
    ) -> HostResult<impl LowerVal<Target = i32> + 'static> {
        Ok(value_a)
    }

    fn common_funcs_roundtrip_string(
        &mut self,
        value_a: <String as wasmi_component::Lift>::Borrowed<'_>,
    ) -> HostResult<impl LowerVal<Target = String> + 'static> {
        println!("incoming string: {value_a}");
        Ok("outgoing string")
    }

    fn inline_imports_inline_add(
        &mut self,
        value_a: <u32 as wasmi_component::Lift>::Borrowed<'_>,
        value_b: <u32 as wasmi_component::Lift>::Borrowed<'_>,
    ) -> HostResult<impl LowerVal<Target = u32> + 'static> {
        Ok(value_a + value_b)
    }
}

pub fn main() {
    let engine = Engine::default();
    let mut store = Store::new(&engine, HostStorage::new());

    let component = Component::new(&engine, WASM).unwrap();
    let exports = bindings::instantiate_test_example_world(&mut store, &component).unwrap();

    println!("Starting host execution");

    let result = exports.add_export.call(&mut store, (8u32, 12u32)).unwrap();
    println!("Result is: {result}");

    let result = exports
        .common_funcs_roundtrip_multiple
        .call(&mut store, ("Hello", 42))
        .unwrap();
    println!("Result is: {result}");

    let result = exports
        .common_funcs_roundtrip_s32
        .call(&mut store, (67,)) // TODO: calling like this is awkward
        .unwrap();
    println!("Result is: {result}");

    let result = exports
        .common_funcs_roundtrip_s32
        .call(&mut store, (69,))
        .unwrap();
    println!("Result is: {result}");

    let result = exports
        .common_funcs_roundtrip_string
        .call(&mut store, ("Hello",))
        .unwrap();
    println!("Result is: {result}");

    let result = exports
        .inline_exports_inline_add
        .call(&mut store, (420u32, 666u32))
        .unwrap();
    println!("Result is: {result}");

    exports
        .common_funcs_roundtrip_string
        .call_with_results(&mut store, ("world!",), |name| {
            println!("Hello {name}");
        })
        .unwrap();

    exports
        .common_funcs_no_arguments
        .call(&mut store, ())
        .unwrap();
}
