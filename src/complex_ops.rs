use std::fmt::{Debug, Display, Error, Formatter};
use std::ops::{Add, Sub, AddAssign, SubAssign, Mul, MulAssign, Div, DivAssign, Neg};
use conv::{ConvUtil, ValueFrom};
use crate::Complex;

impl<L: Copy + Debug> Complex<L> where L: ValueFrom<isize> {
	/// Make a new complex value from a real value
	///
	/// Generates a complex value with the imaginary part being 0
	pub fn from_real(r: L) -> Self {
		Self {
			real: r,
			imaginary: 0.value_as().unwrap()
		}
	}
	
	/// Make a new complex value from an imaginary value
	///
	/// Generates a complex value with the real part being 0
	pub fn from_imaginary(i: L) -> Self {
		Self {
			real: 0.value_as().unwrap(),
			imaginary: i
		}
	}
	
	/// Make a new complex value from a complex pair
	///
	/// Generates a complex value with the given real and imaginary parts. These must be the same
	/// type
	pub fn from_complex<Q>(r: L, i: Q) -> Self where L: ValueFrom<Q> {
		Self {
			real: r,
			imaginary: i.value_as().unwrap()
		}
	}
	
	/// Return the real part of the complex number
	pub fn real(&self) -> L {
		self.real
	}
	
	/// Return the imaginary part of the complex number
	pub fn imag(&self) -> L {
		self.imaginary
	}
	
	/// Returns the complex conjugate of the complex number
	pub fn conj(&self) -> Self where L: Neg<Output=L> {
		Self {
			real: self.real,
			imaginary: -self.imaginary
		}
	}
	
	/// Changes the data type of the values
	pub fn dtype<Q>(&self) -> Complex<Q> where Q: Copy + Debug + ValueFrom<L> {
		Complex {
			real: self.real.value_as::<Q>().unwrap(),
			imaginary: self.imaginary.value_as::<Q>().unwrap()
		}
	}
}

impl<L: Copy + Debug> ValueFrom<isize> for Complex<L> where L: ValueFrom<isize> {
	type Err = Error;

	fn value_from(src: isize) -> Result<Self, Self::Err> {
		Ok(Complex::from_real(src.value_as().unwrap()))
	}
}

impl<L: Copy + Debug> Add<Complex<L>> for Complex<L> where L: Add<Output=L> {
	type Output = Self;
	
	fn add(self, rhs: Self) -> Self::Output {
		Self {
			real: self.real + rhs.real,
			imaginary: self.imaginary + rhs.imaginary
		}
	}
}

impl<L: Copy + Debug> AddAssign<Complex<L>> for Complex<L> where L: Add<Output=L> {
	fn add_assign(&mut self, rhs: Self) {
		self.real = self.real + rhs.real;
		self.imaginary = self.imaginary + rhs.imaginary;
	}
}

impl<L: Copy + Debug> Sub<Complex<L>> for Complex<L> where L: Sub<Output=L> {
	type Output = Self;
	
	fn sub(self, rhs: Self) -> Self::Output {
		Self {
			real: self.real - rhs.real,
			imaginary: self.imaginary - rhs.imaginary
		}
	}
}

impl<L: Copy + Debug> SubAssign<Complex<L>> for Complex<L> where L: Sub<Output=L> {
	fn sub_assign(&mut self, rhs: Self) {
		self.real = self.real - rhs.real;
		self.imaginary = self.imaginary - rhs.imaginary;
	}
}

impl<L: Copy + Debug> Mul<Complex<L>> for Complex<L> where L: Mul<Output=L> + Sub<Output=L> + Add<Output=L> {
	type Output = Self;
	
	fn mul(self, rhs: Self) -> Self::Output {
		Self {
			real: self.real * rhs.real - self.imaginary * rhs.imaginary,
			imaginary: self.imaginary * rhs.real + self.real * rhs.imaginary
		}
	}
}

impl<L: Copy + Debug> MulAssign<Complex<L>> for Complex<L> where L: Mul<Output=L> + Sub<Output=L> + Add<Output=L> {
	fn mul_assign(&mut self, rhs: Self) {
		let real_temp = self.real * rhs.real - self.imaginary * rhs.imaginary;
		self.imaginary = self.imaginary * rhs.real + self.real * rhs.imaginary;
		self.real = real_temp;
	}
}

impl<L: Copy + Debug> Div<Complex<L>> for Complex<L> where L: Mul<Output=L> + Div<Output=L> + Sub<Output=L> + Add<Output=L> + ValueFrom<isize> + Neg<Output=L> {
	type Output = Self;
	
	/// Divide two complex numbers.
	///
	/// Be aware that this may panic if for `a/b`, `b==0`
	fn div(self, rhs: Complex<L>) -> Self::Output {
		let mut top = self.clone();
		top *= rhs.conj();
		let bottom = rhs.real * rhs.real + rhs.imaginary * rhs.imaginary;
		top.real = top.real / bottom;
		top.imaginary = top.imaginary / bottom;
		top
	}
}

impl<L: Copy + Debug> DivAssign<Complex<L>> for Complex<L> where L: Mul<Output=L> + Div<Output=L> + Sub<Output=L> + Add<Output=L> + ValueFrom<isize> + Neg<Output=L> {
	/// Divide two complex numbers.
	///
	/// Be aware that this may panic if for `a/b`, `b==0`
	fn div_assign(&mut self, rhs: Complex<L>) {
		let mut top = self.clone();
		top *= rhs.conj();
		let bottom = rhs.real * rhs.real + rhs.imaginary * rhs.imaginary;
		top.real = top.real / bottom;
		top.imaginary = top.imaginary / bottom;
		self.real = top.real;
		self.imaginary = top.imaginary;
	}
}

impl<L: Display + Debug + Copy> Display for Complex<L> {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		write!(f, "{}{:+}i", self.real, self.imaginary)
	}
}
