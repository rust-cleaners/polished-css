use std::fmt;

use crate::{
	data_type::{Alpha, Chroma, Hue, Lightness},
	prelude::UnitDataType,
};

#[derive(Clone, Debug, PartialEq)]
pub struct Oklch<L, C, H, A>
where
	L: Clone + fmt::Debug + fmt::Display + PartialEq + UnitDataType<Self>,
	C: Clone + fmt::Debug + fmt::Display + PartialEq + UnitDataType<Self>,
	H: Clone + fmt::Debug + fmt::Display + PartialEq + UnitDataType<Self>,
	A: Clone + fmt::Debug + fmt::Display + PartialEq + UnitDataType<Self>,
{
	pub lightness: L,
	pub chroma: C,
	pub hue: H,
	pub alpha: Option<A>,
}

impl<L, C, H, A> fmt::Display for Oklch<L, C, H, A>
where
	L: Clone + fmt::Debug + fmt::Display + PartialEq + UnitDataType<Self>,
	C: Clone + fmt::Debug + fmt::Display + PartialEq + UnitDataType<Self>,
	H: Clone + fmt::Debug + fmt::Display + PartialEq + UnitDataType<Self>,
	A: Clone + fmt::Debug + fmt::Display + PartialEq + UnitDataType<Self>,
{
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(
			f,
			"oklch({} {} {} / {})",
			self.lightness,
			self.chroma,
			self.hue,
			self.alpha
				.clone()
				.map_or_else(Alpha::visible, |alpha| alpha)
		)
	}
}

impl<L, C, H, A> From<(L, C, H, A)> for Oklch<L, C, H, A>
where
	L: Into<Lightness>,
	C: Into<Chroma>,
	H: Into<Hue>,
	A: Into<Option<Alpha>>,
{
	fn from(value: (L, C, H, A)) -> Self {
		let (l, c, h, a) = value;
		Self {
			lightness: l.into(),
			chroma: c.into(),
			hue: h.into(),
			alpha: a.into(),
		}
	}
}

impl<L, C, H> From<(L, C, H)> for Oklch<L, C, H, Alpha>
where
	L: Into<Lightness>,
	C: Into<Chroma>,
	H: Into<Hue>,
{
	fn from(value: (L, C, H)) -> Self {
		let (l, c, h) = value;
		Self {
			lightness: l.into(),
			chroma: c.into(),
			hue: h.into(),
			alpha: Some(Alpha::default()),
		}
	}
}

pub trait OklchStorage: From<Oklch<Lightness, Hue, Chroma, Alpha>> {
	#[must_use]
	fn oklch<L, C, H, A>(value: Oklch<L, C, H, A>) -> Self
	where
		Self: Sized,
		L: Clone + fmt::Debug + fmt::Display + PartialEq + UnitDataType<Self>,
		C: Clone + fmt::Debug + fmt::Display + PartialEq + UnitDataType<Self>,
		H: Clone + fmt::Debug + fmt::Display + PartialEq + UnitDataType<Self>,
		A: Clone + fmt::Debug + fmt::Display + PartialEq + UnitDataType<Self>,
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
			super::Oklch {
				lightness: Lightness::percentage(50.0),
				chroma: Chroma::number(0.4),
				hue: Hue::deg(225.0),
				alpha: Some(Alpha::visible())
			}
			.to_string(),
			String::from("oklch(50% 0.4 225deg / 1)")
		);

		assert_eq!(
			super::Oklch::from((
				Lightness::number(0.33),
				Chroma::percentage(44.0),
				Hue::turn(1.5),
				Some(Alpha::invisible())
			))
			.to_string(),
			String::from("oklch(0.33 44% 1.5turn / 0)")
		);

		assert_eq!(
			super::Oklch::from((
				Lightness::percentage(45.0),
				Chroma::number(0.22),
				Hue::grad(120.0),
				Some(Alpha::number(0.43))
			))
			.to_string(),
			String::from("oklch(45% 0.22 120grad / 0.43)")
		);
	}
}
