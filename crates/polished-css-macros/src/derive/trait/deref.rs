use proc_macro2::TokenStream;
use quote::quote;
use syn::DeriveInput;

use crate::derive::get_tuple_struct_field_type;

pub(crate) fn impl_deref(ast: &DeriveInput) -> TokenStream {
	let struct_ident = &ast.ident;
	let generics = &ast.generics;
	let (impl_generics, type_generics, where_clause) = generics.split_for_impl();
	let single_tuple_type = get_tuple_struct_field_type(ast);

	quote! {
		impl #impl_generics std::ops::Deref for #struct_ident #type_generics #where_clause {
			type Target = #single_tuple_type;

			fn deref(&self) -> &Self::Target {
				&self.0
			}
		}
	}
}
