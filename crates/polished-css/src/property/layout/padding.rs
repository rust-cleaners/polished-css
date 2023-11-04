//! FIXME: Handle shorthands

macro_rules! create_struct {
	($property:ident, $atomic:expr) => {
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

create_struct!(Padding, "p");
create_struct!(PaddingBlock, "p-bl");
create_struct!(PaddingBlockStart, "p-bl-s");
create_struct!(PaddingBlockEnd, "p-bl-e");
create_struct!(PaddingInline, "p-in");
create_struct!(PaddingInlineStart, "p-in-s");
create_struct!(PaddingInlineEnd, "p-in-e");
create_struct!(PaddingTop, "p-t");
create_struct!(PaddingBottom, "p-b");
create_struct!(PaddingLeft, "p-l");
create_struct!(PaddingRight, "p-r");

#[cfg(test)]
mod test {
	#[test]
	fn paddings() {
		macro_rules! test_property {
			($property:ident, $name:expr, $atomic:expr) => {
				crate::test_property_initial_value!($property, Initial);
				crate::test_global_keywords!($property, $name);
				crate::test_function_var!($property, $name);
				#[cfg(feature = "atomic")]
				crate::test_atomic_property!($property, $atomic);
			};
		}
		test_property!(Padding, "padding", "p");
		test_property!(PaddingBlock, "padding-block", "p-bl");
		test_property!(PaddingBlockStart, "padding-block-start", "p-bl-s");
		test_property!(PaddingBlockEnd, "padding-block-end", "p-bl-e");
		test_property!(PaddingInline, "padding-inline", "p-in");
		test_property!(PaddingInlineStart, "padding-inline-start", "p-in-s");
		test_property!(PaddingInlineEnd, "padding-inline-end", "p-in-e");
		test_property!(PaddingTop, "padding-top", "p-t");
		test_property!(PaddingBottom, "padding-bottom", "p-b");
		test_property!(PaddingLeft, "padding-left", "p-l");
		test_property!(PaddingRight, "padding-right", "p-r");
	}
}
