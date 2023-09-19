pub(crate) mod from_data_type;
pub(crate) mod name;

pub use from_data_type::*;
pub use name::*;

use proc_macro2::TokenStream;
use quote::quote;
use syn::DeriveInput;

pub fn impl_property(ast: &DeriveInput) -> TokenStream {
    let struct_ident = &ast.ident;
    let generics = &ast.generics;
    let (impl_generics, type_generics, where_clause) = generics.split_for_impl();

    quote! {
        impl #impl_generics crate::property::Property
            for #struct_ident #type_generics
        #where_clause
        {}

        // Crate's utilities
        impl crate::utils::UnitDataType<#struct_ident<Self>>
            for crate::utils::Nothing
        {}

        // CSS-wide functions traits
        impl<T> crate::utils::UnitDataType<#struct_ident<Self>>
            for crate::function::Var<T>
        where
            T: Clone
                + std::fmt::Debug
                + std::fmt::Display
                + PartialEq
                + crate::utils::UnitDataType<#struct_ident<T>>
        {}
    }
}
