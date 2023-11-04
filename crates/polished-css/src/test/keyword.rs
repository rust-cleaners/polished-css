#[macro_export]
macro_rules! test_global_keywords {
	($property:ident, $name:expr) => {
		::paste::paste! {
			assert_eq!(
				$crate::property::Property::declaration(
					&$crate::property::$property(
						$crate::property::[< $property Value >]::Inherit
					)
				),
				format!("{}:inherit", $name)
			);
			assert_eq!(
				$crate::property::Property::declaration(
					&$crate::property::$property(
						$crate::property::[< $property Value >]::Initial
					)
				),
				format!("{}:initial", $name)
			);
			assert_eq!(
				$crate::property::Property::declaration(
					&$crate::property::$property(
						$crate::property::[< $property Value >]::Revert
					)
				),
				format!("{}:revert", $name)
			);
			assert_eq!(
				$crate::property::Property::declaration(
					&$crate::property::$property(
						$crate::property::[< $property Value >]::RevertLayer
					)
				),
				format!("{}:revert-layer", $name)
			);
			assert_eq!(
				$crate::property::Property::declaration(
					&$crate::property::$property(
						$crate::property::[< $property Value >]::Unset
					)
				),
				format!("{}:unset", $name)
			);
		}
	};
}
