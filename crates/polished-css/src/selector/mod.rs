//! Patterns that match against elements in a tree, and as such form one of
//! several technologies that can be used to select nodes in a document.
//!
//! ### Resources
//!
//! - [CSSWG specification](https://drafts.csswg.org/selectors/)
//! - [MDN documentation](https://developer.mozilla.org/en-US/docs/Web/CSS/CSS_Selectors)

pub mod class;
pub mod element;
pub mod id;
pub mod pseudo;

pub use class::*;
pub use element::*;
pub use id::*;
pub use pseudo::*;

#[derive(Clone, Debug, PartialEq, strum_macros::EnumIs)]
pub enum Selector {
	Inlined,
	Root,
	Id(Id),
	Element(Element),
	Class(Class),
}

impl Default for Selector {
	fn default() -> Self {
		Self::Inlined
	}
}

impl SelectorDisplay for Selector {
	fn as_styles_content(&self) -> String {
		match self {
			Self::Inlined => String::new(),
			Self::Root => String::from(":root"),
			Self::Id(id) => id.as_styles_content(),
			Self::Element(el) => el.as_styles_content(),
			Self::Class(name) => name.as_styles_content(),
		}
	}

	fn as_attribute_value(&self) -> String {
		match self {
			Self::Inlined => String::new(),
			Self::Root => String::from(":root"),
			Self::Id(id) => id.as_attribute_value(),
			Self::Element(el) => el.as_attribute_value(),
			Self::Class(name) => name.as_attribute_value(),
		}
	}
}

impl From<Id> for Selector {
	fn from(value: Id) -> Self {
		Self::Id(value)
	}
}

impl From<Element> for Selector {
	fn from(value: Element) -> Self {
		Self::Element(value)
	}
}

impl From<Class> for Selector {
	fn from(value: Class) -> Self {
		Self::Class(value)
	}
}

pub trait SelectorDisplay {
	fn as_attribute_value(&self) -> String;
	fn as_styles_content(&self) -> String;
}
