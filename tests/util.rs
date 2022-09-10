use lineas::{Matrix, Complex, comp};
mod qol;

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

#[cfg(test)]
mod interpolation {
	use lineas::vector;
	use super::*;
	
	#[test]
	fn lerp() {
		let lhs = Matrix::empty();
		let rhs = Matrix::new([[1, 1, 1], [1, 1, 1], [1, 1, 1]]).dtype::<f32>();
		let res = Matrix::new([[0.5, 0.5, 0.5], [0.5, 0.5, 0.5], [0.5, 0.5, 0.5]]);
		assert_eq!(lhs.lerp(rhs, 0.5), res)
	}
	
	#[cfg(test)]
	mod slerp {
		use super::*;
		
		#[test]
		fn i_j_slerp() {
			let lhs = vector![1, 0, 0].dtype::<f32>();
			let rhs = vector![0, 1, 0].dtype::<f32>();
			let res = vector![1. / 2f32.sqrt(), 1. / 2f32.sqrt(), 0.];
			fuzzy_eq!(lhs.slerp(rhs, 0.5), res)
		}
		
		#[test]
		fn i_neg_i_slerp() {
			let lhs = vector![1, 0, 0].dtype::<f32>();
			let rhs = vector![-1, 0, 0].dtype::<f32>();
			let res = vector![0.5, 0., 0.];
			fuzzy_eq!(lhs.slerp(rhs, 0.25), res)
		}
		
		#[test]
		fn i_scale_i_slerp() {
			let lhs = vector![1, 0, 0].dtype::<f32>();
			let rhs = vector![2, 0, 0].dtype::<f32>();
			let res = vector![1.5, 0., 0.];
			fuzzy_eq!(lhs.slerp(rhs, 0.5), res)
		}
	}
}

#[cfg(test)]
mod iterators {
	use super::*;
	
	#[test]
	fn rows() {
		let a = Matrix::new([[-68, -61, 89], [-117, -28, 75], [90, 123, -67]]);
		let mut a_rows = a.rows();
		assert_eq!(a_rows.next(), Some(&[-68, -61, 89]));
		assert_eq!(a_rows.next(), Some(&[-117, -28, 75]));
		assert_eq!(a_rows.next(), Some(&[90, 123, -67]));
		assert_eq!(a_rows.next(), None);
		assert_eq!(a, Matrix::new([[-68, -61, 89], [-117, -28, 75], [90, 123, -67]]))
	}
}
