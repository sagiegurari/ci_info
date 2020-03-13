#![feature(test)]
extern crate test;

use ci_info;
use test::Bencher;

#[bench]
fn get(bencher: &mut Bencher) {
    bencher.iter(|| {
        let info = ci_info::get();

        assert_eq!(info.ci, info.vendor.is_some());
    });
}
