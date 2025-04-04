use convert_case::{Case, Casing};
use syn::{spanned::Spanned, Ident};

use crate::utils::DATA_TYPE_OPTIONAL_PREFIX;

use super::{remove_angle_brackets, ColorFunction, Unit, DATA_TYPE_TRAIT_SUFFIX};

// NOTE:
// I could not find an easier way to see if the trait exits.
// Hence why I created this enum to support it.
#[derive(
	Clone,
	Debug,
	Eq,
	PartialEq,
	Hash,
	strum_macros::Display,
	strum_macros::EnumIs,
	strum_macros::EnumString,
	strum_macros::AsRefStr,
)]
#[strum(serialize_all = "PascalCase")]
pub enum DataType {
	AbsoluteColor,
	AbsoluteColorFunction,
	AbsoluteLength,
	Alpha,
	Angle,
	Blackness,
	Blue,
	Chroma,
	Color,
	ContainerLength,
	CustomIdent,
	DashedIdent,
	Dimension,
	Flex,
	FontLength,
	Frequency,
	FrequencyPercentage,
	Green,
	HexColor,
	Hue,
	Integer,
	Length,
	LengthPercentage,
	Lightness,
	NamedColor,
	Number,
	NumberPercentage,
	Percentage,
	Ratio,
	Red,
	RelativeLength,
	Resolution,
	Saturation,
	String,
	SystemColor,
	Time,
	ViewportLength,
	Whiteness,
}

impl DataType {
	pub fn get_ident(&self) -> Ident {
		let name = if self.is_string() || self.is_flex() {
			format!("{}{}", DATA_TYPE_OPTIONAL_PREFIX, self.as_ref())
		} else {
			self.as_ref().to_string()
		}
		.to_case(Case::Pascal);
		Ident::new(&name, Spanned::span(&name))
	}

	pub fn get_enum_variant_ident(&self) -> Ident {
		let name = match self {
			Self::AbsoluteColor | Self::AbsoluteLength => "Absolute",
			Self::AbsoluteColorFunction => "Function",
			Self::HexColor => "Hex",
			Self::NamedColor => "Named",
			Self::SystemColor => "System",
			Self::RelativeLength => "Relative",
			Self::ContainerLength => "Container",
			Self::FontLength => "Font",
			Self::ViewportLength => "Viewport",
			_ => self.as_ref(),
		};
		Ident::new(name, Spanned::span(&name))
	}

	pub fn get_trait_ident(&self) -> Ident {
		let name = format!("{}{}", self.as_ref(), DATA_TYPE_TRAIT_SUFFIX).to_case(Case::Pascal);
		Ident::new(&name, Spanned::span(&name))
	}

	pub const fn get_dependant_data_types(&self) -> &[Self] {
		match self {
			Self::AbsoluteColor => {
				&[
					Self::HexColor,
					Self::AbsoluteColorFunction,
					Self::NamedColor,
				]
			}
			Self::Alpha
			| Self::Blackness
			| Self::Chroma
			| Self::Lightness
			| Self::NumberPercentage
			| Self::Blue
			| Self::Green
			| Self::Red
			| Self::Saturation
			| Self::Whiteness => &[Self::Number, Self::Percentage],
			Self::Color => &[Self::AbsoluteColor, Self::SystemColor],
			Self::Dimension => &[Self::Frequency, Self::Length, Self::Resolution, Self::Time],
			Self::FrequencyPercentage => &[Self::Frequency, Self::Percentage],
			Self::Hue => &[Self::Angle, Self::Number],
			Self::Length => &[Self::AbsoluteLength, Self::RelativeLength],
			Self::LengthPercentage => &[Self::Length, Self::Percentage],
			Self::RelativeLength => {
				&[
					Self::ContainerLength,
					Self::FontLength,
					Self::ViewportLength,
				]
			}
			_ => &[],
		}
	}

	pub const fn get_dependant_color_functions(&self) -> &[ColorFunction] {
		match self {
			Self::AbsoluteColorFunction => {
				&[ColorFunction::Hsl, ColorFunction::Oklch, ColorFunction::Rgb]
			}
			_ => &[],
		}
	}

	pub const fn get_dependant_units(&self) -> &[Unit] {
		match self {
			Self::AbsoluteLength => {
				&[
					Unit::Cm,
					Unit::Mm,
					Unit::Q,
					Unit::In,
					Unit::Pc,
					Unit::Pt,
					Unit::Px,
				]
			}
			Self::Angle => &[Unit::Deg, Unit::Rad, Unit::Grad, Unit::Turn],
			Self::ContainerLength => {
				&[
					Unit::Cqw,
					Unit::Cqh,
					Unit::Cqi,
					Unit::Cqb,
					Unit::Cqmin,
					Unit::Cqmax,
				]
			}
			Self::Flex => &[Unit::Fr],
			Self::FontLength => {
				&[
					Unit::Em,
					Unit::Rem,
					Unit::Ex,
					Unit::Rex,
					Unit::Cap,
					Unit::Rcap,
					Unit::Ch,
					Unit::Rch,
					Unit::Ic,
					Unit::Ric,
					Unit::Lh,
					Unit::Rlh,
				]
			}
			Self::Frequency => &[Unit::Hz, Unit::Khz],
			Self::Resolution => &[Unit::Dpi, Unit::Dpcm, Unit::Dppx],
			Self::Time => &[Unit::Ms, Unit::S],
			Self::ViewportLength => {
				&[
					Unit::Vh,
					Unit::Svh,
					Unit::Lvh,
					Unit::Dvh,
					Unit::Vw,
					Unit::Svw,
					Unit::Lvw,
					Unit::Dvw,
					Unit::Vi,
					Unit::Svi,
					Unit::Lvi,
					Unit::Dvi,
					Unit::Vb,
					Unit::Svb,
					Unit::Lvb,
					Unit::Dvb,
					Unit::Vmin,
					Unit::Svmin,
					Unit::Lvmin,
					Unit::Dvmin,
					Unit::Vmax,
					Unit::Svmax,
					Unit::Lvmax,
					Unit::Dvmax,
				]
			}
			_ => &[],
		}
	}

	pub fn get_from_name(name: &str) -> Self
	where
		Self: Sized,
	{
		let name = remove_angle_brackets(name).to_case(Case::Pascal);
		Self::try_from(name.as_str()).unwrap_or_else(|_| panic!("Unrecognized data type: {name}"))
	}

	pub fn get_from_ident(ident: &Ident) -> Self
	where
		Self: Sized,
	{
		let name = format!(
			"<{}>",
			ident
				.to_string()
				.to_case(Case::Kebab)
		);
		Self::get_from_name(&name)
	}
}
