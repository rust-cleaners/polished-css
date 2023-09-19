//! FIXME: Handle shorthands

macro_rules! create_struct {
    ($property:ident, $atomic:expr) => {
        $crate::create_property!(
            $property,
            display = "",
            atomic = $atomic,
            custom = false,
            data_type = "<length-percentage>",
            initial_value = Auto,
            keywords = "auto",
        );
    };
}

create_struct!(ScrollPadding, "scroll-p");
create_struct!(ScrollPaddingBlock, "scroll-p-bl");
create_struct!(ScrollPaddingBlockStart, "scroll-p-bl-s");
create_struct!(ScrollPaddingBlockEnd, "scroll-p-bl-e");
create_struct!(ScrollPaddingInline, "scroll-p-in");
create_struct!(ScrollPaddingInlineStart, "scroll-p-in-s");
create_struct!(ScrollPaddingInlineEnd, "scroll-p-in-e");
create_struct!(ScrollPaddingTop, "scroll-p-t");
create_struct!(ScrollPaddingBottom, "scroll-p-b");
create_struct!(ScrollPaddingLeft, "scroll-p-l");
create_struct!(ScrollPaddingRight, "scroll-p-r");

#[cfg(test)]
mod test {
    #[test]
    fn paddings() {
        macro_rules! test_property {
            ($property:ident, $name:expr, $atomic:expr) => {
                crate::test_property_initial_value!($property, Auto);
                crate::test_global_keywords!($property, $name);
                crate::test_function_var!($property, $name);
                #[cfg(feature = "atomic")]
                crate::test_atomic_property!($property, $atomic);
            };
        }
        test_property!(ScrollPadding, "scroll-padding", "scroll-p");
        test_property!(ScrollPaddingBlock, "scroll-padding-block", "scroll-p-bl");
        test_property!(
            ScrollPaddingBlockStart,
            "scroll-padding-block-start",
            "scroll-p-bl-s"
        );
        test_property!(
            ScrollPaddingBlockEnd,
            "scroll-padding-block-end",
            "scroll-p-bl-e"
        );
        test_property!(ScrollPaddingInline, "scroll-padding-inline", "scroll-p-in");
        test_property!(
            ScrollPaddingInlineStart,
            "scroll-padding-inline-start",
            "scroll-p-in-s"
        );
        test_property!(
            ScrollPaddingInlineEnd,
            "scroll-padding-inline-end",
            "scroll-p-in-e"
        );
        test_property!(ScrollPaddingTop, "scroll-padding-top", "scroll-p-t");
        test_property!(ScrollPaddingBottom, "scroll-padding-bottom", "scroll-p-b");
        test_property!(ScrollPaddingLeft, "scroll-padding-left", "scroll-p-l");
        test_property!(ScrollPaddingRight, "scroll-padding-right", "scroll-p-r");
    }
}
