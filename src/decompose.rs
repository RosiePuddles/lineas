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

/// LU decomposition module
///
/// For more information see the [`lu_decompose` function][Matrix::lu_decompose]
pub mod lu_decompose {
	use super::*;
	impl<const T: usize, L: Copy + Debug> Matrix<T, T, L> where L: ValueFrom<isize> + Div<Output=L> + Mul<Output=L> + Sub<Output=L> + PartialEq {
		/// Generate the LU decomposition of a square matrix
		///
		/// The LU decomposition of a matrix `A` is two matrices `L` and `U` that are lower and
		/// upper matrices respectively such that `LU=A`. For some matrices this decomposition is
		/// not possible so we return an `Option<(Self, Self)>` enum.
		pub fn lu_decompose(&self) -> Option<(Self, Self)> {
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
}

// pub mod plu_decompose {
// 	use super::*;
// 	impl<const T: usize, L: Copy + Debug> Matrix<T, T, L> where L: ValueFrom<isize> + Div<Output=L> + Mul<Output=L> + Sub<Output=L> + PartialEq {
// 		pub fn plu_decompose(&self) -> Option<(Self, Self, Self)> {
// 			None
// 		}
// 	}
// }
