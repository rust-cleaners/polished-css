#[macro_export]
macro_rules! test_function_var {
    ($property:ident, $name:expr) => {
        ::paste::paste! {
            // With fallback
            assert_eq!(
                $crate::property::Property::declaration(&$crate::property::$property(
                    $crate::function::Var::<$crate::property::[< $property Value >]> {
                        dashed_ident: "example-variable".into(),
                        fallback: Some($crate::property::[< $property Value >]::Initial),
                    }
                )),
                format!("{}:var(--example-variable,initial)", $name)
            );
            // Without fallback
            assert_eq!(
                $crate::property::Property::declaration(&$crate::property::$property(
                    $crate::function::Var::<$crate::utils::Nothing>::from("example-variable"))
                ),
                format!("{}:var(--example-variable)", $name)
            );
        }
    };
}
