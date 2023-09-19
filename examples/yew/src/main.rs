mod app;
mod styles_manager;

pub use app::*;
pub use styles_manager::*;

use gloo::utils::document;

pub const ID_ROOT_APP: &str = "app-root";

fn main() {
    wasm_logger::init(wasm_logger::Config::default());

    let root_element = document()
        .get_element_by_id(ID_ROOT_APP)
        .expect("Could not find the HTML element with the id: {ID_ROOT_APP} in the DOM tree");

    yew::Renderer::<App>::with_root(root_element).render();
}
