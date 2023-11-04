pub mod font;
pub mod line;
pub mod text;
pub mod white_space;
pub mod word;

pub use font::*;
pub use line::*;
pub use text::*;
pub use white_space::*;
pub use word::*;

crate::create_property!(
	Hyphens,
	display = "",
	atomic = "hyphens",
	custom = false,
	data_type = "",
	initial_value = None,
	keywords = "none,auto,manual",
);

crate::create_property!(
	Quotes,
	display = "",
	atomic = "quotes",
	custom = false,
	data_type = "<string>",
	initial_value = Initial,
	keywords = "auto,none",
);

crate::create_property!(
	TabSize,
	display = "",
	atomic = "tab",
	custom = false,
	data_type = "<length>",
	initial_value = Initial,
	keywords = "",
);

crate::create_property!(
	Widows,
	display = "",
	atomic = "widows",
	custom = false,
	data_type = "<integer>",
	initial_value = Initial,
	keywords = "",
);

crate::create_property!(
	WritingMode,
	display = "",
	atomic = "writing",
	custom = false,
	data_type = "",
	initial_value = HorizontalTb,
	keywords = "horizontal-tb,vertical-rl,vertical-lr,sideways-rl,sideways-lr",
);

#[cfg(test)]
mod test {
	#[test]
	fn hyphens() {
		let name = "hyphens";
		crate::test_property_initial_value!(Hyphens, None);
		crate::test_global_keywords!(Hyphens, name);
		crate::test_function_var!(Hyphens, name);
		#[cfg(feature = "atomic")]
		crate::test_atomic_property!(Hyphens, "hyphens");
	}

	#[test]
	fn tab_size() {
		let name = "tab-size";
		crate::test_property_initial_value!(TabSize, Initial);
		crate::test_global_keywords!(TabSize, name);
		crate::test_function_var!(TabSize, name);
		#[cfg(feature = "atomic")]
		crate::test_atomic_property!(TabSize, "tab");
	}

	#[test]
	fn widows() {
		let name = "widows";
		crate::test_property_initial_value!(Widows, Initial);
		crate::test_global_keywords!(Widows, name);
		crate::test_function_var!(Widows, name);
		#[cfg(feature = "atomic")]
		crate::test_atomic_property!(Widows, "widows");
	}

	#[test]
	fn writing_mode() {
		let name = "writing-mode";
		crate::test_property_initial_value!(WritingMode, HorizontalTb);
		crate::test_global_keywords!(WritingMode, name);
		crate::test_function_var!(WritingMode, name);
		#[cfg(feature = "atomic")]
		crate::test_atomic_property!(WritingMode, "writing");
	}
}
