extern crate ci_info;

#[test]
fn get() {
    let info = ci_info::get();

    assert_eq!(info.ci, info.vendor.is_some());
}
