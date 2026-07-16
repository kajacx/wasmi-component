mod bindings {
    wit_bindgen::generate!({
        path: "../../../examples/component/primitives/example.wit",
    });

    use super::GuestComponent;
    export!(GuestComponent);
}
struct GuestComponent;

impl bindings::exports::wasmi_component::component_examples::round_trip::Guest for GuestComponent {
    fn trip_s8(value: i8) -> i8 {
        bindings::log(&format!("[GUEST]: Receiving i8 value {value}"));
        let result = bindings::wasmi_component::component_examples::round_trip::trip_s8(value);
        bindings::log(&format!("[GUEST]: Returning i8 value {result}"));
        result
    }

    fn trip_s16(value: i16) -> i16 {
        bindings::log(&format!("[GUEST]: Receiving i16 value {value}"));
        let result = bindings::wasmi_component::component_examples::round_trip::trip_s16(value);
        bindings::log(&format!("[GUEST]: Returning i16 value {result}"));
        result
    }

    fn trip_s32(value: i32) -> i32 {
        bindings::log(&format!("[GUEST]: Receiving i32 value {value}"));
        let result = bindings::wasmi_component::component_examples::round_trip::trip_s32(value);
        bindings::log(&format!("[GUEST]: Returning i32 value {result}"));
        result
    }

    fn trip_s64(value: i64) -> i64 {
        bindings::log(&format!("[GUEST]: Receiving i64 value {value}"));
        let result = bindings::wasmi_component::component_examples::round_trip::trip_s64(value);
        bindings::log(&format!("[GUEST]: Returning i64 value {result}"));
        result
    }

    fn trip_u8(value: u8) -> u8 {
        bindings::log(&format!("[GUEST]: Receiving u8 value {value}"));
        let result = bindings::wasmi_component::component_examples::round_trip::trip_u8(value);
        bindings::log(&format!("[GUEST]: Returning u8 value {result}"));
        result
    }

    fn trip_u16(value: u16) -> u16 {
        bindings::log(&format!("[GUEST]: Receiving u16 value {value}"));
        let result = bindings::wasmi_component::component_examples::round_trip::trip_u16(value);
        bindings::log(&format!("[GUEST]: Returning u16 value {result}"));
        result
    }

    fn trip_u32(value: u32) -> u32 {
        bindings::log(&format!("[GUEST]: Receiving u32 value {value}"));
        let result = bindings::wasmi_component::component_examples::round_trip::trip_u32(value);
        bindings::log(&format!("[GUEST]: Returning u32 value {result}"));
        result
    }

    fn trip_u64(value: u64) -> u64 {
        bindings::log(&format!("[GUEST]: Receiving u64 value {value}"));
        let result = bindings::wasmi_component::component_examples::round_trip::trip_u64(value);
        bindings::log(&format!("[GUEST]: Returning u64 value {result}"));
        result
    }

    fn trip_f32(value: f32) -> f32 {
        bindings::log(&format!("[GUEST]: Receiving f32 value {value}"));
        let result = bindings::wasmi_component::component_examples::round_trip::trip_f32(value);
        bindings::log(&format!("[GUEST]: Returning f32 value {result}"));
        result
    }

    fn trip_f64(value: f64) -> f64 {
        bindings::log(&format!("[GUEST]: Receiving f64 value {value}"));
        let result = bindings::wasmi_component::component_examples::round_trip::trip_f64(value);
        bindings::log(&format!("[GUEST]: Returning f64 value {result}"));
        result
    }

    fn trip_bool(value: bool) -> bool {
        bindings::log(&format!("[GUEST]: Receiving bool value {value}"));
        let result = bindings::wasmi_component::component_examples::round_trip::trip_bool(value);
        bindings::log(&format!("[GUEST]: Returning bool value {result}"));
        result
    }

    fn trip_char(value: char) -> char {
        bindings::log(&format!("[GUEST]: Receiving char value {value}"));
        let result = bindings::wasmi_component::component_examples::round_trip::trip_char(value);
        bindings::log(&format!("[GUEST]: Returning char value {result}"));
        result
    }

    fn trip_string(value: String) -> String {
        bindings::log(&format!("[GUEST]: Receiving string value {value}"));
        let result = bindings::wasmi_component::component_examples::round_trip::trip_string(&value);
        bindings::log(&format!("[GUEST]: Returning string value {result}"));
        result
    }
}
