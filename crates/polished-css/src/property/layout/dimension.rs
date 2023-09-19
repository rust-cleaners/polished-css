macro_rules! create_struct {
    ($property:ident, $atomic:literal) => {
        $crate::create_property!(
            $property,
            display = "",
            atomic = $atomic,
            custom = false,
            data_type = "<length-percentage>",
            initial_value = Initial,
            keywords = "auto,fit-content,max-content,min-content",
        );
    };
}

create_struct!(Width, "w");
// TODO: Remove the MaxWidth from macro - because it has keyword - none
create_struct!(MaxWidth, "max-w");
create_struct!(MinWidth, "min-w");

create_struct!(Height, "h");
// TODO: Remove the MaxHeight from macro - because it has keyword - none
create_struct!(MaxHeight, "max-h");
create_struct!(MinHeight, "min-h");

#[cfg(test)]
mod test {
    #[test]
    fn widths() {
        macro_rules! test_property {
            ($property:ident, $name:expr, $atomic:expr) => {
                crate::test_property_initial_value!($property, Initial);
                crate::test_global_keywords!($property, $name);
                crate::test_function_var!($property, $name);
                #[cfg(feature = "atomic")]
                crate::test_atomic_property!($property, $atomic);
            };
        }
        test_property!(Width, "width", "w");
        test_property!(MaxWidth, "max-width", "max-w");
        test_property!(MinWidth, "min-width", "min-w");
    }

    #[test]
    fn heights() {
        macro_rules! test_property {
            ($property:ident, $name:expr, $atomic:expr) => {
                crate::test_property_initial_value!($property, Initial);
                crate::test_global_keywords!($property, $name);
                crate::test_function_var!($property, $name);
                #[cfg(feature = "atomic")]
                crate::test_atomic_property!($property, $atomic);
            };
        }
        test_property!(Height, "height", "h");
        test_property!(MaxHeight, "max-height", "max-h");
        test_property!(MinHeight, "min-height", "min-h");
    }
}
