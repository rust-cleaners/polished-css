crate::create_property!(
    JustifyContent,
    display = "",
    atomic = "justify-c",
    custom = false,
    data_type = "",
    initial_value = Normal,
    keywords = "normal,left,right,space-around,space-between,space-evenly,stretch,safe,unsafe,\
                center,start,end,flex-start,flex-end",
);

crate::create_property!(
    JustifyItems,
    display = "",
    atomic = "justify-i",
    custom = false,
    data_type = "",
    initial_value = Normal,
    keywords = "legacy,normal,stretch,left,right,first,last,baseline,safe,unsafe,center,start,end,\
                self-start,self-end,flex-start,flex-end",
);

crate::create_property!(
    JustifySelf,
    display = "",
    atomic = "justify-s",
    custom = false,
    data_type = "",
    initial_value = Normal,
    keywords = "auto,normal,stretch,left,right,first,last,baseline,safe,unsafe,center,start,end,\
                self-start,self-end,flex-start,flex-end",
);

#[cfg(test)]
mod test {
    #[test]
    fn justify_content() {
        let name = "justify-content";
        crate::test_property_initial_value!(JustifyContent, Normal);
        crate::test_global_keywords!(JustifyContent, name);
        crate::test_function_var!(JustifyContent, name);
        #[cfg(feature = "atomic")]
        crate::test_atomic_property!(JustifyContent, "justify-c");
    }

    #[test]
    fn justify_items() {
        let name = "justify-items";
        crate::test_property_initial_value!(JustifyItems, Normal);
        crate::test_global_keywords!(JustifyItems, name);
        crate::test_function_var!(JustifyItems, name);
        #[cfg(feature = "atomic")]
        crate::test_atomic_property!(JustifyItems, "justify-i");
    }

    #[test]
    fn justify_self() {
        let name = "justify-self";
        crate::test_property_initial_value!(JustifySelf, Normal);
        crate::test_global_keywords!(JustifySelf, name);
        crate::test_function_var!(JustifySelf, name);
        #[cfg(feature = "atomic")]
        crate::test_atomic_property!(JustifySelf, "justify-s");
    }
}
