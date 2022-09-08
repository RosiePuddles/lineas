use lineas::Matrix;
use lineas::decompose::*;

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
		assert_eq!(l * u, a)
	}
	
	#[test]
	fn incorrect() {
		let a = Matrix::new([[0, 1], [1, 1]]);
		assert!(a.lu_decompose().is_none())
	}
}

#[cfg(test)]
mod plu_decompose {
	use super::*;
	
	#[test]
	fn correct() {
		let a = Matrix::new([[0, 1], [1, 1]]);
		let p_check = Matrix::new([[0, 1], [1, 0]]);
		let l_check = Matrix::new([[1, 0], [0, 1]]);
		let u_check = Matrix::new([[1, 1], [0, 1]]);
		let (p, l, u) = a.plu_decompose().unwrap();
		assert_eq!(p, p_check);
		assert_eq!(l, l_check);
		assert_eq!(u, u_check);
		assert_eq!(p * l * u, a)
	}
	
	// TODO: Find a matrix with no PLU decomposition
	// #[test]
	// fn incorrect() {
	// 	let a = Matrix::new([[0, 1], [1, 1]]);
	// 	assert!(a.lu_decompose().is_none())
	// }
}