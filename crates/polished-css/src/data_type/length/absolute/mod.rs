//! [CSSWG specification](https://www.w3.org/TR/css-values-4/#absolute-lengths)

pub mod unit;

pub use unit::*;

/// [CSSWG specification](https://www.w3.org/TR/css-values-4/#absolute-lengths)
#[derive(
	Clone,
	Debug,
	PartialEq,
	strum_macros::EnumIs,
	polished_css_macros::Display,
	polished_css_macros::DataTypeFromUnits,
)]
#[display(on_enum = true)]
pub enum AbsoluteLength {
	Centimeter(Cm),
	Millimeter(Mm),
	QuarterOfMillimeter(Q),
	Inch(In),
	Pica(Pc),
	Point(Pt),
	Pixel(Px),
}

#[polished_css_macros::create_trait_from_enum_impl()]
impl AbsoluteLength {
	// TODO: Add conversion methods?
}

#[cfg(test)]
mod test {
	#[test]
	fn display() {
		use super::unit::*;

		assert_eq!(
			super::AbsoluteLength::cm(1.0).to_string(),
			String::from("1cm")
		);
		assert_eq!(
			super::AbsoluteLength::mm(1.0).to_string(),
			String::from("1mm")
		);
		assert_eq!(super::AbsoluteLength::q(1).to_string(), String::from("1Q"));
		assert_eq!(
			super::AbsoluteLength::r#in(1.0).to_string(),
			String::from("1in")
		);
		assert_eq!(
			super::AbsoluteLength::pc(1).to_string(),
			String::from("1pc")
		);
		assert_eq!(
			super::AbsoluteLength::pt(1).to_string(),
			String::from("1pt")
		);
		assert_eq!(
			super::AbsoluteLength::px(1).to_string(),
			String::from("1px")
		);
	}
}
