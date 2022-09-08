use lineas::Complex;

#[cfg(test)]
mod ops {
	use super::*;
	
	#[test]
	fn mult() {
		let lhs1 = Complex::from_complex(1, 2);
		let lhs2 = Complex::from_complex(3, 4);
		let rhs = Complex::from_complex(-5, 10);
		assert_eq!(lhs1 * lhs2, rhs)
	}
	
	#[test]
	fn mult_assign() {
		let mut lhs1 = Complex::from_complex(1, 2);
		let lhs2 = Complex::from_complex(3, 4);
		let rhs = Complex::from_complex(-5, 10);
		lhs1 *= lhs2;
		assert_eq!(lhs1, rhs)
	}
	
	#[test]
	fn div() {
		let lhs1 = Complex::from_complex(1, 2).dtype::<f32>();
		let lhs2 = Complex::from_complex(3, 4).dtype();
		let rhs = Complex::from_complex(0.44, 0.08);
		assert_eq!(lhs1 / lhs2, rhs)
	}
	
	#[test]
	fn div_assign() {
		let mut lhs1 = Complex::from_complex(1, 2).dtype::<f32>();
		let lhs2 = Complex::from_complex(3, 4).dtype();
		let rhs = Complex::from_complex(0.44, 0.08);
		lhs1 /= lhs2;
		assert_eq!(lhs1, rhs)
	}
	
	#[test]
	fn add() {
		let lhs1 = Complex::from_complex(1, 2);
		let lhs2 = Complex::from_complex(3, 4);
		let rhs = Complex::from_complex(4, 6);
		assert_eq!(lhs1 + lhs2, rhs)
	}
	
	#[test]
	fn add_assign() {
		let mut lhs1 = Complex::from_complex(1, 2);
		let lhs2 = Complex::from_complex(3, 4);
		let rhs = Complex::from_complex(4, 6);
		lhs1 += lhs2;
		assert_eq!(lhs1, rhs)
	}
	
	#[test]
	fn sub() {
		let lhs1 = Complex::from_complex(1, 2);
		let lhs2 = Complex::from_complex(3, 4);
		let rhs = Complex::from_complex(-2, -2);
		assert_eq!(lhs1 - lhs2, rhs)
	}
	
	#[test]
	fn sub_assign() {
		let mut lhs1 = Complex::from_complex(1, 2);
		let lhs2 = Complex::from_complex(3, 4);
		let rhs = Complex::from_complex(-2, -2);
		lhs1 -= lhs2;
		assert_eq!(lhs1, rhs)
	}
	
	#[test]
	fn neg() {
		let lhs = Complex::from_complex(-5, 7);
		let rhs = Complex::from_complex(5, -7);
		assert_eq!(-lhs, rhs)
	}
}

#[cfg(test)]
mod abs {
	use lineas::comp;
	use lineas::traits::Abs;
	use super::*;
	
	#[test]
	fn abs() {
		let lhs = Complex::from_complex(3, 4);
		assert_eq!(lhs.absolute(), comp!(5))
	}
	
	#[test]
	fn element_abs() {
		let lhs = Complex::from_complex(-9, 12);
		let rhs = Complex::from_complex(9, 12);
		assert_eq!(lhs.element_abs(), rhs)
	}
}

#[test]
fn conj() {
	let lhs = Complex::from_complex(12, -9);
	let rhs = Complex::from_complex(12, 9);
	assert_eq!(lhs.conj(), rhs)
}
