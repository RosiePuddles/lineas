use lineas::Matrix;

#[cfg(test)]
mod add {
	use super::*;

	#[test]
	fn add() {
		let a = Matrix::new([[4, 38, 11], [0, -91, 19]]);
		let b = Matrix::new([[13, 47, 20], [9, -82, 28]]);
		assert_eq!(a.elem_add(9), b)
	}

	#[test]
	fn add_assign() {
		let mut a = Matrix::new([[4, 38, 11], [0, -91, 19]]);
		let b = Matrix::new([[13, 47, 20], [9, -82, 28]]);
		a.elem_add_assign(9);
		assert_eq!(a, b)
	}
}

#[cfg(test)]
mod sub {
	use super::*;

	#[test]
	fn sub() {
		let a = Matrix::new([[-66, -79, -51], [-97, 20, -120]]);
		let b = Matrix::new([[-69, -82, -54], [-100, 17, -123]]);
		assert_eq!(a.elem_sub(3), b)
	}

	#[test]
	fn sub_assign() {
		let mut a = Matrix::new([[-66, -79, -51], [-97, 20, -120]]);
		let b = Matrix::new([[-69, -82, -54], [-100, 17, -123]]);
		a.elem_sub_assign(3);
		assert_eq!(a, b)
	}
}

#[cfg(test)]
mod mult {
	use super::*;

	#[test]
	fn mult() {
		let lhs1 = Matrix::new([[-58, -41, -37], [-99, -8, -46]]);
		let lhs2 = Matrix::new([[2, -1, 3], [1, 5, 2]]);
		let b = Matrix::new([[-116, 41, -111], [-99, -40, -92]]);
		assert_eq!(lhs1.elem_mult(lhs2), b)
	}

	#[test]
	fn mult_assign() {
		let mut lhs1 = Matrix::new([[-58, -41, -37], [-99, -8, -46]]);
		let lhs2 = Matrix::new([[2, -1, 3], [1, 5, 2]]);
		let b = Matrix::new([[-116, 41, -111], [-99, -40, -92]]);
		lhs1.elem_mult_assign(lhs2);
		assert_eq!(lhs1, b)
	}
}

#[cfg(test)]
mod div {
	use super::*;

	#[test]
	fn div() {
		let lhs1 = Matrix::new([[-58, -41, -37], [-99, -8, -46]]);
		let lhs2 = Matrix::new([[3, -4, 2], [-7, 1, 12]]);
		let b = Matrix::new([[-19, 10, -18], [14, -8, -3]]);
		assert_eq!(lhs1.elem_div(lhs2), b)
	}

	#[test]
	fn div_assign() {
		let mut lhs1 = Matrix::new([[-58, -41, -37], [-99, -8, -46]]);
		let lhs2 = Matrix::new([[3, -4, 2], [-7, 1, 12]]);
		let b = Matrix::new([[-19, 10, -18], [14, -8, -3]]);
		lhs1.elem_div_assign(lhs2);
		assert_eq!(lhs1, b)
	}
}
