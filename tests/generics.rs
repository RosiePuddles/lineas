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
fn rotatef32() {
	let lhs = Matrix::rotation(Angle::Degrees(180.));
	let rhs = Matrix::new([[-1f32, 0.], [0., -1.]]);
	assert_eq!(lhs.epsilon_filter(), rhs)
}

#[test]
fn rotatef64() {
	let lhs = Matrix::rotation(Angle::Degrees(180.)).epsilon_filter();
	let rhs = Matrix::new([[-1f64, 0.], [0., -1.]]);
	assert_eq!(lhs.epsilon_filter(), rhs)
}
