use super::{Frequency, Length, Resolution, Time};

/// [MDN documentation](https://developer.mozilla.org/en-US/docs/Web/CSS/dimension)
#[derive(
	Clone,
	Debug,
	PartialEq,
	strum_macros::EnumIs,
	polished_css_macros::Display,
	polished_css_macros::DataTypeFromDataTypes,
)]
#[display(on_enum = true)]
pub enum Dimension {
	Frequency(Frequency),
	Length(Length),
	Resolution(Resolution),
	Time(Time),
}

#[polished_css_macros::create_trait_from_enum_impl()]
impl Dimension {
	// TODO: Add conversion methods?
}

#[cfg(test)]
mod test {
	use crate::prelude::*;

	#[test]
	fn display() {
		assert_eq!(super::Dimension::hz(1.0).to_string(), String::from("1Hz"));

		assert_eq!(super::Dimension::px(1).to_string(), String::from("1px"));
		assert_eq!(super::Dimension::cqh(1.0).to_string(), String::from("1cqh"));
		assert_eq!(super::Dimension::em(1.0).to_string(), String::from("1em"));
		assert_eq!(super::Dimension::vh(1.0).to_string(), String::from("1vh"));

		assert_eq!(super::Dimension::dpi(1.0).to_string(), String::from("1dpi"));

		assert_eq!(super::Dimension::ms(250).to_string(), String::from("250ms"));
	}
}
