#[macro_use]
extern crate bencher;
extern crate rust_performance_test;

const COMPUTATION_SIZE: u32 = 400;

use bencher::Bencher;
use bencher::black_box;
use rust_performance_test::*;

fn computation_imperative(bench: &mut Bencher) {
    bench.iter( || black_box(compute_imperative(COMPUTATION_SIZE)));
}

fn computation_imperative_opt(bench: &mut Bencher) {
    bench.iter( || black_box(compute_imperative_opt(COMPUTATION_SIZE)));
}

fn computation_functional(bench: &mut Bencher) {
    bench.iter(|| black_box(compute_functional(COMPUTATION_SIZE)));
}

fn computation_functional_opt(bench: &mut Bencher) {
    bench.iter(|| black_box(compute_functional_opt(COMPUTATION_SIZE)));
}

fn computation_recursive(bench: &mut Bencher) {
    bench.iter(|| black_box(compute_recursive(COMPUTATION_SIZE)));
}

fn computation_recursive_opt(bench: &mut Bencher) {
    bench.iter(|| black_box(compute_recursive_opt(COMPUTATION_SIZE)));
}

benchmark_group!(computation_benchmarks,
    computation_functional,
    computation_functional_opt,
    computation_imperative,
    computation_imperative_opt,
    computation_recursive,
    computation_recursive_opt);
benchmark_main!(computation_benchmarks);
