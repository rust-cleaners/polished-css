//! Cascading variables as a new primitive value type that is accepted by all
//! CSS properties, and custom properties for defining them.
//!
//! ### Resources
//!
//! - [CSSWG specification](https://drafts.csswg.org/css-variables-1/)
//! - [MDN documentation](https://developer.mozilla.org/en-US/docs/Web/CSS/Using_CSS_custom_properties)

use crate::{data_type::DashedIdent, utils::Nothing};

/// Cascading variables as a new primitive value type that is accepted by all
/// CSS properties, and custom properties for defining them.
///
/// ### Resources
///
/// - [CSSWG specification](https://drafts.csswg.org/css-variables-1/)
/// - [MDN documentation](https://developer.mozilla.org/en-US/docs/Web/CSS/Using_CSS_custom_properties)
#[derive(Clone, Debug, PartialEq)]
pub struct Var<T>
where
    T: Clone + std::fmt::Debug + std::fmt::Display + PartialEq,
{
    pub dashed_ident: DashedIdent,
    pub fallback: Option<T>,
}

impl<T> std::fmt::Display for Var<T>
where
    T: Clone + std::fmt::Debug + std::fmt::Display + PartialEq,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(fallback) = &self.fallback {
            write!(f, "var({},{})", self.dashed_ident, fallback)
        } else {
            write!(f, "var({})", self.dashed_ident)
        }
    }
}

impl<T> crate::utils::UnitDataTypeContainer<T> for Var<T> where
    T: Clone + std::fmt::Debug + std::fmt::Display + PartialEq + crate::utils::UnitDataType<Self>
{
}

impl From<DashedIdent> for Var<Nothing> {
    fn from(value: DashedIdent) -> Self {
        Self {
            dashed_ident: value,
            fallback: None,
        }
    }
}

impl From<&str> for Var<Nothing> {
    fn from(value: &str) -> Self {
        Self {
            dashed_ident: value.into(),
            fallback: None,
        }
    }
}

impl<T> From<(DashedIdent, Option<T>)> for Var<T>
where
    T: Clone + std::fmt::Debug + std::fmt::Display + PartialEq + crate::utils::UnitDataType<Self>,
{
    fn from(value: (DashedIdent, Option<T>)) -> Self {
        let (dashed_ident, fallback) = value;
        Self {
            dashed_ident,
            fallback,
        }
    }
}

impl<T> From<(DashedIdent, T)> for Var<T>
where
    T: Clone + std::fmt::Debug + std::fmt::Display + PartialEq + crate::utils::UnitDataType<Self>,
{
    fn from(value: (DashedIdent, T)) -> Self {
        let (dashed_ident, fallback) = value;
        Self {
            dashed_ident,
            fallback: Some(fallback),
        }
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn display() {
        // With fallback
        assert_eq!(
            super::Var::<crate::property::AllValue> {
                dashed_ident: "example-with-fallback".into(),
                fallback: Some(crate::property::AllValue::Initial)
            }
            .to_string(),
            String::from("var(--example-with-fallback,initial)"),
        );
        // Without fallback
        assert_eq!(
            super::Var::from("example-without-fallback").to_string(),
            String::from("var(--example-without-fallback)"),
        );
    }
}
