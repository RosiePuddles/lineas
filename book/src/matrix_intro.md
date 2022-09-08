# Matrix introduction

Matrices are statically sized in lineas. This might be expanded in later versions to include dynamically sized matrices.

To build a new matrix, we fist need to include the `Matrix` struct
```rust
use lineas::Matrix
```
and then use the `Matrix::new` function to generate a new matrix
```rust
# use lineas::Matrix
Matrix::new([[1, 2], [3, 4]]);
```

Internally, a matrix is represented as an array of arrays `[[L; N]; T]` for a `T` by `N` matrix.

## Vectors and column vectors

Vectors are just a matrix with a condition on them:
```rust
pub type Vector<const T: usize, L> = Matrix<1, T, L>;
```
and column vectors are similarly defined
```rust
pub type ColVector<const T: usize, L> = Matrix<T, 1, L>;
```

For both of these, because they derive from the `Matrix` struct, you can do `Vector::new` or `ColVector::new`. But this requires some extra typing that we can eliminate, especially in the case of a column vector. To make life easier, we have the `vector!` and `cvector!` macros
```rust
use lineas::{vector, Vector};
assert_eq!(vector!([1, 2, 3]), Vector::new([[1, 2, 3]]))
```

```rust
use lineas::{cvector, ColVector};
assert_eq!(cvector!([1, 2, 3]), ColVector::new([[1], [2], [3]]))
```
