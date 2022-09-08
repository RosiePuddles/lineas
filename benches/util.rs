use std::fmt::Debug;
use std::ops::Range;
use rand::{Rng, thread_rng};
use lineas::Matrix;

pub fn f<L: Copy + Debug, const N: usize, const M: usize>(r: Range<L>) -> Matrix<N, M, L> {
	let mut out = [[r.start; M]; N];
	for i in 0..N {
		for n in 0..M {
			out[i][n] = thread_rng().gen_range(r.clone())
		}
	}
	Matrix::new(out)
}
