# Operations

Most common trait based operations have been implemented for matrices:
- Add `a + b`
- Subtract `a - b`
- Multiply `a * b`
- Negate `-a`

We also implement the following in the case of two square matrices:
- Add assign `a += b`
- Subtract assign `a -= b`
- Multiply assign `a *= b`

These operations are all for matrices and not a matrix and a scalar.

## Other operations

There are other operations for matrices that require functions:

- Scalar multiplication [`Matrix::scale`](https://docs.rs/lineas/latest/lineas/prelude/struct.Matrix.html#method.scale)
- Scalar multiplication with assignment [`Matrix::scale_set`](https://docs.rs/lineas/latest/lineas/prelude/struct.Matrix.html#method.scale_set)
- Sum of all values in the matrix [`Matrix::sum`](https://docs.rs/lineas/latest/lineas/prelude/struct.Matrix.html#method.sum)
- Elementwise absolute values [`Matrix::abs`](https://docs.rs/lineas/latest/lineas/prelude/struct.Matrix.html#method.abs)
- Transposition [`Matrix::transpose`](https://docs.rs/lineas/latest/lineas/prelude/struct.Matrix.html#method.transpose)
- Epsilon filter [`Matrix::epsilon_filter`](https://docs.rs/lineas/latest/lineas/prelude/struct.Matrix.html#method.epsilon_filter)
- dtype [`Matrix::dtype`](https://docs.rs/lineas/latest/lineas/prelude/struct.Matrix.html#method.dtype)

with the following for square matrices only:

- Determinant [`Matrix::determinant`](https://docs.rs/lineas/latest/lineas/prelude/struct.Matrix.html#method.determinant)
- Trace [`Matrix::trace`](https://docs.rs/lineas/latest/lineas/prelude/struct.Matrix.html#method.trace)
- Diagonal [`Matrix::diag`](https://docs.rs/lineas/latest/lineas/prelude/struct.Matrix.html#method.diag)
- Transpose assignment [`Matrix::transpose_set`](https://docs.rs/lineas/latest/lineas/prelude/struct.Matrix.html#method.transpose_set)

with the additional function [`Matrix::c_abs`](https://docs.rs/lineas/latest/lineas/prelude/struct.Matrix.html#method.c_abs) for the elementwise complex absolute function. This is not the same as `Matrix::abs` which for complex values would return the magnitude of each value. The equation below gives the definition of the function as abs':
\\[ A\in\mathbb{C}^{n\times m},\mathrm{abs'}(A)_{i,j}=|\Re(A\_{i,j})|+|\Im(A\_{i,j})| \\]

## Some interesting but useful functions

In the lists above, you may not know what `dtype` or `epsilon_filter` are despite them being useful functions.

`Matrix::dtype` converts the type of the values in the matrix
```rust
# use lineas::Matrix;
# fn main() {
let lhs = Matrix::new([[1, 0], [0, 1]]).dtype::<f32>();
let rhs = Matrix::new([[1., 0.], [0., 1.]]);
assert_eq!(lhs, rhs)
# }
```

`Matrix::epsilon_filter` removes very small values less that the epsilon value for the type. For an use case for this, consider that we wanted a 2D rotation matrix for 180º which we get using the following
```rust
let rotation_matrix = Matrix::rotation(Angle::Degrees(180.))
```

But, if we have a look at this matrix we get this:
```
Matrix([[-1.0, 8.742277657347586e-8], [-8.742277657347586e-8, -1.0]])
```
which isn't the expected `Matrix([[-1, 0], [0, -1]])`. But the values we expected to be 0 are very small. If we use the `epsilon_filter` function we get what we want.

```rust
assert_eq!(rotation_matrix.epsilon_filter(), Matrix::new([[1., 0.], [0., 1.]]))
```
