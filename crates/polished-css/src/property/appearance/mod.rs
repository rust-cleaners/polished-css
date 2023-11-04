pub mod animation;
pub mod background;
pub mod border;

pub use animation::*;
pub use background::*;
pub use border::*;

crate::create_property!(
	Appearance,
	display = "",
	atomic = "appearance",
	custom = false,
	data_type = "",
	initial_value = None,
	keywords = "none,auto",
);

crate::create_property!(
	Opacity,
	display = "",
	atomic = "op",
	custom = false,
	data_type = "<alpha>",
	initial_value = Initial,
	keywords = "",
);

#[cfg(test)]
mod test {
	#[test]
	fn appearance() {
		let name = "appearance";
		crate::test_property_initial_value!(Appearance, None);
		crate::test_global_keywords!(Appearance, name);
		crate::test_function_var!(Appearance, name);
		#[cfg(feature = "atomic")]
		crate::test_atomic_property!(Appearance, "appearance");
	}

	#[test]
	fn opacity() {
		let name = "opacity";
		crate::test_property_initial_value!(Opacity, Initial);
		crate::test_global_keywords!(Opacity, name);
		crate::test_function_var!(Opacity, name);
		#[cfg(feature = "atomic")]
		crate::test_atomic_property!(Opacity, "op");
	}
}
