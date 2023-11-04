use proc_macro2::TokenStream;
use quote::quote;
use syn::DeriveInput;

use crate::utils::DataType;

pub fn impl_data_type_traits(ast: &DeriveInput) -> TokenStream {
	let enum_ident = &ast.ident;
	let data_type = DataType::get_from_ident(enum_ident);

	data_type
		.get_dependant_data_types()
		.iter()
		.map(|data_type| {
			let trait_ident = data_type.get_trait_ident();
			let units_stream: TokenStream = data_type
				.get_dependant_units()
				.iter()
				.map(|unit| {
					let trait_ident = unit.get_trait_ident();
					quote!(impl crate::data_type::#trait_ident for #enum_ident {})
				})
				.collect();

			quote! {
				impl crate::data_type::#trait_ident for #enum_ident {}
				#units_stream
			}
		})
		.collect()
}
