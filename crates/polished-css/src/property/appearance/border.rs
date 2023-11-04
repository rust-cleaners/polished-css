macro_rules! create_border_color_struct {
	($property:ident, $atomic:literal) => {
		$crate::create_property!(
			$property,
			display = "",
			atomic = $atomic,
			custom = false,
			data_type = "<color>",
			initial_value = CurrentColor,
			keywords = "currentColor",
		);
	};
}
create_border_color_struct!(BorderColor, "bor-c");
create_border_color_struct!(BorderBlockColor, "bor-c-bl");
create_border_color_struct!(BorderInlineColor, "bor-c-in");
create_border_color_struct!(BorderTopColor, "bor-c-t");
create_border_color_struct!(BorderBottomColor, "bor-c-b");
create_border_color_struct!(BorderLeftColor, "bor-c-l");
create_border_color_struct!(BorderRightColor, "bor-c-r");

macro_rules! create_border_radius_struct {
	($property:ident, $atomic:literal, $custom:literal) => {
		$crate::create_property!(
			$property,
			display = "",
			atomic = $atomic,
			custom = $custom,
			data_type = "<length-percentage>",
			initial_value = Initial,
			keywords = "",
		);
	};
}
create_border_radius_struct!(BorderRadius, "bor-r", false);
create_border_radius_struct!(BorderTopRadius, "bor-r-t", true);
create_border_radius_struct!(BorderBottomRadius, "bor-r-b", true);
create_border_radius_struct!(BorderLeftRadius, "bor-r-l", true);
create_border_radius_struct!(BorderRightRadius, "bor-r-r", true);
create_border_radius_struct!(BorderTopLeftRadius, "bor-r-t-l", false);
create_border_radius_struct!(BorderTopRightRadius, "bor-r-t-r", false);
create_border_radius_struct!(BorderBottomLeftRadius, "bor-r-b-l", false);
create_border_radius_struct!(BorderBottomRightRadius, "bor-r-b-r", false);

macro_rules! create_border_style_struct {
	($property:ident, $atomic:literal) => {
		$crate::create_property!(
			$property,
			display = "",
			atomic = $atomic,
			custom = false,
			data_type = "",
			initial_value = None,
			keywords = "none,hidden,dotted,dashed,solid,double,groove,ridge,inset,outset",
		);
	};
}
create_border_style_struct!(BorderStyle, "bor-s");
create_border_style_struct!(BorderBlockStyle, "bor-s-bl");
create_border_style_struct!(BorderInlineStyle, "bor-s-in");
create_border_style_struct!(BorderTopStyle, "bor-s-t");
create_border_style_struct!(BorderBottomStyle, "bor-s-b");
create_border_style_struct!(BorderLeftStyle, "bor-s-l");
create_border_style_struct!(BorderRightStyle, "bor-s-r");

macro_rules! create_border_width_struct {
	($property:ident, $atomic:literal) => {
		$crate::create_property!(
			$property,
			display = "",
			atomic = $atomic,
			custom = false,
			data_type = "<length-percentage>",
			initial_value = Initial,
			keywords = "",
		);
	};
}
create_border_width_struct!(BorderWidth, "bor-w");
create_border_width_struct!(BorderBlockWidth, "bor-w-bl");
create_border_width_struct!(BorderInlineWidth, "bor-w-in");
create_border_width_struct!(BorderTopWidth, "bor-w-t");
create_border_width_struct!(BorderBottomWidth, "bor-w-b");
create_border_width_struct!(BorderLeftWidth, "bor-w-l");
create_border_width_struct!(BorderRightWidth, "bor-w-r");

