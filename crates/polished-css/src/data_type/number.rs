/// [MDN documentation](https://developer.mozilla.org/en-US/docs/Web/CSS/number)
/// [CSSWG specification](https://drafts.csswg.org/css-values/#number)
#[derive(Clone, Debug, PartialEq, polished_css_macros::Deref, polished_css_macros::Display)]
pub struct Number(pub f64);

impl Number {
	#[must_use]
	pub const fn zero() -> Self {
		Self(0.0)
	}
}

pub trait NumberStorage: From<Number> {
	#[must_use]
	fn number(value: f64) -> Self
	where
		Self: Sized,
	{
		Self::from(Number(value))
	}

	#[must_use]
	fn zero() -> Self
	where
		Self: Sized,
	{
		Self::from(Number(0.0))
	}
}

#[cfg(test)]
mod test {
	#[test]
	fn display() {
		assert_eq!(super::Number(13.37).to_string(), String::from("13.37"));
		assert_eq!(super::Number(-100.0).to_string(), String::from("-100"));
		assert_eq!(super::Number::zero().to_string(), String::from("0"));
	}
}
