use super::*;

#[test]
fn get_test() {
    let info = get();

    assert_eq!(info.ci, info.vendor.is_some());
}

#[test]
fn is_ci_test() {
    let info = get();
    let ci = is_ci();

    assert_eq!(info.ci, ci);
}
