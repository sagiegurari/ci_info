#![feature(test)]
extern crate test;

use ci_info;
use test::Bencher;

#[bench]
fn is_ci(bencher: &mut Bencher) {
    bencher.iter(|| {
        ci_info::is_ci();
    });
}
