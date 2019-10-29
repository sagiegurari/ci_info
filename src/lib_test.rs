use super::*;
use crate::test_env::setup_env;

#[test]
fn get_test() {
    let _lock = setup_env(vec![]);
    let info = get();

    assert_eq!(info.ci, info.vendor.is_some());
}

#[test]
fn is_ci_test() {
    let _lock = setup_env(vec![]);
    let info = get();
    let ci = is_ci();

    assert_eq!(info.ci, ci);
}
