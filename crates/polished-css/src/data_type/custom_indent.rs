/// [MDN](https://developer.mozilla.org/en-US/docs/Web/CSS/custom-ident)
/// [CSSWG](https://drafts.csswg.org/css-values/#custom-idents)
#[derive(Clone, Debug, PartialEq, polished_css_macros::Deref, polished_css_macros::Display)]
pub struct CustomIdent(pub String);

impl From<&str> for CustomIdent {
    fn from(value: &str) -> Self {
        Self(value.to_string())
    }
}

pub trait CustomIdentStorage: From<CustomIdent> {
    #[must_use]
    fn custom_ident(value: &str) -> Self
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
            super::CustomIdent::from("example-kebab").to_string(),
            String::from("example-kebab")
        );
        assert_eq!(
            super::CustomIdent::from("example_snake").to_string(),
            String::from("example_snake")
        );
    }
}
