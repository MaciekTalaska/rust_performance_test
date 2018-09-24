#[macro_use]
extern crate bencher;
extern crate rust_performance_test;

use bencher::Bencher;
use bencher::black_box;
use rust_performance_test::vectors::*;

const VECTOR_SIZE: u32 = 4;

fn vector_imperative(bench: &mut Bencher) {
    bench.iter( || black_box(build_vector_imperative(VECTOR_SIZE)));
}

fn vector_functional(bench: &mut Bencher) {
    bench.iter( || black_box(build_vector_functional(VECTOR_SIZE)));
}

fn vector_recursive(bench: &mut Bencher) {
    bench.iter( || black_box(build_vector_recursive(VECTOR_SIZE)));
}

benchmark_group!(build_vector_benchmarks,
    vector_imperative,
    vector_functional,
    vector_recursive);
benchmark_main!(build_vector_benchmarks);