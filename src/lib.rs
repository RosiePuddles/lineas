#![warn(missing_docs)]
#![doc(issue_tracker_base_url = "https://github.com/RosiePuddles/lineas/issues/")]
//! # Linear algebra in Rust
//!
//! This library gives a wide range of functionality to [matrices](prelude::Matrix) and [vectors](prelude::Vector) in Rust using fast
//! algorithms to make your code run as fast as possible.
//!
//! ## Making a matrix
//!	Matrices need you to specify the size of the matrix (rows×columns) and the data type in the
//! matrix.
//! ```
//! # use lineas::prelude::Matrix;
//! let mut matrix: Matrix<2, 2, isize> = Matrix::new([[1, 0], [0, 1]]);
//! ```
//! You can alter the data type, or dtype, with the [`dtype`](prelude::Matrix::dtype) function and specify the
//! desired dtype. The desired dtype has to satisfy some trait constraits
//! ```
//! # use lineas::prelude::Matrix;
//! # let mut matrix: Matrix<2, 2, isize> = Matrix::new([[1, 0], [0, 1]]);
//! let matrix: Matrix<2, 2, f32> = matrix.dtype::<f32>();
//! ```
//!
//! ## Decomposition
//! This library includes the ability to decompose matrices with various algorithm. The full list
//! can be seen on the [decompose] module doc page.
//!
//! To make use of this, you will need to include the following in your code
//! ```
//! use lineas::decompose::*;
//! ```
//! or replace `*` with the specific decomposition(s) you need
pub mod prelude;
pub mod decompose;
pub mod generics;
mod ops;
