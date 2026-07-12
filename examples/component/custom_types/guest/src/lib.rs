mod bindings {
    wit_bindgen::generate!({
        path: "../example.wit",
    });

    use super::GuestComponent;
    export!(GuestComponent);
}
struct GuestComponent;

impl bindings::exports::wasmi_component::component_examples::round_trip::Guest for GuestComponent {
    fn trip_person(
        value: bindings::exports::wasmi_component::component_examples::round_trip::Person,
    ) -> bindings::exports::wasmi_component::component_examples::round_trip::Person {
        bindings::log(&format!("[GUEST]: Receiving Person value {value:?}"));
        let value = bindings::wasmi_component::component_examples::round_trip::trip_person(
            &guest_to_host(value),
        );
        bindings::log(&format!("[GUEST]: Returning Person value {value:?}"));
        host_to_guest(value)
    }
}

fn guest_to_host(
    person: bindings::exports::wasmi_component::component_examples::round_trip::Person,
) -> bindings::wasmi_component::component_examples::round_trip::Person {
    bindings::wasmi_component::component_examples::round_trip::Person {
        id: person.id,
        name: person.name.clone(),
    }
}

fn host_to_guest(
    person: bindings::wasmi_component::component_examples::round_trip::Person,
) -> bindings::exports::wasmi_component::component_examples::round_trip::Person {
    bindings::exports::wasmi_component::component_examples::round_trip::Person {
        id: person.id,
        name: person.name.clone(),
    }
}
