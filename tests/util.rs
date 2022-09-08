use lineas::Matrix;

#[cfg(test)]
mod det {
	use super::*;
	
	#[test]
	fn identity3() {
		let i3: Matrix<3, 3, f32> = Matrix::identity();
		assert_eq!(i3.determinant(), 1f32)
	}
	
	#[test]
	fn identity4() {
		let i4: Matrix<4, 4, f32> = Matrix::identity();
		assert_eq!(i4.determinant(), 1f32)
	}
	
	#[test]
	fn random_6() {
		let a = Matrix::new([[-109, 16, -29, -21, -18], [-86, 88, -29, 50, -33], [59, 115, -93, 65, -101], [-43, -36, -72, 34, -69], [66, 71, 93, 103, -45]]);
		assert_eq!(a.determinant(), -10037210765i64)
	}
	
	#[test]
	fn random_2() {
		let a = Matrix::new([[-68, -23], [-74, 17]]);
		assert_eq!(a.determinant(), -2858)
	}
	
	#[test]
	fn random_3() {
		let a = Matrix::new([[-89, 44, -122], [52, 48, -39], [-26, -102, -112]]);
		assert_eq!(a.determinant(), 1628210)
	}
}

#[cfg(test)]
mod conjugate {
	use lineas::Complex;
	use super::*;
	
	#[cfg(test)]
	mod real {
		use super::*;
		
		#[test]
		fn conjugate() {
			let lhs = Matrix::new([[1, 2, 3], [4, 5, 6]]);
			let rhs = Matrix::new([
				[Complex::from_real(1), Complex::from_real(2), Complex::from_real(3)],
				[Complex::from_real(4), Complex::from_real(5), Complex::from_real(6)]
			]);
			assert_eq!(lhs.conj(), rhs)
		}
		
		#[test]
		fn double_conjugate() {
			let lhs = Matrix::new([[1, 2, 3], [4, 5, 6]]);
			let rhs = Matrix::new([
				[Complex::from_real(1), Complex::from_real(2), Complex::from_real(3)],
				[Complex::from_real(4), Complex::from_real(5), Complex::from_real(6)]
			]);
			assert_eq!(lhs.conj().conj(), rhs)
		}
	}
	
	#[cfg(test)]
	mod complex {
		use super::*;
		
		#[test]
		fn conjugate() {
			let lhs = Matrix::new([
				[Complex::from_complex(1, 2), Complex::from_complex(12, -2), Complex::from_imaginary(9)],
				[Complex::from_imaginary(-7), Complex::from_complex(5, -5), Complex::from_complex(1, 2)]
			]);
			let rhs = Matrix::new([
				[Complex::from_complex(1, -2), Complex::from_complex(12, 2), Complex::from_imaginary(-9)],
				[Complex::from_imaginary(7), Complex::from_complex(5, 5), Complex::from_complex(1, -2)]
			]);
			assert_eq!(lhs.conj(), rhs)
		}
		
		#[test]
		fn double_conjugate() {
			let lhs = Matrix::new([
				[Complex::from_complex(1, 2), Complex::from_complex(12, -2), Complex::from_imaginary(9)],
				[Complex::from_imaginary(-7), Complex::from_complex(5, -5), Complex::from_complex(1, 2)]
			]);
			assert_eq!(lhs, lhs.conj().conj())
		}
	}
}
