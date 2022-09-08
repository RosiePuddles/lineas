use lineas::{vector, comp};

#[cfg(test)]
mod dot {
	use super::*;
	
	#[cfg(test)]
	mod real {
		use super::*;
		
		#[test]
		fn real() {
			let lhs1 = vector![0, -7, 9];
			let lhs2 = vector![-5, 3, 6];
			assert_eq!(lhs1.dot(lhs2), 33)
		}
		
		#[test]
		fn mult_equivalence() {
			let lhs1 = vector![0, -7, 9];
			let lhs2 = vector![-5, 3, 6];
			assert_eq!(lhs1 * lhs2.transpose(), vector![33])
		}
	}
	
	#[test]
	fn complex() {
		let lhs1 = vector![comp!(2), comp!(, 4), comp!(1)];
		let lhs2 = vector![comp!(-2, 3), comp!(3), comp!(, -2)];
		assert_eq!(lhs1.cdot(lhs2), comp!(-4, 8))
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
			assert_eq!(vector![3, 4, 5].norm(Norm::Euclidean), 7);
		}
	
		#[test]
		fn p_norm_equivalence() {
			assert_eq!(vector![3, 4, 5].norm(Norm::p_norm(2)), 7);
		}
	}
	
	#[cfg(test)]
	mod manhattan {
		use super::*;
		
		#[test]
		fn manhattan() {
			assert_eq!(vector![1, -2, 3].norm(Norm::Manhattan), 6);
		}
	
		#[test]
		fn p_norm_equivalence() {
			assert_eq!(vector![1, -2, 3].norm(Norm::p_norm(1)), 6);
		}
	}
	
	#[test]
	fn p_norm() {
		assert_eq!(vector![5, 6, 7].norm(Norm::p_norm(3)), 8)
	}
	
	#[test]
	fn custom() {
		assert_eq!(vector![3, 4, 5].norm(Norm::Custom(|_t| 0)), 0);
	}
}

#[test]
fn cross() {
	let lhs1 = vector![1, 2, 3];
	let lhs2 = vector![-4, 9, 0];
	let rhs = vector![-27, -12, 17];
	assert_eq!(lhs1.cross(lhs2), rhs)
}
