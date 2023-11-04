crate::create_property!(
	AnimationComposition,
	display = "",
	atomic = "anim-c",
	custom = false,
	data_type = "",
	initial_value = Replace,
	keywords = "replace,add,accumulate",
);

crate::create_property!(
	AnimationDelay,
	display = "",
	atomic = "anim-del",
	custom = false,
	data_type = "<time>",
	initial_value = Initial,
	keywords = "",
);

crate::create_property!(
	AnimationDirection,
	display = "",
	atomic = "anim-dir",
	custom = false,
	data_type = "",
	initial_value = Normal,
	keywords = "normal,reverse,alternate,alternate-reverse",
);

crate::create_property!(
	AnimationDuration,
	display = "",
	atomic = "anim-dur",
	custom = false,
	data_type = "<time>",
	initial_value = Initial,
	keywords = "auto",
);

crate::create_property!(
	AnimationFillMode,
	display = "",
	atomic = "anim-f",
	custom = false,
	data_type = "",
	initial_value = None,
	keywords = "none,forwards,backwards,both",
);

crate::create_property!(
	AnimationIterationCount,
	display = "",
	atomic = "anim-i",
	custom = false,
	data_type = "<number>",
	initial_value = Initial,
	keywords = "infinite",
);

crate::create_property!(
	AnimationName,
	display = "",
	atomic = "anim-n",
	custom = false,
	data_type = "<custom-ident>",
	initial_value = None,
	keywords = "none",
);

crate::create_property!(
	AnimationPlayState,
	display = "",
	atomic = "anim-p",
	custom = false,
	data_type = "",
	initial_value = Running,
	keywords = "running,paused",
);

crate::create_property!(
	AnimationTimingFunction,
	display = "",
	atomic = "anim-fn",
	custom = false,
	data_type = "",
	initial_value = Ease,
	keywords = "linear,ease,ease-in,ease-out,ease-in-out,step-start,step-end,jump-start,jump-end,\
	            jump-none,jump-both,start,end",
);

#[cfg(test)]
mod test {
	#[test]
	fn animation_composition() {
		let name = "animation-composition";
		crate::test_property_initial_value!(AnimationComposition, Replace);
		crate::test_global_keywords!(AnimationComposition, name);
		crate::test_function_var!(AnimationComposition, name);
		#[cfg(feature = "atomic")]
		crate::test_atomic_property!(AnimationComposition, "anim-c");
	}

	#[test]
	fn animation_delay() {
		let name = "animation-delay";
		crate::test_property_initial_value!(AnimationDelay, Initial);
		crate::test_global_keywords!(AnimationDelay, name);
		crate::test_function_var!(AnimationDelay, name);
		#[cfg(feature = "atomic")]
		crate::test_atomic_property!(AnimationDelay, "anim-del");
	}

	#[test]
	fn animation_direction() {
		let name = "animation-direction";
		crate::test_property_initial_value!(AnimationDirection, Normal);
		crate::test_global_keywords!(AnimationDirection, name);
		crate::test_function_var!(AnimationDirection, name);
		#[cfg(feature = "atomic")]
		crate::test_atomic_property!(AnimationDirection, "anim-dir");
	}

	#[test]
	fn animation_duration() {
		let name = "animation-duration";
		crate::test_property_initial_value!(AnimationDuration, Initial);
		crate::test_global_keywords!(AnimationDuration, name);
		crate::test_function_var!(AnimationDuration, name);
		#[cfg(feature = "atomic")]
		crate::test_atomic_property!(AnimationDuration, "anim-dur");
	}

	#[test]
	fn animation_iteration_count() {
		let name = "animation-iteration-count";
		crate::test_property_initial_value!(AnimationIterationCount, Initial);
		crate::test_global_keywords!(AnimationIterationCount, name);
		crate::test_function_var!(AnimationIterationCount, name);
		#[cfg(feature = "atomic")]
		crate::test_atomic_property!(AnimationIterationCount, "anim-i");
	}

	#[test]
	fn animation_fill_mode() {
		let name = "animation-fill-mode";
		crate::test_property_initial_value!(AnimationFillMode, None);
		crate::test_global_keywords!(AnimationFillMode, name);
		crate::test_function_var!(AnimationFillMode, name);
		#[cfg(feature = "atomic")]
		crate::test_atomic_property!(AnimationFillMode, "anim-f");
	}

	#[test]
	fn animation_name() {
		let name = "animation-name";
		crate::test_property_initial_value!(AnimationName, None);
		crate::test_global_keywords!(AnimationName, name);
		crate::test_function_var!(AnimationName, name);
		#[cfg(feature = "atomic")]
		crate::test_atomic_property!(AnimationName, "anim-n");
	}

	#[test]
	fn animation_play_state() {
		let name = "animation-play-state";
		crate::test_property_initial_value!(AnimationPlayState, Running);
		crate::test_global_keywords!(AnimationPlayState, name);
		crate::test_function_var!(AnimationPlayState, name);
		#[cfg(feature = "atomic")]
		crate::test_atomic_property!(AnimationPlayState, "anim-p");
	}

	#[test]
	fn animation_timing_function() {
		let name = "animation-timing-function";
		crate::test_property_initial_value!(AnimationTimingFunction, Ease);
		crate::test_global_keywords!(AnimationTimingFunction, name);
		crate::test_function_var!(AnimationTimingFunction, name);
		#[cfg(feature = "atomic")]
		crate::test_atomic_property!(AnimationTimingFunction, "anim-fn");
	}
}
