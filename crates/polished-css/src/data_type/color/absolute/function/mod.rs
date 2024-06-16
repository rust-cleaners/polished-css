//! Functional notations to represent colors in a variety of color spaces by
//! specifying their channel coordinates.
//!
//! ### Resources
//!
//! - [CSSWG specification](https://www.w3.org/TR/css-color-4/#color-functions)
//! - [MDN documentation](https://developer.mozilla.org/en-US/docs/Web/CSS/color_value)

pub mod oklch;
pub mod rgb;

pub use oklch::*;
pub use rgb::*;

/// [CSSWG specification](https://drafts.csswg.org/css-color/#typedef-absolute-color-function)
#[derive(
	Clone,
	Debug,
	PartialEq,
	strum_macros::EnumIs,
	polished_css_macros::Display,
	polished_css_macros::DataTypeFromDataTypes,
)]
#[display(on_enum = true)]
// TODO: Implement missing color functions - feel free to contribute.
#[non_exhaustive]
pub enum AbsoluteColorFunction {
	/// `rgb()` and its `rgba()` alias - which _(like the hex color notation)_
	/// specify sRGB colors directly by their red/green/blue/alpha channels.
	Rgb(Rgb),

	// /// `hsl()` and its `hsla()` alias - specifies sRGB colors by hue,
	// /// saturation, and lightness using the HSL cylindrical coordinate
	// /// model.
	// Hsl,
	// /// `hwb()` - specifies an sRGB color by hue, whiteness, and blackness
	// /// using the HWB cylindrical coordinate model.
	// Hwb,
	// /// `lab()` - specifies a CIELAB color by CIE Lightness and its a- and
	// /// b-axis hue coordinates (red/green-ness, and yellow/blue-ness) using
	// /// the CIE LAB rectangular coordinate model.
	// Lab,
	// /// `lch()` - specifies a CIELAB color by CIE Lightness, Chroma, and
	// /// hue using the CIE LCH cylindrical coordinate model
	// Lch,
	// /// `oklab()` - specifies an Oklab color by Oklab Lightness and its a-
	// /// and b-axis hue coordinates (red/green-ness, and yellow/blue-ness) using
	// /// the Oklab rectangular coordinate model.
	// Oklab,
	/// `oklch()` - specifies an Oklab color by Oklab Lightness, Chroma, and Hue
	/// using the Oklch cylindrical coordinate model.
	Oklch(Oklch),
	// /// ``color()` - allows specifying colors in a variety of color spaces
	// /// including sRGB, Linear-light sRGB, Display P3, A98 RGB, ProPhoto RGB,
	// /// ITU-R BT.2020-2, and CIE XYZ.
	// Color,
}

// #[polished_css_macros::create_trait(constructor_arg_type =
// AbsoluteColorFunction, from_enum = true)] impl AbsoluteColorFunction {
//     #[must_use]
//     pub fn oklch(value: Oklch) -> Self {
//         Self::Oklch(value)
//     }
// }

pub trait AbsoluteColorFunctionStorage:
	From<AbsoluteColorFunction> + OklchStorage + RgbStorage
{
}

impl From<Oklch> for AbsoluteColorFunction {
	fn from(value: Oklch) -> Self {
		Self::Oklch(value)
	}
}
impl OklchStorage for AbsoluteColorFunction {}

impl From<Rgb> for AbsoluteColorFunction {
	fn from(value: Rgb) -> Self {
		Self::Rgb(value)
	}
}
impl RgbStorage for AbsoluteColorFunction {}

mod test {
	#[test]
	fn display_oklch() {
		use crate::prelude::*;
		assert_eq!(
			super::AbsoluteColorFunction::oklch(Oklch {
				lightness: Lightness::percentage(50.0),
				chroma: Chroma::number(0.4),
				hue: Hue::deg(225.0),
				alpha: Some(Alpha::visible())
			})
			.to_string(),
			String::from("oklch(50% 0.4 225deg / 1)")
		);
	}

	#[test]
	fn display_rgb() {
		use crate::prelude::*;
		assert_eq!(
			super::AbsoluteColorFunction::rgb(Rgb {
				red: Red::percentage(75.0),
				green: Green::number(150.0),
				blue: Blue::number(225.0),
				alpha: Some(Alpha::invisible())
			})
			.to_string(),
			String::from("rgb(75% 150 225 / 0)")
		);
	}
}
