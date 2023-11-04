pub mod list;
pub mod shape;
pub mod svg;

pub use list::*;
pub use shape::*;
pub use svg::*;

crate::create_property!(
	PointerEvents,
	display = "",
	atomic = "pointer-e",
	custom = false,
	data_type = "",
	initial_value = Auto,
	keywords = "auto,none",
);

crate::create_property!(
	UserSelect,
	display = "",
	atomic = "user-s",
	custom = false,
	data_type = "",
	initial_value = Auto,
	keywords = "auto,none,text,contain,all",
);

crate::create_property!(
	TouchAction,
	display = "",
	atomic = "touch",
	custom = false,
	data_type = "",
	initial_value = Auto,
	keywords = "auto,none,pan-x,pan-left,pan-right,pan-y,pan-up,pan-down,pinch-zoom,manipulation",
);

crate::create_property!(
	Visibility,
	display = "",
	atomic = "visibility",
	custom = false,
	data_type = "",
	initial_value = Visible,
	keywords = "visible,hidden,collapse",
);

#[cfg(test)]
mod test {
	#[test]
	fn pointer_events() {
		let name = "pointer-events";
		crate::test_property_initial_value!(PointerEvents, Auto);
		crate::test_global_keywords!(PointerEvents, name);
		crate::test_function_var!(PointerEvents, name);

		#[cfg(feature = "atomic")]
		crate::test_atomic_property!(PointerEvents, "pointer-e");
	}

	#[test]
	fn user_select() {
		let name = "user-select";
		crate::test_property_initial_value!(UserSelect, Auto);
		crate::test_global_keywords!(UserSelect, name);
		crate::test_function_var!(UserSelect, name);
		#[cfg(feature = "atomic")]
		crate::test_atomic_property!(UserSelect, "user-s");
	}

	#[test]
	fn visibilty() {
		let name = "visibility";
		crate::test_property_initial_value!(Visibility, Visible);
		crate::test_global_keywords!(Visibility, name);
		crate::test_function_var!(Visibility, name);
		#[cfg(feature = "atomic")]
		crate::test_atomic_property!(Visibility, "visibility");
	}
}
