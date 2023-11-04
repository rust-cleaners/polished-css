//! [CSSWG specification](https://www.w3.org/TR/css-values-4/#absolute-lengths)

crate::create_unit!(Cm, "cm", f64, Centimeter);
crate::create_unit!(Mm, "mm", f64, Millimeter);
crate::create_unit!(Q, "Q", isize, QuarterOfMillimeter);
crate::create_unit!(In, "in", f64, Inch);
crate::create_unit!(Pc, "pc", isize, Pica);
crate::create_unit!(Pt, "pt", isize, Point);
crate::create_unit!(Px, "px", isize, Pixel);

#[cfg(test)]
mod test {
	#[test]
	fn display() {
		assert_eq!(super::Q(1).to_string(), String::from("1Q"));
		assert_eq!(super::Mm(1.0).to_string(), String::from("1mm"));
		assert_eq!(super::Cm(1.0).to_string(), String::from("1cm"));
		assert_eq!(super::In(1.0).to_string(), String::from("1in"));
		assert_eq!(super::Pc(1).to_string(), String::from("1pc"));
		assert_eq!(super::Pt(1).to_string(), String::from("1pt"));
		assert_eq!(super::Px(1).to_string(), String::from("1px"));
	}
}
