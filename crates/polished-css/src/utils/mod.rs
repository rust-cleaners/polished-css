//! Utilities to be used along with this crate.

pub const PROPERTY_VALUE_SUFFIX: &str = "Value";
pub const DATA_TYPE_OPTIONAL_SUFFIX: &str = "DataType";
pub const DATA_TYPE_TRAIT_SUFFIX: &str = "Storage";

pub trait UnitDataTypeContainer<T: UnitDataType<Self>>: Sized {}

pub trait UnitDataType<Container: UnitDataTypeContainer<Self>>: Sized {}

#[derive(Clone, Debug, PartialEq)]
pub struct Nothing;
impl std::fmt::Display for Nothing {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "")
	}
}
impl UnitDataType<Nothing> for Nothing {}
impl<T> UnitDataTypeContainer<T> for Nothing where
	T: Clone + std::fmt::Debug + std::fmt::Display + PartialEq + UnitDataType<Self>
{
}
