use proc_macro2::TokenStream;
use quote::quote;
use syn::DeriveInput;

use crate::{
    derive::get_data_enum,
    utils::{get_enum_variant_associated_value_type_ident, get_enum_variants, Unit},
};

pub fn impl_from_units_traits(ast: &DeriveInput) -> TokenStream {
    let enum_data = get_data_enum(ast);
    get_enum_variants(enum_data)
        .iter()
        .map(|variant| {
            let enum_ident = &ast.ident;
            let unit = Unit::get_from_ident(
                get_enum_variant_associated_value_type_ident(variant)
                    .expect("Failed to get unit's ident"),
            );
            let enum_variant_ident = unit.get_enum_variant_ident();
            let unit_ident = unit.get_ident();
            let trait_ident = unit.get_trait_ident();

            quote! {
                impl From<crate::data_type::#unit_ident> for #enum_ident {
                    fn from(value: crate::data_type::#unit_ident) -> Self {
                        Self::#enum_variant_ident(value)
                    }
                }
                impl crate::data_type::#trait_ident for #enum_ident {}
            }
        })
        .collect()
}
