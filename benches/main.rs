use criterion::{criterion_group, criterion_main};
mod ops;
mod util;

criterion_group!(ops, ops::mult::main, ops::det::main);
criterion_main!(ops);
