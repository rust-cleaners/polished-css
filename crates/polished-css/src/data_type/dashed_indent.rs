/// [MDN](https://developer.mozilla.org/en-US/docs/Web/CSS/dashed-ident)
/// [CSSWG](https://drafts.csswg.org/css-values/#dashed-idents)
#[derive(Clone, Debug, PartialEq, polished_css_macros::Deref, polished_css_macros::Display)]
#[display(prefix = "--")]
pub struct DashedIdent(pub String);

impl From<&str> for DashedIdent {
	fn from(value: &str) -> Self {
		Self(value.to_string())
	}
}

pub trait DashedIdentStorage: From<DashedIdent> {
	#[must_use]
	fn dashed_ident(value: &str) -> Self
	where
		Self: Sized,
	{
		Self::from(value.into())
	}
}

#[cfg(test)]
mod test {
	#[test]
	fn unit() {
		assert_eq!(
			super::DashedIdent::from("example-kebab").to_string(),
			String::from("--example-kebab")
		);
		assert_eq!(
			super::DashedIdent::from("example_snake").to_string(),
			String::from("--example_snake")
		);
	}
}
