#![feature(const_trait_impl)]
#![deny(
	clippy::all,
    // Groups
	clippy::cargo,
	clippy::complexity,
	clippy::correctness,
	clippy::nursery,
	clippy::pedantic,
	clippy::perf,
	clippy::suspicious,
    // Adjustments
	clippy::cast_possible_truncation,
	clippy::cast_precision_loss,
	clippy::unwrap_used,
	// missing_docs,
	missing_debug_implementations,
	rust_2018_idioms,
	unused_must_use,
	unused_lifetimes,
	unused_results
)]
#![warn(unused_tuple_struct_fields, unused_crate_dependencies)]
#![allow(
	clippy::derive_partial_eq_without_eq,
	clippy::module_name_repetitions,
	ambiguous_glob_reexports
)]

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
