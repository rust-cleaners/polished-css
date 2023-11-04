//! Kurwa mac!
use std::{collections::HashMap, sync::RwLock};

use once_cell::sync::Lazy;

// Reference: https://refactoring.guru/design-patterns/observer/rust/example
#[derive(Debug)]
pub struct AtomicEventEmitter {
	observer: RwLock<AtomicObserver>,
}
impl AtomicEventEmitter {
	/// # Panics
	///
	/// Panics if fails to read the `RwLock` on the `AtomicEventEmitter`'
	/// observer.
	pub fn build(&self, content: &crate::style::Content, layer: &str) {
		self.observer
			.read()
			.expect("Failed to read the RwLock on AtomicEventEmitter")
			.notify(&AtomicEvent::Build, content, layer);
	}

	/// # Panics
	///
	/// Panics if event type is invalid - doesn't match any of `AtomicEvent`
	/// enum variants.
	pub fn subscribe(&self, event: &AtomicEvent, listener: AtomicSubscriber) {
		self.observer
			.write()
			.expect("Couldn't get a lock on atomic observer: poisoned")
			.entry(event.clone())
			.or_default();
		self.observer
			.write()
			.expect("Couldn't get a lock on atomic observer: poisoned")
			.get_mut(event)
			.expect("Invalid atomic styles event type variant")
			.push(listener);
	}

	/// # Panics
	///
	/// Panics if event type is invalid - doesn't match any of `AtomicEvent`
	/// enum variants.
	pub fn unsubscribe(&self, event: &AtomicEvent, listener: AtomicSubscriber) {
		self.observer
			.write()
			.expect("Couldn't get a lock on atomic observer: poisoned")
			.get_mut(event)
			.expect("Invalid atomic styles event type variant")
			.retain(|&x| x != listener);
	}
}

// Reference: https://refactoring.guru/design-patterns/observer/rust/example
pub static ATOMIC: AtomicEventEmitter = AtomicEventEmitter {
	observer: RwLock::new(AtomicObserver(Lazy::new(HashMap::new))),
};

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub enum AtomicEvent {
	Build,
}

pub type AtomicSubscriber = fn(content: &crate::style::Content, layer: &str);

// Reference: https://refactoring.guru/design-patterns/observer/rust/example
#[derive(Debug, polished_css_macros::Deref)]
pub struct AtomicObserver(Lazy<HashMap<AtomicEvent, Vec<AtomicSubscriber>>>);
impl AtomicObserver {
	pub fn notify(&self, event: &AtomicEvent, content: &crate::style::Content, layer: &str) {
		if let Some(listeners) = self.get(event) {
			for listener in listeners {
				listener(content, layer);
			}
		}
	}
}
impl std::ops::DerefMut for AtomicObserver {
	fn deref_mut(&mut self) -> &mut Self::Target {
		&mut self.0
	}
}

pub trait Atomic: crate::property::Property {
	fn atomic_name<'a>(&self) -> &'a str;

	fn style(&self) -> crate::style::Style;

	fn content(&self) -> crate::style::Content {
		self.style().content()
	}

	fn class_name(&self) -> String {
		use crate::selector::SelectorDisplay;
		self.style()
			.selector
			.as_attribute_value()
	}

	fn atomic(&self) -> String {
		ATOMIC.build(&self.content(), "atomic");
		self.class_name()
	}

	fn atomic_to_layer(&self, layer: &str) -> String {
		ATOMIC.build(&self.content(), layer);
		self.class_name()
	}
}
