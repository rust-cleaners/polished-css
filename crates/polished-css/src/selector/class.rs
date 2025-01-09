use once_cell::sync::OnceCell;
use regex::{Captures, Regex};

use super::SelectorDisplay;

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Class(pub String);

impl std::ops::Deref for Class {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl SelectorDisplay for Class {
    #[must_use]
    fn as_styles_content(&self) -> String {
        format!(".{}", escape_special_chars_in_class_name(&self.0))
    }

    #[must_use]
    fn as_attribute_value(&self) -> String {
        (self.0).to_string()
    }
}

impl From<&str> for Class {
    fn from(value: &str) -> Self {
        Self(value.to_string())
    }
}

static SPECIAL_CHAR_REGEX: OnceCell<Regex> = OnceCell::new();

/// # Panics
///
/// Panic when the Regex couldn't be created
#[must_use]
pub fn escape_special_chars_in_class_name(value: &str) -> String {
    let regex = SPECIAL_CHAR_REGEX.get_or_init(|| {
        Regex::new(r#"[!@#$%^&*()+\=\[\]{};':"\\|,.<>\\/?]"#)
            .expect("Failed to create a regex for matching special characters in CSS class name.")
    });
    regex
        .replace_all(value, |caps: &Captures<'_>| {
            caps.iter()
                .map(|char| char.map_or_else(String::new, |char| format!("\\{}", char.as_str())))
                .collect::<String>()
        })
        .to_string()
}
