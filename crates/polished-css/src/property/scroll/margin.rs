//! FIXME: Handle shorthands

macro_rules! create_struct {
    ($property:ident, $atomic:expr) => {
        $crate::create_property!(
            $property,
            display = "",
            atomic = $atomic,
            custom = false,
            data_type = "<length>",
            initial_value = Initial,
            keywords = "",
        );
    };
}

create_struct!(ScrollMargin, "scroll-m");
create_struct!(ScrollMarginBlock, "scroll-m-bl");
create_struct!(ScrollMarginBlockStart, "scroll-m-bl-s");
create_struct!(ScrollMarginBlockEnd, "scroll-m-bl-e");
create_struct!(ScrollMarginInline, "scroll-m-in");
create_struct!(ScrollMarginInlineStart, "scroll-m-in-s");
create_struct!(ScrollMarginInlineEnd, "scroll-m-in-e");
create_struct!(ScrollMarginTop, "scroll-m-t");
create_struct!(ScrollMarginBottom, "scroll-m-b");
create_struct!(ScrollMarginLeft, "scroll-m-l");
create_struct!(ScrollMarginRight, "scroll-m-r");

#[cfg(test)]
mod test {
    #[test]
    fn scroll_margins() {
        macro_rules! test_property {
            ($property:ident, $name:expr, $atomic:expr) => {
                crate::test_property_initial_value!($property, Initial);
                crate::test_global_keywords!($property, $name);
                crate::test_function_var!($property, $name);
                #[cfg(feature = "atomic")]
                crate::test_atomic_property!($property, $atomic);
            };
        }
        test_property!(ScrollMargin, "scroll-margin", "scroll-m");
        test_property!(ScrollMarginBlock, "scroll-margin-block", "scroll-m-bl");
        test_property!(
            ScrollMarginBlockStart,
            "scroll-margin-block-start",
            "scroll-m-bl-s"
        );
        test_property!(
            ScrollMarginBlockEnd,
            "scroll-margin-block-end",
            "scroll-m-bl-e"
        );
        test_property!(ScrollMarginInline, "scroll-margin-inline", "scroll-m-in");
        test_property!(
            ScrollMarginInlineStart,
            "scroll-margin-inline-start",
            "scroll-m-in-s"
        );
        test_property!(
            ScrollMarginInlineEnd,
            "scroll-margin-inline-end",
            "scroll-m-in-e"
        );
        test_property!(ScrollMarginTop, "scroll-margin-top", "scroll-m-t");
        test_property!(ScrollMarginBottom, "scroll-margin-bottom", "scroll-m-b");
        test_property!(ScrollMarginLeft, "scroll-margin-left", "scroll-m-l");
        test_property!(ScrollMarginRight, "scroll-margin-right", "scroll-m-r");
    }
}
