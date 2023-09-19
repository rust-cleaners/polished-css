use super::{Length, Percentage};

/// [MDN documentation](https://developer.mozilla.org/en-US/docs/Web/CSS/length-percentage)
#[derive(
    Clone,
    Debug,
    PartialEq,
    strum_macros::EnumIs,
    polished_css_macros::Display,
    polished_css_macros::DataTypeFromDataTypes,
)]
#[display(on_enum = true)]
pub enum LengthPercentage {
    Length(Length),
    Percentage(Percentage),
}

#[polished_css_macros::create_trait_from_enum_impl()]
impl LengthPercentage {
    // TODO: Add conversion methods?
}

#[cfg(test)]
mod test {
    #[test]
    fn display() {
        use crate::prelude::*;
        assert_eq!(
            super::LengthPercentage::px(1).to_string(),
            String::from("1px")
        );
        assert_eq!(
            super::LengthPercentage::cqw(1.0).to_string(),
            String::from("1cqw")
        );
        assert_eq!(
            super::LengthPercentage::rem(1.0).to_string(),
            String::from("1rem")
        );
        assert_eq!(
            super::LengthPercentage::vw(1.0).to_string(),
            String::from("1vw")
        );

        assert_eq!(
            super::LengthPercentage::percentage(1.0).to_string(),
            String::from("1%")
        );
    }
}
