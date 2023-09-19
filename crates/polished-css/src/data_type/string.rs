/// [MDN documentation](https://developer.mozilla.org/en-US/docs/Web/CSS/string)
/// [CSSWG specification](https://drafts.csswg.org/css-values/#string)
/// NOTE: Is prefixed with `DataType`
/// to avoid naming pitfalls with Rust's literal `String`.
#[derive(Clone, PartialEq, Debug, polished_css_macros::Deref, polished_css_macros::Display)]
pub struct DataTypeString(pub String);

impl From<&str> for DataTypeString {
    fn from(value: &str) -> Self {
        Self(value.to_string())
    }
}

pub trait StringStorage: From<DataTypeString> {
    #[must_use]
    fn string(value: &str) -> Self
    where
        Self: Sized,
    {
        Self::from(value.into())
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn display() {
        assert_eq!(
            super::DataTypeString::from("it").to_string(),
            String::from("it")
        );
        assert_eq!(
            super::DataTypeString(String::from("works")).to_string(),
            String::from("works")
        );
    }
}
