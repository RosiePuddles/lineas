//! Trait implementations
use num_integer::Roots;

/// # Absolute trait
///
/// Returns the absolute value of a value and is implemented for all built-in numerical types (`i8`,
/// `u32`, `f64` etc.).
pub trait Abs where Self: Clone {
	/// Return the absolute value of self
	fn absolute(&self) -> Self;
}

/// # Power and root trait
///
/// This is required for norms and is implemented for all built-in numerical types (`i8`, `u32`,
/// `f64` etc.).
pub trait Pows {
	/// Power function.
	///
	/// Raises the value to a given power
	fn power(&self, p: Self) -> Self;
	/// n-th root function
	///
	/// Calculates the n-th root oc the value
	fn root_n(&self, p: Self) -> Self;
}

/// Epsilon trait for use in filtering out small values
pub trait Epsilon where Self: PartialEq {
	/// This should return the filtered version of a given value
	/// ```
	/// # use lineas::traits::Epsilon;
	///	assert_eq!(f64::EPSILON.epsilon(), 0f64)
	/// ```
	fn epsilon(&self) -> Self;
}

macro_rules! int {
    ($t:ty) => {
		impl Abs for $t {
			fn absolute(&self) -> $t {
				self.clone().abs()
			}
		}
		impl Pows for $t {
			fn power(&self, p: Self) -> $t {
				self.pow(p as u32)
			}
			
			fn root_n(&self, p: Self) -> $t {
				self.clone().nth_root(p as u32)
			}
		}
	};
}

macro_rules! int_u {
    ($t:ty) => {
		impl Abs for $t {
			fn absolute(&self) -> $t {
				self.clone()
			}
		}
		impl Pows for $t {
			fn power(&self, p: Self) -> $t {
				self.pow(p as u32)
			}
			
			fn root_n(&self, p: Self) -> $t {
				self.clone().nth_root(p as u32)
			}
		}
	};
}

macro_rules! float {
    ($t:ty) => {
		impl Abs for $t {
			fn absolute(&self) -> $t {
				self.clone().abs()
			}
		}
		impl Pows for $t {
			fn power(&self, p: Self) -> $t {
				self.powf(p)
			}
			
			fn root_n(&self, p: Self) -> $t {
				self.powf(1. / p)
			}
		}
		impl Epsilon for $t {
			fn epsilon(&self) -> $t {
				if self.abs() < <$t>::EPSILON { 0. } else { *self }
			}
		}
	};
}

int!(i8);
int!(i16);
int!(i32);
int!(i64);
int!(i128);
int!(isize);
float!(f32);
float!(f64);
int_u!(u8);
int_u!(u16);
int_u!(u32);
int_u!(u64);
int_u!(u128);
int_u!(usize);

