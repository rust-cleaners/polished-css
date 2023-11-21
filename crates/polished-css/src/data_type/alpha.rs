//! Alpha channel or transparency of a color.

use crate::data_type::{Number, NumberStorage, Percentage};

/// Alpha channel or transparency of a color.
///
/// ### Resources
///
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
pub enum Alpha {
	// NOTE: We need to override to add bounds
	// TODO: #[custom_constraint(fn_name)]
	Number(Number),
	// NOTE: We need to override to add bounds
	Percentage(Percentage),
}

impl Default for Alpha {
	fn default() -> Self {
		Self::visible()
	}
}

impl From<f64> for Alpha {
	fn from(value: f64) -> Self {
		Self::number(value)
	}
}

#[polished_css_macros::create_trait_from_enum_impl()]
impl Alpha {
	#[must_use]
	pub fn invisible() -> Self {
		Self::number(0.0)
	}

	#[must_use]
	pub fn visible() -> Self {
		Self::number(1.0)
	}

	#[must_use]
	pub const fn full() -> Self {
		Self::Percentage(Percentage::full())
	}

	#[must_use]
	pub const fn reset() -> Self {
		Self::Percentage(Percentage::reset())
	}

	// TODO: Add conversion methods?
}

#[cfg(test)]
mod test {
	#[test]
	fn display() {
		use crate::data_type::*;
		assert_eq!(super::Alpha::number(0.1).to_string(), String::from("0.1"));
		assert_eq!(super::Alpha::visible().to_string(), String::from("1"));
		assert_eq!(super::Alpha::invisible().to_string(), String::from("0"));
		assert_eq!(
			super::Alpha::percentage(10.0).to_string(),
			String::from("10%")
		);
		assert_eq!(super::Alpha::full().to_string(), String::from("100%"));
		assert_eq!(super::Alpha::reset().to_string(), String::from("0%"));
	}
}
