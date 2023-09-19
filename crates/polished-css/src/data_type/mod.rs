//! The common values and units that CSS properties accept.
//!
//! ### Resources
//!
//! - [CSSWG specification](https://drafts.csswg.org/css-values)
//! - [MDN documentation](https://developer.mozilla.org/en-US/docs/Web/CSS/CSS_Values_and_Units)

pub mod alpha;
pub mod angle;
pub mod chroma;
pub mod color;
pub mod custom_indent;
pub mod dashed_indent;
pub mod dimension;
pub mod flex;
pub mod frequency;
pub mod frequency_percentage;
pub mod hue;
pub mod integer;
pub mod length;
pub mod length_percentage;
pub mod lightness;
pub mod number;
pub mod percentage;
pub mod ratio;
pub mod resolution;
pub mod string;
pub mod time;

pub use alpha::*;
pub use angle::*;
pub use chroma::*;
pub use color::*;
pub use custom_indent::*;
pub use dashed_indent::*;
pub use dimension::*;
pub use flex::*;
pub use frequency::*;
pub use frequency_percentage::*;
pub use hue::*;
pub use integer::*;
pub use length::*;
pub use length_percentage::*;
pub use lightness::*;
pub use number::*;
pub use percentage::*;
pub use ratio::*;
pub use resolution::*;
pub use string::*;
pub use time::*;
