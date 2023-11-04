crate::create_property!(
	ScrollTimelineAxis,
	display = "",
	atomic = "scroll-t-a",
	custom = false,
	data_type = "",
	initial_value = Block,
	keywords = "block,inline,x,y",
);

crate::create_property!(
	ScrollTimelineName,
	display = "",
	atomic = "scroll-t-n",
	custom = false,
	data_type = "<dashed-ident>",
	initial_value = None,
	keywords = "none",
);

#[cfg(test)]
mod test {
	#[test]
	fn scroll_timeline_axis() {
		let name = "scroll-timeline-axis";
		crate::test_property_initial_value!(ScrollTimelineAxis, Block);
		crate::test_global_keywords!(ScrollTimelineAxis, name);
		crate::test_function_var!(ScrollTimelineAxis, name);
		#[cfg(feature = "atomic")]
		crate::test_atomic_property!(ScrollTimelineAxis, "scroll-t-a");
	}

	#[test]
	fn scroll_timeline_name() {
		let name = "scroll-timeline-name";
		crate::test_property_initial_value!(ScrollTimelineName, None);
		crate::test_global_keywords!(ScrollTimelineName, name);
		crate::test_function_var!(ScrollTimelineName, name);
		#[cfg(feature = "atomic")]
		crate::test_atomic_property!(ScrollTimelineName, "scroll-t-n");
	}
}
