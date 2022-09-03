use std::fmt::Debug;
use crate::prelude::Matrix;
use conv::{ConvUtil, ValueFrom};

impl<const T: usize, L: Copy + Debug> Matrix<T, T, L> where L: ValueFrom<isize> {
	pub fn identity() -> Self {
		let mut out = Self::empty();
		for i in 0..T {
			out[(i, i)] = 1.value_as().unwrap();
		}
		out
	}
}

impl<const T: usize, const N: usize, L: Copy + Debug> Matrix<T, N, L> where L: ValueFrom<isize> {
	pub fn empty() -> Self {
		Self::new([[0.value_as().unwrap(); N]; T])
	}
}
