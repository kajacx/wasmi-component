#[proc_macro_derive(ComponentValue)]
pub fn derive_component_value(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = proc_macro2::TokenStream::from(input);

    let output = wasmi_component_macros_impl::derive_component_value_stream(input);

    proc_macro::TokenStream::from(output)
}
