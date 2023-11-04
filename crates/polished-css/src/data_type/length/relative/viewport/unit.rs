//! [CSSWG specification](https://www.w3.org/TR/css-values-4/#viewport-relative-lengths)

crate::create_unit!(Vw, "vw", f64, ViewportWidth);
crate::create_unit!(Svw, "svw", f64, SmallViewportWidth);
crate::create_unit!(Lvw, "lvw", f64, LargeViewportWidth);
crate::create_unit!(Dvw, "dvw", f64, DynamicViewportWidth);

crate::create_unit!(Vh, "vh", f64, ViewportHeight);
crate::create_unit!(Svh, "svh", f64, SmallViewportHeight);
crate::create_unit!(Lvh, "lvh", f64, LargeViewportHeight);
crate::create_unit!(Dvh, "dvh", f64, DynamicViewportHeight);

crate::create_unit!(Vi, "vi", f64, ViewportInline);
crate::create_unit!(Svi, "svi", f64, SmallViewportInline);
crate::create_unit!(Lvi, "lvi", f64, LargeViewportInline);
crate::create_unit!(Dvi, "dvi", f64, DynamicViewportInline);

crate::create_unit!(Vb, "vb", f64, ViewportBlock);
crate::create_unit!(Svb, "svb", f64, SmallViewportBlock);
crate::create_unit!(Lvb, "lvb", f64, LargeViewportBlock);
crate::create_unit!(Dvb, "dvb", f64, DynamicViewportBlock);

crate::create_unit!(Vmin, "vmin", f64, ViewportMin);
crate::create_unit!(Svmin, "svmin", f64, SmallViewportMin);
crate::create_unit!(Lvmin, "lvmin", f64, LargeViewportMin);
crate::create_unit!(Dvmin, "dvmin", f64, DynamicViewportMin);

crate::create_unit!(Vmax, "vmax", f64, ViewportMax);
crate::create_unit!(Svmax, "svmax", f64, SmallViewportMax);
crate::create_unit!(Lvmax, "lvmax", f64, LargeViewportMax);
crate::create_unit!(Dvmax, "dvmax", f64, DynamicViewportMax);

#[cfg(test)]
mod test {
	#[test]
	fn display() {
		assert_eq!(super::Vh(1.0).to_string(), String::from("1vh"));
		assert_eq!(super::Svh(1.0).to_string(), String::from("1svh"));
		assert_eq!(super::Lvh(1.0).to_string(), String::from("1lvh"));
		assert_eq!(super::Dvh(1.0).to_string(), String::from("1dvh"));

		assert_eq!(super::Vw(1.0).to_string(), String::from("1vw"));
		assert_eq!(super::Svw(1.0).to_string(), String::from("1svw"));
		assert_eq!(super::Lvw(1.0).to_string(), String::from("1lvw"));
		assert_eq!(super::Dvw(1.0).to_string(), String::from("1dvw"));

		assert_eq!(super::Vi(1.0).to_string(), String::from("1vi"));
		assert_eq!(super::Svi(1.0).to_string(), String::from("1svi"));
		assert_eq!(super::Lvi(1.0).to_string(), String::from("1lvi"));
		assert_eq!(super::Dvi(1.0).to_string(), String::from("1dvi"));

		assert_eq!(super::Vb(1.0).to_string(), String::from("1vb"));
		assert_eq!(super::Svb(1.0).to_string(), String::from("1svb"));
		assert_eq!(super::Lvb(1.0).to_string(), String::from("1lvb"));
		assert_eq!(super::Dvb(1.0).to_string(), String::from("1dvb"));

		assert_eq!(super::Vmin(1.0).to_string(), String::from("1vmin"));
		assert_eq!(super::Svmin(1.0).to_string(), String::from("1svmin"));
		assert_eq!(super::Lvmin(1.0).to_string(), String::from("1lvmin"));
		assert_eq!(super::Dvmin(1.0).to_string(), String::from("1dvmin"));

		assert_eq!(super::Vmax(1.0).to_string(), String::from("1vmax"));
		assert_eq!(super::Svmax(1.0).to_string(), String::from("1svmax"));
		assert_eq!(super::Lvmax(1.0).to_string(), String::from("1lvmax"));
		assert_eq!(super::Dvmax(1.0).to_string(), String::from("1dvmax"));
	}
}
