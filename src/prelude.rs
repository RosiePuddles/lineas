//! All the important bases
//!
//! This defined the matrix and vector as well as some simple operation implementations for
//! matrices. More complex operations are defined in other places
use std::fmt::Debug;
use std::ops::Add;
use conv::{ConvUtil, ValueFrom};
use crate::traits::{Abs, Pows, Rotation};
use crate::constants::PI;

#[doc = include_str!("../docs/matrix.md")]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Matrix<const T: usize, const N: usize, L: Copy + Debug> (pub(crate) [[L; N]; T] );

#[doc(alias="RowVector")]
#[doc = include_str!("../docs/vector.md")]
pub type Vector<const T: usize, L> = Matrix<1, T, L>;

#[doc = include_str!("../docs/colvector.md")]
pub type ColVector<const T: usize, L> = Matrix<T, 1, L>;

/// # Norm enum
///
/// The norm enum is used when calculating the norm of a vector with the [`norm`][Vector::norm]. This provides some set norms but
/// also allows you to specify a custom norm.
#[derive(Copy, Clone)]
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

/// # Complex value macro
///
/// This creates a [`Complex`] instance. You can pass in any of the following:
/// - One value => [`Complex::from_real`]
/// - Comma followed by one value => [`Complex::from_imaginary`]
/// - Two values seperated by a comma => [`Complex::from_complex`]
/// ```
/// use lineas::{Complex, comp};
/// assert_eq!(comp!(1), Complex::from_real(1));
/// assert_eq!(comp!(,-3), Complex::from_imaginary(-3));
/// assert_eq!(comp!(10, 4), Complex::from_complex(10, 4));
/// ```
///
/// To use this macro you do not need to have `use lineas::Complex` in your file
#[macro_export]
macro_rules! comp {
    ($x:expr) => { $crate::Complex::from_real($x) };
	(,$x:expr) => { $crate::Complex::from_imaginary($x) };
	($x:expr,$y:expr) => { $crate::Complex::from_complex($x, $y) };
}

/// Angle enum
///
/// Crate-wide enum for holding angles
pub enum Angle<L: Rotation> {
	/// Angle in degrees
	Degrees(L),
	/// Angle in radians
	Radians(L),
	/// Angle in gradians
	Gradians(L)
}

impl<L: Rotation> Angle<L> {
	pub(crate) fn angle(&self) -> f64 {
		match self {
			Angle::Degrees(v) => v.conv() * PI / 180.,
			Angle::Radians(v) => v.conv(),
			Angle::Gradians(v) => v.conv() * PI / 50.
		}
	}
}
