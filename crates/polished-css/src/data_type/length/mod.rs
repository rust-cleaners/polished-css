//! [CSSWG specification](https://www.w3.org/TR/css-values-4/#lengths)

pub mod absolute;
pub mod relative;

pub use absolute::*;
pub use relative::*;

/// [MDN documentation](https://developer.mozilla.org/en-US/docs/Web/CSS/length)
#[derive(
	Clone,
	Debug,
	PartialEq,
	strum_macros::EnumIs,
	polished_css_macros::Display,
	polished_css_macros::DataTypeFromDataTypes,
)]
#[display(on_enum = true)]
pub enum Length {
	Absolute(AbsoluteLength),
	Relative(RelativeLength),
}

#[polished_css_macros::create_trait_from_enum_impl()]
impl Length {
	// TODO: Add conversion methods?
}

#[cfg(test)]
mod test {
	#[test]
	fn display() {
		use crate::data_type::length::*;

		assert_eq!(super::Length::px(1).to_string(), String::from("1px"));
		assert_eq!(super::Length::cqw(1.0).to_string(), String::from("1cqw"));
		assert_eq!(super::Length::lh(1.0).to_string(), String::from("1lh"));
		assert_eq!(super::Length::vw(1.0).to_string(), String::from("1vw"));
	}
}
