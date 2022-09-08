/// Implementations for the matrix struct
use crate::prelude::{Matrix, Vector};
use std::fmt::Debug;
use std::ops::{Add, Div, Mul, Neg, Sub};
use conv::{ConvUtil, ValueFrom, ValueInto};
use itertools::Itertools;
use crate::{Complex, Norm, Pows};
use crate::traits::{Abs, Epsilon};

/// # All matrices
impl<const T: usize, const N: usize, L: Copy + Debug> Matrix<T, N, L> {
	/// Generate a new matrix from a given array `[[L; N]; T]`
	pub fn new(data: [[L; N]; T]) -> Self {
		Self(data)
	}
	
	/// Conjugate of the matrix
	///
	/// If the matrix is not already, make it a complex matrix and calculate the conjugate of each
	/// element.
	pub fn conj<Q: Copy + Debug>(&self) -> Matrix<T, N, Complex<Q>> where L: ValueFrom<isize> + ValueInto<Complex<Q>>, Q: Neg<Output=Q> + ValueFrom<isize> {
		let mut data = [[Complex::from_real(0.value_as().unwrap()); N]; T];
		for n in 0..T {
			for m in 0..N {
				data[n][m] = self.0[n][m].value_into().unwrap().conj()
			}
		}
		Matrix::new(data)
	}
	
	/// Sums all the elements of a matrix
	pub fn sum(&self) -> L where L: Add<Output=L> + ValueFrom<isize> {
		self.0.iter().fold(
			0.value_as().unwrap(),
			|acc, l| acc + l.iter().fold(
				0.value_as().unwrap(),
				|acc2, t| acc2 + *t
			)
		)
	}
	
	/// Return the maximum value in the matrix
	///
	/// This will panic if the matrix has no values in it (0×0 matrix)
	pub fn max(&self) -> L where L: PartialOrd {
		let mut out = self.0[0][0];
		for i in 0..T {
			for n in 0..N {
				out = if self.0[i][n] > out { self.0[i][n] } else { out }
			}
		}
		out
	}
	
	/// Return the minimum value in the matrix
	///
	/// This will panic if the matrix has no values in it (0×0 matrix)
	pub fn min(&self) -> L where L: PartialOrd {
		let mut out = self.0[0][0];
		for i in 0..T {
			for n in 0..N {
				out = if self.0[i][n] < out { self.0[i][n] } else { out }
			}
		}
		out
	}
	
	/// Return the minimum and maximum value in the matrix
	///
	/// This is equivalent to <code>([self.min()][Matrix::min], [self.max()][Matrix::max])</code>
	///
	/// This will panic if the matrix has no values in it (0×0 matrix)
	pub fn min_max(&self) -> (L, L) where L: PartialOrd {
		(self.min(), self.max())
	}
	
	/// Return the row with maximum norm value
	///
	/// This will panic if the matrix has no values in it (0×0 matrix)
	pub fn max_row(&self, norm: Norm<L>) -> Vector<N, L> where L: PartialOrd + ValueFrom<isize> + Add<Output=L> + Abs + Pows {
		let mut out = Vector::new([self.0[0]]);
		let mut out_norm = out.norm(norm);
		for i in 1..T {
			let c_norm = Vector::new([self.0[i]]).norm(norm);
			if out_norm > c_norm {
				out_norm = c_norm;
				out = Vector::new([self.0[i]])
			}
		}
		out
	}
	
	/// Return the row with minimum norm value
	///
	/// This will panic if the matrix has no values in it (0×0 matrix)
	pub fn min_row(&self, norm: Norm<L>) -> Vector<N, L> where L: PartialOrd + ValueFrom<isize> + Add<Output=L> + Abs + Pows {
		let mut out = Vector::new([self.0[0]]);
		let mut out_norm = out.norm(norm);
		for i in 1..T {
			let c_norm = Vector::new([self.0[i]]).norm(norm);
			if out_norm > c_norm {
				out_norm = c_norm;
				out = Vector::new([self.0[i]])
			}
		}
		out
	}
	
	/// Return the rows with minimum and maximum norm value
	///
	/// This is equivalent to <code>([self.min_row()][Matrix::min_row], [self.max_row()][Matrix::max_row])</code>
	///
	/// This will panic if the matrix has no values in it (0×0 matrix)
	pub fn min_max_row(&self, norm: Norm<L>) -> (Vector<N, L>, Vector<N, L>) where L: PartialOrd + ValueFrom<isize> + Add<Output=L> + Abs + Pows {
		(self.min_row(norm), self.max_row(norm))
	}
	
	/// Return the matrix with the elementwise absolute values
	pub fn abs(&self) -> Self where L: Abs {
		let mut out = self.0.clone();
		for i in 0..N {
			for j in 0..T {
				out[i][j] = out[i][j].absolute()
			}
		}
		Matrix(out)
	}
	
