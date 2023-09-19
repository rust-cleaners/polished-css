crate::create_property!(
    BackgroundColor,
    display = "",
    atomic = "bg-color",
    custom = false,
    data_type = "<color>",
    initial_value = CurrentColor,
    keywords = "currentColor",
);

#[cfg(test)]
mod test {
    #[test]
    fn background_color() {
        let name = "background-color";
        crate::test_property_initial_value!(BackgroundColor, CurrentColor);
        crate::test_global_keywords!(BackgroundColor, name);
        crate::test_function_var!(BackgroundColor, name);
        #[cfg(feature = "atomic")]
        crate::test_atomic_property!(BackgroundColor, "bg-color");
    }
}
