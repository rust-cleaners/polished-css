//! The styles **prelude** - alleviate imports of many common types.
//! ```
//! use styles::prelude::*;
//! ```
#[cfg(feature = "atomic")]
pub use crate::atomic::Atomic;
pub use crate::{data_type::*, function::*, property::*, selector::*, style::*, utils::*};
