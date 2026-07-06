mod bindings {
    wit_bindgen::generate!({
        path: "../example.wit",
    });

    use super::GuestComponent;
    export!(GuestComponent);
}

struct GuestComponent;

impl bindings::exports::wasmi_component::wasi_examples::exported_funcs::Guest for GuestComponent {
    fn print_stdout(text: String) -> () {
        println!("{text}");
    }

    fn print_stderr(text: String) -> () {
        eprintln!("{text}");
    }
}
