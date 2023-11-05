use convert_case::{Case, Casing};
use proc_macro2::TokenStream;
use quote::quote;
use syn::{DeriveInput, Ident};

use crate::{
	derive::get_data_enum,
	utils::{get_enum_variants, DATA_TYPE_TRAIT_SUFFIX},
};

pub(crate) fn create_trait_from_enum(ast: &DeriveInput) -> TokenStream {
	let enum_ident = &ast.ident;
	let trait_ident = Ident::new(
		&format!("{}{}", enum_ident, DATA_TYPE_TRAIT_SUFFIX),
		enum_ident.span(),
	);
	let methods = get_methods_stream(ast);

	quote! {
		pub trait #trait_ident: From<#enum_ident> {
			#methods
		}
	}
}

fn get_methods_stream(ast: &DeriveInput) -> TokenStream {
	let enum_ident = &ast.ident;
	let data_enum = get_data_enum(ast);
	get_enum_variants(data_enum)
		.iter()
		.map(|variant| {
			let variant_ident = &variant.ident;
			let method_ident = Ident::new(
				&variant
					.ident
					.to_string()
					.to_case(Case::Snake),
				variant.ident.span(),
			);
			quote! {
				fn #method_ident() -> Self
				where
					Self: Sized
				{
					Self::from(#enum_ident::#variant_ident)
				}
			}
		})
		.collect()
}
