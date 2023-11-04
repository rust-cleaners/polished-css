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
	clippy::panic,
	clippy::unwrap_used
)]
#![deny(rustdoc::missing_crate_level_docs)]
// #![deny(missing_docs)]
#![deny(missing_debug_implementations)]
#![deny(rust_2018_idioms)]
#![deny(unreachable_pub)]
#![deny(unused_must_use)]
#![deny(unused_results)]
#![deny(unused_lifetimes)]
#![warn(unused_crate_dependencies)]
#![warn(unused_tuple_struct_fields)]
#![allow(clippy::module_name_repetitions)]
#![allow(ambiguous_glob_reexports)]

extern crate proc_macro;

mod attribute;
mod derive;
mod utils;

use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput, ItemImpl};

// Derive proc macros

/// Implements a Rust's std trait - `Default`.
///
/// ### Used on
///
/// - properties
///
/// ### Usage
///
/// It attempts to follow the [CSSWG specification defaulting](https://www.w3.org/TR/css-cascade-5/#defaulting).\
/// However due to possible differences between browsers, the [initial values
/// provided by MDN is used on this crate](https://developer.mozilla.org/en-US/docs/Web/CSS/initial_value).
#[proc_macro_derive(Default, attributes(default))]
pub fn default(input: TokenStream) -> TokenStream {
	let ast = parse_macro_input!(input as DeriveInput);
	derive::r#trait::impl_default(&ast).into()
}

/// Implements a Rust's std trait - `Deref`.
///
/// ### Used on
///
/// Any single tuple structs defined in `styles` crate
#[proc_macro_derive(Deref, attributes(data_type))]
pub fn deref(input: TokenStream) -> TokenStream {
	let ast = parse_macro_input!(input as DeriveInput);
	derive::r#trait::impl_deref(&ast).into()
}

/// Implements a Rust's std trait - `Display`.
///
/// ### Used on
///
/// Any single tuple structs defined in `styles` crate
///
/// ### Usage
///
/// It supports three cases:
///
/// 1. **single tuple struct** - uses `self.0`
/// 2. **enum unit variants** - each variant will displayed in `kebab-case`
/// 3. **enum tuple _(single)_ variants** - uses `self.0` on associated values
#[proc_macro_derive(Display, attributes(display))]
pub fn display(input: TokenStream) -> TokenStream {
	let ast = parse_macro_input!(input as DeriveInput);
	derive::r#trait::impl_display(&ast).into()
}

/// Implements a custom trait - `Atomic`
///
/// ### Used on
///
/// - properties
///
/// ### Purpose
///
/// It does two things:
/// 1. Implements a trait by generating `Style` with a single rule only _(following the [Atomic CSS
///    approcach](https://css-tricks.com/lets-define-exactly-atomic-css/)) - based on the property's name, and the value stored
/// 2. Improves the ergonomic of usage with `yew` _(under the feature flag)_ -
///    `Into<yew::Classes>`
#[proc_macro_derive(Atomic, attributes(atomic))]
pub fn atomic(input: TokenStream) -> TokenStream {
	let ast = parse_macro_input!(input as DeriveInput);
	derive::r#trait::impl_atomic(&ast).into()
}

/// Implements a custom trait - `UnitDataTypeContainer`
///
/// ### Used on
///
/// - properties
/// - data types
///
/// ### Purpose
///
/// Quickly implements a trait.
#[proc_macro_derive(UnitDataTypeContainer)]
pub fn unit_data_type_container(input: TokenStream) -> TokenStream {
	let ast = parse_macro_input!(input as DeriveInput);
	derive::r#trait::impl_unit_data_type_container(&ast).into()
}

/// Implements a custom trait - `Property`
///
/// ### Used on
///
/// - properties
///
/// ### Purpose
///
/// Implements a trait and other utilities, such as `Nothing` as value, CSS-wide
/// functions, etc.
#[proc_macro_derive(Property, attributes(property))]
pub fn property(input: TokenStream) -> TokenStream {
	let ast = parse_macro_input!(input as DeriveInput);
	derive::property::r#trait::impl_property(&ast).into()
}

/// Implements a custom trait - `PropertyName`
///
/// ### Used on
///
/// - properties
///
/// ### Purpose
///
/// Quickly implements a trait based on the property struct name in
/// `kebab-case` - is needed for the correct usage of `Display` trait.
#[proc_macro_derive(PropertyName, attributes(property))]
pub fn property_name(input: TokenStream) -> TokenStream {
	let ast = parse_macro_input!(input as DeriveInput);
	derive::property::r#trait::impl_property_name(&ast).into()
}

/// Creates an enum - `<Property>Value`
///
/// ### Used on
///
/// - properties
///
/// ### Purpose
///
/// Creates an enum with allowed data types and keywords for targeted property.
#[proc_macro_derive(PropertyValue, attributes(property))]
pub fn property_value(input: TokenStream) -> TokenStream {
	let ast = parse_macro_input!(input as DeriveInput);
	derive::property::r#enum::create_property_value_enum(&ast).into()
}

