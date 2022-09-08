use criterion::{criterion_group, criterion_main};
mod ops;

criterion_group!(ops, ops::main);
criterion_main!(ops);
