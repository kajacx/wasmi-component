mod bindings {
    wit_bindgen::generate!({
        path: "../../../examples/component/custom_types/example.wit",
        with: {
            "wasmi-component:component-examples/round-trip@0.1.0/person": crate::Person,
            "wasmi-component:component-examples/round-trip@0.1.0/data": crate::Data,
            "wasmi-component:component-examples/round-trip@0.1.0/status": crate::Status
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Status {
    Ok,
    Error,
}

impl Status {
    #[doc(hidden)]
    pub unsafe fn _lift(val: u8) -> Status {
        match val {
            0 => Status::Ok,
            1 => Status::Error,
            _ => panic!("invalid enum discriminant"),
        }
    }
}

struct GuestComponent;

impl bindings::exports::wasmi_component::component_examples::round_trip::Guest for GuestComponent {
    fn trip_person(value: Person) -> Person {
        bindings::log(&format!("[GUEST]: Receiving person value {value:?}"));
        let value = bindings::wasmi_component::component_examples::round_trip::trip_person(&value);
        bindings::log(&format!("[GUEST]: Returning person value {value:?}"));
        value
    }

    fn trip_data(value: Data) -> Data {
        bindings::log(&format!("[GUEST]: Receiving data value {value:?}"));
        let value = bindings::wasmi_component::component_examples::round_trip::trip_data(&value);
        bindings::log(&format!("[GUEST]: Returning data value {value:?}"));
        value
    }

    fn trip_status(value: Status) -> Status {
        bindings::log(&format!("[GUEST]: Receiving status value {value:?}"));
        let value = bindings::wasmi_component::component_examples::round_trip::trip_status(value);
        bindings::log(&format!("[GUEST]: Returning status value {value:?}"));
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
        let price = bindings::additional_imports::price(&[]);
        bindings::log(&format!("[GUEST]: Hello pet, price is: {price}"));
        Ok(())
    }
}
