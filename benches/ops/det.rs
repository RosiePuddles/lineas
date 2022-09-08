use criterion::Criterion;
use rand::thread_rng;
use lineas::Matrix;
use crate::util::f;

pub fn det2() {
	let a = f::<_, 2, 2>(i8::MIN..i8::MAX);
	let b = a.determinant();
}

pub fn det3() {
let a = Matrix::new([[-89, 44, -122], [52, 48, -39], [-26, -102, -112]]);
assert_eq!(a.determinant(), 1628210)
}

pub fn det6() {
let a = Matrix::new([[-109, 16, -29, -21, -18], [-86, 88, -29, 50, -33], [59, 115, -93, 65, -101], [-43, -36, -72, 34, -69], [66, 71, 93, 103, -45]]);
assert_eq!(a.determinant(), -10037210765i64)
}

pub fn main(c: &mut Criterion) {
	let mut group = c.benchmark_group("Determinant");
	group.bench_function("Det 2", |b| b.iter(|| det2()));
	group.bench_function("Det 3", |b| b.iter(|| det3()));
	group.bench_function("Det 6", |b| b.iter(|| det6()));
    group.finish();
}
