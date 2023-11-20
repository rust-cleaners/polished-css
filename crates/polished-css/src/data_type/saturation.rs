use crate::data_type::{Number, NumberStorage, Percentage};

/// [MDN documentation](https://developer.mozilla.org/en-US/docs/Web/CSS/alpha_value)
#[derive(
	Clone,
	Debug,
	PartialEq,
	strum_macros::EnumIs,
	polished_css_macros::Display,
	polished_css_macros::DataTypeFromDataTypes,
)]
#[display(on_enum = true)]
pub enum Saturation {
	// NOTE: We need to override to add bounds
	// TODO: #[custom_constraint(fn_name)] - min - 0, max - 1
	Number(Number),
	// NOTE: We need to override to add bounds
	Percentage(Percentage),
}

impl From<f64> for Saturation {
	fn from(value: f64) -> Self {
		Self::number(value)
	}
}

#[polished_css_macros::create_trait_from_enum_impl()]
impl Saturation {
	#[must_use]
	/// Completely unsaturated _(gray)_
	pub fn unsaturated() -> Self {
		Self::number(0.0)
	}

	/// Fully saturated
	#[must_use]
	pub fn saturated() -> Self {
		Self::number(1.0)
	}

	// TODO: Add conversion methods?
}

#[cfg(test)]
mod test {
	#[test]
	fn display() {
		use crate::data_type::*;
		assert_eq!(
			super::Saturation::number(0.1).to_string(),
			String::from("0.1")
		);
		assert_eq!(
			super::Saturation::percentage(10.0).to_string(),
			String::from("10%")
		);
		assert_eq!(
			super::Saturation::saturated().to_string(),
			String::from("1")
		);
		assert_eq!(
			super::Saturation::unsaturated().to_string(),
			String::from("0")
		);
	}
}
