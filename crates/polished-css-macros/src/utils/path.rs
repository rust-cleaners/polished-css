use syn::{Field, Ident, Type, TypePath};

pub fn get_last_path_segment(field: &Field) -> Option<&Ident> {
    if let Type::Path(TypePath { path, .. }) = &field.ty {
        if let Some(segment) = path.segments.last() {
            return Some(&segment.ident);
        }
    }
    None
}
