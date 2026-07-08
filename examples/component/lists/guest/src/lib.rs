mod bindings {
    wit_bindgen::generate!({
        path: "../example.wit",
    });

    use super::GuestComponent;
    export!(GuestComponent);
}

struct GuestComponent;

impl bindings::exports::wasmi_component::component_examples::round_trip::Guest for GuestComponent {
    fn list_i32(value: Vec<i32>) -> Vec<i32> {
        bindings::log("[GUEST]: Calling list_i32");

        bindings::wasmi_component::component_examples::round_trip::list_i32(&value)
    }

    fn list_string(value: Vec<String>) -> Vec<String> {
        bindings::log("[GUEST]: Calling list_string");

        bindings::wasmi_component::component_examples::round_trip::list_string(&value)
    }
}

impl bindings::Guest for GuestComponent {
    fn init() -> () {
        std::panic::set_hook(Box::new(|info| {
            bindings::log("[GUEST]: PANIC OK");

            let message = if let Some(s) = info.payload().downcast_ref::<&str>() {
                *s
            } else if let Some(s) = info.payload().downcast_ref::<String>() {
                s.as_str()
            } else {
                "Unknown panic occurred"
            };
            bindings::log("[GUEST]: PANIC B");
            bindings::log(message);

            let location = info
                .location()
                .map(|loc| format!(" at {}:{}:{}", loc.file(), loc.line(), loc.column()))
                .unwrap_or_default();
            bindings::log("[GUEST]: PANIC C");

            let full_error_msg = format!("Guest Panicked: {}{}", message, location);
            bindings::log("[GUEST]: PANIC D");
            bindings::log(&full_error_msg);
        }));
    }
}
