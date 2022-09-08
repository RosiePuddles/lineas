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
		use lineas::{cvector, vector};
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
		
		#[test]
		fn vector_matrix() {
			let lhs1 = vector!([1, 2, 3, 4]);
			let lhs2 = Matrix::new([[9, 17], [3, 0], [42, 69], [1, 7]]);
			let rhs = vector!([145, 252]);
			assert_eq!(lhs1 * lhs2, rhs)
		}
		
		#[test]
		fn matrix_cvector() {
			let lhs1 = Matrix::new([[-5, 4, 0], [2, 0, 15]]);
			let lhs2 = cvector!([-2, 3, 5]);
			let rhs = cvector!([22, 71]);
			assert_eq!(lhs1 * lhs2, rhs)
		}
	}
}

#[cfg(test)]
mod indexing {
	use super::*;
	
	#[cfg(test)]
	mod correct {
		use super::*;
		
		#[test]
		fn value() {
			let lhs = Matrix::new([[-3, 4], [12, 17], [-15, 0]]);
			let rhs = 12;
			assert_eq!(lhs[(1, 0)], rhs)
		}
	}
	
	#[cfg(test)]
	mod incorrect {
		use super::*;
		
		#[test]
		#[should_panic(expected = "Matrix has size 2×3. Row 4 is outside range 0 to 2")]
		fn outside_row() {
			let lhs = Matrix::new([[-3, 4, 0], [2, 1, -2]]);
			lhs[(4, 0)];
		}
		
		#[test]
		#[should_panic(expected = "Matrix has size 2×3. Column 3 is outside range 0 to 3")]
		fn outside_col() {
			let lhs = Matrix::new([[-3, 4, 0], [2, 1, -2]]);
			lhs[(1, 3)];
		}
	}
}

#[cfg(test)]
mod additive {
	use super::*;
	
	#[test]
	fn sum() {
		let lhs = Matrix::new([[-3, 12], [9, 0]]);
		let rhs = 18;
		assert_eq!(lhs.sum(), rhs)
	}
	
	#[test]
	fn trace() {
		let lhs = Matrix::new([[0, 12, 5], [3, -2, 4], [10, 15, 3]]);
		let mut rhs = lhs.clone();
		rhs[(1, 0)] = -5;
		assert_eq!(lhs.trace(), rhs.trace())
	}
	
	#[test]
	fn diag() {
		let lhs = Matrix::new([[0, 12, 5], [3, -2, 4], [10, 15, 3]]);
		let mut rhs = lhs.clone();
		rhs[(1, 0)] = -5;
		assert_eq!(lhs.diag(), rhs.diag())
	}
}

#[cfg(test)]
mod cofactors {
	use super::*;
	
	#[test]
	fn cofactor() {
		let lhs = Matrix::new([[-67, 126, 15], [22, 22, -83], [61, -13, -119]]);
		for (p, r) in [((0, 0), -3697), ((0, 1), 2445)] {
			assert_eq!(lhs.cofactor(p), r)
		}
	}
	
	#[test]
	fn adjoint() {
		let lhs = Matrix::new([[-3, 2, -5], [-1, 0, -2], [3, -4, 1]]);
		let rhs = Matrix::new([[-8, 18, -4], [-5, 12, -1], [4, -6, 2]]);
		assert_eq!(lhs.adjoint(), rhs)
	}
}
