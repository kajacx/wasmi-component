use proc_macro2::{Ident, TokenStream};
use quote::quote;
use syn::DataEnum;

use crate::Generator;

pub struct VariantGenerator<'a> {
    #[allow(unused)]
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

        output.extend(quote! { max });
        output
    }

    fn arg_types(&self) -> TokenStream {
        // TODO: this just grabs first non-empty field and is horrible
        let field_ty = &self
            .data
            .variants
            .iter()
            .filter_map(|case| case.fields.iter().next())
            .next()
            .unwrap()
            .ty;

        quote! {
            let mut types = vec![wasmi_component::wasmi::ValType::I32];
            types.extend(#field_ty::arg_types());
            types
        }
    }

    fn lift_args(&self) -> TokenStream {
        let mut output = quote! {};

        for (index, field) in self.data.variants.iter().enumerate() {
            let index = index as i32;
            let field_ty = &field.fields.iter().next().map(|item| &item.ty);
            let field_name = &field.ident;

            let value = if let Some(ty) = field_ty {
                quote! { (#field_ty::lift_args(&args[1..(1 + #ty::arg_count())], memory)?) }
            } else {
                quote! {}
            };

            output.extend(quote! { #index => {
                Ok(Self::#field_name #value)
            } });
        }

        quote! { match args[0].i32().unwrap() {
            #output
            other => Err(wasmi_component::ConvertError::new(format!("Invalid determinant {other} in TODO: name")))
        } }
    }

    fn lower_args(&self) -> TokenStream {
        let mut output = quote! {};

        for (index, field) in self.data.variants.iter().enumerate() {
            let index = index as i32;
            let field_ty = &field.fields.iter().next().map(|item| &item.ty);
            let field_name = &field.ident;

            if let Some(ty) = field_ty {
                output.extend(quote! { Self::#field_name(value) => {
                    args[0] = wasmi_component::wasmi::Val::I32(#index);
                    value.lower_args(&mut args[1..(1 + #ty::arg_count())], memory)
                } });
            } else {
                output.extend(quote! { Self::#field_name => {
                    args[0] = wasmi_component::wasmi::Val::I32(#index);
                    Ok(())
                } });
            }
        }

        quote! { match self { #output } }
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
        quote! { todo!("lift_bytes variant") }
    }

    fn lower_bytes(&self) -> TokenStream {
        quote! { todo!("lower_bytes variant") }
    }

    // fn lower_bytes(
    //     &self,
    //     range: Range<usize>,
    //     memory: &mut impl MemoryAccess,
    // ) -> ConvertResult<()> {
    //     debug_assert_eq!(range.len(), Option::<T>::byte_size());

    //     let offset = Option::<T>::byte_align();

    //     match self {
    //         None => {
    //             memory
    //                 .slice(range.start..(range.start + 1))?
    //                 .copy_from_slice(&[0]);

    //             Ok(())
    //         }
    //         Some(val) => {
    //             memory
    //                 .slice(range.start..(range.start + 1))?
    //                 .copy_from_slice(&[1]);

    //             val.lower_bytes(range.slice(offset..(offset + T::byte_size())), memory)
    //         }
    //     }
    // }
}
