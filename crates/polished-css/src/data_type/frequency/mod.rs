pub mod unit;

pub use unit::*;

/// [MDN documentation](https://developer.mozilla.org/en-US/docs/Web/CSS/frequency)
#[derive(
	Clone,
	Debug,
	PartialEq,
	strum_macros::EnumIs,
	polished_css_macros::Display,
	polished_css_macros::DataTypeFromUnits,
)]
#[display(on_enum = true)]
pub enum Frequency {
	Hertz(Hz),
	Kilohertz(Khz),
}

#[polished_css_macros::create_trait_from_enum_impl()]
impl Frequency {
	// TODO: Add conversion methods?
}

#[cfg(test)]
mod test {
	#[test]
	fn display() {
		use super::{HertzStorage, KilohertzStorage};
		assert_eq!(super::Frequency::hz(1.0).to_string(), String::from("1Hz"));
		assert_eq!(super::Frequency::khz(1.0).to_string(), String::from("1kHz"));
	}
}
