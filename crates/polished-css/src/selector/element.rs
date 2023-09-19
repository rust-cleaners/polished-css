//! [MDN](https://developer.mozilla.org/en-US/docs/Web/HTML/Element)

use super::SelectorDisplay;

#[derive(Clone, Debug, PartialEq, strum_macros::Display)]
#[strum(serialize_all = "lowercase")]
pub enum Element {
    /// [MDN](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/html)
    Html,

    // TODO: Verify if those are needed, AFAIK - not stylable
    // Document metadata

    // Sectioning root
    /// [MDN](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/body)
    Body,

    // Content sectioning
    Address,
    Article,
    Aside,
    Footer,
    Header,
    H1,
    H2,
    H3,
    H4,
    H5,
    H6,
    HGroup,
    Main,
    Nav,
    Section,
    Search,

    // Text content
    Blockquote,
    Dd,
    Div,
    Dl,
    Dt,
    Figcaption,
    Figure,
    Hr,
    Li,
    Menu,
    Ol,
    P,
    Pre,
    Ul,

    // Inline text semantics
    A,
    Abbr,
    B,
    Bdi,
    Bdo,
    Br,
    Cite,
    Code,
    Data,
    Dfn,
    Em,
    I,
    Kbd,
    Mark,
    Q,
    Rp,
    Rt,
    Ruby,
    S,
    Samp,
    Small,
    Span,
    Strong,
    Sub,
    Sup,
    Time,
    U,
    Var,
    Wbr,
    Area,
    Audio,
    Img,
    Map,
    Track,
    Video,

    // Embedded content - TODO: Verify if they're stylable at all.
    // Embed,
    // Iframe,
    // Object,
    // Picture,
    // Portal,
    // Source,

    // SVG and MathML
    Svg,
    Math,

    // Scripting - TODO: Verify if they're stylable at all.
    // Canvas,
    // Noscript,
    // Script,

    // Demarcating edits
    Del,
    Ins,

    // Table content
    Caption,
    Col,
    Colgroup,
    Table,
    TBody,
    Td,
    Tfoot,
    Th,
    Thead,
    Tr,

    // Forms
    Button,
    Datalist,
    Fieldset,
    Form,
    Input,
    Label,
    Legend,
    Meter,
    Optgroup,
    Option,
    Output,
    Progress,
    Select,
    Textarea,

    // Interactive elements
    Details,
    Dialog,
    Summary,

    // Web components
    Slot,
    Template,
}
impl SelectorDisplay for Element {
    fn as_attribute_value(&self) -> String {
        self.to_string()
    }

    fn as_styles_content(&self) -> String {
        self.to_string()
    }
}
