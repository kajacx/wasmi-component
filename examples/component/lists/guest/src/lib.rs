mod bindings {
    wit_bindgen::generate!({
        path: "../example.wit",
    });

    use super::GuestComponent;
    export!(GuestComponent);
}

struct GuestComponent;

impl bindings::exports::wasmi_component::component_examples::round_trip::Guest for GuestComponent {
    fn list_s8(value: Vec<i8>) -> Vec<i8> {
        bindings::log(&format!("[GUEST]: Receiving Vec<i8> value {value:?}"));
        let result = bindings::wasmi_component::component_examples::round_trip::list_s8(&value);
        bindings::log(&format!("[GUEST]: Returning Vec<i8> value {result:?}"));
        result
    }

    fn list_s16(value: Vec<i16>) -> Vec<i16> {
        bindings::log(&format!("[GUEST]: Receiving Vec<i16> value {value:?}"));
        let result = bindings::wasmi_component::component_examples::round_trip::list_s16(&value);
        bindings::log(&format!("[GUEST]: Returning Vec<i16> value {result:?}"));
        result
    }

    fn list_s32(value: Vec<i32>) -> Vec<i32> {
        bindings::log(&format!("[GUEST]: Receiving Vec<i32> value {value:?}"));
        let result = bindings::wasmi_component::component_examples::round_trip::list_s32(&value);
        bindings::log(&format!("[GUEST]: Returning Vec<i32> value {result:?}"));
        result
    }

    fn list_s64(value: Vec<i64>) -> Vec<i64> {
        bindings::log(&format!("[GUEST]: Receiving Vec<i64> value {value:?}"));
        let result = bindings::wasmi_component::component_examples::round_trip::list_s64(&value);
        bindings::log(&format!("[GUEST]: Returning Vec<i64> value {result:?}"));
        result
    }

    fn list_u8(value: Vec<u8>) -> Vec<u8> {
        bindings::log(&format!("[GUEST]: Receiving Vec<u8> value {value:?}"));
        let result = bindings::wasmi_component::component_examples::round_trip::list_u8(&value);
        bindings::log(&format!("[GUEST]: Returning Vec<u8> value {result:?}"));
        result
    }

    fn list_u16(value: Vec<u16>) -> Vec<u16> {
        bindings::log(&format!("[GUEST]: Receiving Vec<u16> value {value:?}"));
        let result = bindings::wasmi_component::component_examples::round_trip::list_u16(&value);
        bindings::log(&format!("[GUEST]: Returning Vec<u16> value {result:?}"));
        result
    }

    fn list_u32(value: Vec<u32>) -> Vec<u32> {
        bindings::log(&format!("[GUEST]: Receiving Vec<u32> value {value:?}"));
        let result = bindings::wasmi_component::component_examples::round_trip::list_u32(&value);
        bindings::log(&format!("[GUEST]: Returning Vec<u32> value {result:?}"));
        result
    }

    fn list_u64(value: Vec<u64>) -> Vec<u64> {
        bindings::log(&format!("[GUEST]: Receiving Vec<u64> value {value:?}"));
        let result = bindings::wasmi_component::component_examples::round_trip::list_u64(&value);
        bindings::log(&format!("[GUEST]: Returning Vec<u64> value {result:?}"));
        result
    }

    fn list_f32(value: Vec<f32>) -> Vec<f32> {
        bindings::log(&format!("[GUEST]: Receiving Vec<f32> value {value:?}"));
        let result = bindings::wasmi_component::component_examples::round_trip::list_f32(&value);
        bindings::log(&format!("[GUEST]: Returning Vec<f32> value {result:?}"));
        result
    }

    fn list_f64(value: Vec<f64>) -> Vec<f64> {
        bindings::log(&format!("[GUEST]: Receiving Vec<f64> value {value:?}"));
        let result = bindings::wasmi_component::component_examples::round_trip::list_f64(&value);
        bindings::log(&format!("[GUEST]: Returning Vec<f64> value {result:?}"));
        result
    }

    fn list_bool(value: Vec<bool>) -> Vec<bool> {
        bindings::log(&format!("[GUEST]: Receiving Vec<bool> value {value:?}"));
        let result = bindings::wasmi_component::component_examples::round_trip::list_bool(&value);
        bindings::log(&format!("[GUEST]: Returning Vec<bool> value {result:?}"));
        result
    }

    fn list_char(value: Vec<char>) -> Vec<char> {
        bindings::log(&format!("[GUEST]: Receiving Vec<char> value {value:?}"));
        let result = bindings::wasmi_component::component_examples::round_trip::list_char(&value);
        bindings::log(&format!("[GUEST]: Returning Vec<char> value {result:?}"));
        result
    }

    fn list_string(value: Vec<String>) -> Vec<String> {
        bindings::log(&format!("[GUEST]: Receiving Vec<String> value {value:?}"));
        let result = bindings::wasmi_component::component_examples::round_trip::list_string(&value);
        bindings::log(&format!("[GUEST]: Returning Vec<String> value {result:?}"));
        result
    }
}
