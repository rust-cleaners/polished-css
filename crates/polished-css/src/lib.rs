#![feature(const_trait_impl)]
#![deny(clippy::all)]
#![deny(clippy::complexity)]
#![deny(clippy::pedantic)]
#![deny(clippy::perf)]
#![deny(clippy::suspicious)]
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
