use proc_macro2::{Ident, TokenStream};
use quote::quote;
use syn::DataEnum;

use crate::Generator;

pub struct VariantGenerator<'a> {
    pub name: &'a Ident,
    pub data: &'a DataEnum,
}

impl Generator for VariantGenerator<'_> {
    fn value_type(&self) -> TokenStream {
        let mut output = quote! {};
        for field in &self.data.variants {
            let field_name = &field.ident.to_string();
            let field_ty = &field
                .fields
                .iter()
                .next()
                .map_or_else(|| quote! { None }, |ty| quote! {Some(#ty::value_type())});

            output.extend(quote! { (#field_name.to_string(), #field_ty), });
        }
        quote! { wasmi_component::ValueType::Variant(vec![ #output ]) }
    }

    fn arg_count(&self) -> TokenStream {
        let mut output = quote! { let mut max = 0; };

        for field in &self.data.variants {
            let field_ty = &field.fields.iter().next().map(|item| &item.ty);

            if let Some(ty) = field_ty {
                output.extend(quote! { max = std::cmp::max(max, #ty::arg_count()); });
            }
        }

        output.extend(quote! { 1 + max });
        output
    }

    fn arg_types(&self) -> TokenStream {
        let mut output = quote! {};

        for field in &self.data.variants {
            let field_ty = &field.fields.iter().next().map(|item| &item.ty);

            if let Some(ty) = field_ty {
                output.extend(quote! { #ty::arg_types(), });
            }
        }

        quote! { wasmi_component::helpers::variant_types([#output]) }
    }

    fn lift_args(&self) -> TokenStream {
        let mut output = quote! {};

        for (index, field) in self.data.variants.iter().enumerate() {
            let index = index as i32;
            let field_ty = &field.fields.iter().next().map(|item| &item.ty);
            let field_name = &field.ident;

            let value = if let Some(ty) = field_ty {
                quote! { (#ty::lift_args(&args[1..(1 + #ty::arg_count())], memory)?.lift_owned()?) }
            } else {
                quote! {}
            };

            output.extend(quote! { #index => Ok(Self::#field_name #value), });
        }

        let name = &self.name.to_string();

        quote! {
            use wasmi_component::View;
            match args[0].i32()? {
                #output
                other => Err(wasmi_component::ConvertError::new(format!("invalid determinant {other} in {}::lift_args", #name)))
            }
        }
    }

    fn lower_args(&self) -> TokenStream {
        let mut output = quote! {};

        for (index, field) in self.data.variants.iter().enumerate() {
            let index = index as i32;
            let field_ty = &field.fields.iter().next().map(|item| &item.ty);
            let field_name = &field.ident;

            if let Some(ty) = field_ty {
                output.extend(quote! { Self::#field_name(value) => {
                    args[0] = wasmi_component::WasmValue::I32(#index);
                    value.lower_args(&mut args[1..(1 + #ty::arg_count())], memory)?;
                    1 + #ty::arg_count()
                } });
            } else {
                output.extend(quote! { Self::#field_name => {
                    args[0] = wasmi_component::WasmValue::I32(#index);
                    1
                } });
            }
        }

        quote! {
            let written = match self { #output };

            for arg in &mut args[written..] {
                *arg = wasmi_component::WasmValue::Unused;
            }

            Ok(())
        }
    }

    fn byte_align(&self) -> TokenStream {
        // TODO: variant with more than 256 cases
        let mut output = quote! { let mut max = 1; };

        for field in &self.data.variants {
            let field_ty = &field.fields.iter().next().map(|item| &item.ty);

            if let Some(ty) = field_ty {
                output.extend(quote! { max = std::cmp::max(max, #ty::byte_align()); });
            }
        }

        output.extend(quote! { max });
        output
    }

    fn byte_size(&self) -> TokenStream {
        let mut output = quote! { let mut max = 0; };

        for field in &self.data.variants {
            let field_ty = &field.fields.iter().next().map(|item| &item.ty);

            if let Some(ty) = field_ty {
                output.extend(quote! { max = std::cmp::max(max, #ty::byte_align()); });
            }
        }

        // TODO: variant with more than 256 cases
        output.extend(quote! { Self::byte_align() + max });
        output
    }

    fn lift_bytes(&self) -> TokenStream {
        let mut output = quote! {};

        for (index, field) in self.data.variants.iter().enumerate() {
            let index = index as u8; // TODO: more than 256 variants
            let field_ty = &field.fields.iter().next().map(|item| &item.ty);
            let field_name = &field.ident;

            let value = if let Some(ty) = field_ty {
                quote! { (#ty::lift_bytes(&bytes[offset..(#ty::byte_size() + offset)], memory)?.lift_owned()?) }
            } else {
                quote! {}
            };

            output.extend(quote! { #index => Ok(Self::#field_name #value), });
        }

        let name = &self.name.to_string();

        quote! {
            use wasmi_component::View;
            let offset = Self::byte_align();
            match bytes[0] {
                #output
                other => Err(wasmi_component::ConvertError::new(format!("invalid determinant {other} in {}::lift_bytes", #name)))
            }
        }
    }

    fn lower_bytes(&self) -> TokenStream {
        let mut output = quote! {};

        for (index, field) in self.data.variants.iter().enumerate() {
            let index = index as u8; // TODO: more than 256 variants
            let field_ty = &field.fields.iter().next().map(|item| &item.ty);
            let field_name = &field.ident;

            if let Some(ty) = field_ty {
                output.extend(quote! { Self::#field_name(value) => {
                    memory
                        .slice(range.start..(range.start + 1))?
                        .copy_from_slice(&[#index]);

                    value.lower_bytes(range.slice(offset..(offset + #ty::byte_size())), memory)
                } });
            } else {
                output.extend(quote! { Self::#field_name => {
                    memory
                        .slice(range.start..(range.start + 1))?
                        .copy_from_slice(&[0]);

                    Ok(())
                } });
            }
        }

        quote! {
            use wasmi_component::Slice;
            let offset = Self::byte_align();
            match self { #output }
        }
    }
}
