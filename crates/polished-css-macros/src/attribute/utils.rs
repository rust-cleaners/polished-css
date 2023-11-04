use syn::{ItemImpl, Type};

use crate::utils::DataType;

pub fn get_data_type_from_item_impl(ast: &ItemImpl) -> DataType {
	let ident = match &*ast.self_ty {
		Type::Path(type_path) => {
			type_path
				.path
				.segments
				.last()
				.expect("Failed to get the last segment from type path")
				.ident
				.clone()
		}
		_ => panic!("Failed to access data type `impl` ident"),
	};
	DataType::get_from_ident(&ident)
}
