crate::create_property!(
    FlexBasis,
    display = "",
    atomic = "flex-b",
    custom = false,
    data_type = "<length-percentage>",
    initial_value = Auto,
    keywords = "auto,max-content,min-content,fit-content",
);

crate::create_property!(
    FlexDirection,
    display = "",
    atomic = "flex-d",
    custom = false,
    data_type = "",
    initial_value = Row,
    keywords = "column,column-reverse,row,row-reverse",
);

// TODO: Add FlexFlow once there's an idea with tuples
// crate::create_property!(
//     FlexFlow,
//     display = "",
//     atomic = "flex-f",
//     custom = false,
//     data_type = "",
//     initial_value = Initial,
//     keywords = "",
// );

crate::create_property!(
    FlexGrow,
    display = "",
    atomic = "flex-g",
    custom = false,
    data_type = "<number>",
    initial_value = Initial,
    keywords = "",
);

crate::create_property!(
    FlexShrink,
    display = "",
    atomic = "flex-s",
    custom = false,
    data_type = "<number>",
    initial_value = Initial,
    keywords = "",
);

crate::create_property!(
    FlexWrap,
    display = "",
    atomic = "flex-w",
    custom = false,
    data_type = "<number>",
    initial_value = Initial,
    keywords = "wrap,wrap-reverse,nowrap",
);

#[cfg(test)]
mod test {
    #[test]
    fn flex_basis() {
        let name = "flex-basis";
        crate::test_property_initial_value!(FlexBasis, Auto);
        crate::test_global_keywords!(FlexBasis, name);
        crate::test_function_var!(FlexBasis, name);
        #[cfg(feature = "atomic")]
        crate::test_atomic_property!(FlexBasis, "flex-b");
    }

    #[test]
    fn flex_direction() {
        let name = "flex-direction";
        crate::test_property_initial_value!(FlexDirection, Row);
        crate::test_global_keywords!(FlexDirection, name);
        crate::test_function_var!(FlexDirection, name);
        #[cfg(feature = "atomic")]
        crate::test_atomic_property!(FlexDirection, "flex-d");
    }

    #[test]
    fn flex_grow() {
        let name = "flex-grow";
        crate::test_property_initial_value!(FlexGrow, Initial);
        crate::test_global_keywords!(FlexGrow, name);
        crate::test_function_var!(FlexGrow, name);
        #[cfg(feature = "atomic")]
        crate::test_atomic_property!(FlexGrow, "flex-g");
    }

    #[test]
    fn flex_shrink() {
        let name = "flex-shrink";
        crate::test_property_initial_value!(FlexShrink, Initial);
        crate::test_global_keywords!(FlexShrink, name);
        crate::test_function_var!(FlexShrink, name);
        #[cfg(feature = "atomic")]
        crate::test_atomic_property!(FlexShrink, "flex-s");
    }

    #[test]
    fn flex_wrap() {
        let name = "flex-wrap";
        crate::test_property_initial_value!(FlexWrap, Initial);
        crate::test_global_keywords!(FlexWrap, name);
        crate::test_function_var!(FlexWrap, name);
        #[cfg(feature = "atomic")]
        crate::test_atomic_property!(FlexWrap, "flex-w");
    }
}
