use syn::{Data, DataEnum, DeriveInput, Fields, Type};

pub fn get_tuple_struct_field_type(ast: &DeriveInput) -> &Type {
	match &ast.data {
		Data::Struct(data) => {
			match &data.fields {
				Fields::Unnamed(fields) => {
					if fields.unnamed.len() == 1 {
						&fields.unnamed[0].ty
					} else {
						panic!("This is supposed to be tuple-struct with a single-field.");
					}
				}
				_ => {
					panic!("This struct doesn't have any unnamed fields!");
				}
			}
		}
		_ => {
			panic!("The input data is not a struct!");
		}
	}
}

pub fn get_data_enum(ast: &DeriveInput) -> &DataEnum {
	match &ast.data {
		Data::Enum(data) => data,
		_ => panic!("The provided data is not an enum"),
	}
}
