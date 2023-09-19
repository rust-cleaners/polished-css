#[macro_export]
macro_rules! create_property {
    (
        $property:ident,display =
        $display:literal,atomic =
        $atomic:literal,custom =
        $custom:expr,data_type =
        $data_type:literal,initial_value =
        $initial_value:ident,keywords =
        $keywords:literal,
    ) => {
        ::paste::paste! {
            #[doc = "[MDN documentation](https://developer.mozilla.org/en-US/docs/Web/CSS/" $property:lower ")"]
            #[derive(
                Clone,
                Debug,
                PartialEq,
                polished_css_macros::Atomic,
                polished_css_macros::Default,
                polished_css_macros::Display,
                polished_css_macros::Property,
                polished_css_macros::PropertyName,
                polished_css_macros::PropertyValue,
                polished_css_macros::PropertyImpl,
                polished_css_macros::PropertyFromDataType,
                polished_css_macros::UnitDataTypeContainer,
            )]
            #[atomic(name = $atomic)]
            #[default(value = $initial_value)]
            #[property(custom = $custom, display = $display, data_type = $data_type, keywords = $keywords)]
            pub struct $property<T>(pub T)
            where
                T: Clone
                    + std::fmt::Debug
                    + std::fmt::Display
                    + PartialEq
                    + $crate::utils::UnitDataType<Self>;
            }
        }
}
