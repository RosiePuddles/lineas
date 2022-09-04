use std::fmt::{Debug, Error};
use std::ops::{Add, Sub, AddAssign, SubAssign, Mul, MulAssign, Neg};
use conv::{ConvUtil, ValueFrom, ValueInto};
use crate::polynomials::Polynomial;

impl<L: Copy + Debug> ValueFrom<L> for Polynomial<L> {
	type Err = Error;
	
	fn value_from(src: L) -> Result<Self, Self::Err> {
		Ok(Polynomial::new(vec![src]))
	}
}

impl<L: Copy + Debug> Polynomial<L> {
	/// Make a new polynomial from a set of coefficients
	///
	/// The coefficients are in order of decreasing power. For more info see the
	/// [`Polynomial`][Polynomial] struct
	pub fn new(data: Vec<L>) -> Self where L: Sized {
		Self(data)
	}
	
	/// Change the data type of the polynomial coefficients.
	///
	/// This works the same as [`Matrix::dtype`][crate::Matrix::dtype]
	pub fn dtype<Q>(&self) -> Polynomial<Q> where Q: ValueFrom<L> + Copy + Debug {
		Polynomial(self.0.iter().map(|t| (*t).value_as::<Q>().unwrap()).collect())
	}
	
	/// Minify the polynomial coefficients by removing leading zeroes
	///
	/// ```
	/// # use lineas::polynomials::Polynomial;
	/// let example = Polynomial::new(vec![0, 0, 4, 2]);
	/// let minified = Polynomial::new(vec![4, 2]);
	/// assert_eq!(example.minify(), minified)
	/// ```
	pub fn minify(&self) -> Self where L: ValueFrom<isize> + PartialEq {
		let mut out = Vec::new();
		for i in self.0.iter() {
			if out.len() == 0 && i == &0.value_as::<L>().unwrap() {
				continue
			}
			out.push(*i)
		}
		Self(out)
	}
	
	/// Return the degree of the matrix
	///
	/// ```
	/// # use lineas::polynomials::Polynomial;
	/// assert_eq!(Polynomial::new(vec![0, 4, 3]).degree(), 1)
	/// ```
	pub fn degree(&self) -> usize where L: ValueFrom<isize> + PartialEq {
		self.minify().0.len() - 1
	}
	
	/// Return the real roots of a polynomial
	///
	/// For a 0th, 1st, 2nd, and 3rd degree polynomial this can be exactly calculated, but higher
	/// order polynomial roots must be calculated numerically and thus will be slightly inaccurate
	///
	/// ```
	/// # use lineas::polynomials::Polynomial;
	/// let example = Polynomial::new(vec![1, 1, -6]);
	/// assert_eq!(example.real_roots(), vec![2f32, -3f32])
	/// ```
	pub fn real_roots(&self) -> Vec<f32> where L: ValueFrom<isize> + PartialEq + Mul<Output=L> + Neg<Output=L> + Add<Output=L> + ValueInto<f32> {
		let minified = self.minify();
		match minified.degree() + 1 {
			0 => vec![0.],
			1 => if minified.0 == vec![0.value_as().unwrap()] { vec![0.] } else { Vec::new() },
			2 => {
				let discriminant = minified.0[1] * minified.0[1] + (-4).value_as::<L>().unwrap() * minified.0[2] * minified.0[0];
				let discriminant: f32 = discriminant.value_into().unwrap();
				if discriminant < 0. {
					Vec::new()
				} else if discriminant == 0. {
					vec![-minified.0[1].value_into().unwrap() / (2f32 * minified.0[2].value_into().unwrap())]
				} else {
					vec![
						(-minified.0[1].value_into().unwrap() + discriminant) / (2f32 * minified.0[2].value_into().unwrap()),
						-(minified.0[1].value_into().unwrap() + discriminant) / (2f32 * minified.0[2].value_into().unwrap())
					]
				}
			},
			_ => panic!("no")
		}
	}
}

impl<L: Copy + Debug, Q> Add<Q> for Polynomial<L> where Q: ValueInto<Polynomial<L>>, L: ValueFrom<isize> + Add<Output=L> + PartialEq {
	type Output = Polynomial<L>;
	
	fn add(self, rhs: Q) -> Self::Output {
		let rhs = rhs.value_as::<Polynomial<L>>().unwrap();
		let len = rhs.0.len().max(self.0.len());
		let (rhs, lhs) = if rhs.0.len() < len {
			let mut rhs_new = vec![0.value_as::<L>().unwrap(); len - rhs.0.len()];
			rhs_new.extend(rhs.0);
			(Polynomial(rhs_new), self)
		} else if self.0.len() < len {
			let mut self_new = vec![0.value_as::<L>().unwrap(); len - self.0.len()];
			self_new.extend(self.0);
			(rhs, Polynomial(self_new))
		} else {
			(rhs, self)
		};
		let mut out = Vec::new();
		let mut iter = rhs.0.iter().zip(lhs.0.iter());
		while let Some((l, r)) = iter.next() {
			if *l + *r == 0.value_as::<L>().unwrap() && out.len() == 0 {
				continue
			}
			out.push(*l + *r);
		}
		Polynomial(out)
	}
}

