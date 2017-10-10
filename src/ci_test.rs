use super::*;

use std::env;

#[test]
fn is_env_equal_same() {
    env::set_var("CI_TEST_SAME", "YES");

    let same = is_env_equal("CI_TEST_SAME", "YES");

    assert!(same);
}

#[test]
fn is_env_equal_different() {
    env::set_var("CI_TEST_DIFF", "NO");

    let same = is_env_equal("CI_TEST_DIFF", "YES");

    assert!(!same);
}

#[test]
fn is_env_equal_not_defined() {
    let same = is_env_equal("CI_TEST_NOT_DEFINED", "BAD");

    assert!(!same);
}

#[test]
fn is_env_defined_yes() {
    env::set_var("CI_DEFINED_YES", "YES");

    let defined = is_env_defined("CI_DEFINED_YES");

    assert!(defined);
}

#[test]
fn is_env_defined_not_defined() {
    let defined = is_env_defined("CI_DEFINED_NO");

    assert!(!defined);
}

#[test]
fn get_test() {
    let info = get();

    assert_eq!(info.ci, info.vendor.is_some());
}
