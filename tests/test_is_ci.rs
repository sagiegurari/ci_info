extern crate ci_info;

#[test]
fn is_ci_test() {
    let info = ci_info::get();
    let ci = ci_info::is_ci();

    assert_eq!(info.ci, ci);
}
