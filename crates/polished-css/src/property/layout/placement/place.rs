// TODO: Needs rethinking how to handle the tuple case.
// Idea: Probably with a trait to make it easier?

crate::create_property!(
	PlaceContent,
	display = "",
	atomic = "place-c",
	custom = false,
	data_type = "",
	initial_value = Normal,
	keywords = "normal,first,last,baseline,safe,unsafe,center,start,end,self-start,self-end,\
	            flex-start,flex-end",
);

crate::create_property!(
	PlaceItems,
	display = "",
	atomic = "place-i",
	custom = false,
	data_type = "",
	initial_value = Normal,
	keywords = "normal,first,last,baseline,safe,unsafe,center,start,end,self-start,self-end,\
	            flex-start,flex-end",
);

crate::create_property!(
	PlaceSelf,
	display = "",
	atomic = "place-s",
	custom = false,
	data_type = "",
	initial_value = Normal,
	keywords = "normal,first,last,baseline,safe,unsafe,center,start,end,self-start,self-end,\
	            flex-start,flex-end",
);

#[cfg(test)]
mod test {
	#[test]
	fn place_content() {
		let name = "place-content";
		crate::test_property_initial_value!(PlaceContent, Normal);
		crate::test_global_keywords!(PlaceContent, name);
		crate::test_function_var!(PlaceContent, name);
		#[cfg(feature = "atomic")]
		crate::test_atomic_property!(PlaceContent, "place-c");
	}

	#[test]
	fn place_items() {
		let name = "place-items";
		crate::test_property_initial_value!(PlaceItems, Normal);
		crate::test_global_keywords!(PlaceItems, name);
		crate::test_function_var!(PlaceItems, name);
		#[cfg(feature = "atomic")]
		crate::test_atomic_property!(PlaceItems, "place-i");
	}

	#[test]
	fn place_self() {
		let name = "place-self";
		crate::test_property_initial_value!(PlaceSelf, Normal);
		crate::test_global_keywords!(PlaceSelf, name);
		crate::test_function_var!(PlaceSelf, name);
		#[cfg(feature = "atomic")]
		crate::test_atomic_property!(PlaceSelf, "place-s");
	}
}
