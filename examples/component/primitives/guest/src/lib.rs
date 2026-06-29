mod bindings {
    wit_bindgen::generate!({
        path: "../example.wit",
    });

    use super::GuestComponent;
    export!(GuestComponent);
}

struct GuestComponent;

impl bindings::exports::wasmi_component::examples::funcs::Guest for GuestComponent {
    fn add_s32(a: i32, b: i32) -> i32 {
        a + b
    }
}
