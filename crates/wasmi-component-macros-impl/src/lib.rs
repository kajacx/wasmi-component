use proc_macro2::{Ident, TokenStream};
use quote::quote;
use syn::{DeriveInput, Visibility, parse_str, parse2};

mod record;
mod variant;
mod wit_enum;

pub fn derive_component_value_str(input: &str) -> TokenStream {
    derive_component_value(parse_str(input).expect("parse derive input"))
}

pub fn derive_component_value_stream(input: TokenStream) -> TokenStream {
    derive_component_value(parse2(input).expect("parse derive input"))
}

pub fn derive_component_value(input: DeriveInput) -> TokenStream {
    let type_name = &input.ident;
    let generator = get_generator(&input, type_name);

    let value_type = generator.value_type();
    let arg_count = generator.arg_count();

    let byte_align = generator.byte_align();
    let byte_size = generator.byte_size();

    let lift = generator.lift();
    let lower = generator.lower();

    let borrowed_name = generator.borrowed_name();
    let borrowed_def = generator.borrowed_def();

    let lift_owned = generator.lift_owned();
    let lift_to = generator.lift_to();

    let (borrowed_lifetime_a, borrowed_lifetime_anon) = if &borrowed_name == type_name {
        (quote! {}, quote! {})
    } else {
        (quote! { <'a> }, quote! { <'_> })
    };

    quote! {
        impl wasmi_component::ComponentValue for #type_name {
            type Borrowed<'a> = #borrowed_name #borrowed_lifetime_a;

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

        #borrowed_def

        impl wasmi_component::Lift<#type_name> for #borrowed_name #borrowed_lifetime_anon {
            fn lift_owned(&self) -> wasmi_component::ConvertResult<#type_name> {
                #lift_owned
            }

            fn lift_to(&self, target: &mut #type_name) -> wasmi_component::ConvertResult<()> {
                #lift_to
            }
        }
    }
}

#[blanket::blanket(derive(Box))]
trait Generator {
    fn value_type(&self) -> TokenStream;
    fn arg_count(&self) -> TokenStream;

    fn byte_align(&self) -> TokenStream;
    fn byte_size(&self) -> TokenStream;

    fn lift(&self) -> TokenStream;
    fn lower(&self) -> TokenStream;

    fn borrowed_name(&self) -> Ident;
    fn borrowed_def(&self) -> TokenStream;

    fn lift_owned(&self) -> TokenStream;
    fn lift_to(&self) -> TokenStream;
}

struct GeneratorData<'a> {
    name: &'a Ident,
    vis: &'a Visibility,
}

fn get_generator<'a>(input: &'a DeriveInput, name: &'a Ident) -> Box<dyn Generator + 'a> {
    let gen_data = GeneratorData {
        name,
        vis: &input.vis,
    };

    match &input.data {
        syn::Data::Struct(data) => Box::new(record::RecordGenerator { data, gen_data }),
        syn::Data::Enum(data) => {
            if data
                .variants
                .iter()
                .all(|variant| variant.fields.is_empty())
            {
                Box::new(wit_enum::EnumGenerator { data, gen_data })
            } else {
                Box::new(variant::VariantGenerator { data, gen_data })
            }
        }
        syn::Data::Union(_) => {
            unimplemented!("unions are currently not implemented ({})", &input.ident)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::derive_component_value_str;

    #[test]
    fn generated_value_type_names_use_kebab_case() {
        let tokens = derive_component_value_str(r#"struct MyType { first_name: u32 }"#);
        let rendered = tokens.to_string();

        assert!(rendered.contains("\"my-type\""), "{rendered}");
        assert!(rendered.contains("\"first-name\""), "{rendered}");
    }

    #[test]
    fn generated_enum_case_names_use_kebab_case() {
        let tokens = derive_component_value_str(r#"enum MyType { FirstVariant, SecondVariant }"#);
        let rendered = tokens.to_string();

        assert!(rendered.contains("\"first-variant\""), "{rendered}");
        assert!(rendered.contains("\"second-variant\""), "{rendered}");
    }
}
