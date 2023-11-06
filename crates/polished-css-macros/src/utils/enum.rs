use syn::{DataEnum, Fields, FieldsUnnamed, Ident, Variant};

use super::get_last_path_segment;

pub fn get_enum_variants(data_enum: &DataEnum) -> Vec<&Variant> {
	data_enum
		.variants
		.iter()
		.collect()
}

pub fn get_enum_variant_associated_value_type_ident(variant: &Variant) -> Option<&Ident> {
	if let Fields::Unnamed(FieldsUnnamed { unnamed, .. }) = &variant.fields {
		if unnamed.len() == 1 {
			if let Some(field) = unnamed.iter().next() {
				return get_last_path_segment(field);
			}
		}
	}
	None
}
