//! Operation implementations for the matrix

use crate::prelude::{Matrix, Vector};
use std::fmt::Debug;
use std::ops::{Add, Sub, Index, AddAssign, IndexMut, SubAssign, Mul, MulAssign, Neg};
use conv::{ConvUtil, ValueFrom, ValueInto};
use itertools::Itertools;
use crate::Complex;

impl<const T: usize, const N: usize, L: Copy + Debug> Add for Matrix<T, N, L> where L: Add + AddAssign {
	type Output = Self;
	
	fn add(self, rhs: Self) -> Self::Output {
		let mut out = self;
		out += rhs;
		out
	}
}

impl<const T: usize, const N: usize, L: Copy + Debug> AddAssign for Matrix<T, N, L> where L: Add + AddAssign {
	fn add_assign(&mut self, rhs: Self) {
		for i in 0..N {
			for n in 0..T {
				self[(i, n)] += rhs[(i, n)]
			}
		}
	}
}

impl<const T: usize, const N: usize, L: Copy + Debug> Sub for Matrix<T, N, L> where L: Sub + SubAssign {
	type Output = Self;
	
	fn sub(self, rhs: Self) -> Self::Output {
		let mut out = self;
		out -= rhs;
		out
	}
}

impl<const T: usize, const N: usize, L: Copy + Debug> SubAssign for Matrix<T, N, L> where L: Sub + SubAssign {
	fn sub_assign(&mut self, rhs: Self) {
		for i in 0..N {
			for n in 0..T {
				self[(i, n)] -= rhs[(i, n)]
			}
		}
	}
}

impl<const T: usize, const N: usize, const P: usize, L: Copy + Debug + ValueFrom<isize>> Mul<Matrix<N, P, L>> for Matrix<T, N, L> where L: Add<Output=L> + AddAssign + Mul<Output=L> + ValueFrom<isize> {
	type Output = Matrix<T, P, L>;
	
	/// Multiply two matrices together
	///
	/// The first matrix must have the same number of rows as the second matrix has columns
	/// otherwise multiplication is not defined
	///
	/// Note that matrix multiplication is not commutative; i.e. `A*B` is not necessarily the same
	/// as `B * A`.
	fn mul(self, rhs: Matrix<N, P, L>) -> Self::Output {
		let mut data = [[0.value_as().unwrap(); P]; T];
		for i in 0..P {
			for j in 0..T {
				let mut res: L = 0.value_as().unwrap();
				for k in 0..N {
					res = res + (self.0[i][k] * rhs.0[k][j]);
				}
				data[i][j] = res;
			}
		}
		Matrix::new(data)
	}
}

impl<const T: usize, const N: usize, L: Copy + Debug + ValueFrom<isize>> MulAssign<Matrix<N, N, L>> for Matrix<T, N, L> where L: Add<Output=L> + AddAssign + Mul<Output=L> + ValueFrom<isize> {
	fn mul_assign(&mut self, rhs: Matrix<N, N, L>) {
		let mut data = [[0.value_as().unwrap(); N]; T];
		for i in 0..N {
			for j in 0..T {
				let mut res: L = 0.value_as().unwrap();
				for k in 0..N {
					res = res + self[(i, k)] * rhs[(k, j)];
				}
				data[i][j] = res;
			}
		}
		self.0 = data;
	}
}

impl<const T: usize, const N: usize, L: Copy + Debug> Neg for Matrix<T, N, L> where L: Add<Output=L> + AddAssign + ValueFrom<isize> + Mul<Output=L> {
	type Output = Self;
	
	fn neg(self) -> Self::Output {
		let mut data = self.0.clone();
		for i in 0..T {
			for j in 0..N {
				data[i][j] = data[i][j] * (-1).value_as::<L>().unwrap();
			}
		}
		Matrix::new(data)
	}
}

impl<const T: usize, const N: usize, L: Copy + Debug> Index<(usize, usize)> for Matrix<T, N, L> {
	type Output = L;
	
	fn index(&self, index: (usize, usize)) -> &Self::Output {
		&self.0[index.0][index.1]
	}
}

impl<const T: usize, const N: usize, L: Copy + Debug> IndexMut<(usize, usize)> for Matrix<T, N, L> {
	fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
		&mut self.0[index.0][index.1]
	}
}

impl<const T: usize, const N: usize, L: Copy + Debug> Matrix<T, N, L> {
	/// Generate a new matrix from a given array `[[L; N]; T]`
	pub fn new(data: [[L; N]; T]) -> Self {
		Self(data)
	}
	
	pub fn conj<Q: Copy + Debug>(&self) -> Matrix<T, N, Complex<Q>> where L: ValueFrom<isize> + ValueInto<Complex<Q>>, Q: Neg<Output=Q> + ValueFrom<isize> {
		let mut data = [[Complex::from_real(0.value_as().unwrap()); N]; T];
		for n in 0..T {
			for m in 0..N {
				data[n][m] = self.0[n][m].value_into().unwrap().conj()
			}
		}
		Matrix::new(data)
	}
}

impl<const T: usize, const N: usize, L: Copy + Debug> Matrix<T, N, L> {
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
}

impl<const T: usize, L: Copy + Debug> Matrix<T, T, L> {
	/// Returns the determinant of a square matrix
	///
	/// Calculated using Leibniz's determinant formula based on set permutations. Used instead of a
	/// recursive formula because recursion is slow
	pub fn determinant(&self) -> L where L: Add<Output=L> + Mul<Output=L> + ValueFrom<isize> {
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
