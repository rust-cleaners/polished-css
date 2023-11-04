crate::create_property!(
	ScrollSnapAlign,
	display = "",
	atomic = "scroll-s-a",
	custom = false,
	data_type = "",
	initial_value = None,
	keywords = "none,start,center,end",
);

crate::create_property!(
	ScrollSnapStop,
	display = "",
	atomic = "scroll-s-s",
	custom = false,
	data_type = "",
	initial_value = Normal,
	keywords = "normal,always",
);

crate::create_property!(
	ScrollSnapType,
	display = "",
	atomic = "scroll-s-t",
	custom = false,
	data_type = "",
	initial_value = None,
	keywords =
		"none,x,y,block,inline,both mandatory,x mandatory,y mandatory,x proximity,y proximity",
);

#[cfg(test)]
mod test {
	#[test]
	fn scroll_snap_align() {
		let name = "scroll-snap-align";
		crate::test_property_initial_value!(ScrollSnapAlign, None);
		crate::test_global_keywords!(ScrollSnapAlign, name);
		crate::test_function_var!(ScrollSnapAlign, name);
		#[cfg(feature = "atomic")]
		crate::test_atomic_property!(ScrollSnapAlign, "scroll-s-a");
	}

	#[test]
	fn scroll_snap_stop() {
		let name = "scroll-snap-stop";
		crate::test_property_initial_value!(ScrollSnapStop, Normal);
		crate::test_global_keywords!(ScrollSnapStop, name);
		crate::test_function_var!(ScrollSnapStop, name);
		#[cfg(feature = "atomic")]
		crate::test_atomic_property!(ScrollSnapStop, "scroll-s-s");
	}

	#[test]
	fn scroll_snap_type() {
		let name = "scroll-snap-type";
		crate::test_property_initial_value!(ScrollSnapType, None);
		crate::test_global_keywords!(ScrollSnapType, name);
		#[cfg(feature = "atomic")]
		crate::test_atomic_property!(ScrollSnapType, "scroll-s-t");
	}
}
