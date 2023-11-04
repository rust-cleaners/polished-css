//! [CSSWG specification](https://www.w3.org/TR/css-values-4/#font-relative-lengths)

pub mod unit;

pub use unit::*;

/// [CSSWG specification](https://www.w3.org/TR/css-values-4/#font-relative-lengths)
#[derive(
	Clone,
	Debug,
	PartialEq,
	strum_macros::EnumIs,
	polished_css_macros::Display,
	polished_css_macros::DataTypeFromUnits,
)]
#[display(on_enum = true)]
pub enum FontLength {
	Em(Em),
	RelativeEm(Rem),
	Ex(Ex),
	RelativeEx(Rex),
	CapHeight(Cap),
	RelativeCapHeight(Rcap),
	CharSize(Ch),
	RelativeCharSize(Rch),
	Ic(Ic),
	RelativeIc(Ric),
	LineHeight(Lh),
	RelativeLineHeight(Rlh),
}

#[polished_css_macros::create_trait_from_enum_impl()]
impl FontLength {
	// TODO: Add conversion methods?
}

#[cfg(test)]
mod test {

	#[test]
	fn display() {
		use super::{
			CapHeightStorage, CharSizeStorage, EmStorage, ExStorage, IcStorage, LineHeightStorage,
			RelativeCapHeightStorage, RelativeCharSizeStorage, RelativeEmStorage,
			RelativeExStorage, RelativeIcStorage, RelativeLineHeightStorage,
		};

		assert_eq!(super::FontLength::em(1.0).to_string(), String::from("1em"));
		assert_eq!(
			super::FontLength::rem(1.0).to_string(),
			String::from("1rem")
		);

		assert_eq!(super::FontLength::ex(1.0).to_string(), String::from("1ex"));
		assert_eq!(
			super::FontLength::rex(1.0).to_string(),
			String::from("1rex")
		);

		assert_eq!(
			super::FontLength::cap(1.0).to_string(),
			String::from("1cap")
		);
		assert_eq!(
			super::FontLength::rcap(1.0).to_string(),
			String::from("1rcap")
		);

		assert_eq!(super::FontLength::ch(1).to_string(), String::from("1ch"));
		assert_eq!(super::FontLength::rch(1).to_string(), String::from("1rch"));

		assert_eq!(super::FontLength::ic(1).to_string(), String::from("1ic"));
		assert_eq!(super::FontLength::ric(1).to_string(), String::from("1ric"));

		assert_eq!(super::FontLength::lh(1.0).to_string(), String::from("1lh"));
		assert_eq!(
			super::FontLength::rlh(1.0).to_string(),
			String::from("1rlh")
		);
	}
}
