crate::create_property!(
    LineBreak,
    display = "",
    atomic = "line-b",
    custom = false,
    data_type = "<number>",
    initial_value = Auto,
    keywords = "auto,anywhere,normal,loose,strict",
);

crate::create_property!(
    LineHeight,
    display = "",
    atomic = "line-h",
    custom = false,
    data_type = "<number>",
    initial_value = Normal,
    keywords = "normal",
);

#[cfg(test)]
mod test {
    #[test]
    fn line_break() {
        let name = "line-break";
        crate::test_property_initial_value!(LineBreak, Auto);
        crate::test_global_keywords!(LineBreak, name);
        crate::test_function_var!(LineBreak, name);
        #[cfg(feature = "atomic")]
        crate::test_atomic_property!(LineBreak, "line-b");
    }

    #[test]
    fn line_height() {
        let name = "line-height";
        crate::test_property_initial_value!(LineHeight, Normal);
        crate::test_global_keywords!(LineHeight, name);
        crate::test_function_var!(LineHeight, name);
        #[cfg(feature = "atomic")]
        crate::test_atomic_property!(LineHeight, "line-h");
    }
}
