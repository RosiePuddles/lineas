use lineas::{vector, cvector, comp};

#[cfg(test)]
mod main {
	use super::*;
	use lineas::{Vector, ColVector, Complex};
	
	#[cfg(test)]
	mod complex {
		use super::*;
		
		#[test]
		fn real() {
			assert_eq!(comp!(1), Complex::from_real(1))
		}
		
		#[test]
		fn imag_only() {
			assert_eq!(comp!(,-3), Complex::from_imaginary(-3));
		}
		
		#[test]
		fn full_complex() {
			assert_eq!(comp!(9. ,-2.3), Complex::from_complex(9., -2.3));
		}
	}
	
	#[cfg(test)]
	mod vectors {
		use super::*;
		
		#[test]
		fn vector() {
			let lhs = vector![1, 2, 3];
			let rhs = Vector::new([[1, 2, 3]]);
			assert_eq!(lhs, rhs)
		}
		
		#[test]
		fn cvector() {
			let lhs = cvector![1, 2, 3];
			let rhs = ColVector::new([[1], [2], [3]]);
			assert_eq!(lhs, rhs)
		}
	}
}

#[cfg(test)]
mod without_imports {
	use super::*;
	
	#[test]
	fn vector() {
		let _a = vector![1, 2, 3];
	}
	
	#[test]
	fn cvector() {
		let _a = cvector![1, 2, 3];
	}
	
	#[test]
	fn imag() {
		let _a = comp!(4);
	}
}
