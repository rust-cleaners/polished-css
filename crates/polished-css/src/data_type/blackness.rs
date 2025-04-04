//! Blackness of the color.

use crate::data_type::{Number, NumberStorage, Percentage};

/// Specifies the amount of black to mix in, from `0%` _(no blackness)_ to
/// `100%` _(full blackness)_.
#[derive(
	Clone,
	Debug,
	PartialEq,
	strum_macros::EnumIs,
	polished_css_macros::Display,
	polished_css_macros::DataTypeFromDataTypes,
)]
#[display(on_enum = true)]
pub enum Blackness {
	// TODO: Add bounds from 0.0 to 1!
	Number(Number),
	// TODO: Add bounds from 0% to 100%!
	Percentage(Percentage),
	/// In certain cases, a color can have one or more **missing color
	/// components**.
	/// [See more in the CSSWG specification](https://drafts.csswg.org/css-color/#missing)
	None,
}

impl From<f64> for Blackness {
	fn from(value: f64) -> Self {
		Self::number(value)
	}
}

#[polished_css_macros::create_trait_from_enum_impl()]
impl Blackness {
	// TODO: Add conversion methods?
}

#[cfg(test)]
mod test {
	#[test]
	fn display() {
		use crate::data_type::*;
		assert_eq!(
			super::Blackness::percentage(1.23).to_string(),
			String::from("1.23%")
		);
		assert_eq!(
			super::Blackness::number(13.37).to_string(),
			String::from("13.37")
		);
		assert_eq!(super::Blackness::None.to_string(), String::from("none"));
	}
}
