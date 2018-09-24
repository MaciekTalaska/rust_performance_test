#[macro_use]
extern crate bencher;
extern crate performance_test;

use bencher::Bencher;
use bencher::black_box;
use performance_test::*;

const VECTOR_SIZE: u32 = 4;

fn imperative(bench: &mut Bencher) {
    bench.iter( || black_box(build_vector_imperative(VECTOR_SIZE)));
}

fn functional(bench: &mut Bencher) {
    bench.iter( || black_box(build_vector_functional(VECTOR_SIZE)));
}

fn recursive(bench: &mut Bencher) {
    bench.iter( || black_box(build_vector_recursive(VECTOR_SIZE)));
}

benchmark_group!(build_vector, imperative, functional, recursive);
benchmark_main!(build_vector);