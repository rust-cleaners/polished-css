crate::create_property!(
	GridArea,
	display = "",
	atomic = "g-area",
	custom = false,
	data_type = "<string>",
	initial_value = Auto,
	keywords = "auto",
);

macro_rules! create_grid_auto_struct {
	($property:ident, $atomic:literal) => {
		$crate::create_property!(
			$property,
			display = "",
			atomic = $atomic,
			custom = false,
			data_type = "<length-percentage>",
			initial_value = Auto,
			keywords = "auto,fit-content,max-content,min-content",
		);
	};
}

create_grid_auto_struct!(GridAutoColumns, "g-auto-col");
create_grid_auto_struct!(GridAutoRows, "g-auto-row");

crate::create_property!(
	GridAutoFlow,
	display = "",
	atomic = "g-auto-flow",
	custom = false,
	data_type = "<length-percentage>",
	initial_value = Row,
	keywords = "row,column,dense",
);

macro_rules! create_grid_flow_struct {
	($property:ident, $atomic:literal) => {
		$crate::create_property!(
			$property,
			display = "",
			atomic = $atomic,
			custom = false,
			data_type = "<integer>",
			initial_value = Auto,
			keywords = "auto",
		);
	};
}

create_grid_flow_struct!(GridColumnStart, "grid-col-s");
create_grid_flow_struct!(GridColumnEnd, "grid-col-e");
create_grid_flow_struct!(GridRowStart, "grid-row-s");
create_grid_flow_struct!(GridRowEnd, "grid-row-e");

#[cfg(test)]
mod test {
	#[test]
	fn grid_area() {
		crate::test_property_initial_value!(GridArea, Auto);
		crate::test_global_keywords!(GridArea, "grid-area");
		crate::test_function_var!(GridArea, "grid-area");
		#[cfg(feature = "atomic")]
		crate::test_atomic_property!(GridArea, "g-area");
	}

	// TODO: Add missing test
}
