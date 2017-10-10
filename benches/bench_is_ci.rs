#![feature(test)]
extern crate ci_info;
extern crate test;

use test::Bencher;

#[bench]
fn get(bencher: &mut Bencher) {
    bencher.iter(|| { ci_info::is_ci(); });
}
