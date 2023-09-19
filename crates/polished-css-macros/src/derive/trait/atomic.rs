use darling::FromDeriveInput;
use proc_macro2::TokenStream;
use quote::quote;
use syn::DeriveInput;

#[derive(Default, darling::FromDeriveInput)]
#[darling(default, attributes(atomic), supports(struct_tuple))]
pub struct AtomicOptions {
    pub name: String,
}

pub fn impl_atomic(ast: &DeriveInput) -> TokenStream {
    let struct_ident = &ast.ident;
    let AtomicOptions { name, .. } =
        AtomicOptions::from_derive_input(ast).expect("Could not parse atomic options.");

    quote! {
        #[cfg(feature = "atomic")]
        impl<T> crate::atomic::Atomic for #struct_ident<T>
        where
            T: Clone + std::fmt::Debug + std::fmt::Display + PartialEq + crate::utils::UnitDataType<Self>
        {
            fn atomic_name<'a>(&self) -> &'a str {
                #name
            }

            fn style(&self) -> crate::style::Style {
                crate::style::Style::builder()
                    .selector(crate::selector::Class(format!(
                        "{}[{}]",
                        &self.atomic_name(),
                        &self.to_string().replace(" ", "_")
                    )))
                    .declarations(&[self])
                    .build()
            }
        }

        #[cfg(feature = "yew")]
        impl<T> From<#struct_ident<T>> for yew::Classes
        where
            T: Clone + std::fmt::Debug + std::fmt::Display + PartialEq + crate::utils::UnitDataType<#struct_ident<T>> {
            fn from(value: #struct_ident<T>) -> Self{
                use crate::atomic::Atomic;
                Self::from(value.atomic())
            }
        }
    }
}
