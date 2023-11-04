// TODO: Implement once <image> data type is ready
crate::create_property!(
	ListStyleImage,
	display = "",
	atomic = "list-s-i",
	custom = false,
	data_type = "",
	initial_value = None,
	keywords = "none",
);

crate::create_property!(
	ListStylePosition,
	display = "",
	atomic = "list-s-p",
	custom = false,
	data_type = "",
	initial_value = Inside,
	keywords = "inside,outside",
);

crate::create_property!(
	ListStyleType,
	display = "",
	atomic = "list-s-t",
	custom = false,
	data_type = "<custom-ident>",
	initial_value = Disc,
	keywords = "disc,circle,square,decimal,georgian,trad-chinese-informal,kannada,cyclic,numeric,\
	            alphabtic,symbolic,fixed,custom-counter-style,none",
);

#[cfg(test)]
mod test {
	#[test]
	fn list_style_image() {
		let name = "list-style-image";
		crate::test_property_initial_value!(ListStyleImage, None);
		crate::test_global_keywords!(ListStyleImage, name);
		crate::test_function_var!(ListStyleImage, name);
		#[cfg(feature = "atomic")]
		crate::test_atomic_property!(ListStyleImage, "list-s-i");
	}

	#[test]
	fn list_style_position() {
		let name = "list-style-position";
		crate::test_property_initial_value!(ListStylePosition, Inside);
		crate::test_global_keywords!(ListStylePosition, name);
		crate::test_function_var!(ListStylePosition, name);
		#[cfg(feature = "atomic")]
		crate::test_atomic_property!(ListStylePosition, "list-s-p");
	}

	#[test]
	fn list_style_type() {
		let name = "list-style-type";
		crate::test_property_initial_value!(ListStyleType, Disc);
		crate::test_global_keywords!(ListStyleType, name);
		crate::test_function_var!(ListStyleType, name);
		#[cfg(feature = "atomic")]
		crate::test_atomic_property!(ListStyleType, "list-s-t");
	}
}
