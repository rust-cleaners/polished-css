use convert_case::{Case, Casing};
use darling::FromDeriveInput;
use proc_macro2::TokenStream;
use quote::quote;
use syn::{parse_quote, DataEnum, DeriveInput, Expr, ExprLit, Fields, Lit};

use crate::derive::get_data_enum;

#[derive(Default, darling::FromDeriveInput)]
#[darling(default, attributes(display))]
struct DisplayOptions {
	custom: Option<String>,
	on_enum: bool,
	prefix: String,
	suffix: String,
}

pub fn impl_display(ast: &DeriveInput) -> TokenStream {
	let DisplayOptions {
		custom,
		on_enum,
		prefix,
		suffix,
		..
	} = DisplayOptions::from_derive_input(ast)
		.expect("Failed to parse Display proc macro derive attributes");
	if on_enum {
		let enum_ident = &ast.ident;
		let enum_data = get_data_enum(ast);
		let variants_lines_to_display = get_enum_variants_lines_to_display(enum_data);

		quote! {
			impl std::fmt::Display for #enum_ident {
				fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
					write!(
						f,
						"{}",
						match self {
							#variants_lines_to_display
						}
					)
				}
			}
		}
	} else {
		let struct_ident = &ast.ident;
		let generics = &ast.generics;
		let (impl_generics, type_generics, where_clause) = generics.split_for_impl();
		let value: Expr = custom.map_or_else(
			|| syn::parse_str("self.0").expect("Failed to create display value."),
			|custom| {
				Expr::Lit(ExprLit {
					attrs: Vec::default(),
					lit: Lit::Str(parse_quote! { #custom }),
				})
			},
		);

		quote! {
			impl #impl_generics std::fmt::Display for #struct_ident #type_generics #where_clause {
				fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
					write!(f, "{}{}{}", #prefix, #value, #suffix)
				}
			}
		}
	}
}

fn get_enum_variants_lines_to_display(data_enum: &DataEnum) -> TokenStream {
	data_enum
		.variants
		.iter()
		.map(|variant| {
			let ident = &variant.ident;
			match &variant.fields {
				// NOTE: The commas needs to be there!
				Fields::Named(_) | Fields::Unnamed(_) => quote!(Self::#ident(v) => v.to_string(),),
				Fields::Unit => {
					let name = ident
						.to_string()
						.to_case(Case::Snake);
					quote!(Self::#ident => String::from(#name),)
				}
			}
		})
		.collect()
}
