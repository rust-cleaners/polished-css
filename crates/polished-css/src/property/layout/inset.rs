macro_rules! create_struct {
	($property:ident, $atomic:literal) => {
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

create_struct!(Inset, "inset");
create_struct!(InsetBlock, "inset-bl");
create_struct!(InsetInline, "inset-in");

create_struct!(Top, "t");
create_struct!(Bottom, "b");
create_struct!(Left, "l");
create_struct!(Right, "r");

#[cfg(test)]
mod test {
	#[test]
	fn insets() {
		macro_rules! test_property {
			($property:ident, $name:expr, $atomic:expr) => {
				crate::test_property_initial_value!($property, Auto);
				crate::test_global_keywords!($property, $name);
				crate::test_function_var!($property, $name);
				#[cfg(feature = "atomic")]
				crate::test_atomic_property!($property, $atomic);
			};
		}

		test_property!(Inset, "inset", "inset");
		test_property!(InsetBlock, "inset-block", "inset-bl");
		test_property!(InsetInline, "inset-inline", "inset-in");

		test_property!(Top, "top", "t");
		test_property!(Bottom, "bottom", "b");
		test_property!(Left, "left", "l");
		test_property!(Right, "right", "r");
	}
}
