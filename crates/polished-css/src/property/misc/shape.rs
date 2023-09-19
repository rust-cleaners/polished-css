// TODO: Implement once <image> data type is ready
crate::create_property!(
    ShapeImageThreshold,
    display = "",
    atomic = "shape-i-t",
    custom = false,
    data_type = "<alpha>",
    initial_value = Initial,
    keywords = "",
);

crate::create_property!(
    ShapeMargin,
    display = "",
    atomic = "shape-m",
    custom = false,
    data_type = "<length-percentage>",
    initial_value = Initial,
    keywords = "",
);

crate::create_property!(
    ShapeOutside,
    display = "",
    atomic = "shape-o",
    custom = false,
    data_type = "",
    initial_value = None,
    keywords = "none,margin-box,border-box,padding-box,content-box",
);

#[cfg(test)]
mod test {
    #[test]
    fn shape_image_threshold() {
        let name = "shape-image-threshold";
        crate::test_property_initial_value!(ShapeImageThreshold, Initial);
        crate::test_global_keywords!(ShapeImageThreshold, name);
        crate::test_function_var!(ShapeImageThreshold, name);
        #[cfg(feature = "atomic")]
        crate::test_atomic_property!(ShapeImageThreshold, "shape-i-t");
    }

    #[test]
    fn shape_margin() {
        let name = "shape-margin";
        crate::test_property_initial_value!(ShapeMargin, Initial);
        crate::test_global_keywords!(ShapeMargin, name);
        crate::test_function_var!(ShapeMargin, name);
        #[cfg(feature = "atomic")]
        crate::test_atomic_property!(ShapeMargin, "shape-m");
    }

    #[test]
    fn shape_style_type() {
        let name = "shape-outside";
        crate::test_property_initial_value!(ShapeOutside, None);
        crate::test_global_keywords!(ShapeOutside, name);
        crate::test_function_var!(ShapeOutside, name);
        #[cfg(feature = "atomic")]
        crate::test_atomic_property!(ShapeOutside, "shape-o");
    }
}
