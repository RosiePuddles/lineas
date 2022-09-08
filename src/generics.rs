//! # Implementation for generic matrix types
//!
//! This module adds implementations for the [`Matrix`][Matrix] struct to allow you to generate
//! generic matrices such as an empty or identity matrix of given sizes.

use std::fmt::Debug;
use crate::prelude::Matrix;
use conv::{ConvUtil, ValueFrom};
use crate::{Angle, Rotation};

impl<const T: usize, L: Copy + Debug> Matrix<T, T, L> where L: ValueFrom<isize> {
	/// Generate an identity matrix of a specified size.
	///
	/// This will only work for a `Matrix<T, T, L>` matrix
	/// ```
	/// # use lineas::prelude::Matrix;
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

impl<L: Copy + Debug> Matrix<2, 2, L> where L: Rotation {
	/// Generate a 2D rotation matrix for a specified angle
	///
	/// This will only work for a `Matrix<T, T, L>` matrix
	/// ```
	/// # use lineas::prelude::Matrix;
	/// assert_eq!(Matrix::new([[1, 0], [0, 1]]), Matrix::<2, 2, _>::identity())
	/// ```
	pub fn rotation<Q: Rotation>(angle: Angle<Q>) -> Self {
		let angle = angle.angle();
		Matrix::new([[L::back(angle.cos()), L::back(-angle.sin())], [L::back(angle.sin()), L::back(angle.cos())]])
	}
}

impl<const T: usize, const N: usize, L: Copy + Debug> Matrix<T, N, L> where L: ValueFrom<isize> {
	/// Generate an empty matrix of a specified size
	/// ```
	/// # use lineas::prelude::Matrix;
	/// assert_eq!(Matrix::new([[0, 0, 0], [0, 0, 0]]), Matrix::<3, 2, _>::empty())
	/// ```
	pub fn empty() -> Self {
		Self::new([[0.value_as().unwrap(); N]; T])
	}
}
