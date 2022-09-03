use lineas::prelude::Matrix;

#[cfg(test)]
mod add {
	use super::*;
	
	#[test]
	fn matrix_matrix() {
		let lhs = Matrix::new([[1, 0, 0],[0, 1, 0], [0, 0, 1]]);
		let rhs = Matrix::new([[1, 0, 0],[0, 1, 0], [0, 0, 1]]);
		let res = Matrix::new([[2, 0, 0],[0, 2, 0], [0, 0, 2]]);
		assert_eq!(lhs + rhs, res)
	}
}
