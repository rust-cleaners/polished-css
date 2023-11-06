pub mod absolute;
pub mod system;

pub use absolute::*;
pub use system::*;

/// [MDN](https://developer.mozilla.org/en-US/docs/Web/CSS/color)
#[derive(
	Clone,
	Debug,
	PartialEq,
	strum_macros::EnumIs,
	polished_css_macros::Display,
	polished_css_macros::DataTypeFromDataTypes,
)]
#[display(on_enum = true)]
pub enum Color {
	Absolute(AbsoluteColor),
	Currentcolor,
	System(SystemColor),
}

#[polished_css_macros::create_trait_from_enum_impl()]
impl Color {
	#[must_use]
	pub const fn current_color() -> Self {
		Self::Currentcolor
	}

	// TODO: Add conversion methods
}

#[cfg(test)]
mod test {
	use crate::prelude::*;

	#[test]
	fn display() {
		assert_eq!(
			super::Color::current_color().to_string(),
			String::from("currentcolor")
		);

		assert_eq!(
			super::Color::oklch(Oklch {
				lightness: Lightness::percentage(50.0),
				chroma: Chroma::number(0.4),
				hue: Hue::deg(225.0),
				alpha: Some(Alpha::visible())
			})
			.to_string(),
			String::from("oklch(50% 0.4 225deg / 1)")
		);

		assert_eq!(
			super::Color::transparent().to_string(),
			String::from("transparent")
		);
	}
}
