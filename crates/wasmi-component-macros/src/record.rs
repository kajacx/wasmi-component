use proc_macro2::TokenStream;
use quote::quote;
use syn::{DataStruct, Ident};

use crate::Generator;

pub struct RecordGenerator<'a> {
    #[allow(unused)]
    pub name: &'a Ident,
    pub data: &'a DataStruct,
}

impl Generator for RecordGenerator<'_> {
    fn value_type(&self) -> TokenStream {
        let mut output = quote! {};
        for field in &self.data.fields {
            let field_name = field.ident.as_ref().unwrap().to_string();
            let field_ty = &field.ty;
            output.extend(quote! { (#field_name.to_string(), #field_ty::value_type()), });
        }
        quote! { wasmi_component::ValueType::Record(vec![ #output ]) }
    }

    fn arg_count(&self) -> TokenStream {
        let mut output = quote! { 0 };
        for field in &self.data.fields {
            let field_ty = &field.ty;
            output.extend(quote! { + #field_ty::arg_count()});
        }
        output
    }

    fn arg_types(&self) -> TokenStream {
        let mut output = quote! { let mut types = Vec::new(); };
        for field in &self.data.fields {
            let field_ty = &field.ty;
            output.extend(quote! { types.extend(#field_ty::arg_types()); });
        }
        output.extend(quote! { types });
        output
    }

    fn lift_args(&self) -> TokenStream {
        let mut output = quote! { use wasmi_component::View; let mut index = 0; };
        let mut result = quote! {};

        for field in &self.data.fields {
            let field_ty = &field.ty;
            let field_name = field.ident.as_ref().unwrap();
            output.extend(quote! { let #field_name = #field_ty::lift_args(&args[index .. (index + #field_ty::arg_count())], memory)?; });
            output.extend(quote! { let #field_name = #field_name.lift_owned()?; });
            output.extend(quote! { index += #field_ty::arg_count(); });
            result.extend(quote! { #field_name, });
        }

        output.extend(quote! { Ok( Self { #result } ) });
        output
    }

    fn lower_args(&self) -> TokenStream {
        let mut output = quote! { let mut index = 0; };

        for field in &self.data.fields {
            let field_ty = &field.ty;
            let field_name = field.ident.as_ref().unwrap();
            output.extend(quote! { #field_ty::lower_args(&self.#field_name, &mut args[index .. (index + #field_ty::arg_count())], memory)?; });
            output.extend(quote! { index += #field_ty::arg_count(); });
        }

        output.extend(quote! { Ok(()) });
        output
    }

    fn byte_align(&self) -> TokenStream {
        let mut output = quote! { let mut result = 0; };
        for field in &self.data.fields {
            let field_ty = &field.ty;
            output.extend(quote! { result = std::cmp::max(result, #field_ty::byte_align()); });
        }
        output.extend(quote! { result });
        output
    }

    fn byte_size(&self) -> TokenStream {
        let mut output = quote! { let align = Self::byte_align(); let mut result = 0; };
        for field in &self.data.fields {
            let field_ty = &field.ty;
            output.extend(
                quote! { result += wasmi_component::helpers::round_up(#field_ty::byte_size(), align); },
            );
        }
        output.extend(quote! { result });
        output
    }

    fn lift_bytes(&self) -> TokenStream {
        let mut output = quote! { use wasmi_component::View; let align = Self::byte_align(); let mut index = 0; };
        let mut result = quote! {};

        for field in &self.data.fields {
            let field_ty = &field.ty;
            let field_name = field.ident.as_ref().unwrap();
            output.extend(quote! { let #field_name = #field_ty::lift_bytes(&bytes[index .. (index + #field_ty::byte_size())], memory)?; });
            output.extend(quote! { let #field_name = #field_name.lift_owned()?; });
            output.extend(quote! { index += wasmi_component::helpers::round_up(#field_ty::byte_size(), align); });
            result.extend(quote! { #field_name, });
        }

        output.extend(quote! { Ok( Self { #result } ) });
        output
    }

    fn lower_bytes(&self) -> TokenStream {
        let mut output = quote! { let align = Self::byte_align(); let mut index = range.start; };

        for field in &self.data.fields {
            let field_ty = &field.ty;
            let field_name = field.ident.as_ref().unwrap();
            output.extend(quote! { #field_ty::lower_bytes(&self.#field_name, index .. (index + #field_ty::byte_size()), memory)?; });
            output.extend(quote! { index += wasmi_component::helpers::round_up(#field_ty::arg_count(), align); });
        }

        output.extend(quote! { Ok(()) });
        output
    }
}
