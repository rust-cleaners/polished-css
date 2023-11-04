//! [CSSWG specification](https://www.w3.org/TR/css-contain-3/#container-lengths)

crate::create_unit!(Cqw, "cqw", f64, ContainerQueryWidth);
crate::create_unit!(Cqh, "cqh", f64, ContainerQueryHeight);

crate::create_unit!(Cqi, "cqi", f64, ContainerQueryInline);
crate::create_unit!(Cqb, "cqb", f64, ContainerQueryBlock);

crate::create_unit!(Cqmin, "cqmin", f64, ContainerQueryMin);
crate::create_unit!(Cqmax, "cqmax", f64, ContainerQueryMax);

#[cfg(test)]
mod test {
	#[test]
	fn display() {
		assert_eq!(super::Cqw(1.0).to_string(), String::from("1cqw"));
		assert_eq!(super::Cqh(1.0).to_string(), String::from("1cqh"));

		assert_eq!(super::Cqi(1.0).to_string(), String::from("1cqi"));
		assert_eq!(super::Cqb(1.0).to_string(), String::from("1cqb"));

		assert_eq!(super::Cqmin(1.0).to_string(), String::from("1cqmin"));
		assert_eq!(super::Cqmax(1.0).to_string(), String::from("1cqmax"));
	}
}
