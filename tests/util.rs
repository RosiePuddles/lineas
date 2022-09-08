use lineas::{Matrix, Complex, comp};

#[cfg(test)]
mod conjugate {
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

#[test]
fn comp_abs() {
	let lhs = Matrix::new([[comp!(, 5), comp!(3, -2)], [comp!(1, -3), comp!(1)]]);
	let rhs = Matrix::new([[comp!(, 5), comp!(3, 2)], [comp!(1, 3), comp!(1)]]);
	assert_eq!(lhs.c_abs(), rhs)
}

#[cfg(test)]
mod min_max {
	use super::*;
	use lineas::{Norm, vector};
	
	#[test]
	fn min() {
		assert_eq!(Matrix::new([[-95, 18, -22], [20, -40, -9], [30, -68, 87]]).min(), -95)
	}
	
	#[test]
	fn max() {
		assert_eq!(Matrix::new([[-88, 90, -17], [-75, 71, -78], [-74, -70, -28]]).max(), 90)
	}
	
	#[test]
	fn min_max() {
		assert_eq!(
			Matrix::new([[-88, 90, -17], [-75, 71, -78], [-74, -70, -28]]).min_max(),
			(-88, 90)
		)
	}
	
	#[test]
	fn min_row_euclidean() {
		assert_eq!(Matrix::new([[-31, 98, 91], [92, 105, 32], [52, 21, 83]]).min_row(Norm::Euclidean), vector![52, 21, 83])
	}
	
	#[test]
	fn max_row_euclidean() {
		assert_eq!(Matrix::new([[-4, 82, 76], [73, -106, -110], [2, 25, 80]]).min_row(Norm::Euclidean), vector![2, 25, 80])
	}
	
	#[test]
	fn min_max_row_manhattan() {
		assert_eq!(
			Matrix::new([[6, -120, 23], [-72, -32, -88], [102, 2, -20]]).min_max_row(Norm::Manhattan),
			(vector![102, 2, -20], vector![102, 2, -20])
		)
	}
}
