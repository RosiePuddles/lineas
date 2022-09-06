use lineas::{vector, imag};

#[cfg(test)]
mod dot {
	use super::*;
	
	#[cfg(test)]
	mod real {
		use super::*;
		
		#[test]
		fn real() {
			let lhs1 = vector!([0, -7, 9]);
			let lhs2 = vector!([-5, 3, 6]);
			assert_eq!(lhs1.dot(lhs2), 33)
		}
		
		#[test]
		fn mult_equivalence() {
			let lhs1 = vector!([0, -7, 9]);
			let lhs2 = vector!([-5, 3, 6]);
			assert_eq!(lhs1 * lhs2.transpose(), vector!([33]))
		}
	}
	
	#[test]
	fn complex() {
		let lhs1 = vector!([imag!(2), imag!(, 4), imag!(1)]);
		let lhs2 = vector!([imag!(-2, 3), imag!(3), imag!(, -2)]);
		assert_eq!(lhs1.cdot(lhs2), imag!(-4, 8))
	}
}

#[cfg(test)]
mod norm {
	use lineas::Norm;
	use super::*;
	
	#[cfg(test)]
	mod euclidean {
		use super::*;
		
		#[test]
		fn euclidean() {
			assert_eq!(vector!([3, 4, 5]).norm(Norm::Euclidean), 7);
		}
	
		#[test]
		fn p_norm_equivalence() {
			assert_eq!(vector!([3, 4, 5]).norm(Norm::p_norm(2)), 7);
		}
	}
	
	#[cfg(test)]
	mod manhattan {
		use super::*;
		
		#[test]
		fn manhattan() {
			assert_eq!(vector!([1, -2, 3]).norm(Norm::Manhattan), 6);
		}
	
		#[test]
		fn p_norm_equivalence() {
			assert_eq!(vector!([1, -2, 3]).norm(Norm::p_norm(1)), 6);
		}
	}
	
	#[test]
	fn custom() {
		assert_eq!(vector!([3, 4, 5]).norm(Norm::Custom(|_t| 0)), 0);
	}
}