	/// Transform the type of the data to the specified type assuming given traits are met
	///
	/// The new data type (dtype) must implement `Copy`, `Debug`, `ValueFrom<L>`, and
	/// `ValueFrom<isize>` where `L` is the current dtype. If you're using any built-in numerical value (`i8` to `i64` and `isize`
	/// and unsigned equivalents as well as `f32`, `f64` satisfy these traits). If you're using a
	/// custom dtype you need to implement these manually.
	///
	/// > The `ValueFrom` trait is defined in the [conv](https://crates.io/crates/conv) crate
	pub fn dtype<Q>(self) -> Matrix<T, N, Q> where Q: Copy + Debug + ValueFrom<L> + ValueFrom<isize> {
		let mut out= [[0.value_as().unwrap(); N]; T];
		for i in 0..T {
			for n in 0..N {
				out[i][n] = self[(i, n)].value_as::<Q>().unwrap()
			}
		}
		Matrix::new(out)
	}
	
	/// Transpose the matrix
	///
	/// Returns the transposed matrix, leaving the original unchanged
	pub fn transpose(&self) -> Matrix<N, T, L> where L: ValueFrom<isize> {
		let mut data = [[0.value_as().unwrap(); T]; N];
		for i in 0..T {
			for j in 0..N {
				data[j][i] = self.0[i][j];
			}
		}
		Matrix::new(data)
	}
	
	/// Scale the matrix
	///
	/// Returns the scaled matrix and does not alter the original
	/// ```
	/// # use lineas::prelude::Matrix;
	/// let lhs = Matrix::new([[1, 2], [3, 4]]);
	/// let rhs = Matrix::new([[2, 4], [6, 8]]);
	/// assert_eq!(lhs.scale(2), rhs)
	/// ```
	pub fn scale<V>(&self, v: V) -> Matrix<T, N, L> where L: Clone + Mul<Output=L> + ValueFrom<V> {
		let mut data = self.0.clone();
		let multiplier: L = v.value_as().unwrap();
		for i in 0..T {
			for j in 0..N {
				data[i][j] = data[i][j] * multiplier;
			}
		}
		Matrix::new(data)
	}
	
	/// Scale the matrix
	///
	/// Scales a matrix in place without returning it.
	/// ```
	/// # use lineas::prelude::Matrix;
	/// let mut lhs = Matrix::new([[1, 2], [3, 4]]);
	/// let rhs = Matrix::new([[2, 4], [6, 8]]);
	/// lhs.scale_set(2);
	/// assert_eq!(lhs, rhs)
	/// ```
	pub fn scale_set<V>(&mut self, v: V) where L: Clone + Mul<Output=L> + ValueFrom<V> {
		let multiplier: L = v.value_as().unwrap();
		for i in 0..T {
			for j in 0..N {
				self.0[i][j] = self.0[i][j] * multiplier;
			}
		}
	}
	
	/// Filters out small values using the [`Epsilon`][crate::traits::Epsilon] trait
	///
	/// This is implemented by default for `f32`, `f64`, `Complex<f32>`, and `Complex<f64>`
	///
	/// This is intended for use with function that will inherently be slightly inaccurate such
	/// trigonometric function. A good sample use would be rotation matrices
	/// ```
	/// # use lineas::prelude::Matrix;
	/// use lineas::Angle;
	/// Matrix::rotation(Angle::Degrees(180.)).epsilon_filter()
	/// ```
	pub fn epsilon_filter(&self) -> Self where L: Epsilon + PartialEq {
		let mut out = self.0.clone();
		for i in 0..T {
			for n in 0..N {
				out[i][n] = out[i][n].epsilon()
			}
		}
		Matrix(out)
	}
}

/// # Complex valued matrices
impl<const T: usize, const N: usize, L: Copy + Debug> Matrix<T, N, Complex<L>> {
	/// Return the matrix with the elementwise complex absolute values.
	///
	/// This is not to be confused with the [`abs`][Matrix::abs] function which when given complex
	/// values will return complex values with no an imaginary component of 0.
	/// ```
	/// # use lineas::{comp, Matrix};
	/// let lhs = Matrix::new([[comp!(, 3), comp!(-4)], [comp!(5, -2), comp!(1, 1)]]);
	/// let rhs = Matrix::new([[comp!(, 3), comp!(4)], [comp!(5, 2), comp!(1, 1)]]);
	/// assert_eq!(lhs.c_abs(), rhs)
	/// ```
	pub fn c_abs(&self) -> Self where L: Abs {
		let mut out = self.0.clone();
		for i in 0..N {
			for j in 0..T {
				out[i][j] = out[i][j].element_abs()
			}
		}
		Matrix(out)
	}
}

