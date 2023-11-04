pub mod r#box;
pub mod dimension;
pub mod display;
pub mod flex;
pub mod gap;
pub mod grid;
pub mod inset;
pub mod margin;
pub mod padding;
pub mod placement;
pub mod position;

pub use dimension::*;
pub use display::*;
pub use flex::*;
pub use gap::*;
pub use grid::*;
pub use inset::*;
pub use margin::*;
pub use padding::*;
pub use placement::*;
pub use position::*;
pub use r#box::*;

crate::create_property!(
	AspectRatio,
	display = "",
	atomic = "ratio",
	custom = false,
	data_type = "<ratio>",
	initial_value = Auto,
	keywords = "auto",
);

crate::create_property!(
	VerticalAlign,
	display = "",
	atomic = "v-align",
	custom = false,
	data_type = "<length-percentage>",
	initial_value = Baseline,
	keywords = "baseline,sub,super,text-top,text-bottom,middle,top,bottom",
);

crate::create_property!(
	Resize,
	display = "",
	atomic = "resize",
	custom = false,
	data_type = "",
	initial_value = None,
	keywords = "none,both,horizontal,vertical,block,inline",
);

#[cfg(test)]
mod test {
	#[test]
	fn aspect_ratio() {
		let name = "aspect-ratio";
		crate::test_property_initial_value!(AspectRatio, Auto);
		crate::test_global_keywords!(AspectRatio, name);
		crate::test_function_var!(AspectRatio, name);
		#[cfg(feature = "atomic")]
		crate::test_atomic_property!(AspectRatio, "ratio");
	}

	#[test]
	fn resize() {
		let name = "resize";
		crate::test_property_initial_value!(Resize, None);
		crate::test_global_keywords!(Resize, name);
		crate::test_function_var!(Resize, name);
		#[cfg(feature = "atomic")]
		crate::test_atomic_property!(Resize, "resize");
	}

	#[test]
	fn vertical_align() {
		let name = "vertical-align";
		crate::test_property_initial_value!(VerticalAlign, Baseline);
		crate::test_global_keywords!(VerticalAlign, name);
		crate::test_function_var!(VerticalAlign, name);
		#[cfg(feature = "atomic")]
		crate::test_atomic_property!(VerticalAlign, "v-align");
	}
}
