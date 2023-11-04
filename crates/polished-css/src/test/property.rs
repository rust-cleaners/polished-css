#[macro_export]
macro_rules! test_property_initial_value {
	($property:ident, $value:ident) => {{
		::paste::paste! {
			assert_eq!(
				super::$property::default(),
				super::$property(super::[< $property Value >]::$value),
			);
		}
	}};
}
