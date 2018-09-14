#[macro_use]
extern crate bencher;
extern crate performance_test;

fn return_5() -> u32 {
    return 5;
}

pub fn compute_functional(dices: u8) -> u32 {
    let indices = (0..dices).collect::<Vec<u8>>().iter()
        .map(|_e| return_5()-1).collect::<Vec<u32>>();
    let index = indices.iter()
        .fold(0, |sum, val| sum * 6 + val);

    index
}

pub fn compute_imperative(dices: u8) -> u32 {
    let mut index: u32 = 0;
    for _i in { 0..dices } {
        index *= 6;
        index += (return_5()) - 1;
    }
    index
}

use bencher::Bencher;
use bencher::black_box;

fn imperative(bench: &mut Bencher) {
    bench.iter( || black_box(self::compute_imperative(2)));
}

fn functional(bench: &mut Bencher) {
    bench.iter(|| black_box(self::compute_functional(2)));
}

benchmark_group!(benches, functional, imperative);
benchmark_main!(benches);
