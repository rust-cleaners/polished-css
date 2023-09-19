crate::create_property!(
    TextDecoration,
    display = "",
    atomic = "txt-d",
    custom = false,
    data_type = "",
    initial_value = None,
    keywords = "none",
);

crate::create_property!(
    TextDecorationColor,
    display = "",
    atomic = "txt-d-c",
    custom = false,
    data_type = "<color>",
    initial_value = CurrentColor,
    keywords = "currentColor",
);

crate::create_property!(
    TextDecorationLine,
    display = "",
    atomic = "txt-d-l",
    custom = false,
    data_type = "",
    initial_value = None,
    keywords = "none,underline,overline,line-through,blink",
);

crate::create_property!(
    TextDecorationSkipLink,
    display = "",
    atomic = "txt-d-sl",
    custom = false,
    data_type = "",
    initial_value = Auto,
    keywords = "none,auto,all",
);

crate::create_property!(
    TextDecorationStyle,
    display = "",
    atomic = "txt-d-s",
    custom = false,
    data_type = "",
    initial_value = Solid,
    keywords = "solid,double,dotted,dashed,wavy",
);

crate::create_property!(
    TextDecorationThickness,
    display = "",
    atomic = "txt-d-t",
    custom = false,
    data_type = "<length-percentage>",
    initial_value = Auto,
    keywords = "auto,from-front",
);

#[cfg(test)]
mod test {
    #[test]
    fn color() {
        let name = "text-decoration-color";
        crate::test_property_initial_value!(TextDecorationColor, CurrentColor);
        crate::test_global_keywords!(TextDecorationColor, name);
        crate::test_function_var!(TextDecorationColor, name);
        #[cfg(feature = "atomic")]
        crate::test_atomic_property!(TextDecorationColor, "txt-d-c");
    }

    #[test]
    fn line() {
        let name = "text-decoration-line";
        crate::test_property_initial_value!(TextDecorationLine, None);
        crate::test_global_keywords!(TextDecorationLine, name);
        crate::test_function_var!(TextDecorationLine, name);
        #[cfg(feature = "atomic")]
        crate::test_atomic_property!(TextDecorationLine, "txt-d-l");
    }

    #[test]
    fn skip_link() {
        let name = "text-decoration-skip-link";
        crate::test_property_initial_value!(TextDecorationSkipLink, Auto);
        crate::test_global_keywords!(TextDecorationSkipLink, name);
        crate::test_function_var!(TextDecorationSkipLink, name);
        #[cfg(feature = "atomic")]
        crate::test_atomic_property!(TextDecorationSkipLink, "txt-d-sl");
    }

    #[test]
    fn style() {
        let name = "text-decoration-style";
        crate::test_property_initial_value!(TextDecorationStyle, Solid);
        crate::test_global_keywords!(TextDecorationStyle, name);
        crate::test_function_var!(TextDecorationStyle, name);
        #[cfg(feature = "atomic")]
        crate::test_atomic_property!(TextDecorationStyle, "txt-d-s");
    }

    #[test]
    fn thickness() {
        let name = "text-decoration-thickness";
        crate::test_property_initial_value!(TextDecorationThickness, Auto);
        crate::test_global_keywords!(TextDecorationThickness, name);
        crate::test_function_var!(TextDecorationThickness, name);
        #[cfg(feature = "atomic")]
        crate::test_atomic_property!(TextDecorationThickness, "txt-d-t");
    }
}
