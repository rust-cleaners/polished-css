use crate::data_type::{Frequency, Percentage};

/// [MDN documentation](https://developer.mozilla.org/en-US/docs/Web/CSS/frequency-percentage)
#[derive(
	Clone,
	Debug,
	PartialEq,
	strum_macros::EnumIs,
	polished_css_macros::Display,
	polished_css_macros::DataTypeFromDataTypes,
)]
#[display(on_enum = true)]
pub enum FrequencyPercentage {
	Frequency(Frequency),
	Percentage(Percentage),
}

#[polished_css_macros::create_trait_from_enum_impl()]
impl FrequencyPercentage {
	// TODO: Add conversion methods?
}

#[cfg(test)]
mod test {
	use crate::prelude::{HertzStorage, PercentageStorage};

	#[test]
	fn display() {
		assert_eq!(
			super::FrequencyPercentage::hz(1.0).to_string(),
			String::from("1Hz")
		);
		assert_eq!(
			super::FrequencyPercentage::percentage(1.0).to_string(),
			String::from("1%")
		);
	}
}
