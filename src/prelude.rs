//! All the important bases
//!
//! This defined the matrix and vector as well as some simple operation implementations for
//! matrices. More complex operations are defined in other places
use std::fmt::Debug;

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
/// use lineas::prelude::Matrix;
/// let matrix: Matrix<2, 2, f64> = Matrix::new([[1, 0], [0, 1]]).dtype::<f64>();
/// ```
/// This gives us an identity matrix of size 2 with each value being an `f64` instead of `{integer}`
#[derive(Copy, Clone, Debug)]
pub struct Matrix<const T: usize, const N: usize, L: Copy + Debug> (pub(crate) [[L; N]; T] );

/// Vector struct
///
/// Derived from the [matrix][Matrix] and is technically a 1×`T` matrix
pub type Vector<const T: usize, L> = Matrix<1, T, L>;
