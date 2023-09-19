pub mod r#impl;

pub use r#impl::*;

use convert_case::{Case, Casing};
use proc_macro2::TokenStream;
use quote::quote;
use syn::{DeriveInput, Ident};

use crate::utils::DATA_TYPE_OPTIONAL_PREFIX;

use super::utils::{
    get_enum_property_value_ident, get_enum_variants_idents_for_data_types,
    get_enum_variants_idents_for_keywords, get_property_options, PropertyOptions,
};

pub fn create_property_value_enum(ast: &DeriveInput) -> TokenStream {
    let options = get_property_options(ast);
    let property_struct_ident = &ast.ident;
    let enum_ident = get_enum_property_value_ident(ast);
    let display_trait = impl_display_trait_for_enum(ast, &options);

    let variants_keywords = get_enum_variants_idents_for_keywords(ast, &options)
        .iter()
        .map(|ident| {
            quote! {
                #ident,
            }
        })
        .collect::<TokenStream>();

    let variants_data_type = get_enum_variants_idents_for_data_types(ast, &options)
        .iter()
        .map(|ident| {
            // NOTE: Ugly workaround, because we can't screw-up with std naming
            let data_type_ident = if ident.to_string().ne("String") {
                ident.clone()
            } else {
                Ident::new(
                    format!("{}{}", DATA_TYPE_OPTIONAL_PREFIX, ident).as_str(),
                    ident.span(),
                )
            };
            quote! {
                #ident(crate::data_type::#data_type_ident),
            }
        })
        .collect::<TokenStream>();

    quote! {
        #[derive(
            Clone,
            Debug,
            PartialEq,
            strum_macros::EnumIs
        )]
        #[strum(serialize_all = "kebab-case")]
        pub enum #enum_ident {
            #variants_keywords
            #variants_data_type
        }

        // Rust std traits
        #display_trait

        // Other
        impl crate::utils::UnitDataType<#property_struct_ident<Self>>
            for #enum_ident
        {}
    }
}

fn impl_display_trait_for_enum(ast: &DeriveInput, options: &PropertyOptions) -> TokenStream {
    let enum_ident = get_enum_property_value_ident(ast);

    let matches_keywords = get_enum_variants_idents_for_keywords(ast, options)
        .iter()
        .map(|ident| {
            let name = ident.to_string().to_case(Case::Kebab);
            quote! {
                Self::#ident => write!(f, "{}", #name),
            }
        })
        .collect::<TokenStream>();

    let matches_data_types = get_enum_variants_idents_for_data_types(ast, options)
        .iter()
        .map(|ident| {
            quote! {
                Self::#ident(value) => write!(f, "{value}"),
            }
        })
        .collect::<TokenStream>();

    quote! {
        impl std::fmt::Display for #enum_ident {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                match self {
                    #matches_data_types
                    #matches_keywords
                }
            }
        }
    }
}
