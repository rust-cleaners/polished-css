use std::fmt::{Display, Formatter, Result};

use super::SelectorDisplay;

#[derive(Clone, Debug, PartialEq)]
pub struct Id(pub String);

impl SelectorDisplay for Id {
    fn as_styles_content(&self) -> String {
        self.to_string()
    }

    fn as_attribute_value(&self) -> String {
        self.to_string()
    }
}

impl Display for Id {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "#{}", self.0)
    }
}

impl From<&str> for Id {
    fn from(value: &str) -> Self {
        Self(value.to_string())
    }
}
