use crate::prelude::NumberStorage;

use super::{Number, Percentage};

#[derive(
    Clone,
    Debug,
    PartialEq,
    strum_macros::EnumIs,
    polished_css_macros::Display,
    polished_css_macros::DataTypeFromDataTypes,
)]
#[display(on_enum = true)]
pub enum Chroma {
    Number(Number),
    Percentage(Percentage),
    None,
}

impl From<f64> for Chroma {
    fn from(value: f64) -> Self {
        Self::number(value)
    }
}

#[polished_css_macros::create_trait_from_enum_impl()]
impl Chroma {
    // TODO: Add bounds to override number and percentage methods
    // number can be only between 0 and 0.4 (interesting case!)
    // percentage from 0% - 0 and 100% for 0.4

    // TODO: Add conversion methods?
}

#[cfg(test)]
mod test {
    #[test]
    fn display() {
        use crate::data_type::{NumberStorage, PercentageStorage};
        assert_eq!(super::Chroma::from(6.66).to_string(), String::from("6.66"));
        assert_eq!(
            super::Chroma::percentage(1.23).to_string(),
            String::from("1.23%")
        );
        assert_eq!(super::Chroma::number(0.4).to_string(), String::from("0.4"));
        assert_eq!(super::Chroma::None.to_string(), String::from("none"));
    }
}
