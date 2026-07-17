use proc_macro2::{Ident, TokenStream};
use quote::quote;
use syn::{DeriveInput, Visibility, parse_macro_input};

mod record;
mod variant;

#[proc_macro_derive(ComponentValue)]
pub fn derive_component_value(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let type_name = &input.ident;
    let borrowed_name = Ident::new(&format!("{}Borrowed", type_name), type_name.span());

    let generator = get_generator(&input, type_name, &borrowed_name);
    let value_type = generator.value_type();

    let arg_count = generator.arg_count();
    let arg_types = generator.arg_types();

    let lift_args = generator.lift_args();
    let lower_args = generator.lower_args();

    let byte_align = generator.byte_align();
    let byte_size = generator.byte_size();

    let lift_bytes = generator.lift_bytes();
    let lower_bytes = generator.lower_bytes();

    let borrowed_def = generator.borrowed_def();

    let lift_owned = generator.lift_owned();
    let lift_to = generator.lift_to();

    let output = quote! {
        impl wasmi_component::ComponentValue for #type_name {
            type Borrowed<'a> = #borrowed_name<'a>;

            fn value_type() -> wasmi_component::ValueType {
                #value_type
            }

            fn arg_count() -> usize {
                #arg_count
            }

            fn arg_types() -> Vec<wasmi_component::wasmi::ValType>{
                #arg_types
            }

            fn lift_args<'a>(args: &[wasmi_component::WasmValue], memory: &'a [u8]) -> wasmi_component::ConvertResult<Self::Borrowed<'a>> {
                #lift_args
            }

            fn byte_align() -> usize {
                #byte_align
            }

            fn byte_size() -> usize {
                #byte_size
            }

            fn lift_bytes<'a>(bytes: &[u8], memory: &'a [u8]) -> wasmi_component::ConvertResult<Self::Borrowed<'a>> {
                #lift_bytes
            }
        }

        #[derive(Clone, Debug)]
        #borrowed_def

        impl wasmi_component::Lift<#type_name> for #borrowed_name<'_> {
            fn lift_owned(&self) -> wasmi_component::ConvertResult<#type_name> {
                #lift_owned
            }

            fn lift_to(&self, target: &mut #type_name) -> wasmi_component::ConvertResult<()> {
                #lift_to
            }
        }

        impl wasmi_component::LowerValue<Self> for #type_name {
            fn lower_args(&self, args: &mut [wasmi_component::WasmValue], memory: &mut impl wasmi_component::MemoryAccess) -> wasmi_component::ConvertResult<()> {
                #lower_args
            }

            fn lower_bytes(&self, range: std::ops::Range<usize>, memory: &mut impl wasmi_component::MemoryAccess) -> wasmi_component::ConvertResult<()> {
                #lower_bytes
            }
        }
    };

    proc_macro::TokenStream::from(output)
}

#[blanket::blanket(derive(Box))]
trait Generator {
    fn value_type(&self) -> TokenStream;

    fn arg_count(&self) -> TokenStream;
    fn arg_types(&self) -> TokenStream;

    fn lift_args(&self) -> TokenStream;
    fn lower_args(&self) -> TokenStream;

    fn byte_align(&self) -> TokenStream;
    fn byte_size(&self) -> TokenStream;

    fn lift_bytes(&self) -> TokenStream;
    fn lower_bytes(&self) -> TokenStream;

    fn borrowed_def(&self) -> TokenStream;

    fn lift_owned(&self) -> TokenStream;
    fn lift_to(&self) -> TokenStream;
}

struct GeneratorData<'a> {
    name: &'a Ident,
    borrowed_name: &'a Ident,
    vis: &'a Visibility,
}

fn get_generator<'a>(
    input: &'a DeriveInput,
    name: &'a Ident,
    borrowed_name: &'a Ident,
) -> Box<dyn Generator + 'a> {
    let gen_data = GeneratorData {
        name,
        borrowed_name,
        vis: &input.vis,
    };

    match &input.data {
        syn::Data::Struct(data) => Box::new(record::RecordGenerator { data, gen_data }),
        syn::Data::Enum(data) => Box::new(variant::VariantGenerator { data, gen_data }),
        syn::Data::Union(_) => {
            unimplemented!("Unions are not currently implemented ({})", &input.ident)
        }
    }
}
