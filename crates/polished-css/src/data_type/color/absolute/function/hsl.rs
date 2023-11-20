use std::fmt;

use crate::data_type::{Alpha, Hue, Lightness, Saturation};

#[derive(Clone, Debug, PartialEq)]
pub struct Hsl {
	pub hue: Hue,
	pub saturation: Saturation,
	pub lightness: Lightness,
	pub alpha: Option<Alpha>,
}

impl fmt::Display for Hsl {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(
			f,
			"hsl({} {} {} / {})",
			self.hue,
			self.saturation,
			self.lightness,
			self.alpha
				.clone()
				.map_or_else(Alpha::visible, |alpha| alpha)
		)
	}
}

impl<H, S, L, A> From<(H, S, L, A)> for Hsl
where
	H: Into<Hue>,
	S: Into<Saturation>,
	L: Into<Lightness>,
	A: Into<Option<Alpha>>,
{
	fn from(value: (H, S, L, A)) -> Self {
		let (r, g, b, a) = value;
		Self {
			hue: r.into(),
			saturation: g.into(),
			lightness: b.into(),
			alpha: a.into(),
		}
	}
}

impl<H, S, L> From<(H, S, L)> for Hsl
where
	H: Into<Hue>,
	S: Into<Saturation>,
	L: Into<Lightness>,
{
	fn from(value: (H, S, L)) -> Self {
		let (r, g, b) = value;
		Self {
			hue: r.into(),
			saturation: g.into(),
			lightness: b.into(),
			alpha: Some(Alpha::default()),
		}
	}
}

pub trait HslStorage: From<Hsl> {
	#[must_use]
	fn hsl(value: Hsl) -> Self
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
			super::Hsl {
				hue: Hue::rad(255.0),
				saturation: Saturation::number(0.77),
				lightness: Lightness::reset(),
				alpha: Some(Alpha::visible())
			}
			.to_string(),
			String::from("hsl(255rad 0.77 0% / 1)")
		);

		assert_eq!(
			super::Hsl {
				hue: Hue::turn(1.0),
				saturation: Saturation::unsaturated(),
				lightness: Lightness::full(),
				alpha: Some(Alpha::visible())
			}
			.to_string(),
			String::from("hsl(1turn 0 100% / 1)")
		);

		assert_eq!(
			super::Hsl::from((
				Hue::deg(33.0),
				Saturation::percentage(66.7),
				Lightness::percentage(100.0),
				Some(Alpha::invisible())
			))
			.to_string(),
			String::from("hsl(33deg 66.7% 100% / 0)")
		);

		assert_eq!(
			super::Hsl::from((
				Hue::grad(45.0),
				Saturation::number(0.22),
				Lightness::percentage(70.0),
				Some(Alpha::number(0.43))
			))
			.to_string(),
			String::from("hsl(45grad 0.22 70% / 0.43)")
		);
	}
}
