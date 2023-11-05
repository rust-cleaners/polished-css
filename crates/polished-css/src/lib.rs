#![feature(const_trait_impl)]
// #![deny(
// 	clippy::all,
//     // Groups
// 	clippy::cargo,
// 	clippy::complexity,
// 	clippy::correctness,
//     clippy::nursery,
// 	clippy::pedantic,
// 	clippy::perf,
// 	clippy::suspicious,
//     // Adjustments
// 	clippy::cast_possible_truncation,
// 	clippy::cast_precision_loss,
// 	clippy::panic,
// 	clippy::unwrap_used
// )]
// #![deny(rustdoc::missing_crate_level_docs)]
// // #![deny(missing_docs)]
// #![deny(missing_debug_implementations)]
// #![deny(rust_2018_idioms)]
// #![deny(unreachable_pub)]
// #![deny(unused_must_use)]
// #![deny(unused_results)]
// #![deny(unused_lifetimes)]
// #![warn(unused_crate_dependencies)]
// #![warn(unused_tuple_struct_fields)]
#![allow(clippy::module_name_repetitions)]
#![allow(ambiguous_glob_reexports)]

#[cfg(feature = "atomic")]
pub mod atomic;
pub mod data_type;
pub mod function;
mod macros;
pub mod prelude;
pub mod property;
pub mod selector;
pub mod style;
#[cfg(test)]
mod test;
pub mod utils;