/// # Square matrices
impl<const T: usize, L: Copy + Debug> Matrix<T, T, L> {
	/// Returns the determinant of a square matrix
	///
	/// Calculated using Leibniz's determinant formula based on set permutations. Used instead of a
	/// recursive formula because recursion is slow
	pub fn determinant(&self) -> L where L: Add<Output=L> + Sub<Output=L> + Mul<Output=L> + ValueFrom<isize> {
		match T {
			1 => self.0[0][0],
			2 => self.0[0][0] * self.0[1][1] - self.0[1][0] * self.0[0][1],
			3 => {
				self.0[0][0] * (self.0[1][1] * self.0[2][2] - self.0[2][1] * self.0[1][2])
				- self.0[0][1] * (self.0[1][0] * self.0[2][2] - self.0[2][0] * self.0[1][2])
				+ self.0[0][2] * (self.0[1][0] * self.0[2][1] - self.0[2][0] * self.0[1][1])
			}
			_ => {
				fn perm_sign(p: Vec<&usize>) -> isize {
					let mut out = 0;
					for i in 0..p.len() {
						for n in i+1..p.len() {
							if p[i] > p[n] { out += 1 }
						}
					}
					if out % 2 == 0 { 1 } else { -1 }
				}
				let mut out = 0.value_as().unwrap();
				let permutation = (0..T).collect::<Vec<usize>>();
				for p in permutation.iter().permutations(T) {
					let mut temp: L = 1.value_as().unwrap();
					for (r, c) in p.iter().enumerate() {
						temp = temp * self[(r, **c)];
					}
					out = out + perm_sign(p).value_as::<L>().unwrap() * temp;
				}
				out
			}
		}
	}
	
	/// Returns the trace of a matrix
	pub fn trace(&self) -> L where L: ValueFrom<isize> + Add<Output=L> {
		let mut out = 0.value_as().unwrap();
		for i in 0..T {
			out  = out + self.0[i][i]
		}
		out
	}
	
	/// Return the diagonal of the matrix
	///
	/// Returns a `Vector<T, L>` of values alog the diagonal of the matrix
	pub fn diag(&self) -> Vector<T, L> where L: ValueFrom<isize> {
		let mut data = [0.value_as().unwrap(); T];
		for i in 0..T {
			data[i] = self[(i, i)];
		}
		Vector::new([data])
	}
	
	/// Set the current matrix to the transposed version of itself
	///
	/// Will only work for square matrices because the transposed matrix has to be the same shape as
	/// the original matrix to be able to assign the transposed version to itself
	pub fn transpose_set(&mut self) {
		for i in 0..T {
			for j in i+1..T {
				let temp = self.0[i][j];
				self.0[i][j] = self.0[j][i];
				self.0[j][i] = temp
			}
		}
	}
}

/// # Elementwise operations
impl<const T: usize, const N: usize, L: Copy + Debug> Matrix<T, N, L> {
	/// Elementwise multiplication
	///
	/// Returns the result of elementwise multiplication
	///
	/// For an assigning with a mutable matrix see [`Matrix::elem_mult_assign`]
	pub fn elem_mult<Q: Copy + Debug>(&self, rhs: Matrix<T, N, Q>) -> Self where Q: Mul<Output=Self>, L: ValueFrom<Q> + ValueFrom<isize> + Mul<Output=L> {
		let rhs = rhs.dtype::<L>().0;
		let mut out = self.0.clone();
		for i in 0..T {
			for n in 0..N {
				out[i][n] = out[i][n] * rhs[i][n]
			}
		}
		Matrix(out)
	}
	
	/// Elementwise multiplication
	///
	/// Assigns the result of elementwise multiplication to `self`
	///
	/// For a non-assigning version see [`Matrix::elem_mult`]
	pub fn elem_mult_assign<Q: Copy + Debug>(&mut self, rhs: Matrix<T, N, Q>) where Q: Mul<Output=Self>, L: ValueFrom<Q> + ValueFrom<isize> + Mul<Output=L> {
		for i in 0..T {
			let rhs = rhs.dtype::<L>().0;
			for n in 0..N {
				self.0[i][n] = self.0[i][n] * rhs[i][n]
			}
		}
	}
	
	/// Elementwise division
	///
	/// Returns the result of elementwise division
	///
	/// For an assigning with a mutable matrix see [`Matrix::elem_div_assign`]
	pub fn elem_div<Q: Copy + Debug>(&self, rhs: Matrix<T, N, Q>) -> Self where Q: Div<Output=Self>, L: ValueFrom<Q> + ValueFrom<isize> + Div<Output=L> {
		let rhs = rhs.dtype::<L>().0;
		let mut out = self.0.clone();
		for i in 0..T {
			for n in 0..N {
				out[i][n] = out[i][n] / rhs[i][n]
			}
		}
		Matrix(out)
	}
	
	/// Elementwise division
	///
	/// Assigns the result of elementwise division to `self`
	///
	/// For a non-assigning version see [`Matrix::elem_div`]
	pub fn elem_div_assign<Q: Copy + Debug>(&mut self, rhs: Matrix<T, N, Q>) where Q: Div<Output=Self>, L: ValueFrom<Q> + ValueFrom<isize> + Div<Output=L> {
		for i in 0..T {
			let rhs = rhs.dtype::<L>().0;
			for n in 0..N {
				self.0[i][n] = self.0[i][n] / rhs[i][n]
			}
		}
	}
}
