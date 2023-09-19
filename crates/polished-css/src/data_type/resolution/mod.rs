pub mod unit;

pub use unit::*;

/// [MDN documentation](https://developer.mozilla.org/en-US/docs/Web/CSS/resolution)
#[derive(
    Clone,
    Debug,
    PartialEq,
    strum_macros::EnumIs,
    polished_css_macros::Display,
    polished_css_macros::DataTypeFromUnits,
)]
#[display(on_enum = true)]
pub enum Resolution {
    DotsPerInch(Dpi),
    DotsPerCentimeter(Dpcm),
    DotsPerPixel(Dppx),
}

#[polished_css_macros::create_trait_from_enum_impl(constructor_arg_type = Resolution)]
impl Resolution {
    // TODO: Add conversion methods
}

#[cfg(test)]
mod test {
    #[test]
    fn display() {
        use super::unit::*;
        assert_eq!(
            super::Resolution::dpi(1.0).to_string(),
            String::from("1dpi")
        );
        assert_eq!(
            super::Resolution::dppx(1.0).to_string(),
            String::from("1dppx")
        );
        assert_eq!(
            super::Resolution::dpcm(1.0).to_string(),
            String::from("1dpcm")
        );
    }
}
