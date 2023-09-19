pub mod decoration;

pub use decoration::*;

crate::create_property!(
    TextAlign,
    display = "",
    atomic = "txt-a",
    custom = false,
    data_type = "",
    initial_value = Start,
    keywords = "start,justify,end,left,right,center,match-parent,justify-all",
);

crate::create_property!(
    TextAlignLast,
    display = "",
    atomic = "txt-a-l",
    custom = false,
    data_type = "",
    initial_value = Auto,
    keywords = "auto,start,justify,end,left,right,center,match-parent",
);

crate::create_property!(
    TextColor,
    display = "color",
    atomic = "txt-c",
    custom = false,
    data_type = "<color>",
    initial_value = CurrentColor,
    keywords = "currentColor",
);

crate::create_property!(
    TextShadowAlpha,
    display = "",
    atomic = "txt-sh-a",
    custom = true,
    data_type = "<alpha>",
    initial_value = Initial,
    keywords = "",
);

crate::create_property!(
    TextShadowColor,
    display = "",
    atomic = "txt-sh-c",
    custom = true,
    data_type = "<color>",
    initial_value = CurrentColor,
    keywords = "currentColor",
);

crate::create_property!(
    TextTransform,
    display = "",
    atomic = "txt-t",
    custom = false,
    data_type = "",
    initial_value = None,
    keywords = "capitalize,lowercase,uppercase,full-width,full-size-kana,none",
);

#[cfg(test)]
mod test {
    #[test]
    fn text_align() {
        let name = "text-align";
        crate::test_property_initial_value!(TextAlign, Start);
        crate::test_global_keywords!(TextAlign, name);
        crate::test_function_var!(TextAlign, name);
        #[cfg(feature = "atomic")]
        crate::test_atomic_property!(TextAlign, "txt-a");
    }

    #[test]
    fn text_color() {
        let name = "color";
        crate::test_property_initial_value!(TextColor, CurrentColor);
        crate::test_global_keywords!(TextColor, name);
        crate::test_function_var!(TextColor, name);
        #[cfg(feature = "atomic")]
        crate::test_atomic_property!(TextColor, "txt-c");
    }

    #[test]
    fn text_shadow_color() {
        let name = "--text-shadow-color";
        crate::test_property_initial_value!(TextShadowColor, CurrentColor);
        crate::test_global_keywords!(TextShadowColor, name);
        crate::test_function_var!(TextShadowColor, name);
        #[cfg(feature = "atomic")]
        crate::test_atomic_property!(TextShadowColor, "txt-sh-c");
    }

    #[test]
    fn text_shadow_opacity() {
        let name = "--text-shadow-alpha";
        crate::test_property_initial_value!(TextShadowAlpha, Initial);
        crate::test_global_keywords!(TextShadowAlpha, name);
        crate::test_function_var!(TextShadowAlpha, name);
        #[cfg(feature = "atomic")]
        crate::test_atomic_property!(TextShadowAlpha, "txt-sh-a");
    }

    #[test]
    fn text_transform() {
        let name = "text-transform";
        crate::test_property_initial_value!(TextTransform, None);
        crate::test_global_keywords!(TextTransform, name);
        crate::test_function_var!(TextTransform, name);
        #[cfg(feature = "atomic")]
        crate::test_atomic_property!(TextTransform, "txt-t");
    }
}
