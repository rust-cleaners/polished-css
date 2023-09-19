// TODO: Finish this feature - feel free to contribute

/// [CSSWG specification](https://drafts.csswg.org/css-color/#typedef-hex-color)
#[derive(Clone, Debug, PartialEq, polished_css_macros::Deref, polished_css_macros::Display)]
#[non_exhaustive]
pub struct HexColor(pub String);

pub trait HexColorStorage: From<HexColor> {
    #[must_use]
    fn hex(value: HexColor) -> Self
    where
        Self: Sized,
    {
        Self::from(value)
    }
}
