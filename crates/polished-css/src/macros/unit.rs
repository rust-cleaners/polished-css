#[macro_export]
macro_rules! create_unit {
    ($unit_ident:ident, $suffix:expr, $type:ty, $trait_ident:ident) => {
        ::paste::paste! {
            #[doc = "[CSSWG specification](https://www.w3.org/TR/css-values-4/#" $suffix ")"]
            #[derive(
                Clone,
                Debug,
                PartialEq,
                polished_css_macros::Deref,
                polished_css_macros::Display,
                polished_css_macros::UnitTrait,
            )]
            #[display(suffix = $suffix)]
            #[unit(trait_ident = $trait_ident)]
            pub struct $unit_ident(pub $type);
        }
    };
}
