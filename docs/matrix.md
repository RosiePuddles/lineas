Matrix struct

A `Matrix<T, N, L>` contains an `[[L; N]; T]` array where `N` is the number of rows and `T` is
the number of columns giving a `T`×`N` matrix

To make a new matrix, do [`Matrix::new(data)`][Matrix::new]. You can also specify the type of the data by
following this with a call to the [`dtype`][Matrix::dtype] function.

For example
```
use lineas::Matrix;
let matrix: Matrix<2, 2, f64> = Matrix::new([[1, 0], [0, 1]]).dtype::<f64>();
```
This gives us an identity matrix of size 2 with each value being an `f64` instead of `{integer}`

## Functionality

Add, subtract, multiply, and negate are implemented for all matrices where such an operation is defined

## Function sections
               
We have split the implementations into several sections:

- [All matrices](#all-matrices)
- [Iterators](#iterators)
- [Interpolations](#interpolations)
- [Complex valued matrices](#complex-valued-matrices)
- [Square matrices](#square-matrices)
- [Elementwise operations](#elementwise-operations)
