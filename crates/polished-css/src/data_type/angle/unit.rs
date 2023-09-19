//! [CSSWG specification](https://www.w3.org/TR/css-values-4/#angles)

crate::create_unit!(Deg, "deg", f64, Degree);

crate::create_unit!(Rad, "rad", f64, Radiant);
crate::create_unit!(Grad, "grad", f64, Gradiant);

crate::create_unit!(Turn, "turn", f64, Turn);

#[cfg(test)]
mod test {
    #[test]
    fn display() {
        assert_eq!(super::Deg(1.0).to_string(), String::from("1deg"));

        assert_eq!(super::Rad(1.0).to_string(), String::from("1rad"));
        assert_eq!(super::Grad(1.0).to_string(), String::from("1grad"));

        assert_eq!(super::Turn(1.0).to_string(), String::from("1turn"));
    }
}
