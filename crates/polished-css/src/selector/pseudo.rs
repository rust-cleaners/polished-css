use std::fmt::{Display, Formatter, Result};

#[derive(Clone, Debug, PartialEq)]
pub enum PseudoSelector {
	Element(PseudoElement),
	Other(String),
}

impl Display for PseudoSelector {
	fn fmt(&self, f: &mut Formatter<'_>) -> Result {
		write!(
			f,
			"{}",
			match self {
				Self::Element(name) => name.to_string(),
				Self::Other(name) => name.to_string(),
			}
		)
	}
}

#[derive(Clone, Debug, PartialEq, strum_macros::Display)]
#[strum(serialize_all = "lowercase")]
pub enum PseudoElement {
	Before,
	After,
}
