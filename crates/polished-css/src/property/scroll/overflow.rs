//! FIXME: Handle shorthands

macro_rules! create_overflow_struct {
    ($property:ident, $atomic:expr) => {
        $crate::create_property!(
            $property,
            display = "",
            atomic = $atomic,
            custom = false,
            data_type = "",
            initial_value = Visible,
            keywords = "visible,hidden,clip,scroll,auto",
        );
    };
}

create_overflow_struct!(Overflow, "flow");
create_overflow_struct!(OverflowX, "flow-x");
create_overflow_struct!(OverflowY, "flow-y");
create_overflow_struct!(OverflowBlock, "flow-bl");
create_overflow_struct!(OverflowInline, "flow-in");

crate::create_property!(
    OverflowAnchor,
    display = "",
    atomic = "flow-a",
    custom = false,
    data_type = "",
    initial_value = Auto,
    keywords = "auto,none",
);

crate::create_property!(
    OverflowClipMargin,
    display = "",
    atomic = "flow-clip-m",
    custom = false,
    data_type = "<length>",
    initial_value = Initial,
    keywords = "content-box,padding-box,border-box",
);

crate::create_property!(
    OverflowWrap,
    display = "",
    atomic = "flow-w",
    custom = false,
    data_type = "",
    initial_value = Normal,
    keywords = "normal,break-word,anywhere",
);

#[cfg(test)]
mod test {
    #[test]
    fn overflows() {
        macro_rules! test_property {
            ($property:ident, $name:expr, $atomic:expr) => {
                crate::test_property_initial_value!($property, Visible);
                crate::test_global_keywords!($property, $name);
                crate::test_function_var!($property, $name);
                #[cfg(feature = "atomic")]
                crate::test_atomic_property!($property, $atomic);
            };
        }
        test_property!(Overflow, "overflow", "flow");
        test_property!(OverflowX, "overflow-x", "flow-x");
        test_property!(OverflowY, "overflow-y", "flow-y");
        test_property!(OverflowBlock, "overflow-block", "flow-bl");
        test_property!(OverflowInline, "overflow-inline", "flow-in");
    }

    #[test]
    fn overflow_anchor() {
        let name = "overflow-anchor";
        crate::test_property_initial_value!(OverflowAnchor, Auto);
        crate::test_global_keywords!(OverflowAnchor, name);
        crate::test_function_var!(OverflowAnchor, name);
        #[cfg(feature = "atomic")]
        crate::test_atomic_property!(OverflowAnchor, "flow-a");
    }

    #[test]
    fn overflow_clip_margin() {
        let name = "overflow-clip-margin";
        crate::test_property_initial_value!(OverflowClipMargin, Initial);
        crate::test_global_keywords!(OverflowClipMargin, name);
        crate::test_function_var!(OverflowClipMargin, name);
        #[cfg(feature = "atomic")]
        crate::test_atomic_property!(OverflowClipMargin, "flow-clip-m");
    }

    #[test]
    fn overflow_wrap() {
        let name = "overflow-wrap";
        crate::test_property_initial_value!(OverflowWrap, Normal);
        crate::test_global_keywords!(OverflowWrap, name);
        crate::test_function_var!(OverflowWrap, name);
        #[cfg(feature = "atomic")]
        crate::test_atomic_property!(OverflowWrap, "flow-w");
    }
}
