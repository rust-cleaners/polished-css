//! [CSSWG specification](https://www.w3.org/TR/css-values-4/#resolution)

crate::create_unit!(Dpi, "dpi", f64, DotsPerInch);
crate::create_unit!(Dpcm, "dpcm", f64, DotsPerCentimeter);
crate::create_unit!(Dppx, "dppx", f64, DotsPerPixel);

#[cfg(test)]
mod test {
    #[test]
    fn display() {
        assert_eq!(super::Dpi(1.0).to_string(), String::from("1dpi"));
        assert_eq!(super::Dpcm(1.0).to_string(), String::from("1dpcm"));
        assert_eq!(super::Dppx(1.0).to_string(), String::from("1dppx"));
    }
}
