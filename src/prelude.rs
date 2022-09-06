//! All the important bases
//!
//! This defined the matrix and vector as well as some simple operation implementations for
//! matrices. More complex operations are defined in other places
use std::fmt::Debug;
use std::ops::Add;
use conv::{ConvUtil, ValueFrom};
use crate::traits::{Abs, Pows};
use crate::constants::PI;

/// Matrix struct
///
/// A `Matrix<T, N, L>` contains an `[[L; N]; T]` array where `N` is the number of rows and `T` is
/// the number of columns giving a `T`×`N` matrix
///
/// To make a new matrix, do [`Matrix::new(data)`][Matrix::new]. You can also specify the type of the data by
/// following this with a call to the [`dtype`][Matrix::dtype] function.
///
/// For example
/// ```
/// use lineas::Matrix;
/// let matrix: Matrix<2, 2, f64> = Matrix::new([[1, 0], [0, 1]]).dtype::<f64>();
/// ```
/// This gives us an identity matrix of size 2 with each value being an `f64` instead of `{integer}`
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Matrix<const T: usize, const N: usize, L: Copy + Debug> (pub(crate) [[L; N]; T] );

/// Vector struct
///
/// This is a row vector. For a column vector see the [`ColVector'][ColVector] struct
///
/// Derived from the [matrix][Matrix] and is technically a 1×`T` matrix
#[doc(alias="RowVector")]
pub type Vector<const T: usize, L> = Matrix<1, T, L>;

/// Column Vector struct
///
/// For a row vector see the [`Vector'][Vector] struct
///
/// Derived from the [matrix][Matrix] and is technically a `T`×1 matrix
pub type ColVector<const T: usize, L> = Matrix<T, 1, L>;

/// # Norm enum
///
/// The norm enum is used when calculating the norm of a vector with the [`norm`][Vector::norm]. This provides some set norms but
/// also allows you to specify a custom norm.
pub enum Norm<Q> {
	/// Euclidean norm (square root of the sum of the squares)
	Euclidean,
	/// Manhattan norm (sum of absolute values)
	Manhattan,
	/// p-norm (p-th root of the sum of the p-th exponents)
	#[allow(non_camel_case_types)]
	p_norm(usize),
	/// Custom norm
	///
	/// This is not the recommended way to use a custom norm. For the recommended way see
	/// [`norm`][Vector::norm]
	Custom(fn(Vec<Q>) -> Q)
}

impl<Q: Debug> Norm<Q> where Q: ValueFrom<isize> + Copy + Add<Output=Q> + Pows + Abs {
	pub(crate) fn call(self, t: Vec<Q>) -> Q {
		match self {
			Norm::Euclidean => t.iter().fold(0.value_as::<Q>().unwrap(), |acc, v| acc + v.power(2.value_as::<Q>().unwrap())).root_n(2.value_as::<Q>().unwrap()),
			Norm::Manhattan => t.iter().fold(0.value_as::<Q>().unwrap(), |acc, v| acc + v.absolute()),
			Norm::p_norm(p) => t.iter().fold(0.value_as::<Q>().unwrap(), |acc, v| acc + v.absolute().power((p as isize).value_as::<Q>().unwrap())).root_n((p as isize).value_as::<Q>().unwrap()),
			Norm::Custom(f) => f(t)
		}
	}
}

/// Complex number struct
///
/// This contains two values, a real and an imaginary. It does complex number things as you would
/// expect.
///
/// To generate a new
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Complex<L: Copy + Debug> {
	pub(crate) real: L,
	pub(crate) imaginary: L
}

/// Angle enum
///
/// Crate-wide enum for holding angles
pub enum Angle {
	/// Angle in degrees
	Degrees(f64),
	/// Angle in radians
	Radians(f64),
	/// Angle in gradians
	Gradians(f64)
}

impl Angle {
	pub(crate) fn angle(&self) -> f64 {
		match self {
			Angle::Degrees(v) => *v * PI / 180.,
			Angle::Radians(v) => *v,
			Angle::Gradians(v) => *v * PI / 50.
		}
	}
}
