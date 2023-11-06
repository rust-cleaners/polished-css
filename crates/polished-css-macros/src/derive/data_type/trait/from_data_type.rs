use proc_macro2::TokenStream;
use quote::quote;
use syn::{DeriveInput, Ident};

use crate::utils::DataType;

pub fn impl_from_data_types_traits(ast: &DeriveInput) -> TokenStream {
	let enum_ident = &ast.ident;
	let enum_data_type = DataType::get_from_ident(enum_ident);

	enum_data_type
		.get_dependant_data_types()
		.iter()
		.map(|data_type| {
			let data_type_ident = data_type.get_ident();
			let data_type_trait_ident = data_type.get_trait_ident();
			let enum_variant_ident = data_type.get_enum_variant_ident();

			let data_types = impl_dependent_data_types(ast, data_type, &enum_variant_ident);
			let color_functions =
				impl_dependent_color_functions(ast, data_type, &enum_variant_ident);
			let units = impl_dependent_units(ast, data_type, &enum_variant_ident);

			quote! {
				impl From<crate::data_type::#data_type_ident> for #enum_ident {
					fn from(value: crate::data_type::#data_type_ident) -> Self {
						Self::#enum_variant_ident(value)
					}
				}
				impl crate::data_type::#data_type_trait_ident for #enum_ident {}
				#data_types
				#color_functions
				#units
			}
		})
		.collect()
}

fn impl_dependent_data_types(
	ast: &DeriveInput,
	data_type: &DataType,
	enum_variant_ident: &Ident,
) -> TokenStream {
	let enum_ident = &ast.ident;

	let mut stream = TokenStream::new();
	let mut dependencies = data_type.get_dependant_data_types();

	while !dependencies.is_empty() {
		for data_type in dependencies {
			let data_type_ident = data_type.get_ident();
			let data_type_trait_ident = data_type.get_trait_ident();
			let color_functions =
				impl_dependent_color_functions(ast, data_type, enum_variant_ident);
			let units = impl_dependent_units(ast, data_type, enum_variant_ident);

			stream.extend(quote! {
				impl From<crate::data_type::#data_type_ident> for #enum_ident {
					fn from(value: crate::data_type::#data_type_ident) -> Self {
						Self::#enum_variant_ident(value.into())
					}
				}
				impl crate::data_type::#data_type_trait_ident for #enum_ident {}
				#color_functions
				#units
			});

			dependencies = data_type.get_dependant_data_types();
		}
	}

	stream
}

fn impl_dependent_color_functions(
	ast: &DeriveInput,
	data_type: &DataType,
	enum_variant_ident: &Ident,
) -> TokenStream {
	let enum_ident = &ast.ident;

	data_type
		.get_dependant_color_functions()
		.iter()
		.map(|color_function| {
			let color_function_ident = color_function.get_ident();
			let color_function_trait_ident = color_function.get_trait_ident();

			quote! {
				impl From<crate::data_type::#color_function_ident> for #enum_ident {
					fn from(value: crate::data_type::#color_function_ident) -> Self {
						Self::#enum_variant_ident(value.into())
					}
				}
				impl crate::data_type::#color_function_trait_ident for #enum_ident {}
			}
		})
		.collect()
}

fn impl_dependent_units(
	ast: &DeriveInput,
	data_type: &DataType,
	enum_variant_ident: &Ident,
) -> TokenStream {
	let enum_ident = &ast.ident;

	data_type
		.get_dependant_units()
		.iter()
		.map(|unit| {
			let unit_ident = unit.get_ident();
			let unit_trait_ident = unit.get_trait_ident();

			quote! {
				impl From<crate::data_type::#unit_ident> for #enum_ident {
					fn from(value: crate::data_type::#unit_ident) -> Self {
						Self::#enum_variant_ident(value.into())
					}
				}
				impl crate::data_type::#unit_trait_ident for #enum_ident {}
			}
		})
		.collect()
}
