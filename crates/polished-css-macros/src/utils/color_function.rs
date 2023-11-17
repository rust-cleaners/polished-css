use quote::quote;
use syn::{spanned::Spanned, Ident, ImplGenerics, TypeGenerics, WhereClause};

use super::DATA_TYPE_TRAIT_SUFFIX;

// NOTE:
// I could not find an easier way to see if the trait exits inside the proc
// macros. Hence why I created this enum to support it.
#[derive(
	strum_macros::Display, strum_macros::EnumIs, strum_macros::EnumString, strum_macros::AsRefStr,
)]
#[strum(serialize_all = "PascalCase")]
pub enum ColorFunction {
	Oklch,
}

impl ColorFunction {
	pub fn get_ident(&self) -> Ident {
		let name = self.to_string();
		Ident::new(&name, Spanned::span(&name))
	}

	pub fn get_trait_ident(&self) -> Ident {
		let name = format!("{self}{DATA_TYPE_TRAIT_SUFFIX}");
		Ident::new(&name, Spanned::span(&name))
	}

	pub fn get_generics<'a>(&self) -> (ImplGenerics<'a>, TypeGenerics<'a>, WhereClause) {
		match self {
			ColorFunction::Oklch => {
				// FIXME: See if anything I can do there - convert TokenStream to generics
				let impl_generics = ImplGenerics::from(quote!(<L,C,H,A>));
				let type_generics = TypeGenerics::from(quote!(<L,C,H,A>));
				let where_clause = WhereClause::from(quote! {
					where
						L: Clone + std::fmt::Debug + std::fmt::Display + PartialEq + crate::utils::UnitDataType<Self>,
						C: Clone + std::fmt::Debug + std::fmt::Display + PartialEq + crate::utils::UnitDataType<Self>,
						H: Clone + std::fmt::Debug + std::fmt::Display + PartialEq + crate::utils::UnitDataType<Self>,
						A: Clone + std::fmt::Debug + std::fmt::Display + PartialEq + crate::utils::UnitDataType<Self>,
				});
				(impl_generics, type_generics, where_clause)
			}
		}
	}
}
