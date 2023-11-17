use proc_macro2::Span;
use syn::{
	spanned::Spanned, GenericParam, Generics, Ident, ImplGenerics, TypeGenerics, WhereClause,
};

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
	pub fn struct_ident(&self) -> Ident {
		let name = self.to_string();
		Ident::new(&name, Spanned::span(&name))
	}

	pub fn trait_ident(&self) -> Ident {
		let name = format!("{self}{DATA_TYPE_TRAIT_SUFFIX}");
		Ident::new(&name, Spanned::span(&name))
	}

	pub fn generics(&self) -> Generics {
		match self {
			Self::Oklch => {
				let mut generics = Generics::default();

				for name in &["L", "C", "H", "A"] {
					generics
						.params
						.push(GenericParam::Type(
							Ident::new(name, Span::call_site()).into(),
						));
				}

				// FIXME: Need to research on creating a where clause with trait bounds
				// let where_clause = WhereClause::from(quote! {
				// 	where
				// 		L: Clone + std::fmt::Debug + std::fmt::Display + PartialEq +
				// crate::utils::UnitDataType<Self>, 		C: Clone + std::fmt::Debug +
				// std::fmt::Display + PartialEq + crate::utils::UnitDataType<Self>, 		H: Clone +
				// std::fmt::Debug + std::fmt::Display + PartialEq +
				// crate::utils::UnitDataType<Self>, 		A: Clone + std::fmt::Debug +
				// std::fmt::Display + PartialEq + crate::utils::UnitDataType<Self>, });
				generics
			}
		}
	}
}
