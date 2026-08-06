use heck::ToKebabCase;
use proc_macro2::TokenStream;
use quote::quote;
use syn::{DataEnum, Ident};

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
        // TODO: enums with more than 256 cases
        quote! { 1 }
    }

    fn byte_size(&self) -> TokenStream {
        // TODO: enums with more than 256 cases
        quote! { 1 }
    }

    fn lift(&self) -> TokenStream {
        let mut output = quote! {};
        for (index, field) in self.data.variants.iter().enumerate() {
            let field_name = &field.ident;
            output.extend(quote! { #index => Ok(Self::#field_name), });
        }

        let name = self.gen_data.name.to_string();
        quote! {
            reader.read_variant::<Self>(|_reader, determinant| match determinant {
                #output
                other => Err(wasmi_component::ConvertError::new(
                    format!("invalid determinant {other} in {}::lift", #name)
                )),
            })
        }
    }

    fn lower(&self) -> TokenStream {
        let main_ty = self.gen_data.name;
        let mut output = quote! {};

        for (index, field) in self.data.variants.iter().enumerate() {
            let field_name = &field.ident;
            output.extend(quote! { Self::#field_name => {
                writer.write_variant::<#main_ty, _>(#index, ())
            } });
        }

        quote! { match self { #output } }
    }

    fn borrowed_name(&self) -> Ident {
        self.gen_data.name.clone()
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
