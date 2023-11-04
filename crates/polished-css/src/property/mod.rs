//! Characteristic _(like `color`)_ whose associated value defines one aspect of
//! how the UI engine should display the element.
//!
//! ### Resources
//!
//! - [CSSWG specification](https://www.w3.org/TR/CSS/#properties)
//! - [MDN documentation](https://developer.mozilla.org/en-US/docs/Glossary/Property/CSS)

pub mod appearance;
pub mod layout;
pub mod misc;
pub mod scroll;
pub mod transform;
pub mod transition;
pub mod typography;

pub use appearance::*;
pub use layout::*;
pub use misc::*;
pub use scroll::*;
pub use transform::*;
pub use transition::*;
pub use typography::*;

pub trait PropertyName {
	fn property_name<'a>(&self) -> &'a str;
}

/// Characteristic _(like `color`)_ whose associated value defines one aspect of
/// how the UI engine should display the element.
///
/// ### Resources
///
/// - [CSSWG specification](https://www.w3.org/TR/CSS/#properties)
/// - [MDN documentation](https://developer.mozilla.org/en-US/docs/Glossary/Property/CSS)
pub trait Property: std::fmt::Debug + std::fmt::Display + PropertyName {
	/// Returns the stringified property name with its value
	/// as a **CSS declaration** to be used within a declaration block.
	///
	/// ### Resources
	///
	/// - [CSSWG specification](https://drafts.csswg.org/css2/#declaration)
	/// - [MDN documentation](https://developer.mozilla.org/en-US/docs/Web/CSS/Syntax#css_declarations)
	fn declaration(&self) -> String {
		format!("{}:{}", &self.property_name(), &self)
	}
}

crate::create_property!(
	All,
	display = "",
	atomic = "all",
	custom = false,
	data_type = "",
	initial_value = Initial,
	keywords = "",
);

#[cfg(test)]
mod test {
	use crate::prelude::*;

	#[test]
	fn name() {
		assert_eq!(
			crate::property::Opacity::initial().property_name(),
			String::from("opacity")
		);
		assert_eq!(
			crate::property::BackgroundColor::initial().property_name(),
			String::from("background-color")
		);
	}

	#[test]
	fn rule() {
		assert_eq!(
			crate::property::Opacity::initial().declaration(),
			String::from("opacity:initial")
		);
		assert_eq!(
			crate::property::Opacity::percentage(0.0).declaration(),
			String::from("opacity:0%")
		);
	}

	#[test]
	fn property_with_data_type_alpha() {
		assert_eq!(
			Opacity::number(1.0),
			Opacity(OpacityValue::Alpha(Alpha::Number(Number(1.0)))),
		);
		assert_eq!(
			Opacity::percentage(100.0),
			Opacity(OpacityValue::Alpha(Alpha::Percentage(Percentage(100.0)))),
		);
	}

	#[test]
	fn property_with_data_type_angle() {
		assert_eq!(
			Rotate::deg(180.0),
			Rotate(RotateValue::Angle(Angle::Degree(Deg(180.0)))),
		);
		assert_eq!(
			Rotate::rad(100.0),
			Rotate(RotateValue::Angle(Angle::Radian(Rad(100.0)))),
		);
		assert_eq!(
			Rotate::grad(120.0),
			Rotate(RotateValue::Angle(Angle::Gradian(Grad(120.0)))),
		);
		assert_eq!(
			Rotate::turn(3.0),
			Rotate(RotateValue::Angle(Angle::Turn(Turn(3.0)))),
		);
	}

	// #[test]
	// fn property_with_data_type_chroma() {
	// TODO: Add est when property based on this data type is created
	// }

	#[test]
	fn property_with_data_type_color() {
		assert_eq!(
			TextColor::current_color(),
			TextColor(TextColorValue::CurrentColor),
		);
		assert_eq!(
			TextColor::transparent(),
			TextColor(TextColorValue::Color(Color::Absolute(
				AbsoluteColor::Transparent
			)))
		);
		assert_eq!(
			TextColor::oklch(Oklch::from((
				Lightness::percentage(10.0),
				Chroma::number(0.3),
				Hue::deg(225.0),
				Alpha::visible(),
			))),
			TextColor(TextColorValue::Color(Color::Absolute(
				AbsoluteColor::Function(AbsoluteColorFunction::Oklch(Oklch {
					lightness: Lightness::Percentage(Percentage(10.0)),
					chroma: Chroma::Number(Number(0.3)),
					hue: Hue::Angle(Angle::Degree(Deg(225.0))),
					alpha: Some(Alpha::Number(Number(1.0))),
				}))
			)))
		);
	}

	#[test]
	fn property_with_data_type_custom_ident() {
		assert_eq!(
			TransitionProperty::custom_ident("color"),
			TransitionProperty(TransitionPropertyValue::CustomIdent(CustomIdent::from(
				"color"
			))),
		);
		assert_eq!(
			Height::px(100),
			Height::length(Length::Absolute(AbsoluteLength::Pixel(Px(100)))),
		);
	}

