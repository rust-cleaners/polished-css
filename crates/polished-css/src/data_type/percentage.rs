/// [MDN documentation](https://developer.mozilla.org/en-US/docs/Web/CSS/percentage)
/// [CSSWG display](https://drafts.csswg.org/css-values/#percentage)
#[derive(Clone, Debug, PartialEq, polished_css_macros::Deref, polished_css_macros::Display)]
#[display(suffix = "%")]
pub struct Percentage(pub f64);

impl Percentage {
	#[must_use]
	pub const fn full() -> Self {
		Self(100.0)
	}

	#[must_use]
	pub const fn reset() -> Self {
		Self(0.0)
	}
}

pub trait PercentageStorage: From<Percentage> {
	#[must_use]
	fn percentage(value: f64) -> Self
	where
		Self: Sized,
	{
		Self::from(Percentage(value))
	}

	#[must_use]
	fn full() -> Self
	where
		Self: Sized,
	{
		Self::percentage(100.0)
	}

	#[must_use]
	fn reset() -> Self
	where
		Self: Sized,
	{
		Self::percentage(0.0)
	}
}

#[cfg(test)]
mod test {
	#[test]
	fn percentage() {
		assert_eq!(super::Percentage(66.67).to_string(), String::from("66.67%"));
		assert_eq!(super::Percentage(-1.0).to_string(), String::from("-1%"));
		assert_eq!(super::Percentage::full().to_string(), String::from("100%"));
		assert_eq!(super::Percentage::reset().to_string(), String::from("0%"));
	}
}
