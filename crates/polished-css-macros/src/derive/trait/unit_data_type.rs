use proc_macro2::TokenStream;
use quote::quote;
use syn::DeriveInput;

pub(crate) fn impl_unit_data_type_container(ast: &DeriveInput) -> TokenStream {
	let struct_ident = &ast.ident;

	quote! {
		impl<T> crate::utils::UnitDataTypeContainer<T>
			for #struct_ident<T>
		where
			T: Clone + std::fmt::Debug + std::fmt::Display + PartialEq + crate::utils::UnitDataType<Self>
		{}
	}
}
