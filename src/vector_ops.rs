use std::fmt::Debug;
use std::ops::{Add, Mul, Neg, Sub};
use conv::{ConvUtil, ValueFrom};
use crate::{Complex, Norm, Vector};
use crate::traits::{Abs, Pows};

/// # Vector macro
///
/// This creates a [`Vector`] instance. Use it like you would `vec!`
/// ```
/// use lineas::{vector, Vector};
/// let lhs = vector!([1, 2, 3]);
/// let rhs = Vector::new([[1, 2, 3]]);
/// assert_eq!(lhs, rhs)
/// ```
///
/// To use this macro you do not need to have `use lineas::Vector` in your file
#[macro_export]
macro_rules! vector {
    ([$($x:expr),+ $(,)?]) => { $crate::Vector::new([[$($x),+]]) };
}

/// # Column vector macro
///
/// This creates a [`ColVector`] instance. Use it like you would `vec!`
/// ```
/// use lineas::{cvector, ColVector};
/// let lhs = cvector!([1, 2, 3]);
/// let rhs = ColVector::new([[1], [2], [3]]);
/// assert_eq!(lhs, rhs)
/// ```
///
/// To use this macro you do not need to have `use lineas::ColVector` in your file
#[macro_export]
macro_rules! cvector {
    ([$($x:expr),+ $(,)?]) => { $crate::ColVector::new([$(([$x])),+]) };
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

impl<const T: usize, L: Copy + Debug> Vector<T, L> {
	/// Calculate the dot product of two vectors
	///
	/// This is equivalent to A . Bᵀ where Bᵀ is B transposed. This function is preferable to the
	/// multiplication based approach because it returns a single value, whereas multiplying vectors
	/// would return a `Matrix<1, 1, L>`
	pub fn dot(&self, rhs: Self) -> L where L: Add<Output=L> + Mul<Output=L> + ValueFrom<isize> {
		self.0[0].iter().zip(rhs.0[0].iter()).fold(0.value_as().unwrap(), |acc, (l, r)| acc + *l * *r)
	}
	
	/// Calculate the magnitude of a vector.
	///
	/// This is equivalent to [`Vector::norm`][Vector::norm] with a [Euclidean norm][Norm::Euclidean]
	pub fn magnitude(&self) -> L where L: Add<Output=L> + Mul<Output=L> + ValueFrom<isize> + Copy + Pows + Abs {
		self.norm(Norm::Euclidean)
	}
	
	/// Calculate the norm of a vector for a given norm
	///
	/// The norm can be either an enum variant from the [`Norm`][Norm] enum, or a function
	/// `fn(Vec<L>) -> L`.
	///
	/// You can pass a custom norm function using the [`Norm::Custom`] variant. For example a norm
	/// that always return 0 would be:
	/// ```
	/// # use lineas::{Norm, vector};
	/// vector!([1, 2, 3]).norm(Norm::Custom(|t| 0))
	/// ```
	pub fn norm(&self, norm: Norm<L>) -> L where L: ValueFrom<isize> + Copy + Add<Output=L> + Pows + Abs {
		norm.call(self.0[0].clone().to_vec())
	}
}

impl<const T: usize, L: Copy + Debug> Vector<T, Complex<L>> {
	/// Complex version of the dot product
	///
	/// If using complex vectors, you should be using this function not [Vector::dot]
	///
	/// This is not a commutative function unlike [Vector::dot], meaning that A . B ≠ B . A
	pub fn cdot(&self, rhs: Self) -> Complex<L> where L: Add<Output=L> + Mul<Output=L> + ValueFrom<isize> + Neg<Output=L> + Sub<Output=L> {
		self.0[0].iter().zip(rhs.0[0].iter()).fold(Complex::from_real(0.value_as().unwrap()), |acc, (l, r)| acc + *l * r.conj())
	}
}
