//! Decomposition modules
//!
//! This module includes modules for decomposition functions for matrices. To use a specific
//! decomposition you need to include the module which will include the implementation.
//!
//! For example to include LU decomposition, you would have
//!```
//! # use lineas::prelude;
//! use lineas::decompose::lu_decompose;
//!```

use std::fmt::Debug;
use std::ops::{Div, Mul, Sub};
use conv::{ConvUtil, ValueFrom};
use crate::prelude::Matrix;
use itertools::Itertools;

/// LU decomposition trait
///
/// This is implemented for square matrices
pub trait LUDecompose {
	/// Generate the LU decomposition of a square matrix
	///
	/// The LU decomposition of a matrix `A` is two matrices `L` and `U` that are lower and
	/// upper matrices respectively such that `LU=A`. For some matrices this decomposition is
	/// not possible so we return an `Option<(Self, Self)>` enum.
	///
	/// For example
	/// ```
	/// # use lineas::Matrix;
	/// # #[allow(non_snake_case)]
	/// let A = Matrix::new([[2, -2], [-12, 16]]);
	/// # #[allow(non_snake_case)]
	/// let L = Matrix::new([[1, 0], [-3, 2]]);
	/// # #[allow(non_snake_case)]
	/// let U = Matrix::new([[4, -2], [0, 5]]);
	/// assert_eq!(A, L * U)
	/// ```
	///
	/// > We recommend using floats for decomposition
	fn lu_decompose(&self) -> Option<(Self, Self)> where Self: Sized;
}

impl<const T: usize, L: Copy + Debug> LUDecompose for Matrix<T, T, L> where L: ValueFrom<isize> + Div<Output=L> + Mul<Output=L> + Sub<Output=L> + PartialEq {
	fn lu_decompose(&self) -> Option<(Self, Self)> {
		let mut upper = self.clone();
		let mut lower = Self::empty();
		for r in 0..T {
			let scale = upper[(r, r)];
			if scale == 0.value_as().unwrap() {
				return None
			}
			for i in r..T {
				lower[(i, r)] = upper[(i, r)]
			}
			for i in 0..T {
				upper[(r, i)] = upper[(r, i)] / scale
			}
			for lower in r + 1..T {
				let second_scale = upper[(lower, r)];
				for col in r..T {
					upper[(lower, col)] = upper[(lower, col)] - second_scale * upper[(r, col)]
				}
			}
		}
		Some((lower, upper))
	}
}

/// PLU decomposition trait
///
/// This is implemented for square matrices
pub trait PLUDecompose {
	/// Attempts to generate a PLU decomposition for a square matrix
	///
	/// A PLU decomposition is an LU decomposition of a permutation of a matrix where `P` is the
	/// permutation matrix. If a PLU decomposition exists for a matrix `A` that means that the
	/// product `P * L * U == A`.
	///
	/// This can fail when no permutation has an LU decomposition.
	///
	/// > We recommend using floats for decomposition
	fn plu_decompose(&self) -> Option<(Self, Self, Self)> where Self: Sized;
}

impl<const T: usize, L: Copy + Debug> PLUDecompose for  Matrix<T, T, L> where L: ValueFrom<isize> + Div<Output=L> + Mul<Output=L> + Sub<Output=L> + PartialEq {
	fn plu_decompose(&self) -> Option<(Self, Self, Self)> {
		for i in (0..T).permutations(T) {
			let mut p_data = [[0.value_as::<L>().unwrap(); T]; T];
			for (n, j) in i.iter().enumerate() {
				p_data[n] = self.0[*j].clone()
			}
			if let Some((l, u)) = Matrix::new(p_data).lu_decompose() {
				let mut permutation = Matrix::<T, T, L>::empty();
				for (n, j) in i.iter().enumerate() {
					permutation.0[n][*j] = 1.value_as().unwrap()
				}
				return Some((permutation, l, u))
			}
		}
		None
	}
}

/// Diagonalisation decomposition
///
/// Implemented for square matrices
#[doc(alias="Diagonalize")]
pub trait Diagonalise {
	/// Diagonalisation a matrix
	///
	/// This means that for a matrix `A`, we have two matrices `P` and `D` such that `PDP⁻¹ = A` where
	/// `P⁻¹` is the inverse of `P`.
	///
	/// The process is linked to finding eigenvalues and so can be inaccurate due to the nature of
	/// eigenvalue calculation.
	///
	/// > We recommend using floats for decomposition
	// TODO: impl Diagonalise for Matrix
	fn diagonalise(&self) -> Option<(Self, Self, Self)> where Self: Sized;
}
