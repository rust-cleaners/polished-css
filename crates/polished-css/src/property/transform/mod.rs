crate::create_property!(
	Perspective,
	display = "",
	atomic = "pers",
	custom = false,
	data_type = "<length>",
	initial_value = None,
	keywords = "none",
);

crate::create_property!(
	PerspectiveOrigin,
	display = "",
	atomic = "pers-o",
	custom = false,
	data_type = "<length-percentage>",
	initial_value = Initial,
	keywords = "left,center,right,top,bottom",
);

crate::create_property!(
	Rotate,
	display = "",
	atomic = "rotate",
	custom = false,
	data_type = "<angle>",
	initial_value = None,
	keywords = "none",
);

crate::create_property!(
	Scale,
	display = "",
	atomic = "scale",
	custom = false,
	data_type = "<percentage>",
	initial_value = None,
	keywords = "none",
);

crate::create_property!(
	Translate,
	display = "",
	atomic = "translate",
	custom = false,
	data_type = "<length-percentage>",
	initial_value = None,
	keywords = "none",
);

crate::create_property!(
	TransformBox,
	display = "",
	atomic = "transform-b",
	custom = false,
	data_type = "",
	initial_value = ViewBox,
	keywords = "content-box,border-box,fill-box,stroke-box,view-box",
);

crate::create_property!(
	TransformOrigin,
	display = "",
	atomic = "transform-o",
	custom = false,
	data_type = "<length-percentage>",
	initial_value = Initial,
	keywords = "left,center,right,top,bottom",
);

crate::create_property!(
	TransformStyle,
	display = "",
	atomic = "transform-s",
	custom = false,
	data_type = "",
	initial_value = Flat,
	keywords = "flat,preserve-3d",
);

#[cfg(test)]
mod test {
	#[test]
	fn perspective() {
		let name = "perspective";
		crate::test_property_initial_value!(Perspective, None);
		crate::test_global_keywords!(Perspective, name);
		crate::test_function_var!(Perspective, name);
		#[cfg(feature = "atomic")]
		crate::test_atomic_property!(Perspective, "pers");
	}

	#[test]
	fn perspective_origin() {
		let name = "perspective-origin";
		crate::test_property_initial_value!(PerspectiveOrigin, Initial);
		crate::test_global_keywords!(PerspectiveOrigin, name);
		crate::test_function_var!(PerspectiveOrigin, name);
		#[cfg(feature = "atomic")]
		crate::test_atomic_property!(PerspectiveOrigin, "pers-o");
	}

	#[test]
	fn rotate() {
		let name = "rotate";
		crate::test_property_initial_value!(Rotate, None);
		crate::test_global_keywords!(Rotate, name);
		crate::test_function_var!(Rotate, name);
		#[cfg(feature = "atomic")]
		crate::test_atomic_property!(Rotate, "rotate");
	}

	#[test]
	fn scale() {
		let name = "scale";
		crate::test_property_initial_value!(Scale, None);
		crate::test_global_keywords!(Scale, name);
		crate::test_function_var!(Scale, name);
		#[cfg(feature = "atomic")]
		crate::test_atomic_property!(Scale, "scale");
	}

	#[test]
	fn translate() {
		let name = "translate";
		crate::test_property_initial_value!(Translate, None);
		crate::test_global_keywords!(Translate, name);
		crate::test_function_var!(Translate, name);
		#[cfg(feature = "atomic")]
		crate::test_atomic_property!(Translate, "translate");
	}

	#[test]
	fn transform_box() {
		let name = "transform-box";
		crate::test_property_initial_value!(TransformBox, ViewBox);
		crate::test_global_keywords!(TransformBox, name);
		crate::test_function_var!(TransformBox, name);
		#[cfg(feature = "atomic")]
		crate::test_atomic_property!(TransformBox, "transform-b");
	}

	#[test]
	fn transform_origin() {
		let name = "transform-origin";
		crate::test_property_initial_value!(TransformOrigin, Initial);
		crate::test_global_keywords!(TransformOrigin, name);
		crate::test_function_var!(TransformOrigin, name);
		#[cfg(feature = "atomic")]
		crate::test_atomic_property!(TransformOrigin, "transform-o");
	}

	#[test]
	fn transform_style() {
		let name = "transform-style";
		crate::test_property_initial_value!(TransformStyle, Flat);
		crate::test_global_keywords!(TransformStyle, name);
		crate::test_function_var!(TransformStyle, name);
		#[cfg(feature = "atomic")]
		crate::test_atomic_property!(TransformStyle, "transform-s");
	}
}
