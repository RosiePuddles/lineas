//! Operation implementations for the matrix struct
use crate::prelude::Matrix;
use std::fmt::Debug;
use std::ops::{Add, Sub, Index, AddAssign, IndexMut, SubAssign, Mul, MulAssign, Neg};
use conv::{ConvUtil, ValueFrom};

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
		for i in 0..T {
			for j in 0..P {
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
