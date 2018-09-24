#[macro_use]
extern crate bencher;
extern crate performance_test;

const COMPUTATION_SIZE: u32 = 400;

use bencher::Bencher;
use bencher::black_box;

fn imperative(bench: &mut Bencher) {
    bench.iter( || black_box(performance_test::compute_imperative(self::COMPUTATION_SIZE)));
}

fn imperative_opt(bench: &mut Bencher) {
    bench.iter( || black_box(performance_test::compute_imperative_opt(self::COMPUTATION_SIZE)));
}

fn functional(bench: &mut Bencher) {
    bench.iter(|| black_box(performance_test::compute_functional(self::COMPUTATION_SIZE)));
}

fn functional_opt(bench: &mut Bencher) {
    bench.iter(|| black_box(performance_test::compute_functional_opt(self::COMPUTATION_SIZE)));
}

fn recursive(bench: &mut Bencher) {
    bench.iter(|| black_box(performance_test::compute_recursive(self::COMPUTATION_SIZE)));
}

fn recursive_opt(bench: &mut Bencher) {
    bench.iter(|| black_box(performance_test::compute_recursive_opt(self::COMPUTATION_SIZE)));
}

benchmark_group!(computation, functional, functional_opt, imperative, imperative_opt, recursive, recursive_opt);
benchmark_main!(computation);
