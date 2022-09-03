use lineas::Matrix;

#[cfg(test)]
mod unary_not {
	use super::*;
	
	#[test]
	fn negate() {
		let lhs = Matrix::new([[1, 2, 3],[4, 5, 6], [7, 8, 9]]);
		let rhs = Matrix::new([[-1, -2, -3],[-4, -5, -6], [-7, -8, -9]]);
		assert_eq!(-lhs, rhs)
	}
	
	#[test]
	fn double_negate() {
		let lhs = Matrix::new([[1, 2, 3],[4, 5, 6], [7, 8, 9]]);
		assert_eq!(-(-lhs), lhs)
	}
}

#[cfg(test)]
mod add_sub {
	use super::*;
	
	#[test]
	fn matrix_add_matrix() {
		let lhs = Matrix::new([[1, 2, 3],[4, 5, 6], [7, 8, 9]]);
		let rhs = Matrix::new([[9, 8, 7],[6, 5, 4], [3, 2, 1]]);
		let res = Matrix::new([[10, 10, 10],[10, 10, 10], [10, 10, 10]]);
		assert_eq!(lhs + rhs, res)
	}
	
	#[test]
	fn matrix_sub_matrix() {
		let lhs = Matrix::new([[10, 10, 10],[10, 10, 10], [10, 10, 10]]);
		let rhs = Matrix::new([[9, 8, 7],[6, 5, 4], [3, 2, 1]]);
		let res = Matrix::new([[1, 2, 3],[4, 5, 6], [7, 8, 9]]);
		assert_eq!(lhs - rhs, res)
	}
	
	#[test]
	fn matrix_add_neg_matrix() {
		let lhs = Matrix::new([[10, 10, 10],[10, 10, 10], [10, 10, 10]]);
		let rhs = Matrix::new([[9, 8, 7],[6, 5, 4], [3, 2, 1]]);
		let res = Matrix::new([[1, 2, 3],[4, 5, 6], [7, 8, 9]]);
		assert_eq!(lhs + (-rhs), res)
	}
	
	#[test]
	fn matrix_sub_neg_matrix() {
		let lhs = Matrix::new([[1, 2, 3],[4, 5, 6], [7, 8, 9]]);
		let rhs = Matrix::new([[9, 8, 7],[6, 5, 4], [3, 2, 1]]);
		let res = Matrix::new([[10, 10, 10],[10, 10, 10], [10, 10, 10]]);
		assert_eq!(lhs - (-rhs), res)
	}
}

#[cfg(test)]
mod transpose {
	use super::*;
	
	#[test]
	fn square() {
		let lhs = Matrix::new([[3, 5, 9],[-2, 0, 4], [-10, 2, 5]]);
		let res = Matrix::new([[3, -2, -10],[5, 0, 2], [9, 4, 5]]);
		assert_eq!(lhs.transpose(), res)
	}
	
	#[test]
	fn square_set() {
		let mut lhs = Matrix::new([[3, 5, 9],[-2, 0, 4], [-10, 2, 5]]);
		lhs.transpose_set();
		let res = Matrix::new([[3, -2, -10],[5, 0, 2], [9, 4, 5]]);
		assert_eq!(lhs, res)
	}
	
	#[test]
	fn rectangular() {
		let lhs = Matrix::new([[4, -2], [10, 11], [-9, 3]]);
		let res = Matrix::new([[4, 10, -9],[-2, 11, 3]]);
		assert_eq!(lhs.transpose(), res)
	}
}

#[cfg(test)]
mod mult {
	use super::*;
	
	#[cfg(test)]
	mod scalar {
		use super::*;
		
		#[test]
		fn scalar() {
			let lhs = Matrix::new([[-3, 5, 4, 9], [-15, -5, -3, 0], [2, 10, -4, 6]]);
			let rhs = Matrix::new([[-9, 15, 12, 27], [-45, -15, -9, 0], [6, 30, -12, 18]]);
			assert_eq!(lhs.scale(3), rhs)
		}
		
		#[test]
		fn scalar_assign() {
			let mut lhs = Matrix::new([[-3, 5, 4, 9], [-15, -5, -3, 0], [2, 10, -4, 6]]);
			lhs.scale_set(3);
			let rhs = Matrix::new([[-9, 15, 12, 27], [-45, -15, -9, 0], [6, 30, -12, 18]]);
			assert_eq!(lhs, rhs)
		}
	}
	
	#[cfg(test)]
	mod matrix {
		use super::*;
		
		#[test]
		fn square() {
			let lhs = Matrix::new([[1, 2, 3],[4, 5, 6], [7, 8, 9]]);
			let rhs = Matrix::new([[30, 36, 42],[66, 81, 96], [102, 126, 150]]);
			assert_eq!(lhs * lhs, rhs)
		}
		
		#[test]
		fn square_assign() {
			let mut lhs = Matrix::new([[1, 2, 3], [4, 5, 6], [7, 8, 9]]);
			let rhs = Matrix::new([[30, 36, 42], [66, 81, 96], [102, 126, 150]]);
			lhs *= lhs;
			assert_eq!(lhs, rhs)
		}
		
		#[test]
		fn rect() {
			let lhs1 = Matrix::new([[9, 17], [3, 0], [42, 69], [1, 7]]);
			let lhs2 = Matrix::new([[-4, 8, 12, -19], [2, -10, 42, 76]]);
			let rhs = Matrix::new([[-2, -98, 822, 1121], [-12, 24, 36, -57], [-30, -354, 3402, 4446], [10, -62, 306, 513]]);
			assert_eq!(lhs1 * lhs2, rhs)
		}
	}
}
