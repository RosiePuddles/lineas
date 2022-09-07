use std::fmt::{Debug, Display, Error, Formatter};
use std::ops::{Add, Sub, AddAssign, SubAssign, Mul, MulAssign, Div, DivAssign, Neg};
use conv::{ConvUtil, ValueFrom};
use crate::{Complex, Epsilon};
use crate::traits::{Abs, Pows};

impl<L: Copy + Debug> Complex<L> {
	/// Make a new complex value from a real value
	///
	/// Generates a complex value with the imaginary part being 0
	pub fn from_real(r: L) -> Self where L: ValueFrom<isize> {
		Self {
			real: r,
			imaginary: 0.value_as().unwrap()
		}
	}
	
	/// Make a new complex value from an imaginary value
	///
	/// Generates a complex value with the real part being 0
	pub fn from_imaginary(i: L) -> Self where L: ValueFrom<isize> {
		Self {
			real: 0.value_as().unwrap(),
			imaginary: i
		}
	}
	
	/// Make a new complex value from a complex pair
	///
	/// Generates a complex value with the given real and imaginary parts. These must be the same
	/// type
	pub fn from_complex(r: L, i: L) -> Self {
		Self {
			real: r,
			imaginary: i
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
	
	/// Return the absolute values of the real and imaginary parts of the value
	/// ```
	/// # use lineas::{comp, Complex};
	/// let example: Complex<_> = comp!(-3, 2);
	/// assert_eq!(example.element_abs(), comp!(example.real(), example.imag()))
	/// ```
	pub fn element_abs(&self) -> Self where L: Abs {
		Complex {
			real: self.real().absolute(),
			imaginary: self.imaginary.absolute()
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
		let e = f.precision().unwrap_or(0);
		let w = f.width().unwrap_or(0);
		write!(f, "{:w$.e$}{:+w$.e$}i", self.real, self.imaginary)
	}
}

impl<L: Debug + Copy> Abs for Complex<L> where L: Pows + ValueFrom<isize> + Add<Output=L> {
	/// Return the absolute value of a complex value
	///
	/// Calculated as `(self.real() ** 2 + self.imaginary() ** 2) ** 0.5`
	fn absolute(&self) -> Self {
		Complex::from_real(self.real.power(2.value_as().unwrap()) + self.imaginary.power(2.value_as().unwrap()).root_n(2.value_as().unwrap()))
	}
}

impl<L: Debug + Copy> Epsilon for Complex<L> where L: Epsilon {
	fn epsilon(&self) -> Self {
		Complex {
			real: self.real.epsilon(),
			imaginary: self.imaginary.epsilon()
		}
	}
}