impl<L: Copy + Debug, Q> AddAssign<Q> for Polynomial<L> where Q: ValueInto<Polynomial<L>>, L: ValueFrom<isize> + Add<Output=L> + PartialEq {
	fn add_assign(&mut self, rhs: Q) {
		let rhs = rhs.value_as::<Polynomial<L>>().unwrap();
		let len = rhs.0.len().max(self.0.len());
		let (rhs, lhs) = if rhs.0.len() < len {
			let mut rhs_new = vec![0.value_as::<L>().unwrap(); len - rhs.0.len()];
			rhs_new.extend(rhs.0);
			(Polynomial(rhs_new), self.clone())
		} else if self.0.len() < len {
			let mut self_new = vec![0.value_as::<L>().unwrap(); len - self.0.len()];
			self_new.extend(self.0.clone());
			(rhs, Polynomial(self_new))
		} else {
			(rhs, self.clone())
		};
		let mut out = Vec::new();
		let mut iter = rhs.0.iter().zip(lhs.0.iter());
		while let Some((l, r)) = iter.next() {
			if *l + *r == 0.value_as::<L>().unwrap() && out.len() == 0 {
				continue
			}
			out.push(*l + *r);
		}
		self.0 = out
	}
}

impl<L: Copy + Debug, Q> Sub<Q> for Polynomial<L> where Q: ValueInto<Polynomial<L>>, L: ValueFrom<isize> + Sub<Output=L> + PartialEq {
	type Output = Polynomial<L>;
	
	fn sub(self, rhs: Q) -> Self::Output {
		let rhs = rhs.value_as::<Polynomial<L>>().unwrap();
		let len = rhs.0.len().max(self.0.len());
		let (rhs, lhs) = if rhs.0.len() < len {
			let mut rhs_new = vec![0.value_as::<L>().unwrap(); len - rhs.0.len()];
			rhs_new.extend(rhs.0);
			(Polynomial(rhs_new), self)
		} else if self.0.len() < len {
			let mut self_new = vec![0.value_as::<L>().unwrap(); len - self.0.len()];
			self_new.extend(self.0);
			(rhs, Polynomial(self_new))
		} else {
			(rhs, self)
		};
		let mut out = Vec::new();
		let mut iter = rhs.0.iter().zip(lhs.0.iter());
		while let Some((l, r)) = iter.next() {
			if *l - *r == 0.value_as::<L>().unwrap() && out.len() == 0 {
				continue
			}
			out.push(*l - *r);
		}
		Polynomial(out)
	}
}

impl<L: Copy + Debug, Q> SubAssign<Q> for Polynomial<L> where Q: ValueInto<Polynomial<L>>, L: ValueFrom<isize> + Sub<Output=L> + PartialEq {
	fn sub_assign(&mut self, rhs: Q) {
		let rhs = rhs.value_as::<Polynomial<L>>().unwrap();
		let len = rhs.0.len().max(self.0.len());
		let (rhs, lhs) = if rhs.0.len() < len {
			let mut rhs_new = vec![0.value_as::<L>().unwrap(); len - rhs.0.len()];
			rhs_new.extend(rhs.0);
			(Polynomial(rhs_new), self.clone())
		} else if self.0.len() < len {
			let mut self_new = vec![0.value_as::<L>().unwrap(); len - self.0.len()];
			self_new.extend(self.0.clone());
			(rhs, Polynomial(self_new))
		} else {
			(rhs, self.clone())
		};
		let mut out = Vec::new();
		let mut iter = rhs.0.iter().zip(lhs.0.iter());
		while let Some((l, r)) = iter.next() {
			if *l - *r == 0.value_as::<L>().unwrap() && out.len() == 0 {
				continue
			}
			out.push(*l - *r);
		}
		self.0 = out
	}
}

impl<L: Copy + Debug, Q> Mul<Q> for Polynomial<L> where Q: ValueInto<Polynomial<L>>, L: ValueFrom<isize> + Add<Output=L> + Mul<Output=L> + PartialEq {
	type Output = Polynomial<L>;
	
	fn mul(self, rhs: Q) -> Self::Output {
		let rhs = rhs.value_as::<Polynomial<L>>().unwrap();
		let mut out = vec![0.value_as::<L>().unwrap(); self.degree() + rhs.degree() + 1];
		for (p1, v1) in self.0.iter().enumerate() {
			for (p2, v2) in rhs.0.iter().enumerate() {
				out[p1 + p2] = out[p1 + p2] + *v1 * *v2
			}
		}
		Polynomial(out).minify()
	}
}

impl<L: Copy + Debug, Q> MulAssign<Q> for Polynomial<L> where Q: ValueInto<Polynomial<L>>, L: ValueFrom<isize> + Add<Output=L> + Mul<Output=L> + PartialEq {
	fn mul_assign(&mut self, rhs: Q) {
		let rhs = rhs.value_as::<Polynomial<L>>().unwrap();
		let mut out = vec![0.value_as::<L>().unwrap(); self.degree() + rhs.degree() + 1];
		for (p1, v1) in self.0.iter().enumerate() {
			for (p2, v2) in rhs.0.iter().enumerate() {
				out[p1 + p2] = out[p1 + p2] + *v1 * *v2
			}
		}
		self.0 = Polynomial(out).minify().0
	}
}

impl<L: Copy + Debug> Neg for Polynomial<L> where L: ValueFrom<isize> + Add<Output=L> + PartialEq + Neg<Output=L> {
	type Output = Polynomial<L>;
	
	fn neg(self) -> Self::Output {
		Polynomial(self.0.iter().map(|t| -*t).collect())
	}
}
