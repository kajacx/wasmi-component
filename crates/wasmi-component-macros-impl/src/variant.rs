use heck::ToKebabCase;
use proc_macro2::TokenStream;
use quote::quote;
use syn::{DataEnum, Ident};

use crate::{Generator, GeneratorData};

pub struct VariantGenerator<'a> {
    pub data: &'a DataEnum,
    pub gen_data: GeneratorData<'a>,
}

impl Generator for VariantGenerator<'_> {
    fn value_type(&self) -> TokenStream {
        let mut output = quote! {};

        for field in &self.data.variants {
            let field_name = &field.ident.to_string().to_kebab_case();
            let field_ty = &field.fields.iter().next().map_or_else(
                || quote! { None },
                |ty| quote! { Some(<#ty>::value_type()) },
            );

            output.extend(quote! { (std::rc::Rc::from(#field_name), #field_ty), });
        }

        let name = self.gen_data.name.to_string().to_kebab_case();
        quote! { wasmi_component::ValueType::Variant {
            name: std::rc::Rc::from(#name),
            cases: std::rc::Rc::from([ #output ]),
        } }
    }

    fn arg_count(&self) -> TokenStream {
        let mut output = quote! { let mut max = 0; };

        for field in &self.data.variants {
            let field_ty = &field.fields.iter().next().map(|item| &item.ty);

            if let Some(ty) = field_ty {
                output.extend(quote! { max = std::cmp::max(max, <#ty>::arg_count()); });
            }
        }

        output.extend(quote! { 1 + max });
        output
    }

    fn byte_align(&self) -> TokenStream {
        let cases_count = self.data.variants.len();
        let mut output = quote! { let mut max = wasmi_component::lib_structs::enum_determinant_size(#cases_count); };

        for field in &self.data.variants {
            let field_ty = &field.fields.iter().next().map(|item| &item.ty);

            if let Some(ty) = field_ty {
                output.extend(quote! { max = std::cmp::max(max, <#ty>::byte_align()); });
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
                output.extend(quote! { max = std::cmp::max(max, <#ty>::byte_align()); });
            }
        }

        output.extend(quote! { Self::byte_align() + max });
        output
    }

    fn lift(&self) -> TokenStream {
        let mut output = quote! {};

        for (index, field) in self.data.variants.iter().enumerate() {
            let field_ty = &field.fields.iter().next().map(|item| &item.ty);
            let field_name = &field.ident;
            let borrowed_name = &self.borrowed_name();

            if let Some(ty) = field_ty {
                output.extend(
                    quote! { #index => Ok(#borrowed_name::#field_name(<#ty>::lift(reader)?)), },
                );
            } else {
                output.extend(quote! { #index => Ok(#borrowed_name::#field_name), });
            }
        }

        let name = &self.gen_data.name.to_string();
        let cases_count = self.data.variants.len();

        quote! {
            reader.read_variant::<Self>(#cases_count, |reader, determinant| match determinant {
                #output
                other => Err(wasmi_component::ConvertError::new(
                    format!("invalid determinant {other} in {}::lift", #name)
                )),
            })
        }
    }

    fn lower(&self) -> TokenStream {
        let main_ty = self.gen_data.name;
        let cases_count = self.data.variants.len();

        let mut output = quote! {};

        for (index, field) in self.data.variants.iter().enumerate() {
            let field_name = &field.ident;
            let field_ty = &field.fields.iter().next().map(|item| &item.ty);

            if let Some(_) = field_ty {
                output.extend(quote! { Self::#field_name(value) => {
                    writer.write_variant::<#main_ty, _>(#cases_count, #index, value)
                } });
            } else {
                output.extend(quote! { Self::#field_name => {
                    writer.write_variant::<#main_ty, _>(#cases_count, #index, ())
                } });
            }
        }

        quote! { match self { #output } }
    }

    fn borrowed_name(&self) -> Ident {
        Ident::new(
            &format!("{}Borrowed", self.gen_data.name),
            self.gen_data.name.span(),
        )
    }

    fn borrowed_def(&self) -> TokenStream {
        let mut output = quote! {};

        for field in &self.data.variants {
            let field_ty = &field.fields.iter().next().map(|item| &item.ty);
            let field_name = &field.ident;
            if let Some(ty) = field_ty {
                output.extend(
                    quote! { #field_name(<#ty as wasmi_component::ComponentValue>::Borrowed<'a>), },
                );
            } else {
                output.extend(quote! { #field_name, });
            }
        }

        let vis = &self.gen_data.vis;
        let borrowed_name = &self.borrowed_name();

        quote! {
            #[derive(Clone, Debug)]
            #vis enum #borrowed_name<'a> { #output }
        }
    }

    fn lift_owned(&self) -> TokenStream {
        let mut output = quote! {};
        let name = &self.gen_data.name;

        for field in &self.data.variants {
            let field_ty = &field.fields.iter().next().map(|item| &item.ty);
            let field_name = &field.ident;
            if let Some(_ty) = field_ty {
                output.extend(
                    quote! { Self::#field_name(value) => #name::#field_name(value.lift_owned()?), },
                );
            } else {
                output.extend(quote! { Self::#field_name => #name::#field_name, });
            }
        }

        quote! { Ok(match self { #output }) }
    }

    fn lift_to(&self) -> TokenStream {
        let mut output = quote! {};
        let name = &self.gen_data.name;

        for field in &self.data.variants {
            let field_ty = &field.fields.iter().next().map(|item| &item.ty);
            let field_name = &field.ident;
            if let Some(_ty) = field_ty {
                output.extend(quote! { Self::#field_name(self_val) => {
                    if let #name::#field_name(target_val) = target {
                        self_val.lift_to(target_val)
                    } else {
                        *target = #name::#field_name(self_val.lift_owned()?);
                        Ok(())
                    }
                } });
            } else {
                output.extend(quote! { Self::#field_name => {
                    *target = #name::#field_name;
                    Ok(())
                } });
            }
        }

        quote! { match self { #output } }
    }
}
