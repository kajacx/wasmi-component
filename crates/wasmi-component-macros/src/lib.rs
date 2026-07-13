use proc_macro2::TokenStream;
use quote::quote;
use syn::{DeriveInput, parse_macro_input};

mod record;
mod variant;

#[proc_macro_derive(ComponentValue)]
pub fn derive_component_value(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let type_name = &input.ident;

    let generator = get_generator(&input);
    let value_type = generator.value_type();

    let arg_count = generator.arg_count();
    let arg_types = generator.arg_types();

    let lift_args = generator.lift_args();
    let lower_args = generator.lower_args();

    let byte_align = generator.byte_align();
    let byte_size = generator.byte_size();

    let lift_bytes = generator.lift_bytes();
    let lower_bytes = generator.lower_bytes();

    let output = quote! {
        impl wasmi_component::ComponentValue for #type_name {
            type Borrowed<'a> = Self;

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

        impl wasmi_component::View<Self> for #type_name {
            fn lift_owned(&self) -> wasmi_component::ConvertResult<Self> {
                Ok(self.clone())
            }

            fn lift_to(&self, target: &mut Self) -> wasmi_component::ConvertResult<()> {
                target.clone_from(self);
                Ok(())
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
}

fn get_generator(input: &DeriveInput) -> Box<dyn Generator + '_> {
    match &input.data {
        syn::Data::Struct(data) => Box::new(record::RecordGenerator {
            name: &input.ident,
            data,
        }),
        syn::Data::Enum(data) => Box::new(variant::VariantGenerator {
            name: &input.ident,
            data,
        }),
        syn::Data::Union(_) => {
            unimplemented!("Unions are not currently implemented ({})", &input.ident)
        }
    }
}
