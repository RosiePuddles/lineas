use std::fmt::Debug;
use std::ops::{Add, Div, Mul, Neg, Sub};
use conv::{ConvUtil, ValueFrom};
use crate::{Complex, Matrix, Norm, Trig, Vector};
use crate::traits::{Abs, Pows};

/// # Vector macro
///
/// This creates a [`Vector`] instance. Use it like you would `vec!`
/// ```
/// use lineas::{vector, Vector};
/// let lhs = vector![1, 2, 3];
/// let rhs = Vector::new([[1, 2, 3]]);
/// assert_eq!(lhs, rhs)
/// ```
///
/// To use this macro you do not need to have `use lineas::Vector` in your file
#[macro_export]
macro_rules! vector {
    [$($x:expr),+ $(,)?] => { $crate::Vector::new([[$($x),+]]) };
}

/// # Column vector macro
///
/// This creates a [`crate::prelude::ColVector`] instance. Use it like you would `vec!`
/// ```
/// use lineas::{cvector, ColVector};
/// let lhs = cvector![1, 2, 3];
/// let rhs = ColVector::new([[1], [2], [3]]);
/// assert_eq!(lhs, rhs)
/// ```
///
/// To use this macro you do not need to have `use lineas::ColVector` in your file
#[macro_export]
macro_rules! cvector {
    [$($x:expr),+ $(,)?] => { $crate::ColVector::new([$(([$x])),+]) };
}

impl<const T: usize, L: Copy + Debug> Vector<T, L> {
	/// Calculate the dot product of two vectors
	///
	/// This is equivalent to A . Bᵀ where Bᵀ is B transposed. This function is preferable to the
	/// multiplication based approach because it returns a single value, whereas multiplying vectors
	/// would return a `Matrix<1, 1, L>`
	pub fn dot<Q: Copy + Debug>(&self, rhs: Vector<T, Q>) -> L where L: Add<Output=L> + Mul<Output=L> + ValueFrom<isize> + ValueFrom<Q> {
		self.0[0].iter().zip(rhs.dtype::<L>().0[0].iter()).fold(0.value_as().unwrap(), |acc, (l, r)| acc + *l * *r)
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
	/// Calculate the spherical liner interpolation between two vectors
	///
	/// This calculate the angle between then as θ, and then calculates
	/// (sin((1-t)θ) * self / sin(θ)) + (sin(tθ) * rhs / sin(θ)). This does not return a normalised
	/// vector unless `self` and `rhs` were both normalised in which case it should. If you
	/// experience errors with the result not being a unit vector then we recommend adding
	/// [`.normalise()`][Vector::normalise] onto the end of your function call
	pub fn slerp<Q: Copy + Debug, B>(&self, rhs: Vector<T, Q>, t: B) -> Self where L: ValueFrom<Q> + ValueFrom<B> + ValueFrom<isize> + Add<Output=L> + Sub<Output=L> + Mul<Output=L> + Div<Output=L> + Pows + Trig + Abs {
		let lhs = self.normalise();
		let rhs = rhs.dtype::<L>().normalise();
		let theta: L = lhs.dot(rhs).arccos();
		let t: L = t.value_as::<L>().unwrap();
		self.scale(((1.value_as::<L>().unwrap() - t) * theta) / theta.sin()) + rhs.scale((t * theta) / theta.sin())
	}
}

impl<L: Copy + Debug, > Vector<3, L> {
	/// Cross product of two vectors in three dimensions
	pub fn cross<Q: Copy + Debug>(&self, rhs: Vector<3, Q>) -> Vector<3, L> where L: ValueFrom<isize> + ValueFrom<Q> + Add<Output=L> + Mul<Output=L> + Sub<Output=L> + Neg<Output=L> {
		let s = Matrix::new([[0.value_as().unwrap(); 3], self.0[0], rhs.dtype().0[0]]);
		vector![s.cofactor((0, 0)), s.cofactor((0, 1)), s.cofactor((0, 2))]
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
