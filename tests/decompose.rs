use lineas::prelude::Matrix;

#[cfg(test)]
mod lu_decompose {
	use super::*;
	
	#[test]
	fn correct() {
		let a = Matrix::new([[2, -8, 4], [1, -1, 11], [-3, 10, -8]]).dtype::<f64>();
		let l_check = Matrix::new([[2, 0, 0], [1, 3, 0], [-3, -2, 4]]).dtype::<f64>();
		let u_check = Matrix::new([[1, -4, 2], [0, 1, 3], [0, 0, 1]]).dtype::<f64>();
		let (l, u) = a.lu_decompose().unwrap();
		assert_eq!(l, l_check);
		assert_eq!(u, u_check);
	}
	
	
}