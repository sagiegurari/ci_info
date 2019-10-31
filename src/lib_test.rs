use super::*;
use crate::test_env::{get_with_env, setup_env};

#[test]
fn get_test() {
    let info = get_with_env(vec![]);

    assert_eq!(info.ci, info.vendor.is_some());
}

#[test]
fn is_ci_test() {
    let lock = setup_env(vec![]);
    let info = get();
    let ci = is_ci();
    drop(lock);
    assert_eq!(info.ci, ci);
}
