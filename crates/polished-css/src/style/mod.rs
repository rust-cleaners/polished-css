//! Specififies the look and feel of a targetted UI element(s).

use crate::{
    property::Property,
    selector::{pseudo::PseudoSelector, Selector, SelectorDisplay},
};

pub type Content = String;
pub type Declarations = Vec<String>;

#[derive(Clone, Debug, Default, PartialEq, typed_builder::TypedBuilder)]
pub struct Style {
    #[builder(default, setter(into))]
    pub selector: Selector,
    #[builder(default, setter(transform = |list: &[&impl Property]| properties_to_declarations(list)))]
    pub declarations: Declarations,
    #[builder(default, setter(strip_option))]
    pub pseudo: Option<PseudoSelector>,
    // TODO: Add media queries
    // TODO: Add container queries
}

fn properties_to_declarations(properties: &[&impl Property]) -> Declarations {
    properties
        .iter()
        .map(|property| property.declaration())
        .collect()
}

impl Style {
    #[must_use]
    pub fn content(&self) -> Content {
        let declarations = self.declarations.join(";");

        if self.selector.is_inlined() {
            declarations
        } else {
            let selector = self.selector.as_styles_content();
            format!("{selector}{{{declarations}}}")
        }
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn builder() {
        use crate::{
            data_type::{AlphaStorage, PercentageStorage},
            selector::{Class, Element, Id},
        };
        assert_eq!(
            super::Style::builder()
                .declarations(&[&crate::property::Opacity::invisible()])
                .build()
                .content(),
            String::from("opacity:0")
        );

        assert_eq!(
            super::Style::builder()
                .selector(Id::from("test"))
                .declarations(&[&crate::property::Opacity::visible()])
                .build()
                .content(),
            String::from("#test{opacity:1}")
        );

        assert_eq!(
            super::Style::builder()
                .selector(Element::Main)
                .declarations(&[&crate::property::Opacity::invisible()])
                .build()
                .content(),
            String::from("main{opacity:0}")
        );

        assert_eq!(
            super::Style::builder()
                .selector(Class::from("my-example-class"))
                .declarations(&[&crate::property::Opacity::percentage(100.0)])
                .build()
                .content(),
            String::from(".my-example-class{opacity:100%}")
        );
    }
}
