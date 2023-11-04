/// [CSSWG specification](https://drafts.csswg.org/css-color/#typedef-system-color)
#[derive(
	Clone,
	Debug,
	PartialEq,
	strum_macros::Display,
	strum_macros::EnumIs,
	polished_css_macros::TraitFromEnum,
)]
#[strum(serialize_all = "PascalCase")]
pub enum SystemColor {
	/// Background of application content or documents.
	Canvas,
	/// Text in application content or documents.
	CanvasText,
	/// Text in non-active, non-visited links.
	/// For light backgrounds, traditionally blue.
	LinkText,
	/// Text in visited links. For light backgrounds, traditionally purple.
	VisitedText,
	/// Text in active links. For light backgrounds, traditionally red.
	ActiveText,
	/// The face background color for push buttons.
	ButtonFace,
	/// Text on push buttons.
	ButtonText,
	/// The base border color for push buttons.
	ButtonBorder,
	/// Background of input fields.
	Field,
	// Text in input fields.
	FieldText,
	/// Background of selected text, for example from ::selection.
	Highlight,
	/// Text of selected text.
	HighlightText,
	/// Background of selected items, for example a selected checkbox.
	SelectedItem,
	/// Text of selected items.
	SelectedItemText,
	/// Background of text that has been specially marked
	/// _(such as by the HTML mark element)_.
	Mark,
	/// Text that has been specially marked
	/// _(such as by the HTML mark element)_.
	MarkText,
	/// Disabled text.
	/// _(Often, but not necessarily, gray.)_
	GrayText,
	/// Background of accented user interface controls.
	AccentColor,
	/// Text of accented user interface controls.
	AccentColorText,
}

#[cfg(test)]
mod test {
	#[test]
	fn display() {
		assert_eq!(
			super::SystemColor::ButtonFace.to_string(),
			String::from("ButtonFace")
		);
	}
}
