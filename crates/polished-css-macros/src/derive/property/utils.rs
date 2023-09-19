use convert_case::{Case, Casing};
use darling::FromDeriveInput;
use syn::{DeriveInput, Ident};

use crate::utils::{remove_angle_brackets, DataType, PROPERTY_VALUE_SUFFIX};

#[derive(darling::FromDeriveInput)]
#[darling(attributes(property), supports(struct_tuple))]
pub struct PropertyOptions {
    #[darling(default)]
    pub custom: bool,
    #[darling(default)]
    pub display: String,
    #[darling(default)]
    pub data_type: String,
    #[darling(default)]
    pub keywords: String,
}

pub fn get_property_options(ast: &DeriveInput) -> PropertyOptions {
    PropertyOptions::from_derive_input(ast)
        .expect("Couldn't parse correctly the proc macro derive attributes for Property.")
}

pub fn get_enum_property_value_ident(ast: &DeriveInput) -> Ident {
    Ident::new(
        &format!("{}{}", ast.ident, PROPERTY_VALUE_SUFFIX),
        ast.ident.span(),
    )
}

fn create_variant_ident(ast: &DeriveInput, name: &str) -> Ident {
    Ident::new(&name.to_case(Case::Pascal), ast.ident.span())
}

pub fn get_enum_variant_ident_for_data_type(ast: &DeriveInput, data_type: &DataType) -> Ident {
    create_variant_ident(ast, &remove_angle_brackets(data_type.as_ref()))
}

pub fn get_enum_variants_idents_for_keywords(
    ast: &DeriveInput,
    options: &PropertyOptions,
) -> Vec<Ident> {
    let PropertyOptions { keywords, .. } = options;

    [
        ["initial", "inherit", "revert", "revert-layer", "unset"]
            .iter()
            .map(|k| create_variant_ident(ast, k))
            .collect::<Vec<Ident>>(),
        if !keywords.is_empty() {
            keywords
                .split(',')
                .map(|keyword| create_variant_ident(ast, keyword))
                .collect()
        } else {
            Vec::default()
        },
    ]
    .concat()
}

pub fn get_enum_variants_idents_for_data_types(
    ast: &DeriveInput,
    options: &PropertyOptions,
) -> Vec<Ident> {
    let PropertyOptions { data_type, .. } = options;

    if !data_type.is_empty() {
        data_type
            .split(',')
            .map(|data_type| {
                let data_type = DataType::get_from_name(data_type);
                get_enum_variant_ident_for_data_type(ast, &data_type)
            })
            .collect()
    } else {
        Vec::default()
    }
}