#[cfg(test)]
mod test {
	#[test]
	fn colors() {
		macro_rules! test_property {
			($property:ident, $name:expr, $atomic:expr) => {
				crate::test_property_initial_value!($property, CurrentColor);
				crate::test_global_keywords!($property, $name);
				crate::test_function_var!($property, $name);
				#[cfg(feature = "atomic")]
				crate::test_atomic_property!($property, $atomic);
			};
		}
		test_property!(BorderColor, "border-color", "bor-c");
		test_property!(BorderBlockColor, "border-block-color", "bor-c-bl");
		test_property!(BorderInlineColor, "border-inline-color", "bor-c-in");
		test_property!(BorderTopColor, "border-top-color", "bor-c-t");
		test_property!(BorderBottomColor, "border-bottom-color", "bor-c-b");
		test_property!(BorderLeftColor, "border-left-color", "bor-c-l");
		test_property!(BorderRightColor, "border-right-color", "bor-c-r");
	}

	#[test]
	fn radiuses() {
		macro_rules! test_property {
			($property:ident, $name:expr, $atomic:expr) => {
				crate::test_property_initial_value!($property, Initial);
				crate::test_global_keywords!($property, $name);
				crate::test_function_var!($property, $name);
				#[cfg(feature = "atomic")]
				crate::test_atomic_property!($property, $atomic);
			};
		}
		test_property!(BorderRadius, "border-radius", "bor-r");
		test_property!(BorderTopRadius, "--border-top-radius", "bor-r-t");
		test_property!(BorderBottomRadius, "--border-bottom-radius", "bor-r-b");
		test_property!(BorderLeftRadius, "--border-left-radius", "bor-r-l");
		test_property!(BorderRightRadius, "--border-right-radius", "bor-r-r");
		test_property!(BorderTopLeftRadius, "border-top-left-radius", "bor-r-t-l");
		test_property!(BorderTopRightRadius, "border-top-right-radius", "bor-r-t-r");
		#[rustfmt::skip]
        test_property!(BorderBottomLeftRadius, "border-bottom-left-radius", "bor-r-b-l");
		#[rustfmt::skip]
        test_property!(BorderBottomRightRadius, "border-bottom-right-radius", "bor-r-b-r");
	}

	#[test]
	fn style() {
		macro_rules! test_property {
			($property:ident, $name:expr, $atomic:expr) => {
				crate::test_property_initial_value!($property, None);
				crate::test_global_keywords!($property, $name);
				crate::test_global_keywords!($property, $name);
				crate::test_function_var!($property, $name);
				#[cfg(feature = "atomic")]
				$crate::test_atomic_property!($property, $atomic);
			};
		}
		test_property!(BorderStyle, "border-style", "bor-s");
		test_property!(BorderBlockStyle, "border-block-style", "bor-s-bl");
		test_property!(BorderInlineStyle, "border-inline-style", "bor-s-in");
		test_property!(BorderTopStyle, "border-top-style", "bor-s-t");
		test_property!(BorderBottomStyle, "border-bottom-style", "bor-s-b");
		test_property!(BorderLeftStyle, "border-left-style", "bor-s-l");
		test_property!(BorderRightStyle, "border-right-style", "bor-s-r");
	}

	#[test]
	fn widths() {
		macro_rules! test_property {
			($property:ident, $name:expr, $atomic:expr) => {
				crate::test_property_initial_value!($property, Initial);
				crate::test_global_keywords!($property, $name);
				crate::test_function_var!($property, $name);
				#[cfg(feature = "atomic")]
				crate::test_atomic_property!($property, $atomic);
			};
		}
		test_property!(BorderWidth, "border-width", "bor-w");
		test_property!(BorderBlockWidth, "border-block-width", "bor-w-bl");
		test_property!(BorderInlineWidth, "border-inline-width", "bor-w-in");
		test_property!(BorderTopWidth, "border-top-width", "bor-w-t");
		test_property!(BorderBottomWidth, "border-bottom-width", "bor-w-b");
		test_property!(BorderLeftWidth, "border-left-width", "bor-w-l");
		test_property!(BorderRightWidth, "border-right-width", "bor-w-r");
	}
}
