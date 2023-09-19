use crate::data_type::{Number, NumberStorage, Percentage};

#[derive(
    Clone,
    Debug,
    PartialEq,
    strum_macros::EnumIs,
    polished_css_macros::Display,
    polished_css_macros::DataTypeFromDataTypes,
)]
#[display(on_enum = true)]
pub enum Lightness {
    // TODO: Add bounds from 0.0 to 1!
    Number(Number),
    // TODO: Add bounds from 0% to 100%!
    Percentage(Percentage),
    None,
}

impl From<f64> for Lightness {
    fn from(value: f64) -> Self {
        Self::number(value)
    }
}

#[polished_css_macros::create_trait_from_enum_impl()]
impl Lightness {
    // TODO: Add conversion methods?
}

#[cfg(test)]
mod test {
    #[test]
    fn display() {
        use crate::data_type::*;
        assert_eq!(
            super::Lightness::percentage(1.23).to_string(),
            String::from("1.23%")
        );
        assert_eq!(
            super::Lightness::number(13.37).to_string(),
            String::from("13.37")
        );
        assert_eq!(super::Lightness::None.to_string(), String::from("none"));
    }
}
