pub mod unit;

pub use unit::*;

/// [MDN documentation](https://developer.mozilla.org/en-US/docs/Web/CSS/flex_value)
/// [CSSWG specification](https://drafts.csswg.org/css-grid-2/#typedef-flex)
/// NOTE: Not to be confused with `flex` property!
#[derive(
    Clone,
    Debug,
    PartialEq,
    strum_macros::EnumIs,
    polished_css_macros::Display,
    polished_css_macros::DataTypeFromUnits,
)]
#[display(on_enum = true)]
pub enum FlexDataType {
    Frame(Fr),
}

pub trait FlexStorage: From<FlexDataType> + FrameStorage {
    #[must_use]
    fn flex(value: FlexDataType) -> Self
    where
        Self: Sized,
    {
        Self::from(value)
    }

    // TODO: Add bounds - frame cannot be zero
    // TODO: Add converion methods?
}

#[cfg(test)]
mod test {
    #[test]
    fn display() {
        use super::FrameStorage;
        assert_eq!(super::FlexDataType::fr(1).to_string(), String::from("1fr"));
    }
}
