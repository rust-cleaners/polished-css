use crate::data_type::{Alpha, Chroma, Hue, Lightness};

#[derive(Clone, Debug, PartialEq)]
pub struct Oklch {
	pub lightness: Lightness,
	pub chroma: Chroma,
	pub hue: Hue,
	pub alpha: Option<Alpha>,
}

impl std::fmt::Display for Oklch {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
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

impl<L, C, H, A> From<(L, C, H, A)> for Oklch
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

impl<L, C, H> From<(L, C, H)> for Oklch
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

// TODO: Macro'ify it when possible
pub trait OklchStorage: From<Oklch> {
	#[must_use]
	fn oklch(value: Oklch) -> Self
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
