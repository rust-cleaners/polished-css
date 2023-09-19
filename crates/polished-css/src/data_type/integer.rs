/// [MDN documentation](https://developer.mozilla.org/en-US/docs/Web/CSS/integer)
/// [CSSWG specification](https://drafts.csswg.org/css-values/#integer)
#[derive(Clone, PartialEq, Debug, polished_css_macros::Deref, polished_css_macros::Display)]
pub struct Integer(pub isize);

impl Integer {
    #[must_use]
    pub fn zero() -> Self {
        Self(0)
    }
}

pub trait IntegerStorage: From<Integer> {
    #[must_use]
    fn integer(value: isize) -> Self {
        Self::from(Integer(value))
    }

    #[must_use]
    fn zero() -> Self {
        Self::integer(0)
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn display() {
        assert_eq!(super::Integer(1).to_string(), String::from("1"));
        assert_eq!(super::Integer(-1).to_string(), String::from("-1"));
        assert_eq!(super::Integer::zero().to_string(), String::from("0"));
    }
}
