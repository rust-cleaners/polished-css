use proc_macro2::TokenStream;
use quote::quote;
use syn::DeriveInput;

use crate::{
	derive::property::{
		get_enum_property_value_ident, get_enum_variant_ident_for_data_type, get_property_options,
		PropertyOptions,
	},
	utils::DataType,
};

pub fn impl_from_data_type(ast: &DeriveInput) -> TokenStream {
	let PropertyOptions { data_type, .. } = get_property_options(ast);

	if !data_type.is_empty() {
		let struct_ident = &ast.ident;
		let generics = &ast.generics;
		let (impl_generics, type_generics, where_clause) = generics.split_for_impl();
		let impl_data_types_values_traits = impl_data_types_values_traits(ast);

		quote! {
			// Generic
			impl #impl_generics From #type_generics
				for #struct_ident #type_generics
			#where_clause
			{
				fn from(value: T) -> Self {
					Self(value)
				}
			}

			// NOTE: In the meantime,
			// implement a trait for this data type, when the property depends on it
			#impl_data_types_values_traits

		}
	} else {
		TokenStream::default()
	}
}

fn impl_data_types_values_traits(ast: &DeriveInput) -> TokenStream {
	let PropertyOptions { data_type, .. } = get_property_options(ast);

	let data_type = (!data_type.is_empty()).then(|| DataType::get_from_name(&data_type));
	if let Some(data_type) = data_type {
		let data_type_ident = data_type.get_ident();
		let enum_variant_data_type_ident = get_enum_variant_ident_for_data_type(ast, &data_type);
		let trait_ident = data_type.get_trait_ident();

		let struct_ident = &ast.ident;
		let enum_value_ident = get_enum_property_value_ident(ast);

		let data_types = get_data_types_dependencies(ast, &data_type);
		let color_functions = get_color_functions_dependencies(ast, &data_type);
		let units = get_units_dependencies(ast, &data_type);

		quote! {
			impl<T> From<T>
				for #struct_ident<#enum_value_ident>
			where
				T: Into<crate::data_type::#data_type_ident>
			{
				fn from(value: T) -> Self {
					Self(#enum_value_ident::#enum_variant_data_type_ident(value.into()))
				}
			}

			impl crate::data_type::#trait_ident
				for #struct_ident<#enum_value_ident>
			{}
			#data_types
			#color_functions
			#units
		}
	} else {
		TokenStream::default()
	}
}

fn get_data_types_dependencies(ast: &DeriveInput, data_type: &DataType) -> TokenStream {
	let struct_ident = &ast.ident;
	let enum_value_ident = get_enum_property_value_ident(ast);

	data_type
		.get_dependant_data_types()
		.iter()
		.map(|data_type| {
			let trait_ident = data_type.get_trait_ident();
			let data_types = get_data_types_dependencies(ast, data_type);
			let color_functions = get_color_functions_dependencies(ast, data_type);
			let units = get_units_dependencies(ast, data_type);
			quote! {
				impl crate::data_type::#trait_ident for #struct_ident<#enum_value_ident> {}
				#data_types
				#color_functions
				#units
			}
		})
		.collect()
}

fn get_units_dependencies(ast: &DeriveInput, data_type: &DataType) -> TokenStream {
	let struct_ident = &ast.ident;
	let enum_value_ident = get_enum_property_value_ident(ast);

	data_type
		.get_dependant_units()
		.iter()
		.map(|unit| {
			let trait_ident = unit.get_trait_ident();
			quote!(impl crate::data_type::#trait_ident for #struct_ident<#enum_value_ident> {})
		})
		.collect()
}

fn get_color_functions_dependencies(ast: &DeriveInput, data_type: &DataType) -> TokenStream {
	let struct_ident = &ast.ident;
	let enum_value_ident = get_enum_property_value_ident(ast);

	data_type
		.get_dependant_color_functions()
		.iter()
		.map(|color_function| {
			let trait_ident = color_function.get_trait_ident();
			quote!(impl crate::data_type::#trait_ident for #struct_ident<#enum_value_ident> {})
		})
		.collect()
}
