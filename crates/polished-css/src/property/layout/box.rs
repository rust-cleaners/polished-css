crate::create_property!(
    BoxSizing,
    display = "",
    atomic = "box-s",
    custom = false,
    data_type = "",
    initial_value = ContentBox,
    keywords = "border-box,content-box",
);

crate::create_property!(
    BoxShadowColor,
    display = "",
    atomic = "box-sh-c",
    custom = true,
    data_type = "<color>",
    initial_value = CurrentColor,
    keywords = "currentColor",
);

#[cfg(test)]
mod test {
    #[test]
    fn box_sizing() {
        let name = "box-sizing";
        crate::test_property_initial_value!(BoxSizing, ContentBox);
        crate::test_global_keywords!(BoxSizing, name);
        crate::test_function_var!(BoxSizing, name);
        #[cfg(feature = "atomic")]
        crate::test_atomic_property!(BoxSizing, "box-s");
    }

    #[test]
    fn box_shadow_color() {
        let name = "--box-shadow-color";
        crate::test_property_initial_value!(BoxShadowColor, CurrentColor);
        crate::test_global_keywords!(BoxShadowColor, name);
        crate::test_function_var!(BoxShadowColor, name);
        #[cfg(feature = "atomic")]
        crate::test_atomic_property!(BoxShadowColor, "box-sh-c");
    }
}
