use convert_case::{Case, Casing};
use syn::{spanned::Spanned, Ident};

use super::DATA_TYPE_TRAIT_SUFFIX;

// NOTE:
// I could not find an easier way to see if the trait exits.
// Hence why I created this enum to support it.
#[derive(
	strum_macros::Display, strum_macros::EnumIs, strum_macros::EnumString, strum_macros::AsRefStr,
)]
#[strum(serialize_all = "PascalCase")]
pub enum Unit {
	// Angle
	Deg,
	Rad,
	Grad,
	Turn,
	// Absolute length
	Cm,
	Mm,
	Q,
	In,
	Pc,
	Pt,
	Px,
	// Container length
	Cqw,
	Cqh,
	Cqi,
	Cqb,
	Cqmin,
	Cqmax,
	// Font length
	Em,
	Rem,
	Ex,
	Rex,
	Cap,
	Rcap,
	Ch,
	Rch,
	Ic,
	Ric,
	Lh,
	Rlh,
	// Viewport length
	Vh,
	Svh,
	Lvh,
	Dvh,
	Vw,
	Svw,
	Lvw,
	Dvw,
	Vb,
	Svb,
	Lvb,
	Dvb,
	Vi,
	Svi,
	Lvi,
	Dvi,
	Vmin,
	Svmin,
	Lvmin,
	Dvmin,
	Vmax,
	Svmax,
	Lvmax,
	Dvmax,
	// Frequency
	Hz,
	Khz,
	// Frame
	Fr,
	// Resolution
	Dpi,
	Dpcm,
	Dppx,
	// Time
	Ms,
	S,
}

impl Unit {
	pub const fn get_full_name(&self) -> &str {
		match self {
			// Angle
			Self::Deg => "Degree",
			Self::Rad => "Radiant",
			Self::Grad => "Gradiant",
			Self::Turn => "Turn",
			// Absolute length
			Self::Cm => "Centimeter",
			Self::Mm => "Millimeter",
			Self::Q => "QuarterOfMillimeter",
			Self::In => "Inch",
			Self::Pc => "Pica",
			Self::Pt => "Point",
			Self::Px => "Pixel",
			// Self::Px => "Container length
			Self::Cqw => "ContainerQueryWidth",
			Self::Cqh => "ContainerQueryHeight",
			Self::Cqi => "ContainerQueryInline",
			Self::Cqb => "ContainerQueryBlock",
			Self::Cqmin => "ContainerQueryMin",
			Self::Cqmax => "ContainerQueryMax",
			// Self::Px => "Font length
			Self::Em => "Em",
			Self::Rem => "RelativeEm",
			Self::Ex => "Ex",
			Self::Rex => "RelativeEx",
			Self::Cap => "CapHeight",
			Self::Rcap => "RelativeCapHeight",
			Self::Ch => "CharSize",
			Self::Rch => "RelativeCharSize",
			Self::Ic => "Ic",
			Self::Ric => "RelativeIc",
			Self::Lh => "LineHeight",
			Self::Rlh => "RelativeLineHeight",
			// Viewport length
			Self::Vh => "ViewportHeight",
			Self::Svh => "SmallViewportHeight",
			Self::Lvh => "LargeViewportHeight",
			Self::Dvh => "DynamicViewportHeight",
			Self::Vw => "ViewportWidth",
			Self::Svw => "SmallViewportWidth",
			Self::Lvw => "LargeViewportWidth",
			Self::Dvw => "DynamicViewportWidth",
			Self::Vi => "ViewportInline",
			Self::Svi => "SmallViewportInline",
			Self::Lvi => "LargeViewportInline",
			Self::Dvi => "DynamicViewportInline",
			Self::Vb => "ViewportBlock",
			Self::Svb => "SmallViewportBlock",
			Self::Lvb => "LargeViewportBlock",
			Self::Dvb => "DynamicViewportBlock",
			Self::Vmin => "ViewportMin",
			Self::Svmin => "SmallViewportMin",
			Self::Lvmin => "LargeViewportMin",
			Self::Dvmin => "DynamicViewportMin",
			Self::Vmax => "ViewportMax",
			Self::Svmax => "SmallViewportMax",
			Self::Lvmax => "LargeViewportMax",
			Self::Dvmax => "DynamicViewportMax",
			// Frame
			Self::Fr => "Frame",
			// Frequency
			Self::Hz => "Hertz",
			Self::Khz => "Kilohertz",
			// Resolution
			Self::Dpi => "DotsPerInch",
			Self::Dpcm => "DotsPerCentimeter",
			Self::Dppx => "DotsPerPixel",
			// Time
			Self::Ms => "Millisecond",
			Self::S => "Second",
		}
	}

	pub fn get_ident(&self) -> Ident {
		let name = self
			.to_string()
			.to_case(Case::Pascal);
		Ident::new(&name, Spanned::span(&name))
	}

	pub fn get_enum_variant_ident(&self) -> Ident {
		let name = match self {
			Self::Cqw => "QueryWidth",
			Self::Cqh => "QueryHeight",
			Self::Cqi => "QueryInline",
			Self::Cqb => "QueryBlock",
			Self::Cqmin => "QueryMin",
			Self::Cqmax => "QueryMax",
			Self::Vh => "Height",
			Self::Svh => "SmallHeight",
			Self::Lvh => "LargeHeight",
			Self::Dvh => "DynamicHeight",
			Self::Vw => "Width",
			Self::Svw => "SmallWidth",
			Self::Lvw => "LargeWidth",
			Self::Dvw => "DynamicWidth",
			Self::Vb => "Block",
			Self::Svb => "SmallBlock",
			Self::Lvb => "LargeBlock",
			Self::Dvb => "DynamicBlock",
			Self::Vi => "Inline",
			Self::Svi => "SmallInline",
			Self::Lvi => "LargeInline",
			Self::Dvi => "DynamicInline",
			Self::Vmin => "Min",
			Self::Svmin => "SmallMin",
			Self::Lvmin => "LargeMin",
			Self::Dvmin => "DynamicMin",
			Self::Vmax => "Max",
			Self::Svmax => "SmallMax",
			Self::Lvmax => "LargeMax",
			Self::Dvmax => "DynamicMax",
			_ => self.get_full_name(),
		};
		Ident::new(name, Spanned::span(&name))
	}

	pub fn get_trait_ident(&self) -> Ident {
		let name = format!("{}{}", self.get_full_name(), DATA_TYPE_TRAIT_SUFFIX);
		Ident::new(&name, Spanned::span(&name))
	}

	pub fn get_from_name(name: &str) -> Self
	where
		Self: Sized,
	{
		Self::try_from(
			name.to_case(Case::Pascal)
				.as_str(),
		)
		.unwrap_or_else(|_| panic!("Unrecognized unit name: {name}"))
	}

	pub fn get_from_ident(ident: &Ident) -> Self
	where
		Self: Sized,
	{
		let name = ident
			.to_string()
			.to_case(Case::Kebab);
		Self::get_from_name(&name)
	}
}
