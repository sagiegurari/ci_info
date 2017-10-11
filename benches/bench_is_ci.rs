#![feature(test)]
extern crate ci_info;
extern crate test;

use test::Bencher;

#[bench]
fn is_ci(bencher: &mut Bencher) {
    bencher.iter(|| { ci_info::is_ci(); });
}
