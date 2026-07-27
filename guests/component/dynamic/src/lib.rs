mod bindings {
    wit_bindgen::generate!({
        path: "../../../examples/component/dynamic/example.wit",
        with: {
            "wasmi-component:component-examples/round-trip@0.1.0/person": crate::Person,
        }
    });

    use super::GuestComponent;
    export!(GuestComponent);
}
struct GuestComponent;

#[derive(Debug, Clone, Default)]
pub struct Person {
    name: String,
    id: u64,
    birthday: Option<String>,
}

impl bindings::exports::wasmi_component::component_examples::round_trip::Guest for GuestComponent {
    fn trip_s32(value: i32) -> i32 {
        bindings::log(&format!("[GUEST]: Receiving i32 value {value}"));
        let result = bindings::wasmi_component::component_examples::round_trip::trip_s32(value);
        bindings::log(&format!("[GUEST]: Returning i32 value {result}"));
        result
    }

    fn trip_string(value: String) -> String {
        bindings::log(&format!("[GUEST]: Receiving string value {value}"));
        let result = bindings::wasmi_component::component_examples::round_trip::trip_string(&value);
        bindings::log(&format!("[GUEST]: Returning string value {result}"));
        result
    }

    fn trip_person(value: Person) -> Person {
        todo!()
    }

    fn list_s32(value: Vec<i32>) -> Vec<i32> {
        todo!()
    }

    fn list_string(value: Vec<String>) -> Vec<String> {
        todo!()
    }

    fn list_person(value: Vec<Person>) -> Vec<Person> {
        todo!()
    }

    fn result_s32(value: Result<i32, i32>) -> Result<i32, i32> {
        todo!()
    }

    fn result_string(value: Result<String, String>) -> Result<String, String> {
        todo!()
    }

    fn result_person(value: Result<Person, Person>) -> Result<Person, Person> {
        todo!()
    }
}
