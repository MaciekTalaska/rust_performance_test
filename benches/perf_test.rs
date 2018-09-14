#[macro_use]
extern crate bencher;
extern crate performance_test;


use bencher::Bencher;
use bencher::black_box;

fn imperative(bench: &mut Bencher) {
    bench.iter( || black_box(performance_test::compute_imperative(2)));
}

fn imperative_opt(bench: &mut Bencher) {
    bench.iter( || black_box(performance_test::compute_imperative_opt(2)));
}

fn functional(bench: &mut Bencher) {
    bench.iter(|| black_box(performance_test::compute_functional(2)));
}

fn functional_opt(bench: &mut Bencher) {
    bench.iter(|| black_box(performance_test::compute_functional(2)));
}

fn recursive(bench: &mut Bencher) {
    bench.iter(|| black_box(performance_test::compute_recursive(2)));
}

fn recursive_opt(bench: &mut Bencher) {
    bench.iter(|| black_box(performance_test::compute_recursive_opt(2)));
}

benchmark_group!(benches, functional, functional_opt, imperative, imperative_opt, recursive, recursive_opt);
benchmark_main!(benches);
