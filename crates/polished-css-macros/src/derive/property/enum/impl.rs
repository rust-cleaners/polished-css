use convert_case::{Case, Casing};
use proc_macro2::TokenStream;
use quote::quote;
use syn::{DeriveInput, Ident};

use crate::utils::has_syntax_reserved_keywords;

use super::{
	get_enum_property_value_ident, get_enum_variants_idents_for_keywords, get_property_options,
};

pub(crate) fn create_property_impl(ast: &DeriveInput) -> TokenStream {
	let options = get_property_options(ast);
	let struct_ident = &ast.ident;
	let enum_value_ident = get_enum_property_value_ident(ast);

	let methods_for_keywords = get_enum_variants_idents_for_keywords(ast, &options)
		.iter()
		.map(|variant| {
			let method_name = variant
				.to_string()
				.to_case(Case::Snake);
			let method_ident = get_method_ident(ast, &method_name);
			quote! {
				#[must_use]
				pub fn #method_ident() -> Self {
					Self(#enum_value_ident::#variant)
				}
			}
		})
		.collect::<TokenStream>();

	quote! {
		impl #struct_ident<#enum_value_ident> {
			#methods_for_keywords
		}
	}
}

fn get_method_ident(ast: &DeriveInput, name: &str) -> Ident {
	if has_syntax_reserved_keywords(name) {
		Ident::new_raw(
			// NOTE: Ugly workaround, because:
			// https://docs.rs/syn/latest/syn/struct.Ident.html#method.new_raw
			if name.eq("super") {
				"keyword_super"
			} else {
				name
			},
			ast.ident.span(),
		)
	} else {
		Ident::new(name, ast.ident.span())
	}
}
