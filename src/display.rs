use std::fmt::{Formatter, Display, Debug};
use crate::{Matrix, Vector, polynomials::Polynomial};
use conv::{ConvUtil, ValueFrom, ValueInto};
use itertools::Itertools;

impl<const T: usize, const N: usize, L: Copy + Debug + Display> Display for Matrix<T, N, L> {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		todo!()
	}
}

impl<L: Copy + Debug + Display> Display for Polynomial<L> where L: ValueFrom<isize> + PartialEq {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		write!(
			f, "{}",
			self.0.iter().enumerate().filter_map(|(v, c)| {
				if c == &0.value_as::<L>().unwrap() {
					None
				} else {
					Some(format!(
						"{}{}",
						if c == &1.value_as::<L>().unwrap() { "+".to_string() } else if c == &(-1).value_as::<L>().unwrap() { "-".to_string() } else {
							let e = f.precision().unwrap_or(0);
							format!("{:+.e$?}", c)
						},
						if v == self.0.len() - 1 { String::new() } else {
							if self.degree() - v == 1 { "x".to_string() } else {
								format!("x{}", format!("{}", self.degree() - v).chars().map(|t| {
									"⁰¹²³⁴⁵⁶⁷⁸⁹".chars().nth((t as u8) as usize - 48).unwrap().to_string()
								}).join(""))
							}
						}
					))
				}
			}).collect::<Vec<String>>().join("")
		)
	}
}
