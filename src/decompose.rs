use std::fmt::Debug;
use std::ops::{Div, Mul, Sub};
use conv::{ConvUtil, ValueFrom};
use crate::prelude::Matrix;

/// LU Decomposition
///
/// Attempts to turn a square matrix into a lower and upper matrix such that the lower multiplied by
/// the upper results in the original matrix.
///
/// This will fail if during the process of constructing the upper matrix a 0 is found along the
/// diagonal. This does not mean that a 0 cannot occur in the diagonal of the original matrix
pub mod lu_decompose {
	use super::*;
	impl<const T: usize, L: Copy + Debug> Matrix<T, T, L> where L: ValueFrom<isize> + Div<Output=L> + Mul<Output=L> + Sub<Output=L> + PartialEq {
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

pub mod plu_decompose {
	use super::*;
	impl<const T: usize, L: Copy + Debug> Matrix<T, T, L> where L: ValueFrom<isize> + Div<Output=L> + Mul<Output=L> + Sub<Output=L> + PartialEq {
		pub fn plu_decompose(&self) -> Option<(Self, Self, Self)> {
			None
		}
	}
}
