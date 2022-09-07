<div style="text-align: center;" align="center">
<h1>Lineas</h1>
<h3>Linear algebra in Rust</h3>
<img src="https://img.shields.io/crates/v/lineas.svg" alt="crate.io version badge">
<img src="https://img.shields.io/docsrs/lineas/latest" alt="docs.rs docs status badge">
</div>

This fast and simple-to-use library provides the ability to perform linear algebra within Rust. It provides matrices and vectors as well as complex numbers for all your computing needs.

## Examples
### Use lineas
If you want to use a matrix, you ned to import it. Operations are included as default too.
```rust
use lineas::Matrix;
```

### A new matrix
```rust
Matrix::new([[1, 2, 3], [4, 5, 6]]);
```
Make sure that the array you give is two dimensional and that each element is the same size

### DTypes
You can change the dtype (data type taken from NumPy) with the `dtype` function
```rust
Matrix::new([[1, 2], [3, 4]]).dtype::<f32>();
```

### Decomposition
If you need to use decompositions, you'll need to specify that:
```rust
use lineas::decompose::{
    LUDecompose,
    PLUDecompose,
    Diagonalise
};

let a = Matrix::new([[1, 2], [3, 4]]);

a.lu_decompose();
a.plu_decompose();
a.diagonalise();
```

## Contribute

If you want to contribute, feel free. That is the nature of open source after all. Do a pull request or something.
