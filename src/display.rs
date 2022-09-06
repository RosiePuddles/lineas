use std::fmt::{Formatter, Display, Debug};
use crate::{Matrix, polynomials::Polynomial};
use conv::{ConvUtil, ValueFrom};
use itertools::Itertools;

impl<const T: usize, const N: usize, L: Copy + Debug + Display> Display for Matrix<T, N, L> {
	/// Display [`Matrix`], [`Vector`], or [`ColVector`]
	///
	/// You can specify precision, width, and the presence of a sign before the elements
	///
	/// For [`ColVectors`] (or more specifically any `Matrix<T, 1, L>`) you can also specify the
	/// alternative display with the `#` flag. This will return the vector all on one line with a
	/// superscript T at the end to indicate a transposition.
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		fn fm<L>(q: &L, f: &&mut Formatter<'_>) -> String where L: Display {
			let p = f.precision().unwrap_or(0);
			let w = f.width().unwrap_or(0);
			if f.sign_plus() {
				format!("{:+w$.p$}", q)
			} else {
				format!("{:w$.p$}", q)
			}
		}
		if T == 0 || N == 0 {
			write!(f, "")
		} else if N == 1 && f.alternate() {
			write!(f, "|{}|ᵀ", self.0.iter().map(|t| fm(&t[0], &f)).collect::<Vec<String>>().join(", "))
		} else {
			write!(f, "{}", self.0.iter().map(|t| {
				format!("|{}|", t.iter().map(|l| fm(l, &f)).collect::<Vec<String>>().join(", "))
			}).collect::<Vec<String>>().join("\n"))
		}
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
