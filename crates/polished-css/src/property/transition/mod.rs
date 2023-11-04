crate::create_property!(
	TransitionDelay,
	display = "",
	atomic = "trans-del",
	custom = false,
	data_type = "<time>",
	initial_value = Initial,
	keywords = "",
);

crate::create_property!(
	TransitionDuration,
	display = "",
	atomic = "trans-dur",
	custom = false,
	data_type = "<time>",
	initial_value = Initial,
	keywords = "",
);

crate::create_property!(
	TransitionProperty,
	display = "",
	atomic = "trans-p",
	custom = false,
	data_type = "<custom-ident>",
	initial_value = All,
	keywords = "none,all",
);

crate::create_property!(
	TransitionTimingFunction,
	display = "",
	atomic = "trans-fn",
	custom = false,
	data_type = "",
	initial_value = Ease,
	keywords = "linear,ease,ease-in,ease-out,ease-in-out,step-start,step-end,jump-start,jump-end,\
	            jump-none,jump-both,start,end",
);

#[cfg(test)]
mod test {
	#[test]
	fn transitiong_delay() {
		let name = "transition-delay";
		crate::test_property_initial_value!(TransitionDelay, Initial);
		crate::test_global_keywords!(TransitionDelay, name);
		crate::test_function_var!(TransitionDelay, name);
		#[cfg(feature = "atomic")]
		crate::test_atomic_property!(TransitionDelay, "trans-del");
	}

	#[test]
	fn transitiong_duration() {
		let name = "transition-duration";
		crate::test_property_initial_value!(TransitionDuration, Initial);
		crate::test_global_keywords!(TransitionDuration, name);
		crate::test_function_var!(TransitionDuration, name);
		#[cfg(feature = "atomic")]
		crate::test_atomic_property!(TransitionDuration, "trans-dur");
	}

	#[test]
	fn transition_property() {
		let name = "transition-property";
		crate::test_property_initial_value!(TransitionProperty, All);
		crate::test_global_keywords!(TransitionProperty, name);
		crate::test_function_var!(TransitionProperty, name);
		#[cfg(feature = "atomic")]
		crate::test_atomic_property!(TransitionProperty, "trans-p");
	}
}
