crate::create_property!(
	FontFamily,
	display = "",
	atomic = "font-f",
	custom = false,
	data_type = "<string>",
	initial_value = Initial,
	keywords = "",
);

crate::create_property!(
	FontSize,
	display = "",
	atomic = "font-s",
	custom = false,
	data_type = "<length-percentage>",
	initial_value = Initial,
	keywords = "",
);

crate::create_property!(
	FontWeight,
	display = "",
	atomic = "font-w",
	custom = false,
	data_type = "<number>",
	initial_value = Initial,
	keywords = "",
);

#[cfg(test)]
mod test {
	#[test]
	fn font_family() {
		let name = "font-family";
		crate::test_property_initial_value!(FontFamily, Initial);
		crate::test_global_keywords!(FontFamily, name);
		crate::test_function_var!(FontFamily, name);
		#[cfg(feature = "atomic")]
		crate::test_atomic_property!(FontFamily, "font-f");
	}

	#[test]
	fn font_size() {
		let name = "font-size";
		crate::test_property_initial_value!(FontSize, Initial);
		crate::test_global_keywords!(FontSize, name);
		crate::test_function_var!(FontSize, name);
		#[cfg(feature = "atomic")]
		crate::test_atomic_property!(FontSize, "font-s");
	}

	#[test]
	fn font_weight() {
		let name = "font-weight";
		crate::test_property_initial_value!(FontWeight, Initial);
		crate::test_global_keywords!(FontWeight, name);
		crate::test_function_var!(FontWeight, name);
		#[cfg(feature = "atomic")]
		crate::test_atomic_property!(FontWeight, "font-w");
	}
}
