pub mod unit;

pub use unit::*;

/// [MDN documentation](https://developer.mozilla.org/en-US/docs/Web/CSS/angle)
#[derive(
	Clone,
	Debug,
	PartialEq,
	strum_macros::EnumIs,
	polished_css_macros::Display,
	polished_css_macros::DataTypeFromUnits,
)]
#[display(on_enum = true)]
pub enum Angle {
	Degree(Deg),
	Radian(Rad),
	Gradian(Grad),
	Turn(Turn),
}

#[polished_css_macros::create_trait_from_enum_impl()]
impl Angle {
	#[must_use]
	const fn reset() -> Self
	where
		Self: Sized,
	{
		Self::Degree(Deg(0.0))
	}

	// TODO: Add conversion methods?
}

#[cfg(test)]
mod test {
	#[test]
	fn display() {
		assert_eq!(super::Angle::reset(), super::Angle::Degree(super::Deg(0.0)));

		assert_eq!(
			super::Angle::from(super::Deg(180.0)).to_string(),
			String::from("180deg")
		);
		assert_eq!(
			super::Angle::from(super::Rad(120.0)).to_string(),
			String::from("120rad")
		);
		assert_eq!(
			super::Angle::from(super::Grad(100.0)).to_string(),
			String::from("100grad")
		);
		assert_eq!(
			super::Angle::from(super::Turn(1.0)).to_string(),
			String::from("1turn")
		);
	}
}
