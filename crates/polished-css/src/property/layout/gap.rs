macro_rules! create_struct {
	($property:ident, $atomic:literal) => {
		$crate::create_property!(
			$property,
			display = "",
			atomic = $atomic,
			custom = false,
			data_type = "<length-percentage>",
			initial_value = Normal,
			keywords = "normal",
		);
	};
}

create_struct!(Gap, "gap");
create_struct!(RowGap, "row");
create_struct!(ColumnGap, "col");

#[cfg(test)]
mod test {
	#[allow(clippy::cognitive_complexity)]
	#[test]
	fn gaps() {
		macro_rules! test_property {
			($property:ident, $name:expr, $atomic:expr) => {
				crate::test_property_initial_value!($property, Normal);
				crate::test_global_keywords!($property, $name);
				crate::test_function_var!($property, $name);
				#[cfg(feature = "atomic")]
				crate::test_atomic_property!($property, $atomic);
			};
		}
		test_property!(Gap, "gap", "gap");
		test_property!(RowGap, "row-gap", "row");
		test_property!(ColumnGap, "column-gap", "col");
	}
}
