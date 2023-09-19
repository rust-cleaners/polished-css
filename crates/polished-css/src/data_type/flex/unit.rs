//! [CSSWG specification](https://www.w3.org/TR/css-grid-2/#fr-unitx)

crate::create_unit!(Fr, "fr", isize, Frame);

#[cfg(test)]
mod test {
    #[test]
    fn display() {
        assert_eq!(super::Fr(1).to_string(), String::from("1fr"));
    }
}
