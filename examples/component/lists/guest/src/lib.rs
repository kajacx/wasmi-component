mod bindings {
    wit_bindgen::generate!({
        path: "../example.wit",
    });

    use super::GuestComponent;
    export!(GuestComponent);
}
struct GuestComponent;

impl bindings::Guest for GuestComponent {
    fn round_trip_u32(values: Vec<u32>) -> Vec<u32> {
        values
    }
}
