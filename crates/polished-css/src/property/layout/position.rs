crate::create_property!(
    Position,
    display = "",
    atomic = "pos",
    custom = false,
    data_type = "",
    initial_value = Static,
    keywords = "static,relative,absolute,sticky,fixed",
);

crate::create_property!(
    ZIndex,
    display = "",
    atomic = "z",
    custom = false,
    data_type = "<integer>",
    initial_value = Initial,
    keywords = "",
);

#[cfg(test)]
mod test {
    #[test]
    fn position() {
        let name = "position";
        crate::test_property_initial_value!(Position, Static);
        crate::test_global_keywords!(Position, name);
        crate::test_function_var!(Position, name);
        #[cfg(feature = "atomic")]
        crate::test_atomic_property!(Position, "pos");
    }

    #[test]
    fn z_index() {
        let name = "z-index";
        crate::test_property_initial_value!(ZIndex, Initial);
        crate::test_global_keywords!(ZIndex, name);
        crate::test_function_var!(ZIndex, name);
        #[cfg(feature = "atomic")]
        crate::test_atomic_property!(ZIndex, "z");
    }
}
