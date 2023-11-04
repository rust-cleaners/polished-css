#[macro_export]
macro_rules! test_atomic_property {
	($property:ident, $name:expr) => {
		::paste::paste! {
			assert_eq!(
				$crate::atomic::Atomic::atomic(
					&$crate::property::$property($crate::property::[< $property Value >]::Initial)
				),
				format!("{}[initial]", $name),
			);
		}
	};
}
