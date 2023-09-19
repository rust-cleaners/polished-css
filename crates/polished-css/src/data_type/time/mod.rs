pub mod unit;

pub use unit::*;

/// [MDN documentation](https://developer.mozilla.org/en-US/docs/Web/CSS/time)
#[derive(
    Clone,
    Debug,
    PartialEq,
    strum_macros::EnumIs,
    polished_css_macros::Display,
    polished_css_macros::DataTypeFromUnits,
)]
#[display(on_enum = true)]
pub enum Time {
    Millisecond(Ms),
    Second(S),
}

#[polished_css_macros::create_trait_from_enum_impl()]
impl Time {
    // TODO: Add conversion methods
}

#[cfg(test)]
mod test {
    #[test]
    fn display() {
        use super::unit::*;
        assert_eq!(super::Time::ms(250).to_string(), String::from("250ms"));
        assert_eq!(super::Time::s(1.0).to_string(), String::from("1s"));
    }
}
