use proc_macro2::TokenStream;
use quote::quote;
use syn::DataStruct;

use crate::{Generator, GeneratorData};

pub struct RecordGenerator<'a> {
    pub data: &'a DataStruct,
    pub gen_data: GeneratorData<'a>,
}

impl Generator for RecordGenerator<'_> {
    fn value_type(&self) -> TokenStream {
        let mut output = quote! {};

        for field in &self.data.fields {
            let field_name = field.ident.as_ref().unwrap().to_string();
            let field_ty = &field.ty;
            output.extend(quote! { (std::rc::Rc::from(#field_name), <#field_ty>::value_type()), });
        }

        let name = self.gen_data.name.to_string();
        quote! { wasmi_component::ValueType::Record {
            name: std::rc::Rc::from(#name),
            fields: std::rc::Rc::from([ #output ]),
        } }
    }

    fn arg_count(&self) -> TokenStream {
        let mut output = quote! { 0 };
        for field in &self.data.fields {
            let field_ty = &field.ty;
            output.extend(quote! { + <#field_ty>::arg_count()});
        }
        output
    }

    fn lower_args(&self) -> TokenStream {
        let mut output = quote! { let mut index = 0; };

        for field in &self.data.fields {
            let field_ty = &field.ty;
            let field_name = field.ident.as_ref().unwrap();
            output.extend(quote! { <#field_ty>::lower_args(&self.#field_name, &mut args[index .. (index + <#field_ty>::arg_count())], memory)?; });
            output.extend(quote! { index += <#field_ty>::arg_count(); });
        }

        output.extend(quote! { Ok(()) });
        output
    }

    fn byte_align(&self) -> TokenStream {
        let mut output = quote! { let mut result = 0; };
        for field in &self.data.fields {
            let field_ty = &field.ty;
            output.extend(quote! { result = std::cmp::max(result, <#field_ty>::byte_align()); });
        }
        output.extend(quote! { result });
        output
    }

    fn byte_size(&self) -> TokenStream {
        let mut output = quote! { let align = Self::byte_align(); let mut result = 0; };
        for field in &self.data.fields {
            let field_ty = &field.ty;
            output.extend(
                quote! { result += wasmi_component::helpers::round_up(<#field_ty>::byte_size(), align); },
            );
        }
        output.extend(quote! { result });
        output
    }

    fn lift(&self) -> TokenStream {
        let mut output = quote! { let align = Self::byte_align(); };
        let mut result = quote! {};

        for field in &self.data.fields {
            let field_ty = &field.ty;
            let field_name = field.ident.as_ref().unwrap();
            output.extend(
                quote! { let #field_name = reader.read_record_field::<#field_ty>(align)?; },
            );
            result.extend(quote! { #field_name, });
        }

        let borrowed_name = &self.gen_data.borrowed_name;
        output.extend(quote! { Ok( #borrowed_name { #result } ) });
        output
    }

    fn lower_bytes(&self) -> TokenStream {
        let mut output = quote! { let align = Self::byte_align(); let mut index = range.start; };

        for field in &self.data.fields {
            let field_ty = &field.ty;
            let field_name = field.ident.as_ref().unwrap();
            output.extend(quote! { <#field_ty>::lower_bytes(&self.#field_name, index .. (index + <#field_ty>::byte_size()), memory)?; });
            output.extend(quote! { index += wasmi_component::helpers::round_up(<#field_ty>::arg_count(), align); });
        }

        output.extend(quote! { Ok(()) });
        output
    }

    fn borrowed_def(&self) -> TokenStream {
        let mut output = quote! {};

        for field in &self.data.fields {
            let field_ty = &field.ty;
            let field_name = field.ident.as_ref().unwrap();
            let vis = &field.vis;
            output.extend(quote! { #vis #field_name: <#field_ty as wasmi_component::ComponentValue>::Borrowed<'a>, });
        }

        let vis = &self.gen_data.vis;
        let borrowed_name = &self.gen_data.borrowed_name;
        quote! { #vis struct #borrowed_name<'a> { #output } }
    }

    fn lift_owned(&self) -> TokenStream {
        let mut output = quote! {};

        for field in &self.data.fields {
            let field_name = field.ident.as_ref().unwrap();
            output.extend(quote! { #field_name: self.#field_name.lift_owned()?, });
        }

        let name = &self.gen_data.name;
        quote! { Ok(#name { #output }) }
    }

    fn lift_to(&self) -> TokenStream {
        let mut output = quote! {};

        for field in &self.data.fields {
            let field_name = field.ident.as_ref().unwrap();
            output.extend(quote! { self.#field_name.lift_to(&mut target.#field_name)?; });
        }

        output.extend(quote! {Ok(())});
        output
    }
}
