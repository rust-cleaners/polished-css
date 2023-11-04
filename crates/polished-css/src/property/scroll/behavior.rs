crate::create_property!(
	ScrollBehavior,
	display = "",
	atomic = "scroll-b",
	custom = false,
	data_type = "",
	initial_value = Auto,
	keywords = "auto,smooth",
);

#[cfg(test)]
mod test {
	#[test]
	fn scroll_behavior() {
		let name = "scroll-behavior";
		crate::test_property_initial_value!(ScrollBehavior, Auto);
		crate::test_global_keywords!(ScrollBehavior, name);
		crate::test_function_var!(ScrollBehavior, name);
		#[cfg(feature = "atomic")]
		crate::test_atomic_property!(ScrollBehavior, "scroll-b");
	}
}
