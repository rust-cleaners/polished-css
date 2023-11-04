use darling::FromDeriveInput;
use syn::{DeriveInput, Ident};

#[derive(darling::FromDeriveInput)]
#[darling(attributes(unit))]
pub struct UnitOptions {
	pub trait_ident: Ident,
}

pub fn get_unit_options(ast: &DeriveInput) -> UnitOptions {
	UnitOptions::from_derive_input(ast)
		.expect("Couldn't parse correctly the proc macro derive attributes for unit.")
}
