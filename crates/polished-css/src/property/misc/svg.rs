crate::create_property!(
    Fill,
    display = "",
    atomic = "fill",
    custom = false,
    data_type = "<color>",
    initial_value = CurrentColor,
    keywords = "currentColor",
);

#[cfg(test)]
mod test {
    #[test]
    fn fill() {
        let name = "fill";
        crate::test_property_initial_value!(Fill, CurrentColor);
        crate::test_global_keywords!(Fill, name);
        crate::test_function_var!(Fill, name);
        #[cfg(feature = "atomic")]
        crate::test_atomic_property!(Fill, "fill");
    }
}
