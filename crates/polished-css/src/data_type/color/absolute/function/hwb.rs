//! [HWB](https://en.wikipedia.org/wiki/HWB_color_model) color model.

use std::fmt;

use crate::data_type::{Alpha, Blackness, Hue, Whiteness};

/// `hwb()` and its `hwba()` alias - specifies sRGB colors by hue,
/// saturation, and lightness using the hwb cylindrical coordinate
/// model.
///
/// ### Resources
///
/// - [MDN documentation](https://developer.mozilla.org/en-US/docs/Web/CSS/hwb)
/// - [CSSWG specification](https://www.w3.org/TR/css-color-3/#hwb-color)
#[derive(Clone, Debug, PartialEq)]
pub struct Hwb {
	/// Dominant wavelength of a color
	pub hue: Hue,
	pub whiteness: Whiteness,
	pub blackness: Blackness,
	/// Optional alpha channel - by default is `1`
	pub alpha: Option<Alpha>,
}

impl fmt::Display for Hwb {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(
			f,
			"hwb({} {} {} / {})",
			self.hue,
			self.whiteness,
			self.blackness,
			self.alpha
				.clone()
				.map_or_else(Alpha::visible, |alpha| alpha)
		)
	}
}

impl<H, W, B, A> From<(H, W, B, A)> for Hwb
where
	H: Into<Hue>,
	W: Into<Whiteness>,
	B: Into<Blackness>,
	A: Into<Option<Alpha>>,
{
	fn from(value: (H, W, B, A)) -> Self {
		let (h, w, b, a) = value;
		Self {
			hue: h.into(),
			whiteness: w.into(),
			blackness: b.into(),
			alpha: a.into(),
		}
	}
}

impl<H, W, B> From<(H, W, B)> for Hwb
where
	H: Into<Hue>,
	W: Into<Whiteness>,
	B: Into<Blackness>,
{
	fn from(value: (H, W, B)) -> Self {
		let (h, w, b) = value;
		Self {
			hue: h.into(),
			whiteness: w.into(),
			blackness: b.into(),
			alpha: Some(Alpha::default()),
		}
	}
}

/// Use `[Hwb]` to store as CSS value
pub trait HwbStorage: From<Hwb> {
	#[must_use]
	fn hwb(value: Hwb) -> Self
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
			super::Hwb {
				hue: Hue::rad(255.0),
				whiteness: Whiteness::reset(),
				blackness: Blackness::number(0.77),
				alpha: Some(Alpha::visible())
			}
			.to_string(),
			String::from("hwb(255rad 0% 0.77 / 1)")
		);

		assert_eq!(
			super::Hwb {
				hue: Hue::turn(1.0),
				whiteness: Whiteness::full(),
				blackness: Blackness::reset(),
				alpha: Some(Alpha::visible())
			}
			.to_string(),
			String::from("hwb(1turn 100% 0% / 1)")
		);

		assert_eq!(
			super::Hwb::from((
				Hue::deg(33.0),
				Whiteness::percentage(66.7),
				Blackness::percentage(100.0),
				Some(Alpha::invisible())
			))
			.to_string(),
			String::from("hwb(33deg 66.7% 100% / 0)")
		);

		assert_eq!(
			super::Hwb::from((
				Hue::grad(45.0),
				Whiteness::number(0.22),
				Blackness::percentage(70.0),
				Some(Alpha::number(0.43))
			))
			.to_string(),
			String::from("hwb(45grad 0.22 70% / 0.43)")
		);
	}
}
