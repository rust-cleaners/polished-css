// FIXME: This is not a "data type", this is a keyword "placeholder"

/// [MDN documentation](https://developer.mozilla.org/en-US/docs/Web/CSS/absolute-size)
#[derive(Debug, strum_macros::Display, strum_macros::EnumIs)]
#[strum(serialize_all = "kebab-case")]
pub enum AbsoluteSize {
    /// An absolute size 60% the size of medium.
    XxSmall,
    /// An absolute size 75% the size of medium.
    XSmall,
    /// An absolute size 89% the size of medium.
    Small,
    /// A user's preferred font size.
    /// This value is used as the reference middle value.
    Medium,
    /// An absolute size 20% larger than medium.
    Large,
    /// An absolute size 50% larger than medium.
    XLarge,
    /// An absolute size twice the size of medium.
    XxLarge,
    /// An absolute size three times the size of medium.
    XxxLarge,
}
