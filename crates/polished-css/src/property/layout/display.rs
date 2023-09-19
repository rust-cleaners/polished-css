crate::create_property!(
    Display,
    display = "",
    atomic = "dis",
    custom = false,
    data_type = "",
    initial_value = Inline,
    keywords = "block,inline,flex,inline-flex,grid,inline-grid,table,inline-table,none",
);

#[cfg(test)]
mod test {
    #[test]
    fn display() {
        let name = "display";
        crate::test_property_initial_value!(Display, Inline);
        crate::test_global_keywords!(Display, name);
        crate::test_function_var!(Display, name);
        #[cfg(feature = "atomic")]
        crate::test_atomic_property!(Display, "dis");
    }
}
