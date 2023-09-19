//! [CSSWG specification](https://www.w3.org/TR/css-values-4/#relative-lengths)

pub mod container;
pub mod font;
pub mod viewport;

pub use container::*;
pub use font::*;
pub use viewport::*;

/// [CSSWG specification](https://www.w3.org/TR/css-values-4/#relative-lengths)
#[derive(
    Clone,
    Debug,
    PartialEq,
    strum_macros::EnumIs,
    polished_css_macros::Display,
    polished_css_macros::DataTypeFromDataTypes,
)]
#[display(on_enum = true)]
pub enum RelativeLength {
    Container(ContainerLength),
    Font(FontLength),
    Viewport(ViewportLength),
}

#[polished_css_macros::create_trait_from_enum_impl()]
impl RelativeLength {
    // TODO: Add conversion methods?
}

#[cfg(test)]
mod test {
    #[test]
    fn display() {
        use crate::data_type::length::relative::*;

        assert_eq!(
            super::RelativeLength::cqw(1.0).to_string(),
            String::from("1cqw")
        );
        assert_eq!(
            super::RelativeLength::em(1.0).to_string(),
            String::from("1em")
        );
        assert_eq!(
            super::RelativeLength::dvh(1.0).to_string(),
            String::from("1dvh")
        );
    }
}
