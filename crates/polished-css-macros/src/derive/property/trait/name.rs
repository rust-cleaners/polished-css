use convert_case::{Case, Casing};
use proc_macro2::TokenStream;
use quote::quote;
use syn::DeriveInput;

use super::super::utils::{get_property_options, PropertyOptions};

pub fn impl_property_name(ast: &DeriveInput) -> TokenStream {
    let PropertyOptions {
        display, custom, ..
    } = get_property_options(ast);
    let struct_ident = &ast.ident;
    let generics = &ast.generics;
    let (impl_generics, type_generics, where_clause) = generics.split_for_impl();

    let mut name: String = if display.is_empty() {
        ast.ident.to_string()
    } else {
        display
    }
    .to_case(Case::Kebab)
    .to_lowercase();

    name = if custom {
        format!("--{name}")
    } else {
        name
    };

    quote! {
        impl #impl_generics crate::property::PropertyName
            for #struct_ident #type_generics
        #where_clause
        {
            fn property_name<'a>(&self) -> &'a str {
                #name
            }
        }
    }
}
