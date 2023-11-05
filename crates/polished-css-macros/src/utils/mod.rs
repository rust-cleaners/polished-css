mod color_function;
mod data_type;
mod r#enum;
mod path;
mod unit;

pub(crate) use color_function::*;
pub(crate) use data_type::*;
pub(crate) use path::*;
pub(crate) use r#enum::*;
pub(crate) use unit::*;

pub(crate) const PROPERTY_VALUE_SUFFIX: &str = "Value";
pub(crate) const DATA_TYPE_TRAIT_SUFFIX: &str = "Storage";
pub(crate) const DATA_TYPE_OPTIONAL_PREFIX: &str = "DataType";

pub(crate) fn has_syntax_reserved_keywords(name: &str) -> bool {
	["in", "static", "super", "unsafe"].contains(&name)
}

pub(crate) fn remove_angle_brackets(input: &str) -> String {
	input.replace(['<', '>'], "")
}
