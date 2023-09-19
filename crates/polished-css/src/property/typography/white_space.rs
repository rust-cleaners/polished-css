crate::create_property!(
    WhiteSpace,
    display = "",
    atomic = "wspace",
    custom = false,
    data_type = "",
    initial_value = Normal,
    keywords = "normal,pre,nowrap,pre-wrap,pre-line,break-spaces",
);

#[cfg(test)]
mod test {
    #[test]
    fn white_space() {
        let name = "white-space";
        crate::test_property_initial_value!(WhiteSpace, Normal);
        crate::test_global_keywords!(WhiteSpace, name);
        crate::test_function_var!(WhiteSpace, name);
        #[cfg(feature = "atomic")]
        crate::test_atomic_property!(WhiteSpace, "wspace");
    }
}
