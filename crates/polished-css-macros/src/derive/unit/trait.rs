use convert_case::{Case, Casing};
use proc_macro2::TokenStream;
use quote::quote;
use syn::{DeriveInput, Ident};

use crate::{
	derive::get_tuple_struct_field_type,
	utils::{has_syntax_reserved_keywords, DATA_TYPE_TRAIT_SUFFIX},
};

use super::{get_unit_options, UnitOptions};

pub(crate) fn create_trait(ast: &DeriveInput) -> TokenStream {
	let UnitOptions { trait_ident, .. } = get_unit_options(ast);
	let struct_ident = &ast.ident;
	let trait_ident = Ident::new(
		format!("{trait_ident}{DATA_TYPE_TRAIT_SUFFIX}").as_str(),
		struct_ident.span(),
	);
	let value_type_ident = get_tuple_struct_field_type(ast);
	let method_name = struct_ident
		.to_string()
		.to_case(Case::Snake);
	let method_ident = if has_syntax_reserved_keywords(&method_name) {
		Ident::new_raw(&method_name, struct_ident.span())
	} else {
		Ident::new(&method_name, struct_ident.span())
	};

	let doc = format!(
		r#"
        Use a CSS unit `"{method_name}"` _(`{struct_ident}`)_ as value to store.

        ### Resources

        - [CSSWG specification](https://drafts.csswg.org/css-values/#{method_name})
        "#,
	);

	quote! {
		#[doc = #doc]
		pub trait #trait_ident: From<#struct_ident> {
			#[doc = #doc]
			fn #method_ident(value: #value_type_ident) -> Self
			where
				Self: Sized
			{
				Self::from(#struct_ident(value))
			}
		}
	}
}
