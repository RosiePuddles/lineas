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
}
