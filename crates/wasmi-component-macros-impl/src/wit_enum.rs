use heck::ToKebabCase;
use proc_macro2::TokenStream;
use quote::quote;
use syn::DataEnum;

use crate::{Generator, GeneratorData};

pub struct EnumGenerator<'a> {
    pub data: &'a DataEnum,
    pub gen_data: GeneratorData<'a>,
}

impl Generator for EnumGenerator<'_> {
    fn value_type(&self) -> TokenStream {
        let mut output = quote! {};
        let name = self.gen_data.name.to_string().to_kebab_case();

        for field in &self.data.variants {
            let field_name = &field.ident.to_string().to_kebab_case();
            output.extend(quote! { std::rc::Rc::from(#field_name), });
        }

        quote! { wasmi_component::ValueType::Enum {
            name: std::rc::Rc::from(#name),
            cases: std::rc::Rc::from([ #output ]),
        } }
    }

    fn arg_count(&self) -> TokenStream {
        quote! { 1 }
    }

    fn byte_align(&self) -> TokenStream {
        let cases_count = self.data.variants.len();
        quote! { wasmi_component::lib_structs::enum_determinant_size(#cases_count) }
    }

    fn byte_size(&self) -> TokenStream {
        let cases_count = self.data.variants.len();
        quote! { wasmi_component::lib_structs::enum_determinant_size(#cases_count) }
    }

    fn lift(&self) -> TokenStream {
        let mut output = quote! {};

        for (index, field) in self.data.variants.iter().enumerate() {
            let field_name = &field.ident;
            output.extend(quote! { #index => Ok(Self::#field_name), });
        }

        let name = self.gen_data.name.to_string();
        let cases_count = self.data.variants.len();

        quote! {
            match reader.read_enum_determinant(#cases_count, 1) {
                #output
                other => Err(wasmi_component::ConvertError::new(
                    format!("invalid determinant {other} in {}::lift", #name)
                )),
            }
        }
    }

    fn lower(&self) -> TokenStream {
        let cases_count = self.data.variants.len();
        let mut output = quote! {};

        for (index, field) in self.data.variants.iter().enumerate() {
            let field_name = &field.ident;
            output.extend(quote! { Self::#field_name => {
                writer.write_enum_determinant(#cases_count, #index, 1)
            } });
        }

        quote! { match self { #output }; Ok(()) }
    }

    fn borrowed_def(&self) -> TokenStream {
        quote! {}
    }

    fn lift_owned(&self) -> TokenStream {
        quote! { Ok(*self) }
    }

    fn lift_to(&self) -> TokenStream {
        quote! { *target = *self; Ok(()) }
    }
}
