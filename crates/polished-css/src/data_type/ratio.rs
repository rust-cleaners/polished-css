/// [MDN](https://developer.mozilla.org/en-US/docs/Web/CSS/ratio)
/// [CSSWG](https://drafts.csswg.org/css-values/#ratios)
#[derive(Clone, Debug, PartialEq)]
pub struct Ratio {
	pub a: f64,
	pub b: Option<f64>,
}

impl std::fmt::Display for Ratio {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		if let Some(b) = self.b {
			write!(f, "{}/{}", self.a, b)
		} else {
			write!(f, "{}", self.a)
		}
	}
}

impl From<i32> for Ratio {
	fn from(value: i32) -> Self {
		Self {
			a: value.into(),
			b: None,
		}
	}
}
impl From<f64> for Ratio {
	fn from(value: f64) -> Self {
		Self { a: value, b: None }
	}
}

impl From<(i32, i32)> for Ratio {
	fn from(value: (i32, i32)) -> Self {
		Self {
			a: value.0.into(),
			b: Some(value.1.into()),
		}
	}
}
impl From<(f64, f64)> for Ratio {
	fn from(value: (f64, f64)) -> Self {
		Self {
			a: value.0,
			b: Some(value.1),
		}
	}
}

impl From<(i32, Option<i32>)> for Ratio {
	fn from(value: (i32, Option<i32>)) -> Self {
		Self {
			a: value.0.into(),
			b: value
				.1
				.map(std::convert::Into::into),
		}
	}
}
impl From<(f64, Option<f64>)> for Ratio {
	fn from(value: (f64, Option<f64>)) -> Self {
		Self {
			a: value.0,
			b: value.1,
		}
	}
}

pub trait RatioStorage: From<Ratio> {
	#[must_use]
	fn ratio(value: Ratio) -> Self
	where
		Self: Sized + From<Ratio>,
	{
		Self::from(value)
	}
}

// TODO: Add bounds, cannot be zero

#[cfg(test)]
mod test {
	#[test]
	fn display() {
		assert_eq!(super::Ratio::from((1, None)).to_string(), "1");
		assert_eq!(super::Ratio::from(1).to_string(), "1");
		assert_eq!(super::Ratio::from((4, Some(3))).to_string(), "4/3");
		assert_eq!(super::Ratio::from((16.0, Some(9.0))).to_string(), "16/9");
		assert_eq!(
			super::Ratio::from((66.6, Some(133.7))).to_string(),
			"66.6/133.7"
		);
	}
}
