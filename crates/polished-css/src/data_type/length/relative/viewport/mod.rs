//! [CSSWG specification](https://www.w3.org/TR/css-values-4/#viewport-relative-lengths)

pub mod unit;

pub use unit::*;

/// [CSSWG specification](https://www.w3.org/TR/css-values-4/#viewport-relative-lengths)
#[derive(
    Clone,
    Debug,
    PartialEq,
    strum_macros::EnumIs,
    polished_css_macros::Display,
    polished_css_macros::DataTypeFromUnits,
)]
#[display(on_enum = true)]
pub enum ViewportLength {
    Height(Vh),
    SmallHeight(Svh),
    LargeHeight(Lvh),
    DynamicHeight(Dvh),
    Width(Vw),
    SmallWidth(Svw),
    LargeWidth(Lvw),
    DynamicWidth(Dvw),
    Inline(Vi),
    SmallInline(Svi),
    LargeInline(Lvi),
    DynamicInline(Dvi),
    Block(Vb),
    SmallBlock(Svb),
    LargeBlock(Lvb),
    DynamicBlock(Dvb),
    Min(Vmin),
    SmallMin(Svmin),
    LargeMin(Lvmin),
    DynamicMin(Dvmin),
    Max(Vmax),
    SmallMax(Svmax),
    LargeMax(Lvmax),
    DynamicMax(Dvmax),
}

#[polished_css_macros::create_trait_from_enum_impl()]
impl ViewportLength {
    // TODO: Add conversion methods?
}

#[cfg(test)]
mod test {
    #[test]
    fn display() {
        use crate::data_type::length::relative::viewport::unit::*;

        assert_eq!(
            super::ViewportLength::vh(1.0).to_string(),
            String::from("1vh")
        );
        assert_eq!(
            super::ViewportLength::svh(1.0).to_string(),
            String::from("1svh")
        );
        assert_eq!(
            super::ViewportLength::lvh(1.0).to_string(),
            String::from("1lvh")
        );
        assert_eq!(
            super::ViewportLength::dvh(1.0).to_string(),
            String::from("1dvh")
        );

        assert_eq!(
            super::ViewportLength::vw(1.0).to_string(),
            String::from("1vw")
        );
        assert_eq!(
            super::ViewportLength::svw(1.0).to_string(),
            String::from("1svw")
        );
        assert_eq!(
            super::ViewportLength::lvw(1.0).to_string(),
            String::from("1lvw")
        );
        assert_eq!(
            super::ViewportLength::dvw(1.0).to_string(),
            String::from("1dvw")
        );

        assert_eq!(
            super::ViewportLength::vi(1.0).to_string(),
            String::from("1vi")
        );
        assert_eq!(
            super::ViewportLength::svi(1.0).to_string(),
            String::from("1svi")
        );
        assert_eq!(
            super::ViewportLength::lvi(1.0).to_string(),
            String::from("1lvi")
        );
        assert_eq!(
            super::ViewportLength::dvi(1.0).to_string(),
            String::from("1dvi")
        );

        assert_eq!(
            super::ViewportLength::vb(1.0).to_string(),
            String::from("1vb")
        );
        assert_eq!(
            super::ViewportLength::svb(1.0).to_string(),
            String::from("1svb")
        );
        assert_eq!(
            super::ViewportLength::lvb(1.0).to_string(),
            String::from("1lvb")
        );
        assert_eq!(
            super::ViewportLength::dvb(1.0).to_string(),
            String::from("1dvb")
        );

        assert_eq!(
            super::ViewportLength::vmin(1.0).to_string(),
            String::from("1vmin")
        );
        assert_eq!(
            super::ViewportLength::svmin(1.0).to_string(),
            String::from("1svmin")
        );
        assert_eq!(
            super::ViewportLength::lvmin(1.0).to_string(),
            String::from("1lvmin")
        );
        assert_eq!(
            super::ViewportLength::dvmin(1.0).to_string(),
            String::from("1dvmin")
        );

        assert_eq!(
            super::ViewportLength::vmax(1.0).to_string(),
            String::from("1vmax")
        );
        assert_eq!(
            super::ViewportLength::svmax(1.0).to_string(),
            String::from("1svmax")
        );
        assert_eq!(
            super::ViewportLength::lvmax(1.0).to_string(),
            String::from("1lvmax")
        );
        assert_eq!(
            super::ViewportLength::dvmax(1.0).to_string(),
            String::from("1dvmax")
        );
    }
}
