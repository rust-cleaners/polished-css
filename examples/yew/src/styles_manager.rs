use std::{
	cell::RefCell,
	collections::{HashMap, HashSet},
	convert::AsRef,
	rc::Rc,
};

use gloo::utils::document_element;
use polished_css::{
	atomic::{AtomicEvent, ATOMIC},
	prelude::Content,
};
use strum::IntoEnumIterator;
use yew::prelude::*;
use yewdux::prelude::*;

#[derive(
	Clone,
	Debug,
	Eq,
	PartialEq,
	Hash,
	strum_macros::AsRefStr,
	strum_macros::EnumIter,
	strum_macros::EnumString,
)]
#[strum(serialize_all = "kebab-case")]
pub enum Layer {
	Reset,
	DesignTokens,
	Atomic,
	Components,
}

pub type StylesContents = RefCell<HashMap<Layer, (HashSet<Content>, String)>>;

#[derive(Clone, Debug, Default, PartialEq, yewdux::store::Store)]
pub struct StateStyles(StylesContents);
impl StateStyles {
	/// # Panics
	///
	/// Panics if the layer name does not match any variant
	/// from the `Layer` enum.
	pub fn inject(&mut self, content: &Content, layer: &Layer) {
		let mut map = self.borrow_mut();
		match map.get_mut(layer) {
			None => {
				map.insert(layer.clone(), (HashSet::new(), String::new()));
			}
			Some((content_set, content_string)) => {
				if content_set.insert(content.clone()) {
					content_string.push_str(content);
				}
			}
		}
	}

	fn get_content(&self, layer: &Layer) -> AttrValue {
		let mut map = self.borrow_mut();
		if !map.contains_key(layer) {
			map.insert(layer.clone(), (HashSet::new(), String::new()));
		}
		let (_, content) = &map
			.get(layer)
			.expect("Unrecognized layer");
		AttrValue::from(format!("@layer {} {{{content}}}", layer.as_ref()))
	}
}

impl std::ops::Deref for StateStyles {
	type Target = StylesContents;

	fn deref(&self) -> &Self::Target {
		&self.0
	}
}

impl std::ops::DerefMut for StateStyles {
	fn deref_mut(&mut self) -> &mut Self::Target {
		&mut self.0
	}
}

pub enum StylesManagerMsg {
	StylesInjected(Rc<StateStyles>),
}

pub struct StylesManager {
	_state_listener: Dispatch<StateStyles>,
	state: Rc<StateStyles>,
}
impl yew::Component for StylesManager {
	type Message = StylesManagerMsg;
	type Properties = ();

	fn create(ctx: &yew::Context<Self>) -> Self {
		let cb = ctx
			.link()
			.callback(Self::Message::StylesInjected);
		let dispatch = Dispatch::<StateStyles>::subscribe(cb);
		use std::str::FromStr;
		ATOMIC.subscribe(&AtomicEvent::Build, |content, layer| {
			Dispatch::<StateStyles>::new().reduce_mut(|styles| {
				styles.inject(
					content,
					&Layer::from_str(layer).expect("Invalid layer name"),
				);
			});
		});
		Self {
			state: dispatch.get(),
			_state_listener: dispatch,
		}
	}

	fn view(&self, _ctx: &yew::Context<Self>) -> Html {
		let head = document_element()
			.query_selector("head")
			.expect("Could not access the DOM tree.")
			.expect("Could not find the <head> element in the DOM tree");

		create_portal(
			Layer::iter()
				.map(|layer| {
					let id = AttrValue::from(format!("style-{}", layer.as_ref()));
					html! {
						<Styles {id}>
							{&*self.state.get_content(&layer)}
						</Styles>
					}
				})
				.collect::<Html>(),
			head,
		)
	}

	fn update(&mut self, _ctx: &yew::Context<Self>, msg: Self::Message) -> bool {
		match msg {
			Self::Message::StylesInjected(state) => {
				self.state = state;
				true
			}
		}
	}

	fn destroy(&mut self, _ctx: &yew::Context<Self>) {
		ATOMIC.unsubscribe(&AtomicEvent::Build, |_content, _layer| {
			// ... Whatever you might need to do on unsubscribtion
		});
	}
}

#[derive(Debug, PartialEq, yew::Properties)]
struct StylesProps {
	id: AttrValue,
	children: Children,
}

#[function_component(Styles)]
fn styles(props: &StylesProps) -> Html {
	let StylesProps { children, id, .. } = props;
	html! {
		<style {id}>{children.clone()}</style>
	}
}
