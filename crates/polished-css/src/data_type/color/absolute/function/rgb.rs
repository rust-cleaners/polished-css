//! Related to the [RGB](https://en.wikipedia.org/wiki/RGB_color_model) color model.

use std::fmt;

use crate::data_type::{Alpha, Blue, Green, Red};

/// `rgb()` and its `rgba()` alias - which _(like the hex color notation)_
/// specify sRGB colors directly by their red/green/blue/alpha channels.
///
/// ### Resources
///
/// - [MDN documentation](https://developer.mozilla.org/en-US/docs/Web/CSS/rgb)
/// - [CSSWG specification](https://www.w3.org/TR/css-color-3/#rgb-color)
#[derive(Clone, Debug, PartialEq)]
pub struct Rgb {
	/// Red channel
	pub red: Red,
	/// Green channel
	pub green: Green,
	/// Blue channel
	pub blue: Blue,
	/// Optional alpha channel - by default is `1`
	pub alpha: Option<Alpha>,
}

impl fmt::Display for Rgb {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(
			f,
			"rgb({} {} {} / {})",
			self.red,
			self.green,
			self.blue,
			self.alpha
				.clone()
				.map_or_else(Alpha::visible, |alpha| alpha)
		)
	}
}

impl<R, G, B, A> From<(R, G, B, A)> for Rgb
where
	R: Into<Red>,
	G: Into<Green>,
	B: Into<Blue>,
	A: Into<Option<Alpha>>,
{
	fn from(value: (R, G, B, A)) -> Self {
		let (r, g, b, a) = value;
		Self {
			red: r.into(),
			green: g.into(),
			blue: b.into(),
			alpha: a.into(),
		}
	}
}

impl<R, G, B> From<(R, G, B)> for Rgb
where
	R: Into<Red>,
	G: Into<Green>,
	B: Into<Blue>,
{
	fn from(value: (R, G, B)) -> Self {
		let (r, g, b) = value;
		Self {
			red: r.into(),
			green: g.into(),
			blue: b.into(),
			alpha: Some(Alpha::default()),
		}
	}
}

/// Use absolute color function `[Rgb]` to store the CSS value.
pub trait RgbStorage: From<Rgb> {
	#[must_use]
	fn rgb(value: Rgb) -> Self
	where
		Self: Sized,
	{
		Self::from(value)
	}
}

#[cfg(test)]
mod test {
	#[test]
	fn display() {
		use crate::data_type::*;

		assert_eq!(
			super::Rgb {
				red: Red::number(255.0),
				green: Green::number(255.0),
				blue: Blue::number(255.0),
				alpha: Some(Alpha::visible())
			}
			.to_string(),
			String::from("rgb(255 255 255 / 1)")
		);

		assert_eq!(
			super::Rgb {
				red: Red::min(),
				green: Green::max(),
				blue: Blue::min(),
				alpha: Some(Alpha::visible())
			}
			.to_string(),
			String::from("rgb(0 255 0 / 1)")
		);

		assert_eq!(
			super::Rgb::from((
				Red::percentage(33.0),
				Green::percentage(66.7),
				Blue::percentage(100.0),
				Some(Alpha::invisible())
			))
			.to_string(),
			String::from("rgb(33% 66.7% 100% / 0)")
		);

		assert_eq!(
			super::Rgb::from((
				Red::percentage(45.0),
				Green::number(0.22),
				Blue::percentage(70.0),
				Some(Alpha::number(0.43))
			))
			.to_string(),
			String::from("rgb(45% 0.22 70% / 0.43)")
		);
	}
}