/// Adds an `impl` on the `<Property>Value`
///
/// ### Used on
///
/// - properties
///
/// ### Purpose
///
/// Creates an `impl` for the enum _(`<PropertyValue>Value`)_, with methods for
/// the keywords _(for the ergonomic usage)_ - if the property has any keywords.
/// Also, implements those units traits on data types
#[proc_macro_derive(PropertyImpl, attributes(property))]
pub fn property_impl(input: TokenStream) -> TokenStream {
	let ast = parse_macro_input!(input as DeriveInput);
	derive::property::r#enum::r#impl::create_property_impl(&ast).into()
}

/// Implements a Rust's std trait - `From`.
///
/// ### Used on
///
/// - properties
///
/// ### Usage
///
/// Based on property allowed data type(s), it implements this trait to set as
/// its unit data type value _(using `UnitDataTypeContainer`)_
#[proc_macro_derive(PropertyFromDataType, attributes(property))]
pub fn property_from_data_type(input: TokenStream) -> TokenStream {
	let ast = parse_macro_input!(input as DeriveInput);
	derive::property::r#trait::impl_from_data_type(&ast).into()
}

/// Creates a custom trait for CSS unit `<name>Value`
///
/// ### Used on
///
/// - units
///
/// ### Purpose
///
/// Quickly generates a trait with bounds and the "constructor" method, to use
/// on any data type that relies on this targeted unit.
#[proc_macro_derive(UnitTrait, attributes(unit))]
pub fn create_unit_trait(input: TokenStream) -> TokenStream {
	let ast = parse_macro_input!(input as DeriveInput);
	derive::unit::r#trait::create_trait(&ast).into()
}

/// Implements a Rust's std trait - `From`.
///
/// ### Used on
///
/// - data types
///
/// ### Usage
///
/// Based on data type allowed units from its own enum, it implements the `From`
/// trait from those units structs. And the custom trait `<unit>Storage` from
/// each variant.
#[proc_macro_derive(DataTypeFromUnits, attributes(data_type))]
pub fn data_type_from_units(input: TokenStream) -> TokenStream {
	let ast = parse_macro_input!(input as DeriveInput);
	derive::data_type::r#trait::impl_from_units_traits(&ast).into()
}

/// Implements a custom trait - `<data type name>Storage`.
///
/// ### Used on
///
/// - data types
///
/// ### Usage
///
/// Based on data type allowed units from its own enum,
/// it implements the `<data type name>Storage` trait from each variant.
#[proc_macro_derive(ImplDataTypeTraits, attributes(data_type))]
pub fn impl_data_type_traits(input: TokenStream) -> TokenStream {
	let ast = parse_macro_input!(input as DeriveInput);
	derive::data_type::r#trait::impl_data_type_traits(&ast).into()
}

/// Implements a Rust's std trait - `From`.
///
/// ### Used on
///
/// - data types
///
/// ### Purpose
///
/// Based on data type allowed data types from its own enum, it implements the
/// `From` trait from those units structs. And the trait for each variant with
/// associated value.
#[proc_macro_derive(DataTypeFromDataTypes, attributes(data_type))]
pub fn data_type_from_data_types(input: TokenStream) -> TokenStream {
	let ast = parse_macro_input!(input as DeriveInput);
	derive::data_type::r#trait::impl_from_data_types_traits(&ast).into()
}

/// Creates a custom trait from enum - `<name>Storage`.
///
/// ### Used on
///
/// - data types
///
/// ### Purpose
///
/// Based on **enum with field variants only**, creates a trait `<name>Storage`
/// with methods for each variant to construct self with targeted variant.
#[proc_macro_derive(TraitFromEnum, attributes(data_type))]
pub fn trait_from_enum(input: TokenStream) -> TokenStream {
	let ast = parse_macro_input!(input as DeriveInput);
	derive::data_type::r#trait::create_trait_from_enum(&ast).into()
}

/// Creates a custom trait `<DataType>Storage`
///
/// ### Used on
///
/// - data types
///
/// ### Purpose
///
/// Based on the provided data type `impl`:
///
/// - creates a **constructor** method, for the data types _(if is combined)_,
/// - implement methods from the targeted data types for the ergonomic usage
///   with properties
#[proc_macro_attribute]
pub fn create_trait_from_enum_impl(args: TokenStream, input: TokenStream) -> TokenStream {
	let unparsed_input = input.clone();
	let ast = parse_macro_input!(unparsed_input as ItemImpl);
	let trait_impl: TokenStream =
		attribute::data_type::create_trait_from_enum_impl(args.into(), &ast).into();
	[input, trait_impl]
		.into_iter()
		.collect()
}
