use super::{Angle, Number, NumberStorage};

/// [MDN documentation](https://developer.mozilla.org/en-US/docs/Web/CSS/hue)
#[derive(
	Clone,
	Debug,
	PartialEq,
	strum_macros::EnumIs,
	polished_css_macros::Display,
	polished_css_macros::DataTypeFromDataTypes,
)]
#[display(on_enum = true)]
pub enum Hue {
	/// An angle expressed in **degrees, gradians, radians, or turns**.
	Angle(Angle),
	/// A real number, representing **degrees of the hue's angle**.
	Number(Number),
	/// In certain cases, a color can have one or more **missing color
	/// components**.
	/// [See more in the CSSWG specification](https://drafts.csswg.org/css-color/#missing)
	None,
}

impl From<f64> for Hue {
	fn from(value: f64) -> Self {
		Self::number(value)
	}
}

#[polished_css_macros::create_trait_from_enum_impl(constructor_arg_type = Hue)]
impl Hue {
	// TODO: Add conversion methods?
}

#[cfg(test)]
mod test {
	#[test]
	fn display() {
		use crate::data_type::{DegreeStorage, NumberStorage};
		assert_eq!(super::Hue::deg(0.1).to_string(), String::from("0.1deg"));
		assert_eq!(super::Hue::number(13.37).to_string(), String::from("13.37"));
	}
}
