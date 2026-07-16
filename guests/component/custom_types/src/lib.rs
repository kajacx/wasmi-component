mod bindings {
    wit_bindgen::generate!({
        path: "../../../examples/component/custom_types/example.wit",
        with: {
            "wasmi-component:component-examples/round-trip@0.1.0/person": crate::Person,
            "wasmi-component:component-examples/round-trip@0.1.0/data": crate::Data
        }
    });

    use super::GuestComponent;
    export!(GuestComponent);
}

#[derive(Debug, Clone, Default)]
pub struct Person {
    name: String,
    id: u64,
}

#[derive(Debug, Clone)]
pub enum Data {
    Number(f64),
    Text(String),
}

struct GuestComponent;

impl bindings::exports::wasmi_component::component_examples::round_trip::Guest for GuestComponent {
    fn trip_person(value: Person) -> Person {
        bindings::log(&format!("[GUEST]: Receiving Person value {value:?}"));
        let value = bindings::wasmi_component::component_examples::round_trip::trip_person(&value);
        bindings::log(&format!("[GUEST]: Returning Person value {value:?}"));
        value
    }

    fn trip_data(value: Data) -> Data {
        bindings::log(&format!("[GUEST]: Receiving Data value {value:?}"));
        let value = bindings::wasmi_component::component_examples::round_trip::trip_data(&value);
        bindings::log(&format!("[GUEST]: Returning Data value {value:?}"));
        value
    }

    fn trip_mixed(_a: crate::Person, _b: i32, _c: Result<crate::Data, String>) -> () {
        bindings::log(&format!("[GUEST]: Hello trip mixed"));
    }
}

impl bindings::Guest for GuestComponent {
    fn init(_args: Vec<String>) -> bindings::Outcome {
        bindings::log(&format!("[GUEST]: Hello init"));
        bindings::Outcome::Ok
    }
}

impl bindings::exports::additional_exports::Guest for GuestComponent {
    fn pet(
        _target: bindings::exports::additional_exports::Animal,
        _pets: u32,
    ) -> Result<(), String> {
        bindings::log(&format!("[GUEST]: Hello pet"));
        Ok(())
    }
}