	// #[test]
	// fn property_with_data_type_custom_ident() {
	//     // NOTE: Not applicable to properties (used with var)
	// }

	// #[test]
	// fn property_with_data_type_dimension() {
	//     // TODO: Add some test when property based on this data type is created
	// }

	// #[test]
	// fn property_with_data_type_flex() {
	//     // TODO: Add some test when property based on this data type is created
	// }

	// #[test]
	// fn property_with_data_type_frequency() {
	//     // TODO: Add some test when property based on this data type is created
	// }

	// #[test]
	// fn property_with_data_type_frequency_percentage() {
	//     // TODO: Add some test when property based on this data type is created
	// }

	// #[test]
	// fn property_with_data_type_hue() {
	//     // TODO: Add some test when property based on this data type is created
	// }

	#[test]
	fn property_with_data_type_integer() {
		assert_eq!(
			ZIndex::integer(100),
			ZIndex(ZIndexValue::Integer(Integer(100))),
		);
	}

	#[test]
	fn property_with_data_type_length() {
		assert_eq!(
			ScrollMarginBlockStart::px(100),
			ScrollMarginBlockStart(ScrollMarginBlockStartValue::Length(Length::Absolute(
				AbsoluteLength::Pixel(Px(100))
			))),
		);
		assert_eq!(
			ScrollMarginBlockStart::cqmax(100.0),
			ScrollMarginBlockStart(ScrollMarginBlockStartValue::Length(Length::Relative(
				RelativeLength::Container(ContainerLength::QueryMax(Cqmax(100.0)))
			))),
		);
		assert_eq!(
			ScrollMarginBlockStart::em(1.5),
			ScrollMarginBlockStart(ScrollMarginBlockStartValue::Length(Length::Relative(
				RelativeLength::Font(FontLength::Em(Em(1.5)))
			))),
		);
		assert_eq!(
			ScrollMarginBlockStart::vh(75.0),
			ScrollMarginBlockStart(ScrollMarginBlockStartValue::Length(Length::Relative(
				RelativeLength::Viewport(ViewportLength::Height(Vh(75.0)))
			))),
		);
	}

	#[test]
	fn property_with_data_type_length_percentage() {
		assert_eq!(
			Height::px(100),
			Height(HeightValue::LengthPercentage(LengthPercentage::Length(
				Length::Absolute(AbsoluteLength::Pixel(Px(100)))
			))),
		);
		assert_eq!(
			Height::cqmin(25.0),
			Height(HeightValue::LengthPercentage(LengthPercentage::Length(
				Length::Relative(RelativeLength::Container(ContainerLength::QueryMin(Cqmin(
					25.0
				))))
			)))
		);
		assert_eq!(
			Height::lh(1.0),
			Height(HeightValue::LengthPercentage(LengthPercentage::Length(
				Length::Relative(RelativeLength::Font(FontLength::LineHeight(Lh(1.0))))
			))),
		);
		assert_eq!(
			Height::dvw(50.0),
			Height(HeightValue::LengthPercentage(LengthPercentage::Length(
				Length::Relative(RelativeLength::Viewport(ViewportLength::DynamicWidth(Dvw(
					50.0
				))))
			))),
		);
		assert_eq!(
			Height::percentage(100.0),
			Height(HeightValue::LengthPercentage(LengthPercentage::Percentage(
				Percentage(100.0)
			))),
		);
	}

	// #[test]
	// fn property_with_data_type_lightness() {
	//     // TODO: Add some test when property based on this data type is created
	// }

	#[test]
	fn property_with_data_type_number() {
		assert_eq!(
			LineHeight::number(1.75),
			LineHeight(LineHeightValue::Number(Number(1.75))),
		);
	}

	#[test]
	fn property_with_data_type_percentage() {
		assert_eq!(
			Scale::reset(),
			Scale(ScaleValue::Percentage(Percentage(0.0))),
		);
	}

	#[test]
	fn property_with_data_type_ratio() {
		assert_eq!(
			AspectRatio::ratio(Ratio::from((4, 3))),
			AspectRatio(AspectRatioValue::Ratio(Ratio {
				a: 4.0,
				b: Some(3.0)
			})),
		);
	}

	// #[test]
	// fn property_with_data_type_resolution() {
	//     // TODO: Add some test when property based on this data type is created
	// }

	#[test]
	fn property_with_data_type_string() {
		assert_eq!(
			FontFamily::string("Arial"),
			FontFamily(FontFamilyValue::String(DataTypeString::from("Arial"))),
		);
	}

	#[test]
	fn property_with_data_type_time() {
		assert_eq!(
			TransitionDuration::ms(1_000),
			TransitionDuration(TransitionDurationValue::Time(Time::Millisecond(Ms(1_000)))),
		);
	}
}
