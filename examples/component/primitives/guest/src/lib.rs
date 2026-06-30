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

impl bindings::Guest for GuestComponent {
    fn add_u32(a: u32, b: u32) -> u32 {
        a + b
    }
}

impl bindings::exports::additional::Guest for GuestComponent {
    fn add_f32(a: f32, b: f32) -> f32 {
        a + b
    }
}
