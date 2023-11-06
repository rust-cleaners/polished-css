use darling::FromDeriveInput;
use proc_macro2::TokenStream;
use quote::quote;
use syn::{DeriveInput, Ident};

#[derive(darling::FromDeriveInput)]
#[darling(attributes(default), supports(struct_tuple))]
pub struct DefaultOptions {
	pub value: Ident,
}

pub fn impl_default(ast: &DeriveInput) -> TokenStream {
	let DefaultOptions { value, .. } = DefaultOptions::from_derive_input(ast)
		.expect("Failed to parse Default proc macro derive attributes");
	let struct_ident = &ast.ident;
	let enum_value_ident = Ident::new(format!("{struct_ident}Value").as_str(), struct_ident.span());

	quote! {
		impl Default for #struct_ident<#enum_value_ident> {
			fn default() -> Self {
				Self(#enum_value_ident::#value)
			}
		}
	}
}
