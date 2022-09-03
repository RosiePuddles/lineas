//! # Implementation for generic matrix types
//!
//! This module adds implementations for the [`Matrix`][Matrix] struct to allow you to generate
//! generic matrices such as an empty or identity matrix of given sizes.

use std::fmt::Debug;
use crate::prelude::Matrix;
use conv::{ConvUtil, ValueFrom};

impl<const T: usize, L: Copy + Debug> Matrix<T, T, L> where L: ValueFrom<isize> {
	/// Generate an identity matrix of a specified size.
	///
	/// This will only work for a `Matrix<T, T, L>` matrix
	/// ```
	/// # use lineas::prelude::Matrix;
	/// use lineas::generics;
	/// assert_eq!(Matrix::new([[1, 0], [0, 1]]), Matrix::<2, 2, _>::identity())
	/// ```
	pub fn identity() -> Self {
		let mut out = Self::empty();
		for i in 0..T {
			out[(i, i)] = 1.value_as().unwrap();
		}
		out
	}
}

impl<const T: usize, const N: usize, L: Copy + Debug> Matrix<T, N, L> where L: ValueFrom<isize> {
	/// Generate an empty matrix of a specified size
	/// ```
	/// # use lineas::prelude::Matrix;
	/// use lineas::generics;
	/// assert_eq!(Matrix::new([[0, 0, 0], [0, 0, 0]]), Matrix::<3, 2, _>::empty())
	/// ```
	pub fn empty() -> Self {
		Self::new([[0.value_as().unwrap(); N]; T])
	}
}
