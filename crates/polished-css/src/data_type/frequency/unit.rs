//! [CSSWG specification](https://www.w3.org/TR/css-values-4/#frequency)

crate::create_unit!(Hz, "Hz", f64, Hertz);
crate::create_unit!(Khz, "kHz", f64, Kilohertz);

#[cfg(test)]
mod test {
    #[test]
    fn display() {
        assert_eq!(super::Hz(1.0).to_string(), String::from("1Hz"));
        assert_eq!(super::Khz(1.0).to_string(), String::from("1kHz"));
    }
}
