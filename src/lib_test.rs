use super::*;

#[test]
fn get_test() {
    let info = get();

    assert_eq!(info.ci, info.vendor.is_some());
}
