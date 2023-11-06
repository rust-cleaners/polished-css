pub mod function;
pub mod hex;
pub mod named;

pub use function::*;
pub use hex::*;
pub use named::*;

/// [CSSWG specification](https://drafts.csswg.org/css-color/#typedef-absolute-color-base)
#[derive(
	Clone,
	Debug,
	PartialEq,
	strum_macros::EnumIs,
	polished_css_macros::Display,
	polished_css_macros::DataTypeFromDataTypes,
)]
#[display(on_enum = true)]
pub enum AbsoluteColor {
	#[non_exhaustive]
	Hex(HexColor),
	Function(AbsoluteColorFunction),
	Named(NamedColor),
	Transparent,
}

#[polished_css_macros::create_trait_from_enum_impl()]
impl AbsoluteColor {
	#[must_use]
	pub const fn transparent() -> Self {
		Self::Transparent
	}
}

#[cfg(test)]
mod test {
	use crate::prelude::*;

	#[test]
	fn display() {
		assert_eq!(
			super::AbsoluteColor::oklch(Oklch {
				lightness: Lightness::percentage(50.0),
				chroma: Chroma::number(0.4),
				hue: Hue::deg(225.0),
				alpha: Some(Alpha::visible())
			})
			.to_string(),
			String::from("oklch(50% 0.4 225deg / 1)")
		);

		assert_eq!(
			super::AbsoluteColor::Transparent.to_string(),
			String::from("transparent")
		);
	}
}
