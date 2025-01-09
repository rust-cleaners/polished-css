use polished_css::prelude::*;
use yew::prelude::*;

use super::{Layer, StylesManager};

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <>
            <StylesManager />
            <Content />
        </>
    }
}

#[function_component(Content)]
fn content() -> Html {
    let div_classes = classes!(
        Width::dvw(100.0).atomic_to_layer(Layer::Reset.as_ref()),
        Height::dvh(100.0).atomic_to_layer(Layer::DesignTokens.as_ref()),
        Overflow::hidden(),
        ZIndex::integer(100),
    );

    let p_classes = classes!(
        TextTransform::capitalize(),
        FontWeight::number(700.0),
        LineHeight::normal(),
        WhiteSpace::nowrap(),
    );

    html! {
        <div class={div_classes}>
            <p class={p_classes}>{"Hello from Yew!"}</p>
        </div>
    }
}
