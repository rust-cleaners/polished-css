//! [CSSWG specification](https://www.w3.org/TR/css-contain-3/#container-lengths)

pub mod unit;

pub use unit::*;

/// [CSSWG specification](https://www.w3.org/TR/css-contain-3/#container-lengths)
#[derive(
	Clone,
	Debug,
	PartialEq,
	strum_macros::EnumIs,
	polished_css_macros::Display,
	polished_css_macros::DataTypeFromUnits,
)]
#[display(on_enum = true)]
pub enum ContainerLength {
	/// 1% of a query container's width
	QueryWidth(Cqw),
	/// 1% of a query container's height
	QueryHeight(Cqh),
	/// 1% of a query container's inline size
	QueryInline(Cqi),
	/// 1% of a query container's block size
	QueryBlock(Cqb),
	/// The smaller value of either `cqi` or `cqb`
	QueryMin(Cqmin),
	/// The larger value of either `cqi` or `cqb`
	QueryMax(Cqmax),
}

#[polished_css_macros::create_trait_from_enum_impl()]
impl ContainerLength {
	// TODO: Add conversion methods
}

#[cfg(test)]
mod test {
	#[test]
	fn display() {
		use super::{
			ContainerQueryBlockStorage, ContainerQueryHeightStorage, ContainerQueryInlineStorage,
			ContainerQueryMaxStorage, ContainerQueryMinStorage, ContainerQueryWidthStorage,
		};

		assert_eq!(
			super::ContainerLength::cqw(1.0).to_string(),
			String::from("1cqw")
		);
		assert_eq!(
			super::ContainerLength::cqh(1.0).to_string(),
			String::from("1cqh")
		);

		assert_eq!(
			super::ContainerLength::cqi(1.0).to_string(),
			String::from("1cqi")
		);
		assert_eq!(
			super::ContainerLength::cqb(1.0).to_string(),
			String::from("1cqb")
		);

		assert_eq!(
			super::ContainerLength::cqmin(1.0).to_string(),
			String::from("1cqmin")
		);
		assert_eq!(
			super::ContainerLength::cqmax(1.0).to_string(),
			String::from("1cqmax")
		);
	}
}
