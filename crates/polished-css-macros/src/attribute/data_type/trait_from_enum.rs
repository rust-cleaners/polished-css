use convert_case::{Case, Casing};
use proc_macro2::TokenStream;
use quote::quote;
use syn::{
	punctuated::Punctuated, token::Comma, FnArg, Ident, ImplItem, ImplItemFn, ItemImpl, Pat, Type,
};

use crate::{attribute::get_data_type_from_item_impl, utils::DataType};

pub(crate) fn create_trait_from_enum_impl(_args: &TokenStream, ast: &ItemImpl) -> TokenStream {
	let data_type = get_data_type_from_item_impl(ast);
	let data_type_ident = data_type.get_ident();
	let data_type_methods_from_impl = get_data_type_methods_from_impl(ast);
	let trait_ident = data_type.get_trait_ident();
	let constructor_ident = get_constructor_ident(&data_type);

	quote! {
		pub trait #trait_ident: From<#data_type_ident> {
			#[must_use]
			fn #constructor_ident(value: #data_type_ident) -> Self
			where
				Self: Sized,
			{
				Self::from(value)
			}

			#data_type_methods_from_impl
		}
	}
}

pub(crate) fn get_constructor_ident(data_type: &DataType) -> Ident {
	let data_type_ident = data_type.get_ident();
	Ident::new(
		data_type_ident
			.to_string()
			.to_case(Case::Snake)
			.as_str(),
		data_type_ident.span(),
	)
}

fn get_data_type_methods_from_impl(ast: &ItemImpl) -> TokenStream {
	ast.items
		.iter()
		.filter_map(|item| {
			if let ImplItem::Fn(method) = item {
				Some(create_method(ast, method))
			} else {
				None
			}
		})
		.collect()
}

fn create_method(ast: &ItemImpl, method: &ImplItemFn) -> TokenStream {
	let data_type = get_data_type_from_item_impl(ast);
	let data_type_ident = data_type.get_ident();
	let constructor_data_type_ident = get_constructor_ident(&data_type);

	let method_inputs = &method.sig.inputs;
	let method_ident = &method.sig.ident;
	let method_args: TokenStream = get_method_args(method_inputs)
		.iter()
		.map(|(arg_ident, arg_type)| quote!(#arg_ident: #arg_type))
		.collect();
	let method_values: TokenStream = get_input_idents(method_inputs)
		.iter()
		.map(|i| quote!(#i))
		.collect();

	quote! {
		#[must_use]
		fn #method_ident(#method_args) -> Self
		where
			Self: Sized
		{
			Self::#constructor_data_type_ident(#data_type_ident::#method_ident(#method_values))
		}
	}
}

fn get_input_idents(inputs: &Punctuated<FnArg, Comma>) -> Vec<Ident> {
	inputs
		.iter()
		.filter_map(|fn_arg| {
			if let FnArg::Typed(pat_type) = fn_arg {
				if let Pat::Ident(ident) = &*pat_type.pat {
					Some(ident.ident.clone())
				} else {
					None
				}
			} else {
				None
			}
		})
		.collect()
}

fn get_method_args(inputs: &Punctuated<FnArg, Comma>) -> Vec<(Ident, Type)> {
	inputs
		.iter()
		.filter_map(|fn_arg| {
			if let FnArg::Typed(pat_type) = fn_arg {
				if let Pat::Ident(ident) = &*pat_type.pat {
					Some((ident.ident.clone(), *pat_type.ty.clone()))
				} else {
					None
				}
			} else {
				None
			}
		})
		.collect()
}
