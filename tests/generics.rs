use lineas::{Matrix, Angle};

#[test]
fn identity() {
	let lhs = Matrix::<3, 3, _>::identity();
	let rhs = Matrix::new([[1, 0, 0], [0, 1, 0], [0, 0, 1]]);
	assert_eq!(lhs, rhs)
}

#[test]
fn empty() {
	let lhs = Matrix::<3, 4, _>::empty();
	let rhs = Matrix::new([[0, 0, 0, 0], [0, 0, 0, 0], [0, 0, 0, 0]]);
	assert_eq!(lhs, rhs)
}

#[test]
fn rotate() {
	let lhs = Matrix::rotation(Angle::Degrees(180.));
	let rhs = Matrix::new([[-1., 0.], [0., -1.]]);
	assert!((lhs - rhs).sum() < 4. * f64::EPSILON)
}
