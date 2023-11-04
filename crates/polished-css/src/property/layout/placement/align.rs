crate::create_property!(
	AlignContent,
	display = "",
	atomic = "align-c",
	custom = false,
	data_type = "",
	initial_value = Normal,
	keywords = "normal,first,last,baseline,space-around,space-between,space-evenly,stretch,safe,\
	            unsafe,center,start,end,flex-start,flex-end",
);

crate::create_property!(
	AlignItems,
	display = "",
	atomic = "align-i",
	custom = false,
	data_type = "",
	initial_value = Normal,
	keywords = "normal,stretch,first,last,baseline,safe,unsafe,center,start,end,self-start,\
	            self-end,flex-start,flex-end",
);

crate::create_property!(
	AlignSelf,
	display = "",
	atomic = "align-s",
	custom = false,
	data_type = "",
	initial_value = Normal,
	keywords = "auto,normal,stretch,first,last,baseline,safe,unsafe,center,start,end,self-start,\
	            self-end,flex-start,flex-end",
);

#[cfg(test)]
mod test {

	#[test]
	fn align_content() {
		let name = "align-content";
		crate::test_property_initial_value!(AlignContent, Normal);
		crate::test_global_keywords!(AlignContent, name);
		crate::test_function_var!(AlignContent, name);
		#[cfg(feature = "atomic")]
		crate::test_atomic_property!(AlignContent, "align-c");
	}

	#[test]
	fn align_items() {
		let name = "align-items";
		crate::test_property_initial_value!(AlignItems, Normal);
		crate::test_global_keywords!(AlignItems, name);
		crate::test_function_var!(AlignItems, name);
		#[cfg(feature = "atomic")]
		crate::test_atomic_property!(AlignItems, "align-i");
	}

	#[test]
	fn align_self() {
		let name = "align-self";
		crate::test_property_initial_value!(AlignSelf, Normal);
		crate::test_global_keywords!(AlignSelf, name);
		crate::test_function_var!(AlignSelf, name);
		#[cfg(feature = "atomic")]
		crate::test_atomic_property!(AlignSelf, "align-s");
	}
}
