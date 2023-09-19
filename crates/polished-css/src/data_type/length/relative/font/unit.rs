//! [CSSWG specification](https://www.w3.org/TR/css-values-4/#font-relative-lengths)

crate::create_unit!(Em, "em", f64, Em);
crate::create_unit!(Rem, "rem", f64, RelativeEm);

crate::create_unit!(Ex, "ex", f64, Ex);
crate::create_unit!(Rex, "rex", f64, RelativeEx);

crate::create_unit!(Cap, "cap", f64, CapHeight);
crate::create_unit!(Rcap, "rcap", f64, RelativeCapHeight);

crate::create_unit!(Ch, "ch", isize, CharSize);
crate::create_unit!(Rch, "rch", isize, RelativeCharSize);

crate::create_unit!(Ic, "ic", isize, Ic);
crate::create_unit!(Ric, "ric", isize, RelativeIc);

crate::create_unit!(Lh, "lh", f64, LineHeight);
crate::create_unit!(Rlh, "rlh", f64, RelativeLineHeight);

#[cfg(test)]
mod test {
    #[test]
    fn display() {
        assert_eq!(super::Em(1.0).to_string(), String::from("1em"));
        assert_eq!(super::Rem(1.0).to_string(), String::from("1rem"));

        assert_eq!(super::Ex(1.0).to_string(), String::from("1ex"));
        assert_eq!(super::Rex(1.0).to_string(), String::from("1rex"));

        assert_eq!(super::Cap(1.0).to_string(), String::from("1cap"));
        assert_eq!(super::Rcap(1.0).to_string(), String::from("1rcap"));

        assert_eq!(super::Ch(1).to_string(), String::from("1ch"));
        assert_eq!(super::Rch(1).to_string(), String::from("1rch"));

        assert_eq!(super::Ic(1).to_string(), String::from("1ic"));
        assert_eq!(super::Ric(1).to_string(), String::from("1ric"));

        assert_eq!(super::Lh(1.0).to_string(), String::from("1lh"));
        assert_eq!(super::Rlh(1.0).to_string(), String::from("1rlh"));
    }
}
