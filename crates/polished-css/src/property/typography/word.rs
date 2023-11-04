crate::create_property!(
	WordBreak,
	display = "",
	atomic = "word-b",
	custom = false,
	data_type = "",
	initial_value = Normal,
	keywords = "normal,break-all,keep-all,break-word",
);

crate::create_property!(
	WordSpacing,
	display = "",
	atomic = "word-s",
	custom = false,
	data_type = "<length>",
	initial_value = Normal,
	keywords = "normal",
);

#[cfg(test)]
mod test {
	#[test]
	fn word_break() {
		let name = "word-break";
		crate::test_property_initial_value!(WordBreak, Normal);
		crate::test_global_keywords!(WordBreak, name);
		crate::test_function_var!(WordBreak, name);
		#[cfg(feature = "atomic")]
		crate::test_atomic_property!(WordBreak, "word-b");
	}

	#[test]
	fn word_spacing() {
		let name = "word-spacing";
		crate::test_property_initial_value!(WordSpacing, Normal);
		crate::test_global_keywords!(WordSpacing, name);
		crate::test_function_var!(WordSpacing, name);
		#[cfg(feature = "atomic")]
		crate::test_atomic_property!(WordSpacing, "word-s");
	}
}
