mod bindings {
    wit_bindgen::generate!({
        path: "../example.wit",
        with: {
            "wasmi-component:component-examples/round-trip@0.1.0/person": crate::Person,
            "wasmi-component:component-examples/round-trip@0.1.0/data": crate::Data
        }
    });

    use super::GuestComponent;
    export!(GuestComponent);
}

#[derive(Debug, Clone)]
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
}
