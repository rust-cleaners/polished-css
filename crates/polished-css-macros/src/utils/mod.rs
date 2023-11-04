mod color_function;
mod data_type;
mod r#enum;
mod path;
mod unit;

pub use color_function::*;
pub use data_type::*;
pub use path::*;
pub use r#enum::*;
pub use unit::*;

pub const PROPERTY_VALUE_SUFFIX: &str = "Value";
pub const DATA_TYPE_TRAIT_SUFFIX: &str = "Storage";
pub const DATA_TYPE_OPTIONAL_PREFIX: &str = "DataType";

pub fn has_syntax_reserved_keywords(name: &str) -> bool {
	["in", "static", "super", "unsafe"].contains(&name)
}

pub fn remove_angle_brackets(input: &str) -> String {
	input.replace(['<', '>'], "")
}
