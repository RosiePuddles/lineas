//! Main structs for the polynomials
use std::fmt::Debug;

/// Polynomial struct
///
/// This contains a `Vec<L>` of polynomial coefficients in decreasing order of exponent.
///
/// For example the polynomial 3x²-5x+1 would be represented as
/// ```
/// # use lineas::polynomials::Polynomial;
/// Polynomial::new(vec![3, -5, 1])
/// ```
///
/// Make sure you include the polynomial struct as
/// ```
/// use lineas::polynomials::Polynomial;
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct Polynomial<L: Copy + Debug> (pub(crate) Vec<L>);
