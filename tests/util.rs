use lineas::prelude::Matrix;

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
	fn random() {
		let a = Matrix::new([
			[1, 5, 3, 0],
			[2, -9, -2, 10],
			[11, 0, 5, -3],
			[-7, 2, 4, -2]
		]).dtype::<f32>();
		assert_eq!(a.determinant(), -4111f32)
	}
}