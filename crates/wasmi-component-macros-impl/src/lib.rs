use proc_macro2::{Ident, TokenStream};
use quote::quote;
use syn::{DeriveInput, Visibility, parse_str, parse2};

mod record;
mod variant;

pub fn derive_component_value_str(input: &str) -> TokenStream {
    derive_component_value(parse_str(input).expect("parse derive input"))
}

pub fn derive_component_value_stream(input: TokenStream) -> TokenStream {
    derive_component_value(parse2(input).expect("parse derive input"))
}

pub fn derive_component_value(input: DeriveInput) -> TokenStream {
    let type_name = &input.ident;
    let borrowed_name = Ident::new(&format!("{}Borrowed", type_name), type_name.span());

    let generator = get_generator(&input, type_name, &borrowed_name);

    let value_type = generator.value_type();
    let arg_count = generator.arg_count();

    let byte_align = generator.byte_align();
    let byte_size = generator.byte_size();

    let lift = generator.lift();
    let lower = generator.lower();

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

            fn byte_align() -> usize {
                #byte_align
            }

            fn byte_size() -> usize {
                #byte_size
            }

            fn lift<'mem>(
                reader: &mut impl wasmi_component::lib_structs::LiftReader<'mem>
            ) -> wasmi_component::ConvertResult<Self::Borrowed<'mem>> {
                #lift
            }
        }

        impl wasmi_component::Lower<Self> for #type_name {
            fn lower(
                &self,
                writer: &mut impl wasmi_component::lib_structs::LowerWriter
            ) -> wasmi_component::ConvertResult<()> {
                #lower
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
    };

    output
}

#[blanket::blanket(derive(Box))]
trait Generator {
    fn value_type(&self) -> TokenStream;
    fn arg_count(&self) -> TokenStream;

    fn byte_align(&self) -> TokenStream;
    fn byte_size(&self) -> TokenStream;

    fn lift(&self) -> TokenStream;
    fn lower(&self) -> TokenStream;

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
