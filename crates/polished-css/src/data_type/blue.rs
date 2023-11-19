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
pub enum Blue {
	// NOTE: We need to override to add bounds - min - 0 max 255
	// TODO: #[custom_constraint(fn_name)]
	Number(Number),
	// NOTE: We need to override to add bounds
	Percentage(Percentage),
}

impl From<f64> for Blue {
	fn from(value: f64) -> Self {
		Self::number(value)
	}
}

#[polished_css_macros::create_trait_from_enum_impl()]
impl Blue {
	#[must_use]
	pub const fn min() -> Self {
		Self::Number(Number(0.0))
	}

	#[must_use]
	pub const fn max() -> Self {
		Self::Number(Number(255.0))
	}

	// TODO: Add conversion methods?
}

#[cfg(test)]
mod test {
	#[test]
	fn display() {
		use crate::data_type::*;
		assert_eq!(super::Blue::number(0.1).to_string(), String::from("0.1"));
		assert_eq!(
			super::Blue::percentage(10.0).to_string(),
			String::from("10%")
		);
		assert_eq!(super::Blue::min().to_string(), String::from("0"));
		assert_eq!(super::Blue::max().to_string(), String::from("255"));
	}
}
