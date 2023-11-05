use syn::{spanned::Spanned, Ident};

use super::DATA_TYPE_TRAIT_SUFFIX;

// NOTE:
// I could not find an easier way to see if the trait exits inside the proc
// macros. Hence why I created this enum to support it.
#[derive(
	strum_macros::Display, strum_macros::EnumIs, strum_macros::EnumString, strum_macros::AsRefStr,
)]
#[strum(serialize_all = "PascalCase")]
pub(crate) enum ColorFunction {
	Oklch,
}

impl ColorFunction {
	pub(crate) fn get_ident(&self) -> Ident {
		let name = self.to_string();
		Ident::new(&name, Spanned::span(&name))
	}

	pub(crate) fn get_trait_ident(&self) -> Ident {
		let name = format!("{self}{DATA_TYPE_TRAIT_SUFFIX}");
		Ident::new(&name, Spanned::span(&name))
	}
}
