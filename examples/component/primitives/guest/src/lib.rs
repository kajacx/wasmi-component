mod bindings {
    wit_bindgen::generate!({
        path: "../example.wit",
    });

    use super::GuestComponent;
    export!(GuestComponent);
}
struct GuestComponent;

impl bindings::exports::wasmi_component::component_examples::common_funcs::Guest
    for GuestComponent
{
    fn roundtrip_s32(value_a: i32) -> i32 {
        bindings::wasmi_component::component_examples::common_funcs::roundtrip_s32(value_a)
    }

    fn roundtrip_string(value_a: String) -> String {
        bindings::wasmi_component::component_examples::common_funcs::roundtrip_string(&value_a)
    }

    fn roundtrip_multiple(value_a: String, value_b: i32) -> String {
        bindings::wasmi_component::component_examples::common_funcs::roundtrip_multiple(
            &value_a, value_b,
        )
    }

    fn no_arguments() -> () {
        bindings::wasmi_component::component_examples::common_funcs::no_arguments()
    }
}

impl bindings::Guest for GuestComponent {
    fn add_export(value_a: u32, value_b: u32) -> u32 {
        bindings::add_import(value_a, value_b)
    }
}

impl bindings::exports::inline_exports::Guest for GuestComponent {
    fn inline_add(value_a: u32, value_b: u32) -> u32 {
        bindings::inline_imports::inline_add(value_a, value_b)
    }
}
