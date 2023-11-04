//! [CSSWG specification](https://www.w3.org/TR/css-values-4/#time)

crate::create_unit!(Ms, "ms", usize, Millisecond);
crate::create_unit!(S, "s", f64, Second);

#[cfg(test)]
mod test {
	#[test]
	fn display() {
		assert_eq!(super::Ms(1).to_string(), String::from("1ms"));
		assert_eq!(super::S(1.0).to_string(), String::from("1s"));
	}
}
