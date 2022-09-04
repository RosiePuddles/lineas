use lineas::polynomials::Polynomial;

#[cfg(test)]
mod add {
	use super::*;
	
	#[cfg(test)]
	mod polynomials {
		use super::*;
		
		#[test]
		fn add() {
			let lhs1 = Polynomial::new(vec![1, 2, 3]);
			let lhs2 = Polynomial::new(vec![-5, 3, 10]);
			let rhs = Polynomial::new(vec![-4, 5, 13]);
			assert_eq!(lhs1 + lhs2, rhs)
		}
		
		#[test]
		fn add_zero_last_2() {
			let lhs1 = Polynomial::new(vec![2, 0, 5, -6]);
			let lhs2 = Polynomial::new(vec![-2, 0, 1, 3]);
			let rhs = Polynomial::new(vec![6, -3]);
			assert_eq!(lhs1 + lhs2, rhs)
		}
		
		#[test]
		fn add_assign() {
			let mut lhs1 = Polynomial::new(vec![1, 2, 3]);
			let lhs2 = Polynomial::new(vec![-5, 3, 10]);
			let rhs = Polynomial::new(vec![-4, 5, 13]);
			lhs1 += lhs2;
			assert_eq!(lhs1, rhs)
		}
		
		#[test]
		fn add_assign_zero_last_2() {
			let mut lhs1 = Polynomial::new(vec![2, 0, 5, -6]);
			let lhs2 = Polynomial::new(vec![-2, 0, 1, 3]);
			let rhs = Polynomial::new(vec![6, -3]);
			lhs1 += lhs2;
			assert_eq!(lhs1, rhs)
		}
	}
	
	#[cfg(test)]
	mod literals {
		use super::*;
		
		#[test]
		fn add() {
			let lhs1 = Polynomial::new(vec![1, 2, 3]);
			let lhs2 = 10;
			let rhs = Polynomial::new(vec![1, 2, 13]);
			assert_eq!(lhs1 + lhs2, rhs)
		}
		
		#[test]
		fn add_zero_last_2() {
			let lhs1 = Polynomial::new(vec![0, 0, 5, -6]);
			let lhs2 = 4;
			let rhs = Polynomial::new(vec![5, -2]);
			assert_eq!(lhs1 + lhs2, rhs)
		}
		
		#[test]
		fn add_assign() {
			let mut lhs1 = Polynomial::new(vec![1, 2, 3]);
			let lhs2 = 10;
			let rhs = Polynomial::new(vec![1, 2, 13]);
			lhs1 += lhs2;
			assert_eq!(lhs1, rhs)
		}
		
		#[test]
		fn add_assign_zero_last_2() {
			let mut lhs1 = Polynomial::new(vec![0, 0, 5, -6]);
			let lhs2 = 4;
			let rhs = Polynomial::new(vec![5, -2]);
			lhs1 += lhs2;
			assert_eq!(lhs1, rhs)
		}
	}
}

#[cfg(test)]
mod mult {
	use super::*;
	
	#[cfg(test)]
	mod polynomials {
		use super::*;
		
		#[test]
		fn mult() {
			let lhs1 = Polynomial::new(vec![1, -2, 3]);
			let lhs2 = Polynomial::new(vec![4, 5, -6]);
			let rhs = Polynomial::new(vec![4, -3, -4, 27, -18]);
			assert_eq!(lhs1 * lhs2, rhs)
		}
		
		#[test]
		fn mult_assign() {
			let mut lhs1 = Polynomial::new(vec![2, 5, -6]);
			let lhs2 = Polynomial::new(vec![4, 1, 3]);
			let rhs = Polynomial::new(vec![8, 22, -13, 9, -18]);
			lhs1 *= lhs2;
			assert_eq!(lhs1, rhs)
		}
	}
	
	#[cfg(test)]
	mod literals {
		use super::*;
		
		#[test]
		fn mult() {
			let lhs1 = Polynomial::new(vec![1, 2, 3]);
			let lhs2 = -4;
			let rhs = Polynomial::new(vec![-4, -8, -12]);
			assert_eq!(lhs1 * lhs2, rhs)
		}
		
		#[test]
		fn mult_assign() {
			let mut lhs1 = Polynomial::new(vec![1, 3, -4]);
			let lhs2 = 2;
			let rhs = Polynomial::new(vec![2, 6, -8]);
			lhs1 *= lhs2;
			assert_eq!(lhs1, rhs)
		}
	}
}
