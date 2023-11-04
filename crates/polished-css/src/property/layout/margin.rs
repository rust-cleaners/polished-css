//! FIXME: Handle shorthands

macro_rules! create_struct {
	($property:ident, $atomic:expr) => {
		$crate::create_property!(
			$property,
			display = "",
			atomic = $atomic,
			custom = false,
			data_type = "<length-percentage>",
			initial_value = Auto,
			keywords = "auto",
		);
	};
}

create_struct!(Margin, "m");
create_struct!(MarginBlock, "m-bl");
create_struct!(MarginBlockStart, "m-bl-s");
create_struct!(MarginBlockEnd, "m-bl-e");
create_struct!(MarginInline, "m-in");
create_struct!(MarginInlineStart, "m-in-s");
create_struct!(MarginInlineEnd, "m-in-e");
create_struct!(MarginTop, "m-t");
create_struct!(MarginBottom, "m-b");
create_struct!(MarginLeft, "m-l");
create_struct!(MarginRight, "m-r");

#[cfg(test)]
mod test {
	#[test]
	fn margins() {
		macro_rules! test_property {
			($property:ident, $name:expr, $atomic:expr) => {
				crate::test_property_initial_value!($property, Auto);
				crate::test_global_keywords!($property, $name);
				crate::test_function_var!($property, $name);
				#[cfg(feature = "atomic")]
				crate::test_atomic_property!($property, $atomic);
			};
		}
		test_property!(Margin, "margin", "m");
		test_property!(MarginBlock, "margin-block", "m-bl");
		test_property!(MarginBlockStart, "margin-block-start", "m-bl-s");
		test_property!(MarginBlockEnd, "margin-block-end", "m-bl-e");
		test_property!(MarginInline, "margin-inline", "m-in");
		test_property!(MarginInlineStart, "margin-inline-start", "m-in-s");
		test_property!(MarginInlineEnd, "margin-inline-end", "m-in-e");
		test_property!(MarginTop, "margin-top", "m-t");
		test_property!(MarginBottom, "margin-bottom", "m-b");
		test_property!(MarginLeft, "margin-left", "m-l");
		test_property!(MarginRight, "margin-right", "m-r");
	}
}
