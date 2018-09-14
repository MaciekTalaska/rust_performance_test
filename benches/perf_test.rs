#[macro_use]
extern crate bencher;
extern crate performance_test;


use bencher::Bencher;
use bencher::black_box;

fn imperative(bench: &mut Bencher) {
    bench.iter( || black_box(performance_test::compute_imperative(2)));
}

fn functional(bench: &mut Bencher) {
    bench.iter(|| black_box(performance_test::compute_functional(2)));
}

benchmark_group!(benches, functional, imperative);
benchmark_main!(benches);
